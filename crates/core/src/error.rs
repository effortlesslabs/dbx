use thiserror::Error;
use std::time::Duration;

/// Database operation errors
#[derive(Debug, Error)]
pub enum DbError {
    /// Connection-related errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Query execution errors
    #[error("Query error: {0}")]
    Query(String),

    /// Configuration errors
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Driver-specific errors
    #[error("Driver error: {0}")]
    Driver(String),

    /// Timeout errors
    #[error("Operation timed out after {0:?}")]
    Timeout(Duration),

    /// Transaction errors
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Data type conversion errors
    #[error("Data type conversion error: {0}")]
    DataType(String),

    /// Resource not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Resource already exists errors
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Permission errors
    #[error("Permission denied: {0}")]
    Permission(String),
}

/// Result type for database operations
pub type DbResult<T> = Result<T, DbError>;

/// Extension trait for adding context to errors
pub trait ErrorContext {
    fn context(self, msg: impl Into<String>) -> Self;
}

impl<T> ErrorContext for DbResult<T> {
    fn context(self, msg: impl Into<String>) -> Self {
        self.map_err(|e| {
            match e {
                DbError::Connection(e) => DbError::Connection(format!("{}: {}", msg.into(), e)),
                DbError::Query(e) => DbError::Query(format!("{}: {}", msg.into(), e)),
                DbError::Config(e) => DbError::Config(format!("{}: {}", msg.into(), e)),
                DbError::Driver(e) => DbError::Driver(format!("{}: {}", msg.into(), e)),
                DbError::Timeout(d) => DbError::Timeout(d),
                DbError::Transaction(e) => DbError::Transaction(format!("{}: {}", msg.into(), e)),
                DbError::Auth(e) => DbError::Auth(format!("{}: {}", msg.into(), e)),
                DbError::DataType(e) => DbError::DataType(format!("{}: {}", msg.into(), e)),
                DbError::NotFound(e) => DbError::NotFound(format!("{}: {}", msg.into(), e)),
                DbError::AlreadyExists(e) =>
                    DbError::AlreadyExists(format!("{}: {}", msg.into(), e)),
                DbError::Permission(e) => DbError::Permission(format!("{}: {}", msg.into(), e)),
            }
        })
    }
}
