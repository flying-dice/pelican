use crate::web::AppData;
use log::{debug, error, info};
use mlua::prelude::{LuaError, LuaFunction, LuaResult, LuaTable, LuaValue};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};
use serde_json::Value::Null;
use std::string::ToString;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

const JSON_RPC_VERSION: &str = "2.0";
const JSON_RPC_METHOD_NOT_FOUND: i16 = -32601;
const JSON_RPC_INTERNAL_ERROR: i16 = -32603;
const JSON_RPC_PARSE_ERROR: i16 = -32700;
const JSON_RPC_INVALID_REQUEST: i16 = -32600;
const JSON_RPC_INVALID_PARAMS: i16 = -32602;

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub result: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
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
            data.rpc_queue.push_back(AppRequest {
                request,
                response_sender: Some(sender),
            });
            Ok(Some(receiver))
        }
        None => {
            debug!("Adding notification to queue");
            data.rpc_queue.push_back(AppRequest {
                request,
                response_sender: None,
            });
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum ProcessRpcError {
    LuaError(LuaError),
    SerdeError(serde_json::Error),
}

impl ProcessRpcError {
    fn name(&self) -> String {
        match self {
            ProcessRpcError::LuaError(_) => "LuaError".to_string(),
            ProcessRpcError::SerdeError(_) => "SerdeError".to_string(),
        }
    }

    fn to_str(&self) -> String {
        match self {
            ProcessRpcError::LuaError(err) => {
                let msg = err.to_string();

                msg.split("\nstack traceback:")
                    .next()
                    .unwrap_or(&*msg)
                    .to_string()
            }
            ProcessRpcError::SerdeError(err) => format!("Serde error: {}", err),
        }
    }
}

pub struct AppRequest {
    pub request: JsonRpcRequest,
    pub response_sender: Option<oneshot::Sender<JsonRpcResponse>>,
}

pub fn process_request(
    lua: &Lua,
    request: JsonRpcRequest,
    methods: &LuaTable,
) -> Result<Option<JsonRpcResponse>, LuaError> {
    info!("Processing request: {:?}", request);

    debug!("Extracting method from request");

    let method_name = request.method.as_str();

    debug!("Method name: {:?}", method_name);

    let callback: LuaResult<LuaFunction> = methods.get(method_name);

    match callback {
        Ok(callback) => {
            debug!("Found method: {:?}", method_name);

            match process_rpc(lua, &callback, &request) {
                Ok(response) => Ok(Some(JsonRpcResponse {
                    jsonrpc: JSON_RPC_VERSION.to_string(),
                    id: response.id,
                    result: response.result,
                    error: response.error,
                })),
                Err(err) => {
                    error!("Error processing request: {:?}", err);
                    Ok(Some(JsonRpcResponse {
                        jsonrpc: JSON_RPC_VERSION.to_string(),
                        id: request.id.clone().unwrap_or("notification".to_string()),
                        result: None,
                        error: Some(serde_json::json!({
                            "code": JSON_RPC_INTERNAL_ERROR,
                            "message": err.name(),
                            "data": err.to_str(),
                        })),
                    }))
                }
            }
        }
        Err(_) => {
            debug!("Method not found: {:?}", method_name);

            match request.id {
                Some(id) => Ok(Some(JsonRpcResponse {
                    jsonrpc: JSON_RPC_VERSION.to_string(),
                    id: id.clone(),
                    result: None,
                    error: Some(serde_json::json!({
                        "code": JSON_RPC_METHOD_NOT_FOUND,
                        "message": format!("Method not found: {}", method_name),
                    })),
                })),
                None => Ok(None),
            }
        }
    }
}

pub fn process_rpc(
    lua: &Lua,
    callback: &LuaFunction,
    request: &JsonRpcRequest,
) -> Result<JsonRpcResponse, ProcessRpcError> {
    info!(
        "Processing request: {:?} with callback {:?}",
        request, callback
    );
    let luav = lua.to_value(&request).map_err(ProcessRpcError::LuaError)?;

    let lua_table = luav.as_table().unwrap();

    // Remove the id field from the table sent to the Lua function.
    if request.id.is_none() {
        lua_table
            .raw_remove("id")
            .map_err(ProcessRpcError::LuaError)?;
    }

    let params = lua_table
        .get::<LuaValue>("params")
        .map_err(ProcessRpcError::LuaError)?;

    info!("Calling callback with args: {:?}", lua_table);
    let result = callback
        .call::<Value>(params)
        .map_err(ProcessRpcError::LuaError)?;

    let serde_value = serde_json::to_value(&result).map_err(ProcessRpcError::SerdeError)?;

    let mut response = JsonRpcResponse {
        jsonrpc: JSON_RPC_VERSION.to_string(),
        id: request.id.clone().unwrap_or("notification".to_string()),
        result: Some(serde_value),
        error: None,
    };

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
