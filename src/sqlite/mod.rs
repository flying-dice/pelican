mod sqlite_connection;

use crate::sqlite::sqlite_connection::_SQLiteConnection;
use mlua::prelude::LuaTable;
use mlua::{Lua, Result};

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set("SQLiteConnection", lua.create_proxy::<_SQLiteConnection>()?)?;

    table.set("sqlite", m)?;

    Ok(())
}
