use redis::{ AsyncCommands, aio::ConnectionManager };
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;

/// Handles Redis HyperLogLog operations
pub struct RedisHyperLogLog {
    connection: RedisConnection,
}

impl RedisHyperLogLog {
    /// Create a new Redis HyperLogLog handler
    pub fn new(connection: RedisConnection) -> Self {
        Self { connection }
    }

    /// Add elements to a HyperLogLog
    pub async fn add(&self, key: &str, elements: &[&str]) -> RedisResult<bool> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("PFADD").arg(key);

        for element in elements {
            cmd.arg(element);
        }

        let changed: bool = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(changed)
    }

    /// Get the approximate cardinality of a HyperLogLog
    pub async fn count(&self, key: &str) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let count: u64 = redis
            ::cmd("PFCOUNT")
            .arg(key)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(count)
    }

    /// Get the approximate cardinality of multiple HyperLogLogs
    pub async fn count_many(&self, keys: &[&str]) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("PFCOUNT");

        for key in keys {
            cmd.arg(key);
        }

        let count: u64 = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(count)
    }

    /// Merge multiple HyperLogLogs into a destination HyperLogLog
    pub async fn merge(&self, dest_key: &str, source_keys: &[&str]) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("PFMERGE").arg(dest_key);

        for key in source_keys {
            cmd.arg(key);
        }

        cmd.query_async(&mut conn).await.map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(())
    }

    /// Get the internal representation of a HyperLogLog
    pub async fn get(&self, key: &str) -> RedisResult<Vec<u8>> {
        let mut conn = self.connection.get_connection().await?;
        let data: Vec<u8> = redis
            ::cmd("GET")
            .arg(key)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(data)
    }

    /// Set the internal representation of a HyperLogLog
    pub async fn set(&self, key: &str, data: &[u8]) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("SET")
            .arg(key)
            .arg(data)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::HyperLogLog(e.to_string()))?;

        Ok(())
    }
}
