use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ ApiResponse, DeleteResponse, ExistsResponse, StringValue, TtlResponse },
};

/// Query parameters for key operations
#[derive(Debug, Deserialize)]
pub struct KeyQuery {
    pub pattern: Option<String>,
}

impl RedisHandler {
    // Key operation handlers

    pub async fn get_keys(
        State(handler): State<Arc<RedisHandler>>,
        Path(pattern): Path<String>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /keys/{}", pattern);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().keys(&pattern) {
            Ok(keys) => {
                let string_values: Vec<StringValue> = keys
                    .into_iter()
                    .map(|key| StringValue { value: key })
                    .collect();
                Ok(Json(ApiResponse::success(string_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn key_exists(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /keys/{}/exists", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().exists(&key) {
            Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_key_ttl(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /keys/{}/ttl", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().ttl(&key) {
            Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_key(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /keys/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().del(&key) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(DeleteResponse {
                            deleted_count: 1,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
