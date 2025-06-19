use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

/// Represents a Redis Hash data type with operations for managing field-value maps.
///
/// This implementation supports:
/// - Basic hash operations (set, get, delete, exists)
/// - Multi-field operations (mset, mget, getall)
/// - Increment operations (numeric and float)
/// - Field expiration (Redis 7.0+ features)
/// - Pipelined operations (for efficiency)
/// - Random field selection
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

    /// Sets field in the hash stored at key to value
    pub fn hset(&self, key: &str, field: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset(key, field, value)
    }

    /// Sets multiple field-value pairs in the hash stored at key
    pub fn hset_multiple(&self, key: &str, items: Vec<(&str, &str)>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset_multiple(key, &items)
    }

    /// Gets the value of field in the hash stored at key
    pub fn hget(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hget(key, field)
    }

    /// Gets the values of multiple fields in the hash stored at key
    pub fn hget_multiple(&self, key: &str, fields: Vec<&str>) -> RedisResult<Vec<Option<String>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for field in fields {
            let value: Option<String> = conn.hget(key, field)?;
            results.push(value);
        }
        Ok(results)
    }

    /// Gets all fields and values in a hash
    pub fn hgetall(&self, key: &str) -> RedisResult<std::collections::HashMap<String, String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hgetall(key)
    }

    /// Deletes one or more hash fields
    pub fn hdel(&self, key: &str, field: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel(key, field)
    }

    /// Deletes multiple hash fields
    pub fn hdel_multiple(&self, key: &str, fields: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut total = 0i64;
        for field in fields {
            let result: i64 = conn.hdel(key, field)?;
            total += result;
        }
        Ok(total)
    }

    /// Determines if a hash field exists
    pub fn hexists(&self, key: &str, field: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.hexists(key, field)
    }

    /// Gets the number of fields in a hash
    pub fn hlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hlen(key)
    }

    /// Gets all the fields in a hash
    pub fn hkeys(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hkeys(key)
    }

    /// Gets all the values in a hash
    pub fn hvals(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hvals(key)
    }

    // Increment Operations

    /// Increments the number stored at field in the hash stored at key by increment
    pub fn hincrby(&self, key: &str, field: &str, increment: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hincr(key, field, increment)
    }

    /// Increments the float number stored at field in the hash stored at key by increment
    pub fn hincrbyfloat(&self, key: &str, field: &str, increment: f64) -> RedisResult<f64> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for float increment
        let mut cmd = redis::cmd("HINCRBYFLOAT");
        cmd.arg(key).arg(field).arg(increment);
        cmd.query(&mut *conn)
    }

    // Conditional Set Operations

    /// Sets field in the hash stored at key to value, only if field does not yet exist
    pub fn hsetnx(&self, key: &str, field: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hsetnx
        let mut cmd = redis::cmd("HSETNX");
        cmd.arg(key).arg(field).arg(value);
        let result: i64 = cmd.query(&mut *conn)?;
        Ok(result == 1)
    }

    // String Operations

    /// Gets the string length of the value associated with field in the hash stored at key
    pub fn hstrlen(&self, key: &str, field: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hstrlen
        let mut cmd = redis::cmd("HSTRLEN");
        cmd.arg(key).arg(field);
        cmd.query(&mut *conn)
    }

    // Random Field Operations

    /// Returns a random field from the hash value stored at key
    pub fn hrandfield(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hrandfield
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key);
        cmd.query(&mut *conn)
    }

    /// Returns multiple random fields from the hash value stored at key
    pub fn hrandfield_count(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hrandfield with count
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key).arg(count);
        cmd.query(&mut *conn)
    }

    /// Returns random field-value pairs from the hash value stored at key
    pub fn hrandfield_withvalues(&self, key: &str, count: i64) -> RedisResult<Vec<(String, String)>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hrandfield with withvalues
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key).arg(count).arg("WITHVALUES");
        cmd.query(&mut *conn)
    }

    // Advanced Operations

    /// Returns the value associated with field and removes it from the hash stored at key
    pub fn hgetdel(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for hgetdel
        let mut cmd = redis::cmd("HGETDEL");
        cmd.arg(key).arg(field);
        cmd.query(&mut *conn)
    }

    // Expiration Operations (Redis 7.0+)

    /// Set an expiry (TTL or absolute time) on one or more hash fields
    pub fn hexpire(&self, key: &str, seconds: i64, fields: Vec<&str>) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for field in fields {
            // Simulate hexpire using individual commands since it may not be available
            let mut cmd = redis::cmd("HEXPIRE");
            cmd.arg(key).arg(seconds).arg("FIELDS").arg(1).arg(field);
            let result: i64 = cmd.query(&mut *conn)?;
            results.push(result);
        }
        Ok(results)
    }

    /// Set an expiry (absolute time) on one or more hash fields
    pub fn hexpireat(&self, key: &str, timestamp: i64, fields: Vec<&str>) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for field in fields {
            // Simulate hexpireat using individual commands since it may not be available
            let mut cmd = redis::cmd("HEXPIREAT");
            cmd.arg(key).arg(timestamp).arg("FIELDS").arg(1).arg(field);
            let result: i64 = cmd.query(&mut *conn)?;
            results.push(result);
        }
        Ok(results)
    }

    /// Returns the remaining TTL (time to live) of hash fields
    pub fn httl(&self, key: &str, fields: Vec<&str>) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for field in fields {
            // Simulate httl using individual commands since it may not be available
            let mut cmd = redis::cmd("HTTL");
            cmd.arg(key).arg("FIELDS").arg(1).arg(field);
            let result: i64 = cmd.query(&mut *conn)?;
            results.push(result);
        }
        Ok(results)
    }

    /// Remove the existing timeout on hash fields
    pub fn hpersist(&self, key: &str, fields: Vec<&str>) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for field in fields {
            // Simulate hpersist using individual commands since it may not be available
            let mut cmd = redis::cmd("HPERSIST");
            cmd.arg(key).arg("FIELDS").arg(1).arg(field);
            let result: i64 = cmd.query(&mut *conn)?;
            results.push(result);
        }
        Ok(results)
    }

    // Scanning Operations

    /// Scans the hash for field-value pairs matching a pattern
    pub fn hscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<usize>) -> RedisResult<(u64, Vec<(String, String)>)> {
        let mut conn = self.conn.lock().unwrap();
        
        if let Some(pat) = pattern {
            let mut iter = conn.hscan_match(key, pat)?;
            let pairs: Vec<(String, String)> = iter.collect();
            Ok((0, pairs)) // For simplicity, return 0 as cursor since we collected all
        } else {
            let mut iter = conn.hscan(key)?;
            let pairs: Vec<(String, String)> = iter.collect();
            Ok((0, pairs)) // For simplicity, return 0 as cursor since we collected all
        }
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
    pub fn hset_many(&self, operations: Vec<(&str, Vec<(&str, &str)>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, fields) in operations {
                for (field, value) in fields {
                    pipe.hset(key, field, value).ignore();
                }
            }
            pipe
        })
    }

    /// Batch get operations using pipeline
    pub fn hget_many(&self, operations: Vec<(&str, &str)>) -> RedisResult<Vec<Option<String>>> {
        self.with_pipeline(|pipe| {
            for (key, field) in operations {
                pipe.hget(key, field);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if hash is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let len = self.hlen(key)?;
        Ok(len == 0)
    }

    /// Clear the hash (remove all fields)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Copy hash fields to another hash
    pub fn copy_hash(&self, source: &str, destination: &str) -> RedisResult<()> {
        let hash_data = self.hgetall(source)?;
        let items: Vec<(&str, &str)> = hash_data.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        self.hset_multiple(destination, items).map(|_| ())
    }

    /// Check if all fields exist in hash
    pub fn fields_exist(&self, key: &str, fields: Vec<&str>) -> RedisResult<bool> {
        for field in fields {
            if !self.hexists(key, field)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Get field count for fields that exist
    pub fn existing_field_count(&self, key: &str, fields: Vec<&str>) -> RedisResult<i64> {
        let mut count = 0i64;
        for field in fields {
            if self.hexists(key, field)? {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Merge two hashes
    pub fn merge_hashes(&self, source: &str, destination: &str, overwrite: bool) -> RedisResult<i64> {
        let source_data = self.hgetall(source)?;
        let mut merged_count = 0i64;
        
        for (field, value) in source_data {
            if overwrite || !self.hexists(destination, &field)? {
                self.hset(destination, &field, &value)?;
                merged_count += 1;
            }
        }
        
        Ok(merged_count)
    }

    /// Get field values that match a pattern
    pub fn get_fields_by_pattern(&self, key: &str, pattern: &str) -> RedisResult<std::collections::HashMap<String, String>> {
        let (_, pairs) = self.hscan(key, 0, Some(pattern), None)?;
        Ok(pairs.into_iter().collect())
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
        let items = vec![("field1", "value1"), ("field2", "value2"), ("field3", "value3")];
        let _set_result = redis_hash.hset_multiple("test_hash", items);
        
        // Test bulk get operations
        let fields = vec!["field1", "field2"];
        let _get_result = redis_hash.hget_multiple("test_hash", fields);
        
        // Clean up
        let _ = redis_hash.clear("test_hash");
    }
}