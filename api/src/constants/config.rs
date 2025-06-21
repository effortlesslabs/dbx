/// Default configuration values
pub struct ConfigDefaults;

impl ConfigDefaults {
    /// Default host for the server
    pub const HOST: &'static str = "127.0.0.1";

    /// Default port for the server
    pub const PORT: u16 = 3000;

    /// Default connection pool size
    pub const POOL_SIZE: u32 = 10;

    /// Default database type
    #[allow(dead_code)]
    pub const DATABASE_TYPE: &'static str = "redis";
}
