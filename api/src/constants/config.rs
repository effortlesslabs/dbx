/// Default configuration values
pub struct ConfigDefaults;

impl ConfigDefaults {
    /// Default host for the server
    pub const HOST: &'static str = "127.0.0.1";

    /// Default port for the server
    pub const PORT: u16 = 3000;

    /// Default connection pool size
    pub const POOL_SIZE: u32 = 10;

    /// Default log level
    pub const LOG_LEVEL: &'static str = "INFO";

    /// Default database type
    pub const DATABASE_TYPE: &'static str = "redis";
}

/// Environment variable names
pub struct EnvVars;

impl EnvVars {
    /// Database URL environment variable
    pub const DATABASE_URL: &'static str = "DATABASE_URL";

    /// Host environment variable
    pub const HOST: &'static str = "HOST";

    /// Port environment variable
    pub const PORT: &'static str = "PORT";

    /// Pool size environment variable
    pub const POOL_SIZE: &'static str = "POOL_SIZE";

    /// Log level environment variable
    pub const LOG_LEVEL: &'static str = "LOG_LEVEL";

    /// Database type environment variable
    pub const DATABASE_TYPE: &'static str = "DATABASE_TYPE";
}
