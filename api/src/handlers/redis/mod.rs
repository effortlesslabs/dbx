pub mod hash;
pub mod keys;
pub mod set;
pub mod string;

use axum::{ extract::State, http::StatusCode, response::Json };
use std::sync::Arc;
use tracing::info;

use crate::{
    config::Config,
    constants::errors::ErrorMessages,
    models::{ ApiResponse, BooleanValue, IntegerValue, StringValue },
    middleware::handle_redis_error,
};

use dbx_crates::adapter::redis::{ RedisPoolAdapter, client::RedisPool };

/// Redis handler that provides access to Redis operations
pub struct RedisHandler {
    pub pool_adapter: Arc<RedisPoolAdapter>,
    pub config: Config,
}

impl RedisHandler {
    /// Create a new Redis handler
    pub fn new(pool_adapter: RedisPoolAdapter, config: Config) -> Self {
        Self {
            pool_adapter: Arc::new(pool_adapter),
            config,
        }
    }

    /// Get a Redis instance from the connection pool
    pub fn get_redis(&self) -> Result<dbx_crates::adapter::redis::Redis, redis::RedisError> {
        self.pool_adapter.get_instance()
    }

    /// Health check endpoint
    pub async fn health(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<StringValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        info!("GET /health");

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.ping() {
            Ok(true) =>
                Ok(
                    Json(
                        ApiResponse::success(StringValue {
                            value: "OK".to_string(),
                        })
                    )
                ),
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
        info!("GET /info");

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let mut conn = match redis.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };
        match redis::cmd("INFO").query::<String>(&mut conn) {
            Ok(info) => Ok(Json(ApiResponse::success(StringValue { value: info }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Get Redis database size
    pub async fn dbsize(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<IntegerValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        info!("GET /dbsize");

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let mut conn = match redis.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };
        match redis::cmd("DBSIZE").query::<i64>(&mut conn) {
            Ok(size) => Ok(Json(ApiResponse::success(IntegerValue { value: size }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Flush all databases
    pub async fn flushall(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<BooleanValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        info!("POST /flushall");

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let mut conn = match redis.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };
        match redis::cmd("FLUSHALL").query::<String>(&mut conn) {
            Ok(_) => Ok(Json(ApiResponse::success(BooleanValue { value: true }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Flush current database
    pub async fn flushdb(State(handler): State<Arc<RedisHandler>>) -> Result<
        Json<ApiResponse<BooleanValue>>,
        (StatusCode, Json<ApiResponse<()>>)
    > {
        info!("POST /flushdb");

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let mut conn = match redis.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };
        match redis::cmd("FLUSHDB").query::<String>(&mut conn) {
            Ok(_) => Ok(Json(ApiResponse::success(BooleanValue { value: true }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Clone for RedisHandler {
    fn clone(&self) -> Self {
        Self {
            pool_adapter: self.pool_adapter.clone(),
            config: self.config.clone(),
        }
    }
}
