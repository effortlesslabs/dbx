//! Common traits implemented by database adapters
//!
//! This module defines the standard interfaces that all database adapters
//! should implement to ensure consistency across different database systems.

/// Basic database operations that should be supported by all adapters
pub trait DatabaseAdapter: Send + Sync {
    /// The error type returned by this adapter
    type Error: std::error::Error + Send + Sync;

    /// Get the connection status
    fn is_connected(&self) -> bool;
}

/// Trait for adapters that support ping operations
pub trait PingAdapter: DatabaseAdapter {
    /// Check if the database connection is alive
    fn ping(&self) -> Result<bool, Self::Error>;
}

/// Trait for adapters that support connection management
pub trait ConnectionAdapter: DatabaseAdapter {
    /// Close the database connection
    fn close(&self) -> Result<(), Self::Error>;
}

/// Trait for adapters that support key-value operations
pub trait KeyValueAdapter: DatabaseAdapter {
    /// Get a value by key
    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<String>, Self::Error>;

    /// Set a key-value pair
    fn set<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: V) -> Result<(), Self::Error>;

    /// Set a key-value pair with expiration
    fn set_with_expiry<K: AsRef<str>, V: AsRef<str>>(
        &self,
        key: K,
        value: V,
        expiry_seconds: u64
    ) -> Result<(), Self::Error>;

    /// Delete a key
    fn delete<K: AsRef<str>>(&self, key: K) -> Result<bool, Self::Error>;

    /// Check if a key exists
    fn exists<K: AsRef<str>>(&self, key: K) -> Result<bool, Self::Error>;
}

/// Trait for adapters that support hash operations
pub trait HashAdapter: DatabaseAdapter {
    /// Get a field from a hash
    fn hget<K: AsRef<str>, F: AsRef<str>>(
        &self,
        key: K,
        field: F
    ) -> Result<Option<String>, Self::Error>;

    /// Set a field in a hash
    fn hset<K: AsRef<str>, F: AsRef<str>, V: AsRef<str>>(
        &self,
        key: K,
        field: F,
        value: V
    ) -> Result<(), Self::Error>;

    /// Get all fields from a hash
    fn hgetall<K: AsRef<str>>(
        &self,
        key: K
    ) -> Result<std::collections::HashMap<String, String>, Self::Error>;

    /// Delete a field from a hash
    fn hdel<K: AsRef<str>, F: AsRef<str>>(&self, key: K, field: F) -> Result<bool, Self::Error>;
}

/// Trait for adapters that support set operations
pub trait SetAdapter: DatabaseAdapter {
    /// Add a member to a set
    fn sadd<K: AsRef<str>, M: AsRef<str>>(&self, key: K, member: M) -> Result<bool, Self::Error>;

    /// Remove a member from a set
    fn srem<K: AsRef<str>, M: AsRef<str>>(&self, key: K, member: M) -> Result<bool, Self::Error>;

    /// Check if a member exists in a set
    fn sismember<K: AsRef<str>, M: AsRef<str>>(
        &self,
        key: K,
        member: M
    ) -> Result<bool, Self::Error>;

    /// Get all members of a set
    fn smembers<K: AsRef<str>>(&self, key: K) -> Result<Vec<String>, Self::Error>;
}

/// Trait for adapters that support connection pooling
pub trait PooledAdapter: DatabaseAdapter {
    /// Get a connection from the pool
    fn get_connection(&self) -> Result<Box<dyn DatabaseAdapter<Error = Self::Error>>, Self::Error>;

    /// Return a connection to the pool
    fn return_connection(
        &self,
        connection: Box<dyn DatabaseAdapter<Error = Self::Error>>
    ) -> Result<(), Self::Error>;
}
