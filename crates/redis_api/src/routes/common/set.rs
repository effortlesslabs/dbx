use serde::{ Serialize, Deserialize };
use std::sync::{ Arc, Mutex };
use dbx_adapter::redis::primitives::set::RedisSet;
use redis::Connection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetOperation {
    pub key: String,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetResponse {
    pub success: bool,
    pub data: Option<Vec<String>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetInfo {
    pub key: String,
    pub members: Vec<String>,
    pub cardinality: usize,
    pub ttl: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetOperationRequest {
    pub keys: Vec<String>,
    pub members: Option<Vec<String>>,
}

fn redis_set(conn: Arc<Mutex<Connection>>) -> RedisSet {
    RedisSet::new(conn)
}

// =========================
// Single Set Operations
// =========================

pub fn add_to_set(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    members: &[&str]
) -> redis::RedisResult<usize> {
    redis_set(conn).sadd(key, members)
}

pub fn remove_from_set(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    members: &[&str]
) -> redis::RedisResult<usize> {
    redis_set(conn).srem(key, members)
}

pub fn get_set_members(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).smembers(key)
}

pub fn set_exists(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    member: &str
) -> redis::RedisResult<bool> {
    redis_set(conn).sismember(key, member)
}

pub fn get_set_cardinality(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<usize> {
    redis_set(conn).scard(key)
}

pub fn get_random_set_member(
    conn: Arc<Mutex<Connection>>,
    key: &str
) -> redis::RedisResult<Option<String>> {
    redis_set(conn).srandmember(key)
}

pub fn get_random_set_members(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    count: usize
) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).srandmember_count(key, count)
}

pub fn pop_set_member(
    conn: Arc<Mutex<Connection>>,
    key: &str
) -> redis::RedisResult<Option<String>> {
    redis_set(conn).spop(key)
}

pub fn pop_set_members(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    count: usize
) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).spop_count(key, count)
}

pub fn move_set_member(
    conn: Arc<Mutex<Connection>>,
    source: &str,
    destination: &str,
    member: &str
) -> redis::RedisResult<bool> {
    redis_set(conn).smove(source, destination, member)
}

// =========================
// Set Operations
// =========================

pub fn intersect_sets(
    conn: Arc<Mutex<Connection>>,
    keys: &[&str]
) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).sinter(keys)
}

pub fn union_sets(conn: Arc<Mutex<Connection>>, keys: &[&str]) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).sunion(keys)
}

pub fn difference_sets(
    conn: Arc<Mutex<Connection>>,
    keys: &[&str]
) -> redis::RedisResult<Vec<String>> {
    redis_set(conn).sdiff(keys)
}

pub fn intersect_sets_store(
    conn: Arc<Mutex<Connection>>,
    destination: &str,
    keys: &[&str]
) -> redis::RedisResult<usize> {
    redis_set(conn).sinterstore(destination, keys)
}

pub fn union_sets_store(
    conn: Arc<Mutex<Connection>>,
    destination: &str,
    keys: &[&str]
) -> redis::RedisResult<usize> {
    redis_set(conn).sunionstore(destination, keys)
}

pub fn difference_sets_store(
    conn: Arc<Mutex<Connection>>,
    destination: &str,
    keys: &[&str]
) -> redis::RedisResult<usize> {
    redis_set(conn).sdiffstore(destination, keys)
}

// =========================
// Set Management
// =========================

pub fn delete_set(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<bool> {
    let exists = redis_set(conn.clone()).exists(key)?;
    if exists {
        redis_set(conn).del(key)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn set_exists_key(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<bool> {
    redis_set(conn).exists(key)
}

pub fn get_set_ttl(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<i64> {
    redis_set(conn).ttl(key)
}

pub fn set_set_ttl(conn: Arc<Mutex<Connection>>, key: &str, ttl: u64) -> redis::RedisResult<bool> {
    redis_set(conn).expire(key, ttl)
}

// =========================
// Batch Operations
// =========================

pub fn add_to_multiple_sets(
    conn: Arc<Mutex<Connection>>,
    set_members: Vec<(&str, Vec<&str>)>
) -> redis::RedisResult<Vec<usize>> {
    redis_set(conn).sadd_many(set_members)
}

pub fn remove_from_multiple_sets(
    conn: Arc<Mutex<Connection>>,
    set_members: Vec<(&str, Vec<&str>)>
) -> redis::RedisResult<Vec<usize>> {
    redis_set(conn).srem_many(set_members)
}

pub fn get_multiple_set_members(
    conn: Arc<Mutex<Connection>>,
    keys: Vec<&str>
) -> redis::RedisResult<Vec<Vec<String>>> {
    redis_set(conn).smembers_many(keys)
}

pub fn check_multiple_set_members(
    conn: Arc<Mutex<Connection>>,
    key_members: Vec<(&str, &str)>
) -> redis::RedisResult<Vec<bool>> {
    redis_set(conn).sismember_many(key_members)
}

pub fn get_multiple_set_cardinalities(
    conn: Arc<Mutex<Connection>>,
    keys: Vec<&str>
) -> redis::RedisResult<Vec<usize>> {
    redis_set(conn).scard_many(keys)
}

pub fn delete_multiple_sets(
    conn: Arc<Mutex<Connection>>,
    keys: Vec<&str>
) -> redis::RedisResult<()> {
    redis_set(conn).del_many(keys)
}
