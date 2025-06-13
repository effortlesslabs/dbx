mod connection;
mod error;

pub use connection::RedisConnection;
pub use error::{ RedisError, RedisResult };

// Re-export commonly used types
pub use nucleus::{ DbConfig, DbError, DbResult, DbxDatabase, KeyValueStore, RedisRegistry };
pub use serde_json::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use async_trait::async_trait;

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

#[async_trait]
impl DbxDatabase for RedisConnection {
    async fn connect(&self, config: &DbConfig) -> DbResult<()> {
        self.connect(config).await.map_err(|e| DbError::Connection(e.to_string()))
    }

    async fn disconnect(&self) -> DbResult<()> {
        self.disconnect().await.map_err(|e| DbError::Connection(e.to_string()))
    }

    async fn query(&self, sql: &str) -> DbResult<Value> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: String = redis::cmd("EVAL").arg(sql).arg(0).query_async(conn).await?;
                Ok(serde_json::from_str(&result).unwrap_or(Value::Null))
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn execute_prepared(&self, query: &str, params: &[Value]) -> DbResult<Value> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let mut cmd = redis::cmd("EVAL");
                cmd.arg(query).arg(0);
                for param in params {
                    cmd.arg(param.to_string());
                }
                let result: String = cmd.query_async(conn).await?;
                Ok(serde_json::from_str(&result).unwrap_or(Value::Null))
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn insert(&self, table: &str, data: Value) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let key = format!("{}:{}", table, data["id"].as_str().unwrap_or(""));
                redis::cmd("HSET").arg(&key).arg(data.to_string()).query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn update(&self, table: &str, data: Value, condition: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let key = format!("{}:{}", table, data["id"].as_str().unwrap_or(""));
                let result: u64 = redis
                    ::cmd("HSET")
                    .arg(&key)
                    .arg(data.to_string())
                    .query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn delete(&self, table: &str, condition: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let pattern = format!("{}:*", table);
                let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query_async(conn).await?;
                if keys.is_empty() {
                    return Ok(0);
                }
                let result: u64 = redis::cmd("DEL").arg(&keys).query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn get_metadata(&self) -> DbResult<Value> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let info: String = redis::cmd("INFO").query_async(conn).await?;
                Ok(serde_json::from_str(&info).unwrap_or(Value::Null))
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn begin_transaction(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("MULTI").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Transaction(e.to_string()))
    }

    async fn commit(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("EXEC").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Transaction(e.to_string()))
    }

    async fn rollback(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("DISCARD").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Transaction(e.to_string()))
    }

    async fn is_connected(&self) -> bool {
        self.is_connected().await
    }

    async fn get_version(&self) -> DbResult<String> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let info: String = redis::cmd("INFO").arg("server").query_async(conn).await?;
                let version = info
                    .lines()
                    .find(|line| line.starts_with("redis_version:"))
                    .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                Ok(version)
            })
        }).await.map_err(|e| DbError::Query(e.to_string()))
    }

    async fn ping(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("PING").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Connection(e.to_string()))
    }
}

#[async_trait]
impl KeyValueStore for RedisConnection {
    async fn set(&self, key: &str, value: &str, ttl: Option<u64>) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let mut cmd = redis::cmd("SET");
                cmd.arg(key).arg(value);
                if let Some(ttl) = ttl {
                    cmd.arg("EX").arg(ttl);
                }
                cmd.query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }

    async fn get(&self, key: &str) -> DbResult<Option<String>> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Option<String> = redis::cmd("GET").arg(key).query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }

    async fn del(&self, key: &str) -> DbResult<bool> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: u64 = redis::cmd("DEL").arg(key).query_async(conn).await?;
                Ok(result > 0)
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }

    async fn exists(&self, key: &str) -> DbResult<bool> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: u64 = redis::cmd("EXISTS").arg(key).query_async(conn).await?;
                Ok(result > 0)
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }

    async fn expire(&self, key: &str, seconds: u64) -> DbResult<bool> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: u64 = redis
                    ::cmd("EXPIRE")
                    .arg(key)
                    .arg(seconds)
                    .query_async(conn).await?;
                Ok(result > 0)
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }

    async fn ttl(&self, key: &str) -> DbResult<i64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: i64 = redis::cmd("TTL").arg(key).query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::RedisCommand(e.to_string()))
    }
}

