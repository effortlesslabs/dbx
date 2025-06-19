use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

/// Represents a Redis Hash data type with operations for manipulating field-value maps.
///
/// This implementation supports:
/// - Basic hash operations (set, get, delete fields)
/// - Field existence and counting operations
/// - Increment operations for numeric fields
/// - Bulk operations on multiple fields
/// - Pipelined operations (for efficiency)
/// - Scanning operations
#[derive(Clone)]
pub struct RedisHash {
    conn: Arc<Mutex<Connection>>,
}

impl RedisHash {
    /// Creates a new RedisHash instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic Hash Operations

    /// Sets the value of a field in a hash
    pub fn hset(&self, key: &str, field: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset(key, field, value)
    }

    /// Sets multiple field-value pairs in a hash
    pub fn hset_multiple(&self, key: &str, items: &[(&str, &str)]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset_multiple(key, items)
    }

    /// Sets the value of a field only if the field doesn't exist
    pub fn hsetnx(&self, key: &str, field: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset_nx(key, field, value)
    }

    /// Gets the value of a field in a hash
    pub fn hget(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hget(key, field)
    }

    /// Gets the values of multiple fields in a hash
    pub fn hmget(&self, key: &str, fields: &[&str]) -> RedisResult<Vec<Option<String>>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hget_multiple(key, fields)
    }

    /// Gets all fields and values in a hash
    pub fn hgetall(&self, key: &str) -> RedisResult<HashMap<String, String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hgetall(key)
    }

    /// Deletes one or more fields from a hash
    pub fn hdel(&self, key: &str, field: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel(key, field)
    }

    /// Deletes multiple fields from a hash
    pub fn hdel_multiple(&self, key: &str, fields: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel_multiple(key, fields)
    }

    /// Checks if a field exists in a hash
    pub fn hexists(&self, key: &str, field: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.hexists(key, field)
    }

    /// Returns the number of fields in a hash
    pub fn hlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hlen(key)
    }

    /// Returns all field names in a hash
    pub fn hkeys(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hkeys(key)
    }

    /// Returns all values in a hash
    pub fn hvals(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hvals(key)
    }

    /// Returns the length of the value stored in a field
    pub fn hstrlen(&self, key: &str, field: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HSTRLEN")
            .arg(key)
            .arg(field)
            .query(&mut *conn)
    }

    // Increment Operations

    /// Increments the integer value of a field by the given amount
    pub fn hincrby(&self, key: &str, field: &str, increment: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hincr(key, field, increment)
    }

    /// Increments the float value of a field by the given amount
    pub fn hincrbyfloat(&self, key: &str, field: &str, increment: f64) -> RedisResult<f64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HINCRBYFLOAT")
            .arg(key)
            .arg(field)
            .arg(increment)
            .query(&mut *conn)
    }

    // Random Operations

    /// Returns a random field from the hash
    pub fn hrandfield(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HRANDFIELD")
            .arg(key)
            .query(&mut *conn)
    }

    /// Returns multiple random fields from the hash
    pub fn hrandfield_count(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HRANDFIELD")
            .arg(key)
            .arg(count)
            .query(&mut *conn)
    }

    /// Returns random fields with their values from the hash
    pub fn hrandfield_withvalues(&self, key: &str, count: i64) -> RedisResult<Vec<(String, String)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HRANDFIELD")
            .arg(key)
            .arg(count)
            .arg("WITHVALUES")
            .query(&mut *conn)
    }

    // Scanning

    /// Scans the hash for fields and values matching a pattern
    pub fn hscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<u64>) -> RedisResult<(u64, Vec<(String, String)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("HSCAN");
        cmd.arg(key).arg(cursor);
        
        if let Some(pattern) = pattern {
            cmd.arg("MATCH").arg(pattern);
        }
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        cmd.query(&mut *conn)
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for hash operations
    pub fn with_pipeline<F, T>(&self, f: F) -> RedisResult<T>
    where
        F: FnOnce(&mut Pipeline) -> &mut Pipeline,
        T: FromRedisValue,
    {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        let result = f(&mut pipe).query(&mut *conn)?;
        Ok(result)
    }

    /// Batch set operations using pipeline
    pub fn hset_many(&self, operations: Vec<(&str, &[(&str, &str)])>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, items) in operations {
                for (field, value) in items {
                    pipe.cmd("HSET").arg(key).arg(field).arg(value);
                }
            }
            pipe
        })
    }

    /// Batch get operations using pipeline
    pub fn hget_many(&self, operations: Vec<(&str, &str)>) -> RedisResult<Vec<Option<String>>> {
        self.with_pipeline(|pipe| {
            for (key, field) in operations {
                pipe.cmd("HGET").arg(key).arg(field);
            }
            pipe
        })
    }

    /// Batch delete operations using pipeline
    pub fn hdel_many(&self, operations: Vec<(&str, &[&str])>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, fields) in operations {
                pipe.cmd("HDEL").arg(key).arg(fields);
            }
            pipe
        })
    }

    /// Batch exists checks using pipeline
    pub fn hexists_many(&self, operations: Vec<(&str, &str)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (key, field) in operations {
                pipe.cmd("HEXISTS").arg(key).arg(field);
            }
            pipe
        })
    }

    /// Batch length checks using pipeline
    pub fn hlen_many(&self, keys: Vec<&str>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("HLEN").arg(key);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if hash is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let length = self.hlen(key)?;
        Ok(length == 0)
    }

    /// Clear the hash (remove all fields)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Update multiple fields atomically using HMSET
    pub fn hmset(&self, key: &str, items: &[(&str, &str)]) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HMSET")
            .arg(key)
            .arg(items)
            .query(&mut *conn)
    }

    /// Copy all fields from one hash to another
    pub fn copy_hash(&self, source: &str, destination: &str) -> RedisResult<i64> {
        let fields_and_values = self.hgetall(source)?;
        if fields_and_values.is_empty() {
            return Ok(0);
        }
        
        let items: Vec<(&str, &str)> = fields_and_values
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        
        self.hset_multiple(destination, &items)
    }

    /// Get fields that match a pattern (using HSCAN internally)
    pub fn get_fields_by_pattern(&self, key: &str, pattern: &str) -> RedisResult<HashMap<String, String>> {
        let mut result = HashMap::new();
        let mut cursor = 0;
        
        loop {
            let (next_cursor, fields): (u64, Vec<(String, String)>) = 
                self.hscan(key, cursor, Some(pattern), Some(100))?;
            
            for (field, value) in fields {
                result.insert(field, value);
            }
            
            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }
        
        Ok(result)
    }

    /// Get all fields with a specific prefix
    pub fn get_fields_with_prefix(&self, key: &str, prefix: &str) -> RedisResult<HashMap<String, String>> {
        let pattern = format!("{}*", prefix);
        self.get_fields_by_pattern(key, &pattern)
    }

    /// Get multiple field values as a HashMap
    pub fn hmget_as_map(&self, key: &str, fields: &[&str]) -> RedisResult<HashMap<String, Option<String>>> {
        let values = self.hmget(key, fields)?;
        let mut result = HashMap::new();
        
        for (i, field) in fields.iter().enumerate() {
            if let Some(value) = values.get(i) {
                result.insert(field.to_string(), value.clone());
            }
        }
        
        Ok(result)
    }

    /// Set expiration time for specific fields (Redis 7.0+)
    pub fn hexpire(&self, key: &str, seconds: u64, fields: &[&str]) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HEXPIRE")
            .arg(key)
            .arg(seconds)
            .arg("FIELDS")
            .arg(fields.len())
            .arg(fields)
            .query(&mut *conn)
    }

    /// Set expiration time for specific fields at a Unix timestamp (Redis 7.0+)
    pub fn hexpireat(&self, key: &str, timestamp: u64, fields: &[&str]) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HEXPIREAT")
            .arg(key)
            .arg(timestamp)
            .arg("FIELDS")
            .arg(fields.len())
            .arg(fields)
            .query(&mut *conn)
    }

    /// Get TTL for specific fields (Redis 7.0+)
    pub fn httl(&self, key: &str, fields: &[&str]) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HTTL")
            .arg(key)
            .arg("FIELDS")
            .arg(fields.len())
            .arg(fields)
            .query(&mut *conn)
    }

    /// Remove expiration from specific fields (Redis 7.0+)
    pub fn hpersist(&self, key: &str, fields: &[&str]) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HPERSIST")
            .arg(key)
            .arg("FIELDS")
            .arg(fields.len())
            .arg(fields)
            .query(&mut *conn)
    }

    /// Get and delete a field value atomically
    pub fn hgetdel(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("HGETDEL")
            .arg(key)
            .arg(field)
            .query(&mut *conn)
    }

    /// Merge another hash into this hash
    pub fn merge_hash(&self, destination: &str, source: &str, overwrite: bool) -> RedisResult<i64> {
        let source_data = self.hgetall(source)?;
        let mut count = 0;
        
        for (field, value) in source_data {
            if overwrite {
                self.hset(destination, &field, &value)?;
                count += 1;
            } else {
                if self.hsetnx(destination, &field, &value)? {
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }

    /// Get fields and values where the field contains a substring
    pub fn get_fields_containing(&self, key: &str, substring: &str) -> RedisResult<HashMap<String, String>> {
        let all_fields = self.hgetall(key)?;
        let mut result = HashMap::new();
        
        for (field, value) in all_fields {
            if field.contains(substring) {
                result.insert(field, value);
            }
        }
        
        Ok(result)
    }

    /// Increment multiple fields atomically
    pub fn hincrby_many(&self, key: &str, increments: &[(&str, i64)]) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (field, increment) in increments {
                pipe.cmd("HINCRBY").arg(key).arg(field).arg(increment);
            }
            pipe
        })
    }

    /// Get field count matching a pattern
    pub fn count_fields_by_pattern(&self, key: &str, pattern: &str) -> RedisResult<usize> {
        let fields = self.get_fields_by_pattern(key, pattern)?;
        Ok(fields.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn create_test_connection() -> Arc<Mutex<redis::Connection>> {
        let client = redis::Client::open("redis://127.0.0.1/").expect("Creating test client");
        match client.get_connection() {
            Ok(conn) => Arc::new(Mutex::new(conn)),
            Err(_) => {
                panic!("This test requires a Redis server running locally")
            }
        }
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_basic_hash_operations() {
        let conn = create_test_connection();
        let redis_hash = RedisHash::new(conn);

        // Test basic set/get operations
        let _set_result = redis_hash.hset("test_hash", "field1", "value1");
        let _set_result = redis_hash.hset("test_hash", "field2", "value2");
        let _get_result = redis_hash.hget("test_hash", "field1");
        let _exists = redis_hash.hexists("test_hash", "field1");
        let _length = redis_hash.hlen("test_hash");
        let _all_fields = redis_hash.hgetall("test_hash");
        
        // Clean up
        let _ = redis_hash.clear("test_hash");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_hash_increments() {
        let conn = create_test_connection();
        let redis_hash = RedisHash::new(conn);

        // Test increment operations
        let _set_result = redis_hash.hset("test_hash", "counter", "10");
        let _incr_result = redis_hash.hincrby("test_hash", "counter", 5);
        let _float_incr = redis_hash.hincrbyfloat("test_hash", "float_counter", 1.5);
        
        // Clean up
        let _ = redis_hash.clear("test_hash");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_hash_bulk_operations() {
        let conn = create_test_connection();
        let redis_hash = RedisHash::new(conn);

        // Test bulk set operations
        let items = [("field1", "value1"), ("field2", "value2"), ("field3", "value3")];
        let _set_result = redis_hash.hset_multiple("test_hash", &items);
        
        // Test bulk get operations
        let fields = ["field1", "field2"];
        let _get_result = redis_hash.hmget("test_hash", &fields);
        
        // Clean up
        let _ = redis_hash.clear("test_hash");
    }
}