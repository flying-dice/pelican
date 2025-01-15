use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_ws::{Message, Session};
use mlua::prelude::*;
use mlua::{Lua, Result, Value};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::Duration;
use std::thread;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<String>,
}

struct AppData {
    rpc_queue: VecDeque<String>,
    rpc_responses: HashMap<String, String>,
    sessions: HashMap<String, Session>,
}

async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    data: web::Data<Mutex<AppData>>,
) -> actix_web::Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body).unwrap();

    data.lock()
        .unwrap()
        .sessions
        .insert(Uuid::new_v4().to_string(), session.clone());

    actix_web::rt::spawn({
        let data = data.clone();

        async move {
            while let Some(Ok(msg)) = msg_stream.recv().await {
                match msg {
                    Message::Text(text) => {
                        let mut data = data.lock().unwrap();
                        data.rpc_queue.push_back(text.clone().parse().unwrap());
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

async fn rpc_handler(data: Data<Mutex<AppData>>, body: String) -> impl Responder {
    match serde_json::from_str::<JsonRpcRequest>(&body) {
        Ok(request) => {
            let request_id = request.id.clone().unwrap_or("notification".to_string());

            println!("<< [{}]: '{}'", request_id, body);
            data.lock().unwrap().rpc_queue.push_back(body);

            if request.id.is_none() {
                return HttpResponse::Accepted().body("OK");
            }

            while !data.lock().unwrap().rpc_responses.contains_key(&request_id) {
                sleep(Duration::from_millis(5)).await;
            }

            HttpResponse::Ok().body(
                data.lock()
                    .unwrap()
                    .rpc_responses
                    .remove(&request_id)
                    .unwrap(),
            )
        }
        Err(e) => {
            println!("Error parsing RPC: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

#[mlua::lua_module]
fn json_rpc_server(lua: &Lua) -> Result<LuaTable> {
    let exports = lua.create_table()?;

    // Create the shared queue
    let data = Data::new(Mutex::new(AppData {
        rpc_queue: VecDeque::new(),
        rpc_responses: HashMap::new(),
        sessions: HashMap::new(),
    }));

    let cloned = data.clone();
    exports.set(
        "start_server",
        lua.create_async_function({
            move |_lua, port: u16| {
                let data = cloned.clone();
                async move {
                    thread::spawn(move || {
                        actix_web::rt::System::new().block_on(async move {
                            println!("Starting server on port {}", port);
                            HttpServer::new(move || {
                                App::new()
                                    .app_data(Data::clone(&data))
                                    .route("/rpc", web::post().to(rpc_handler))
                                    .route("/ws", web::get().to(ws_handler))
                            })
                            .bind(format!("0.0.0.0:{}", port))
                            .expect("Failed to bind address")
                            .run()
                            .await
                            .expect("Failed to run server");
                        });
                    });
                    Ok(())
                }
            }
        })?,
    )?;

    exports.set(
        "process_rpc",
        lua.create_async_function(move |_lua, callback: LuaFunction| {
            let data = data.clone();
            async move {
                let mut data = data.lock().unwrap();
                if let Some(rpc) = data.rpc_queue.pop_front() {
                    match callback.call::<Value>(rpc) {
                        Ok(result) => {
                            match result {
                                Value::Nil => {
                                    println!("RPC returned nil");
                                    return Ok(())
                                }
                                Value::String(_response) => {
                                    let response = _response.to_str()?.to_owned();
                                    println!("RPC returned string");
                                    let id = serde_json::from_str::<JsonRpcResponse>(&response).unwrap().id.clone().unwrap_or("notification".to_string());
                                    println!(">> [{}]: '{}'", id, response);
                                    data.rpc_responses.insert(id, response.clone());

                                    println!("Sending message to all sessions");
                                    println!("Sessions: {:?}", data.sessions.len());
                                    let mut sessions_to_remove = Vec::new();

                                    for (id, session) in data.sessions.iter_mut() {
                                        match session.text(response.clone()).await {
                                            Ok(_) => {
                                                println!("Message sent to session");
                                            },
                                            Err(e) => {
                                                println!("Error sending message to session, removing due to: {:?}", e);
                                                sessions_to_remove.push(id.clone());
                                            }
                                        };
                                    }

                                    for id in sessions_to_remove {
                                        data.sessions.remove(&id);
                                    }
                                }
                                _ => {
                                    println!("RPC returned unexpected value: {:?}", result);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error processing RPC: {:?}", e);
                        }
                    }
                }
                Ok(())
            }
        })?,
    )?;

    exports.set(
        "encode",
        lua.create_function(|_lua, value: LuaValue| {
            // println!("to_json: {:?}", value);
            let json = serde_json::to_string(&value).unwrap();
            Ok(json)
        })?,
    )?;

    exports.set(
        "decode",
        lua.create_function(|lua, json: LuaString| {
            // println!("from_json: {:?}", json);
            let value: serde_json::Value = serde_json::from_str(&json.to_str()?).unwrap();
            let lua_value = lua.to_value(&value)?;
            Ok(lua_value)
        })?,
    )?;

    Ok(exports)
}
