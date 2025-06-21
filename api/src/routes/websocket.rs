use crate::handlers::websocket::WebSocketHandler;

/// Create WebSocket routes
pub fn create_routes() -> axum::Router<WebSocketHandler> {
    axum::Router::new().route("/ws", axum::routing::get(WebSocketHandler::handle_websocket))
}
