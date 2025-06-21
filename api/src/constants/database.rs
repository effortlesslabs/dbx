/// Database connection URLs with environment variable support
pub struct DatabaseUrls;

impl DatabaseUrls {
    /// Default Redis connection URL (can be overridden by REDIS_URL env var)
    pub const REDIS_DEFAULT: &'static str = "redis://127.0.0.1:6379";

    /// Test Redis connection URL (can be overridden by REDIS_TEST_URL env var)
    pub const REDIS_TEST: &'static str = "redis://default:redispw@localhost:55000";
}

impl DatabaseUrls {
    /// Get Redis URL with environment variable override
    pub fn redis_url() -> String {
        std::env::var("REDIS_URL").unwrap_or_else(|_| Self::REDIS_DEFAULT.to_string())
    }

    /// Get test Redis URL with environment variable override
    pub fn redis_test_url() -> String {
        std::env::var("REDIS_TEST_URL").unwrap_or_else(|_| Self::REDIS_TEST.to_string())
    }
}

/// Database patterns and keys
pub struct DatabasePatterns;

impl DatabasePatterns {
    /// Maximum TTL value
    pub const MAX_TTL: usize = usize::MAX;

    /// Default TTL value
    pub const DEFAULT_TTL: u64 = 0;
}
