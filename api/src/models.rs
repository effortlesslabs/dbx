use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
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
pub struct FloatValue {
    pub value: f64,
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
    pub deleted_count: i64,
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

// List Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListLength {
    pub length: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListPushRequest {
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPushSingleRequest {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ListRangeRequest {
    pub start: i64,
    pub stop: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListIndexRequest {
    pub index: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListSetRequest {
    pub index: i64,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ListInsertRequest {
    pub pivot: String,
    pub value: String,
    pub before: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRemoveRequest {
    pub count: i64,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTrimRequest {
    pub start: i64,
    pub stop: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListPopRequest {
    pub count: Option<isize>,
}

#[derive(Debug, Deserialize)]
pub struct ListBlockingPopRequest {
    pub keys: Vec<String>,
    pub timeout: usize,
}

// Set Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct SetMembersResponse {
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCardinalityResponse {
    pub cardinality: i64,
}

#[derive(Debug, Deserialize)]
pub struct SetAddRequest {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetRemoveRequest {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetMemberRequest {
    pub member: String,
}

#[derive(Debug, Deserialize)]
pub struct SetOperationRequest {
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetOperationStoreRequest {
    pub destination: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetRandomMemberRequest {
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SetMoveRequest {
    pub destination: String,
    pub member: String,
}

// Sorted Set Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetMember {
    pub member: String,
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetResponse {
    pub members: Vec<SortedSetMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetMembersResponse {
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetRankResponse {
    pub rank: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetScoreResponse {
    pub score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortedSetScoresResponse {
    pub scores: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetAddRequest {
    pub members: Vec<SortedSetMember>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRemoveRequest {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRangeRequest {
    pub start: i64,
    pub stop: i64,
    pub with_scores: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRangeByScoreRequest {
    pub min: f64,
    pub max: f64,
    pub with_scores: Option<bool>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRangeByLexRequest {
    pub min: String,
    pub max: String,
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetIncrByRequest {
    pub member: String,
    pub increment: f64,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetCountRequest {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRemoveRangeRequest {
    pub start: i64,
    pub stop: i64,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetRemoveByScoreRequest {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetUnionRequest {
    pub keys: Vec<String>,
    pub weights: Option<Vec<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct SortedSetInterRequest {
    pub keys: Vec<String>,
    pub weights: Option<Vec<f64>>,
}

// Hash Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct HashFieldValue {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashResponse {
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashFieldsResponse {
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashValuesResponse {
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashLengthResponse {
    pub length: i64,
}

#[derive(Debug, Deserialize)]
pub struct HashSetRequest {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct HashSetMultipleRequest {
    pub fields: Vec<HashFieldValue>,
}

#[derive(Debug, Deserialize)]
pub struct HashGetRequest {
    pub field: String,
}

#[derive(Debug, Deserialize)]
pub struct HashGetMultipleRequest {
    pub fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HashDeleteRequest {
    pub fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HashIncrByRequest {
    pub field: String,
    pub increment: i64,
}

#[derive(Debug, Deserialize)]
pub struct HashIncrByFloatRequest {
    pub field: String,
    pub increment: f64,
}

#[derive(Debug, Deserialize)]
pub struct HashExistsRequest {
    pub field: String,
}

// Stream Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamEntry {
    pub id: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResponse {
    pub entries: Vec<StreamEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamLengthResponse {
    pub length: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamIdResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamAddRequest {
    pub fields: HashMap<String, String>,
    pub id: Option<String>,
    pub maxlen: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StreamReadRequest {
    pub streams: HashMap<String, String>, // stream_name -> last_id
    pub count: Option<i64>,
    pub block: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct StreamRangeRequest {
    pub start: String,
    pub end: String,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StreamDeleteRequest {
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamTrimRequest {
    pub maxlen: Option<i64>,
    pub min_id: Option<String>,
    pub approximate: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct StreamGroupCreateRequest {
    pub group: String,
    pub id: String,
    pub mkstream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct StreamGroupReadRequest {
    pub group: String,
    pub consumer: String,
    pub streams: HashMap<String, String>,
    pub count: Option<i64>,
    pub block: Option<u64>,
    pub noack: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct StreamAckRequest {
    pub group: String,
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamClaimRequest {
    pub group: String,
    pub consumer: String,
    pub min_idle_time: u64,
    pub ids: Vec<String>,
}

// Bitmap Operation Models

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapBitResponse {
    pub bit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapCountResponse {
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapPositionResponse {
    pub position: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapOperationResponse {
    pub length: i64,
}

#[derive(Debug, Deserialize)]
pub struct BitmapSetRequest {
    pub offset: u64,
    pub value: bool,
}

#[derive(Debug, Deserialize)]
pub struct BitmapGetRequest {
    pub offset: u64,
}

#[derive(Debug, Deserialize)]
pub struct BitmapCountRequest {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BitmapPositionRequest {
    pub bit: bool,
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BitmapOperationRequest {
    pub operation: String, // AND, OR, XOR, NOT
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BitmapSetMultipleRequest {
    pub operations: Vec<BitmapBitOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapBitOperation {
    pub offset: u64,
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapGetMultipleRequest {
    pub offsets: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BitmapMultipleBitsResponse {
    pub bits: Vec<bool>,
}
