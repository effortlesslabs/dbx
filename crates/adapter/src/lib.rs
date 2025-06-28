//! DBX Adapter library
//!
//! This library provides various adapters and utilities for database interactions.

pub mod redis;
pub mod traits;
pub mod error;
pub use redis::*;

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
