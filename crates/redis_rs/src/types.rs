use serde::{ Deserialize, Serialize };

// =========================
// String Types
// =========================

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

// =========================
// Set Types
// =========================

/// Request for adding a member to a set
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMemberRequest {
    pub member: String,
}

/// Request for adding multiple members to a set
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMembersRequest {
    pub members: Vec<String>,
}

/// Request for set operations (intersect, union, difference)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetKeysRequest {
    pub keys: Vec<String>,
}

/// Set operation for batch operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetOperation {
    pub key: String,
    pub members: Vec<String>,
}

/// Set information response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetInfo {
    pub key: String,
    pub members: Vec<String>,
    pub cardinality: usize,
    pub ttl: Option<i64>,
}

// =========================
// Generic Response Types
// =========================

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Pattern search results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternResults {
    pub grouped: bool,
    pub results: serde_json::Value,
}

/// Grouped pattern results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupedPatternResult {
    pub pattern: String,
    pub results: std::collections::HashMap<String, Option<String>>,
}
