mod connection;
mod error;

pub use connection::RedisConnection;
pub use error::{ RedisError, RedisResult };

// Re-export commonly used types
pub use nucleus::{
    DbConfig,
    DatabaseMetadata,
    DbError,
    DbxDatabase,
    QueryResult,
    DatabaseType,
    RedisDatabaseMetadata,
};

use std::sync::Arc;
use tokio::sync::Mutex;
use redis::{ Client, AsyncCommands, aio::ConnectionManager };
use std::collections::HashMap;

/// Redis data type enum
#[derive(Debug, Clone, PartialEq)]
pub enum RedisDataType {
    String,
    List,
    Set,
    Hash,
    SortedSet,
    Stream,
    Bitmap,
    HyperLogLog,
    Geo,
    Module,
}

/// Redis key pattern metadata
#[derive(Debug, Clone)]
pub struct RedisKeyPattern {
    /// Pattern string (e.g., "user:*")
    pub pattern: String,

    /// Data type of keys matching this pattern
    pub data_type: RedisDataType,

    /// Time-to-live in seconds (if set)
    pub ttl: Option<u64>,

    /// Number of keys matching this pattern
    pub key_count: u64,

    /// Total memory used by keys matching this pattern
    pub memory_usage: u64,

    /// Additional pattern-specific metadata
    pub extra: HashMap<String, serde_json::Value>,
}

/// Redis database implementation
pub struct RedisDatabase {
    connection: Arc<Mutex<RedisConnection>>,
}

#[async_trait::async_trait]
impl DbxDatabase for RedisDatabase {
    async fn connect(&self, config: &DbConfig) -> Result<(), DbError> {
        let mut manager_guard = self.connection.lock().await;
        manager_guard.connect(config).await.map_err(|e| DbError::Connection(e.to_string()))
    }

    async fn query(&self, sql: &str) -> Result<QueryResult, DbError> {
        let manager_guard = self.connection.lock().await;

        // Execute the command and get the result
        let result: String = manager_guard
            .execute_command(|conn| {
                Box::pin(async move { redis::cmd(sql).query_async(conn).await })
            }).await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Convert the result to QueryResult format
        Ok(QueryResult {
            database_type: "redis".to_string(),
            columns: vec!["result".to_string()],
            rows: vec![vec![serde_json::Value::String(result)]],
            rows_affected: 1,
            last_insert_id: None,
            execution_time_ms: 0,
            extra: HashMap::new(),
            database_metadata: None,
        })
    }

