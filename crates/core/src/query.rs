use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Represents a database query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Column names
    pub columns: Vec<String>,

    /// Query result rows
    pub rows: Vec<Vec<serde_json::Value>>,

    /// Number of rows affected by the query
    #[serde(default)]
    pub rows_affected: Option<u64>,

    /// Last inserted ID (if applicable)
    #[serde(default)]
    pub last_insert_id: Option<serde_json::Value>,

    /// Query execution time in milliseconds
    #[serde(default)]
    pub execution_time_ms: Option<u64>,

    /// Additional query-specific metadata
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl QueryResult {
    /// Create a new empty query result
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            rows_affected: None,
            last_insert_id: None,
            execution_time_ms: None,
            extra: HashMap::new(),
        }
    }

    /// Create a new query result with columns and rows
    pub fn with_data(columns: Vec<String>, rows: Vec<Vec<serde_json::Value>>) -> Self {
        Self {
            columns,
            rows,
            rows_affected: None,
            last_insert_id: None,
            execution_time_ms: None,
            extra: HashMap::new(),
        }
    }

    /// Get the number of rows in the result
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns in the result
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if the result is empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get a row by index
    pub fn get_row(&self, index: usize) -> Option<&[serde_json::Value]> {
        self.rows.get(index).map(|row| row.as_slice())
    }

    /// Get a column by name
    pub fn get_column(&self, name: &str) -> Option<Vec<&serde_json::Value>> {
        let col_index = self.columns.iter().position(|col| col == name)?;
        Some(
            self.rows
                .iter()
                .map(|row| &row[col_index])
                .collect()
        )
    }
}

/// Represents a query parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParam {
    /// Parameter name
    pub name: String,

    /// Parameter value
    pub value: serde_json::Value,

    /// Parameter type (if known)
    #[serde(default)]
    pub param_type: Option<String>,
}

/// Represents a prepared query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedQuery {
    /// Query text
    pub query: String,

    /// Query parameters
    pub params: Vec<QueryParam>,

    /// Query timeout in milliseconds
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}
