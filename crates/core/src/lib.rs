mod config;
mod error;
mod traits;

pub use config::*;
pub use error::*;
pub use traits::*;

// Re-export commonly used types
pub use serde_json::Value;
pub use tokio::sync::OnceCell;

use thiserror::Error;

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

    // /// Execute a query and return results
    // async fn query(&self, args: &str) -> Result<QueryResult, DbError>;

    /// Insert data into the database
    async fn insert(&self, table: &str, data: serde_json::Value) -> Result<(), DbError>;

    // /// Get database metadata
    // async fn get_metadata(&self) -> Result<DatabaseMetadata, DbError>;
}

/// Global database instance
pub static DB: OnceCell<Box<dyn DbxDatabase>> = OnceCell::const_new();
