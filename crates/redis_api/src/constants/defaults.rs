/// Default configuration values used throughout the application
pub struct Defaults;

impl Defaults {
    /// Default database URL for Redis connection
    pub const DATABASE_URL: &'static str = "redis://default:redispw@localhost:55000";

    /// Default server host address
    pub const HOST: &'static str = "0.0.0.0";

    /// Default server port
    pub const PORT: u16 = 3000;

    /// Default connection pool size
    pub const POOL_SIZE: u32 = 10;

    // Test-specific defaults
    /// Default test database URL for Redis connection
    pub const TEST_DATABASE_URL: &'static str = "redis://default:redispw@localhost:55000";

    /// Default test server host address
    pub const TEST_HOST: &'static str = "127.0.0.1";

    /// Default test connection pool size
    pub const TEST_POOL_SIZE: u32 = 5;
}
