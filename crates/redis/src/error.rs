use thiserror::Error;
use nucleus::DbError;

/// Redis-specific error types
#[derive(Error, Debug)]
pub enum RedisError {
    /// Connection-related errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Query execution errors
    #[error("Query error: {0}")]
    Query(String),

    /// Invalid data type errors
    #[error("Invalid data type: {0}")]
    InvalidDataType(String),

    /// Not connected to Redis
    #[error("Not connected to Redis")]
    NotConnected,

    /// Transaction-related errors
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Prepared statement errors
    #[error("Prepared statement error: {0}")]
    PreparedStatement(String),

    /// Command execution errors
    #[error("Command execution error: {0}")]
    Command(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Pub/Sub errors
    #[error("Pub/Sub error: {0}")]
    PubSub(String),

    /// Script errors
    #[error("Script error: {0}")]
    Script(String),

    /// Pipeline errors
    #[error("Pipeline error: {0}")]
    Pipeline(String),

    /// Stream errors
    #[error("Stream error: {0}")]
    Stream(String),

    /// HyperLogLog errors
    #[error("HyperLogLog error: {0}")]
    HyperLogLog(String),
}

impl From<RedisError> for DbError {
    fn from(err: RedisError) -> Self {
        match err {
            RedisError::Connection(e) => DbError::Connection(e),
            RedisError::Query(e) => DbError::Query(e),
            RedisError::InvalidDataType(e) => DbError::InvalidDataType(e),
            RedisError::NotConnected => DbError::Connection("Not connected to Redis".to_string()),
            RedisError::Transaction(e) => DbError::Transaction(e),
            RedisError::PreparedStatement(e) => DbError::Query(e),
            RedisError::Command(e) => DbError::Query(e),
            RedisError::Config(e) => DbError::Config(e),
            RedisError::PubSub(e) => DbError::Custom(format!("Pub/Sub error: {}", e)),
            RedisError::Script(e) => DbError::Custom(format!("Script error: {}", e)),
            RedisError::Pipeline(e) => DbError::Custom(format!("Pipeline error: {}", e)),
            RedisError::Stream(e) => DbError::Custom(format!("Stream error: {}", e)),
            RedisError::HyperLogLog(e) => DbError::Custom(format!("HyperLogLog error: {}", e)),
        }
    }
}

/// Result type for Redis operations
pub type RedisResult<T> = Result<T, RedisError>;
