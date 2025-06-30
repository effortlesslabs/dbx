//! DBX Adapter library
//!
//! This library provides various adapters and utilities for database interactions.

pub mod error;
pub mod redis;
pub mod traits;
pub use redis::*;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

// Load environment variables from .env file for tests
#[cfg(test)]
#[ctor::ctor]
fn init() {
    dotenv::dotenv().ok();
}

#[cfg(test)]
mod test_helpers {
    use std::env;

    /// Get Redis URL from environment variable with fallback to default
    pub fn get_test_redis_url() -> String {
        env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://default:redispw@localhost:55000".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty(), "Version should be defined");
    }

    #[test]
    fn test_redis_url_from_env() {
        use test_helpers::get_test_redis_url;

        // Test that the function returns a valid URL
        let url = get_test_redis_url();
        println!("Redis URL from environment: {}", url);
        assert!(!url.is_empty(), "Redis URL should not be empty");
        assert!(
            url.starts_with("redis://"),
            "Redis URL should start with redis://"
        );
    }
}
