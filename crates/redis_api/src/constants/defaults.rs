/// Default configuration values used throughout the application
pub struct Defaults;

impl Defaults {
    /// Default Redis URL for connection
    pub const REDIS_URL: &'static str = "redis://default:redispw@localhost:55000";

    /// Default server host address
    pub const HOST: &'static str = "0.0.0.0";

    /// Default server port
    pub const PORT: u16 = 3000;

    /// Default connection pool size
    pub const POOL_SIZE: u32 = 10;
}
