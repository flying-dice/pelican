mod collections;
mod json;
mod json_rpc;
mod json_schema;
mod logger;
mod requests;
mod sqlite;
mod uuid;
mod web;

use mlua::prelude::LuaTable;
use mlua::{Lua, Result};

#[mlua::lua_module]
pub fn pelican(lua: &Lua) -> Result<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("name", "pelican")?;
    exports.set("version", "0.1.0")?;

    json::inject_module(lua, &exports)?;
    logger::inject_module(lua, &exports)?;
    uuid::inject_module(lua, &exports)?;
    web::inject_module(lua, &exports)?;
    json_schema::inject_module(lua, &exports)?;
    requests::inject_module(lua, &exports)?;
    sqlite::inject_module(lua, &exports)?;
    collections::inject_module(lua, &exports)?;

    Ok(exports)
}
