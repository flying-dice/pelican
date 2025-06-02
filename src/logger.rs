use log::{debug, error, info, warn};
use mlua::prelude::{LuaMetaMethod, LuaTable};
use mlua::{Lua, Result, UserData, UserDataMethods};

struct _Logger {
    ns: String,
}

impl _Logger {
    pub fn new(ns: String) -> Self {
        _Logger { ns }
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
        methods.add_function("new", |_lua: &Lua, ns: String| Ok(_Logger::new(ns)));

        methods.add_meta_method(LuaMetaMethod::ToString, |_: &Lua, this, (): ()| {
            Ok(format!("Logger({})", this.ns))
        });

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
    m.set("Logger", lua.create_proxy::<_Logger>()?)?;

    table.set("logger", m)?;

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
