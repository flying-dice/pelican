use log::info;
use mlua::prelude::{LuaMultiValue, LuaResult, LuaTable, LuaValue};
use mlua::Error::RuntimeError;
use mlua::{
    ExternalError, FromLua, IntoLua, IntoLuaMulti, Lua, LuaSerdeExt, Result, UserData,
    UserDataMethods,
};
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct _HttpRequestOptions {
    pub headers: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
}

impl FromLua for _HttpRequestOptions {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let value = lua.from_value(value)?;
        serde_json::from_value::<_HttpRequestOptions>(value).map_err(|e| e.into_lua_err())
    }
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

struct BlockingHttpClient {
    client: reqwest::blocking::Client,
}

impl BlockingHttpClient {
    fn new() -> LuaResult<BlockingHttpClient> {
        info!("Creating new BlockingHttpClient");
        Ok(Self {
            client: reqwest::blocking::Client::new(),
        })
    }

    fn get(
        &self,
        lua: &Lua,
        url: String,
        options: Option<_HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("GET {}", url);
        match send_request(self.client.get(&url), options) {
            Ok(response) => map_blocking_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn post(
        &self,
        lua: &Lua,
        url: String,
        body: String,
        options: Option<_HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("POST {}", url);
        match send_request(self.client.post(&url).body(body), options) {
            Ok(response) => map_blocking_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn put(
        &self,
        lua: &Lua,
        url: String,
        body: String,
        options: Option<_HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("PUT {}", url);
        match send_request(self.client.put(&url).body(body), options) {
            Ok(response) => map_blocking_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn delete(
        &self,
        lua: &Lua,
        url: String,
        options: Option<_HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("DELETE {}", url);
        match send_request(self.client.delete(&url), options) {
            Ok(response) => map_blocking_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }
}

impl UserData for BlockingHttpClient {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua: &Lua, (): ()| BlockingHttpClient::new());

        methods.add_method(
            "get",
            |lua: &Lua,
             this: &BlockingHttpClient,
             (url, options): (String, Option<_HttpRequestOptions>)| {
                this.get(lua, url, options)
            },
        );

        methods.add_method(
            "post",
            |lua: &Lua,
             this,
             (url, body, options): (String, String, Option<_HttpRequestOptions>)| {
                this.post(lua, url, body, options)
            },
        );

        methods.add_method(
            "put",
            |lua: &Lua,
             this,
             (url, body, options): (String, String, Option<_HttpRequestOptions>)| {
                this.put(lua, url, body, options)
            },
        );

        methods.add_method(
            "delete",
            |lua: &Lua, this, (url, options): (String, Option<_HttpRequestOptions>)| {
                this.delete(lua, url, options)
            },
        );
    }
}

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

fn map_blocking_response(response: Response) -> _Response {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response.text().unwrap();
    _Response::new(status, headers, body)
}

fn send_request(builder: RequestBuilder, options: Option<_HttpRequestOptions>) -> Result<Response> {
    let mut builder = builder;

    if let Some(options) = options {
        if let Some(header_map) = options.headers {
            for (key, value) in header_map {
                builder = builder.header(key, value);
            }
        }

        if let Some(timeout) = options.timeout {
            builder = builder.timeout(std::time::Duration::from_secs(timeout));
        }
    }

    builder.send().map_err(|e| RuntimeError(e.to_string()))
}
