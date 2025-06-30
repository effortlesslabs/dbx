use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::routes::common::admin::{
    config_get, config_get_all, config_reset_statistics, config_rewrite, config_set,
    flush_all_databases, flush_current_database, get_client_stats, get_database_size,
    get_memory_stats, get_server_info, get_server_info_section, get_server_stats, get_server_time,
    get_server_version, health_check, ping_server, server_status,
};
use dbx_adapter::redis::client::RedisPool;
use dbx_adapter::redis::primitives::admin::{HealthCheck, ServerStatus};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum AdminWsMessage {
    // Basic Health & Status messages
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "info")]
    Info { section: Option<String> },
    #[serde(rename = "dbsize")]
    DbSize,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "version")]
    Version,

    // Health Check messages
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "status")]
    Status,

    // Statistics messages
    #[serde(rename = "memory_stats")]
    MemoryStats,
    #[serde(rename = "client_stats")]
    ClientStats,
    #[serde(rename = "server_stats")]
    ServerStats,

    // Configuration messages
    #[serde(rename = "config_set")]
    ConfigSet { parameter: String, value: String },
    #[serde(rename = "config_get")]
    ConfigGet { parameter: String },
    #[serde(rename = "config_get_all")]
    ConfigGetAll,
    #[serde(rename = "config_resetstat")]
    ConfigResetStat,
    #[serde(rename = "config_rewrite")]
    ConfigRewrite,

    // Database Management messages
    #[serde(rename = "flushdb")]
    FlushDb,
    #[serde(rename = "flushall")]
    FlushAll,

    // Response messages
    #[serde(rename = "ping_result")]
    PingResult { response: String },
    #[serde(rename = "info_result")]
    InfoResult { info: String },
    #[serde(rename = "dbsize_result")]
    DbSizeResult { size: i64 },
    #[serde(rename = "time_result")]
    TimeResult { seconds: i64, microseconds: i64 },
    #[serde(rename = "version_result")]
    VersionResult { version: String },
    #[serde(rename = "health_result")]
    HealthResult { health: HealthCheck },
    #[serde(rename = "status_result")]
    StatusResult { status: ServerStatus },
    #[serde(rename = "memory_stats_result")]
    MemoryStatsResult { stats: HashMap<String, String> },
    #[serde(rename = "client_stats_result")]
    ClientStatsResult { stats: HashMap<String, String> },
    #[serde(rename = "server_stats_result")]
    ServerStatsResult { stats: HashMap<String, String> },
    #[serde(rename = "config_get_result")]
    ConfigGetResult { parameter: String, value: String },
    #[serde(rename = "config_get_all_result")]
    ConfigGetAllResult { config: HashMap<String, String> },
    #[serde(rename = "config_set_result")]
    ConfigSetResult { parameter: String, value: String },
    #[serde(rename = "config_resetstat_result")]
    ConfigResetStatResult,
    #[serde(rename = "config_rewrite_result")]
    ConfigRewriteResult,
    #[serde(rename = "flushdb_result")]
    FlushDbResult,
    #[serde(rename = "flushall_result")]
    FlushAllResult,

    // Error message
    #[serde(rename = "error")]
    Error(String),
}

async fn redis_ws_admin_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(pool): axum::extract::State<Arc<RedisPool>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_redis_ws_admin_socket(socket, pool))
}

