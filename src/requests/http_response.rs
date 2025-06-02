use crate::requests::http_request_options::HttpHeaderMap;
use mlua::prelude::LuaNil;
use mlua::{IntoLuaMulti, Lua, LuaSerdeExt, UserData, UserDataMethods};
use reqwest::blocking::Response;
use serde_json::{from_str, Value};

pub struct HttpResponse {
    status: u16,
    headers: HttpHeaderMap,
    body: String,
}

impl HttpResponse {
    fn new(status: u16, headers: HttpHeaderMap, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub(crate) fn from_response(response: Response) -> HttpResponse {
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.text().unwrap();
        HttpResponse::new(status, HttpHeaderMap(headers), body)
    }

    fn get_status(&self) -> u16 {
        self.status
    }

    fn get_headers(&self) -> HttpHeaderMap {
        self.headers.clone()
    }

    fn get_header_value(&self, key: String) -> Result<String, String> {
        let Ok(key) = key.parse::<String>();

        let Some(value) = self.headers.0.get(&key) else {
            return Err(format!("Header '{}' not found", key));
        };

        let Ok(value_str) = value.to_str() else {
            return Err(format!("Header '{}' is not a valid string", key));
        };

        Ok(value_str.to_string())
    }

    fn get_text(&self) -> String {
        self.body.clone()
    }

    fn get_json(&self) -> Result<Value, String> {
        from_str::<Value>(&self.body).map_err(|e| e.to_string())
    }
}

impl UserData for HttpResponse {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_status", |_lua, this, (): ()| Ok(this.get_status()));
        methods.add_method("get_headers", |_lua, this, (): ()| Ok(this.get_headers()));
        methods.add_method("get_header_value", |lua: &Lua, this, key: String| {
            this.get_header_value(key).into_lua_multi(lua)
        });
        methods.add_method("get_text", |_lua, this, (): ()| Ok(this.get_text()));
        methods.add_method("get_json", |lua, this, (): ()| match this.get_json() {
            Ok(json) => lua.to_value(&json).into_lua_multi(lua),
            Err(err) => (LuaNil, err).into_lua_multi(lua),
        });
    }
}
