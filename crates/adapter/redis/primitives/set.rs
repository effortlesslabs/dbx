use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Set data type with operations for managing unique, unordered collections.
///
/// This implementation supports:
/// - Basic set operations (add, remove, members, etc.)
/// - Set arithmetic (union, intersection, difference)
/// - Random member selection and popping
/// - Pipelined operations (for efficiency)
/// - Set scanning and pattern matching
#[derive(Clone)]
pub struct RedisSet {
    conn: Arc<Mutex<Connection>>,
}

impl RedisSet {
    /// Creates a new RedisSet instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic Set Operations

    /// Adds one or more members to a set
    pub fn sadd(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sadd(key, member)
    }

    /// Adds multiple members to a set
    pub fn sadd_multiple(&self, key: &str, members: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut total = 0i64;
        for member in members {
            let result: i64 = conn.sadd(key, member)?;
            total += result;
        }
        Ok(total)
    }

    /// Removes one or more members from a set
    pub fn srem(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.srem(key, member)
    }

    /// Removes multiple members from a set
    pub fn srem_multiple(&self, key: &str, members: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut total = 0i64;
        for member in members {
            let result: i64 = conn.srem(key, member)?;
            total += result;
        }
        Ok(total)
    }

    /// Gets all members of a set
    pub fn smembers(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.smembers(key)
    }

    /// Checks if a member exists in a set
    pub fn sismember(&self, key: &str, member: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.sismember(key, member)
    }

    /// Checks if multiple members exist in a set
    pub fn smismember(&self, key: &str, members: Vec<&str>) -> RedisResult<Vec<bool>> {
        let mut conn = self.conn.lock().unwrap();
        let mut results = Vec::new();
        for member in members {
            let exists: bool = conn.sismember(key, member)?;
            results.push(exists);
        }
        Ok(results)
    }

    /// Gets the cardinality (number of members) of a set
    pub fn scard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.scard(key)
    }

    /// Gets a random member from a set
    pub fn srandmember(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.srandmember(key)
    }

    /// Gets multiple random members from a set
    pub fn srandmember_multiple(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.srandmember_multiple(key, count as usize)
    }

    /// Removes and returns a random member from a set
    pub fn spop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.spop(key)
    }

    /// Removes and returns multiple random members from a set
    pub fn spop_count(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        // Use pipeline to call spop multiple times
        let mut results = Vec::new();
        for _ in 0..count {
            if let Some(member) = conn.spop(key)? {
                results.push(member);
            } else {
                break;
            }
        }
        Ok(results)
    }

    // Set Arithmetic Operations

    /// Computes the union of multiple sets
    pub fn sunion(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sunion(keys)
    }

    /// Computes the union of multiple sets and stores the result in a destination key
    pub fn sunionstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sunionstore(destination, keys)
    }

    /// Computes the intersection of multiple sets
    pub fn sinter(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sinter(keys)
    }

    /// Computes the intersection of multiple sets and stores the result in a destination key
    pub fn sinterstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sinterstore(destination, keys)
    }

    /// Computes the difference between the first set and all successive sets
    pub fn sdiff(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sdiff(keys)
    }

    /// Computes the difference between sets and stores the result in a destination key
    pub fn sdiffstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sdiffstore(destination, keys)
    }

    /// Counts the number of members in the intersection of multiple sets (Redis 7.0+)
    pub fn sintercard(&self, keys: Vec<&str>, limit: Option<i64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for newer Redis commands
        let mut cmd = redis::cmd("SINTERCARD");
        cmd.arg(keys.len()).arg(keys);
        if let Some(l) = limit {
            cmd.arg("LIMIT").arg(l);
        }
        cmd.query(&mut *conn)
    }

    // Advanced Operations

    /// Moves a member from one set to another
    pub fn smove(&self, source: &str, destination: &str, member: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.smove(source, destination, member)
    }

    // Scanning Operations

    /// Scans a set with optional pattern matching
    pub fn sscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<usize>) -> RedisResult<(u64, Vec<String>)> {
        let mut conn = self.conn.lock().unwrap();
        
        if let Some(pat) = pattern {
            let mut iter = conn.sscan_match(key, pat)?;
            let members: Vec<String> = iter.collect();
            Ok((0, members)) // For simplicity, return 0 as cursor since we collected all
        } else {
            let mut iter = conn.sscan(key)?;
            let members: Vec<String> = iter.collect();
            Ok((0, members)) // For simplicity, return 0 as cursor since we collected all
        }
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for set operations
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

    /// Batch add operations using pipeline
    pub fn sadd_many(&self, operations: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, members) in operations {
                for member in members {
                    pipe.sadd(key, member).ignore();
                }
            }
            pipe
        })
    }

    /// Batch remove operations using pipeline
    pub fn srem_many(&self, operations: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, members) in operations {
                for member in members {
                    pipe.srem(key, member).ignore();
                }
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if set is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let count = self.scard(key)?;
        Ok(count == 0)
    }

    /// Clear the set (remove all members)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Check if one set is a subset of another
    pub fn is_subset(&self, subset_key: &str, superset_key: &str) -> RedisResult<bool> {
        let subset_members = self.smembers(subset_key)?;
        for member in subset_members {
            if !self.sismember(superset_key, &member)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Check if one set is a superset of another
    pub fn is_superset(&self, superset_key: &str, subset_key: &str) -> RedisResult<bool> {
        self.is_subset(subset_key, superset_key)
    }

    /// Get symmetric difference between two sets
    pub fn symmetric_difference(&self, key1: &str, key2: &str) -> RedisResult<Vec<String>> {
        // Get elements in key1 but not in key2
        let diff1 = self.sdiff(vec![key1, key2])?;
        // Get elements in key2 but not in key1  
        let diff2 = self.sdiff(vec![key2, key1])?;
        
        // Combine the results
        let mut result = diff1;
        result.extend(diff2);
        Ok(result)
    }

    /// Copy a set to another key
    pub fn copy_set(&self, source: &str, destination: &str) -> RedisResult<()> {
        let members = self.smembers(source)?;
        self.sadd_multiple(destination, members.iter().map(|s| s.as_str()).collect())
            .map(|_| ())
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
    fn test_basic_set_operations() {
        let conn = create_test_connection();
        let redis_set = RedisSet::new(conn);

        // Test basic add/remove operations
        let _add_result = redis_set.sadd("test_set", "member1");
        let _add_result = redis_set.sadd("test_set", "member2");
        let _is_member = redis_set.sismember("test_set", "member1");
        let _cardinality = redis_set.scard("test_set");
        let _members = redis_set.smembers("test_set");
        
        // Clean up
        let _ = redis_set.clear("test_set");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_set_operations() {
        let conn = create_test_connection();
        let redis_set = RedisSet::new(conn);

        // Setup test sets
        let _add_result = redis_set.sadd_multiple("set1", vec!["a", "b", "c"]);
        let _add_result = redis_set.sadd_multiple("set2", vec!["b", "c", "d"]);
        
        // Test union operation
        let _union = redis_set.sunion(vec!["set1", "set2"]);
        
        // Test intersection operation
        let _intersection = redis_set.sinter(vec!["set1", "set2"]);
        
        // Test difference operation
        let _difference = redis_set.sdiff(vec!["set1", "set2"]);
        
        // Clean up
        let _ = redis_set.clear("set1");
        let _ = redis_set.clear("set2");
    }
}