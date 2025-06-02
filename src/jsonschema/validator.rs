use jsonschema::ValidationError;
use log::debug;
use mlua::{
    Error as LuaError, IntoLuaMulti, Lua, LuaSerdeExt, MetaMethod as LuaMetaMethod, UserData,
    UserDataMethods, Value, Value as LuaValue,
};

pub struct _Validator {
    pub validator: jsonschema::Validator,
    pub schema_raw: String,
}

impl _Validator {
    fn new(
        schema_raw: String,
        schema: serde_json::Value,
    ) -> Result<Self, ValidationError<'static>> {
        let validator = jsonschema::Validator::new(&schema)?;

        Ok(_Validator {
            validator,
            schema_raw,
        })
    }

    fn validate<'i>(&self, value: &'i serde_json::Value) -> Result<(), ValidationError<'i>> {
        self.validator.validate(value)
    }
}

impl UserData for _Validator {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua: &Lua, value: Value| {
            debug!("Creating new Validator with value: {:?}", value);
            let schema: serde_json::Value = lua.from_value(value)?;
            let schema_raw: String = schema.to_string();

            debug!("Parsed value: {:?}", schema_raw);
            _Validator::new(schema_raw, schema).map_err(LuaError::external)
        });

        methods.add_meta_method(
            LuaMetaMethod::ToString,
            |_: &Lua, this: &_Validator, (): ()| Ok(format!("Validator({})", this.schema_raw)),
        );

        methods.add_method(
            "validate",
            |lua: &Lua, this: &_Validator, value: LuaValue| {
                let value: serde_json::Value = lua.from_value(value)?;
                match this.validate(&value) {
                    Ok(_) => true.into_lua_multi(lua),
                    Err(e) => (false, e.to_string()).into_lua_multi(lua),
                }
            },
        );
    }
}
