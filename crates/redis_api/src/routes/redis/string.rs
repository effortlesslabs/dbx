use axum::{
    extract::{ State, Path, Json },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post, delete, any },
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::routes::common::string::{
    get_string,
    set_string,
    delete_string,
    get_string_info,
    get_multiple_strings,
    set_multiple_strings,
    get_strings_by_patterns,
    get_strings_by_patterns_grouped,
    StringOperation,
    StringInfo,
    set_string_with_ttl,
};
use dbx_adapter::redis::client::RedisPool;

#[derive(Debug, Deserialize)]
struct SetStringRequest {
    value: String,
    ttl: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct BatchGetRequest {
    keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BatchSetRequest {
    operations: Vec<StringOperation>,
}

#[derive(Debug, Deserialize)]
struct BatchGetPatternsRequest {
    patterns: Vec<String>,
    grouped: Option<bool>,
}

async fn get_string_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Option<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let value = get_string(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(value))
}

async fn set_string_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<SetStringRequest>
) -> Result<StatusCode, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    if let Some(ttl) = payload.ttl {
        set_string_with_ttl(conn_arc, &key, &payload.value, ttl).map_err(
            |_| StatusCode::INTERNAL_SERVER_ERROR
        )?;
    } else {
        set_string(conn_arc, &key, &payload.value).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(StatusCode::OK)
}

async fn delete_string_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let deleted = delete_string(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(deleted))
}

async fn get_string_info_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Option<StringInfo>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let info = get_string_info(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(info))
}

// Batch operations
async fn batch_get_strings_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchGetRequest>
) -> Result<Json<Vec<Option<String>>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let values = get_multiple_strings(conn_arc, &payload.keys).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(values))
}

async fn batch_set_strings_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchSetRequest>
) -> Result<StatusCode, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    set_multiple_strings(conn_arc, &payload.operations).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(StatusCode::OK)
}

async fn batch_get_patterns_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<BatchGetPatternsRequest>
) -> Result<Json<serde_json::Value>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));

    if payload.grouped.unwrap_or(false) {
        let results = get_strings_by_patterns_grouped(conn_arc, &payload.patterns).map_err(
            |_| StatusCode::INTERNAL_SERVER_ERROR
        )?;

        let grouped_results: Vec<serde_json::Value> = results
            .into_iter()
            .map(|(pattern, key_values)| {
                let key_value_map: std::collections::HashMap<String, Option<String>> = key_values
                    .into_iter()
                    .collect();
                serde_json::json!({
                    "pattern": pattern,
                    "results": key_value_map
                })
            })
            .collect();

        Ok(
            Json(
                serde_json::json!({
            "grouped": true,
            "results": grouped_results
        })
            )
        )
    } else {
        let results = get_strings_by_patterns(conn_arc, &payload.patterns).map_err(
            |_| StatusCode::INTERNAL_SERVER_ERROR
        )?;

        let key_value_map: std::collections::HashMap<String, Option<String>> = results
            .into_iter()
            .collect();

        Ok(
            Json(
                serde_json::json!({
            "grouped": false,
            "results": key_value_map
        })
            )
        )
    }
}

async fn method_not_allowed() -> impl IntoResponse {
    (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
}

pub fn create_redis_string_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        .route("/string/:key", get(get_string_handler))
        .route("/string/:key", post(set_string_handler))
        .route("/string/:key", delete(delete_string_handler))
        .route("/string/:key", any(method_not_allowed))
        .route("/string/:key/info", get(get_string_info_handler))
        .route("/string/batch/get", post(batch_get_strings_handler))
        .route("/string/batch/set", post(batch_set_strings_handler))
        .route("/string/batch/patterns", post(batch_get_patterns_handler))
        .with_state(pool)
}
