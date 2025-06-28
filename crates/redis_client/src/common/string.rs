use serde::{ Deserialize, Serialize };

/// Request for setting a string value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStringRequest {
    pub value: String,
    pub ttl: Option<u64>,
}

/// Request for batch getting strings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchGetRequest {
    pub keys: Vec<String>,
}

/// Request for batch setting strings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchSetRequest {
    pub operations: Vec<StringOperation>,
}

/// Request for getting strings by patterns
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchGetPatternsRequest {
    pub patterns: Vec<String>,
    pub grouped: Option<bool>,
}

/// String operation for batch operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringOperation {
    pub key: String,
    pub value: Option<String>,
    pub ttl: Option<u64>,
}

/// String information response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringInfo {
    pub key: String,
    pub value: String,
    pub ttl: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
    pub encoding: String,
    pub size: usize,
}
