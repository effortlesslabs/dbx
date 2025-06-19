use axum::{
    extract::{ Path, Query, State },
    http::StatusCode,
    response::Json,
    routing::{ delete, get, post },
    Router,
};
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
        KeysResponse,
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

/// Create Redis routes
pub fn create_redis_routes(redis: Arc<Redis>) -> Router {
    Router::new()
        // String operations
        .route("/strings/:key", get(get_string))
        .route("/strings/:key", post(set_string))
        .route("/strings/:key", delete(delete_string))
        .route("/strings/:key/exists", get(exists_string))
        .route("/strings/:key/ttl", get(get_ttl))
        .route("/strings/:key/incr", post(incr_string))
        .route("/strings/:key/incrby", post(incr_by_string))
        .route("/strings/:key/setnx", post(set_if_not_exists))
        .route("/strings/:key/cas", post(compare_and_set))
        .route("/strings/batch/set", post(set_many))
        .route("/strings/batch/get", post(get_many))
        .route("/strings/batch/delete", post(delete_many))
        .route("/strings/batch/incr", post(incr_many))
        .route("/strings/batch/incrby", post(incr_many_by))
        // Key operations
        .route("/keys", get(list_keys))
        .route("/keys/:key", delete(delete_key))
        .route("/keys/:key/exists", get(exists_key))
        .route("/keys/:key/ttl", get(get_key_ttl))
        // Lua script operations
        .route("/scripts/rate-limiter", post(rate_limiter))
        .route("/scripts/multi-counter", post(multi_counter))
        .route("/scripts/multi-set-ttl", post(multi_set_with_ttl))
        .with_state(redis)
}

// String operation handlers

