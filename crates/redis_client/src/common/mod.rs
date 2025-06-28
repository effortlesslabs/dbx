//! Common functionality shared between HTTP and WebSocket clients

pub mod error;
pub mod set;
pub mod string;
pub mod client;
pub mod types;

use crate::error::Result;
use serde_json::Value;
use url::Url;

// Re-export types for convenience
pub use set::*;
pub use string::*;
pub use types::*;
pub use client::HttpClientBase;
#[cfg(feature = "websocket")]
pub use client::WebSocketClientBase;

/// Common trait for string operations
pub trait StringOperations {
    /// Get a string value by key
    async fn get(&mut self, key: &str) -> Result<Option<String>>;

    /// Set a string value
    async fn set(&mut self, key: &str, value: &str, ttl: Option<u64>) -> Result<()>;

    /// Delete a string value
    async fn delete(&mut self, key: &str) -> Result<bool>;

    /// Get string information
    async fn info(&mut self, key: &str) -> Result<Option<StringInfo>>;

    /// Batch get multiple strings
    async fn batch_get(&mut self, keys: &[String]) -> Result<Vec<Option<String>>>;

    /// Batch set multiple strings
    async fn batch_set(&mut self, operations: &[StringOperation]) -> Result<()>;

    /// Get strings by patterns
    async fn get_by_patterns(
        &mut self,
        patterns: &[String],
        grouped: Option<bool>
    ) -> Result<Value>;

    /// Convenience method to set a string without TTL
    async fn set_simple(&mut self, key: &str, value: &str) -> Result<()> {
        self.set(key, value, None).await
    }

    /// Convenience method to set a string with TTL
    async fn set_with_ttl(&mut self, key: &str, value: &str, ttl: u64) -> Result<()> {
        self.set(key, value, Some(ttl)).await
    }
}

/// Common trait for set operations
pub trait SetOperations {
    /// Add a member to a set
    async fn add(&mut self, key: &str, member: &str) -> Result<usize>;

    /// Add multiple members to a set
    async fn add_many(&mut self, key: &str, members: &[&str]) -> Result<usize>;

    /// Remove a member from a set
    async fn remove(&mut self, key: &str, member: &str) -> Result<usize>;

    /// Get all members of a set
    async fn members(&mut self, key: &str) -> Result<Vec<String>>;

    /// Get the cardinality (size) of a set
    async fn cardinality(&mut self, key: &str) -> Result<usize>;

    /// Check if a member exists in a set
    async fn exists(&mut self, key: &str, member: &str) -> Result<bool>;

    /// Intersect multiple sets
    async fn intersect(&mut self, keys: &[String]) -> Result<Vec<String>>;

    /// Union multiple sets
    async fn union(&mut self, keys: &[String]) -> Result<Vec<String>>;

    /// Get the difference of multiple sets (first set minus others)
    async fn difference(&mut self, keys: &[String]) -> Result<Vec<String>>;

    /// Convenience method to add a single member
    async fn add_one(&mut self, key: &str, member: &str) -> Result<usize> {
        self.add(key, member).await
    }

    /// Convenience method to check if a member exists
    async fn contains(&mut self, key: &str, member: &str) -> Result<bool> {
        self.exists(key, member).await
    }

    /// Convenience method to get set size
    async fn size(&mut self, key: &str) -> Result<usize> {
        self.cardinality(key).await
    }
}

/// Common client trait
pub trait Client {
    /// Get the base URL
    fn base_url(&self) -> &Url;
}
