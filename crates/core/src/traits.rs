use async_trait::async_trait;
use crate::{ config::DbConfig, error::{ DbResult } };

/// Main database trait that all database drivers must implement
#[async_trait]
pub trait DbxDatabase: Send + Sync {
    /// Connect to the database
    async fn connect(&self, config: &DbConfig) -> DbResult<()>;

    /// Disconnect from the database
    async fn disconnect(&self) -> DbResult<()>;

    /// Execute a query and return results
    // async fn query(&self, sql: &str) -> DbResult<QueryResult>;

    // /// Execute a prepared query with parameters
    // async fn execute_prepared(&self, query: &PreparedQuery) -> DbResult<QueryResult>;

    /// Insert data into the database
    async fn insert(&self, table: &str, data: serde_json::Value) -> DbResult<()>;

    /// Update data in the database
    async fn update(&self, table: &str, data: serde_json::Value, condition: &str) -> DbResult<u64>;

    /// Delete data from the database
    async fn delete(&self, table: &str, condition: &str) -> DbResult<u64>;

    // /// Get database metadata
    // async fn get_metadata(&self) -> DbResult<DatabaseMetadata>;

    /// Begin a transaction
    async fn begin_transaction(&self) -> DbResult<()>;

    /// Commit the current transaction
    async fn commit(&self) -> DbResult<()>;

    /// Rollback the current transaction
    async fn rollback(&self) -> DbResult<()>;

    /// Check if the database is connected
    async fn is_connected(&self) -> bool;

    /// Get the database version
    async fn get_version(&self) -> DbResult<String>;

    /// Ping the database to check connectivity
    async fn ping(&self) -> DbResult<()>;
}

/// Trait for database drivers that support transactions
#[async_trait]
pub trait TransactionSupport: DbxDatabase {
    /// Begin a new transaction
    async fn begin(&self) -> DbResult<()>;

    /// Commit the current transaction
    async fn commit(&self) -> DbResult<()>;

    /// Rollback the current transaction
    async fn rollback(&self) -> DbResult<()>;

    /// Check if a transaction is active
    async fn is_transaction_active(&self) -> bool;
}

/// Trait for database drivers that support connection pooling
#[async_trait]
pub trait ConnectionPoolSupport: DbxDatabase {
    /// Get the current pool size
    async fn pool_size(&self) -> DbResult<u32>;

    /// Get the number of active connections
    async fn active_connections(&self) -> DbResult<u32>;

    /// Get the number of idle connections
    async fn idle_connections(&self) -> DbResult<u32>;
}
