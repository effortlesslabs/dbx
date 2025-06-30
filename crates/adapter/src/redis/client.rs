//! Redis client module
//!
//! This module provides client functionality for establishing and managing
//! Redis connections, including support for connection pooling and different
//! connection types.

use redis::{Client, Connection, RedisError, RedisResult};
use std::sync::{Arc, Mutex};

use super::primitives::hash::RedisHash;
use super::primitives::set::RedisSet;
use super::primitives::string::RedisString;

/// A simple Redis client wrapper that manages a single connection
#[derive(Clone)]
pub struct RedisClient {
    client: Arc<Client>,
    connection: Arc<Mutex<Connection>>,
}

impl RedisClient {
    /// Create a new Redis client from a connection string
    ///
    /// # Example
    /// ```no_run
    /// # use dbx_adapter::redis::client::RedisClient;
    /// let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    /// let client = RedisClient::from_url(&redis_url).unwrap();
    /// ```
    pub fn from_url(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        let connection = client.get_connection()?;
        Ok(Self {
            client: Arc::new(client),
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// Create a new Redis client from an existing client and connection
    pub fn new(client: Client, connection: Connection) -> Self {
        Self {
            client: Arc::new(client),
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// Get the raw Redis client
    pub fn client(&self) -> &Arc<Client> {
        &self.client
    }

    /// Get the connection
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.connection
    }

    /// Get a new connection from the client
    pub fn get_new_connection(&self) -> RedisResult<Connection> {
        self.client.get_connection()
    }

    /// Check if the connection is valid
    pub fn ping(&self) -> RedisResult<bool> {
        let mut conn = self.connection.lock().unwrap();
        let pong: String = redis::cmd("PING").query(&mut *conn)?;
        Ok(pong == "PONG")
    }

    /// Get a RedisString primitive for string operations
    pub fn string(&self) -> RedisString {
        RedisString::new(self.connection.clone())
    }

    /// Get a RedisSet primitive for set operations
    pub fn set(&self) -> RedisSet {
        RedisSet::new(self.connection.clone())
    }

    /// Get a RedisHash primitive for hash operations
    pub fn hash(&self) -> RedisHash {
        RedisHash::new(self.connection.clone())
    }
}

/// A Redis connection pool for handling concurrent requests
/// This is available when the "connection-pool" feature is enabled
#[cfg(feature = "connection-pool")]
pub struct RedisPool {
    client: Arc<Client>,
    pool_size: u32,
}

#[cfg(feature = "connection-pool")]
impl RedisPool {
    /// Create a new Redis pool with the specified pool size
    ///
    /// # Example
    /// ```no_run
    /// # use dbx_adapter::redis::client::RedisPool;
    /// let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    /// let pool = RedisPool::new(&redis_url, 10).unwrap();
    /// ```
    pub fn new(url: &str, pool_size: u32) -> RedisResult<Self> {
        let client = Client::open(url)?;
        Ok(Self {
            client: Arc::new(client),
            pool_size,
        })
    }

    /// Get the pool size
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    /// Get the raw Redis client
    pub fn client(&self) -> &Arc<Client> {
        &self.client
    }

    /// Get a synchronous connection from the pool
    pub fn get_connection(&self) -> RedisResult<Connection> {
        self.client.get_connection()
    }

    /// Get an asynchronous connection from the pool
    #[cfg(feature = "async")]
    pub async fn get_async_connection(&self) -> RedisResult<redis::aio::Connection> {
        self.client.get_async_connection().await
    }
}

#[cfg(feature = "connection-pool")]
impl Clone for RedisPool {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            pool_size: self.pool_size,
        }
    }
}

/// A trait for Redis clients that provides common functionality
pub trait RedisClientTrait {
    /// Get a connection from the client
    fn get_connection(&self) -> RedisResult<Connection>;

    /// Check if the connection is valid
    fn ping(&self) -> RedisResult<bool>;
}

impl RedisClientTrait for RedisClient {
    fn get_connection(&self) -> RedisResult<Connection> {
        self.get_new_connection()
    }

    fn ping(&self) -> RedisResult<bool> {
        self.ping()
    }
}

#[cfg(feature = "connection-pool")]
impl RedisClientTrait for RedisPool {
    fn get_connection(&self) -> RedisResult<Connection> {
        self.get_connection()
    }

    fn ping(&self) -> RedisResult<bool> {
        let mut conn = self.get_connection()?;
        let pong: String = redis::cmd("PING").query(&mut conn)?;
        Ok(pong == "PONG")
    }
}

/// Create a Redis client from a connection string
pub fn create_client(url: &str) -> RedisResult<RedisClient> {
    RedisClient::from_url(url)
}

/// Create a Redis pool from a connection string with the specified pool size
#[cfg(feature = "connection-pool")]
pub fn create_pool(url: &str, pool_size: u32) -> RedisResult<RedisPool> {
    RedisPool::new(url, pool_size)
}

/// Convert a Redis error to a standard error message
pub fn format_redis_error(error: &RedisError) -> String {
    format!("Redis error: {}", error)
}
