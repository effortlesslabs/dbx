use async_trait::async_trait;
use crate::{ config::DbConfig, error::DbResult };
use serde_json::Value;

/// Main database trait that all database drivers must implement
#[async_trait]
pub trait DbxDatabase: Send + Sync + std::any::Any {
    /// Connect to the database
    async fn connect(&self, config: &DbConfig) -> DbResult<()>;

    /// Disconnect from the database
    async fn disconnect(&self) -> DbResult<()>;

    /// Execute a query and return results
    async fn query(&self, sql: &str) -> DbResult<Value>;

    /// Execute a prepared query with parameters
    async fn execute_prepared(&self, query: &str, params: &[Value]) -> DbResult<Value>;

    /// Insert data into the database
    async fn insert(&self, table: &str, data: Value) -> DbResult<()>;

    /// Update data in the database
    async fn update(&self, table: &str, data: Value, condition: &str) -> DbResult<u64>;

    /// Delete data from the database
    async fn delete(&self, table: &str, condition: &str) -> DbResult<u64>;

    /// Get database metadata
    async fn get_metadata(&self) -> DbResult<Value>;

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

/// Trait for key-value store operations
#[async_trait]
pub trait KeyValueStore: DbxDatabase {
    /// Set a key-value pair with optional expiration
    async fn set(&self, key: &str, value: &str, ttl: Option<u64>) -> DbResult<()>;

    /// Get a value by key
    async fn get(&self, key: &str) -> DbResult<Option<String>>;

    /// Delete a key
    async fn del(&self, key: &str) -> DbResult<bool>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> DbResult<bool>;

    /// Set key expiration time in seconds
    async fn expire(&self, key: &str, seconds: u64) -> DbResult<bool>;

    /// Get key time to live in seconds
    async fn ttl(&self, key: &str) -> DbResult<i64>;
}

/// Trait for vector database operations
#[async_trait]
pub trait VectorStore: DbxDatabase {
    /// Create a new collection with vector configuration
    async fn create_collection(&self, name: &str, vector_size: u32, distance: &str) -> DbResult<()>;

    /// Delete a collection
    async fn delete_collection(&self, name: &str) -> DbResult<()>;

    /// Insert vectors with metadata
    async fn insert_vectors(&self, collection: &str, vectors: &[(Vec<f32>, Value)]) -> DbResult<()>;

    /// Search for similar vectors
    async fn search_vectors(
        &self,
        collection: &str,
        query_vector: &[f32],
        limit: u32
    ) -> DbResult<Vec<(f32, Value)>>;

    /// Delete vectors by filter
    async fn delete_vectors(&self, collection: &str, filter: &str) -> DbResult<u64>;

    /// Update vector metadata
    async fn update_vectors(&self, collection: &str, vectors: &[(Vec<f32>, Value)]) -> DbResult<()>;

    /// Get vector by ID
    async fn get_vector(&self, collection: &str, id: &str) -> DbResult<Option<(Vec<f32>, Value)>>;
}

/// Extension trait for downcasting database instances
pub trait DbxDatabaseExt: DbxDatabase {
    /// Downcast to Any type
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: DbxDatabase + 'static> DbxDatabaseExt for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Add implementation for trait object
// Remove trait object implementation since it's not supported yet
// See: https://github.com/rust-lang/rust/issues/65991

/// Extension trait for Redis operations
pub trait RedisExt: DbxDatabaseExt {
    /// Get Redis-specific operations
    fn as_redis(&self) -> Option<&dyn crate::registries::RedisRegistry>;
}

/// Extension trait for Qdrant operations
pub trait QdrantExt: DbxDatabaseExt {
    /// Get Qdrant-specific operations
    fn as_qdrant(&self) -> Option<&dyn crate::registries::QdrantRegistry>;
}

/// Extension trait for PostgreSQL operations
pub trait PostgresExt: DbxDatabaseExt {
    /// Get PostgreSQL-specific operations
    fn as_postgres(&self) -> Option<&dyn crate::registries::PostgresRegistry>;
}

impl<T: DbxDatabaseExt> RedisExt for T {
    fn as_redis(&self) -> Option<&dyn crate::registries::RedisRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn crate::registries::RedisRegistry>>()
            .map(|b| b.as_ref())
    }
}

impl<T: DbxDatabaseExt> QdrantExt for T {
    fn as_qdrant(&self) -> Option<&dyn crate::registries::QdrantRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn crate::registries::QdrantRegistry>>()
            .map(|b| b.as_ref())
    }
}

impl<T: DbxDatabaseExt> PostgresExt for T {
    fn as_postgres(&self) -> Option<&dyn crate::registries::PostgresRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn crate::registries::PostgresRegistry>>()
            .map(|b| b.as_ref())
    }
}
