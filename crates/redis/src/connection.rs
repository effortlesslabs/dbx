use redis::{ Client, AsyncCommands, aio::ConnectionManager };
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::{ RedisError, RedisResult };
use dbx_core::config::DbConfig;

/// Manages Redis connection and connection pool
pub struct RedisConnection {
    manager: Arc<Mutex<Option<ConnectionManager>>>,
    config: Arc<Mutex<Option<DbConfig>>>,
}

impl RedisConnection {
    /// Create a new Redis connection manager
    pub fn new() -> Self {
        Self {
            manager: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(None)),
        }
    }

    /// Connect to Redis with the given configuration
    pub async fn connect(&self, config: &DbConfig) -> RedisResult<()> {
        let client = redis::Client
            ::open(config.url.as_str())
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        // Configure connection pool
        let manager = ConnectionManager::new(client).await.map_err(|e|
            RedisError::Connection(e.to_string())
        )?;

        // Test the connection
        let mut test_conn = manager.clone();
        redis
            ::cmd("PING")
            .query_async(&mut test_conn).await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        // Store the connection manager and config
        let mut manager_guard = self.manager.lock().await;
        *manager_guard = Some(manager);

        let mut config_guard = self.config.lock().await;
        *config_guard = Some(config.clone());

        Ok(())
    }

    /// Disconnect from Redis
    pub async fn disconnect(&self) -> RedisResult<()> {
        let mut manager_guard = self.manager.lock().await;
        *manager_guard = None;
        Ok(())
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> RedisResult<ConnectionManager> {
        let manager_guard = self.manager.lock().await;
        manager_guard.as_ref().cloned().ok_or(RedisError::NotConnected)
    }

    /// Execute a command using a connection from the pool
    pub async fn execute_command<F, T>(&self, f: F) -> RedisResult<T>
        where F: FnOnce(&mut ConnectionManager) -> redis::RedisFuture<T>
    {
        let mut manager = self.get_connection().await?;
        f(&mut manager).await.map_err(|e| RedisError::Command(e.to_string()))
    }

    /// Check if connected to Redis
    pub async fn is_connected(&self) -> bool {
        self.manager.lock().await.is_some()
    }

    /// Get the current configuration
    pub async fn get_config(&self) -> Option<DbConfig> {
        self.config.lock().await.clone()
    }
}
