use crate::lua_utils::serialize_lua_to_json;
use log::debug;
use mlua::prelude::{LuaTable, LuaValue};
use mlua::{IntoLuaMulti, Lua, LuaSerdeExt, Result};
use serde_json::from_str;
use serde_json::Value;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set(
        "encode",
        lua.create_function(|lua: &Lua, lua_value: LuaValue| {
            match serde_json::to_string(&lua_value) {
                Ok(json_string) => json_string.into_lua_multi(lua),
                Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
            }
        })?,
    )?;

    m.set(
        "safe_encode",
        lua.create_function(|lua: &Lua, lua_value: LuaValue| {
            let value = serialize_lua_to_json(&lua_value);
            match value {
                Some(value) => match serde_json::to_string(&value) {
                    Ok(json_string) => json_string.into_lua_multi(lua),
                    Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
                },
                None => (
                    LuaValue::Nil,
                    format!(
                        "Unsupported Lua value for JSON serialization {:?}",
                        lua_value
                    ),
                )
                    .into_lua_multi(lua),
            }
        })?,
    )?;

    m.set(
        "decode",
        lua.create_function(|lua: &Lua, value: String| {
            debug!("json.decode: {}", value);

            match from_str::<Value>(&value) {
                Ok(value) => lua.to_value(&value).into_lua_multi(lua),
                Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
            }
        })?,
    )?;

    table.set("json", m)?;

    Ok(())
}
