//! Rust SDK for DBX Redis API
//!
//! This crate provides a high-level interface for interacting with the DBX Redis API.
//! It supports both string and set operations with a clean, idiomatic Rust API.
//!
//! # Example
//!
//! ```rust,no_run
//! use redis_rs::DbxClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = DbxClient::new("http://localhost:8080")?;
//!
//!     // String operations
//!     client.string().set("my_key", "my_value", None).await?;
//!     let value = client.string().get("my_key").await?;
//!     println!("Value: {:?}", value);
//!
//!     // Set operations
//!     client.set().add_many("my_set", &["member1", "member2"]).await?;
//!     let members = client.set().members("my_set").await?;
//!     println!("Members: {:?}", members);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod string;
pub mod set;
pub mod error;
pub mod types;

pub use client::DbxClient;
pub use error::{ DbxError, Result };
pub use types::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::client::DbxClient;
    pub use crate::error::{ DbxError, Result };
    pub use crate::string::StringClient;
    pub use crate::set::SetClient;
    pub use crate::types::*;
}
