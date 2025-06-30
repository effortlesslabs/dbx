use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::routes::common::set::{
    add_to_set, difference_sets, get_set_cardinality, get_set_members, intersect_sets,
    remove_from_set, set_exists, union_sets,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SetWsMessage {
    #[serde(rename = "add")]
    Add {
        #[serde(rename = "data")]
        data: AddData,
    },
    #[serde(rename = "remove")]
    Remove {
        #[serde(rename = "data")]
        data: RemoveData,
    },
    #[serde(rename = "members")]
    Members {
        #[serde(rename = "data")]
        data: MembersData,
    },
    #[serde(rename = "exists")]
    Exists {
        #[serde(rename = "data")]
        data: ExistsData,
    },
    #[serde(rename = "cardinality")]
    Cardinality {
        #[serde(rename = "data")]
        data: CardinalityData,
    },
    #[serde(rename = "intersect")]
    Intersect {
        #[serde(rename = "data")]
        data: IntersectData,
    },
    #[serde(rename = "union")]
    Union {
        #[serde(rename = "data")]
        data: UnionData,
    },
    #[serde(rename = "difference")]
    Difference {
        #[serde(rename = "data")]
        data: DifferenceData,
    },
    // Response types
    #[serde(rename = "added")]
    Added {
        #[serde(rename = "data")]
        data: AddedData,
    },
    #[serde(rename = "removed")]
    Removed {
        #[serde(rename = "data")]
        data: RemovedData,
    },
    #[serde(rename = "members_result")]
    MembersResult {
        #[serde(rename = "data")]
        data: MembersResultData,
    },
    #[serde(rename = "exists_result")]
    ExistsResult {
        #[serde(rename = "data")]
        data: ExistsResultData,
    },
    #[serde(rename = "cardinality_result")]
    CardinalityResult {
        #[serde(rename = "data")]
        data: CardinalityResultData,
    },
    #[serde(rename = "intersect_result")]
    IntersectResult {
        #[serde(rename = "data")]
        data: IntersectResultData,
    },
    #[serde(rename = "union_result")]
    UnionResult {
        #[serde(rename = "data")]
        data: UnionResultData,
    },
    #[serde(rename = "difference_result")]
    DifferenceResult {
        #[serde(rename = "data")]
        data: DifferenceResultData,
    },
    #[serde(rename = "result")]
    Result {
        #[serde(rename = "data")]
        data: ResultData,
    },
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddData {
    pub key: String,
    pub member: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoveData {
    pub key: String,
    pub member: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MembersData {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExistsData {
    pub key: String,
    pub member: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardinalityData {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntersectData {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnionData {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DifferenceData {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddedData {
    pub key: String,
    pub member: String,
    pub added: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemovedData {
    pub key: String,
    pub member: String,
    pub removed: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MembersResultData {
    pub key: String,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExistsResultData {
    pub key: String,
    pub member: String,
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardinalityResultData {
    pub key: String,
    pub cardinality: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntersectResultData {
    pub keys: Vec<String>,
    pub intersection: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnionResultData {
    pub keys: Vec<String>,
    pub union: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DifferenceResultData {
    pub keys: Vec<String>,
    pub difference: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultData {
    pub key: String,
    pub value: Option<serde_json::Value>,
}

async fn redis_ws_set_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>,
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
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&SetWsMessage::Error(format!(
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
                        SetWsMessage::Add { data } => {
                            let added = add_to_set(conn_arc.clone(), &data.key, &[&data.member])
                                .unwrap_or(0);
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::Added {
                                            data: AddedData {
                                                key: data.key,
                                                member: data.member,
                                                added,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Remove { data } => {
                            let removed =
                                remove_from_set(conn_arc.clone(), &data.key, &[&data.member])
                                    .unwrap_or(0);
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::Removed {
                                            data: RemovedData {
                                                key: data.key,
                                                member: data.member,
                                                removed,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Members { data } => {
                            let members =
                                get_set_members(conn_arc.clone(), &data.key).unwrap_or_default();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::Result {
                                            data: ResultData {
                                                key: data.key,
                                                value: Some(serde_json::json!(members)),
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Exists { data } => {
                            let exists = set_exists(conn_arc.clone(), &data.key, &data.member)
                                .unwrap_or(false);
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::ExistsResult {
                                            data: ExistsResultData {
                                                key: data.key,
                                                member: data.member,
                                                exists,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Cardinality { data } => {
                            let cardinality =
                                get_set_cardinality(conn_arc.clone(), &data.key).unwrap_or(0);
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::CardinalityResult {
                                            data: CardinalityResultData {
                                                key: data.key,
                                                cardinality,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Intersect { data } => {
                            let key_refs: Vec<&str> =
                                data.keys.iter().map(|k| k.as_str()).collect();
                            let intersection =
                                intersect_sets(conn_arc.clone(), &key_refs).unwrap_or_default();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::IntersectResult {
                                            data: IntersectResultData {
                                                keys: data.keys,
                                                intersection,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Union { data } => {
                            let key_refs: Vec<&str> =
                                data.keys.iter().map(|k| k.as_str()).collect();
                            let union = union_sets(conn_arc.clone(), &key_refs).unwrap_or_default();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::UnionResult {
                                            data: UnionResultData {
                                                keys: data.keys,
                                                union,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Difference { data } => {
                            let key_refs: Vec<&str> =
                                data.keys.iter().map(|k| k.as_str()).collect();
                            let difference =
                                difference_sets(conn_arc.clone(), &key_refs).unwrap_or_default();
                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(
                                        &(SetWsMessage::DifferenceResult {
                                            data: DifferenceResultData {
                                                keys: data.keys,
                                                difference,
                                            },
                                        }),
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                        SetWsMessage::Ping => {
                            let pong = SetWsMessage::Pong;
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
                    tracing::error!("[WS SET] Failed to parse message: {}", e);
                    tracing::error!("[WS SET] Raw message: {}", text);
                }
            }
        }
    }
}

pub fn create_redis_ws_set_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        .route("/set/ws", get(redis_ws_set_handler))
        .with_state(pool)
}
