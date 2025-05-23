use log::info;
use mlua::prelude::LuaTable;
use mlua::{IntoLua, IntoLuaMulti, Lua, LuaSerdeExt, Nil, Result, UserData, UserDataMethods};
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use serde_json::{from_str, Value};

pub fn inject_module(lua: &Lua, table: &LuaTable) -> Result<()> {
    let m = lua.create_table()?;
    m.set(
        "client",
        lua.create_function(|_lua: &Lua, _url: String| {
            _Requests::new().map(|requests| _lua.create_userdata(requests))
        })?,
    )?;

    m.set(
        "get",
        lua.create_function(|lua: &Lua, url: String| {
            info!("GET {}", url);
            match reqwest::blocking::get(&url) {
                Ok(response) => map_blocking_response(response).into_lua_multi(lua),
                Err(e) => (Nil, format!("Error: {}", e)).into_lua_multi(lua),
            }
        })?,
    )?;

    table.set("requests", m)?;
    Ok(())
}

struct _Response {
    status: u16,
    headers: HeaderMap,
    body: String,
}

impl _Response {
    fn new(status: u16, headers: HeaderMap, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

fn map_blocking_response(response: Response) -> _Response {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response.text().unwrap();
    _Response::new(status, headers, body)
}

impl UserData for _Response {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_status", |_, this, _: ()| Ok(this.status));

        methods.add_method("get_headers", |lua, this, _: ()| {
            let headers = lua.create_table().unwrap();
            for (key, value) in this.headers.iter() {
                headers.set(key.as_str(), value.to_str().unwrap()).unwrap();
            }
            Ok(headers)
        });

        methods.add_method("get_header_value", |lua: &Lua, this, key: String| {
            let value = this.headers.get(key.as_str()).unwrap().to_str().unwrap();
            Ok(value.into_lua(lua).unwrap())
        });

        methods.add_method("get_text", |_, this, _: ()| Ok(this.body.clone()));

        methods.add_method("get_json", |lua, this, _: ()| {
            let serde_value = from_str::<Value>(&this.body).unwrap();
            let lua_value = lua.to_value(&serde_value).unwrap();
            Ok(lua_value)
        });
    }
}

struct _Requests {
    client: reqwest::blocking::Client,
}

impl _Requests {
    fn new() -> Result<_Requests> {
        Ok(Self {
            client: reqwest::blocking::Client::new(),
        })
    }
}

impl UserData for _Requests {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |lua: &Lua, this: &_Requests, url: String| {
            info!("GET {}", url);
            match this.client.get(&url).send() {
                Ok(response) => map_blocking_response(response).into_lua_multi(lua),
                Err(e) => (Nil, format!("Error: {}", e)).into_lua_multi(lua),
            }
        });

        methods.add_method("post", |lua: &Lua, this, (url, body): (String, String)| {
            info!("POST {}", url);
            match this.client.post(&url).body(body).send() {
                Ok(response) => map_blocking_response(response).into_lua_multi(lua),
                Err(e) => (Nil, format!("Error: {}", e)).into_lua_multi(lua),
            }
        });

        methods.add_method("put", |lua: &Lua, this, (url, body): (String, String)| {
            info!("PUT {}", url);
            match this.client.put(&url).body(body).send() {
                Ok(response) => map_blocking_response(response).into_lua_multi(lua),
                Err(e) => (Nil, format!("Error: {}", e)).into_lua_multi(lua),
            }
        });

        methods.add_method("delete", |lua: &Lua, this, url: String| {
            info!("DELETE {}", url);
            match this.client.delete(&url).send() {
                Ok(response) => map_blocking_response(response).into_lua_multi(lua),
                Err(e) => (Nil, format!("Error: {}", e)).into_lua_multi(lua),
            }
        });
    }
}
