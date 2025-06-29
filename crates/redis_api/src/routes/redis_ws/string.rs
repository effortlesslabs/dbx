use axum::{
    extract::{ ws::WebSocket, WebSocketUpgrade },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{ StreamExt, SinkExt };
use serde::{ Deserialize, Serialize };
use std::sync::Arc;

use crate::routes::common::string::{
    get_string,
    set_string,
    delete_string,
    get_string_info,
    get_multiple_strings,
    set_multiple_strings,
    StringOperation,
    StringInfo,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum StringWsMessage {
    #[serde(rename = "get")] Get {
        key: String,
    },
    #[serde(rename = "set")] Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "del")] Del {
        key: String,
    },
    #[serde(rename = "info")] Info {
        key: String,
    },
    #[serde(rename = "batch_get")] BatchGet {
        keys: Vec<String>,
    },
    #[serde(rename = "batch_set")] BatchSet {
        operations: Vec<StringOperation>,
    },
    #[serde(rename = "result")] Result {
        key: String,
        value: Option<String>,
    },
    #[serde(rename = "batch_result")] BatchResult {
        keys: Vec<String>,
        values: Vec<Option<String>>,
    },
    #[serde(rename = "info_result")] InfoResult {
        info: Option<StringInfo>,
    },
    #[serde(rename = "deleted")] Deleted {
        key: String,
        deleted: bool,
    },
    #[serde(rename = "error")] Error(String),
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

async fn redis_ws_string_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_redis_ws_string_socket(socket, pool))
}

async fn handle_redis_ws_string_socket(socket: WebSocket, pool: Arc<RedisPool>) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(Ok(msg)) = receiver.next().await {
        tracing::debug!("[WS STRING] Received WebSocket message: {:?}", msg);
        if let axum::extract::ws::Message::Text(text) = msg {
            tracing::debug!("[WS STRING] Received text: {}", text);
            match serde_json::from_str::<StringWsMessage>(&text) {
                Ok(message) => {
                    tracing::debug!("[WS STRING] Parsed message: {:?}", message);
                    let conn = match pool.get_connection() {
                        Ok(c) => c,
                        Err(e) => {
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &StringWsMessage::Error(format!("Redis error: {}", e))
                                        )
                                        .unwrap()
                                )
                            ).await;
                            continue;
                        }
                    };
                    let conn_arc = Arc::new(std::sync::Mutex::new(conn));

                    match message {
                        StringWsMessage::Get { key } => {
                            let value = get_string(conn_arc.clone(), &key).ok().flatten();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(StringWsMessage::Result { key, value }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::Set { key, value, ttl } => {
                            let res = if let Some(ttl) = ttl {
                                set_string(conn_arc.clone(), &key, &value).and_then(|_|
                                    redis
                                        ::cmd("EXPIRE")
                                        .arg(&key)
                                        .arg(ttl)
                                        .query(&mut *conn_arc.lock().unwrap())
                                )
                            } else {
                                set_string(conn_arc.clone(), &key, &value)
                            };
                            let msg = match res {
                                Ok(_) => StringWsMessage::Result { key, value: Some(value) },
                                Err(e) => StringWsMessage::Error(format!("Set error: {}", e)),
                            };
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json::to_string(&msg).unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::Del { key } => {
                            let deleted = delete_string(conn_arc.clone(), &key).unwrap_or(false);
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(StringWsMessage::Deleted { key, deleted }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::Info { key } => {
                            let info = get_string_info(conn_arc.clone(), &key).ok().flatten();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(StringWsMessage::InfoResult { info }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::BatchGet { keys } => {
                            let values = get_multiple_strings(
                                conn_arc.clone(),
                                &keys
                            ).unwrap_or_default();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(StringWsMessage::BatchResult { keys, values })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::BatchSet { operations } => {
                            let res = set_multiple_strings(conn_arc.clone(), &operations);
                            let msg = match res {
                                Ok(_) =>
                                    StringWsMessage::Result {
                                        key: "batch".to_string(),
                                        value: Some(
                                            format!(
                                                "Successfully set {} operations",
                                                operations.len()
                                            )
                                        ),
                                    },
                                Err(e) => StringWsMessage::Error(format!("Batch set error: {}", e)),
                            };
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json::to_string(&msg).unwrap()
                                )
                            ).await;
                        }
                        StringWsMessage::Ping => {
                            let pong = StringWsMessage::Pong;
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json::to_string(&pong).unwrap()
                                )
                            ).await;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    tracing::error!("[WS STRING] Failed to parse message: {}", e);
                    tracing::error!("[WS STRING] Raw message: {}", text);
                }
            }
        }
    }
}

pub fn create_redis_ws_string_routes(pool: Arc<RedisPool>) -> Router {
    Router::new().route("/string/ws", get(redis_ws_string_handler)).with_state(pool)
}
