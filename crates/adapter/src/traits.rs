//! Common traits implemented by database adapters
//!
//! This module defines the standard interfaces that all database adapters
//! should implement to ensure consistency across different database systems.

use async_trait::async_trait;

/// Basic database operations that should be supported by all adapters
#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    /// The error type returned by this adapter
    type Error: std::error::Error + Send + Sync;

    /// Check if the database connection is alive
    async fn ping(&self) -> Result<bool, Self::Error>;

    /// Close the database connection
    async fn close(&self) -> Result<(), Self::Error>;

    /// Get the connection status
    fn is_connected(&self) -> bool;
}

/// Trait for adapters that support key-value operations
#[async_trait]
pub trait KeyValueAdapter: DatabaseAdapter {
    /// Get a value by key
    async fn get<K: AsRef<str> + Send>(&self, key: K) -> Result<Option<String>, Self::Error>;

    /// Set a key-value pair
    async fn set<K: AsRef<str> + Send, V: AsRef<str> + Send>(
        &self,
        key: K,
        value: V
    ) -> Result<(), Self::Error>;

    /// Set a key-value pair with expiration
    async fn set_with_expiry<K: AsRef<str> + Send, V: AsRef<str> + Send>(
        &self,
        key: K,
        value: V,
        expiry_seconds: u64
    ) -> Result<(), Self::Error>;

    /// Delete a key
    async fn delete<K: AsRef<str> + Send>(&self, key: K) -> Result<bool, Self::Error>;

    /// Check if a key exists
    async fn exists<K: AsRef<str> + Send>(&self, key: K) -> Result<bool, Self::Error>;
}

/// Trait for adapters that support hash operations
#[async_trait]
pub trait HashAdapter: DatabaseAdapter {
    /// Get a field from a hash
    async fn hget<K: AsRef<str> + Send, F: AsRef<str> + Send>(
        &self,
        key: K,
        field: F
    ) -> Result<Option<String>, Self::Error>;

    /// Set a field in a hash
    async fn hset<K: AsRef<str> + Send, F: AsRef<str> + Send, V: AsRef<str> + Send>(
        &self,
        key: K,
        field: F,
        value: V
    ) -> Result<(), Self::Error>;

    /// Get all fields from a hash
    async fn hgetall<K: AsRef<str> + Send>(
        &self,
        key: K
    ) -> Result<std::collections::HashMap<String, String>, Self::Error>;

    /// Delete a field from a hash
    async fn hdel<K: AsRef<str> + Send, F: AsRef<str> + Send>(
        &self,
        key: K,
        field: F
    ) -> Result<bool, Self::Error>;
}

/// Trait for adapters that support set operations
#[async_trait]
pub trait SetAdapter: DatabaseAdapter {
    /// Add a member to a set
    async fn sadd<K: AsRef<str> + Send, M: AsRef<str> + Send>(
        &self,
        key: K,
        member: M
    ) -> Result<bool, Self::Error>;

    /// Remove a member from a set
    async fn srem<K: AsRef<str> + Send, M: AsRef<str> + Send>(
        &self,
        key: K,
        member: M
    ) -> Result<bool, Self::Error>;

    /// Check if a member exists in a set
    async fn sismember<K: AsRef<str> + Send, M: AsRef<str> + Send>(
        &self,
        key: K,
        member: M
    ) -> Result<bool, Self::Error>;

    /// Get all members of a set
    async fn smembers<K: AsRef<str> + Send>(&self, key: K) -> Result<Vec<String>, Self::Error>;
}

/// Trait for adapters that support connection pooling
#[async_trait]
pub trait PooledAdapter: DatabaseAdapter {
    /// Get a connection from the pool
    async fn get_connection(
        &self
    ) -> Result<Box<dyn DatabaseAdapter<Error = Self::Error>>, Self::Error>;

    /// Return a connection to the pool
    async fn return_connection(
        &self,
        connection: Box<dyn DatabaseAdapter<Error = Self::Error>>
    ) -> Result<(), Self::Error>;
}
