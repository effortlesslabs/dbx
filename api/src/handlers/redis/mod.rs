pub mod string;
pub mod set;
pub mod keys;
pub mod scripts;
pub mod hash;

use axum::{ extract::State, http::StatusCode, response::Json };
use std::sync::Arc;
use tracing::debug;

use crate::{
    middleware::handle_redis_error,
    models::{ ApiResponse, BooleanValue, IntegerValue, StringValue },
};

use dbx_crates::adapter::redis::client::RedisClient;

/// Redis handler that provides access to Redis operations
pub struct RedisHandler {
    pub redis: Arc<RedisClient>,
}

impl RedisHandler {
    /// Create a new Redis handler
    pub fn new(redis: RedisClient) -> Self {
        Self { redis: Arc::new(redis) }
    }

    /// Health check endpoint
    pub async fn health(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<StringValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        debug!("GET /health");

        match handler.redis.ping() {
            Ok(true) => Ok(Json(ApiResponse::success(StringValue { value: "OK".to_string() }))),
            Ok(false) =>
                Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ApiResponse::error("Redis ping failed".to_string())),
                )),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Get Redis info
    pub async fn info(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<StringValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        debug!("GET /info");

        let mut conn = handler.redis.connection().lock().unwrap();
        match redis::cmd("INFO").query::<String>(&mut *conn) {
            Ok(info) => Ok(Json(ApiResponse::success(StringValue { value: info }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Get Redis database size
    pub async fn dbsize(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<IntegerValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        debug!("GET /dbsize");

        let mut conn = handler.redis.connection().lock().unwrap();
        match redis::cmd("DBSIZE").query::<i64>(&mut *conn) {
            Ok(size) => Ok(Json(ApiResponse::success(IntegerValue { value: size }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Flush all databases
    pub async fn flushall(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<BooleanValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        debug!("POST /flushall");

        let mut conn = handler.redis.connection().lock().unwrap();
        match redis::cmd("FLUSHALL").query::<String>(&mut *conn) {
            Ok(_) => Ok(Json(ApiResponse::success(BooleanValue { value: true }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Flush current database
    pub async fn flushdb(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<BooleanValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        debug!("POST /flushdb");

        let mut conn = handler.redis.connection().lock().unwrap();
        match redis::cmd("FLUSHDB").query::<String>(&mut *conn) {
            Ok(_) => Ok(Json(ApiResponse::success(BooleanValue { value: true }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}

impl Clone for RedisHandler {
    fn clone(&self) -> Self {
        Self { redis: self.redis.clone() }
    }
}
