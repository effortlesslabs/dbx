use crate::constants::{ config::ConfigDefaults, database::DatabaseUrls, errors::ErrorMessages };
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
    pub fn new(database_type: DatabaseType) -> Self {
        let default_url = match database_type {
            DatabaseType::Redis => DatabaseUrls::redis_url(),
            // DatabaseType::Postgres => DatabaseUrls::postgres_url(),
            // DatabaseType::MongoDB => DatabaseUrls::mongodb_url(),
            // DatabaseType::MySQL => DatabaseUrls::mysql_url(),
        };

        Self {
            database_type,
            database_url: default_url,
            host: ConfigDefaults::HOST.to_string(),
            port: ConfigDefaults::PORT,
            pool_size: ConfigDefaults::POOL_SIZE,
        }
    }

    /// Create a new configuration from environment variables
    pub fn from_env() -> Self {
        let database_type = std::env
            ::var("DATABASE_TYPE")
            .unwrap_or_else(|_| ConfigDefaults::DATABASE_TYPE.to_string())
            .parse()
            .unwrap_or(DatabaseType::Redis);

        let default_url = match database_type {
            DatabaseType::Redis => DatabaseUrls::redis_url(),
            // DatabaseType::Postgres => DatabaseUrls::postgres_url(),
            // DatabaseType::MongoDB => DatabaseUrls::mongodb_url(),
            // DatabaseType::MySQL => DatabaseUrls::mysql_url(),
        };

        Self {
            database_type,
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| default_url),
            host: std::env::var("HOST").unwrap_or_else(|_| ConfigDefaults::HOST.to_string()),
            port: std::env
                ::var("PORT")
                .unwrap_or_else(|_| ConfigDefaults::PORT.to_string())
                .parse()
                .unwrap_or(ConfigDefaults::PORT),
            pool_size: std::env
                ::var("POOL_SIZE")
                .unwrap_or_else(|_| ConfigDefaults::POOL_SIZE.to_string())
                .parse()
                .unwrap_or(ConfigDefaults::POOL_SIZE),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(DatabaseType::Redis)
    }
}
