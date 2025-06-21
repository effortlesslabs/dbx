use crate::handlers::redis::RedisHandler;
use axum::{ extract::State, http::StatusCode, response::Json, routing::get, Router };
use serde_json::json;
use std::sync::Arc;

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
pub async fn server_info(State(redis_handler): State<Arc<RedisHandler>>) -> (
    StatusCode,
    Json<serde_json::Value>,
) {
    (
        StatusCode::OK,
        Json(
            json!({
            "service": "dbx-api",
            "version": env!("CARGO_PKG_VERSION"),
            "database_type": "redis",
            "database_url": redis_handler.config().database_url,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        ),
    )
}

/// Create common routes
pub fn create_routes() -> Router<Arc<RedisHandler>> {
    Router::new().route("/health", get(health_check)).route("/info", get(server_info))
}
