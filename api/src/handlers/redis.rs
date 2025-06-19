use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

use crate::{
    middleware::handle_redis_error,
    models::{
        ApiResponse,
        BooleanValue,
        CompareAndSetRequest,
        DeleteResponse,
        ExistsResponse,
        IncrByRequest,
        IntegerValue,
        KeyValues,
        SetIfNotExistsRequest,
        SetManyRequest,
        SetRequest,
        StringValue,
        TtlResponse,
    },
};

use dbx_crates::adapter::redis::{ Redis, primitives::string::RedisString };

/// Query parameters for key operations
#[derive(Debug, Deserialize)]
pub struct KeyQuery {
    pub pattern: Option<String>,
}

/// Redis handler that holds the Redis client
#[derive(Clone)]
pub struct RedisHandler {
    pub redis: Arc<Redis>,
}

impl RedisHandler {
    /// Create a new Redis handler
    pub fn new(redis: Arc<Redis>) -> Self {
        Self { redis }
    }

    // String operation handlers

    pub async fn get_string(
        State(handler): State<Self>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}", key);

        match handler.redis.string().get(&key) {
            Ok(Some(value)) => Ok(Json(ApiResponse::success(StringValue { value }))),
            Ok(None) =>
                Err((StatusCode::NOT_FOUND, Json(ApiResponse::error("Key not found".to_string())))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_string(
        State(handler): State<Self>,
        Path(key): Path<String>,
        Json(request): Json<SetRequest>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}", key);

        let result = if let Some(ttl) = request.ttl {
            handler.redis
                .string()
                .set_with_expiry(&key, &request.value, ttl.try_into().unwrap_or(usize::MAX))
        } else {
            handler.redis.string().set(&key, &request.value)
        };

        match result {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(StringValue {
                            value: request.value,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_string(
        State(handler): State<Self>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /strings/{}", key);

        match handler.redis.string().del(&key) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn exists(
        State(handler): State<Self>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}/exists", key);

        match handler.redis.string().exists(&key) {
            Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_ttl(
        State(handler): State<Self>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}/ttl", key);

        match handler.redis.string().ttl(&key) {
            Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn incr(
        State(handler): State<Self>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incr", key);

        match handler.redis.string().incr(&key) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn incr_by(
        State(handler): State<Self>,
        Path(key): Path<String>,
        Json(request): Json<IncrByRequest>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incrby", key);

        match handler.redis.string().incr_by(&key, request.increment) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_nx(
        State(handler): State<Self>,
        Path(key): Path<String>,
        Json(request): Json<SetIfNotExistsRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/setnx", key);

        let script = RedisString::set_if_not_exists_script();
        let result: i32 = match handler.redis.eval_script(&script, &[&key], &[&request.value]) {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let success = result == 1;
        if let Some(ttl) = request.ttl {
            if success {
                let _ = handler.redis.string().expire(&key, ttl);
            }
        }

        Ok(Json(ApiResponse::success(BooleanValue { value: success })))
    }

    pub async fn compare_and_set(
        State(handler): State<Self>,
        Path(key): Path<String>,
        Json(request): Json<CompareAndSetRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/cas", key);

        let script = RedisString::compare_and_set_with_ttl_script();
        let ttl = request.ttl.unwrap_or(0);
        let result: i32 = match
            handler.redis.eval_script(
                &script,
                &[&key],
                &[&request.expected_value, &request.new_value, &ttl.to_string()]
            )
        {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let success = result == 1;
        Ok(Json(ApiResponse::success(BooleanValue { value: success })))
    }

    pub async fn batch_set(
        State(handler): State<Self>,
        Json(request): Json<SetManyRequest>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/set");

        let mut results = HashMap::new();
        for (key, value) in request.key_values {
            match handler.redis.string().set(&key, &value) {
                Ok(_) => {
                    results.insert(key, value);
                }
                Err(e) => {
                    return Err(handle_redis_error(e));
                }
            }
        }

        Ok(Json(ApiResponse::success(KeyValues { key_values: results })))
    }

    pub async fn batch_get(
        State(handler): State<Self>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/get");

        let mut results = HashMap::new();
        for key in keys {
            match handler.redis.string().get(&key) {
                Ok(Some(value)) => {
                    results.insert(key, value);
                }
                Ok(None) => {
                    // Skip keys that don't exist
                }
                Err(e) => {
                    return Err(handle_redis_error(e));
                }
            }
        }

        Ok(Json(ApiResponse::success(KeyValues { key_values: results })))
    }

    pub async fn batch_delete(
        State(handler): State<Self>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/delete");

        let mut deleted_count = 0;
        for key in keys {
            match handler.redis.string().del(&key) {
                Ok(_) => {
                    deleted_count += 1;
                }
                Err(e) => {
                    return Err(handle_redis_error(e));
                }
            }
        }

        Ok(Json(ApiResponse::success(DeleteResponse { deleted_count })))
    }

    pub async fn batch_incr(
        State(handler): State<Self>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incr");

        let mut results = Vec::new();
        for key in keys {
            match handler.redis.string().incr(&key) {
                Ok(value) => {
                    results.push(IntegerValue { value });
                }
                Err(e) => {
                    return Err(handle_redis_error(e));
                }
            }
        }

        Ok(Json(ApiResponse::success(results)))
    }

    pub async fn batch_incr_by(
        State(handler): State<Self>,
        Json(kvs): Json<Vec<(String, i64)>>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incrby");

        let mut results = Vec::new();
        for (key, increment) in kvs {
            match handler.redis.string().incr_by(&key, increment) {
                Ok(value) => {
                    results.push(IntegerValue { value });
                }
                Err(e) => {
                    return Err(handle_redis_error(e));
                }
            }
        }

        Ok(Json(ApiResponse::success(results)))
    }

    // Script operation handlers

    pub async fn rate_limiter_script(
        State(handler): State<Self>,
        Json(request): Json<RateLimiterRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /scripts/rate-limiter");

        let script = RedisString::rate_limiter_script();
        let result: i32 = match
            handler.redis.eval_script(
                &script,
                &[&request.key],
                &[&request.limit.to_string(), &request.window.to_string()]
            )
        {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        Ok(Json(ApiResponse::success(BooleanValue { value: result == 1 })))
    }

    pub async fn multi_counter_script(
        State(handler): State<Self>,
        Json(request): Json<MultiCounterRequest>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /scripts/multi-counter");

        let script = RedisString::multi_counter_script();
        let keys: Vec<String> = request.counters
            .iter()
            .map(|(k, _)| k.clone())
            .collect();
        let values: Vec<String> = request.counters
            .iter()
            .map(|(_, v)| v.to_string())
            .collect();

        let result: Vec<i64> = match handler.redis.eval_script(&script, &keys, &values) {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let results = result
            .into_iter()
            .map(|v| IntegerValue { value: v })
            .collect();
        Ok(Json(ApiResponse::success(results)))
    }

    pub async fn multi_set_ttl_script(
        State(handler): State<Self>,
        Json(request): Json<MultiSetTtlRequest>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /scripts/multi-set-ttl");

        let script = RedisString::multi_set_with_ttl_script();
        let mut keys_and_values = Vec::new();
        for (key, value) in &request.key_values {
            keys_and_values.push(key.clone());
            keys_and_values.push(value.clone());
        }

        let _: i32 = match
            handler.redis.eval_script(&script, &keys_and_values, &[&request.ttl.to_string()])
        {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        Ok(Json(ApiResponse::success(KeyValues { key_values: request.key_values })))
    }
}

// Request models for script operations

#[derive(Debug, Deserialize)]
pub struct RateLimiterRequest {
    pub key: String,
    pub limit: i64,
    pub window: i64,
}

#[derive(Debug, Deserialize)]
pub struct MultiCounterRequest {
    pub counters: Vec<(String, i64)>,
}

#[derive(Debug, Deserialize)]
pub struct MultiSetTtlRequest {
    pub key_values: HashMap<String, String>,
    pub ttl: u64,
}
