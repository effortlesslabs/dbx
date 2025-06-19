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
#[derive(Debug, Serialize)]
pub struct StringValue {
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct IntegerValue {
    pub value: i64,
}

#[derive(Debug, Serialize)]
pub struct BooleanValue {
    pub value: bool,
}

#[derive(Debug, Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct KeyValues {
    pub key_values: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct KeysResponse {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub deleted_count: u64,
}

#[derive(Debug, Serialize)]
pub struct ExistsResponse {
    pub exists: bool,
}

#[derive(Debug, Serialize)]
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