async fn handle_redis_ws_admin_socket(socket: WebSocket, pool: Arc<RedisPool>) {
    let (mut sender, mut receiver) = socket.split();
    while let Some(Ok(msg)) = receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = msg {
            if let Ok(message) = serde_json::from_str::<AdminWsMessage>(&text) {
                let conn = match pool.get_connection() {
                    Ok(c) => c,
                    Err(e) => {
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&AdminWsMessage::Error(format!(
                                    "Redis error: {}",
                                    e
                                )))
                                .unwrap(),
                            ))
                            .await;
                        continue;
                    }
                };
                let conn_arc = Arc::new(std::sync::Mutex::new(conn));

                match message {
                    AdminWsMessage::Ping => {
                        let response =
                            ping_server(conn_arc.clone()).unwrap_or_else(|_| "ERROR".to_string());
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::PingResult { response }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::Info { section } => {
                        let info = if let Some(section) = section {
                            get_server_info_section(conn_arc.clone(), &section)
                                .unwrap_or_else(|_| "ERROR".to_string())
                        } else {
                            get_server_info(conn_arc.clone())
                                .unwrap_or_else(|_| "ERROR".to_string())
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::InfoResult { info }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::DbSize => {
                        let size = get_database_size(conn_arc.clone()).unwrap_or(-1);
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::DbSizeResult { size }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::Time => {
                        let time = get_server_time(conn_arc.clone()).unwrap_or((0, 0));
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::TimeResult {
                                        seconds: time.0,
                                        microseconds: time.1,
                                    }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::Version => {
                        let version = get_server_version(conn_arc.clone())
                            .unwrap_or_else(|_| "UNKNOWN".to_string());
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::VersionResult { version }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::Health => {
                        let health =
                            health_check(conn_arc.clone()).unwrap_or_else(|_| HealthCheck {
                                is_healthy: false,
                                ping_response: "ERROR".to_string(),
                                database_size: -1,
                                version: "UNKNOWN".to_string(),
                                memory_usage: HashMap::new(),
                            });
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::HealthResult { health }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::Status => {
                        let status =
                            server_status(conn_arc.clone()).unwrap_or_else(|_| ServerStatus {
                                timestamp: 0,
                                uptime_seconds: 0,
                                connected_clients: 0,
                                used_memory: 0,
                                total_commands_processed: 0,
                                keyspace_hits: 0,
                                keyspace_misses: 0,
                                version: "UNKNOWN".to_string(),
                                role: "UNKNOWN".to_string(),
                            });
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&(AdminWsMessage::StatusResult { status }))
                                    .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::MemoryStats => {
                        let stats = get_memory_stats(conn_arc.clone()).unwrap_or_default();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::MemoryStatsResult { stats }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ClientStats => {
                        let stats = get_client_stats(conn_arc.clone()).unwrap_or_default();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::ClientStatsResult { stats }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ServerStats => {
                        let stats = get_server_stats(conn_arc.clone()).unwrap_or_default();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::ServerStatsResult { stats }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ConfigSet { parameter, value } => {
                        let res = config_set(conn_arc.clone(), &parameter, &value);
                        let msg = match res {
                            Ok(_) => AdminWsMessage::ConfigSetResult { parameter, value },
                            Err(e) => AdminWsMessage::Error(format!("Config set error: {}", e)),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ConfigGet { parameter } => {
                        let value = config_get(conn_arc.clone(), &parameter)
                            .unwrap_or_else(|_| "ERROR".to_string());
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::ConfigGetResult { parameter, value }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ConfigGetAll => {
                        let config = config_get_all(conn_arc.clone()).unwrap_or_default();
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(
                                    &(AdminWsMessage::ConfigGetAllResult { config }),
                                )
                                .unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ConfigResetStat => {
                        let res = config_reset_statistics(conn_arc.clone());
                        let msg = match res {
                            Ok(_) => AdminWsMessage::ConfigResetStatResult,
                            Err(e) => {
                                AdminWsMessage::Error(format!("Config resetstat error: {}", e))
                            }
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::ConfigRewrite => {
                        let res = config_rewrite(conn_arc.clone());
                        let msg = match res {
                            Ok(_) => AdminWsMessage::ConfigRewriteResult,
                            Err(e) => AdminWsMessage::Error(format!("Config rewrite error: {}", e)),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::FlushDb => {
                        let res = flush_current_database(conn_arc.clone());
                        let msg = match res {
                            Ok(_) => AdminWsMessage::FlushDbResult,
                            Err(e) => AdminWsMessage::Error(format!("FlushDB error: {}", e)),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    AdminWsMessage::FlushAll => {
                        let res = flush_all_databases(conn_arc.clone());
                        let msg = match res {
                            Ok(_) => AdminWsMessage::FlushAllResult,
                            Err(e) => AdminWsMessage::Error(format!("FlushAll error: {}", e)),
                        };
                        let _ = sender
                            .send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap(),
                            ))
                            .await;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn create_redis_ws_admin_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        .route("/admin/ws", get(redis_ws_admin_handler))
        .with_state(pool)
}
