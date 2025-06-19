//! DBX Crates library
//!
//! This library provides various adapters and utilities for database interactions.

/// Adapter modules for different database systems and services
pub mod adapter;

/// Re-export the Redis adapter for easier access
pub use adapter::redis;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty(), "Version should be defined");
    }
}
