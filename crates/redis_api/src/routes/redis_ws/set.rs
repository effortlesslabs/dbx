use axum::{
    extract::{ ws::WebSocket, WebSocketUpgrade },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{ StreamExt, SinkExt };
use serde::{ Deserialize, Serialize };
use std::sync::Arc;

use crate::routes::common::set::{
    add_to_set,
    remove_from_set,
    get_set_members,
    set_exists,
    get_set_cardinality,
    intersect_sets,
    union_sets,
    difference_sets,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum SetWsMessage {
    #[serde(rename = "add")] Add {
        key: String,
        member: String,
    },
    #[serde(rename = "remove")] Remove {
        key: String,
        member: String,
    },
    #[serde(rename = "members")] Members {
        key: String,
    },
    #[serde(rename = "exists")] Exists {
        key: String,
        member: String,
    },
    #[serde(rename = "cardinality")] Cardinality {
        key: String,
    },
    #[serde(rename = "intersect")] Intersect {
        keys: Vec<String>,
    },
    #[serde(rename = "union")] Union {
        keys: Vec<String>,
    },
    #[serde(rename = "difference")] Difference {
        keys: Vec<String>,
    },
    #[serde(rename = "result")] Result {
        key: String,
        value: Option<serde_json::Value>,
    },
    #[serde(rename = "error")] Error(String),
    #[serde(rename = "ping")] Ping,
    #[serde(rename = "pong")] Pong,
}

async fn redis_ws_set_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_redis_ws_set_socket(socket, pool))
}

async fn handle_redis_ws_set_socket(socket: WebSocket, pool: Arc<RedisPool>) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(Ok(msg)) = receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = msg {
            if let Ok(message) = serde_json::from_str::<SetWsMessage>(&text) {
                let conn = match pool.get_connection() {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(
                                serde_json
                                    ::to_string(&SetWsMessage::Error(format!("Redis error: {}", e)))
                                    .unwrap()
                            )
                        ).await;
                        continue;
                    }
                };
                let conn_arc = Arc::new(std::sync::Mutex::new(conn));
                match message {
                    SetWsMessage::Add { key, member } => {
                        let res = add_to_set(conn_arc.clone(), &key, &[&member]);
                        let msg = match res {
                            Ok(count) =>
                                SetWsMessage::Result { key, value: Some(serde_json::json!(count)) },
                            Err(e) => SetWsMessage::Error(format!("Add error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Remove { key, member } => {
                        let res = remove_from_set(conn_arc.clone(), &key, &[&member]);
                        let msg = match res {
                            Ok(count) =>
                                SetWsMessage::Result { key, value: Some(serde_json::json!(count)) },
                            Err(e) => SetWsMessage::Error(format!("Remove error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Members { key } => {
                        let res = get_set_members(conn_arc.clone(), &key);
                        let msg = match res {
                            Ok(members) =>
                                SetWsMessage::Result {
                                    key,
                                    value: Some(serde_json::json!(members)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Members error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Exists { key, member } => {
                        let res = set_exists(conn_arc.clone(), &key, &member);
                        let msg = match res {
                            Ok(exists) =>
                                SetWsMessage::Result {
                                    key,
                                    value: Some(serde_json::json!(exists)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Exists error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Cardinality { key } => {
                        let res = get_set_cardinality(conn_arc.clone(), &key);
                        let msg = match res {
                            Ok(cardinality) =>
                                SetWsMessage::Result {
                                    key,
                                    value: Some(serde_json::json!(cardinality)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Cardinality error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Intersect { keys } => {
                        let key_refs: Vec<&str> = keys
                            .iter()
                            .map(|k| k.as_str())
                            .collect();
                        let res = intersect_sets(conn_arc.clone(), &key_refs);
                        let msg = match res {
                            Ok(values) =>
                                SetWsMessage::Result {
                                    key: "intersect".to_string(),
                                    value: Some(serde_json::json!(values)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Intersect error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Union { keys } => {
                        let key_refs: Vec<&str> = keys
                            .iter()
                            .map(|k| k.as_str())
                            .collect();
                        let res = union_sets(conn_arc.clone(), &key_refs);
                        let msg = match res {
                            Ok(values) =>
                                SetWsMessage::Result {
                                    key: "union".to_string(),
                                    value: Some(serde_json::json!(values)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Union error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Difference { keys } => {
                        let key_refs: Vec<&str> = keys
                            .iter()
                            .map(|k| k.as_str())
                            .collect();
                        let res = difference_sets(conn_arc.clone(), &key_refs);
                        let msg = match res {
                            Ok(values) =>
                                SetWsMessage::Result {
                                    key: "difference".to_string(),
                                    value: Some(serde_json::json!(values)),
                                },
                            Err(e) => SetWsMessage::Error(format!("Difference error: {}", e)),
                        };
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())
                        ).await;
                    }
                    SetWsMessage::Ping => {
                        let pong = SetWsMessage::Pong;
                        let _ = sender.send(
                            axum::extract::ws::Message::Text(serde_json::to_string(&pong).unwrap())
                        ).await;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn create_redis_ws_set_routes(pool: Arc<RedisPool>) -> Router {
    Router::new().route("/set/ws", get(redis_ws_set_handler)).with_state(pool)
}
