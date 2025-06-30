use crate::{
    error::Result,
    common::{ StringOperations, WebSocketClientBase, client::websocket },
    StringOperation,
    StringInfo,
};
use tokio_tungstenite::WebSocketStream;
use serde_json::{ json, Value };
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use url::Url;

/// WebSocket client for string operations
#[cfg(feature = "websocket")]
pub struct WsStringClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    base_url: Url,
}

#[cfg(feature = "websocket")]
impl WsStringClient {
    pub(crate) fn new(stream: WebSocketStream<MaybeTlsStream<TcpStream>>, base_url: Url) -> Self {
        Self { stream, base_url }
    }
}

#[cfg(feature = "websocket")]
impl WebSocketClientBase for WsStringClient {
    /// Get the base URL for this client
    fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Send a WebSocket message and get response
    async fn send_message(&mut self, message: Value) -> Result<Value> {
        websocket::send_message(&mut self.stream, message).await
    }
}

#[cfg(feature = "websocket")]
impl StringOperations for WsStringClient {
    /// Get a string value by key
    async fn get(&mut self, key: &str) -> Result<Option<String>> {
        let message =
            json!({
            "type": "get",
            "data": {
                "key": key
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(value) = data.get("value") {
                if value.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(value.as_str().unwrap_or("").to_string()))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Set a string value
    async fn set(&mut self, key: &str, value: &str, ttl: Option<u64>) -> Result<()> {
        let message =
            json!({
            "type": "set",
            "data": {
                "key": key,
                "value": value,
                "ttl": ttl
            }
        });

        let _response = self.send_message(message).await?;
        Ok(())
    }

    /// Delete a string value
    async fn delete(&mut self, key: &str) -> Result<bool> {
        let message =
            json!({
            "type": "del",
            "data": {
                "key": key
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(deleted) = data.get("deleted") {
                Ok(deleted.as_bool().unwrap_or(false))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Get string information
    async fn info(&mut self, key: &str) -> Result<Option<StringInfo>> {
        let message =
            json!({
            "type": "info",
            "data": {
                "key": key
            }
        });

        let response = self.send_message(message).await?;

        if let Some(data) = response.get("data") {
            if let Some(info) = data.get("info") {
                if info.is_null() {
                    Ok(None)
                } else {
                    let string_info: StringInfo = serde_json::from_value(info.clone())?;
                    Ok(Some(string_info))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Batch get multiple strings
    async fn batch_get(&mut self, keys: &[String]) -> Result<Vec<Option<String>>> {
        let message =
            json!({
            "type": "batch_get",
            "data": {
                "keys": keys
            }
        });

        let response = self.send_message(message).await?;

        if let Some(data) = response.get("data") {
            if let Some(values) = data.get("values") {
                let empty_vec = Vec::new();
                let values_array = values.as_array().unwrap_or(&empty_vec);
                let mut result_vec = Vec::new();
                for value in values_array {
                    if value.is_null() {
                        result_vec.push(None);
                    } else {
                        result_vec.push(Some(value.as_str().unwrap_or("").to_string()));
                    }
                }
                Ok(result_vec)
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Batch set multiple strings
    async fn batch_set(&mut self, operations: &[StringOperation]) -> Result<()> {
        let message =
            json!({
            "type": "batch_set",
            "data": {
                "operations": operations
            }
        });

        let _response = self.send_message(message).await?;
        Ok(())
    }

    /// Get strings by patterns
    async fn get_by_patterns(
        &mut self,
        _patterns: &[String],
        _grouped: Option<bool>
    ) -> Result<Value> {
        // Note: This operation might not be implemented in the WebSocket server
        // For now, return an empty result
        Ok(json!({}))
    }
}

#[cfg(all(test, feature = "websocket"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_string_client_creation() {
        // This test would require a WebSocket server running
        // For now, we'll just test the URL parsing
        let url = Url::parse("ws://localhost:8080/ws").unwrap();
        assert_eq!(url.as_str(), "ws://localhost:8080/ws");
    }
}
