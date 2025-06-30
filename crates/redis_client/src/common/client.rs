use crate::error::{DbxError, Result};
use serde_json;
use url::Url;

/// Common trait for HTTP clients
pub trait HttpClientBase {
    /// Get the base URL
    fn base_url(&self) -> &Url;
}

/// Common trait for WebSocket clients
#[cfg(feature = "websocket")]
pub trait WebSocketClientBase {
    /// Get the base URL
    fn base_url(&self) -> &Url;

    /// Send a WebSocket message and get response
    async fn send_message(&mut self, message: serde_json::Value) -> Result<serde_json::Value>;
}

/// Common HTTP response handling utilities
pub mod http {
    use super::*;
    use reqwest::Response;

    /// Handle HTTP response and extract JSON data
    pub async fn handle_response<T>(response: Response, operation: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if response.status().is_success() {
            let data: T = response.json().await?;
            Ok(data)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to {}: HTTP {}", operation, response.status()),
            })
        }
    }

    /// Handle HTTP response for operations that don't return data
    pub async fn handle_empty_response(response: Response, operation: &str) -> Result<()> {
        if response.status().is_success() {
            Ok(())
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to {}: HTTP {}", operation, response.status()),
            })
        }
    }
}

/// Common WebSocket message handling utilities
#[cfg(feature = "websocket")]
pub mod websocket {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpStream;
    use tokio_tungstenite::MaybeTlsStream;
    use tokio_tungstenite::WebSocketStream;

    /// Send a WebSocket message and get response
    pub async fn send_message(
        stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
        message: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let message_str = serde_json::to_string(&message)?;
        stream
            .send(tokio_tungstenite::tungstenite::Message::Text(message_str))
            .await?;

        if let Some(response) = stream.next().await {
            match response? {
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    let value: serde_json::Value = serde_json::from_str(&text)?;
                    Ok(value)
                }
                _ => Err(DbxError::Api {
                    status: 0,
                    message: "Unexpected WebSocket message type".to_string(),
                }),
            }
        } else {
            Err(DbxError::Api {
                status: 0,
                message: "No response received from WebSocket".to_string(),
            })
        }
    }

    /// Extract string value from WebSocket response
    pub fn extract_string_value(response: &serde_json::Value, field: &str) -> Option<String> {
        response
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Extract boolean value from WebSocket response
    pub fn extract_bool_value(response: &serde_json::Value, field: &str) -> bool {
        response
            .get(field)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Extract usize value from WebSocket response
    pub fn extract_usize_value(response: &serde_json::Value, field: &str) -> usize {
        response.get(field).and_then(|v| v.as_u64()).unwrap_or(0) as usize
    }

    /// Extract string array from WebSocket response
    pub fn extract_string_array(response: &serde_json::Value, field: &str) -> Vec<String> {
        response
            .get(field)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }
}
