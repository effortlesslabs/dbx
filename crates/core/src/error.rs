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

    /// Pool errors
    #[error("Pool error: {0}")]
    Pool(String),

    /// Redis-specific errors
    #[error("Redis error: {0}")]
    Redis(String),

    /// Redis key errors
    #[error("Redis key error: {0}")]
    RedisKey(String),

    /// Redis command errors
    #[error("Redis command error: {0}")]
    RedisCommand(String),

    /// Qdrant-specific errors
    #[error("Qdrant error: {0}")]
    Qdrant(String),

    /// Qdrant collection errors
    #[error("Qdrant collection error: {0}")]
    QdrantCollection(String),

    /// Qdrant vector errors
    #[error("Qdrant vector error: {0}")]
    QdrantVector(String),

    /// Qdrant index errors
    #[error("Qdrant index error: {0}")]
    QdrantIndex(String),

    /// PostgreSQL-specific errors
    #[error("PostgreSQL error: {0}")]
    Postgres(String),

    /// PostgreSQL schema errors
    #[error("PostgreSQL schema error: {0}")]
    PostgresSchema(String),

    /// PostgreSQL table errors
    #[error("PostgreSQL table error: {0}")]
    PostgresTable(String),

    /// PostgreSQL constraint errors
    #[error("PostgreSQL constraint error: {0}")]
    PostgresConstraint(String),

    /// Generic IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
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
                DbError::Pool(e) => DbError::Pool(format!("{}: {}", msg.into(), e)),
                DbError::Redis(e) => DbError::Redis(format!("{}: {}", msg.into(), e)),
                DbError::RedisKey(e) => DbError::RedisKey(format!("{}: {}", msg.into(), e)),
                DbError::RedisCommand(e) => DbError::RedisCommand(format!("{}: {}", msg.into(), e)),
                DbError::Qdrant(e) => DbError::Qdrant(format!("{}: {}", msg.into(), e)),
                DbError::QdrantCollection(e) =>
                    DbError::QdrantCollection(format!("{}: {}", msg.into(), e)),
                DbError::QdrantVector(e) => DbError::QdrantVector(format!("{}: {}", msg.into(), e)),
                DbError::QdrantIndex(e) => DbError::QdrantIndex(format!("{}: {}", msg.into(), e)),
                DbError::Postgres(e) => DbError::Postgres(format!("{}: {}", msg.into(), e)),
                DbError::PostgresSchema(e) =>
                    DbError::PostgresSchema(format!("{}: {}", msg.into(), e)),
                DbError::PostgresTable(e) =>
                    DbError::PostgresTable(format!("{}: {}", msg.into(), e)),
                DbError::PostgresConstraint(e) =>
                    DbError::PostgresConstraint(format!("{}: {}", msg.into(), e)),
                DbError::Io(e) => DbError::Io(e),
                DbError::Serialization(e) => DbError::Serialization(e),
                DbError::Unknown(e) => DbError::Unknown(format!("{}: {}", msg.into(), e)),
            }
        })
    }
}
