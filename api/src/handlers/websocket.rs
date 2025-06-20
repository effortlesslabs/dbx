use axum::{
    extract::{ ws::{ Message, WebSocket, WebSocketUpgrade }, State },
    response::IntoResponse,
};
use futures_util::{ sink::SinkExt, stream::StreamExt };
use serde_json;
use std::sync::Arc;
use tracing::{ debug, error, info };
use uuid::Uuid;

use crate::{
    handlers::redis::RedisHandler,
    models::{ WebSocketCommand, WebSocketMessage, WebSocketResponse, WebSocketState },
    middleware::handle_redis_error,
};

/// WebSocket handler that processes JSON commands
#[derive(Clone)]
pub struct WebSocketHandler {
    pub redis_handler: Arc<RedisHandler>,
}

impl WebSocketHandler {
    /// Create a new WebSocket handler
    pub fn new(redis_handler: RedisHandler) -> Self {
        Self {
            redis_handler: Arc::new(redis_handler),
        }
    }

    /// Handle WebSocket upgrade and connection
    pub async fn handle_websocket(
        ws: WebSocketUpgrade,
        State(handler): State<Self>
    ) -> impl IntoResponse {
        let connection_id = Uuid::new_v4().to_string();
        info!("WebSocket connection established: {}", connection_id);

        ws.on_upgrade(|socket| async move {
            Self::handle_websocket_connection(socket, handler, connection_id).await
        })
    }

    /// Handle the WebSocket connection and process messages
    async fn handle_websocket_connection(
        socket: WebSocket,
        handler: WebSocketHandler,
        connection_id: String
    ) {
        let (mut sender, mut receiver) = socket.split();
        let _ws_state = WebSocketState {
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
                            let response = Self::process_command(
                                &handler,
                                &_ws_state,
                                ws_message
                            ).await;

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

    /// Process a WebSocket command and return a response
    async fn process_command(
        handler: &WebSocketHandler,
        _ws_state: &WebSocketState,
        ws_message: WebSocketMessage
    ) -> WebSocketResponse {
        let command_id = ws_message.id.clone();

        match ws_message.command {
            WebSocketCommand::Get { key } => {
                match handler.redis_handler.redis.string().get(&key) {
                    Ok(Some(value)) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "value": value })
                        )
                    }
                    Ok(None) => {
                        WebSocketResponse::error(command_id, "Key not found".to_string())
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::Set { key, value, ttl } => {
                let result = if let Some(ttl) = ttl {
                    handler.redis_handler.redis
                        .string()
                        .set_with_expiry(&key, &value, ttl.try_into().unwrap_or(usize::MAX))
                } else {
                    handler.redis_handler.redis.string().set(&key, &value)
                };

                match result {
                    Ok(_) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "value": value })
                        )
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::Delete { key } => {
                match handler.redis_handler.redis.string().del(&key) {
                    Ok(_) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "deleted_count": 1 })
                        )
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::Exists { key } => {
                match handler.redis_handler.redis.string().exists(&key) {
                    Ok(exists) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "exists": exists })
                        )
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::Ttl { key } => {
                match handler.redis_handler.redis.string().ttl(&key) {
                    Ok(ttl) => {
                        WebSocketResponse::success(command_id, serde_json::json!({ "ttl": ttl }))
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::Incr { key } => {
                match handler.redis_handler.redis.string().incr(&key) {
                    Ok(value) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "value": value })
                        )
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::IncrBy { key, increment } => {
                match handler.redis_handler.redis.string().incr_by(&key, increment) {
                    Ok(value) => {
                        WebSocketResponse::success(
                            command_id,
                            serde_json::json!({ "value": value })
                        )
                    }
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        )
                    }
                }
            }

