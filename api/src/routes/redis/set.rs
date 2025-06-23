use axum::{
    extract::{ State, Path, Json },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post, delete, any },
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::routes::common::set::{
    add_to_set,
    remove_from_set,
    get_set_members,
    set_exists,
    get_set_cardinality,
    intersect_sets,
    union_sets,
    difference_sets,
    SetOperation,
};
use dbx_crates::adapter::redis::client::RedisPool;

#[derive(Debug, Deserialize)]
struct SetMemberRequest {
    member: String,
}

#[derive(Debug, Deserialize)]
struct SetMembersRequest {
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SetKeysRequest {
    keys: Vec<String>,
}

// Add member to set
async fn add_to_set_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>,
    Json(payload): Json<SetMemberRequest>
) -> Result<Json<usize>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let added = add_to_set(conn_arc, &key, &[&payload.member]).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(added))
}

// Remove member from set
async fn remove_from_set_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, member)): Path<(String, String)>
) -> Result<Json<usize>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let removed = remove_from_set(conn_arc, &key, &[&member]).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(removed))
}

// Get all set members
async fn get_set_members_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let members = get_set_members(conn_arc, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(members))
}

// Get set cardinality
async fn get_set_cardinality_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(key): Path<String>
) -> Result<Json<usize>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let cardinality = get_set_cardinality(conn_arc, &key).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(cardinality))
}

// Check if member exists in set
async fn set_exists_handler(
    State(pool): State<Arc<RedisPool>>,
    Path((key, member)): Path<(String, String)>
) -> Result<Json<bool>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let exists = set_exists(conn_arc, &key, &member).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(exists))
}

// Intersect sets
async fn intersect_sets_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<SetKeysRequest>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let key_refs: Vec<&str> = payload.keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let result = intersect_sets(conn_arc, &key_refs).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

// Union sets
async fn union_sets_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<SetKeysRequest>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let key_refs: Vec<&str> = payload.keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let result = union_sets(conn_arc, &key_refs).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(result))
}

// Difference of sets
async fn difference_sets_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<SetKeysRequest>
) -> Result<Json<Vec<String>>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let key_refs: Vec<&str> = payload.keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let result = difference_sets(conn_arc, &key_refs).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(result))
}

async fn method_not_allowed() -> impl IntoResponse {
    (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
}

pub fn create_redis_set_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        .route("/set/:key", post(add_to_set_handler))
        .route("/set/:key/:member", delete(remove_from_set_handler))
        .route("/set/:key/members", get(get_set_members_handler))
        .route("/set/:key/cardinality", get(get_set_cardinality_handler))
        .route("/set/:key/:member/exists", get(set_exists_handler))
        .route("/set/intersect", post(intersect_sets_handler))
        .route("/set/union", post(union_sets_handler))
        .route("/set/difference", post(difference_sets_handler))
        .with_state(pool)
}
