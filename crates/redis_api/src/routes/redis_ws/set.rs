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
#[serde(tag = "type")]
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
    // Response types
    #[serde(rename = "added")] Added {
        key: String,
        member: String,
        added: usize,
    },
    #[serde(rename = "removed")] Removed {
        key: String,
        member: String,
        removed: usize,
    },
    #[serde(rename = "members_result")] MembersResult {
        key: String,
        members: Vec<String>,
    },
    #[serde(rename = "exists_result")] ExistsResult {
        key: String,
        member: String,
        exists: bool,
    },
    #[serde(rename = "cardinality_result")] CardinalityResult {
        key: String,
        cardinality: usize,
    },
    #[serde(rename = "intersect_result")] IntersectResult {
        keys: Vec<String>,
        intersection: Vec<String>,
    },
    #[serde(rename = "union_result")] UnionResult {
        keys: Vec<String>,
        union: Vec<String>,
    },
    #[serde(rename = "difference_result")] DifferenceResult {
        keys: Vec<String>,
        difference: Vec<String>,
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
    println!("[DEBUG] WebSocket upgrade requested for /redis_ws/set/ws");
    ws.on_upgrade(|socket| handle_redis_ws_set_socket(socket, pool))
}

async fn handle_redis_ws_set_socket(socket: WebSocket, pool: Arc<RedisPool>) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(Ok(msg)) = receiver.next().await {
        tracing::debug!("[WS SET] Received WebSocket message: {:?}", msg);
        if let axum::extract::ws::Message::Text(text) = msg {
            tracing::debug!("[WS SET] Received text: {}", text);
            match serde_json::from_str::<SetWsMessage>(&text) {
                Ok(message) => {
                    tracing::debug!("[WS SET] Parsed message: {:?}", message);
                    let conn = match pool.get_connection() {
                        Ok(c) => c,
                        Err(e) => {
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &SetWsMessage::Error(format!("Redis error: {}", e))
                                        )
                                        .unwrap()
                                )
                            ).await;
                            continue;
                        }
                    };
                    let conn_arc = Arc::new(std::sync::Mutex::new(conn));

                    match message {
                        SetWsMessage::Add { key, member } => {
                            let added = add_to_set(conn_arc.clone(), &key, &[&member]).unwrap_or(0);
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(SetWsMessage::Added { key, member, added }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Remove { key, member } => {
                            let removed = remove_from_set(
                                conn_arc.clone(),
                                &key,
                                &[&member]
                            ).unwrap_or(0);
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(SetWsMessage::Removed { key, member, removed })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Members { key } => {
                            let members = get_set_members(
                                conn_arc.clone(),
                                &key
                            ).unwrap_or_default();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(SetWsMessage::MembersResult { key, members }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Exists { key, member } => {
                            let exists = set_exists(conn_arc.clone(), &key, &member).unwrap_or(
                                false
                            );
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(SetWsMessage::ExistsResult { key, member, exists })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Cardinality { key } => {
                            let cardinality = get_set_cardinality(conn_arc.clone(), &key).unwrap_or(
                                0
                            );
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(SetWsMessage::CardinalityResult { key, cardinality })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Intersect { keys } => {
                            let key_refs: Vec<&str> = keys
                                .iter()
                                .map(|k| k.as_str())
                                .collect();
                            let intersection = intersect_sets(
                                conn_arc.clone(),
                                &key_refs
                            ).unwrap_or_default();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(SetWsMessage::IntersectResult { keys, intersection })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Union { keys } => {
                            let key_refs: Vec<&str> = keys
                                .iter()
                                .map(|k| k.as_str())
                                .collect();
                            let union = union_sets(conn_arc.clone(), &key_refs).unwrap_or_default();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(&(SetWsMessage::UnionResult { keys, union }))
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Difference { keys } => {
                            let key_refs: Vec<&str> = keys
                                .iter()
                                .map(|k| k.as_str())
                                .collect();
                            let difference = difference_sets(
                                conn_arc.clone(),
                                &key_refs
                            ).unwrap_or_default();
                            let _ = sender.send(
                                axum::extract::ws::Message::Text(
                                    serde_json
                                        ::to_string(
                                            &(SetWsMessage::DifferenceResult { keys, difference })
                                        )
                                        .unwrap()
                                )
                            ).await;
                        }
                        SetWsMessage::Ping => {
                            let pong = SetWsMessage::Pong;
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
                    tracing::error!("[WS SET] Failed to parse message: {}", e);
                    tracing::error!("[WS SET] Raw message: {}", text);
                }
            }
        }
    }
}

pub fn create_redis_ws_set_routes(pool: Arc<RedisPool>) -> Router {
    Router::new().route("/set/ws", get(redis_ws_set_handler)).with_state(pool)
}
