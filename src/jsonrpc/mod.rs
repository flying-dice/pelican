pub mod router;
pub mod server;

use crate::jsonrpc::router::JsonRpcRouter;
use crate::jsonrpc::server::JsonRpcServer;
use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;
use serde::{Deserialize, Serialize};

pub const JSON_RPC_VERSION: &str = "2.0";
pub const JSON_RPC_METHOD_NOT_FOUND: i16 = -32601;
pub const JSON_RPC_INTERNAL_ERROR: i16 = -32603;
// const JSON_RPC_PARSE_ERROR: i16 = -32700;
// const JSON_RPC_INVALID_REQUEST: i16 = -32600;
// const JSON_RPC_INVALID_PARAMS: i16 = -32602;

/// https://www.jsonrpc.org/specification#request_object:~:text=aspects%20of%201.0.-,4%20Request%20object,-A%20rpc%20call
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: Option<String>,
    pub params: Option<serde_json::Value>,
}

/// https://www.jsonrpc.org/specification#request_object:~:text=NOT%20be%20included.-,5.1%20Error%20object,-When%20a%20rpc
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i16,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// https://www.jsonrpc.org/specification#request_object:~:text=method%27s%20expected%20parameters.-,5%20Response%20object,-When%20a%20rpc
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;

    m.set("JsonRpcServer", lua.create_proxy::<JsonRpcServer>()?)?;
    m.set("JsonRpcRouter", lua.create_proxy::<JsonRpcRouter>()?)?;

    table.set("jsonrpc", m)?;

    Ok(())
}
