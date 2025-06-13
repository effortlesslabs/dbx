use async_trait::async_trait;
use crate::{ config::DbConfig, error::DbResult, traits::DbxDatabase };

/// PostgreSQL-specific configuration
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
    pub pool_size: u32,
    pub connection_timeout: u64,
}

/// PostgreSQL registry implementation
#[async_trait]
pub trait PostgresRegistry: DbxDatabase {
    /// Get PostgreSQL server version
    async fn version(&self) -> DbResult<String>;

    /// Get PostgreSQL server status
    async fn status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server settings
    async fn settings(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server statistics
    async fn statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server activity
    async fn activity(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server locks
    async fn locks(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server tablespaces
    async fn tablespaces(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server databases
    async fn databases(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server schemas
    async fn schemas(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server tables
    async fn tables(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server views
    async fn views(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server functions
    async fn functions(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server triggers
    async fn triggers(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server indexes
    async fn indexes(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server sequences
    async fn sequences(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server users
    async fn users(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server roles
    async fn roles(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server extensions
    async fn extensions(&self) -> DbResult<Vec<String>>;

    /// Get PostgreSQL server replication status
    async fn replication_status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server backup status
    async fn backup_status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server vacuum status
    async fn vacuum_status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server analyze status
    async fn analyze_status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server autovacuum status
    async fn autovacuum_status(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server deadlocks
    async fn deadlocks(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server blocking queries
    async fn blocking_queries(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server long running queries
    async fn long_running_queries(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server slow queries
    async fn slow_queries(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Get PostgreSQL server query statistics
    async fn query_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server table statistics
    async fn table_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server index statistics
    async fn index_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server function statistics
    async fn function_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server trigger statistics
    async fn trigger_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server sequence statistics
    async fn sequence_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server user statistics
    async fn user_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server role statistics
    async fn role_statistics(&self) -> DbResult<serde_json::Value>;

    /// Get PostgreSQL server extension statistics
    async fn extension_statistics(&self) -> DbResult<serde_json::Value>;
}
