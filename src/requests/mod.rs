mod blocking_http_client;
mod http_request_options;
mod http_response;

use crate::requests::http_request_options::Url;
use blocking_http_client::BlockingHttpClient;
use http_response::HttpResponse;
use log::info;
use mlua::prelude::{LuaNil, LuaResult, LuaTable};
use mlua::{IntoLuaMulti, Lua};
use reqwest::Error;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;

    m.set(
        "BlockingHttpClient",
        lua.create_proxy::<BlockingHttpClient>()?,
    )?;

    m.set(
        "get",
        lua.create_function(|lua: &Lua, url: Url| match get(lua, url) {
            Ok(response) => response.into_lua_multi(lua),
            Err(e) => (LuaNil, e.to_string()).into_lua_multi(lua),
        })?,
    )?;

    table.set("requests", m)?;
    Ok(())
}

fn get(_lua: &Lua, url: Url) -> Result<HttpResponse, Error> {
    info!("GET {:?}", url);
    reqwest::blocking::get(url.0).map(HttpResponse::from_response)
}
