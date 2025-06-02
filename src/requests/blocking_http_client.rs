use crate::requests::http_request_options::{HttpRequestOptions, Url};
use crate::requests::http_response::HttpResponse;
use log::info;
use mlua::prelude::LuaNil;
use mlua::{IntoLuaMulti, Lua, MetaMethod, UserData, UserDataMethods};
use reqwest::blocking::RequestBuilder;
use reqwest::Error;

pub struct BlockingHttpClient {
    client: reqwest::blocking::Client,
}

impl BlockingHttpClient {
    fn new() -> BlockingHttpClient {
        info!("Creating new BlockingHttpClient");
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get(&self, url: Url, options: Option<HttpRequestOptions>) -> Result<HttpResponse, Error> {
        info!("GET {:?}", url);
        send_request(self.client.get(url.0), options)
    }

    fn post(
        &self,
        url: Url,
        body: String,
        options: Option<HttpRequestOptions>,
    ) -> Result<HttpResponse, Error> {
        info!("POST {:?}", url);
        send_request(self.client.post(url.0).body(body), options)
    }

    fn put(
        &self,
        url: Url,
        body: String,
        options: Option<HttpRequestOptions>,
    ) -> Result<HttpResponse, Error> {
        info!("PUT {:?}", url);
        send_request(self.client.put(url.0).body(body), options)
    }

    fn delete(&self, url: Url, options: Option<HttpRequestOptions>) -> Result<HttpResponse, Error> {
        info!("DELETE {:?}", url);
        send_request(self.client.delete(url.0), options)
    }
}

impl UserData for BlockingHttpClient {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua: &Lua, (): ()| Ok(BlockingHttpClient::new()));
        methods.add_meta_method(MetaMethod::ToString, |_: &Lua, this, (): ()| {
            Ok(format!("BlockingHttpClient({:p})", this))
        });

        methods.add_method(
            "get",
            |lua: &Lua,
             this: &BlockingHttpClient,
             (url, options): (Url, Option<HttpRequestOptions>)| {
                match this.get(url, options) {
                    Ok(response) => response.into_lua_multi(lua),
                    Err(e) => (LuaNil, e.to_string()).into_lua_multi(lua),
                }
            },
        );

        methods.add_method(
            "post",
            |_lua: &Lua, this, (url, body, options): (Url, String, Option<HttpRequestOptions>)| {
                match this.post(url, body, options) {
                    Ok(response) => response.into_lua_multi(_lua),
                    Err(e) => (LuaNil, e.to_string()).into_lua_multi(_lua),
                }
            },
        );

        methods.add_method(
            "put",
            |_lua: &Lua, this, (url, body, options): (Url, String, Option<HttpRequestOptions>)| {
                match this.put(url, body, options) {
                    Ok(response) => response.into_lua_multi(_lua),
                    Err(e) => (LuaNil, e.to_string()).into_lua_multi(_lua),
                }
            },
        );

        methods.add_method(
            "delete",
            |_lua: &Lua, this, (url, options): (Url, Option<HttpRequestOptions>)| match this
                .delete(url, options)
            {
                Ok(response) => response.into_lua_multi(_lua),
                Err(e) => (LuaNil, e.to_string()).into_lua_multi(_lua),
            },
        );
    }
}

fn send_request(
    builder: RequestBuilder,
    options: Option<HttpRequestOptions>,
) -> Result<HttpResponse, Error> {
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

    builder.send().map(HttpResponse::from_response)
}
