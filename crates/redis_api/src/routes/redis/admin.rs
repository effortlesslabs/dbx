use axum::{
    extract::{ State, Path, Json },
    http::StatusCode,
    routing::{ get, post, delete },
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use std::collections::HashMap;
use crate::routes::common::admin::{
    ping_server,
    get_server_info,
    get_server_info_section,
    get_database_size,
    get_server_time,
    get_server_version,
    health_check,
    server_status,
    get_memory_stats,
    get_client_stats,
    get_server_stats,
    config_set,
    config_get,
    config_get_all,
    config_reset_statistics,
    config_rewrite,
    flush_current_database,
    flush_all_databases,
};
use dbx_adapter::redis::client::RedisPool;
use dbx_adapter::redis::primitives::admin::{ HealthCheck, ServerStatus };

#[derive(Debug, Deserialize)]
struct ConfigSetPayload {
    parameter: String,
    value: String,
}

// =========================
// Basic Health & Status Handlers
// =========================

async fn ping_handler(State(pool): State<Arc<RedisPool>>) -> Result<Json<String>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let response = ping_server(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(response))
}

async fn info_handler(State(pool): State<Arc<RedisPool>>) -> Result<Json<String>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let info = get_server_info(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(info))
}

async fn info_section_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(section): Path<String>
) -> Result<Json<String>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let info = get_server_info_section(conn_arc, &section).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(Json(info))
}

async fn dbsize_handler(State(pool): State<Arc<RedisPool>>) -> Result<Json<i64>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let size = get_database_size(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(size))
}

async fn time_handler(State(pool): State<Arc<RedisPool>>) -> Result<Json<(i64, i64)>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let time = get_server_time(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(time))
}

async fn version_handler(State(pool): State<Arc<RedisPool>>) -> Result<Json<String>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let version = get_server_version(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(version))
}

// =========================
// Health Check Handlers
// =========================

async fn health_check_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<HealthCheck>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let health = health_check(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(health))
}

async fn server_status_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<ServerStatus>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let status = server_status(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(status))
}

// =========================
// Statistics Handlers
// =========================

async fn memory_stats_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<HashMap<String, String>>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let stats = get_memory_stats(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}

async fn client_stats_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<HashMap<String, String>>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let stats = get_client_stats(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}

async fn server_stats_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<HashMap<String, String>>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let stats = get_server_stats(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}

// =========================
// Configuration Handlers
// =========================

async fn config_set_handler(
    State(pool): State<Arc<RedisPool>>,
    Json(payload): Json<ConfigSetPayload>
) -> Result<StatusCode, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    config_set(conn_arc, &payload.parameter, &payload.value).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(StatusCode::OK)
}

async fn config_get_handler(
    State(pool): State<Arc<RedisPool>>,
    Path(parameter): Path<String>
) -> Result<Json<String>, StatusCode> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let value = config_get(conn_arc, &parameter).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(value))
}

async fn config_get_all_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    Json<HashMap<String, String>>,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    let config = config_get_all(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(config))
}

async fn config_reset_statistics_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    StatusCode,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    config_reset_statistics(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

async fn config_rewrite_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    StatusCode,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    config_rewrite(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

// =========================
// Database Management Handlers
// =========================

async fn flush_current_database_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    StatusCode,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    flush_current_database(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

async fn flush_all_databases_handler(State(pool): State<Arc<RedisPool>>) -> Result<
    StatusCode,
    StatusCode
> {
    let conn = pool.get_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let conn_arc = Arc::new(std::sync::Mutex::new(conn));
    flush_all_databases(conn_arc).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub fn create_redis_admin_routes(pool: Arc<RedisPool>) -> Router {
    Router::new()
        // Basic Health & Status routes
        .route("/admin/ping", get(ping_handler))
        .route("/admin/info", get(info_handler))
        .route("/admin/info/:section", get(info_section_handler))
        .route("/admin/dbsize", get(dbsize_handler))
        .route("/admin/time", get(time_handler))
        .route("/admin/version", get(version_handler))

        // Health Check routes
        .route("/admin/health", get(health_check_handler))
        .route("/admin/status", get(server_status_handler))

        // Statistics routes
        .route("/admin/stats/memory", get(memory_stats_handler))
        .route("/admin/stats/clients", get(client_stats_handler))
        .route("/admin/stats/server", get(server_stats_handler))

        // Configuration routes
        .route("/admin/config/set", post(config_set_handler))
        .route("/admin/config/get/:parameter", get(config_get_handler))
        .route("/admin/config/all", get(config_get_all_handler))
        .route("/admin/config/resetstat", post(config_reset_statistics_handler))
        .route("/admin/config/rewrite", post(config_rewrite_handler))

        // Database Management routes
        .route("/admin/flushdb", delete(flush_current_database_handler))
        .route("/admin/flushall", delete(flush_all_databases_handler))
        .with_state(pool)
}
