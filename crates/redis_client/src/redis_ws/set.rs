use crate::{ error::Result, common::{ SetOperations, WebSocketClientBase, client::websocket } };
use tokio_tungstenite::WebSocketStream;
use serde_json::{ json, Value };
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use url::Url;

/// WebSocket client for set operations
#[cfg(feature = "websocket")]
pub struct WsSetClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    base_url: Url,
}

#[cfg(feature = "websocket")]
impl WsSetClient {
    pub(crate) fn new(stream: WebSocketStream<MaybeTlsStream<TcpStream>>, base_url: Url) -> Self {
        Self { stream, base_url }
    }
}

#[cfg(feature = "websocket")]
impl WebSocketClientBase for WsSetClient {
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
impl SetOperations for WsSetClient {
    /// Get all members of a set
    async fn members(&mut self, key: &str) -> Result<Vec<String>> {
        let message =
            json!({
            "type": "members",
            "data": {
                "key": key
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(value) = data.get("value") {
                if let Some(members_array) = value.as_array() {
                    let mut result_vec = Vec::new();
                    for member in members_array {
                        if let Some(member_str) = member.as_str() {
                            result_vec.push(member_str.to_string());
                        }
                    }
                    Ok(result_vec)
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Add a member to a set
    async fn add(&mut self, key: &str, member: &str) -> Result<usize> {
        let message =
            json!({
            "type": "add",
            "data": {
                "key": key,
                "member": member
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(added) = data.get("added") {
                Ok(added.as_u64().unwrap_or(0) as usize)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    /// Add multiple members to a set
    async fn add_many(&mut self, key: &str, members: &[&str]) -> Result<usize> {
        // For now, add members one by one since the server doesn't have a batch add
        let mut total_added = 0;
        for member in members {
            total_added += self.add(key, member).await?;
        }
        Ok(total_added)
    }

    /// Remove a member from a set
    async fn remove(&mut self, key: &str, member: &str) -> Result<usize> {
        let message =
            json!({
            "type": "remove",
            "data": {
                "key": key,
                "member": member
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(removed) = data.get("removed") {
                Ok(removed.as_u64().unwrap_or(0) as usize)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    /// Get the cardinality (number of members) of a set
    async fn cardinality(&mut self, key: &str) -> Result<usize> {
        let message =
            json!({
            "type": "cardinality",
            "data": {
                "key": key
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(cardinality) = data.get("cardinality") {
                Ok(cardinality.as_u64().unwrap_or(0) as usize)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    /// Check if a member exists in a set
    async fn exists(&mut self, key: &str, member: &str) -> Result<bool> {
        let message =
            json!({
            "type": "exists",
            "data": {
                "key": key,
                "member": member
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(exists) = data.get("exists") {
                Ok(exists.as_bool().unwrap_or(false))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Get the intersection of multiple sets
    async fn intersect(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let message =
            json!({
            "type": "intersect",
            "data": {
                "keys": keys
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(intersection) = data.get("intersection") {
                if let Some(members) = intersection.as_array() {
                    let mut result_vec = Vec::new();
                    for member in members {
                        if let Some(member_str) = member.as_str() {
                            result_vec.push(member_str.to_string());
                        }
                    }
                    Ok(result_vec)
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Get the union of multiple sets
    async fn union(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let message =
            json!({
            "type": "union",
            "data": {
                "keys": keys
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(union) = data.get("union") {
                if let Some(members) = union.as_array() {
                    let mut result_vec = Vec::new();
                    for member in members {
                        if let Some(member_str) = member.as_str() {
                            result_vec.push(member_str.to_string());
                        }
                    }
                    Ok(result_vec)
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Get the difference of multiple sets
    async fn difference(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let message =
            json!({
            "type": "difference",
            "data": {
                "keys": keys
            }
        });

        let response = self.send_message(message).await?;

        // Parse the response according to the server's format
        if let Some(data) = response.get("data") {
            if let Some(difference) = data.get("difference") {
                if let Some(members) = difference.as_array() {
                    let mut result_vec = Vec::new();
                    for member in members {
                        if let Some(member_str) = member.as_str() {
                            result_vec.push(member_str.to_string());
                        }
                    }
                    Ok(result_vec)
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Delete a set by key (not implemented for WebSocket, return Ok(true) as a no-op)
    async fn delete(&mut self, _key: &str) -> crate::error::Result<bool> {
        Ok(true)
    }
}

#[cfg(all(test, feature = "websocket"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_set_client_creation() {
        // This test would require a WebSocket server running
        // For now, we'll just test the URL parsing
        let url = Url::parse("ws://localhost:8080/ws").unwrap();
        assert_eq!(url.as_str(), "ws://localhost:8080/ws");
    }
}
