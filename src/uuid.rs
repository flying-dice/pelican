use mlua::prelude::LuaTable;
use mlua::{Lua, Result};
use uuid::Uuid;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("v4", lua.create_function(v4)?)?;

    table.set("uuid", m)?;

    Ok(())
}

fn v4(_: &Lua, (): ()) -> Result<String> {
    let uuid = Uuid::new_v4();
    Ok(uuid.to_string())
}
