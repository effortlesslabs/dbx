use std::fmt;
use anyhow;

/// Custom error type for DBX SDK operations
#[derive(Debug)]
pub enum DbxError {
    /// HTTP request failed
    Request(reqwest::Error),
    /// API returned an error response
    Api {
        status: u16,
        message: String,
    },
    /// Invalid URL
    InvalidUrl(url::ParseError),
    /// JSON serialization/deserialization error
    Json(serde_json::Error),
    /// Other errors
    Other(anyhow::Error),
}

impl fmt::Display for DbxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbxError::Request(e) => write!(f, "Request failed: {}", e),
            DbxError::Api { status, message } => write!(f, "API error ({}): {}", status, message),
            DbxError::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            DbxError::Json(e) => write!(f, "JSON error: {}", e),
            DbxError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for DbxError {}

impl From<reqwest::Error> for DbxError {
    fn from(err: reqwest::Error) -> Self {
        DbxError::Request(err)
    }
}

impl From<url::ParseError> for DbxError {
    fn from(err: url::ParseError) -> Self {
        DbxError::InvalidUrl(err)
    }
}

impl From<serde_json::Error> for DbxError {
    fn from(err: serde_json::Error) -> Self {
        DbxError::Json(err)
    }
}

impl From<anyhow::Error> for DbxError {
    fn from(err: anyhow::Error) -> Self {
        DbxError::Other(err)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DbxError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DbxError::Other(anyhow::anyhow!("{}", err))
    }
}

/// Result type for DBX SDK operations
pub type Result<T> = std::result::Result<T, DbxError>;
