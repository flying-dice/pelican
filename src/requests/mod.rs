mod blocking_http_client;
mod http_header_map;
mod http_request_options;
mod http_response;

use blocking_http_client::BlockingHttpClient;
use http_header_map::HttpHeaderMap;
use http_response::HttpResponse;
use log::info;
use mlua::prelude::{LuaMultiValue, LuaResult, LuaTable, LuaValue};
use mlua::{IntoLuaMulti, Lua, Result};
use reqwest::blocking::Response;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;

    m.set(
        "BlockingHttpClient",
        lua.create_proxy::<BlockingHttpClient>()?,
    )?;

    m.set(
        "get",
        lua.create_function(|lua: &Lua, url: String| get(lua, url))?,
    )?;

    table.set("requests", m)?;
    Ok(())
}

fn get(lua: &Lua, url: String) -> LuaResult<LuaMultiValue> {
    info!("GET {}", url);
    match reqwest::blocking::get(&url) {
        Ok(response) => map_blocking_response(response).into_lua_multi(lua),
        Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
    }
}

fn map_blocking_response(response: Response) -> HttpResponse {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response.text().unwrap();
    HttpResponse::new(status, HttpHeaderMap(headers), body)
}
