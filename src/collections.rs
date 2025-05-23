use mlua::prelude::*;
use mlua::{UserData, UserDataMethods, Value};

#[derive(Debug, Clone)]
pub struct _Vec {
    data: Vec<LuaValue>,
}

impl _Vec {
    pub fn new(_: &Lua, table: Option<LuaTable>) -> LuaResult<Self> {
        let mut data = Vec::new();

        if let Some(table) = table {
            for value in table.sequence_values::<Value>() {
                data.push(value?);
            }
        }

        Ok(_Vec { data })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<LuaValue> {
        self.data.get(index).cloned()
    }

    pub fn push(&mut self, value: LuaValue) {
        self.data.push(value);
    }

    pub fn set(&mut self, index: usize, value: LuaValue) {
        self.data[index] = value
    }

    pub fn pop(&mut self) -> LuaResult<Option<LuaValue>> {
        Ok(self.data.pop())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn for_each(&self, func: LuaFunction) -> LuaResult<()> {
        for value in &self.data {
            func.call::<()>(value)?;
        }
        Ok(())
    }

    pub fn map(&self, lua: &Lua, func: LuaFunction) -> LuaResult<_Vec> {
        let mut result = _Vec::new(lua, None)?;
        for value in &self.data {
            let mapped_value = func.call::<LuaValue>(value)?;
            result.push(mapped_value);
        }
        Ok(result)
    }

    pub fn reverse(&mut self, lua: &Lua) -> LuaResult<()> {
        self.data.reverse();
        Ok(())
    }

    pub fn filter(&self, lua: &Lua, func: LuaFunction) -> LuaResult<_Vec> {
        let mut result = _Vec::new(lua, None)?;
        for value in &self.data {
            if func.call::<bool>(value)? {
                result.push(value.clone());
            }
        }
        Ok(result)
    }

    pub fn reduce(
        &self,
        lua: &Lua,
        func: LuaFunction,
        accumulator: LuaValue,
    ) -> LuaResult<LuaValue> {
        let mut accumulator = accumulator;
        for value in &self.data {
            accumulator = func.call::<LuaValue>((accumulator, value))?;
        }
        Ok(accumulator)
    }

    pub fn to_lua_table(&self, lua: &Lua) -> LuaResult<LuaTable> {
        let table = lua.create_table()?;
        for (i, value) in self.data.iter().enumerate() {
            table.set(i + 1, value)?;
        }
        Ok(table)
    }
}

impl UserData for _Vec {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("len", |_lua, this, ()| Ok(this.len()));
        methods.add_method("get", |_lua, this, index: usize| Ok(this.get(index)));
        methods.add_method_mut("set", |_lua, this, (index, value): (usize, LuaValue)| {
            this.set(index, value);
            Ok(())
        });
        methods.add_method_mut("push", |_lua, this, value: LuaValue| {
            this.push(value);
            Ok(())
        });
        methods.add_method_mut("pop", |_lua, this, (): ()| Ok(this.pop()));
        methods.add_method_mut("clear", |_lua, this, (): ()| {
            this.clear();
            Ok(())
        });
        methods.add_method_mut("for_each", |_lua, this, func: LuaFunction| {
            this.for_each(func)?;
            Ok(())
        });
        methods.add_method("map", |lua, this, func: LuaFunction| {
            let result = this.map(lua, func)?;
            Ok(result)
        });
        methods.add_method("filter", |lua, this, func: LuaFunction| {
            let result = this.filter(lua, func)?;
            Ok(result)
        });
        methods.add_method(
            "reduce",
            |lua, this, (func, accumulator): (LuaFunction, LuaValue)| {
                let result = this.reduce(lua, func, accumulator)?;
                Ok(result)
            },
        );
        methods.add_method_mut("reverse", |lua, this, (): ()| {
            this.reverse(lua)?;
            Ok(())
        });

        methods.add_method("to_lua_table", |lua, this, ()| this.to_lua_table(lua));
    }
}

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;

    let vec_metatable = lua.create_table()?;
    vec_metatable.set(
        "__call",
        lua.create_function(|lua, (_, initial_table): (LuaTable, Option<LuaTable>)| {
            _Vec::new(lua, initial_table)
        })?,
    )?;

    let vec_cls = lua.create_table()?;

    m.set(
        "Vec",
        lua.create_function(|lua, initial_table: Option<LuaTable>| _Vec::new(lua, initial_table))?,
    )?;

    vec_cls.set_metatable(Some(vec_metatable));

    m.set("Vec", vec_cls)?;

    table.set("collections", m)?;

    Ok(())
}
