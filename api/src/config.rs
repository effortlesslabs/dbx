use serde::{ Deserialize, Serialize };

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Redis connection URL
    pub redis_url: String,
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Connection pool size
    pub pool_size: u32,
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self {
            redis_url: "redis://127.0.0.1:6379".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3000,
            pool_size: 10,
        }
    }

    /// Create a new configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            redis_url: std::env
                ::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env
                ::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            pool_size: std::env
                ::var("POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