async fn get_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /strings/{}", key);

    match redis.string().get(&key) {
        Ok(Some(value)) => Ok(Json(ApiResponse::success(StringValue { value }))),
        Ok(None) =>
            Err((StatusCode::NOT_FOUND, Json(ApiResponse::error("Key not found".to_string())))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn set_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>,
    Json(request): Json<SetRequest>
) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/{}", key);

    let result = if let Some(ttl) = request.ttl {
        redis.string().set_with_expiry(&key, &request.value, ttl.try_into().unwrap_or(usize::MAX))
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

async fn delete_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("DELETE /strings/{}", key);

    match redis.string().del(&key) {
        Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn exists_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /strings/{}/exists", key);

    match redis.string().exists(&key) {
        Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn get_ttl(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /strings/{}/ttl", key);

    match redis.string().ttl(&key) {
        Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn incr_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/{}/incr", key);

    match redis.string().incr(&key) {
        Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn incr_by_string(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>,
    Json(request): Json<IncrByRequest>
) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/{}/incrby", key);

    match redis.string().incr_by(&key, request.increment) {
        Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn set_if_not_exists(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>,
    Json(request): Json<SetIfNotExistsRequest>
) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/{}/setnx", key);

    let script = RedisString::set_if_not_exists_script();
    let result: i32 = match redis.eval_script(&script, &[&key], &[&request.value]) {
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

async fn compare_and_set(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>,
    Json(request): Json<CompareAndSetRequest>
) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/{}/cas", key);

    let script = RedisString::compare_and_set_with_ttl_script();
    let ttl = request.ttl.unwrap_or(0);
    let result: i32 = match
        redis.eval_script(
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

// Batch operation handlers

async fn set_many(
    State(redis): State<Arc<Redis>>,
    Json(request): Json<SetManyRequest>
) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/batch/set");

    let kvs: Vec<(&str, &str)> = request.key_values
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    match redis.string().set_many(kvs) {
        Ok(_) =>
            Ok(
                Json(
                    ApiResponse::success(KeyValues {
                        key_values: request.key_values,
                    })
                )
            ),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn get_many(
    State(redis): State<Arc<Redis>>,
    Json(keys): Json<Vec<String>>
) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/batch/get");

    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    match redis.string().get_many(key_refs) {
        Ok(values) => {
            let mut key_values = HashMap::new();
            for (key, value) in keys.iter().zip(values.iter()) {
                if let Some(val) = value {
                    key_values.insert(key.clone(), val.clone());
                }
            }
            Ok(Json(ApiResponse::success(KeyValues { key_values })))
        }
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn delete_many(
    State(redis): State<Arc<Redis>>,
    Json(keys): Json<Vec<String>>
) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/batch/delete");

    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    match redis.string().del_many(key_refs) {
        Ok(_) =>
            Ok(
                Json(
                    ApiResponse::success(DeleteResponse {
                        deleted_count: keys.len() as u64,
                    })
                )
            ),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn incr_many(
    State(redis): State<Arc<Redis>>,
    Json(keys): Json<Vec<String>>
) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/batch/incr");

    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    match redis.string().incr_many(key_refs) {
        Ok(values) => {
            let result: Vec<IntegerValue> = values
                .into_iter()
                .map(|v| IntegerValue { value: v })
                .collect();
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn incr_many_by(
    State(redis): State<Arc<Redis>>,
    Json(kvs): Json<Vec<(String, i64)>>
) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /strings/batch/incrby");

    let kv_refs: Vec<(&str, i64)> = kvs
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();
    match redis.string().incr_many_by(kv_refs) {
        Ok(values) => {
            let result: Vec<IntegerValue> = values
                .into_iter()
                .map(|v| IntegerValue { value: v })
                .collect();
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => Err(handle_redis_error(e)),
    }
}

// Key operation handlers

async fn list_keys(
    State(redis): State<Arc<Redis>>,
    Query(query): Query<KeyQuery>
) -> Result<Json<ApiResponse<KeysResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /keys");

    let pattern = query.pattern.unwrap_or_else(|| "*".to_string());
    match redis.string().keys(&pattern) {
        Ok(keys) => Ok(Json(ApiResponse::success(KeysResponse { keys }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn delete_key(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("DELETE /keys/{}", key);

    match redis.string().del(&key) {
        Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn exists_key(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /keys/{}/exists", key);

    match redis.string().exists(&key) {
        Ok(exists) => Ok(Json(ApiResponse::success(ExistsResponse { exists }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

async fn get_key_ttl(
    State(redis): State<Arc<Redis>>,
    Path(key): Path<String>
) -> Result<Json<ApiResponse<TtlResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("GET /keys/{}/ttl", key);

    match redis.string().ttl(&key) {
        Ok(ttl) => Ok(Json(ApiResponse::success(TtlResponse { ttl }))),
        Err(e) => Err(handle_redis_error(e)),
    }
}

// Lua script handlers

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

async fn rate_limiter(
    State(redis): State<Arc<Redis>>,
    Json(request): Json<RateLimiterRequest>
) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/rate-limiter");

    let script = RedisString::rate_limiter_script();
    let result: i32 = match
        redis.eval_script(
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

    let allowed = result == 1;
    Ok(Json(ApiResponse::success(BooleanValue { value: allowed })))
}

async fn multi_counter(
    State(redis): State<Arc<Redis>>,
    Json(request): Json<MultiCounterRequest>
) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/multi-counter");

    let script = RedisString::multi_counter_script();
    let keys: Vec<&str> = request.counters
        .iter()
        .map(|(k, _)| k.as_str())
        .collect();
    let args: Vec<String> = request.counters
        .iter()
        .map(|(_, v)| v.to_string())
        .collect();
    let args_refs: Vec<&str> = args
        .iter()
        .map(|s| s.as_str())
        .collect();

    let result: Vec<i64> = match redis.eval_script(&script, keys, args_refs) {
        Ok(result) => result,
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    let values: Vec<IntegerValue> = result
        .into_iter()
        .map(|v| IntegerValue { value: v })
        .collect();

    Ok(Json(ApiResponse::success(values)))
}

async fn multi_set_with_ttl(
    State(redis): State<Arc<Redis>>,
    Json(request): Json<MultiSetTtlRequest>
) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/multi-set-ttl");

    let script = RedisString::multi_set_with_ttl_script();
    let keys: Vec<&str> = request.key_values
        .keys()
        .map(|k| k.as_str())
        .collect();
    let ttl_str = request.ttl.to_string();
    let args = vec![&ttl_str];

    let _: () = match redis.eval_script(&script, keys, args) {
        Ok(result) => result,
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    Ok(
        Json(
            ApiResponse::success(KeyValues {
                key_values: request.key_values,
            })
        )
    )
}
