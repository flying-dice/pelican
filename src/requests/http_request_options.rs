use mlua::{ExternalError, FromLua, Lua, LuaSerdeExt, Result as LuaResult, Value as LuaValue};
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
        serde_json::from_value::<HttpRequestOptions>(value).map_err(|e| e.into_lua_err())
    }
}
