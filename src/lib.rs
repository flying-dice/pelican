pub mod json;
mod json_rpc;
mod json_schema;
mod logger;
mod requests;
mod sqlite;
pub mod uuid;
mod web;

use log::{error, info, LevelFilter};
use mlua::prelude::{LuaFunction, LuaTable, LuaValue};
use mlua::{Lua, LuaSerdeExt, Result};
use std::env;
use std::path::PathBuf;

#[mlua::lua_module]
pub fn pelican(lua: &Lua) -> Result<LuaTable> {
    match logger::init_config(
        get_logger_file(lua)?,
        get_logger_level(lua).unwrap_or(LevelFilter::Trace),
    ) {
        Ok(_) => info!("Logger initialized successfully"),
        Err(e) => error!("Failed to initialize logger: {}", e),
    };

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

    Ok(exports)
}

fn get_logger_file(lua: &Lua) -> Result<PathBuf> {
    match get_lfs_writedir(lua) {
        Ok(writedir) => Ok(PathBuf::from(writedir).join("Logs/pelican.log")),
        Err(_err) => Ok(env::current_dir()?.join("pelican.log")),
    }
}

fn get_lfs_writedir(lua: &Lua) -> Result<String> {
    lua.globals()
        .get::<LuaTable>("lfs")?
        .get::<LuaFunction>("writedir")?
        .call(())
}

fn get_logger_level(lua: &Lua) -> Result<LevelFilter> {
    match lua
        .globals()
        .get::<LuaTable>("PELICAN")?
        .get::<LuaValue>("logger_level")
    {
        Ok(level) => lua.from_value::<LevelFilter>(level),
        Err(_) => Ok(LevelFilter::Info),
    }
}
