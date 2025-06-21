use serde::{ Deserialize, Serialize };
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

/// String command types for RedisWs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StringCommand {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    Del {
        key: String,
    },
    Exists {
        key: String,
    },
    Ttl {
        key: String,
    },
}

/// Set command types for RedisWs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SetCommand {
    Sadd {
        key: String,
        members: Vec<String>,
    },
    Srem {
        key: String,
        members: Vec<String>,
    },
    Smembers {
        key: String,
    },
    Scard {
        key: String,
    },
    Sismember {
        key: String,
        member: String,
    },
    Spop {
        key: String,
        count: Option<usize>,
    },
}

// RedisWs Models

/// RedisWs command types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "action", content = "params")]
pub enum RedisWsCommand {
    // String commands
    #[serde(rename = "get")] Get {
        key: String,
    },
    #[serde(rename = "set")] Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "delete")] Delete {
        key: String,
    },
    #[serde(rename = "exists")] Exists {
        key: String,
    },
    #[serde(rename = "ttl")] Ttl {
        key: String,
    },
    #[serde(rename = "incr")] Incr {
        key: String,
    },
    #[serde(rename = "incrby")] IncrBy {
        key: String,
        increment: i64,
    },
    #[serde(rename = "setnx")] SetNx {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "cas")] CompareAndSet {
        key: String,
        expected_value: String,
        new_value: String,
        ttl: Option<u64>,
    },
    #[serde(rename = "batch_get")] BatchGet {
        keys: Vec<String>,
    },
    #[serde(rename = "batch_set")] BatchSet {
        key_values: HashMap<String, String>,
        ttl: Option<u64>,
    },
    #[serde(rename = "batch_delete")] BatchDelete {
        keys: Vec<String>,
    },
    #[serde(rename = "batch_incr")] BatchIncr {
        keys: Vec<String>,
    },
    #[serde(rename = "batch_incrby")] BatchIncrBy {
        key_increments: Vec<(String, i64)>,
    },

    // Set commands
    #[serde(rename = "sadd")] Sadd {
        key: String,
        members: Vec<String>,
    },
    #[serde(rename = "srem")] Srem {
        key: String,
        members: Vec<String>,
    },
    #[serde(rename = "smembers")] Smembers {
        key: String,
    },
    #[serde(rename = "scard")] Scard {
        key: String,
    },
    #[serde(rename = "sismember")] Sismember {
        key: String,
        member: String,
    },
    #[serde(rename = "spop")] Spop {
        key: String,
    },
    #[serde(rename = "srandmember")] Srandmember {
        key: String,
    },
    #[serde(rename = "smove")] Smove {
        source: String,
        destination: String,
        member: String,
    },
    #[serde(rename = "sunion")] Sunion {
        keys: Vec<String>,
    },
    #[serde(rename = "sinter")] Sinter {
        keys: Vec<String>,
    },
    #[serde(rename = "sdiff")] Sdiff {
        keys: Vec<String>,
    },

    // Hash commands
    #[serde(rename = "hset")] Hset {
        key: String,
        field: String,
        value: String,
    },
    #[serde(rename = "hget")] Hget {
        key: String,
        field: String,
    },
    #[serde(rename = "hdel")] Hdel {
        key: String,
        field: String,
    },
    #[serde(rename = "hexists")] Hexists {
        key: String,
        field: String,
    },
    #[serde(rename = "hlen")] Hlen {
        key: String,
    },
    #[serde(rename = "hkeys")] Hkeys {
        key: String,
    },
    #[serde(rename = "hvals")] Hvals {
        key: String,
    },
    #[serde(rename = "hgetall")] Hgetall {
        key: String,
    },
    #[serde(rename = "hmset")] Hmset {
        key: String,
        fields: HashMap<String, String>,
    },
    #[serde(rename = "hmget")] Hmget {
        key: String,
        fields: Vec<String>,
    },

    // Key commands
    #[serde(rename = "keys")] Keys {
        pattern: Option<String>,
    },
    #[serde(rename = "del")] Del {
        keys: Vec<String>,
    },

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
    #[serde(rename = "list_keys")] ListKeys {
        pattern: Option<String>,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "subscribe")] Subscribe {
        channels: Vec<String>,
    },
    #[serde(rename = "unsubscribe")] Unsubscribe {
        channels: Vec<String>,
    },
}

/// RedisWs message wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisWsMessage {
    pub id: Option<String>,
    pub command: RedisWsCommand,
}

/// RedisWs response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub enum RedisWsResponse {
    StringValue {
        value: Option<String>,
    },
    IntegerValue {
        value: i64,
    },
    BooleanValue {
        value: bool,
    },
    ArrayValue {
        value: Vec<String>,
    },
    ObjectValue {
        value: std::collections::HashMap<String, String>,
    },
    Success {
        id: Option<String>,
        data: serde_json::Value,
    },
    Error {
        id: Option<String>,
        error: String,
    },
}

impl RedisWsResponse {
    pub fn error(id: Option<String>, error: String) -> Self {
        Self::Error { id, error }
    }
}

/// RedisWs connection state
#[derive(Debug, Clone)]
pub struct RedisWsState {
    pub connection_id: String,
    pub subscribed_channels: std::sync::Arc<tokio::sync::RwLock<Vec<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct BatchGetRequest {
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchDeleteRequest {
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchIncrRequest {
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchIncrByRequest {
    pub key_increments: Vec<(String, i64)>,
}
