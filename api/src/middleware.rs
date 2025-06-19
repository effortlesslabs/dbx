use axum::{ http::StatusCode, response::{ IntoResponse, Json } };
use tracing::error;
use crate::{ constants::errors::ErrorMessages, models::ApiResponse };

/// Global error handler
pub async fn error_handler(err: axum::BoxError) -> impl IntoResponse {
    error!("Unhandled error: {}", err);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::<()>::error(ErrorMessages::INTERNAL_SERVER_ERROR.to_string())),
    )
}

/// Handle Redis errors and convert them to appropriate HTTP responses
pub fn handle_redis_error(err: impl std::fmt::Display) -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::<()>::error(ErrorMessages::INTERNAL_SERVER_ERROR.to_string())),
    )
}
