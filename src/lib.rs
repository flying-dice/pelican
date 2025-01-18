mod app;
mod json_rpc;
mod lua_json;

use crate::app::{create_app, AppConfig, AppData};
use crate::json_rpc::process_rpc;
use crate::lua_json::{decode, encode};
use actix_web::dev::ServerHandle;
use actix_web::web::Data;
use log::{error, info, warn};
use mlua::prelude::*;
use mlua::{Function, Lua, Nil, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

#[mlua::lua_module]
pub fn lua_json_rpc(lua: &Lua) -> Result<LuaTable> {
    let exports = lua.create_table()?;

    // Create the shared queue
    let _data: Data<Mutex<AppData>> = Data::new(Mutex::new(AppData {
        rpc_queue: VecDeque::new(),
        rpc_response_listeners: HashMap::new(),
        api_key: None,
    }));

    let data = _data.clone();
    exports.set(
        "start_server",
        lua.create_function(move |lua: &Lua, config: LuaValue| -> Result<Function> {
            let config: AppConfig = serde_json::from_value::<AppConfig>(lua.from_value(config)?)
                .map_err(|e| {
                    error!("Error parsing server config: {:?}", e);
                    mlua::Error::RuntimeError(format!("Error parsing server config: {:?}", e))
                })?;

            match data.lock() {
                Ok(mut data_guard) => {
                    data_guard.api_key = config.api_key.clone();
                }
                Err(e) => {
                    warn!("Error acquiring data lock, failed to set API Key: {:?}", e);
                }
            }

            info!("Setting up server with config {:?}", config);
            let handle: ServerHandle = create_app(data.clone(), &config).map_err(|e| {
                error!("Error creating server: {:?}", e);
                mlua::Error::RuntimeError(format!("Error creating server: {:?}", e))
            })?;

            lua.create_function(move |lua: &Lua, ()| {
                info!("Shutting Down Server");
                handle.stop(false);
                Ok(())
            })
        })?,
    )?;

    let data = _data.clone();
    exports.set(
        "process_rpc",
        lua.create_function(move |lua: &Lua, callback: LuaFunction| {
            let mut data_guard = data.lock().map_err(|e| {
                error!("Error acquiring data lock: {:?}", e);
                mlua::Error::RuntimeError(format!("Error acquiring data lock: {:?}", e))
            })?;

            match data_guard.rpc_queue.pop_front() {
                Some(request) => match process_rpc(lua, &callback, &request) {
                    Ok(response) => {
                        info!("Processed request: {:?}", response);
                        let response_message = serde_json::to_string(&response).map_err(|e| {
                            error!("Error serializing response: {:?}", e);
                            mlua::Error::RuntimeError(format!(
                                "Error serializing response: {:?}",
                                e
                            ))
                        })?;

                        if let Some(sender) = data_guard.rpc_response_listeners.remove(&response.id)
                        {
                            match sender.send(response) {
                                Ok(_) => info!("Published Response: {}", response_message),
                                Err(e) => info!(
                                    "Unable to publish response, likely due to no consumer: {:?}",
                                    e
                                ),
                            }
                        }

                        Ok(())
                    }
                    Err(err) => {
                        error!("Error processing request: {:?}", err);
                        Ok(())
                    }
                },
                None => Ok(()),
            }
        })?,
    )?;

    exports.set(
        "encode",
        lua.create_function(|lua: &Lua, value: LuaValue| match encode(lua, &value) {
            Ok(value) => Ok(value.into_lua_multi(lua)?),
            Err(err) => Ok((Nil, format!("Failed to encode JSON {:?}", err)).into_lua_multi(lua)?),
        })?,
    )?;

    exports.set(
        "decode",
        lua.create_function(|lua: &Lua, json: LuaString| match decode(lua, &json) {
            Ok(value) => Ok(value),
            Err(err) => Ok((Nil, format!("Failed to decode JSON {:?}", err)).into_lua_multi(lua)?),
        })?,
    )?;

    Ok(exports)
}
