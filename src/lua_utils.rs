use log::{info, warn};
use mlua::prelude::{LuaTable, LuaValue};
use serde_json::Value;

/**
 * Check if a Lua table is an array by checking if it has contiguous integer keys starting from 1.
 */
pub fn is_lua_array(table: &LuaTable) -> mlua::Result<bool> {
    let mut last_index = 0;

    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (key, _) = pair?;

        let index = match key {
            LuaValue::Integer(i) if i > 0 => i,
            _ => return Ok(false), // Non-integer or non-positive index
        };

        if index != last_index + 1 {
            return Ok(false); // Not contiguous
        }

        last_index = index;
    }

    Ok(true)
}

pub fn serialize_lua_to_json(lua_value: &LuaValue) -> Option<Value> {
    info!("Serializing Lua value: {:?}", lua_value);
    match lua_value {
        LuaValue::Nil => Some(Value::Null),
        LuaValue::Boolean(b) => Some(Value::Bool(*b)),
        LuaValue::Integer(i) => Some(Value::Number((*i).into())),
        LuaValue::Number(n) => Some(Value::Number(serde_json::Number::from_f64(*n).unwrap())),
        LuaValue::String(s) => Some(Value::String(s.to_str().unwrap().to_string())),
        LuaValue::Table(table) => match is_lua_array(table) {
            Ok(true) => serialize_lua_array_to_json(table),
            Ok(false) => serialize_lua_table_to_json(table),
            Err(_) => {
                warn!("Failed to determine if Lua table is an array");
                None
            }
        },
        other => Some(other.type_name().into()),
    }
}

fn serialize_lua_table_to_json(table: &LuaTable) -> Option<Value> {
    let mut map = serde_json::Map::new();
    for pair in table.pairs::<LuaValue, LuaValue>() {
        match pair {
            Ok((key, value)) => {
                if let Some(key_str) = key.as_string() {
                    info!("Serializing Lua table key: {:?}", key_str);
                    // TODO: figure out how to handle non-string keys
                    if let Some(value_json) = serialize_lua_to_json(&value) {
                        map.insert(key_str.to_string_lossy(), value_json);
                    }
                }
            }
            Err(_) => return None,
        }
    }
    Some(Value::Object(map))
}

fn serialize_lua_array_to_json(table: &LuaTable) -> Option<Value> {
    info!(
        "Serializing Lua array: {:?} with {:?} elements",
        table,
        table.len()
    );
    let mut vec = Vec::new();
    for pair in table.pairs::<LuaValue, LuaValue>() {
        match pair {
            Ok((_, value)) => {
                info!("Serializing Lua array element: {:?}", value);
                vec.push(serialize_lua_to_json(&value)?);
            }
            Err(_) => {
                warn!("Failed to serialize Lua array element {:?}", pair);
            }
        }
    }
    Some(Value::Array(vec))
}
