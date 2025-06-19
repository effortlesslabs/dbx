use serde::{ Deserialize, Serialize };

/// Supported database types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    Redis,
    // Future database types
    // Postgres,
    // MongoDB,
    // MySQL,
}

impl std::str::FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "redis" => Ok(DatabaseType::Redis),
            // "postgres" => Ok(DatabaseType::Postgres),
            // "mongodb" => Ok(DatabaseType::MongoDB),
            // "mysql" => Ok(DatabaseType::MySQL),
            _ => Err(format!("Unsupported database type: {}", s)),
        }
    }
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Redis => write!(f, "redis"),
            // DatabaseType::Postgres => write!(f, "postgres"),
            // DatabaseType::MongoDB => write!(f, "mongodb"),
            // DatabaseType::MySQL => write!(f, "mysql"),
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Database type
    pub database_type: DatabaseType,
    /// Database connection URL
    pub database_url: String,
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
            database_type: DatabaseType::Redis,
            database_url: "redis://127.0.0.1:6379".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3000,
            pool_size: 10,
        }
    }

    /// Create a new configuration for a specific database type
    pub fn new_for_database(database_type: DatabaseType, database_url: String) -> Self {
        Self {
            database_type,
            database_url,
            host: "127.0.0.1".to_string(),
            port: 3000,
            pool_size: 10,
        }
    }

    /// Create a new configuration from environment variables
    pub fn from_env() -> Self {
        let database_type = std::env
            ::var("DATABASE_TYPE")
            .unwrap_or_else(|_| "redis".to_string())
            .parse()
            .unwrap_or(DatabaseType::Redis);

        let default_url = match database_type {
            DatabaseType::Redis => "redis://127.0.0.1:6379",
            // DatabaseType::Postgres => "postgresql://localhost:5432/dbx",
            // DatabaseType::MongoDB => "mongodb://localhost:27017/dbx",
            // DatabaseType::MySQL => "mysql://localhost:3306/dbx",
        };

        Self {
            database_type,
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| default_url.to_string()),
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
