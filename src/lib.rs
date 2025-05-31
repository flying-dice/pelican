pub mod json;
mod json_rpc;
mod json_schema;
mod logger;
mod module_config;
mod requests;
mod sqlite;
pub mod uuid;
mod web;

use log::LevelFilter::Trace;
use log::{error, info, LevelFilter};
use mlua::prelude::{LuaFunction, LuaResult, LuaTable};
use mlua::Lua;
use module_config::ModuleConfig;
use std::env;
use std::path::PathBuf;

#[mlua::lua_module]
pub fn pelican(lua: &Lua) -> LuaResult<LuaTable> {
    let module_config: ModuleConfig = lua
        .globals()
        .get::<ModuleConfig>("PELICAN")
        .unwrap_or_default();

    let logger_level: LevelFilter = module_config.logger_level.unwrap_or(Trace);

    match logger::init_config(get_logger_file_path(lua)?, logger_level) {
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

fn get_logger_file_path(lua: &Lua) -> LuaResult<PathBuf> {
    if let Ok(writedir) = get_lfs_writedir(lua) {
        return Ok(PathBuf::from(writedir).join("Logs/pelican.log"));
    }

    if let Ok(current_dir) = env::current_dir() {
        return Ok(current_dir.join("pelican.log"));
    }

    Ok("./pelican.log".into())
}

fn get_lfs_writedir(lua: &Lua) -> LuaResult<String> {
    lua.globals()
        .get::<LuaTable>("lfs")?
        .get::<LuaFunction>("writedir")?
        .call(())
}
