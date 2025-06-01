use crate::requests::http_request_options::HttpRequestOptions;
use crate::requests::http_response::HttpResponse;
use log::info;
use mlua::Error::RuntimeError;
use mlua::{
    IntoLuaMulti, Lua, MultiValue as LuaMultiValue, Result as LuaResult, UserData, UserDataMethods,
    Value as LuaValue,
};
use reqwest::blocking::{RequestBuilder, Response};

pub struct BlockingHttpClient {
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
        options: Option<HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("GET {}", url);
        match send_request(self.client.get(&url), options) {
            Ok(response) => HttpResponse::from_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn post(
        &self,
        lua: &Lua,
        url: String,
        body: String,
        options: Option<HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("POST {}", url);
        match send_request(self.client.post(&url).body(body), options) {
            Ok(response) => HttpResponse::from_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn put(
        &self,
        lua: &Lua,
        url: String,
        body: String,
        options: Option<HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("PUT {}", url);
        match send_request(self.client.put(&url).body(body), options) {
            Ok(response) => HttpResponse::from_response(response).into_lua_multi(lua),
            Err(e) => (LuaValue::Nil, e.to_string()).into_lua_multi(lua),
        }
    }

    fn delete(
        &self,
        lua: &Lua,
        url: String,
        options: Option<HttpRequestOptions>,
    ) -> LuaResult<LuaMultiValue> {
        info!("DELETE {}", url);
        match send_request(self.client.delete(&url), options) {
            Ok(response) => HttpResponse::from_response(response).into_lua_multi(lua),
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
             (url, options): (String, Option<HttpRequestOptions>)| {
                this.get(lua, url, options)
            },
        );

        methods.add_method(
            "post",
            |lua: &Lua,
             this,
             (url, body, options): (String, String, Option<HttpRequestOptions>)| {
                this.post(lua, url, body, options)
            },
        );

        methods.add_method(
            "put",
            |lua: &Lua,
             this,
             (url, body, options): (String, String, Option<HttpRequestOptions>)| {
                this.put(lua, url, body, options)
            },
        );

        methods.add_method(
            "delete",
            |lua: &Lua, this, (url, options): (String, Option<HttpRequestOptions>)| {
                this.delete(lua, url, options)
            },
        );
    }
}

fn send_request(
    builder: RequestBuilder,
    options: Option<HttpRequestOptions>,
) -> mlua::Result<Response> {
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
