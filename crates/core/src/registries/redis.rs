use async_trait::async_trait;
use crate::{ error::DbResult, traits::KeyValueStore };

/// Redis-specific configuration
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i32,
    pub pool_size: u32,
    pub tls: bool,
}

/// Redis registry implementation
#[async_trait]
pub trait RedisRegistry: KeyValueStore {
    /// Get Redis server info
    async fn info(&self) -> DbResult<serde_json::Value>;

    /// Flush all databases
    async fn flush_all(&self) -> DbResult<()>;

    /// Flush current database
    async fn flush_db(&self) -> DbResult<()>;

    /// Get Redis memory usage
    async fn memory_usage(&self) -> DbResult<u64>;

    /// Get Redis client list
    async fn client_list(&self) -> DbResult<Vec<serde_json::Value>>;

    /// Set Redis configuration
    async fn config_set(&self, parameter: &str, value: &str) -> DbResult<()>;

    /// Get Redis configuration
    async fn config_get(&self, parameter: &str) -> DbResult<serde_json::Value>;

    /// Monitor Redis commands
    async fn monitor(&self) -> DbResult<()>;

    /// Get Redis slow log
    async fn slow_log(&self, count: i64) -> DbResult<Vec<serde_json::Value>>;

    /// Get Redis key pattern
    async fn keys(&self, pattern: &str) -> DbResult<Vec<String>>;

    /// Get Redis key type
    async fn type_of(&self, key: &str) -> DbResult<String>;

    /// Get Redis key size
    async fn size_of(&self, key: &str) -> DbResult<u64>;

    /// Get Redis key encoding
    async fn encoding_of(&self, key: &str) -> DbResult<String>;

    /// Get Redis key idle time
    async fn idle_time(&self, key: &str) -> DbResult<u64>;

    /// Get Redis key frequency
    async fn frequency(&self, key: &str) -> DbResult<u64>;

    /// Get Redis key refcount
    async fn refcount(&self, key: &str) -> DbResult<u64>;

    /// Get Redis key memory usage
    async fn memory_usage_of(&self, key: &str) -> DbResult<u64>;

    /// Get Redis key time to live
    async fn ttl_of(&self, key: &str) -> DbResult<i64>;

    /// Get Redis key time to live in milliseconds
    async fn pttl_of(&self, key: &str) -> DbResult<i64>;

    /// Get Redis key time to live in seconds
    async fn ttl_of_seconds(&self, key: &str) -> DbResult<i64>;

    /// Get Redis key time to live in milliseconds
    async fn ttl_of_milliseconds(&self, key: &str) -> DbResult<i64>;

    /// Get Redis key time to live in microseconds
    async fn ttl_of_microseconds(&self, key: &str) -> DbResult<i64>;

    /// Get Redis key time to live in nanoseconds
    async fn ttl_of_nanoseconds(&self, key: &str) -> DbResult<i64>;
}
