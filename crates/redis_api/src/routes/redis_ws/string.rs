use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::routes::common::string::{
    delete_string, get_multiple_strings, get_string, get_string_info, set_multiple_strings,
    set_string, StringInfo, StringOperation,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum StringWsMessage {
    #[serde(rename = "get")]
    Get {
        #[serde(rename = "data")]
        data: GetData,
    },
    #[serde(rename = "set")]
    Set {
        #[serde(rename = "data")]
        data: SetData,
    },
    #[serde(rename = "del")]
    Del {
        #[serde(rename = "data")]
        data: DelData,
    },
    #[serde(rename = "info")]
    Info {
        #[serde(rename = "data")]
        data: InfoData,
    },
    #[serde(rename = "batch_get")]
    BatchGet {
        #[serde(rename = "data")]
        data: BatchGetData,
    },
    #[serde(rename = "batch_set")]
    BatchSet {
        #[serde(rename = "data")]
        data: BatchSetData,
    },
    #[serde(rename = "result")]
    Result {
        #[serde(rename = "data")]
        data: ResultData,
    },
    #[serde(rename = "batch_result")]
    BatchResult {
        #[serde(rename = "data")]
        data: BatchResultData,
    },
    #[serde(rename = "info_result")]
    InfoResult {
        #[serde(rename = "data")]
        data: InfoResultData,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(rename = "data")]
        data: DeletedData,
    },
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetData {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetData {
    pub key: String,
    pub value: String,
    pub ttl: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelData {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoData {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchGetData {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchSetData {
    pub operations: Vec<StringOperation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultData {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchResultData {
    pub keys: Vec<String>,
    pub values: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoResultData {
    pub info: Option<StringInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeletedData {
    pub key: String,
    pub deleted: bool,
}

async fn redis_ws_string_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>,
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
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&StringWsMessage::Error(format!(
                                        "Redis error: {e}"
                                    )))
                                    .unwrap(),
                                ))
                                .await;
                            continue;
                        }
                    };
                    let conn_arc = Arc::new(std::sync::Mutex::new(conn));

                    match message {
                        StringWsMessage::Get { data } => {
                            let value = get_string(conn_arc.clone(), &data.key).ok().flatten();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(StringWsMessage::Result {
                                            data: ResultData {
                                                key: data.key,
                                                value,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::Set { data } => {
                            let res = if let Some(ttl) = data.ttl {
                                set_string(conn_arc.clone(), &data.key, &data.value).and_then(
                                    |_| {
                                        redis::cmd("EXPIRE")
                                            .arg(&data.key)
                                            .arg(ttl)
                                            .query(&mut *conn_arc.lock().unwrap())
                                    },
                                )
                            } else {
                                set_string(conn_arc.clone(), &data.key, &data.value)
                            };
                            let msg = match res {
                                Ok(_) => StringWsMessage::Result {
                                    data: ResultData {
                                        key: data.key,
                                        value: Some(data.value),
                                    },
                                },
                                Err(e) => StringWsMessage::Error(format!("Set error: {e}")),
                            };
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&msg).unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::Del { data } => {
                            let deleted =
                                delete_string(conn_arc.clone(), &data.key).unwrap_or(false);
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(StringWsMessage::Deleted {
                                            data: DeletedData {
                                                key: data.key,
                                                deleted,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::Info { data } => {
                            let info = get_string_info(conn_arc.clone(), &data.key).ok().flatten();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(StringWsMessage::InfoResult {
                                            data: InfoResultData { info },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::BatchGet { data } => {
                            let values = get_multiple_strings(conn_arc.clone(), &data.keys)
                                .unwrap_or_default();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(StringWsMessage::BatchResult {
                                            data: BatchResultData {
                                                keys: data.keys,
                                                values,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::BatchSet { data } => {
                            let res = set_multiple_strings(conn_arc.clone(), &data.operations);
                            let msg = match res {
                                Ok(_) => StringWsMessage::Result {
                                    data: ResultData {
                                        key: "batch".to_string(),
                                        value: Some(format!(
                                            "Successfully set {} operations",
                                            data.operations.len()
                                        )),
                                    },
                                },
                                Err(e) => StringWsMessage::Error(format!("Batch set error: {e}")),
                            };
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&msg).unwrap(),
                                ))
                                .await;
                        }
                        StringWsMessage::Ping => {
                            let pong = StringWsMessage::Pong;
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&pong).unwrap(),
                                ))
                                .await;
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
    Router::new()
        .route("/string/ws", get(redis_ws_string_handler))
        .with_state(pool)
}
