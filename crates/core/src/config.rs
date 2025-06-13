use serde::{ Deserialize, Serialize };
use std::time::Duration;

/// Database type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbType {
    Redis,
    Qdrant,
    Postgres,
    MySQL,
    SQLite,
    MongoDB,
    Custom(String),
}

/// Base database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// Database type
    pub db_type: DbType,

    /// Host address
    pub host: String,

    /// Port number
    pub port: u16,

    /// Database name
    pub database: Option<String>,

    /// Username
    pub username: Option<String>,

    /// Password
    pub password: Option<String>,

    /// SSL mode
    pub ssl_mode: Option<String>,

    /// Connection pool size
    pub pool_size: Option<u32>,

    /// Connection timeout in seconds
    pub timeout: Option<u64>,

    /// Additional options
    #[serde(default)]
    pub options: serde_json::Value,
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
            db_type: DbType::Custom("".to_string()),
            host: "localhost".to_string(),
            port: 0,
            database: None,
            username: None,
            password: None,
            ssl_mode: None,
            pool_size: None,
            timeout: None,
            options: serde_json::Value::Null,
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

impl DbConfig {
    /// Create a new database configuration
    pub fn new(db_type: DbType) -> Self {
        Self {
            db_type,
            host: "localhost".to_string(),
            port: 0,
            database: None,
            username: None,
            password: None,
            ssl_mode: None,
            pool_size: None,
            timeout: None,
            options: serde_json::Value::Null,
        }
    }

    /// Set the host
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Set the port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the database name
    pub fn with_database(mut self, database: impl Into<String>) -> Self {
        self.database = Some(database.into());
        self
    }

    /// Set the username
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Set the password
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set the SSL mode
    pub fn with_ssl_mode(mut self, ssl_mode: impl Into<String>) -> Self {
        self.ssl_mode = Some(ssl_mode.into());
        self
    }

    /// Set the pool size
    pub fn with_pool_size(mut self, pool_size: u32) -> Self {
        self.pool_size = Some(pool_size);
        self
    }

    /// Set the timeout
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set additional options
    pub fn with_options(mut self, options: serde_json::Value) -> Self {
        self.options = options;
        self
    }

    /// Get Redis-specific configuration
    pub fn as_redis_config(&self) -> crate::registries::RedisConfig {
        crate::registries::RedisConfig {
            host: self.host.clone(),
            port: self.port,
            password: self.password.clone(),
            db: self.options
                .get("db")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32,
            pool_size: self.pool_size.unwrap_or(10),
            tls: self.ssl_mode.as_deref() == Some("require"),
        }
    }

    /// Get Qdrant-specific configuration
    pub fn as_qdrant_config(&self) -> crate::registries::QdrantConfig {
        crate::registries::QdrantConfig {
            host: self.host.clone(),
            port: self.port,
            api_key: self.password.clone(),
            timeout: self.timeout.unwrap_or(30),
            pool_size: self.pool_size.unwrap_or(10),
            tls: self.ssl_mode.as_deref() == Some("require"),
        }
    }

    /// Get PostgreSQL-specific configuration
    pub fn as_postgres_config(&self) -> crate::registries::PostgresConfig {
        crate::registries::PostgresConfig {
            host: self.host.clone(),
            port: self.port,
            database: self.database.clone().unwrap_or_default(),
            username: self.username.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default(),
            ssl_mode: self.ssl_mode.clone().unwrap_or_else(|| "disable".to_string()),
            pool_size: self.pool_size.unwrap_or(10),
            connection_timeout: self.timeout.unwrap_or(30),
        }
    }
}
