use axum::extract::ws::{ Message, WebSocket };
use serde_json;
use tracing::{ error, info, warn };

use super::handler::RedisWsHandler;
use crate::{
    constants::websocket::{
        WELCOME_MESSAGE,
        CONNECTION_ID_FIELD,
        SUPPORTED_COMMANDS_FIELD,
        SUPPORTED_COMMANDS,
        ERROR_BINARY_MESSAGES,
    },
    models::{ RedisWsMessage, RedisWsResponse },
};

/// RedisWs connection handler
pub struct RedisWsConnection;

impl RedisWsConnection {
    /// Handle WebSocket connection
    pub async fn handle_connection(
        mut socket: WebSocket,
        handler: RedisWsHandler,
        connection_id: String
    ) {
        info!("RedisWs connection started: {}", connection_id);

        // Send welcome message
        let welcome = RedisWsResponse::Success {
            id: None,
            data: serde_json::json!({
                "message": WELCOME_MESSAGE,
                CONNECTION_ID_FIELD: connection_id,
                SUPPORTED_COMMANDS_FIELD: SUPPORTED_COMMANDS
            }),
        };

        if let Err(e) = socket.send(Message::Text(serde_json::to_string(&welcome).unwrap())).await {
            error!("Failed to send welcome message: {}", e);
            return;
        }

        while let Some(msg) = socket.recv().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<RedisWsMessage>(&text) {
                        Ok(message) => {
                            let response = handler.handle_message(message).await;
                            if
                                let Err(e) = socket.send(
                                    Message::Text(serde_json::to_string(&response).unwrap())
                                ).await
                            {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("Invalid message format: {}", e);
                            let error_response = RedisWsResponse::error(
                                None,
                                format!("Invalid message format: {}", e)
                            );
                            if
                                let Err(e) = socket.send(
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
                    info!("RedisWs connection closed: {}", connection_id);
                    break;
                }
                Ok(Message::Ping(data)) => {
                    if let Err(e) = socket.send(Message::Pong(data)).await {
                        error!("Failed to send pong: {}", e);
                        break;
                    }
                }
                Ok(Message::Pong(_)) => {
                    // Ignore pong messages
                }
                Ok(Message::Binary(_)) => {
                    warn!("Binary messages not supported");
                    let error_response = RedisWsResponse::error(
                        None,
                        ERROR_BINARY_MESSAGES.to_string()
                    );
                    if
                        let Err(e) = socket.send(
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

        info!("RedisWs connection ended: {}", connection_id);
    }
}
