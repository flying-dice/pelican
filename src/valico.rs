use mlua::prelude::{LuaMultiValue, LuaString, LuaTable, LuaValue};
use mlua::{IntoLuaMulti, Lua, LuaSerdeExt, MultiValue, Nil, Result, Value};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::fmt::format;
use valico::json_schema;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("validate", lua.create_function(validate)?)?;

    table.set("valico", m)?;

    Ok(())
}

fn validate(lua: &Lua, (schema, data): (LuaValue, LuaValue)) -> Result<LuaMultiValue> {
    let mut scope = json_schema::Scope::new();

    let schema: JsonValue = lua.from_value(schema)?;
    let data: JsonValue = lua.from_value(data)?;

    let schema = scope.compile_and_return(schema, true).unwrap();

    let validation = schema.validate(&data);

    if validation.is_valid() {
        Ok(true.into_lua_multi(lua)?)
    } else {
        let error_message = format!("{:?}", validation.errors);
        Ok((Nil, error_message).into_lua_multi(lua)?)
    }
}
