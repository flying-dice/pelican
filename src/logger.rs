use log::{error, info};
use mlua::prelude::LuaString;
use mlua::Lua;
use mlua::MultiValue;
use mlua::Nil;
use mlua::{Error, IntoLuaMulti};

pub fn configure_log4rs(lua: &Lua, lstring: &LuaString) -> Result<MultiValue, Error> {
    let log4rs_config_path = match lstring.to_str().map(|s| s.to_string()) {
        Ok(s) => {
            info!("Configuring logger with file: {:?}", s);
            s
        }
        Err(e) => {
            error!("Error converting Lua string to Rust string: {:?}", e);
            return (
                Nil,
                "Error converting Lua string to Rust string".to_string(),
            )
                .into_lua_multi(lua);
        }
    };

    match log4rs::init_file(&log4rs_config_path, Default::default()) {
        Ok(_) => {
            info!("Logger configured with file: {:?}", log4rs_config_path);
            (true, Nil).into_lua_multi(lua)
        }
        Err(e) => {
            error!("Error configuring logger: {:?}", e);
            (Nil, format!("Error configuring logger: {:?}", e)).into_lua_multi(lua)
        }
    }
}
