use crate::requests::http_header_map::HttpHeaderMap;
use mlua::Error::RuntimeError;
use mlua::{
    ExternalError, Lua, LuaSerdeExt, Result as LuaResult, UserData, UserDataMethods,
    Value as LuaValue,
};
use serde_json::{from_str, Value};

pub struct HttpResponse {
    status: u16,
    headers: HttpHeaderMap,
    body: String,
}

impl HttpResponse {
    pub(crate) fn new(status: u16, headers: HttpHeaderMap, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    fn get_status(&self, _lua: &Lua) -> mlua::Result<u16> {
        Ok(self.status)
    }

    fn get_headers(&self, _lua: &Lua) -> mlua::Result<HttpHeaderMap> {
        Ok(self.headers.clone())
    }

    fn get_header_value(&self, lua: &Lua, key: String) -> LuaResult<String> {
        match self.headers.0.get(key.as_str()) {
            Some(value) => value
                .to_str()
                .map(|s| s.to_string())
                .map_err(|e| e.into_lua_err()),
            None => Err(RuntimeError(format!("Header '{}' not found", key))),
        }
    }

    fn get_text(&self, _lua: &Lua) -> mlua::Result<String> {
        Ok(self.body.clone())
    }

    fn get_json(&self, lua: &Lua) -> LuaResult<LuaValue> {
        let serde_value = from_str::<Value>(&self.body).map_err(|e| e.into_lua_err())?;
        lua.to_value(&serde_value).map_err(|e| e.into_lua_err())
    }
}

impl UserData for HttpResponse {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_status", |lua, this, (): ()| this.get_status(lua));
        methods.add_method("get_headers", |lua, this, (): ()| this.get_headers(lua));
        methods.add_method("get_header_value", |lua: &Lua, this, key: String| {
            this.get_header_value(lua, key)
        });
        methods.add_method("get_text", |lua, this, (): ()| this.get_text(lua));
        methods.add_method("get_json", |lua, this, (): ()| this.get_json(lua));
    }
}
