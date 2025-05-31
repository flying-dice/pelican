use log::LevelFilter;
use mlua::{FromLua, Lua, LuaSerdeExt, Value as LuaValue};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleConfig {
    pub logger_level: Option<LevelFilter>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            logger_level: Some(LevelFilter::Trace),
        }
    }
}

impl FromLua for ModuleConfig {
    fn from_lua(value: LuaValue, lua: &Lua) -> mlua::Result<Self> {
        let value = lua.from_value(value)?;
        serde_json::from_value::<ModuleConfig>(value).map_err(mlua::Error::runtime)
    }
}
