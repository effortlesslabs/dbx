use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

/// Represents a Redis Stream data type with operations for manipulating append-only logs.
///
/// This implementation supports:
/// - Basic stream operations (add, read, length, etc.)
/// - Consumer group operations
/// - Blocking reads
/// - Trimming operations
/// - Claiming and acknowledgment operations
#[derive(Clone)]
pub struct RedisStream {
    conn: Arc<Mutex<Connection>>,
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

    /// Adds an entry to a stream with auto-generated ID
    pub fn xadd(&self, key: &str, fields: &[(&str, &str)]) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XADD");
        cmd.arg(key).arg("*");
        
        for (field, value) in fields {
            cmd.arg(field).arg(value);
        }
        
        cmd.query(&mut *conn)
    }

    /// Adds an entry to a stream with a specific ID
    pub fn xadd_id(&self, key: &str, id: &str, fields: &[(&str, &str)]) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XADD");
        cmd.arg(key).arg(id);
        
        for (field, value) in fields {
            cmd.arg(field).arg(value);
        }
        
        cmd.query(&mut *conn)
    }

    /// Adds an entry with maximum length limit
    pub fn xadd_maxlen(&self, key: &str, maxlen: i64, fields: &[(&str, &str)]) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XADD");
        cmd.arg(key).arg("MAXLEN").arg("~").arg(maxlen).arg("*");
        
        for (field, value) in fields {
            cmd.arg(field).arg(value);
        }
        
        cmd.query(&mut *conn)
    }

    /// Returns the length of a stream
    pub fn xlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XLEN").arg(key).query(&mut *conn)
    }

    /// Reads entries from one or more streams
    pub fn xread(&self, streams: &[(&str, &str)], count: Option<i64>, block: Option<u64>) -> RedisResult<Vec<(String, Vec<(String, HashMap<String, String>)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XREAD");
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        if let Some(block_ms) = block {
            cmd.arg("BLOCK").arg(block_ms);
        }
        
        cmd.arg("STREAMS");
        
        // Add stream names
        for (stream, _) in streams {
            cmd.arg(stream);
        }
        
        // Add IDs
        for (_, id) in streams {
            cmd.arg(id);
        }
        
        cmd.query(&mut *conn)
    }

    /// Reads a range of entries from a stream
    pub fn xrange(&self, key: &str, start: &str, end: &str, count: Option<i64>) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XRANGE");
        cmd.arg(key).arg(start).arg(end);
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        cmd.query(&mut *conn)
    }

    /// Reads a range of entries from a stream in reverse order
    pub fn xrevrange(&self, key: &str, end: &str, start: &str, count: Option<i64>) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XREVRANGE");
        cmd.arg(key).arg(end).arg(start);
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        cmd.query(&mut *conn)
    }

    /// Deletes entries from a stream
    pub fn xdel(&self, key: &str, ids: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XDEL")
            .arg(key)
            .arg(ids)
            .query(&mut *conn)
    }

    /// Trims a stream to a maximum length
    pub fn xtrim(&self, key: &str, maxlen: i64, approximate: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XTRIM");
        cmd.arg(key).arg("MAXLEN");
        
        if approximate {
            cmd.arg("~");
        }
        
        cmd.arg(maxlen).query(&mut *conn)
    }

    /// Trims a stream to a minimum ID
    pub fn xtrim_minid(&self, key: &str, min_id: &str, approximate: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XTRIM");
        cmd.arg(key).arg("MINID");
        
        if approximate {
            cmd.arg("~");
        }
        
        cmd.arg(min_id).query(&mut *conn)
    }

    // Consumer Group Operations

    /// Creates a consumer group
    pub fn xgroup_create(&self, key: &str, group: &str, id: &str, mkstream: bool) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XGROUP");
        cmd.arg("CREATE").arg(key).arg(group).arg(id);
        
        if mkstream {
            cmd.arg("MKSTREAM");
        }
        
        cmd.query(&mut *conn)
    }

    /// Creates a consumer in a group
    pub fn xgroup_createconsumer(&self, key: &str, group: &str, consumer: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XGROUP")
            .arg("CREATECONSUMER")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .query(&mut *conn)
    }

    /// Deletes a consumer from a group
    pub fn xgroup_delconsumer(&self, key: &str, group: &str, consumer: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XGROUP")
            .arg("DELCONSUMER")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .query(&mut *conn)
    }

    /// Destroys a consumer group
    pub fn xgroup_destroy(&self, key: &str, group: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XGROUP")
            .arg("DESTROY")
            .arg(key)
            .arg(group)
            .query(&mut *conn)
    }

    /// Sets the ID of a consumer group
    pub fn xgroup_setid(&self, key: &str, group: &str, id: &str) -> RedisResult<String> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XGROUP")
            .arg("SETID")
            .arg(key)
            .arg(group)
            .arg(id)
            .query(&mut *conn)
    }

    /// Reads from a stream as part of a consumer group
    pub fn xreadgroup(&self, group: &str, consumer: &str, streams: &[(&str, &str)], count: Option<i64>, block: Option<u64>, noack: bool) -> RedisResult<Vec<(String, Vec<(String, HashMap<String, String>)>)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XREADGROUP");
        cmd.arg("GROUP").arg(group).arg(consumer);
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        if let Some(block_ms) = block {
            cmd.arg("BLOCK").arg(block_ms);
        }
        
        if noack {
            cmd.arg("NOACK");
        }
        
        cmd.arg("STREAMS");
        
        // Add stream names
        for (stream, _) in streams {
            cmd.arg(stream);
        }
        
        // Add IDs
        for (_, id) in streams {
            cmd.arg(id);
        }
        
        cmd.query(&mut *conn)
    }

    /// Acknowledges processed messages
    pub fn xack(&self, key: &str, group: &str, ids: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XACK")
            .arg(key)
            .arg(group)
            .arg(ids)
            .query(&mut *conn)
    }

    /// Claims pending messages
    pub fn xclaim(&self, key: &str, group: &str, consumer: &str, min_idle_time: u64, ids: &[&str]) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XCLAIM")
            .arg(key)
            .arg(group)
            .arg(consumer)
            .arg(min_idle_time)
            .arg(ids)
            .query(&mut *conn)
    }

    /// Auto-claims pending messages
    pub fn xautoclaim(&self, key: &str, group: &str, consumer: &str, min_idle_time: u64, start: &str, count: Option<i64>) -> RedisResult<(String, Vec<(String, HashMap<String, String>)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XAUTOCLAIM");
        cmd.arg(key)
           .arg(group)
           .arg(consumer)
           .arg(min_idle_time)
           .arg(start);
        
        if let Some(count) = count {
            cmd.arg("COUNT").arg(count);
        }
        
        cmd.query(&mut *conn)
    }

    // Pending and Info Operations

    /// Gets information about pending messages
    pub fn xpending(&self, key: &str, group: &str) -> RedisResult<(i64, Option<String>, Option<String>, Vec<(String, i64)>)> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XPENDING")
            .arg(key)
            .arg(group)
            .query(&mut *conn)
    }

    /// Gets detailed information about pending messages
    pub fn xpending_range(&self, key: &str, group: &str, start: &str, end: &str, count: i64, consumer: Option<&str>) -> RedisResult<Vec<(String, String, u64, i64)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("XPENDING");
        cmd.arg(key).arg(group).arg(start).arg(end).arg(count);
        
        if let Some(consumer) = consumer {
            cmd.arg(consumer);
        }
        
        cmd.query(&mut *conn)
    }

    /// Gets information about a stream
    pub fn xinfo_stream(&self, key: &str) -> RedisResult<HashMap<String, redis::Value>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XINFO")
            .arg("STREAM")
            .arg(key)
            .query(&mut *conn)
    }

    /// Gets information about consumer groups
    pub fn xinfo_groups(&self, key: &str) -> RedisResult<Vec<HashMap<String, redis::Value>>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XINFO")
            .arg("GROUPS")
            .arg(key)
            .query(&mut *conn)
    }

    /// Gets information about consumers in a group
    pub fn xinfo_consumers(&self, key: &str, group: &str) -> RedisResult<Vec<HashMap<String, redis::Value>>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("XINFO")
            .arg("CONSUMERS")
            .arg(key)
            .arg(group)
            .query(&mut *conn)
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
    pub fn xadd_many(&self, operations: Vec<(&str, &[(&str, &str)])>) -> RedisResult<Vec<String>> {
        self.with_pipeline(|pipe| {
            for (key, fields) in operations {
                let mut cmd = pipe.cmd("XADD");
                cmd.arg(key).arg("*");
                for (field, value) in fields {
                    cmd.arg(field).arg(value);
                }
            }
            pipe
        })
    }

    /// Batch delete operations using pipeline
    pub fn xdel_many(&self, operations: Vec<(&str, &[&str])>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, ids) in operations {
                pipe.cmd("XDEL").arg(key).arg(ids);
            }
            pipe
        })
    }

    /// Batch length operations using pipeline
    pub fn xlen_many(&self, keys: Vec<&str>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("XLEN").arg(key);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if stream is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let length = self.xlen(key)?;
        Ok(length == 0)
    }

    /// Clear the stream (delete all entries)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Get the first entry in the stream
    pub fn get_first_entry(&self, key: &str) -> RedisResult<Option<(String, HashMap<String, String>)>> {
        let entries = self.xrange(key, "-", "+", Some(1))?;
        Ok(entries.into_iter().next())
    }

    /// Get the last entry in the stream
    pub fn get_last_entry(&self, key: &str) -> RedisResult<Option<(String, HashMap<String, String>)>> {
        let entries = self.xrevrange(key, "+", "-", Some(1))?;
        Ok(entries.into_iter().next())
    }

    /// Get all entries in the stream
    pub fn get_all_entries(&self, key: &str) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        self.xrange(key, "-", "+", None)
    }

    /// Get entries since a specific ID
    pub fn get_entries_since(&self, key: &str, since_id: &str) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        self.xrange(key, since_id, "+", None)
    }

    /// Get entries until a specific ID
    pub fn get_entries_until(&self, key: &str, until_id: &str) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        self.xrange(key, "-", until_id, None)
    }

    /// Get entries between two IDs
    pub fn get_entries_between(&self, key: &str, start_id: &str, end_id: &str, count: Option<i64>) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        self.xrange(key, start_id, end_id, count)
    }

    /// Count entries between two IDs
    pub fn count_entries_between(&self, key: &str, start_id: &str, end_id: &str) -> RedisResult<usize> {
        let entries = self.xrange(key, start_id, end_id, None)?;
        Ok(entries.len())
    }

    /// Check if an entry ID exists in the stream
    pub fn entry_exists(&self, key: &str, id: &str) -> RedisResult<bool> {
        let entries = self.xrange(key, id, id, Some(1))?;
        Ok(!entries.is_empty())
    }

    /// Get stream statistics
    pub fn get_stream_stats(&self, key: &str) -> RedisResult<StreamStats> {
        let info = self.xinfo_stream(key)?;
        let length = self.xlen(key)?;
        
        // Extract stats from the info response
        let first_entry = info.get("first-entry");
        let last_entry = info.get("last-entry");
        
        Ok(StreamStats {
            length,
            first_entry_id: first_entry.and_then(|v| {
                if let redis::Value::Bulk(ref entries) = v {
                    if let Some(redis::Value::Data(ref id_bytes)) = entries.get(0) {
                        String::from_utf8(id_bytes.clone()).ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            last_entry_id: last_entry.and_then(|v| {
                if let redis::Value::Bulk(ref entries) = v {
                    if let Some(redis::Value::Data(ref id_bytes)) = entries.get(0) {
                        String::from_utf8(id_bytes.clone()).ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
        })
    }

    /// Wait for new entries in a stream
    pub fn wait_for_entries(&self, key: &str, last_id: &str, timeout_ms: u64) -> RedisResult<Vec<(String, HashMap<String, String>)>> {
        let streams = vec![(key, last_id)];
        let result = self.xread(&streams, None, Some(timeout_ms))?;
        
        if let Some((_, entries)) = result.into_iter().next() {
            Ok(entries)
        } else {
            Ok(Vec::new())
        }
    }
}

/// Stream statistics structure
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub length: i64,
    pub first_entry_id: Option<String>,
    pub last_entry_id: Option<String>,
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
        let fields = [("field1", "value1"), ("field2", "value2")];
        let _add_result = redis_stream.xadd("test_stream", &fields);
        let _length = redis_stream.xlen("test_stream");
        let _entries = redis_stream.get_all_entries("test_stream");
        
        // Clean up
        let _ = redis_stream.clear("test_stream");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_stream_range_operations() {
        let conn = create_test_connection();
        let redis_stream = RedisStream::new(conn);

        // Add some test data
        let fields1 = [("sensor", "temperature"), ("value", "23.5")];
        let fields2 = [("sensor", "humidity"), ("value", "45.2")];
        let _id1 = redis_stream.xadd("test_stream", &fields1);
        let _id2 = redis_stream.xadd("test_stream", &fields2);
        
        // Test range operations
        let _all_entries = redis_stream.xrange("test_stream", "-", "+", None);
        let _first_entry = redis_stream.get_first_entry("test_stream");
        let _last_entry = redis_stream.get_last_entry("test_stream");
        
        // Clean up
        let _ = redis_stream.clear("test_stream");
    }
}