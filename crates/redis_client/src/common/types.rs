use serde::{ Deserialize, Serialize };

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
