use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// String operations request models
#[derive(Debug, Deserialize)]
pub struct SetRequest {
    pub value: String,
    pub ttl: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct SetManyRequest {
    pub key_values: HashMap<String, String>,
    pub ttl: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct IncrByRequest {
    pub increment: i64,
}

#[derive(Debug, Deserialize)]
pub struct SetIfNotExistsRequest {
    pub value: String,
    pub ttl: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CompareAndSetRequest {
    pub expected_value: String,
    pub new_value: String,
    pub ttl: Option<u64>,
}

/// String operations response models
#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerValue {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValues {
    pub key_values: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysResponse {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub deleted_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExistsResponse {
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtlResponse {
    pub ttl: i64,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub redis_connected: bool,
    pub timestamp: String,
}

/// Server info response
#[derive(Debug, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub redis_url: String,
    pub pool_size: u32,
}

// WebSocket Models

/// WebSocket command types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "action", content = "params")]
pub enum WebSocketCommand {
    // String commands
    #[serde(rename = "get")]
    Get { key: String },
    #[serde(rename = "set")]
    Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "delete")]
    Delete { key: String },
    #[serde(rename = "exists")]
    Exists { key: String },
    #[serde(rename = "ttl")]
    Ttl { key: String },
    #[serde(rename = "incr")]
    Incr { key: String },
    #[serde(rename = "incrby")]
    IncrBy { key: String, increment: i64 },
    #[serde(rename = "setnx")]
    SetNx {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "cas")]
    CompareAndSet {
        key: String,
        expected_value: String,
        new_value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "batch_get")]
    BatchGet { keys: Vec<String> },
    #[serde(rename = "batch_set")]
    BatchSet {
        key_values: HashMap<String, String>,
        ttl: Option<u64>,
    },
    #[serde(rename = "batch_delete")]
    BatchDelete { keys: Vec<String> },
    #[serde(rename = "batch_incr")]
    BatchIncr { keys: Vec<String> },
    #[serde(rename = "batch_incrby")]
    BatchIncrBy { key_increments: Vec<(String, i64)> },

    // Set commands
    #[serde(rename = "sadd")]
    Sadd { key: String, members: Vec<String> },
    #[serde(rename = "srem")]
    Srem { key: String, members: Vec<String> },
    #[serde(rename = "smembers")]
    Smembers { key: String },
    #[serde(rename = "scard")]
    Scard { key: String },
    #[serde(rename = "sismember")]
    Sismember { key: String, member: String },
    #[serde(rename = "spop")]
    Spop { key: String },
    #[serde(rename = "srandmember")]
    Srandmember { key: String },
    #[serde(rename = "smove")]
    Smove {
        source: String,
        destination: String,
        member: String,
    },
    #[serde(rename = "sunion")]
    Sunion { keys: Vec<String> },
    #[serde(rename = "sinter")]
    Sinter { keys: Vec<String> },
    #[serde(rename = "sdiff")]
    Sdiff { keys: Vec<String> },

    // Hash commands
    #[serde(rename = "hset")]
    Hset {
        key: String,
        field: String,
        value: String,
    },
    #[serde(rename = "hget")]
    Hget { key: String, field: String },
    #[serde(rename = "hdel")]
    Hdel { key: String, field: String },
    #[serde(rename = "hexists")]
    Hexists { key: String, field: String },
    #[serde(rename = "hlen")]
    Hlen { key: String },
    #[serde(rename = "hkeys")]
    Hkeys { key: String },
    #[serde(rename = "hvals")]
    Hvals { key: String },
    #[serde(rename = "hgetall")]
    Hgetall { key: String },
    #[serde(rename = "hmset")]
    Hmset {
        key: String,
        fields: HashMap<String, String>,
    },
    #[serde(rename = "hmget")]
    Hmget { key: String, fields: Vec<String> },

    // Key commands
    #[serde(rename = "keys")]
    Keys { pattern: Option<String> },
    #[serde(rename = "del")]
    Del { keys: Vec<String> },

    // Admin commands
    #[serde(rename = "flushall")]
    FlushAll,
    #[serde(rename = "flushdb")]
    FlushDb,
    #[serde(rename = "dbsize")]
    DbSize,
    #[serde(rename = "info")]
    Info,

    // Utility
    #[serde(rename = "list_keys")]
    ListKeys { pattern: Option<String> },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },
}

/// WebSocket message wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub id: Option<String>,
    pub command: WebSocketCommand,
}

/// WebSocket response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub id: Option<String>,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl WebSocketResponse {
    pub fn success(id: Option<String>, data: serde_json::Value) -> Self {
        Self {
            id,
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(id: Option<String>, error: String) -> Self {
        Self {
            id,
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// WebSocket connection state
#[derive(Debug, Clone)]
pub struct WebSocketState {
    pub connection_id: String,
    pub subscribed_channels: std::sync::Arc<tokio::sync::RwLock<Vec<String>>>,
}
