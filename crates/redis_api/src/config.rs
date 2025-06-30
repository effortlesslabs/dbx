use serde::{ Deserialize, Serialize };
use std::str::FromStr;

/// Supported database types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    Redis,
    // Future database types
    // Postgres,
    // MongoDB,
    // MySQL,
}

impl FromStr for DatabaseType {
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
    /// Database connection URL
    pub database_url: String,
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Connection pool size
    pub pool_size: u32,
}