#[async_trait]
impl RedisRegistry for RedisConnection {
    async fn info(&self) -> DbResult<Value> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let info: String = redis::cmd("INFO").query_async(conn).await?;
                Ok(serde_json::from_str(&info).unwrap_or(Value::Null))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn flush_all(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("FLUSHALL").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn flush_db(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("FLUSHDB").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn memory_usage(&self) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let info: String = redis::cmd("INFO").arg("memory").query_async(conn).await?;
                let used_memory = info
                    .lines()
                    .find(|line| line.starts_with("used_memory:"))
                    .map(|line|
                        line.split(':').nth(1).unwrap_or("0").trim().parse::<u64>().unwrap_or(0)
                    )
                    .unwrap_or(0);
                Ok(used_memory)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn client_list(&self) -> DbResult<Vec<Value>> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let info: String = redis::cmd("CLIENT").arg("LIST").query_async(conn).await?;
                let clients: Vec<Value> = info
                    .lines()
                    .map(|line| {
                        let mut map = serde_json::Map::new();
                        for part in line.split(' ') {
                            if let Some((key, value)) = part.split_once('=') {
                                map.insert(key.to_string(), Value::String(value.to_string()));
                            }
                        }
                        Value::Object(map)
                    })
                    .collect();
                Ok(clients)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn config_set(&self, parameter: &str, value: &str) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("CONFIG").arg("SET").arg(parameter).arg(value).query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn config_get(&self, parameter: &str) -> DbResult<Value> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<String> = redis
                    ::cmd("CONFIG")
                    .arg("GET")
                    .arg(parameter)
                    .query_async(conn).await?;
                let mut map = serde_json::Map::new();
                for chunk in result.chunks(2) {
                    if chunk.len() == 2 {
                        map.insert(chunk[0].clone(), Value::String(chunk[1].clone()));
                    }
                }
                Ok(Value::Object(map))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn monitor(&self) -> DbResult<()> {
        self.execute_command(|conn| {
            Box::pin(async move {
                redis::cmd("MONITOR").query_async(conn).await?;
                Ok(())
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn slow_log(&self, count: i64) -> DbResult<Vec<Value>> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<Vec<String>> = redis
                    ::cmd("SLOWLOG")
                    .arg("GET")
                    .arg(count)
                    .query_async(conn).await?;
                let logs: Vec<Value> = result
                    .into_iter()
                    .map(|log| {
                        let mut map = serde_json::Map::new();
                        for (i, value) in log.into_iter().enumerate() {
                            map.insert(i.to_string(), Value::String(value));
                        }
                        Value::Object(map)
                    })
                    .collect();
                Ok(logs)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn keys(&self, pattern: &str) -> DbResult<Vec<String>> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let keys: Vec<String> = redis::cmd("KEYS").arg(pattern).query_async(conn).await?;
                Ok(keys)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn type_of(&self, key: &str) -> DbResult<String> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let type_str: String = redis::cmd("TYPE").arg(key).query_async(conn).await?;
                Ok(type_str)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn size_of(&self, key: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let type_str: String = redis::cmd("TYPE").arg(key).query_async(conn).await?;
                let size = match type_str.as_str() {
                    "string" => {
                        let len: usize = redis::cmd("STRLEN").arg(key).query_async(conn).await?;
                        len as u64
                    }
                    "list" => {
                        let len: i64 = redis::cmd("LLEN").arg(key).query_async(conn).await?;
                        len as u64
                    }
                    "set" => {
                        let len: i64 = redis::cmd("SCARD").arg(key).query_async(conn).await?;
                        len as u64
                    }
                    "hash" => {
                        let len: i64 = redis::cmd("HLEN").arg(key).query_async(conn).await?;
                        len as u64
                    }
                    "zset" => {
                        let len: i64 = redis::cmd("ZCARD").arg(key).query_async(conn).await?;
                        len as u64
                    }
                    _ => 0,
                };
                Ok(size)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn encoding_of(&self, key: &str) -> DbResult<String> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<String> = redis
                    ::cmd("OBJECT")
                    .arg("ENCODING")
                    .arg(key)
                    .query_async(conn).await?;
                Ok(result.join(" "))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn idle_time(&self, key: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<String> = redis
                    ::cmd("OBJECT")
                    .arg("IDLETIME")
                    .arg(key)
                    .query_async(conn).await?;
                Ok(result.join(" ").parse::<u64>().unwrap_or(0))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn frequency(&self, key: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<String> = redis
                    ::cmd("OBJECT")
                    .arg("FREQ")
                    .arg(key)
                    .query_async(conn).await?;
                Ok(result.join(" ").parse::<u64>().unwrap_or(0))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn refcount(&self, key: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: Vec<String> = redis
                    ::cmd("OBJECT")
                    .arg("REFCOUNT")
                    .arg(key)
                    .query_async(conn).await?;
                Ok(result.join(" ").parse::<u64>().unwrap_or(0))
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn memory_usage_of(&self, key: &str) -> DbResult<u64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: i64 = redis
                    ::cmd("MEMORY")
                    .arg("USAGE")
                    .arg(key)
                    .query_async(conn).await?;
                Ok(result as u64)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn ttl_of(&self, key: &str) -> DbResult<i64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: i64 = redis::cmd("TTL").arg(key).query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn pttl_of(&self, key: &str) -> DbResult<i64> {
        self.execute_command(|conn| {
            Box::pin(async move {
                let result: i64 = redis::cmd("PTTL").arg(key).query_async(conn).await?;
                Ok(result)
            })
        }).await.map_err(|e| DbError::Redis(e.to_string()))
    }

    async fn ttl_of_seconds(&self, key: &str) -> DbResult<i64> {
        self.ttl_of(key).await
    }

    async fn ttl_of_milliseconds(&self, key: &str) -> DbResult<i64> {
        self.pttl_of(key).await
    }

    async fn ttl_of_microseconds(&self, key: &str) -> DbResult<i64> {
        self.pttl_of(key).await.map(|ms| ms * 1000)
    }

    async fn ttl_of_nanoseconds(&self, key: &str) -> DbResult<i64> {
        self.pttl_of(key).await.map(|ms| ms * 1_000_000)
    }
}
