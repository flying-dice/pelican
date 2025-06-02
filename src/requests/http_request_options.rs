use mlua::{
    Error as LuaError, FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult, Value as LuaValue,
};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct HttpRequestOptions {
    pub headers: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
}

impl FromLua for HttpRequestOptions {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let value = lua.from_value(value)?;
        serde_json::from_value::<HttpRequestOptions>(value).map_err(LuaError::external)
    }
}

#[derive(Debug, Clone)]
pub struct HttpHeaderMap(pub HeaderMap);

impl IntoLua for HttpHeaderMap {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let headers = lua.create_table()?;
        for (key, value) in self.0.iter() {
            headers.set(key.as_str(), value.to_str().unwrap())?;
        }

        headers.into_lua(lua)
    }
}

#[derive(Debug, Clone)]
pub struct Url(pub reqwest::Url);

impl FromLua for Url {
    fn from_lua(value: LuaValue, lua: &Lua) -> mlua::Result<Self> {
        let url_str: String = lua.from_value(value)?;
        reqwest::Url::parse(&url_str)
            .map(Url)
            .map_err(LuaError::external)
    }
}
