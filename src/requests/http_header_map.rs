use mlua::{IntoLua, Lua, Result as LuaResult, Value as LuaValue};
use reqwest::header::HeaderMap;

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
