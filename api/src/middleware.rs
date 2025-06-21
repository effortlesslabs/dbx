use axum::{
    extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, response::Json,
};

use crate::{constants::errors::ErrorMessages, models::ApiResponse};

/// Handle Redis errors and convert them to HTTP responses
pub fn handle_redis_error(_error: impl std::fmt::Display) -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::<()>::error(
            ErrorMessages::INTERNAL_SERVER_ERROR.to_string(),
        )),
    )
}

/// Custom error handler for JSON extraction errors
pub async fn handle_json_rejection(rejection: JsonRejection) -> impl IntoResponse {
    let (status, error_message) = match rejection {
        JsonRejection::JsonDataError(_) => (StatusCode::BAD_REQUEST, "Invalid JSON data"),
        JsonRejection::JsonSyntaxError(_) => (StatusCode::BAD_REQUEST, "Invalid JSON syntax"),
        JsonRejection::MissingJsonContentType(_) => (
            StatusCode::BAD_REQUEST,
            "Missing Content-Type: application/json header",
        ),
        JsonRejection::BytesRejection(_) => {
            (StatusCode::BAD_REQUEST, "Failed to read request body")
        }
        _ => (StatusCode::BAD_REQUEST, "Invalid request body"),
    };

    (
        status,
        Json(ApiResponse::<()>::error(error_message.to_string())),
    )
}
