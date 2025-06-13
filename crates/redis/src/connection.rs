use redis::{ aio::ConnectionManager };
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::{ RedisError, RedisResult };
use nucleus::DbConfig;

/// Manages Redis connection and connection pool
#[derive(Clone)]
pub struct RedisConnection {
    manager: Arc<Mutex<Option<ConnectionManager>>>,
    config: Arc<Mutex<Option<DbConfig>>>,
}

impl RedisConnection {
    /// Create a new Redis connection manager
    pub fn new(config: DbConfig) -> Self {
        Self {
            manager: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(Some(config))),
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
        let _: String = redis
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;
    use std::env;

    fn get_redis_url() -> String {
        dotenv::dotenv().ok();
        env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string())
    }

    async fn setup_test_config() -> DbConfig {
        DbConfig {
            url: get_redis_url(),
            username: None,
            password: None,
            options: std::collections::HashMap::new(),
            pool: None,
            timeout: None,
        }
    }

    #[tokio::test]
    async fn test_connection_lifecycle() {
        let redis = RedisConnection::new(setup_test_config().await);
        let config = setup_test_config().await;

        // Test initial state
        assert!(!redis.is_connected().await);
        assert_eq!(redis.get_config().await.unwrap().url, get_redis_url());

        // Test connection
        redis.connect(&config).await.expect("Failed to connect to Redis");
        assert!(redis.is_connected().await);
        assert_eq!(redis.get_config().await.unwrap().url, get_redis_url());

        // Test disconnection
        redis.disconnect().await.expect("Failed to disconnect from Redis");
        assert!(!redis.is_connected().await);
    }

    #[tokio::test]
    async fn test_command_execution() {
        let redis = RedisConnection::new(setup_test_config().await);
        let config = setup_test_config().await;

        // Connect to Redis
        redis.connect(&config).await.expect("Failed to connect to Redis");

        // Test SET and GET commands
        let test_key = "test_key";
        let test_value = "test_value";

        // Set a value
        let _: () = redis
            .execute_command(|conn| {
                Box::pin(async move {
                    redis::cmd("SET").arg(test_key).arg(test_value).query_async(conn).await
                })
            }).await
            .expect("Failed to execute SET command");

        // Get the value
        let result: String = redis
            .execute_command(|conn| {
                Box::pin(async move { redis::cmd("GET").arg(test_key).query_async(conn).await })
            }).await
            .expect("Failed to execute GET command");

        assert_eq!(result, test_value);

        // Clean up
        let _: () = redis
            .execute_command(|conn| {
                Box::pin(async move { redis::cmd("DEL").arg(test_key).query_async(conn).await })
            }).await
            .expect("Failed to execute DEL command");

        // Disconnect
        redis.disconnect().await.expect("Failed to disconnect from Redis");
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let redis = RedisConnection::new(setup_test_config().await);
        let config = setup_test_config().await;

        // Connect to Redis
        redis.connect(&config).await.expect("Failed to connect to Redis");

        // Spawn multiple tasks that use the connection pool
        let mut handles = vec![];
        for i in 0..5 {
            let redis = redis.clone();
            let handle = tokio::spawn(async move {
                let key = format!("pool_test_key_{}", i);
                let value = format!("pool_test_value_{}", i);

                // Set a value
                let _: () = redis
                    .execute_command(|conn| {
                        let key = key.clone();
                        let value = value.clone();
                        Box::pin(async move {
                            redis::cmd("SET").arg(&key).arg(&value).query_async(conn).await
                        })
                    }).await
                    .expect("Failed to execute SET command");

                // Small delay to simulate work
                sleep(Duration::from_millis(10)).await;

                // Get the value
                let result: String = redis
                    .execute_command(|conn| {
                        let key = key.clone();
                        Box::pin(async move { redis::cmd("GET").arg(&key).query_async(conn).await })
                    }).await
                    .expect("Failed to execute GET command");

                assert_eq!(result, value);

                // Clean up
                let _: () = redis
                    .execute_command(|conn| {
                        let key = key.clone();
                        Box::pin(async move { redis::cmd("DEL").arg(&key).query_async(conn).await })
                    }).await
                    .expect("Failed to execute DEL command");
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.expect("Task failed");
        }

        // Disconnect
        redis.disconnect().await.expect("Failed to disconnect from Redis");
    }

    #[tokio::test]
    async fn test_error_handling() {
        let redis = RedisConnection::new(setup_test_config().await);

        // Try to execute command without connecting
        let result: RedisResult<String> = redis.execute_command(|conn| {
            Box::pin(async move { redis::cmd("PING").query_async(conn).await })
        }).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RedisError::NotConnected));
    }
}
