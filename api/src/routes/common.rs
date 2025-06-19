use axum::{ extract::State, http::StatusCode, response::Json, routing::get, Router };
use serde_json::json;
use crate::server::Server;

/// Health check endpoint
pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(
            json!({
            "status": "healthy",
            "service": "dbx-api",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        ),
    )
}

/// Server information endpoint
pub async fn server_info(State(server): State<Server>) -> (StatusCode, Json<serde_json::Value>) {
    let config = server.config();
    (
        StatusCode::OK,
        Json(
            json!({
            "service": "dbx-api",
            "version": env!("CARGO_PKG_VERSION"),
            "database_type": config.database_type.to_string(),
            "database_url": config.database_url,
            "host": config.host,
            "port": config.port,
            "pool_size": config.pool_size,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        ),
    )
}

/// Create common routes
pub fn create_routes() -> Router<Server> {
    Router::new().route("/health", get(health_check)).route("/info", get(server_info))
}
