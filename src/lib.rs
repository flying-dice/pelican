pub mod json;
mod jsonrpc;
mod jsonschema;
mod logger;
mod module_config;
mod requests;
mod sqlite;
pub mod uuid;

use log::LevelFilter::Warn;
use log::{error, info, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;
use mlua::prelude::{LuaFunction, LuaResult, LuaTable};
use mlua::{ExternalError, Lua};
use module_config::ModuleConfig;
use std::env;
use std::path::PathBuf;

#[mlua::lua_module]
pub fn pelican(lua: &Lua) -> LuaResult<LuaTable> {
    let module_config: ModuleConfig = lua
        .globals()
        .get::<ModuleConfig>("PELICAN")
        .unwrap_or_default();

    let logger_level: LevelFilter = module_config.logger_level.unwrap_or(Warn);

    match init_config(get_logger_file_path(lua)?, logger_level) {
        Ok(_) => info!("Logger initialized successfully"),
        Err(e) => error!("Failed to initialize logger: {}", e),
    };

    let exports = lua.create_table()?;

    exports.set("name", "pelican")?;
    exports.set("version", "0.1.0")?;

    json::inject_module(lua, &exports)?;
    logger::inject_module(lua, &exports)?;
    uuid::inject_module(lua, &exports)?;
    jsonrpc::inject_module(lua, &exports)?;
    jsonschema::inject_module(lua, &exports)?;
    requests::inject_module(lua, &exports)?;
    sqlite::inject_module(lua, &exports)?;

    Ok(exports)
}

pub fn init_config(file: PathBuf, level: LevelFilter) -> mlua::Result<()> {
    let appender = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {t} - {m}{n}")))
        .build(file)
        .map_err(|e| e.into_lua_err())?;

    // Build the logging configuration
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("appender", Box::new(appender)),
        )
        .build(Root::builder().appender("appender").build(level))
        .map_err(|e| e.into_lua_err())?;

    log4rs::init_config(config).map_err(|e| e.into_lua_err())?;

    Ok(())
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
