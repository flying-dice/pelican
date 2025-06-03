use actix_web::dev::ServerHandle;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json, Payload};
use actix_ws::{Message, Session};

use crate::jsonrpc::router::JsonRpcRouter;
use crate::jsonrpc::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, JSON_RPC_INTERNAL_ERROR,
    JSON_RPC_METHOD_NOT_FOUND, JSON_RPC_VERSION,
};
use actix_web::{get, middleware, post, App, HttpRequest, HttpResponse, HttpServer};
use log::{debug, error, info, warn};
use mlua::prelude::{LuaError, LuaNil, LuaValue};
use mlua::Error::RuntimeError;
use mlua::{
    FromLua, IntoLuaMulti, Lua, LuaSerdeExt, MetaMethod, UserData, UserDataMethods, UserDataRef,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::task::spawn_local;
use tokio::time::timeout;

pub struct AppRequest {
    pub request: JsonRpcRequest,
    pub response_sender: Option<oneshot::Sender<JsonRpcResponse>>,
}

#[derive(Default)]
pub struct AppData {
    pub rpc_queue: VecDeque<AppRequest>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Health {
    name: String,
    status: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

impl FromLua for ServerConfig {
    fn from_lua(value: LuaValue, lua: &Lua) -> mlua::Result<Self> {
        let value = lua.from_value(value)?;
        serde_json::from_value::<ServerConfig>(value).map_err(LuaError::external)
    }
}

pub struct JsonRpcServer {
    config: ServerConfig,
    handle: ServerHandle,
    app_data: Data<Mutex<AppData>>,
}

impl JsonRpcServer {
    fn new(config: ServerConfig) -> Result<Self, actix_web::Error> {
        let app_data = Data::new(Mutex::new(AppData::default()));
        let app_data_2 = app_data.clone();

        let host = config.host.clone();
        let port = config.port;

        let server = HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(get_ws)
                .service(get_health)
                .service(post_rpc)
                .app_data(Data::clone(&app_data_2))
        })
        .workers(1)
        .bind((host, port))?
        .run();

        let handle = server.handle();

        thread::spawn(move || {
            info!("Starting server in new thread");
            actix_web::rt::System::new().block_on(async {
                match server.await {
                    Ok(_) => info!("Server stopped!"),
                    Err(e) => error!("Error running server: {:?}", e),
                }
            });
        });

        Ok(Self {
            config,
            handle,
            app_data,
        })
    }

    fn stop(&self, graceful: Option<bool>) -> Result<(), tokio::io::Error> {
        info!("Stopping server...");

        let graceful = graceful.unwrap_or(false);

        Runtime::new()?.block_on(async move {
            self.handle.stop(graceful).await;
            info!("Server fully stopped (blocking)");
        });

        Ok(())
    }
}

impl Drop for JsonRpcServer {
    fn drop(&mut self) {
        info!("Dropping server...");
        let handle = self.handle.clone();
        Runtime::new().unwrap().block_on(async move {
            handle.stop(false).await;
            info!("Server fully dropped");
        });
    }
}

impl UserData for JsonRpcServer {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua: &Lua, config: ServerConfig| {
            Ok(JsonRpcServer::new(config).map_err(LuaError::external)?)
        });

        methods.add_meta_method(MetaMethod::ToString, |_, this: &Self, ()| {
            Ok(format!("JsonRpcServer({:?})", this.config))
        });

        methods.add_method(
            "process_rpc",
            |lua: &Lua, this: &JsonRpcServer, router: UserDataRef<JsonRpcRouter>| {
                let mut data_guard = this.app_data.lock().map_err(|e| {
                    error!("Error acquiring data lock: {:?}", e);
                    RuntimeError(format!("Error acquiring data lock: {:?}", e))
                })?;

                while let Some(request) = data_guard.rpc_queue.pop_front() {
                    match process_request(lua, router.deref(), request.request) {
                        Ok(response) => match response {
                            Some(response) => match serde_json::to_string(&response) {
                                Ok(response_message) => {
                                    info!("Sending response: {:?}", response_message);

                                    match request.response_sender {
                                        Some(_sender) => {
                                            if _sender.send(response).is_err() {
                                                error!("Failed to send response");
                                            }
                                        }
                                        None => {
                                            info!("Processed notification: {:?}", response);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Error processing request: {:?}", e);
                                }
                            },
                            None => {
                                info!("Processed notification");
                            }
                        },
                        Err(e) => {
                            error!("Failed to process request: {:?}", e);
                        }
                    }
                }

                true.into_lua_multi(lua)
            },
        );

        methods.add_method(
            "stop",
            |_lua: &Lua, this: &JsonRpcServer, graceful: Option<bool>| {
                this.stop(graceful).map_err(LuaError::external)?;
                Ok(())
            },
        );
    }
}

#[post("/rpc")]
async fn post_rpc(
    _req: HttpRequest,
    data: Data<Mutex<AppData>>,
    body: Json<JsonRpcRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut data_guard = data
        .lock()
        .map_err(|e| ErrorInternalServerError(format!("Failed to acquire data lock: {}", e)))?;

    let request = body.into_inner();

    let maybe_receiver = push_rpc_request(&mut data_guard, request);

    drop(data_guard);

    let Some(receiver) = maybe_receiver else {
        return Ok(HttpResponse::Accepted().body("OK"));
    };

    let result = timeout(Duration::from_secs(5), receiver)
        .await
        .map_err(ErrorInternalServerError)?;

    let response = result.map_err(ErrorInternalServerError)?;

    let body = serde_json::to_string(&response).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(body))
}

