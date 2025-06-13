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

use serde::{ Serialize, Deserialize };
use thiserror::Error;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_db_config_serialization() {
        let config = DbConfig {
            url: "redis://localhost:6379".to_string(),
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            options: std::collections::HashMap::new(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: DbConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.url, deserialized.url);
        assert_eq!(config.username, deserialized.username);
        assert_eq!(config.password, deserialized.password);
    }

    #[test]
    fn test_query_result_serialization() {
        let result = QueryResult {
            columns: vec!["id".to_string(), "name".to_string()],
            rows: vec![vec![json!(1), json!("test1")], vec![json!(2), json!("test2")]],
        };

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: QueryResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(result.columns, deserialized.columns);
        assert_eq!(result.rows, deserialized.rows);
    }

    #[test]
    fn test_database_metadata_serialization() {
        let metadata = DatabaseMetadata {
            name: "test_db".to_string(),
            version: "1.0.0".to_string(),
            tables: vec![TableMetadata {
                name: "users".to_string(),
                columns: vec![
                    ColumnMetadata {
                        name: "id".to_string(),
                        data_type: "INTEGER".to_string(),
                        is_nullable: false,
                        is_primary_key: true,
                    },
                    ColumnMetadata {
                        name: "name".to_string(),
                        data_type: "TEXT".to_string(),
                        is_nullable: false,
                        is_primary_key: false,
                    }
                ],
            }],
        };

        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: DatabaseMetadata = serde_json::from_str(&serialized).unwrap();

        assert_eq!(metadata.name, deserialized.name);
        assert_eq!(metadata.version, deserialized.version);
        assert_eq!(metadata.tables.len(), deserialized.tables.len());
        assert_eq!(metadata.tables[0].name, deserialized.tables[0].name);
        assert_eq!(metadata.tables[0].columns.len(), deserialized.tables[0].columns.len());
    }

    #[test]
    fn test_db_error_display() {
        let connection_error = DbError::Connection("Failed to connect".to_string());
        let query_error = DbError::Query("Invalid query".to_string());
        let config_error = DbError::Config("Invalid config".to_string());
        let driver_error = DbError::Driver("Driver error".to_string());

        assert_eq!(connection_error.to_string(), "Connection error: Failed to connect");
        assert_eq!(query_error.to_string(), "Query error: Invalid query");
        assert_eq!(config_error.to_string(), "Invalid configuration: Invalid config");
        assert_eq!(driver_error.to_string(), "Driver error: Driver error");
    }
}
