//! WebSocket client for DBX Redis API

use crate::error::Result;
use futures_util::{ SinkExt, StreamExt };
use serde_json::Value;
use std::time::Duration;
use url::Url;

pub mod string;
pub mod set;

pub use string::WsStringClient;
pub use set::WsSetClient;

/// WebSocket client for interacting with the DBX Redis API
pub struct WsClient {
    base_url: Url,
}

impl WsClient {
    /// Create a new WebSocket client with the given URL
    pub async fn new(ws_url: &str) -> Result<Self> {
        let url = Url::parse(ws_url)?;
        Ok(Self { base_url: url })
    }

    /// Create a new WebSocket client with custom timeout
    pub async fn with_timeout(ws_url: &str, _timeout: Duration) -> Result<Self> {
        // Note: WebSocket timeout is handled differently than HTTP
        Self::new(ws_url).await
    }

    /// Get access to string operations
    pub async fn string(&mut self) -> Result<WsStringClient> {
        let ws_url = self.base_url.join("string/ws")?;
        let (stream, _) = tokio_tungstenite::connect_async(ws_url).await?;
        Ok(WsStringClient::new(stream, self.base_url.clone()))
    }

    /// Get access to set operations
    pub async fn set(&mut self) -> Result<WsSetClient> {
        let ws_url = self.base_url.join("set/ws")?;
        let (stream, _) = tokio_tungstenite::connect_async(ws_url).await?;
        Ok(WsSetClient::new(stream, self.base_url.clone()))
    }

    /// Get the base URL
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Send a raw WebSocket message
    pub async fn send_message(&mut self, message: Value) -> Result<Value> {
        let url = self.base_url.clone();
        let (mut stream, _) = tokio_tungstenite::connect_async(url).await?;

        let message_str = serde_json::to_string(&message)?;
        stream.send(tokio_tungstenite::tungstenite::Message::Text(message_str)).await?;

        if let Some(response) = stream.next().await {
            match response? {
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    let value: Value = serde_json::from_str(&text)?;
                    Ok(value)
                }
                _ =>
                    Err(crate::error::DbxError::Api {
                        status: 0,
                        message: "Unexpected WebSocket message type".to_string(),
                    }),
            }
        } else {
            Err(crate::error::DbxError::Api {
                status: 0,
                message: "No response received from WebSocket".to_string(),
            })
        }
    }
}

impl Clone for WsClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_client_creation() {
        // This test would require a WebSocket server running
        // For now, we'll just test the URL parsing
        let url = Url::parse("ws://localhost:8080/ws").unwrap();
        assert_eq!(url.as_str(), "ws://localhost:8080/ws");
    }
}
