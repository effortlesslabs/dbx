use axum::{ http::StatusCode, response::{ IntoResponse, Json } };
use tracing::error;

use crate::models::ApiResponse;

/// Global error handler
pub async fn error_handler(err: axum::BoxError) -> impl IntoResponse {
    error!("Unhandled error: {}", err);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::<()>::error("Internal server error".to_string())),
    )
}

/// Convert Redis errors to HTTP responses
pub fn handle_redis_error(err: redis::RedisError) -> (StatusCode, Json<ApiResponse<()>>) {
    let message = err.to_string();

    // Simple error mapping based on error message content
    let status = if message.contains("authentication") || message.contains("auth") {
        StatusCode::UNAUTHORIZED
    } else if message.contains("invalid") || message.contains("wrong") {
        StatusCode::BAD_REQUEST
    } else if message.contains("connection") || message.contains("timeout") {
        StatusCode::SERVICE_UNAVAILABLE
    } else if message.contains("not found") || message.contains("no such") {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (status, Json(ApiResponse::error(message)))
}
