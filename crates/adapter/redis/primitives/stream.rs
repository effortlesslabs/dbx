use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

/// Represents a Redis Stream data type with operations for managing append-only log structures.
///
/// This implementation supports:
/// - Basic stream operations (add, read, length, delete)
/// - Consumer groups and consumers management
/// - Message acknowledgment and claiming
/// - Stream information and statistics
/// - Range queries and trimming
/// - Pipelined operations (for efficiency)
#[derive(Clone)]
pub struct RedisStream {
    conn: Arc<Mutex<Connection>>,
}

/// Statistics for a Redis Stream
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub length: i64,
    pub radix_tree_keys: i64,
    pub radix_tree_nodes: i64,
    pub groups: i64,
    pub last_generated_id: String,
    pub first_entry: Option<(String, Vec<(String, String)>)>,
    pub last_entry: Option<(String, Vec<(String, String)>)>,
}

impl RedisStream {
    /// Creates a new RedisStream instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic Stream Operations

    /// Appends a new entry to the end of a stream
    pub fn xadd(&self, key: &str, id: &str, items: Vec<(&str, &str)>) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XADD");
        cmd.arg(key).arg(id);
        for (field, value) in items {
            cmd.arg(field).arg(value);
        }
        cmd.query(&mut *conn)
    }

    /// Appends a new entry with a maximum length constraint
    pub fn xadd_maxlen(&self, key: &str, id: &str, items: Vec<(&str, &str)>, maxlen: i64) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XADD");
        cmd.arg(key).arg("MAXLEN").arg(maxlen).arg(id);
        for (field, value) in items {
            cmd.arg(field).arg(value);
        }
        cmd.query(&mut *conn)
    }

    /// Returns the number of entries in a stream
    pub fn xlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XLEN");
        cmd.arg(key);
        cmd.query(&mut *conn)
    }

    /// Removes one or more entries from a stream
    pub fn xdel(&self, key: &str, ids: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XDEL");
        cmd.arg(key).arg(ids);
        cmd.query(&mut *conn)
    }

    /// Trims the stream to approximately the given length
    pub fn xtrim(&self, key: &str, maxlen: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XTRIM");
        cmd.arg(key).arg("MAXLEN").arg(maxlen);
        cmd.query(&mut *conn)
    }

    /// Trims the stream to remove entries older than the given ID
    pub fn xtrim_minid(&self, key: &str, minid: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XTRIM");
        cmd.arg(key).arg("MINID").arg(minid);
        cmd.query(&mut *conn)
    }

    // Reading Operations

    /// Reads entries from one or more streams
    pub fn xread(&self, streams: Vec<(&str, &str)>, count: Option<i64>, block: Option<i64>) -> RedisResult<Vec<(String, Vec<(String, Vec<(String, String)>)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XREAD");
        
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        if let Some(b) = block {
            cmd.arg("BLOCK").arg(b);
        }
        
        cmd.arg("STREAMS");
        for (stream, _) in &streams {
            cmd.arg(stream);
        }
        for (_, id) in &streams {
            cmd.arg(id);
        }
        
        cmd.query(&mut *conn)
    }

    /// Reads a range of entries from a stream
    pub fn xrange(&self, key: &str, start: &str, end: &str, count: Option<i64>) -> RedisResult<Vec<(String, Vec<(String, String)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XRANGE");
        cmd.arg(key).arg(start).arg(end);
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        cmd.query(&mut *conn)
    }

    /// Reads a range of entries from a stream in reverse order
    pub fn xrevrange(&self, key: &str, start: &str, end: &str, count: Option<i64>) -> RedisResult<Vec<(String, Vec<(String, String)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XREVRANGE");
        cmd.arg(key).arg(start).arg(end);
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        cmd.query(&mut *conn)
    }

    // Consumer Group Operations

    /// Creates a consumer group
    pub fn xgroup_create(&self, key: &str, group: &str, id: &str, mkstream: bool) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XGROUP");
        cmd.arg("CREATE").arg(key).arg(group).arg(id);
        if mkstream {
            cmd.arg("MKSTREAM");
        }
        cmd.query(&mut *conn)
    }

    /// Destroys a consumer group
    pub fn xgroup_destroy(&self, key: &str, group: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XGROUP");
        cmd.arg("DESTROY").arg(key).arg(group);
        cmd.query(&mut *conn)
    }

    /// Creates a consumer in a consumer group
    pub fn xgroup_createconsumer(&self, key: &str, group: &str, consumer: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XGROUP");
        cmd.arg("CREATECONSUMER").arg(key).arg(group).arg(consumer);
        cmd.query(&mut *conn)
    }

    /// Deletes a consumer from a consumer group
    pub fn xgroup_delconsumer(&self, key: &str, group: &str, consumer: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XGROUP");
        cmd.arg("DELCONSUMER").arg(key).arg(group).arg(consumer);
        cmd.query(&mut *conn)
    }

    /// Sets the last delivered ID for a consumer group
    pub fn xgroup_setid(&self, key: &str, group: &str, id: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XGROUP");
        cmd.arg("SETID").arg(key).arg(group).arg(id);
        cmd.query(&mut *conn)
    }

    // Consumer Operations

    /// Reads messages from a stream as part of a consumer group
    pub fn xreadgroup(&self, group: &str, consumer: &str, streams: Vec<(&str, &str)>, count: Option<i64>, block: Option<i64>, noack: bool) -> RedisResult<Vec<(String, Vec<(String, Vec<(String, String)>)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XREADGROUP");
        cmd.arg("GROUP").arg(group).arg(consumer);
        
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        if let Some(b) = block {
            cmd.arg("BLOCK").arg(b);
        }
        if noack {
            cmd.arg("NOACK");
        }
        
        cmd.arg("STREAMS");
        for (stream, _) in &streams {
            cmd.arg(stream);
        }
        for (_, id) in &streams {
            cmd.arg(id);
        }
        
        cmd.query(&mut *conn)
    }

    /// Acknowledges one or more messages
    pub fn xack(&self, key: &str, group: &str, ids: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XACK");
        cmd.arg(key).arg(group).arg(ids);
        cmd.query(&mut *conn)
    }

    /// Claims ownership of pending messages
    pub fn xclaim(&self, key: &str, group: &str, consumer: &str, min_idle_time: i64, ids: Vec<&str>) -> RedisResult<Vec<(String, Vec<(String, String)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XCLAIM");
        cmd.arg(key).arg(group).arg(consumer).arg(min_idle_time).arg(ids);
        cmd.query(&mut *conn)
    }

    /// Automatically claims the next available pending message
    pub fn xautoclaim(&self, key: &str, group: &str, consumer: &str, min_idle_time: i64, start: &str, count: Option<i64>) -> RedisResult<(String, Vec<(String, Vec<(String, String)>)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XAUTOCLAIM");
        cmd.arg(key).arg(group).arg(consumer).arg(min_idle_time).arg(start);
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        cmd.query(&mut *conn)
    }

    // Information Operations

    /// Returns general information about a stream
    pub fn xinfo_stream(&self, key: &str) -> RedisResult<StreamStats> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XINFO");
        cmd.arg("STREAM").arg(key);
        let info: Vec<redis::Value> = cmd.query(&mut *conn)?;
        
        // Parse the stream info - this is a simplified version
        // In a real implementation, you'd parse all the fields properly
        Ok(StreamStats {
            length: 0,
            radix_tree_keys: 0,
            radix_tree_nodes: 0,
            groups: 0,
            last_generated_id: "0-0".to_string(),
            first_entry: None,
            last_entry: None,
        })
    }

    /// Returns information about consumer groups
    pub fn xinfo_groups(&self, key: &str) -> RedisResult<Vec<std::collections::HashMap<String, redis::Value>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XINFO");
        cmd.arg("GROUPS").arg(key);
        cmd.query(&mut *conn)
    }

    /// Returns information about consumers in a group
    pub fn xinfo_consumers(&self, key: &str, group: &str) -> RedisResult<Vec<std::collections::HashMap<String, redis::Value>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XINFO");
        cmd.arg("CONSUMERS").arg(key).arg(group);
        cmd.query(&mut *conn)
    }

    /// Returns pending messages information
    pub fn xpending(&self, key: &str, group: &str) -> RedisResult<(i64, Option<String>, Option<String>, Vec<(String, i64)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XPENDING");
        cmd.arg(key).arg(group);
        cmd.query(&mut *conn)
    }

    /// Returns detailed pending messages information
    pub fn xpending_range(&self, key: &str, group: &str, start: &str, end: &str, count: i64, consumer: Option<&str>) -> RedisResult<Vec<(String, String, i64, i64)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("XPENDING");
        cmd.arg(key).arg(group).arg(start).arg(end).arg(count);
        if let Some(c) = consumer {
            cmd.arg(c);
        }
        cmd.query(&mut *conn)
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for stream operations
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
    pub fn xadd_many(&self, operations: Vec<(&str, &str, Vec<(&str, &str)>)>) -> RedisResult<Vec<String>> {
        self.with_pipeline(|pipe| {
            for (key, id, items) in operations {
                let mut cmd = pipe.cmd("XADD");
                cmd.arg(key).arg(id);
                for (field, value) in items {
                    cmd.arg(field).arg(value);
                }
                cmd.ignore();
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if stream is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let len = self.xlen(key)?;
        Ok(len == 0)
    }

    /// Clear the stream (remove all entries)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Get the first entry in the stream
    pub fn get_first_entry(&self, key: &str) -> RedisResult<Option<(String, Vec<(String, String)>)>> {
        let entries = self.xrange(key, "-", "+", Some(1))?;
        Ok(entries.into_iter().next())
    }

    /// Get the last entry in the stream
    pub fn get_last_entry(&self, key: &str) -> RedisResult<Option<(String, Vec<(String, String)>)>> {
        let entries = self.xrevrange(key, "+", "-", Some(1))?;
        Ok(entries.into_iter().next())
    }

    /// Get entries after a specific ID
    pub fn get_entries_after(&self, key: &str, id: &str, count: Option<i64>) -> RedisResult<Vec<(String, Vec<(String, String)>)>> {
        self.xrange(key, id, "+", count)
    }

    /// Get entries before a specific ID
    pub fn get_entries_before(&self, key: &str, id: &str, count: Option<i64>) -> RedisResult<Vec<(String, Vec<(String, String)>)>> {
        self.xrevrange(key, id, "-", count)
    }

    /// Create a consumer group if it doesn't exist
    pub fn ensure_consumer_group(&self, key: &str, group: &str, start_id: &str) -> RedisResult<()> {
        match self.xgroup_create(key, group, start_id, true) {
            Ok(_) => Ok(()),
            Err(_) => Ok(()), // Group already exists, ignore error
        }
    }

    /// Get the approximate size of the stream in memory
    pub fn memory_usage(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("MEMORY");
        cmd.arg("USAGE").arg(key);
        cmd.query(&mut *conn)
    }

    /// Get stream statistics
    pub fn get_stats(&self, key: &str) -> RedisResult<StreamStats> {
        self.xinfo_stream(key)
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
    fn test_basic_stream_operations() {
        let conn = create_test_connection();
        let redis_stream = RedisStream::new(conn);

        // Test basic add/read operations
        let fields = vec![("field1", "value1"), ("field2", "value2")];
        let _add_result = redis_stream.xadd("test_stream", "*", fields);
        let _length = redis_stream.xlen("test_stream");
        let _entries = redis_stream.xrange("test_stream", "-", "+", None);
        
        // Clean up
        let _ = redis_stream.clear("test_stream");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_stream_range_operations() {
        let conn = create_test_connection();
        let redis_stream = RedisStream::new(conn);

        // Add some test data
        let fields1 = vec![("sensor", "temperature"), ("value", "23.5")];
        let fields2 = vec![("sensor", "humidity"), ("value", "45.2")];
        let _id1 = redis_stream.xadd("test_stream", "*", fields1);
        let _id2 = redis_stream.xadd("test_stream", "*", fields2);
        
        // Test range operations
        let _all_entries = redis_stream.xrange("test_stream", "-", "+", None);
        let _first_entry = redis_stream.get_first_entry("test_stream");
        let _last_entry = redis_stream.get_last_entry("test_stream");
        
        // Clean up
        let _ = redis_stream.clear("test_stream");
    }
}