#[get("/ws")]
async fn get_ws(
    req: HttpRequest,
    body: Payload,
    data: Data<Mutex<AppData>>,
) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    info!("WebSocket connection established");

    spawn_local(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(text) => {
                    let message = text.to_string();
                    let Ok(request) = serde_json::from_str::<JsonRpcRequest>(&message) else {
                        error!("Failed to parse request: {}", message);
                        return;
                    };

                    let Ok(mut data_guard) = data.lock() else {
                        error!("Failed to acquire data lock");
                        return;
                    };

                    let maybe_receiver = push_rpc_request(&mut data_guard, request);
                    drop(data_guard);

                    if let Some(receiver) = maybe_receiver {
                        notify_session(session.clone(), receiver)
                            .await
                            .unwrap_or_else(|e| error!("{}", e))
                    }
                }
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        error!("Failed to send pong");
                    }
                }
                Message::Close(reason) => {
                    let _ = session.close(reason).await;
                    break;
                }
                _ => break,
            }
        }
    });

    Ok(response)
}

#[get("/health")]
async fn get_health() -> Json<Health> {
    let health = Health {
        name: "pelican".to_string(),
        status: "OK".to_string(),
        version: "0.3.0".to_string(),
    };

    Json(health)
}

fn process_request(
    lua: &Lua,
    router: &JsonRpcRouter,
    request: JsonRpcRequest,
) -> Result<Option<JsonRpcResponse>, LuaError> {
    debug!("Processing RPC request: {:?}", request);

    let method_name = request.method.clone();

    debug!("Getting method: {:?}", method_name);
    let Some(method) = router.get_method(request.method) else {
        warn!("Method not found!");
        return match request.id {
            Some(id) => {
                let error = JsonRpcError {
                    code: JSON_RPC_METHOD_NOT_FOUND,
                    message: format!("Method not found: {}", method_name),
                    data: None,
                };

                let response = JsonRpcResponse {
                    jsonrpc: JSON_RPC_VERSION.to_string(),
                    id,
                    result: None,
                    error: Some(serde_json::to_value(error).map_err(LuaError::external)?),
                };
                Ok(Some(response))
            }
            None => Ok(None),
        };
    };

    debug!("Method found, mapping parameters: {:?}", request.params);
    let params: LuaValue = match request.params {
        Some(params) => lua.to_value(&params).map_err(LuaError::external)?,
        None => LuaNil,
    };

    debug!("Calling Lua method with params: {:?}, {:?}", method, params);

    match method.call::<LuaValue>(params) {
        Ok(result) => match request.id {
            Some(id) => {
                debug!("Method call successful, result: {:?}", result);
                let response = JsonRpcResponse {
                    jsonrpc: JSON_RPC_VERSION.to_string(),
                    id,
                    result: Some(serde_json::to_value(result).map_err(LuaError::external)?),
                    error: None,
                };
                Ok(Some(response))
            }
            None => Ok(None),
        },
        Err(e) => {
            error!("Method call failed: {}", e);

            let msg = e.to_string();

            let msg = msg.split("\nstack traceback:").next().unwrap_or(&*msg);

            match request.id {
                Some(id) => {
                    let error = JsonRpcError {
                        code: JSON_RPC_INTERNAL_ERROR,
                        message: "LuaError".to_string(),
                        data: Some(serde_json::to_value(msg).map_err(LuaError::external)?),
                    };

                    let response = JsonRpcResponse {
                        jsonrpc: JSON_RPC_VERSION.to_string(),
                        id,
                        result: None,
                        error: Some(serde_json::to_value(error).map_err(LuaError::external)?),
                    };
                    Ok(Some(response))
                }
                None => Ok(None),
            }
        }
    }
}

fn push_rpc_request(
    data: &mut AppData,
    request: JsonRpcRequest,
) -> Option<Receiver<JsonRpcResponse>> {
    let request_id = &request.id;

    info!(
        "<< [{}]: '{:?}'",
        request_id.clone().unwrap_or("notification".to_string()),
        request
    );

    match request_id {
        Some(id) => {
            debug!("Adding request to queue with id: {}", id);
            let (sender, receiver) = oneshot::channel::<JsonRpcResponse>();
            data.rpc_queue.push_back(AppRequest {
                request,
                response_sender: Some(sender),
            });
            Some(receiver)
        }
        None => {
            debug!("Adding notification to queue");
            data.rpc_queue.push_back(AppRequest {
                request,
                response_sender: None,
            });
            None
        }
    }
}

async fn notify_session(
    mut session: Session,
    receiver: Receiver<JsonRpcResponse>,
) -> Result<(), String> {
    let response = timeout(Duration::from_secs(5), receiver)
        .await
        .map_err(|e| format!("ERR: TIMEOUT: {:?}", e))?
        .map_err(|e| format!("ERR: FAILED RES: {:?}", e))?;

    let response_body =
        serde_json::to_string(&response).map_err(|e| format!("ERR: RESP SERDE FAILED: {:?}", e))?;

    session
        .text(response_body)
        .await
        .map_err(|e| format!("ERR: RESP SERDE FAILED: {:?}", e))?;

    Ok(())
}
