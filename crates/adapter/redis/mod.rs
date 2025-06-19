//! Redis adapter module
//!
//! This module provides adapters for interacting with Redis,
//! organized by Redis data type (string, list, hash, set, sorted set).
//! It includes support for individual commands, pipelined operations,
//! transactions, and Lua scripts.

pub mod client;
pub mod primitives;

use redis::{Connection, RedisError, RedisResult, Script};

use client::RedisClient;
use primitives::string::RedisString;

/// Redis data type adapters providing type-specific operations
pub mod types {
    pub use super::primitives::string::RedisString;
    // Other Redis types will be added here as they're implemented:
    // pub use super::primitives::list::RedisList;
    // pub use super::primitives::hash::RedisHash;
    // pub use super::primitives::set::RedisSet;
    // pub use super::primitives::sorted_set::RedisSortedSet;
}

/// Commonly used Redis Lua scripts
pub mod scripts {
    use redis::Script;

    /// Get and set a key atomically
    pub fn get_set() -> Script {
        super::primitives::string::RedisString::get_set_script()
    }

    /// Set a key only if it doesn't exist
    pub fn set_if_not_exists() -> Script {
        super::primitives::string::RedisString::set_if_not_exists_script()
    }

    /// Update a key only if current value matches expected value
    pub fn compare_and_set_with_ttl() -> Script {
        super::primitives::string::RedisString::compare_and_set_with_ttl_script()
    }

    /// Increment multiple counters atomically
    pub fn multi_counter() -> Script {
        super::primitives::string::RedisString::multi_counter_script()
    }

    /// Set multiple keys with TTL atomically
    pub fn multi_set_with_ttl() -> Script {
        super::primitives::string::RedisString::multi_set_with_ttl_script()
    }

    /// Implement a rate limiter pattern
    pub fn rate_limiter() -> Script {
        super::primitives::string::RedisString::rate_limiter_script()
    }
}

/// Redis client wrapper that provides access to all data type adapters
pub struct Redis {
    client: RedisClient,
}

impl Redis {
    /// Create a new Redis instance with the provided client
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    /// Create a new Redis instance from a connection string
    pub fn from_url(url: &str) -> RedisResult<Self> {
        let client = RedisClient::from_url(url)?;
        Ok(Self::new(client))
    }

    /// Get the raw Redis client
    pub fn client(&self) -> &RedisClient {
        &self.client
    }

    /// Get a new connection from the client
    pub fn get_connection(&self) -> RedisResult<Connection> {
        self.client.get_new_connection()
    }

    /// Get access to string operations
    pub fn string(&self) -> RedisString {
        RedisString::new(self.client.connection().clone())
    }

    /// Execute a Lua script directly
    pub fn eval_script<T, K, A>(&self, script: &Script, keys: K, args: A) -> RedisResult<T>
    where
        T: redis::FromRedisValue,
        K: redis::ToRedisArgs,
        A: redis::ToRedisArgs,
    {
        self.string().eval_script(script, keys, args)
    }

    /// Add a Lua script to a pipeline
    pub fn add_script_to_pipeline<'a, K, A>(
        &self,
        pipe: &'a mut redis::Pipeline,
        script: &Script,
        keys: K,
        args: A,
    ) -> &'a mut redis::Pipeline
    where
        K: redis::ToRedisArgs,
        A: redis::ToRedisArgs,
    {
        primitives::string::RedisString::add_script_to_pipeline(pipe, script, keys, args)
    }

    /// Check if the connection is valid
    pub fn ping(&self) -> RedisResult<bool> {
        self.client.ping()
    }

    /// Create batch operations helper for multiple string operations
    pub fn batch() -> BatchOperations {
        BatchOperations::new()
    }

    /// Create a new Redis instance with a connection pool for handling concurrent requests
    #[cfg(feature = "connection-pool")]
    pub fn with_connection_pool(url: &str, pool_size: u32) -> RedisResult<RedisPoolAdapter> {
        let pool = client::create_pool(url, pool_size)?;
        Ok(RedisPoolAdapter::new(pool))
    }
}

/// Batch operations helper for multiple Redis operations
pub struct BatchOperations;

impl BatchOperations {
    /// Create a new batch operations helper
    pub fn new() -> Self {
        Self
    }

    /// Execute multiple SET operations in a pipeline
    pub fn set_many(redis: &Redis, kvs: Vec<(&str, &str)>) -> RedisResult<()> {
        redis.string().set_many(kvs)
    }

    /// Execute multiple GET operations in a pipeline
    pub fn get_many(redis: &Redis, keys: Vec<&str>) -> RedisResult<Vec<Option<String>>> {
        redis.string().get_many(keys)
    }

    /// Execute multiple SETEX operations in a pipeline
    pub fn set_many_with_expiry(redis: &Redis, kvs: Vec<(&str, &str, usize)>) -> RedisResult<()> {
        redis.string().set_many_with_expiry(kvs)
    }

    /// Execute multiple INCR operations in a pipeline
    pub fn incr_many(redis: &Redis, keys: Vec<&str>) -> RedisResult<Vec<i64>> {
        redis.string().incr_many(keys)
    }

    /// Execute multiple INCRBY operations in a pipeline
    pub fn incr_many_by(redis: &Redis, kvs: Vec<(&str, i64)>) -> RedisResult<Vec<i64>> {
        redis.string().incr_many_by(kvs)
    }

    /// Execute multiple DEL operations in a pipeline
    pub fn del_many(redis: &Redis, keys: Vec<&str>) -> RedisResult<()> {
        redis.string().del_many(keys)
    }
}

/// Redis connection pool adapter
#[cfg(feature = "connection-pool")]
pub struct RedisPoolAdapter {
    pool: client::RedisPool,
}

#[cfg(feature = "connection-pool")]
impl RedisPoolAdapter {
    /// Create a new Redis pool adapter
    pub fn new(pool: client::RedisPool) -> Self {
        Self { pool }
    }

    /// Get the pool
    pub fn pool(&self) -> &client::RedisPool {
        &self.pool
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> RedisResult<Connection> {
        self.pool.get_connection()
    }

    /// Get a Redis instance with a connection from the pool
    pub fn get_instance(&self) -> RedisResult<Redis> {
        let connection = self.get_connection()?;
        let client = Client::clone(self.pool.client());
        let redis_client = RedisClient::new(client, connection);
        Ok(Redis::new(redis_client))
    }

    /// Get an asynchronous connection from the pool
    #[cfg(feature = "async")]
    pub async fn get_async_connection(&self) -> RedisResult<redis::aio::Connection> {
        self.pool.get_async_connection().await
    }
}

/// Error types for Redis operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Helper functions for Redis operations
pub mod helpers {
    use super::*;

    /// Create a Redis instance from a connection string
    pub fn create_redis(url: &str) -> RedisResult<Redis> {
        Redis::from_url(url)
    }

    /// Format a Redis error
    pub fn format_error(error: &RedisError) -> String {
        client::format_redis_error(error)
    }
}
