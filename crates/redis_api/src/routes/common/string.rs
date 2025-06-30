use dbx_adapter::redis::primitives::string::RedisString;
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Type alias for complex return type
type PatternGroupedResults = Vec<(String, Vec<(String, Option<String>)>)>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringOperation {
    pub key: String,
    pub value: Option<String>,
    pub ttl: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringResponse {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringInfo {
    pub key: String,
    pub value: String,
    pub ttl: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
    pub encoding: String,
    pub size: usize,
}

fn redis_string(conn: Arc<Mutex<Connection>>) -> RedisString {
    RedisString::new(conn)
}

// =========================
// Single Key Operations
// =========================

pub fn get_string(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<Option<String>> {
    redis_string(conn).get(key)
}

pub fn set_string(conn: Arc<Mutex<Connection>>, key: &str, value: &str) -> redis::RedisResult<()> {
    redis_string(conn).set(key, value)
}

pub fn set_string_with_ttl(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    value: &str,
    ttl: u64,
) -> redis::RedisResult<()> {
    redis_string(conn).set_with_expiry(key, value, ttl as usize)
}

pub fn delete_string(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<bool> {
    // RedisString::del returns (), so we check existence first
    let exists = redis_string(conn.clone()).exists(key)?;
    if exists {
        redis_string(conn).del(key)?;
        Ok(true)
    } else {
        // Return false if key doesn't exist (Redis DEL behavior - returns 0 for non-existent keys)
        Ok(false)
    }
}

pub fn get_string_info(
    conn: Arc<Mutex<Connection>>,
    key: &str,
) -> redis::RedisResult<Option<StringInfo>> {
    let redis_str = redis_string(conn.clone());
    let type_ = if redis_str.exists(key)? {
        "string".to_string()
    } else {
        return Ok(None);
    };
    let value = redis_str.get(key)?.unwrap_or_default();
    let ttl = redis_str.ttl(key).ok();
    let encoding = "raw".to_string();
    let size = value.len();
    Ok(Some(StringInfo {
        key: key.to_string(),
        value,
        ttl,
        type_,
        encoding,
        size,
    }))
}

pub fn increment_string(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<i64> {
    redis_string(conn).incr(key)
}

pub fn increment_string_by(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    amount: i64,
) -> redis::RedisResult<i64> {
    redis_string(conn).incr_by(key, amount)
}

pub fn decrement_string(conn: Arc<Mutex<Connection>>, key: &str) -> redis::RedisResult<i64> {
    redis_string(conn).decr(key)
}

pub fn decrement_string_by(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    amount: i64,
) -> redis::RedisResult<i64> {
    redis_string(conn).decr_by(key, amount)
}

pub fn append_string(
    conn: Arc<Mutex<Connection>>,
    key: &str,
    value: &str,
) -> redis::RedisResult<usize> {
    redis_string(conn).append(key, value)
}

pub fn get_string_length(
    conn: Arc<Mutex<Connection>>,
    key: &str,
) -> redis::RedisResult<Option<usize>> {
    let redis_str = redis_string(conn);
    if redis_str.exists(key)? {
        let len = redis_str.get(key)?.map(|v| v.len()).unwrap_or(0);
        Ok(Some(len))
    } else {
        Ok(None)
    }
}

// =========================
// Batch Operations
// =========================

pub fn get_multiple_strings(
    conn: Arc<Mutex<Connection>>,
    keys: &[String],
) -> redis::RedisResult<Vec<Option<String>>> {
    let redis_str = redis_string(conn);
    let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
    redis_str.get_many(key_refs)
}

pub fn set_multiple_strings(
    conn: Arc<Mutex<Connection>>,
    operations: &[StringOperation],
) -> redis::RedisResult<()> {
    let redis_str = redis_string(conn);
    let mut kvs = Vec::new();
    for op in operations {
        if let Some(value) = &op.value {
            if let Some(ttl) = op.ttl {
                redis_str.set_with_expiry(&op.key, value, ttl as usize)?;
            } else {
                kvs.push((op.key.as_str(), value.as_str()));
            }
        }
    }
    if !kvs.is_empty() {
        redis_str.set_many(kvs)?;
    }
    Ok(())
}

/// Get multiple strings by patterns, expanding each pattern to matching keys
pub fn get_strings_by_patterns(
    conn: Arc<Mutex<Connection>>,
    patterns: &[String],
) -> redis::RedisResult<Vec<(String, Option<String>)>> {
    let redis_str = redis_string(conn);
    let mut results = Vec::new();

    for pattern in patterns {
        // Get all keys matching the pattern
        let matching_keys = redis_str.keys(pattern)?;

        if matching_keys.is_empty() {
            // If no keys match, add the pattern with None value
            results.push((pattern.clone(), None));
        } else {
            // Get values for all matching keys
            let key_refs: Vec<&str> = matching_keys.iter().map(|k| k.as_str()).collect();
            let values = redis_str.get_many(key_refs)?;

            // Combine keys with their values
            for (key, value) in matching_keys.into_iter().zip(values.into_iter()) {
                results.push((key, value));
            }
        }
    }

    Ok(results)
}

/// Get multiple strings by patterns, returning results grouped by pattern
pub fn get_strings_by_patterns_grouped(
    conn: Arc<Mutex<Connection>>,
    patterns: &[String],
) -> redis::RedisResult<PatternGroupedResults> {
    let redis_str = redis_string(conn);
    let mut results = Vec::new();

    for pattern in patterns {
        // Get all keys matching the pattern
        let matching_keys = redis_str.keys(pattern)?;

        if matching_keys.is_empty() {
            // If no keys match, add the pattern with empty results
            results.push((pattern.clone(), Vec::new()));
        } else {
            // Get values for all matching keys
            let key_refs: Vec<&str> = matching_keys.iter().map(|k| k.as_str()).collect();
            let values = redis_str.get_many(key_refs)?;

            // Combine keys with their values
            let pattern_results: Vec<(String, Option<String>)> =
                matching_keys.into_iter().zip(values.into_iter()).collect();

            results.push((pattern.clone(), pattern_results));
        }
    }

    Ok(results)
}
