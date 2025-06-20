use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use std::sync::Arc;
use tracing::debug;

use crate::{
    constants::database::DatabasePatterns,
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{
        ApiResponse,
        BooleanValue,
        CompareAndSetRequest,
        DeleteResponse,
        IncrByRequest,
        IntegerValue,
        KeyValues,
        SetIfNotExistsRequest,
        SetManyRequest,
        SetRequest,
        StringValue,
        TtlResponse,
        BatchGetRequest,
        BatchDeleteRequest,
        BatchIncrRequest,
        BatchIncrByRequest,
    },
};

use dbx_crates::adapter::redis::primitives::string::RedisString;

impl RedisHandler {
    // String operation handlers

    pub async fn get_string(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().get(&key) {
            Ok(Some(value)) => Ok(Json(ApiResponse::success(StringValue { value }))),
            Ok(None) =>
                Err((StatusCode::NOT_FOUND, Json(ApiResponse::error("Key not found".to_string())))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_string(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<SetRequest>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let result = if let Some(ttl) = request.ttl {
            redis
                .string()
                .set_with_expiry(
                    &key,
                    &request.value,
                    ttl.try_into().unwrap_or(DatabasePatterns::MAX_TTL)
                )
        } else {
            redis.string().set(&key, &request.value)
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
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /strings/{}", key);

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

    pub async fn get_ttl(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}/ttl", key);

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

    pub async fn incr(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incr", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().incr(&key) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn incr_by(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<IncrByRequest>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incrby", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().incr_by(&key, request.increment) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_nx(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<SetIfNotExistsRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/setnx", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let script = RedisString::set_if_not_exists_script();
        let result: i32 = match redis.string().eval_script(&script, &[&key], &[&request.value]) {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let success = result == 1;
        if let Some(ttl) = request.ttl {
            if success {
                let _ = redis.string().expire(&key, ttl);
            }
        }

        Ok(Json(ApiResponse::success(BooleanValue { value: success })))
    }

    pub async fn compare_and_set(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<CompareAndSetRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/cas", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        let script = RedisString::compare_and_set_with_ttl_script();
        let ttl = request.ttl.unwrap_or(DatabasePatterns::DEFAULT_TTL);
        let result: i32 = match
            redis
                .string()
                .eval_script(
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

        Ok(
            Json(
                ApiResponse::success(BooleanValue {
                    value: result == 1,
                })
            )
        )
    }

    pub async fn batch_set(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<SetManyRequest>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/set");

        let mut key_values = std::collections::HashMap::new();

        let kvs: Vec<(&str, &str)> = request.key_values
            .iter()
            .map(|(k, v)| {
                key_values.insert(k.clone(), v.clone());
                (k.as_str(), v.as_str())
            })
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                println!("Failed to get redis: {}", e);
                return Err(handle_redis_error(e));
            }
        };

        let result = if let Some(ttl) = request.ttl {
            // Use set_many_with_expiry if TTL is provided
            let kvs_with_ttl: Vec<(&str, &str, usize)> = kvs
                .iter()
                .map(|(k, v)| (*k, *v, ttl as usize))
                .collect();
            println!("set_many_with_expiry: {:?}", kvs_with_ttl);
            redis.string().set_many_with_expiry(kvs_with_ttl)
        } else {
            // Otherwise, use set_many
            redis.string().set_many(kvs)
        };

        match result {
            Ok(_) => Ok(Json(ApiResponse::success(KeyValues { key_values }))),
            Err(e) => {
                println!("Failed to set many: {:?}", e);
                Err(handle_redis_error(e))
            }
        }
    }

    pub async fn batch_get(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<BatchGetRequest>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/get");

        let mut key_values = std::collections::HashMap::new();

        let key_refs: Vec<&str> = request.keys
            .iter()
            .map(|k| k.as_str())
            .collect();
        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().get_many(key_refs) {
            Ok(results) => {
                for (key, result) in request.keys.iter().zip(results.iter()) {
                    let value = result.clone().unwrap_or_else(|| "".to_string());
                    key_values.insert(key.clone(), value);
                }
                Ok(Json(ApiResponse::success(KeyValues { key_values })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_delete(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<BatchDeleteRequest>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/delete");

        let key_refs: Vec<&str> = request.keys
            .iter()
            .map(|k| k.as_str())
            .collect();
        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().del_many(key_refs) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(DeleteResponse {
                            deleted_count: request.keys.len() as u64,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_incr(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<BatchIncrRequest>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incr");

        let key_refs: Vec<&str> = request.keys
            .iter()
            .map(|k| k.as_str())
            .collect();
        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().incr_many(key_refs) {
            Ok(results) => {
                let values: Vec<IntegerValue> = results
                    .iter()
                    .map(|v| IntegerValue { value: *v })
                    .collect();
                Ok(Json(ApiResponse::success(values)))
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("not an integer") || err_str.contains("out of range") {
                    let api_err = ApiResponse::<()>::error(
                        format!("Batch increment failed: {}", err_str)
                    );
                    Err((StatusCode::BAD_REQUEST, Json(api_err)))
                } else {
                    Err(handle_redis_error(e))
                }
            }
        }
    }

    pub async fn batch_incr_by(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<BatchIncrByRequest>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incrby");

        let kv_refs: Vec<(&str, i64)> = request.key_increments
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();
        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().incr_many_by(kv_refs) {
            Ok(results) => {
                let values: Vec<IntegerValue> = results
                    .iter()
                    .map(|v| IntegerValue { value: *v })
                    .collect();
                Ok(Json(ApiResponse::success(values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
