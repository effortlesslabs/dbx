use std::sync::Arc;
use dbx_crates::adapter::redis::Redis;

/// Redis handler that holds the Redis client
#[derive(Clone)]
pub struct RedisHandler {
    pub redis: Arc<Redis>,
}

impl RedisHandler {
    /// Create a new Redis handler
    pub fn new(redis: Arc<Redis>) -> Self {
        Self { redis }
    }
}

// Import handler modules
pub mod string;
pub mod set;
pub mod keys;
pub mod scripts;

// Re-export all handler functions
pub use string::*;
pub use set::*;
pub use keys::*;