            WebSocketCommand::SetNx { key, value, ttl } => {
                let script =
                    dbx_crates::adapter::redis::primitives::string::RedisString::set_if_not_exists_script();
                let result: i32 = match
                    handler.redis_handler.redis.string().eval_script(&script, &[&key], &[&value])
                {
                    Ok(result) => result,
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        return WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        );
                    }
                };

                let success = result == 1;
                if let Some(ttl) = ttl {
                    if success {
                        let _ = handler.redis_handler.redis.string().expire(&key, ttl);
                    }
                }

                WebSocketResponse::success(command_id, serde_json::json!({ "success": success }))
            }

            WebSocketCommand::CompareAndSet { key, expected_value, new_value, ttl } => {
                let script =
                    dbx_crates::adapter::redis::primitives::string::RedisString::compare_and_set_with_ttl_script();
                let ttl = ttl.unwrap_or(0);
                let result: i32 = match
                    handler.redis_handler.redis
                        .string()
                        .eval_script(
                            &script,
                            &[&key],
                            &[&expected_value, &new_value, &ttl.to_string()]
                        )
                {
                    Ok(result) => result,
                    Err(e) => {
                        let (_, error_response) = handle_redis_error(e);
                        return WebSocketResponse::error(
                            command_id,
                            error_response.error.clone().unwrap_or_default()
                        );
                    }
                };

                let success = result == 1;
                WebSocketResponse::success(command_id, serde_json::json!({ "success": success }))
            }

            WebSocketCommand::BatchGet { keys } => {
                let mut results = std::collections::HashMap::new();
                for key in keys {
                    if let Ok(Some(value)) = handler.redis_handler.redis.string().get(&key) {
                        results.insert(key, value);
                    }
                }
                WebSocketResponse::success(command_id, serde_json::json!({ "key_values": results }))
            }

            WebSocketCommand::BatchSet { key_values, ttl } => {
                let mut results = std::collections::HashMap::new();
                for (key, value) in key_values {
                    let result = if let Some(ttl) = ttl {
                        handler.redis_handler.redis
                            .string()
                            .set_with_expiry(&key, &value, ttl.try_into().unwrap_or(usize::MAX))
                    } else {
                        handler.redis_handler.redis.string().set(&key, &value)
                    };

                    if result.is_ok() {
                        results.insert(key, value);
                    }
                }
                WebSocketResponse::success(command_id, serde_json::json!({ "key_values": results }))
            }

            WebSocketCommand::BatchDelete { keys } => {
                let mut deleted_count = 0;
                for key in keys {
                    if handler.redis_handler.redis.string().del(&key).is_ok() {
                        deleted_count += 1;
                    }
                }
                WebSocketResponse::success(
                    command_id,
                    serde_json::json!({ "deleted_count": deleted_count })
                )
            }

            WebSocketCommand::BatchIncr { keys } => {
                let mut results = Vec::new();
                for key in keys {
                    if let Ok(value) = handler.redis_handler.redis.string().incr(&key) {
                        results.push(serde_json::json!({ "key": key, "value": value }));
                    }
                }
                WebSocketResponse::success(command_id, serde_json::json!({ "results": results }))
            }

            WebSocketCommand::BatchIncrBy { key_increments } => {
                let mut results = Vec::new();
                for (key, increment) in key_increments {
                    if
                        let Ok(value) = handler.redis_handler.redis
                            .string()
                            .incr_by(&key, increment)
                    {
                        results.push(serde_json::json!({ "key": key, "value": value }));
                    }
                }
                WebSocketResponse::success(command_id, serde_json::json!({ "results": results }))
            }

            WebSocketCommand::ListKeys { pattern: _ } => {
                // This would need to be implemented in the Redis adapter
                WebSocketResponse::error(command_id, "List keys not yet implemented".to_string())
            }

            WebSocketCommand::Ping => {
                WebSocketResponse::success(command_id, serde_json::json!({ "pong": true }))
            }

            WebSocketCommand::Subscribe { channels: _ } => {
                // This would need Redis PubSub implementation
                WebSocketResponse::error(command_id, "Subscribe not yet implemented".to_string())
            }

            WebSocketCommand::Unsubscribe { channels: _ } => {
                // This would need Redis PubSub implementation
                WebSocketResponse::error(command_id, "Unsubscribe not yet implemented".to_string())
            }
        }
    }
}
