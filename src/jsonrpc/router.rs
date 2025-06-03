use log::debug;
use mlua::prelude::LuaFunction;
use mlua::{Lua, UserData, UserDataMethods};
use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonRpcRouter {
    methods: HashMap<String, LuaFunction>,
}

impl JsonRpcRouter {
    fn new() -> Self {
        Self {
            methods: HashMap::new(),
        }
    }

    fn add_method(&mut self, name: String, callback: mlua::Function) {
        debug!("Adding method: {:?}", name);
        self.methods.insert(name, callback);
    }

    pub fn get_method(&self, name: String) -> Option<&LuaFunction> {
        debug!("Getting method: {:?}", name);
        self.methods.get(name.as_str())
    }
}

impl UserData for JsonRpcRouter {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua: &Lua, (): ()| Ok(JsonRpcRouter::new()));

        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this: &Self, ()| {
            Ok(format!("Server: {:?}", this.methods))
        });

        methods.add_method_mut(
            "add_method",
            |_lua: &Lua, this: &mut JsonRpcRouter, (name, callback): (String, LuaFunction)| {
                debug!("Adding method: {:?}", name);
                this.add_method(name, callback);
                Ok(())
            },
        );
    }
}
