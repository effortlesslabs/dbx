use axum::{ routing::get, Router };
use crate::handlers::websocket::WebSocketHandler;

/// Create WebSocket routes
pub fn create_routes() -> Router<WebSocketHandler> {
    Router::new().route("/ws", get(WebSocketHandler::handle_websocket))
}
