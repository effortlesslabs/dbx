use serde::{ Serialize, Deserialize };
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use dbx_adapter::redis::primitives::admin::{ AdminOperations, HealthCheck, ServerStatus };
use redis::Connection;
use axum::{ extract::State, http::StatusCode, Json };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSetRequest {
    pub parameter: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigGetRequest {
    pub parameter: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub info: String,
    pub section: Option<String>,
}

fn redis_admin(conn: Arc<Mutex<Connection>>) -> AdminOperations {
    AdminOperations::new(conn)
}

// =========================
// Basic Health & Status Operations
// =========================

pub fn ping_server(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<String> {
    redis_admin(conn).ping()
}

pub fn get_server_info(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<String> {
    redis_admin(conn).info()
}

pub fn get_server_info_section(
    conn: Arc<Mutex<Connection>>,
    section: &str
) -> redis::RedisResult<String> {
    redis_admin(conn).info_section(section)
}

pub fn get_database_size(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<i64> {
    redis_admin(conn).dbsize()
}

pub fn get_server_time(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<(i64, i64)> {
    redis_admin(conn).time()
}

pub fn get_server_version(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<String> {
    redis_admin(conn).version()
}

// =========================
// Health Check Operations
// =========================

pub fn health_check(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<HealthCheck> {
    redis_admin(conn).health_check()
}

pub fn server_status(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<ServerStatus> {
    redis_admin(conn).server_status()
}

// =========================
// Statistics Operations
// =========================

pub fn get_memory_stats(
    conn: Arc<Mutex<Connection>>
) -> redis::RedisResult<HashMap<String, String>> {
    redis_admin(conn).memory_stats()
}

pub fn get_client_stats(
    conn: Arc<Mutex<Connection>>
) -> redis::RedisResult<HashMap<String, String>> {
    redis_admin(conn).client_stats()
}

pub fn get_server_stats(
    conn: Arc<Mutex<Connection>>
) -> redis::RedisResult<HashMap<String, String>> {
    redis_admin(conn).server_stats()
}

// =========================
// Configuration Operations
// =========================

pub fn config_set(
    conn: Arc<Mutex<Connection>>,
    parameter: &str,
    value: &str
) -> redis::RedisResult<()> {
    redis_admin(conn).config_set(parameter, value)
}

pub fn config_get(conn: Arc<Mutex<Connection>>, parameter: &str) -> redis::RedisResult<String> {
    redis_admin(conn).config_get(parameter)
}

pub fn config_get_all(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<HashMap<String, String>> {
    redis_admin(conn).config_get_all()
}

pub fn config_reset_statistics(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<()> {
    redis_admin(conn).config_resetstat()
}

pub fn config_rewrite(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<()> {
    redis_admin(conn).config_rewrite()
}

// =========================
// Database Management Operations
// =========================

pub fn flush_current_database(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<()> {
    redis_admin(conn).flushdb()
}

pub fn flush_all_databases(conn: Arc<Mutex<Connection>>) -> redis::RedisResult<()> {
    redis_admin(conn).flushall()
}
