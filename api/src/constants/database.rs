/// Database connection URLs
pub struct DatabaseUrls;

impl DatabaseUrls {
    /// Default Redis connection URL
    pub const REDIS_DEFAULT: &'static str = "redis://127.0.0.1:6379";

    /// Default PostgreSQL connection URL
    pub const POSTGRES_DEFAULT: &'static str = "postgresql://localhost:5432/dbx";

    /// Default MongoDB connection URL
    pub const MONGODB_DEFAULT: &'static str = "mongodb://localhost:27017/dbx";

    /// Default MySQL connection URL
    pub const MYSQL_DEFAULT: &'static str = "mysql://localhost:3306/dbx";

    /// Test Redis connection URL
    pub const REDIS_TEST: &'static str = "redis://default:redispw@localhost:55000";
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