    async fn insert(&self, table: &str, data: serde_json::Value) -> Result<(), DbError> {
        let manager_guard = self.connection.lock().await;

        // Convert the data to a Redis command
        let key = format!("{}:{}", table, uuid::Uuid::new_v4());
        let value = serde_json::to_string(&data).map_err(|e| DbError::Query(e.to_string()))?;

        // Set the key-value pair
        manager_guard
            .execute_command(|conn| {
                let key = key.clone();
                let value = value.clone();
                Box::pin(async move {
                    redis::cmd("SET").arg(&key).arg(&value).query_async(conn).await
                })
            }).await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_metadata(&self) -> Result<DatabaseMetadata, DbError> {
        let manager_guard = self.connection.lock().await;

        // Get server info
        let (config, stats) = self
            .collect_server_info().await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Get key patterns
        let key_patterns = self
            .collect_key_patterns().await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Calculate total keys and memory
        let total_keys = key_patterns
            .iter()
            .map(|p| p.key_count)
            .sum();
        let total_memory = key_patterns
            .iter()
            .map(|p| p.memory_usage)
            .sum();

        // Get server version
        let version = stats
            .get("redis_version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        // Create Redis-specific metadata
        let redis_metadata = RedisDatabaseMetadata {
            name: "redis".to_string(),
            version,
            key_patterns,
            total_keys,
            total_memory,
            server_config: config,
            server_stats: stats,
            extra: HashMap::new(),
        };

        // Create and return database metadata
        Ok(DatabaseMetadata {
            name: "redis".to_string(),
            version: redis_metadata.version.clone(),
            tables: Vec::new(), // Redis doesn't have tables
            redis: Some(redis_metadata),
            extra: HashMap::new(),
        })
    }
}

impl RedisDatabase {
    /// Create a new Redis database instance
    pub fn new(config: DbConfig) -> Self {
        let connection = Arc::new(Mutex::new(RedisConnection::new(config)));
        Self { connection }
    }

    /// Collect Redis server information
    async fn collect_server_info(
        &self
    ) -> Result<(HashMap<String, String>, HashMap<String, String>), RedisError> {
        let manager_guard = self.connection.lock().await;

        // Get server configuration
        let config: HashMap<String, String> = manager_guard.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("CONFIG").arg("GET").arg("*").query_async(conn).await
            })
        }).await?;

        // Get server statistics
        let info: String = manager_guard.execute_command(|conn| {
            Box::pin(async move { redis::cmd("INFO").query_async(conn).await })
        }).await?;

        let stats = info
            .lines()
            .filter_map(|line| {
                if line.starts_with('#') || line.is_empty() {
                    None
                } else {
                    line.split_once(':').map(|(k, v)| (k.to_string(), v.to_string()))
                }
            })
            .collect();

        Ok((config, stats))
    }

    /// Collect Redis key patterns and their metadata
    async fn collect_key_patterns(&self) -> Result<Vec<RedisKeyPattern>, RedisError> {
        let manager_guard = self.connection.lock().await;

        // Get all keys
        let keys: Vec<String> = manager_guard.execute_command(|conn| {
            Box::pin(async move { redis::cmd("KEYS").arg("*").query_async(conn).await })
        }).await?;

        // Group keys by pattern
        let mut patterns: HashMap<String, Vec<String>> = HashMap::new();
        for key in keys {
            let pattern = self.extract_pattern(&key);
            patterns.entry(pattern).or_default().push(key);
        }

        // Collect metadata for each pattern
        let mut key_patterns = Vec::new();
        for (pattern, keys) in patterns {
            let mut pattern_metadata = RedisKeyPattern {
                pattern: pattern.clone(),
                data_type: self.get_key_type(&manager_guard, &keys[0]).await?,
                ttl: None,
                key_count: keys.len() as u64,
                memory_usage: 0,
                extra: HashMap::new(),
            };

            // Get TTL for the first key (assuming all keys in pattern have same TTL)
            if
                let Ok(ttl) = manager_guard.execute_command(|conn| {
                    let key = keys[0].clone();
                    Box::pin(async move { redis::cmd("TTL").arg(&key).query_async(conn).await })
                }).await
            {
                pattern_metadata.ttl = if ttl > 0 { Some(ttl as u64) } else { None };
            }

            // Calculate total memory usage
            for key in keys {
                if
                    let Ok(memory) = manager_guard.execute_command(|conn| {
                        let key = key.clone();
                        Box::pin(async move {
                            redis::cmd("MEMORY").arg("USAGE").arg(&key).query_async(conn).await
                        })
                    }).await
                {
                    pattern_metadata.memory_usage += memory as u64;
                }
            }

            key_patterns.push(pattern_metadata);
        }

        Ok(key_patterns)
    }

    /// Extract pattern from a key
    fn extract_pattern(&self, key: &str) -> String {
        // Simple pattern extraction - can be enhanced based on your needs
        if let Some(pos) = key.rfind(':') {
            format!("{}:*", &key[..pos])
        } else {
            key.to_string()
        }
    }

    /// Get Redis data type for a key
    async fn get_key_type(
        &self,
        manager: &RedisConnection,
        key: &str
    ) -> Result<RedisDataType, RedisError> {
        let type_str: String = manager.execute_command(|conn| {
            let key = key.to_string();
            Box::pin(async move { redis::cmd("TYPE").arg(&key).query_async(conn).await })
        }).await?;

        match type_str.to_lowercase().as_str() {
            "string" => Ok(RedisDataType::String),
            "list" => Ok(RedisDataType::List),
            "set" => Ok(RedisDataType::Set),
            "hash" => Ok(RedisDataType::Hash),
            "zset" => Ok(RedisDataType::SortedSet),
            "stream" => Ok(RedisDataType::Stream),
            _ => Ok(RedisDataType::String), // Default to string for unknown types
        }
    }
}
