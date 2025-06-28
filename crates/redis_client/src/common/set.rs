use serde::{ Deserialize, Serialize };

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
