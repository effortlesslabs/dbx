use axum::extract::ws::{ Message, WebSocket };
use futures_util::{ sink::SinkExt, stream::StreamExt };
use serde_json;
use std::sync::Arc;
use tracing::{ debug, error, info };

use crate::{ models::{ WebSocketMessage, WebSocketResponse, WebSocketState } };
use super::handler::WebSocketHandler;

/// WebSocket connection handler
pub struct WebSocketConnection;

impl WebSocketConnection {
    /// Handle the WebSocket connection and process messages
    pub async fn handle_connection(
        socket: WebSocket,
        handler: WebSocketHandler,
        connection_id: String
    ) {
        let (mut sender, mut receiver) = socket.split();
        let ws_state = WebSocketState {
            connection_id: connection_id.clone(),
            subscribed_channels: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        };

        info!("WebSocket connection {} started", connection_id);

        // Send welcome message
        let welcome = WebSocketResponse::success(
            None,
            serde_json::json!({
                "message": "Connected to DBX WebSocket API",
                "connection_id": connection_id,
                "supported_commands": [
                    "get", "set", "delete", "exists", "ttl", "incr", "incrby",
                    "setnx", "cas", "batch_get", "batch_set", "batch_delete",
                    "batch_incr", "batch_incrby", "list_keys", "ping",
                    "subscribe", "unsubscribe"
                ]
            })
        );

        if let Err(e) = sender.send(Message::Text(serde_json::to_string(&welcome).unwrap())).await {
            error!("Failed to send welcome message: {}", e);
            return;
        }

        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!("Received WebSocket message: {}", text);

                    match serde_json::from_str::<WebSocketMessage>(&text) {
                        Ok(ws_message) => {
                            let response = handler.handle_message(ws_message).await;

                            if
                                let Err(e) = sender.send(
                                    Message::Text(serde_json::to_string(&response).unwrap())
                                ).await
                            {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse WebSocket message: {}", e);
                            let error_response = WebSocketResponse::error(
                                None,
                                format!("Invalid JSON: {}", e)
                            );

                            if
                                let Err(e) = sender.send(
                                    Message::Text(serde_json::to_string(&error_response).unwrap())
                                ).await
                            {
                                error!("Failed to send error response: {}", e);
                                break;
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection {} closed by client", connection_id);
                    break;
                }
                Ok(Message::Ping(data)) => {
                    if let Err(e) = sender.send(Message::Pong(data)).await {
                        error!("Failed to send pong: {}", e);
                        break;
                    }
                }
                Ok(Message::Pong(_)) => {
                    // Ignore pong messages
                }
                Ok(Message::Binary(_)) => {
                    let error_response = WebSocketResponse::error(
                        None,
                        "Binary messages not supported".to_string()
                    );
                    if
                        let Err(e) = sender.send(
                            Message::Text(serde_json::to_string(&error_response).unwrap())
                        ).await
                    {
                        error!("Failed to send error response: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        info!("WebSocket connection {} ended", connection_id);
    }
}
