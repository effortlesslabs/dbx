use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use crate::{
    constants::{database::DatabasePatterns, errors::ErrorMessages},
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{
        ApiResponse, BooleanValue, CompareAndSetRequest, DeleteResponse, ExistsResponse,
        IncrByRequest, IntegerValue, KeyValues, SetIfNotExistsRequest, SetManyRequest, SetRequest,
        StringValue, TtlResponse,
    },
};

use dbx_crates::adapter::redis::primitives::string::RedisString;

impl RedisHandler {
    // String operation handlers

    pub async fn get_string(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}", key);

        match handler.redis.string().get(&key) {
            Ok(Some(value)) => Ok(Json(ApiResponse::success(StringValue { value }))),
            Ok(None) => Ok(Json(ApiResponse::success(StringValue {
                value: String::new(),
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_string(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<SetRequest>,
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}", key);

        let result = if let Some(ttl) = request.ttl {
            handler.redis.string().set_with_expiry(
                &key,
                &request.value,
                ttl.try_into().unwrap_or(DatabasePatterns::MAX_TTL),
            )
        } else {
            handler.redis.string().set(&key, &request.value)
        };

        match result {
            Ok(_) => Ok(Json(ApiResponse::success(StringValue {
                value: request.value,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_string(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /strings/{}", key);

        match handler.redis.string().del(&key) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: 1,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn exists(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}/exists", key);

        match handler.redis.string().exists(&key) {
            Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_ttl(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /strings/{}/ttl", key);

        match handler.redis.string().ttl(&key) {
            Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn incr(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incr", key);

        match handler.redis.string().incr(&key) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn incr_by(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<IncrByRequest>,
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/incrby", key);

        match handler.redis.string().incr_by(&key, request.increment) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_nx(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<SetIfNotExistsRequest>,
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/setnx", key);

        let script = RedisString::set_if_not_exists_script();
        let result: i32 =
            match handler
                .redis
                .string()
                .eval_script(&script, &[&key], &[&request.value])
            {
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
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<CompareAndSetRequest>,
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/{}/cas", key);

        let script = RedisString::compare_and_set_with_ttl_script();
        let ttl = request.ttl.unwrap_or(DatabasePatterns::DEFAULT_TTL);
        let result: i32 = match handler.redis.string().eval_script(
            &script,
            &[&key],
            &[
                &request.expected_value,
                &request.new_value,
                &ttl.to_string(),
            ],
        ) {
            Ok(result) => result,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        Ok(Json(ApiResponse::success(BooleanValue {
            value: result == 1,
        })))
    }

    pub async fn batch_set(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<SetManyRequest>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/set");

        let mut key_values = std::collections::HashMap::new();

        // Use the batch operations helper
        let kvs: Vec<(&str, &str)> = request
            .key_values
            .iter()
            .map(|(k, v)| {
                key_values.insert(k.clone(), v.clone());
                (k.as_str(), v.as_str())
            })
            .collect();

        match handler.redis.string().set_many(kvs) {
            Ok(_) => Ok(Json(ApiResponse::success(KeyValues { key_values }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_get(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/get");

        let mut key_values = std::collections::HashMap::new();

        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.string().get_many(key_refs) {
            Ok(results) => {
                for (key, result) in keys.iter().zip(results.iter()) {
                    if let Some(value) = result {
                        key_values.insert(key.clone(), value.clone());
                    }
                }
                Ok(Json(ApiResponse::success(KeyValues { key_values })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_delete(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/delete");

        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.string().del_many(key_refs) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: keys.len() as u64,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_incr(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incr");

        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.string().incr_many(key_refs) {
            Ok(results) => {
                let values: Vec<IntegerValue> =
                    results.iter().map(|v| IntegerValue { value: *v }).collect();
                Ok(Json(ApiResponse::success(values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_incr_by(
        State(handler): State<Arc<RedisHandler>>,
        Json(kvs): Json<Vec<(String, i64)>>,
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /strings/batch/incrby");

        let kv_refs: Vec<(&str, i64)> = kvs.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        match handler.redis.string().incr_many_by(kv_refs) {
            Ok(results) => {
                let values: Vec<IntegerValue> =
                    results.iter().map(|v| IntegerValue { value: *v }).collect();
                Ok(Json(ApiResponse::success(values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
