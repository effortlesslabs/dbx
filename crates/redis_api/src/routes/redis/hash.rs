use crate::routes::common::hash::{
    check_multiple_hash_fields,
    delete_hash,
    delete_hash_field,
    delete_multiple_hash_fields,
    get_all_hash_fields,
    get_hash_field,
    get_hash_fields,
    get_hash_keys,
    get_hash_length,
    get_hash_ttl,
    get_hash_values,
    get_multiple_hash_fields,
    get_multiple_hash_lengths,
    get_random_hash_field,
    get_random_hash_fields,
    get_random_hash_fields_with_values,
    hash_exists,
    hash_exists_key,
    increment_hash_field,
    increment_hash_field_float,
    set_hash_field,
    set_hash_field_if_not_exists,
    set_hash_ttl,
    set_multiple_hash_fields,
    set_multiple_hashes,
};
use axum::{
    extract::{ Json, Path, State },
    http::StatusCode,
    routing::{ delete, get, post },
    Router,
};
use dbx_adapter::redis::client::RedisPool;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct SetHashFieldRequest {
    value: String,
}

#[derive(Debug, Deserialize)]
struct SetMultipleHashFieldsRequest {
    fields: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct GetHashFieldsRequest {
    fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IncrementHashFieldRequest {
    increment: i64,
}

#[derive(Debug, Deserialize)]
struct IncrementHashFieldFloatRequest {
    increment: f64,
}

#[derive(Debug, Deserialize)]
struct GetRandomHashFieldsRequest {
    count: isize,
}

#[derive(Debug, Deserialize)]
struct GetRandomHashFieldsWithValuesRequest {
    count: isize,
}

#[derive(Debug, Deserialize)]
struct SetHashTtlRequest {
    ttl: u64,
}

#[derive(Debug, Deserialize)]
struct BatchGetHashFieldsRequest {
    hash_fields: Vec<(String, String)>, // (key, field) pairs
}

#[derive(Debug, Deserialize)]
struct BatchSetHashFieldsRequest {
    hash_operations: Vec<(String, Vec<(String, String)>)>, // (key, [(field, value)]) pairs
}

#[derive(Debug, Deserialize)]
struct BatchDeleteHashFieldsRequest {
    hash_fields: Vec<(String, Vec<String>)>, // (key, [fields]) pairs
}

#[derive(Debug, Deserialize)]
struct BatchCheckHashFieldsRequest {
    hash_fields: Vec<(String, String)>, // (key, field) pairs
}

#[derive(Debug, Deserialize)]
struct BatchGetHashLengthsRequest {
    keys: Vec<String>,
}

// Single field operations
async fn get_hash_field_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>
) -> Result<Json<Option<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let value = get_hash_field(conn_arc, &key, &field).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(value))
}

async fn set_hash_field_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>,
    Json(payload): Json<SetHashFieldRequest>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let result = set_hash_field(conn_arc, &key, &field, &payload.value).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

async fn delete_hash_field_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let deleted = delete_hash_field(conn_arc, &key, &field).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(deleted))
}

async fn hash_exists_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let exists = hash_exists(conn_arc, &key, &field).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(exists))
}

// Hash operations
async fn get_all_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<std::collections::HashMap<String, String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let fields = get_all_hash_fields(conn_arc, &key).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(fields))
}

async fn get_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<GetHashFieldsRequest>
) -> Result<Json<Vec<Option<String>>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let field_refs: Vec<&str> = payload.fields
        .iter()
        .map(|f| f.as_str())
        .collect();
    let values = get_hash_fields(conn_arc, &key, &field_refs).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(values))
}

async fn set_multiple_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<SetMultipleHashFieldsRequest>
) -> Result<StatusCode, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let field_values: Vec<(&str, &str)> = payload.fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    set_multiple_hash_fields(conn_arc, &key, &field_values).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(StatusCode::OK)
}

async fn get_hash_length_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<usize>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let length = get_hash_length(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(length))
}

async fn get_hash_keys_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let keys = get_hash_keys(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(keys))
}

async fn get_hash_values_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let values = get_hash_values(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(values))
}

async fn increment_hash_field_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>,
    Json(payload): Json<IncrementHashFieldRequest>
) -> Result<Json<i64>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let result = increment_hash_field(conn_arc, &key, &field, payload.increment).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

async fn increment_hash_field_float_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>,
    Json(payload): Json<IncrementHashFieldFloatRequest>
) -> Result<Json<f64>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let result = increment_hash_field_float(conn_arc, &key, &field, payload.increment).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

async fn set_hash_field_if_not_exists_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, field)): Path<(String, String)>,
    Json(payload): Json<SetHashFieldRequest>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let result = set_hash_field_if_not_exists(conn_arc, &key, &field, &payload.value).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

async fn get_random_hash_field_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Option<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let field = get_random_hash_field(conn_arc, &key).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(field))
}

