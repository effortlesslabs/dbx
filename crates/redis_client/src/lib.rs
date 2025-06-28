#![allow(async_fn_in_trait)]

//! Rust SDK for DBX Redis API
//!
//! This crate provides a high-level interface for interacting with the DBX Redis API.
//! It supports both HTTP and WebSocket protocols for string and set operations.
//!
//! # Features
//!
//! - **http**: HTTP client support (enabled by default)
//! - **websocket**: WebSocket client support
//! - **string**: String operations support (enabled by default)
//! - **set**: Set operations support (enabled by default)
//!
//! # Example
//!
//! ```rust,no_run
//! use redis_client::{HttpClient, StringOperations, SetOperations};
//! #[cfg(feature = "websocket")]
//! use redis_client::WsClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // HTTP client
//!     let http_client = HttpClient::new("http://localhost:8080")?;
//!
//!     // String operations via HTTP
//!     let mut string_client = http_client.string();
//!     string_client.set("my_key", "my_value", None).await?;
//!     let value = string_client.get("my_key").await?;
//!     println!("Value: {:?}", value);
//!
//!     // Set operations via HTTP
//!     let mut set_client = http_client.set();
//!     set_client.add_many("my_set", &["member1", "member2"]).await?;
//!     let members = set_client.members("my_set").await?;
//!     println!("Members: {:?}", members);
//!
//!     // WebSocket client (only if websocket feature is enabled)
//!     #[cfg(feature = "websocket")]
//!     {
//!         let mut ws_client = WsClient::new("ws://localhost:8080/ws").await?;
//!
//!         // String operations via WebSocket
//!         let mut ws_string_client = ws_client.string();
//!         ws_string_client.set("ws_key", "ws_value", None).await?;
//!         let ws_value = ws_string_client.get("ws_key").await?;
//!         println!("WS Value: {:?}", ws_value);
//!     }
//!
//!     Ok(())
//! }
//! ```

// Common functionality
pub mod common;

// Protocol-specific modules
#[cfg(feature = "http")]
pub mod redis; // HTTP operations
#[cfg(feature = "websocket")]
pub mod redis_ws; // WebSocket operations

pub use common::error::{ DbxError, Result };
pub use common::*;

// Re-export clients based on features
#[cfg(feature = "http")]
pub use redis::HttpClient;
#[cfg(feature = "websocket")]
pub use redis_ws::WsClient;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::common::error::{ DbxError, Result };
    pub use crate::common::*;

    #[cfg(feature = "http")]
    pub use crate::redis::HttpClient;
    #[cfg(feature = "websocket")]
    pub use crate::redis_ws::WsClient;
}
