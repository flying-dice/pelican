use crate::app::AppData;
use log::{debug, info};
use mlua::prelude::LuaFunction;
use mlua::{Error, Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

/// https://www.jsonrpc.org/specification#request_object:~:text=aspects%20of%201.0.-,4%20Request%20object,-A%20rpc%20call
#[derive(Debug, Serialize, Deserialize)]
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
    pub data: Option<serde_json::Value>,
}

/// https://www.jsonrpc.org/specification#request_object:~:text=method%27s%20expected%20parameters.-,5%20Response%20object,-When%20a%20rpc
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

pub enum RpcError {
    ParseError,
}

pub fn push_rpc_request(
    data: &mut AppData,
    request_string: &String,
) -> Result<Option<Receiver<JsonRpcResponse>>, RpcError> {
    let request =
        serde_json::from_str::<JsonRpcRequest>(request_string).map_err(|_| RpcError::ParseError)?;

    let request_id = &request.id;

    info!(
        "<< [{}]: '{}'",
        request_id.clone().unwrap_or("notification".to_string()),
        request_string
    );

    match request_id {
        Some(id) => {
            debug!("Adding request to queue with id: {}", id);
            let (sender, receiver) = oneshot::channel::<JsonRpcResponse>();
            data.rpc_response_listeners.insert(id.clone(), sender);
            data.rpc_queue.push_back(request);
            Ok(Some(receiver))
        }
        None => {
            debug!("Adding notification to queue");
            data.rpc_queue.push_back(request);
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum ProcessRpcError {
    LuaError(Error),
    SerdeError(serde_json::Error),
}

pub fn process_rpc(
    lua: &Lua,
    callback: &LuaFunction,
    request: &JsonRpcRequest,
) -> Result<JsonRpcResponse, ProcessRpcError> {
    let luav = lua.to_value(&request).map_err(ProcessRpcError::LuaError)?;

    let lua_table = luav.as_table().unwrap();

    // Remove the id field from the table sent to the Lua function.
    if let None = &request.id {
        lua_table
            .raw_remove("id")
            .map_err(ProcessRpcError::LuaError)?;
    }

    let result = callback
        .call::<Value>(lua_table)
        .map_err(ProcessRpcError::LuaError)?;

    let serde_value = serde_json::to_value(&result).map_err(ProcessRpcError::SerdeError)?;

    let mut response = serde_json::from_value::<JsonRpcResponse>(serde_value)
        .map_err(ProcessRpcError::SerdeError)?;

    if response.result == Some(Null) {
        response.result = None;
    }

    if response.error == Some(Null) {
        response.error = None;
    }

    let response_message = serde_json::to_string(&response).map_err(ProcessRpcError::SerdeError)?;

    info!(">> [{}]: '{}'", response.id, &response_message);

    Ok(response)
}
