//! Database Adapters Module
//!
//! This module contains adapters for various database systems and services.
//! Each adapter provides a consistent interface for interacting with a specific
//! database technology.

/// Redis adapter for working with Redis databases
pub mod redis;

// Future adapters can be added here:
// pub mod postgres;
// pub mod mysql;
// pub mod mongodb;
// pub mod dynamodb;
// pub mod elasticsearch;

/// Common traits implemented by database adapters
pub mod traits {
    /// Basic database operations that should be supported by all adapters
    pub trait DatabaseAdapter {
        /// The error type returned by this adapter
        type Error;

        /// Check if the database connection is alive
        fn ping(&self) -> Result<bool, Self::Error>;

        /// Close the database connection
        fn close(&self) -> Result<(), Self::Error>;
    }
}

/// Common error types used across adapters
pub mod error {
    /// A generic connection error
    #[derive(Debug, thiserror::Error)]
    pub enum ConnectionError {
        #[error("Failed to connect: {0}")]
        ConnectionFailed(String),

        #[error("Connection timeout: {0}")]
        Timeout(String),

        #[error("Authentication failed: {0}")]
        AuthenticationFailed(String),

        #[error("Connection closed: {0}")]
        ConnectionClosed(String),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_adapter_module_exists() {
        // This test just verifies that the module compiles
        assert!(true);
    }
}