async fn get_random_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<GetRandomHashFieldsRequest>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let fields = get_random_hash_fields(conn_arc, &key, payload.count).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(fields))
}

async fn get_random_hash_fields_with_values_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<GetRandomHashFieldsWithValuesRequest>
) -> Result<Json<Vec<(String, String)>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let fields = get_random_hash_fields_with_values(conn_arc, &key, payload.count).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(fields))
}

// Hash management
async fn delete_hash_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let deleted = delete_hash(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(deleted))
}

async fn hash_exists_key_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let exists = hash_exists_key(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(exists))
}

async fn get_hash_ttl_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<i64>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let ttl = get_hash_ttl(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ttl))
}

async fn set_hash_ttl_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<SetHashTtlRequest>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let result = set_hash_ttl(conn_arc, &key, payload.ttl).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

// Batch operations
async fn batch_get_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchGetHashFieldsRequest>
) -> Result<Json<Vec<Option<String>>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let hash_fields: Vec<(&str, &str)> = payload.hash_fields
        .iter()
        .map(|(k, f)| (k.as_str(), f.as_str()))
        .collect();
    let values = get_multiple_hash_fields(conn_arc, hash_fields).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(values))
}

async fn batch_set_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchSetHashFieldsRequest>
) -> Result<Json<Vec<bool>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let hash_operations: Vec<(&str, Vec<(&str, &str)>)> = payload.hash_operations
        .iter()
        .map(|(k, fields)| {
            let field_values: Vec<(&str, &str)> = fields
                .iter()
                .map(|(f, v)| (f.as_str(), v.as_str()))
                .collect();
            (k.as_str(), field_values)
        })
        .collect();
    let results = set_multiple_hashes(conn_arc, hash_operations).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(results))
}

async fn batch_delete_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchDeleteHashFieldsRequest>
) -> Result<Json<Vec<usize>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let hash_fields: Vec<(&str, Vec<&str>)> = payload.hash_fields
        .iter()
        .map(|(k, fields)| {
            let field_refs: Vec<&str> = fields
                .iter()
                .map(|f| f.as_str())
                .collect();
            (k.as_str(), field_refs)
        })
        .collect();
    let results = delete_multiple_hash_fields(conn_arc, hash_fields).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(results))
}

async fn batch_check_hash_fields_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchCheckHashFieldsRequest>
) -> Result<Json<Vec<bool>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let hash_fields: Vec<(&str, &str)> = payload.hash_fields
        .iter()
        .map(|(k, f)| (k.as_str(), f.as_str()))
        .collect();
    let results = check_multiple_hash_fields(conn_arc, hash_fields).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(results))
}

async fn batch_get_hash_lengths_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchGetHashLengthsRequest>
) -> Result<Json<Vec<usize>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let key_refs: Vec<&str> = payload.keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let lengths = get_multiple_hash_lengths(conn_arc, key_refs).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(lengths))
}

pub fn create_redis_hash_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        // Single field operations
        .route("/hash/:key/:field", get(get_hash_field_handler))
        .route("/hash/:key/:field", post(set_hash_field_handler))
        .route("/hash/:key/:field", delete(delete_hash_field_handler))
        .route("/hash/:key/:field/exists", get(hash_exists_handler))
        .route("/hash/:key/:field/increment", post(increment_hash_field_handler))
        .route("/hash/:key/:field/increment_float", post(increment_hash_field_float_handler))
        .route("/hash/:key/:field/setnx", post(set_hash_field_if_not_exists_handler))
        // Hash operations
        .route("/hash/:key", get(get_all_hash_fields_handler))
        .route("/hash/:key/fields", post(get_hash_fields_handler))
        .route("/hash/:key/batch", post(set_multiple_hash_fields_handler))
        .route("/hash/:key/length", get(get_hash_length_handler))
        .route("/hash/:key/keys", get(get_hash_keys_handler))
        .route("/hash/:key/values", get(get_hash_values_handler))
        .route("/hash/:key/random", get(get_random_hash_field_handler))
        .route("/hash/:key/random_fields", post(get_random_hash_fields_handler))
        .route(
            "/hash/:key/random_fields_with_values",
            post(get_random_hash_fields_with_values_handler)
        )
        // Hash management
        .route("/hash/:key", delete(delete_hash_handler))
        .route("/hash/:key/exists", get(hash_exists_key_handler))
        .route("/hash/:key/ttl", get(get_hash_ttl_handler))
        .route("/hash/:key/ttl", post(set_hash_ttl_handler))
        // Batch operations
        .route("/hash/batch/get", post(batch_get_hash_fields_handler))
        .route("/hash/batch/set", post(batch_set_hash_fields_handler))
        .route("/hash/batch/delete", post(batch_delete_hash_fields_handler))
        .route("/hash/batch/exists", post(batch_check_hash_fields_handler))
        .route("/hash/batch/lengths", post(batch_get_hash_lengths_handler))
        .with_state(pool)
}
