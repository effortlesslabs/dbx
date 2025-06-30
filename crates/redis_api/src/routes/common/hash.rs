use dbx_adapter::redis::primitives::hash::RedisHash;
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashOperation {
    pub key: String,
    pub field: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashInfo {
    pub key: String,
    pub field: String,
    pub value: String,
    pub ttl: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashField {
    pub field: String,
    pub value: String,
}

fn redis_hash(conn: Arc<Mutex<Connection>>) -> RedisHash {
    RedisHash::new(conn)
}

// =========================
// Single Field Operations
// =========================

pub fn get_hash_field(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
) -> redis::RedisResult<Option<String>> {
    redis_hash(conn).hget(key, field)
}

pub fn set_hash_field(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
    value: &str,
) -> redis::RedisResult<bool> {
    redis_hash(conn).hset(key, field, value)
}

pub fn delete_hash_field(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
) -> redis::RedisResult<bool> {
    let deleted = redis_hash(conn).hdel(key, &[field])?;
    Ok(deleted > 0)
}

pub fn hash_exists(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
) -> redis::RedisResult<bool> {
    redis_hash(conn).hexists(key, field)
}

// =========================
// Hash Operations
// =========================

pub fn get_all_hash_fields(
    conn: Arc<Mutex<Connection>>,
    key: &str,
) -> redis::RedisResult<std::collections::HashMap<String, String>> {
    redis_hash(conn).hgetall(key)
}

pub fn get_hash_fields(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    fields: &[&str],
) -> redis::RedisResult<Vec<Option<String>>> {
    redis_hash(conn).hmget(key, fields)
}

pub fn set_multiple_hash_fields(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    fields: &[(&str, &str)],
) -> redis::RedisResult<()> {
    redis_hash(conn).hmset(key, fields)
}

pub fn get_hash_length(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<usize> {
    redis_hash(conn).hlen(key)
}

pub fn get_hash_keys(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<Vec<String>> {
    redis_hash(conn).hkeys(key)
}

pub fn get_hash_values(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<Vec<String>> {
    redis_hash(conn).hvals(key)
}

pub fn increment_hash_field(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
    increment: i64,
) -> redis::RedisResult<i64> {
    redis_hash(conn).hincrby(key, field, increment)
}

pub fn increment_hash_field_float(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
    increment: f64,
) -> redis::RedisResult<f64> {
    redis_hash(conn).hincrbyfloat(key, field, increment)
}

pub fn set_hash_field_if_not_exists(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    field: &str,
    value: &str,
) -> redis::RedisResult<bool> {
    redis_hash(conn).hsetnx(key, field, value)
}

pub fn get_random_hash_field(
    conn: Arc<Mutex<Connection>>,
    key: &str,
) -> redis::RedisResult<Option<String>> {
    redis_hash(conn).hrandfield(key)
}

pub fn get_random_hash_fields(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    count: isize,
) -> redis::RedisResult<Vec<String>> {
    redis_hash(conn).hrandfield_count(key, count)
}

pub fn get_random_hash_fields_with_values(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    count: isize,
) -> redis::RedisResult<Vec<(String, String)>> {
    redis_hash(conn).hrandfield_withvalues(key, count)
}

// =========================
// Hash Management
// =========================

pub fn delete_hash(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<bool> {
    let exists = redis_hash(conn.clone()).exists(key)?;
    if exists {
        redis_hash(conn).del(key)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn hash_exists_key(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<bool> {
    redis_hash(conn).exists(key)
}

pub fn get_hash_ttl(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<i64> {
    redis_hash(conn).ttl(key)
}

pub fn set_hash_ttl(conn: Arc<Mutex<Connection>>, key: &str, ttl: u64) -> redis::RedisResult<bool> {
    redis_hash(conn).expire(key, ttl)
}

// =========================
// Batch Operations
// =========================

pub fn get_multiple_hash_fields(
    conn: Arc<Mutex<Connection>>,
    hash_fields: Vec<(&str, &str)>,
) -> redis::RedisResult<Vec<Option<String>>> {
    redis_hash(conn).hget_many(hash_fields)
}

pub fn set_multiple_hashes(
    conn: Arc<Mutex<Connection>>,
    hash_operations: Vec<(&str, Vec<(&str, &str)>)>,
) -> redis::RedisResult<Vec<bool>> {
    redis_hash(conn).hset_many(hash_operations)
}

pub fn delete_multiple_hash_fields(
    conn: Arc<Mutex<Connection>>,
    hash_fields: Vec<(&str, Vec<&str>)>,
) -> redis::RedisResult<Vec<usize>> {
    redis_hash(conn).hdel_many(hash_fields)
}

pub fn check_multiple_hash_fields(
    conn: Arc<Mutex<Connection>>,
    hash_fields: Vec<(&str, &str)>,
) -> redis::RedisResult<Vec<bool>> {
    redis_hash(conn).hexists_many(hash_fields)
}

pub fn get_multiple_hash_lengths(
    conn: Arc<Mutex<Connection>>,
    keys: Vec<&str>,
) -> redis::RedisResult<Vec<usize>> {
    redis_hash(conn).hlen_many(keys)
}
