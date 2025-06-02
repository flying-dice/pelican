//! Lua module for generating UUIDs
use mlua::prelude::{LuaResult, LuaTable};
use mlua::{Lua, Result};
use uuid::Uuid;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("v4", lua.create_function(v4)?)?;
    m.set("v7", lua.create_function(v7)?)?;

    table.set("uuid", m)?;

    Ok(())
}

fn v4(_: &Lua, (): ()) -> LuaResult<String> {
    let uuid = Uuid::new_v4();
    Ok(uuid.to_string())
}

fn v7(_: &Lua, (): ()) -> LuaResult<String> {
    let uuid = Uuid::now_v7();
    Ok(uuid.to_string())
}
