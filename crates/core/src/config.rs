use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::time::Duration;

/// Represents a database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// Database connection URL
    pub url: String,

    /// Optional username for authentication
    #[serde(default)]
    pub username: Option<String>,

    /// Optional password for authentication
    #[serde(default)]
    pub password: Option<String>,

    /// Additional connection options
    #[serde(default)]
    pub options: HashMap<String, String>,

    /// Connection pool configuration
    #[serde(default)]
    pub pool: Option<PoolConfig>,

    /// Timeout configuration
    #[serde(default)]
    pub timeout: Option<TimeoutConfig>,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of idle connections
    #[serde(default = "default_min_idle")]
    pub min_idle: u32,

    /// Maximum lifetime of a connection
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime: Duration,

    /// Idle timeout for connections
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: Duration,
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Connection timeout
    #[serde(default = "default_connect_timeout")]
    pub connect: Duration,

    /// Query timeout
    #[serde(default = "default_query_timeout")]
    pub query: Duration,

    /// Transaction timeout
    #[serde(default = "default_transaction_timeout")]
    pub transaction: Duration,
}

// Default values
fn default_max_connections() -> u32 {
    10
}
fn default_min_idle() -> u32 {
    2
}
fn default_max_lifetime() -> Duration {
    Duration::from_secs(1800)
}
fn default_idle_timeout() -> Duration {
    Duration::from_secs(300)
}
fn default_connect_timeout() -> Duration {
    Duration::from_secs(5)
}
fn default_query_timeout() -> Duration {
    Duration::from_secs(30)
}
fn default_transaction_timeout() -> Duration {
    Duration::from_secs(60)
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            username: None,
            password: None,
            options: HashMap::new(),
            pool: Some(PoolConfig::default()),
            timeout: Some(TimeoutConfig::default()),
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            min_idle: default_min_idle(),
            max_lifetime: default_max_lifetime(),
            idle_timeout: default_idle_timeout(),
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect: default_connect_timeout(),
            query: default_query_timeout(),
            transaction: default_transaction_timeout(),
        }
    }
}
