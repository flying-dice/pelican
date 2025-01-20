use crate::json_rpc::{push_rpc_request, JsonRpcRequest, JsonRpcResponse, RpcError};
use actix_web::dev::ServerHandle;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::{Data, Payload};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::{Message, Session};
use log::{error, info};
use serde::Deserialize;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::time::timeout;

pub struct AppData {
    pub rpc_queue: VecDeque<AppRequest>,
    pub api_key: Option<String>,
}

pub struct AppRequest {
    pub request: JsonRpcRequest,
    pub response_sender: Option<oneshot::Sender<JsonRpcResponse>>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub api_key: Option<String>,
}

fn validate_api_key(req: &HttpRequest, data: &Data<Mutex<AppData>>) -> Result<(), Error> {
    let expected_api_key = &data
        .lock()
        .map_err(|e| ErrorInternalServerError(e.to_string()))?
        .api_key;

    match expected_api_key {
        Some(key) => match req.headers().get("x-api-key") {
            Some(api_key) => {
                if api_key == key {
                    Ok(())
                } else {
                    Err(ErrorUnauthorized("Invalid API key"))
                }
            }
            None => Err(ErrorUnauthorized("Missing API key")),
        },
        None => Ok(()),
    }
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

async fn ws_handler(
    req: HttpRequest,
    body: Payload,
    data: Data<Mutex<AppData>>,
) -> actix_web::Result<HttpResponse> {
    validate_api_key(&req, &data)?;

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

async fn rpc_handler(
    req: HttpRequest,
    data: Data<Mutex<AppData>>,
    body: String,
) -> Result<HttpResponse, Error> {
    validate_api_key(&req, &data)?;

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

pub fn create_app(data: Data<Mutex<AppData>>, config: &AppConfig) -> Result<ServerHandle, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .wrap(middleware::Logger::default())
            .route("/rpc", web::post().to(rpc_handler))
            .route("/ws", web::get().to(ws_handler))
    })
    .workers(config.workers)
    .bind((config.host.clone(), config.port))?
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

        info!("Server stopped!");
    });

    Ok(handle)
}
