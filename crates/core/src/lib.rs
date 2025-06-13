mod config;
mod error;
mod metadata;
mod query;
mod traits;

pub use config::*;
pub use error::*;
pub use metadata::*;
pub use query::*;
pub use traits::*;

// Re-export commonly used types
pub use serde_json::Value;
pub use tokio::sync::OnceCell;

/// Represents a database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub url: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub options: std::collections::HashMap<String, String>,
}

/// Represents a database query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// Represents database metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    pub name: String,
    pub version: String,
    pub tables: Vec<TableMetadata>,
}

/// Represents table metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    pub name: String,
    pub columns: Vec<ColumnMetadata>,
}

/// Represents column metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMetadata {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
}

/// Database operation errors
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Connection error: {0}")] Connection(String),

    #[error("Query error: {0}")] Query(String),

    #[error("Invalid configuration: {0}")] Config(String),

    #[error("Driver error: {0}")] Driver(String),
}

/// Main database trait that all database drivers must implement
#[async_trait::async_trait]
pub trait DbxDatabase: Send + Sync {
    /// Connect to the database
    async fn connect(&self, config: &DbConfig) -> Result<(), DbError>;

    /// Execute a query and return results
    async fn query(&self, sql: &str) -> Result<QueryResult, DbError>;

    /// Insert data into the database
    async fn insert(&self, table: &str, data: serde_json::Value) -> Result<(), DbError>;

    /// Get database metadata
    async fn get_metadata(&self) -> Result<DatabaseMetadata, DbError>;
}

/// Global database instance
pub static DB: OnceCell<Box<dyn DbxDatabase>> = OnceCell::const_new();
