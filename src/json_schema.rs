use log::debug;
use mlua::prelude::*;
use mlua::{Nil, UserData, UserDataMethods};
use serde_json::Value as JsonValue;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;
    m.set("validator_for", lua.create_function(validator_for)?)?;
    table.set("jsonschema", m)?;
    Ok(())
}

struct _Validator {
    pub validator: jsonschema::Validator,
}

impl _Validator {
    fn new(validator: jsonschema::Validator) -> Self {
        _Validator { validator }
    }
}

impl UserData for _Validator {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "validate",
            |lua: &Lua, this: &_Validator, value: LuaValue| {
                debug!("Validating value: {:?}", value);
                let value: JsonValue = lua.from_value(value)?;
                match this.validator.validate(&value) {
                    Ok(_) => {
                        debug!("Value is valid");
                        true.into_lua_multi(lua)
                    }
                    Err(e) => {
                        debug!("Value is invalid: {:?}", e);
                        (Nil, e.to_string()).into_lua_multi(lua)
                    }
                }
            },
        );
    }
}

fn validator_for(lua: &Lua, schema: LuaValue) -> LuaResult<_Validator> {
    let schema: JsonValue = lua.from_value(schema)?;

    let validator = jsonschema::Validator::new(&schema).map_err(|e| {
        debug!("Failed to create validator: {:?}", e);
        LuaError::RuntimeError(format!("Invalid schema: {:?}", e))
    })?;

    Ok(_Validator::new(validator))
}
