use log::{debug, error};
use mlua::prelude::{LuaString, LuaValue};
use mlua::{Error, IntoLuaMulti, Lua, LuaSerdeExt, MultiValue, Nil};
use serde_json::from_str;
use serde_json::Value;

pub fn decode(lua: &Lua, lstring: &LuaString) -> Result<MultiValue, Error> {
    debug!("Decoding JSON: {:?}", lstring);

    let json_string = lstring.to_str();
    let json_string = match json_string {
        Ok(s) => s,
        Err(err) => {
            error!("Error converting Lua string to Rust string: {:?}", err);
            return (
                Nil,
                "Error converting Lua string to Rust string".to_string(),
            )
                .into_lua_multi(lua);
        }
    };

    let serde_value = from_str::<Value>(&json_string);
    let serde_value = match serde_value {
        Ok(v) => v,
        Err(err) => {
            error!("Error decoding JSON: {:?}", &err);
            return (Nil, format!("Invalid JSON: {}", &json_string)).into_lua_multi(lua);
        }
    };

    let lua_value = lua.to_value(&serde_value);
    let lua_value = match lua_value {
        Ok(v) => v,
        Err(err) => {
            error!("Error converting to Lua value: {:?}", &err);
            return (Nil, "Error converting to Lua value".to_string()).into_lua_multi(lua);
        }
    };

    debug!("Decoded JSON: {:?}", lua_value);
    (lua_value, Nil).into_lua_multi(lua)
}

pub fn encode(lua: &Lua, lua_value: &LuaValue) -> Result<MultiValue, Error> {
    debug!("Encoding Lua value: {:?}", lua_value);

    let json_string = serde_json::to_string(&lua_value);

    let json_string = match json_string {
        Ok(s) => s,
        Err(err) => {
            error!("Error encoding JSON: {:?}", &err);
            return (Nil, format!("Error encoding JSON: {:?}", err)).into_lua_multi(lua);
        }
    };

    let lua_string = lua.create_string(&json_string);
    let lua_string = match lua_string {
        Ok(s) => s,
        Err(err) => {
            error!("Error converting to Lua string: {:?}", &err);
            return (Nil, "Error converting to Lua string".to_string()).into_lua_multi(lua);
        }
    };

    debug!("Encoded JSON: {:?}", lua_string);
    (lua_string, Nil).into_lua_multi(lua)
}
