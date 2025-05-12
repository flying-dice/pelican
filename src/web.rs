use crate::json_rpc::{
    process_request, process_rpc, push_rpc_request, AppRequest, JsonRpcRequest, JsonRpcResponse,
    RpcError,
};
use actix_web::dev::ServerHandle;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::http::header;
use actix_web::web::{Data, Payload};
use actix_ws::{Message, Session};

use actix_web::{
    get, middleware, post, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::{debug, error, info};
use metrics::gauge;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use mlua::prelude::{LuaFunction, LuaResult, LuaString, LuaTable, LuaValue};
use mlua::{Lua, LuaSerdeExt, UserData, UserDataMethods, UserDataRef};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;
use tokio::time::timeout;

pub fn inject_module(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
    let m = lua.create_table()?;

    m.set("serve", lua.create_function(serve)?)?;
    m.set("router", lua.create_function(router)?)?;

    table.set("web", m)?;

    Ok(())
}

async fn notify_session(
    mut session: Session,
    receiver: Receiver<JsonRpcResponse>,
) -> Result<(), Error> {
    match timeout(Duration::from_secs(5), receiver).await {
        Ok(response) => match response {
            Ok(response) => {
                info!("Received response: {:?}", response);
                match serde_json::to_string(&response) {
                    Ok(response_message) => match session.text(response_message).await {
                        Ok(_) => info!("Sent response to session"),
                        Err(e) => error!("Failed to send response to session: {}", e),
                    },
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                    }
                }
            }
            Err(_) => {
                error!("Failed to receive response");
            }
        },
        Err(_) => {
            error!("Timed out waiting for response");
        }
    }

    Ok(())
}

pub struct AppData {
    pub rpc_queue: VecDeque<AppRequest>,
    pub prometheus_handle: Arc<PrometheusHandle>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Health {
    name: String,
    status: String,
    version: String,
}

#[post("/rpc")]
async fn post_rpc(
    req: HttpRequest,
    data: Data<Mutex<AppData>>,
    body: String,
) -> Result<HttpResponse, Error> {
    let mut data_guard = data
        .lock()
        .map_err(|e| ErrorInternalServerError(format!("Failed to acquire data lock: {}", e)))?;

    let receiver = push_rpc_request(&mut data_guard, &body).map_err(|err| match err {
        RpcError::ParseError => ErrorBadRequest("Failed to parse request"),
    })?;

    drop(data_guard);

    match receiver {
        Some(receiver) => match timeout(Duration::from_secs(5), receiver).await {
            Ok(response) => match response {
                Ok(response) => serde_json::to_string(&response)
                    .map(|s| HttpResponse::Ok().body(s))
                    .map_err(|e| ErrorInternalServerError(e.to_string())),
                Err(_) => Err(ErrorInternalServerError("Failed to receive response")),
            },
            Err(_) => Err(ErrorInternalServerError("Timed out waiting for response")),
        },
        None => Ok(HttpResponse::Accepted().body("OK")),
    }
}

#[get("/ws")]
async fn get_ws(
    req: HttpRequest,
    body: Payload,
    data: Data<Mutex<AppData>>,
) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn({
        async move {
            while let Some(Ok(msg)) = msg_stream.recv().await {
                match msg {
                    Message::Text(text) => {
                        let message = text.to_string();

                        match data.lock() {
                            Ok(mut data_guard) => {
                                let response = push_rpc_request(&mut data_guard, &message);
                                drop(data_guard);

                                match response {
                                    Ok(Some(receiver)) => {
                                        notify_session(session.clone(), receiver).await.ok();
                                    }
                                    Ok(None) => {
                                        info!("Processed notification: {}", message);
                                    }
                                    Err(e) => match e {
                                        RpcError::ParseError => {
                                            error!("Failed to parse request: {}", message);
                                        }
                                    },
                                }
                            }
                            Err(e) => {
                                error!("Failed to acquire data lock: {}", e);
                            }
                        }
                    }
                    Message::Ping(bytes) => {
                        if session.pong(&bytes).await.is_err() {
                            return;
                        }
                    }
                    Message::Close(reason) => {
                        let _ = session.close(reason).await;
                        break;
                    }
                    _ => break,
                }
            }
        }
    });

    Ok(response)
}

#[get("/health")]
async fn get_health(app_data: Data<AppData>) -> impl Responder {
    let health = Health {
        name: "pelican".to_string(),
        status: "OK".to_string(),
        version: "0.1.0".to_string(),
    };

    match serde_json::to_string(&health) {
        Ok(res) => HttpResponse::Ok()
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .body(res),
        Err(e) => {
            error!("Failed to serialize health response: {:?}", e);
            HttpResponse::InternalServerError()
                .insert_header(header::ContentType(mime::APPLICATION_JSON))
                .body("{\"error\": \"Internal Server Error\"}")
        }
    }
}

