use crate::collections::_Vec;
use log::{debug, error, info, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;
use mlua::prelude::{LuaResult, LuaString, LuaTable};
use mlua::{ExternalError, Lua, Result, UserData, UserDataMethods};
use std::path::PathBuf;

struct _Logger {
    ns: String,
}

impl _Logger {
    pub fn new(_: &Lua, ns: LuaString) -> LuaResult<Self> {
        let ns = ns.to_str().map_err(|e| e.into_lua_err())?.to_string();

        Ok(_Logger { ns })
    }

    pub fn debug(&self, msg: String) {
        debug!(target: &self.ns, "{}", msg);
    }

    pub fn info(&self, msg: String) {
        info!(target: &self.ns, "{}", msg);
    }

    pub fn warn(&self, msg: String) {
        warn!(target: &self.ns, "{}", msg);
    }

    pub fn error(&self, msg: String) {
        error!(target: &self.ns, "{}", msg);
    }
}

impl UserData for _Logger {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("debug", |_lua, this, msg: String| {
            this.debug(msg);
            Ok(())
        });

        methods.add_method("info", |_lua, this, msg: String| {
            this.info(msg);
            Ok(())
        });

        methods.add_method("warn", |_lua, this, msg: String| {
            this.warn(msg);
            Ok(())
        });

        methods.add_method("error", |_lua, this, msg: String| {
            this.error(msg);
            Ok(())
        });
    }
}

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;
    m.set("debug", lua.create_function(debug)?)?;
    m.set("info", lua.create_function(info)?)?;
    m.set("warn", lua.create_function(warn)?)?;
    m.set("error", lua.create_function(error)?)?;

    let logger_metatable = lua.create_table()?;
    logger_metatable.set(
        "__call",
        lua.create_function(|lua, (_self, ns): (LuaTable, LuaString)| _Logger::new(lua, ns))?,
    )?;

    let logger_cls = lua.create_table()?;
    logger_cls.set_metatable(Some(logger_metatable));

    m.set("Logger", logger_cls)?;

    table.set("logger", m)?;
    Ok(())
}

pub fn init_config(file: PathBuf, level: LevelFilter) -> Result<()> {
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

fn debug(_: &Lua, (msg, ns): (String, Option<String>)) -> Result<()> {
    match ns {
        Some(namespace) => debug!(target: &namespace, "{}", msg),
        None => debug!("{}", msg),
    }
    Ok(())
}

fn info(_: &Lua, (msg, ns): (String, Option<String>)) -> Result<()> {
    match ns {
        Some(namespace) => info!(target: &namespace, "{}", msg),
        None => info!("{}", msg),
    }
    Ok(())
}

fn warn(_: &Lua, (msg, ns): (String, Option<String>)) -> Result<()> {
    match ns {
        Some(namespace) => warn!(target: &namespace, "{}", msg),
        None => warn!("{}", msg),
    }
    Ok(())
}

fn error(_: &Lua, (msg, ns): (String, Option<String>)) -> Result<()> {
    match ns {
        Some(namespace) => error!(target: &namespace, "{}", msg),
        None => error!("{}", msg),
    }
    Ok(())
}
