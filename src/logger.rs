use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::append::Append;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;
use mlua::prelude::{LuaTable, LuaValue};
use mlua::{Lua, LuaSerdeExt, Result};
use serde::{Deserialize, Serialize};

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;
    m.set("init_config", lua.create_function(init_config)?)?;
    m.set("debug", lua.create_function(debug)?)?;
    m.set("info", lua.create_function(info)?)?;
    m.set("warn", lua.create_function(warn)?)?;
    m.set("error", lua.create_function(error)?)?;

    table.set("logger", m)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct LoggerConfig {
    level: LevelFilter,
    pattern: String,
    file: Option<String>,
}

fn init_config(lua: &Lua, props: LuaValue) -> Result<()> {
    let props: LoggerConfig = lua.from_value(props).expect("Failed to load logger config");
    info!("Initializing logger with config: {:?}", props);

    // Create a console appender with pattern formatting
    let appender: Box<dyn Append> = match props.file {
        Some(file) => Box::new(
            FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(&props.pattern)))
                .build(file)
                .expect("Failed to create file appender"),
        ),
        None => Box::new(
            ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new(&props.pattern)))
                .build(),
        ),
    };

    // Build the logging configuration
    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(props.level)))
                .build("appender", appender),
        )
        .build(Root::builder().appender("appender").build(props.level))
        .unwrap();

    log4rs::init_config(config).unwrap();

    Ok(())
}

fn debug(_: &Lua, string: String) -> Result<()> {
    log::debug!("{}", string);

    Ok(())
}

fn info(_: &Lua, string: String) -> Result<()> {
    log::info!("{}", string);

    Ok(())
}

fn warn(_: &Lua, string: String) -> Result<()> {
    log::warn!("{}", string);

    Ok(())
}

fn error(_: &Lua, string: String) -> Result<()> {
    log::error!("{}", string);

    Ok(())
}
