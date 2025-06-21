use axum::{ http::StatusCode, response::Json };

use crate::{ constants::errors::ErrorMessages, models::ApiResponse };

/// Handle Redis errors and convert them to HTTP responses
pub fn handle_redis_error(_error: impl std::fmt::Display) -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::<()>::error(ErrorMessages::INTERNAL_SERVER_ERROR.to_string())),
    )
}
