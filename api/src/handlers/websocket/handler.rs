use axum::{ extract::{ ws::WebSocketUpgrade, State }, response::IntoResponse };
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;
use tokio::sync::Mutex;

use crate::{
    models::{ WebSocketMessage, WebSocketResponse },
    handlers::redis::RedisHandler,
    handlers::websocket::commands::WebSocketCommandProcessor,
};
use super::connection::WebSocketConnection;

/// WebSocket handler that processes JSON commands
#[derive(Clone)]
pub struct WebSocketHandler {
    pub redis_handler: Arc<Mutex<RedisHandler>>,
    command_processor: Arc<WebSocketCommandProcessor>,
}

impl WebSocketHandler {
    /// Create a new WebSocket handler
    pub fn new(redis_handler: Arc<Mutex<RedisHandler>>) -> Self {
        let command_processor = Arc::new(WebSocketCommandProcessor::new(redis_handler.clone()));
        Self {
            redis_handler,
            command_processor,
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
            WebSocketConnection::handle_connection(socket, handler, connection_id).await
        })
    }

    pub async fn handle_message(&self, message: WebSocketMessage) -> WebSocketResponse {
        self.command_processor.process_command(message).await
    }
}