#[get("/metrics")]
async fn get_metrics(app_data: Data<AppData>) -> impl Responder {
    let mut sys = System::new_all();

    gauge!("used_memory").set(sys.used_memory() as f64);
    gauge!("total_memory").set(sys.total_memory() as f64);
    gauge!("total_swap").set(sys.total_swap() as f64);
    gauge!("used_swap").set(sys.used_swap() as f64);
    gauge!("cpus").set(sys.cpus().len() as f64);
    gauge!("global_cpu_usage").set(sys.global_cpu_usage());

    // Wait a bit because CPU usage is based on diff.
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    // Refresh CPU usage to get actual value.
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cpu(),
    );

    let process = sys.process(sysinfo::get_current_pid().unwrap()).unwrap();

    if let Some(tasks) = process.tasks() {
        tasks.iter().for_each(|pid| {
            if let Some(process) = sys.process(*pid) {
                gauge!("process_memory", "pid" => process.pid().to_string())
                    .set(process.memory() as f64);
                gauge!("process_virtual_memory","pid" => process.pid().to_string())
                    .set(process.virtual_memory() as f64);
                gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
            }
        });
    } else {
        gauge!("process_memory", "pid" => process.pid().to_string()).set(process.memory() as f64);
        gauge!("process_virtual_memory","pid" => process.pid().to_string())
            .set(process.virtual_memory() as f64);
        gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
    }

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
        .body(app_data.prometheus_handle.render())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
struct _Router {
    methods: LuaTable,
}

impl _Router {
    fn new(table: LuaTable) -> Self {
        Self { methods: table }
    }
}

impl UserData for _Router {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this: &Self, ()| {
            Ok(format!("Server: {:?}", this.methods))
        });

        methods.add_method(
            "add_method",
            |lua: &Lua, this: &_Router, (name, callback): (LuaString, LuaFunction)| {
                debug!("Adding method: {:?}", name);

                this.methods
                    .set(name, callback)
                    .expect("Failed to add method");

                Ok(())
            },
        );
    }
}

fn router(lua: &Lua, _: ()) -> LuaResult<_Router> {
    let table = lua.create_table()?;
    let router = _Router::new(table);

    debug!("Router: {:?}", router);

    Ok(router)
}

struct _Server {
    config: ServerConfig,
    handle: ServerHandle,
    app_data: Data<Mutex<AppData>>,
}

impl _Server {
    fn new(config: ServerConfig, handle: ServerHandle, app_data: Data<Mutex<AppData>>) -> Self {
        Self {
            config,
            handle,
            app_data,
        }
    }
}

impl UserData for _Server {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method("__tostring", |_, this: &Self, ()| {
            Ok(format!("Server: {:?}", this.config))
        });

        methods.add_method(
            "process_rpc",
            |lua: &Lua, this: &_Server, router: UserDataRef<_Router>| {
                let mut data_guard = this.app_data.lock().map_err(|e| {
                    error!("Error acquiring data lock: {:?}", e);
                    mlua::Error::RuntimeError(format!("Error acquiring data lock: {:?}", e))
                })?;

                while let Some(request) = data_guard.rpc_queue.pop_front() {
                    match process_request(lua, request.request, &router.methods) {
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

                Ok(())
            },
        );

        methods.add_method(
            "stop",
            |lua: &Lua, this: &_Server, graceful: Option<bool>| {
                info!("Stopping server...");

                let graceful = graceful.unwrap_or(false);

                let handle = this.handle.clone();
                Runtime::new()?.block_on(async move {
                    handle.stop(graceful).await;
                    info!("Server fully stopped (blocking)");
                });

                Ok(())
            },
        );

        methods.add_async_method(
            "async_stop",
            |lua: Lua, this: UserDataRef<_Server>, graceful: bool| async move {
                info!("Stopping server...");

                let handle = this.handle.clone();
                handle.stop(graceful).await;

                info!("Server fully stopped (non-blocking)");

                Ok(())
            },
        );
    }
}

impl Drop for _Server {
    fn drop(&mut self) {
        info!("Dropping server...");
        let handle = self.handle.clone();
        Runtime::new().unwrap().block_on(async move {
            handle.stop(false).await;
            info!("Server fully dropped");
        });
    }
}

fn serve(lua: &Lua, config: LuaValue) -> LuaResult<_Server> {
    let config: ServerConfig = lua
        .from_value(config)
        .expect("Failed to load server config");

    debug!("Serving Web Server: {:?}", config);

    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to create PrometheusBuilder");

    let app_data = Data::new(Mutex::new(AppData {
        rpc_queue: VecDeque::new(),
        prometheus_handle: Arc::new(prometheus_handle),
    }));

    let app_data_clone = app_data.clone();

    let config_read_copy = config.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_ws)
            .service(get_metrics)
            .service(get_health)
            .service(post_rpc)
            .app_data(Data::clone(&app_data))
    })
    .workers(1)
    .bind((config.host, config.port))
    .expect("Failed to bind to port")
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

    Ok(_Server::new(config_read_copy, handle, app_data_clone))
}
