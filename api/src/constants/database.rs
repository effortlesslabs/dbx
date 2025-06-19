/// Database connection URLs with environment variable support
pub struct DatabaseUrls;

impl DatabaseUrls {
    /// Default Redis connection URL (can be overridden by REDIS_URL env var)
    pub const REDIS_DEFAULT: &'static str = "redis://127.0.0.1:6379";

    /// Default PostgreSQL connection URL (can be overridden by POSTGRES_URL env var)
    pub const POSTGRES_DEFAULT: &'static str = "postgresql://localhost:5432/dbx";

    /// Default MongoDB connection URL (can be overridden by MONGODB_URL env var)
    pub const MONGODB_DEFAULT: &'static str = "mongodb://localhost:27017/dbx";

    /// Default MySQL connection URL (can be overridden by MYSQL_URL env var)
    pub const MYSQL_DEFAULT: &'static str = "mysql://localhost:3306/dbx";

    /// Test Redis connection URL (can be overridden by REDIS_TEST_URL env var)
    pub const REDIS_TEST: &'static str = "redis://default:redispw@localhost:55000";
}

impl DatabaseUrls {
    /// Get Redis URL with environment variable override
    pub fn redis_url() -> String {
        std::env::var("REDIS_URL").unwrap_or_else(|_| Self::REDIS_DEFAULT.to_string())
    }

    /// Get PostgreSQL URL with environment variable override
    pub fn postgres_url() -> String {
        std::env::var("POSTGRES_URL").unwrap_or_else(|_| Self::POSTGRES_DEFAULT.to_string())
    }

    /// Get MongoDB URL with environment variable override
    pub fn mongodb_url() -> String {
        std::env::var("MONGODB_URL").unwrap_or_else(|_| Self::MONGODB_DEFAULT.to_string())
    }

    /// Get MySQL URL with environment variable override
    pub fn mysql_url() -> String {
        std::env::var("MYSQL_URL").unwrap_or_else(|_| Self::MYSQL_DEFAULT.to_string())
    }

    /// Get test Redis URL with environment variable override
    pub fn redis_test_url() -> String {
        std::env::var("REDIS_TEST_URL").unwrap_or_else(|_| Self::REDIS_TEST.to_string())
    }
}

/// Database patterns and keys
pub struct DatabasePatterns;

impl DatabasePatterns {
    /// Default key pattern for listing keys
    pub const DEFAULT_KEY_PATTERN: &'static str = "*";

    /// Maximum TTL value
    pub const MAX_TTL: usize = usize::MAX;

    /// Default TTL value
    pub const DEFAULT_TTL: u64 = 0;
}
