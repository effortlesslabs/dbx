use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::routes::common::hash::{
    delete_hash_field, get_all_hash_fields, get_hash_field, set_hash_field,
    set_multiple_hash_fields,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum HashWsMessage {
    #[serde(rename = "get")]
    Get { key: String, field: String },
    #[serde(rename = "set")]
    Set {
        key: String,
        field: String,
        value: String,
    },
    #[serde(rename = "get_all")]
    GetAll { key: String },
    #[serde(rename = "del")]
    Del { key: String, field: String },
    #[serde(rename = "exists")]
    Exists { key: String, field: String },
    #[serde(rename = "batch_set")]
    BatchSet {
        key: String,
        fields: Vec<(String, String)>,
    },
    #[serde(rename = "result")]
    Result {
        key: String,
        field: Option<String>,
        value: Option<String>,
    },
    #[serde(rename = "all_result")]
    AllResult {
        key: String,
        fields: std::collections::HashMap<String, String>,
    },
    #[serde(rename = "deleted")]
    Deleted {
        key: String,
        field: String,
        deleted: bool,
    },
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

async fn redis_ws_hash_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_redis_ws_hash_socket(socket, pool))
}

async fn handle_redis_ws_hash_socket(socket: WebSocket, pool: Arc<RedisPool>) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(Ok(msg)) = receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = msg {
            if let Ok(message) = serde_json::from_str::<HashWsMessage>(&text) {
                let conn = match pool.get_connection() {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&HashWsMessage::Error(format!(
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
                    HashWsMessage::Get { key, field } => {
                        let value = get_hash_field(conn_arc.clone(), &key, &field)
                            .ok()
                            .flatten();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(HashWsMessage::Result {
                                        key,
                                        field: Some(field),
                                        value,
                                    }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    HashWsMessage::Set { key, field, value } => {
                        let res = set_hash_field(conn_arc.clone(), &key, &field, &value);
                        let msg = match res {
                            Ok(_) => HashWsMessage::Result {
                                key,
                                field: Some(field),
                                value: Some(value),
                            },
                            Err(e) => HashWsMessage::Error(format!("Set error: {e}")),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    HashWsMessage::Del { key, field } => {
                        let deleted =
                            delete_hash_field(conn_arc.clone(), &key, &field).unwrap_or(false);
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(HashWsMessage::Deleted {
                                        key,
                                        field,
                                        deleted,
                                    }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    HashWsMessage::GetAll { key } => {
                        let fields =
                            get_all_hash_fields(conn_arc.clone(), &key).unwrap_or_default();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(HashWsMessage::AllResult { key, fields }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    HashWsMessage::BatchSet { key, fields } => {
                        let field_refs: Vec<(&str, &str)> = fields
                            .iter()
                            .map(|(f, v)| (f.as_str(), v.as_str()))
                            .collect();
                        let res = set_multiple_hash_fields(conn_arc.clone(), &key, &field_refs);
                        let msg = match res {
                            Ok(_) => HashWsMessage::Result {
                                key,
                                field: None,
                                value: Some("Batch set success".to_string()),
                            },
                            Err(e) => HashWsMessage::Error(format!("Batch set error: {e}")),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    HashWsMessage::Ping => {
                        let pong = HashWsMessage::Pong;
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&pong).unwrap(),
                            ))
                            .await;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn create_redis_ws_hash_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        .route("/hash/ws", get(redis_ws_hash_handler))
        .with_state(pool)
}
