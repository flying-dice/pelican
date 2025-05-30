use log::debug;
use mlua::prelude::{LuaMultiValue, LuaTable, LuaValue};
use mlua::{IntoLuaMulti, Lua, LuaSerdeExt, Nil, Result};
use serde_json::from_str;
use serde_json::Value;

pub fn decode(lua: &Lua, string: String) -> Result<LuaMultiValue> {
    debug!("Decoding JSON: {:?}", string);

    let serde_value = match from_str::<Value>(&string) {
        Ok(value) => value,
        Err(e) => {
            debug!("Failed to decode JSON: {:?}", e);
            return (Nil, format!("Invalid JSON: {:?}", string)).into_lua_multi(lua);
        }
    };

    debug!("Decoded JSON value: {:?}", serde_value);

    let lua_value = match lua.to_value(&serde_value) {
        Ok(value) => value,
        Err(e) => {
            debug!("Failed to convert to Lua value: {:?}", e);
            return (Nil, "Failed to convert to Lua value").into_lua_multi(lua);
        }
    };

    debug!("Converted to Lua value: {:?}", lua_value);

    lua_value.into_lua_multi(lua)
}

pub fn encode(lua: &Lua, lua_value: LuaValue) -> Result<LuaMultiValue> {
    debug!("Encoding Lua value: {:?}", lua_value);

    let json_string = match serde_json::to_string(&lua_value) {
        Ok(value) => value,
        Err(e) => {
            debug!("Failed to encode Lua value: {:?}", e);
            return (Nil, "Failed to encode Lua value").into_lua_multi(lua);
        }
    };

    debug!("Encoded JSON string: {:?}", json_string);

    json_string.into_lua_multi(lua)
}

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("encode", lua.create_function(encode)?)?;
    m.set("decode", lua.create_function(decode)?)?;

    table.set("json", m)?;

    Ok(())
}
