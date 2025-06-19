use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Set data type with operations for manipulating unordered collections of unique elements.
///
/// This implementation supports:
/// - Basic set operations (add, remove, members, etc.)
/// - Set operations (union, intersection, difference)
/// - Random member selection
/// - Pipelined operations (for efficiency)
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
        conn.sadd_multiple(key, &members)
    }

    /// Removes one or more members from a set
    pub fn srem(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.srem(key, member)
    }

    /// Removes multiple members from a set
    pub fn srem_multiple(&self, key: &str, members: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.srem_multiple(key, &members)
    }

    /// Returns all members of a set
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
        conn.cmd("SMISMEMBER")
            .arg(key)
            .arg(&members)
            .query(&mut *conn)
    }

    /// Returns the number of members in a set
    pub fn scard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.scard(key)
    }

    /// Returns a random member from the set
    pub fn srandmember(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.srandmember(key)
    }

    /// Returns multiple random members from the set
    pub fn srandmember_multiple(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.srandmember_multiple(key, count)
    }

    /// Removes and returns a random member from the set
    pub fn spop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.spop(key)
    }

    /// Removes and returns multiple random members from the set
    pub fn spop_multiple(&self, key: &str, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("SPOP")
            .arg(key)
            .arg(count)
            .query(&mut *conn)
    }

    /// Moves a member from one set to another
    pub fn smove(&self, source: &str, destination: &str, member: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        conn.smove(source, destination, member)
    }

    // Set Operations

    /// Returns the union of multiple sets
    pub fn sunion(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sunion(&keys)
    }

    /// Stores the union of multiple sets in a destination key
    pub fn sunionstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sunionstore(destination, &keys)
    }

    /// Returns the intersection of multiple sets
    pub fn sinter(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sinter(&keys)
    }

    /// Stores the intersection of multiple sets in a destination key
    pub fn sinterstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sinterstore(destination, &keys)
    }

    /// Returns the cardinality of the intersection of multiple sets
    pub fn sintercard(&self, keys: Vec<&str>, limit: Option<i64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("SINTERCARD");
        cmd.arg(keys.len()).arg(&keys);
        if let Some(limit) = limit {
            cmd.arg("LIMIT").arg(limit);
        }
        cmd.query(&mut *conn)
    }

    /// Returns the difference between sets
    pub fn sdiff(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.sdiff(&keys)
    }

    /// Stores the difference between sets in a destination key
    pub fn sdiffstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.sdiffstore(destination, &keys)
    }

    // Scanning

    /// Scans the set for members matching a pattern
    pub fn sscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<u64>) -> RedisResult<(u64, Vec<String>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("SSCAN");
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
                pipe.cmd("SADD").arg(key).arg(&members);
            }
            pipe
        })
    }

    /// Batch remove operations using pipeline
    pub fn srem_many(&self, operations: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, members) in operations {
                pipe.cmd("SREM").arg(key).arg(&members);
            }
            pipe
        })
    }

    /// Batch membership checks using pipeline
    pub fn sismember_many(&self, operations: Vec<(&str, &str)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (key, member) in operations {
                pipe.cmd("SISMEMBER").arg(key).arg(member);
            }
            pipe
        })
    }

    /// Batch cardinality checks using pipeline
    pub fn scard_many(&self, keys: Vec<&str>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("SCARD").arg(key);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if set is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let cardinality = self.scard(key)?;
        Ok(cardinality == 0)
    }

    /// Clear the set (remove all members)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Get all members as a sorted vector
    pub fn get_sorted_members(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut members = self.smembers(key)?;
        members.sort();
        Ok(members)
    }

    /// Check if two sets are equal
    pub fn sets_equal(&self, key1: &str, key2: &str) -> RedisResult<bool> {
        let diff1 = self.sdiff(vec![key1, key2])?;
        let diff2 = self.sdiff(vec![key2, key1])?;
        Ok(diff1.is_empty() && diff2.is_empty())
    }

    /// Get a subset of random members
    pub fn get_random_subset(&self, key: &str, size: i64) -> RedisResult<Vec<String>> {
        self.srandmember_multiple(key, size)
    }

    /// Copy all members from one set to another
    pub fn copy_set(&self, source: &str, destination: &str) -> RedisResult<i64> {
        let members = self.smembers(source)?;
        if members.is_empty() {
            return Ok(0);
        }
        
        let member_refs: Vec<&str> = members.iter().map(|s| s.as_str()).collect();
        self.sadd_multiple(destination, member_refs)
    }

    /// Check if set1 is a subset of set2
    pub fn is_subset(&self, set1: &str, set2: &str) -> RedisResult<bool> {
        let diff = self.sdiff(vec![set1, set2])?;
        Ok(diff.is_empty())
    }

    /// Check if set1 is a superset of set2
    pub fn is_superset(&self, set1: &str, set2: &str) -> RedisResult<bool> {
        self.is_subset(set2, set1)
    }

    /// Get the symmetric difference of two sets
    pub fn symmetric_difference(&self, key1: &str, key2: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        
        // Get key1 - key2 and key2 - key1, then union them
        pipe.cmd("SDIFF").arg(key1).arg(key2);
        pipe.cmd("SDIFF").arg(key2).arg(key1);
        
        let results: (Vec<String>, Vec<String>) = pipe.query(&mut *conn)?;
        
        let mut symmetric_diff = results.0;
        symmetric_diff.extend(results.1);
        
        Ok(symmetric_diff)
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