use redis::{ AsyncCommands, aio::ConnectionManager };
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use serde_json::Value;

/// Handles Redis Lua scripting operations
pub struct RedisScript {
    connection: RedisConnection,
    script_cache: std::collections::HashMap<String, String>,
}

impl RedisScript {
    /// Create a new Redis Lua script handler
    pub fn new(connection: RedisConnection) -> Self {
        Self {
            connection,
            script_cache: std::collections::HashMap::new(),
        }
    }

    /// Load a Lua script into Redis
    pub async fn load(&mut self, script: &str) -> RedisResult<String> {
        let mut conn = self.connection.get_connection().await?;
        let script_hash: String = redis
            ::cmd("SCRIPT")
            .arg("LOAD")
            .arg(script)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Script(e.to_string()))?;

        self.script_cache.insert(script_hash.clone(), script.to_string());
        Ok(script_hash)
    }

    /// Execute a Lua script by its hash
    pub async fn execute(
        &self,
        script_hash: &str,
        keys: &[&str],
        args: &[&str]
    ) -> RedisResult<Value> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("EVALSHA").arg(script_hash).arg(keys.len());

        for key in keys {
            cmd.arg(key);
        }
        for arg in args {
            cmd.arg(arg);
        }

        let result: redis::RedisResult<Value> = cmd.query_async(&mut conn).await;
        match result {
            Ok(value) => Ok(value),
            Err(e) if e.to_string().contains("NOSCRIPT") => {
                // Script not found, try to reload it
                if let Some(script) = self.script_cache.get(script_hash) {
                    let mut conn = self.connection.get_connection().await?;
                    let mut cmd = redis::cmd("EVAL").arg(script).arg(keys.len());

                    for key in keys {
                        cmd.arg(key);
                    }
                    for arg in args {
                        cmd.arg(arg);
                    }

                    cmd.query_async(&mut conn).await.map_err(|e| RedisError::Script(e.to_string()))
                } else {
                    Err(RedisError::Script("Script not found in cache".to_string()))
                }
            }
            Err(e) => Err(RedisError::Script(e.to_string())),
        }
    }

    /// Execute a Lua script directly
    pub async fn eval(&self, script: &str, keys: &[&str], args: &[&str]) -> RedisResult<Value> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("EVAL").arg(script).arg(keys.len());

        for key in keys {
            cmd.arg(key);
        }
        for arg in args {
            cmd.arg(arg);
        }

        cmd.query_async(&mut conn).await.map_err(|e| RedisError::Script(e.to_string()))
    }

    /// Check if a script exists in Redis
    pub async fn exists(&self, script_hash: &str) -> RedisResult<bool> {
        let mut conn = self.connection.get_connection().await?;
        let result: Vec<bool> = redis
            ::cmd("SCRIPT")
            .arg("EXISTS")
            .arg(script_hash)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Script(e.to_string()))?;

        Ok(result.get(0).copied().unwrap_or(false))
    }

    /// Flush all loaded scripts from Redis
    pub async fn flush(&self) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("SCRIPT")
            .arg("FLUSH")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Script(e.to_string()))?;
        Ok(())
    }

    /// Kill any running scripts
    pub async fn kill(&self) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("SCRIPT")
            .arg("KILL")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Script(e.to_string()))?;
        Ok(())
    }
}
