use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use serde::Deserialize;
use tracing::debug;
use std::sync::Arc;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ ApiResponse, DeleteResponse, ExistsResponse, KeysResponse, TtlResponse },
};

/// Query parameters for key operations
#[derive(Debug, Deserialize)]
pub struct KeyQuery {
    pub pattern: Option<String>,
}

impl RedisHandler {
    // Key operation handlers

    pub async fn list_keys(
        State(handler): State<Arc<RedisHandler>>,
        axum::extract::Query(params): axum::extract::Query<
            std::collections::HashMap<String, String>
        >
    ) -> Result<Json<ApiResponse<Vec<String>>>, (StatusCode, Json<ApiResponse<()>>)> {
        let pattern = params
            .get("pattern")
            .cloned()
            .unwrap_or_else(|| "*".to_string());
        debug!("GET /keys?pattern={}", pattern);

        match handler.redis.string().keys(&pattern) {
            Ok(keys) => Ok(Json(ApiResponse::success(keys))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn key_exists(
        State(handler): State<Arc<RedisHandler>>,
        axum::extract::Path(key): axum::extract::Path<String>
    ) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /keys/{}/exists", key);

        match handler.redis.string().exists(&key) {
            Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn key_ttl(
        State(handler): State<Arc<RedisHandler>>,
        axum::extract::Path(key): axum::extract::Path<String>
    ) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /keys/{}/ttl", key);

        match handler.redis.string().ttl(&key) {
            Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_key(
        State(handler): State<Arc<RedisHandler>>,
        axum::extract::Path(key): axum::extract::Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /keys/{}", key);

        match handler.redis.string().del(&key) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
