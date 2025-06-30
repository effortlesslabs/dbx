//! Common error types used across adapters
//!
//! This module defines standard error types that should be used by all
//! database adapters to ensure consistent error handling.

use thiserror::Error;

/// A generic connection error
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Failed to connect: {0}")]
    ConnectionFailed(String),

    #[error("Connection timeout: {0}")]
    Timeout(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    #[error("Invalid connection URL: {0}")]
    InvalidUrl(String),

    #[error("Connection pool exhausted: {0}")]
    PoolExhausted(String),
}

/// A generic operation error
#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Invalid value format: {0}")]
    InvalidValue(String),

    #[error("Operation timeout: {0}")]
    Timeout(String),

    #[error("Operation failed: {0}")]
    Failed(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// A generic adapter error that combines connection and operation errors
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Connection error: {0}")]
    Connection(#[from] ConnectionError),

    #[error("Operation error: {0}")]
    Operation(#[from] OperationError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<String> for AdapterError {
    fn from(err: String) -> Self {
        AdapterError::Internal(err)
    }
}

impl From<&str> for AdapterError {
    fn from(err: &str) -> Self {
        AdapterError::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AdapterError {
    fn from(err: std::io::Error) -> Self {
        AdapterError::Connection(ConnectionError::ConnectionFailed(err.to_string()))
    }
}

impl From<serde_json::Error> for AdapterError {
    fn from(err: serde_json::Error) -> Self {
        AdapterError::Operation(OperationError::Serialization(err.to_string()))
    }
}
