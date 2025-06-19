use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis List data type with operations for manipulating ordered collections.
///
/// This implementation supports:
/// - Basic list operations (push, pop, range, etc.)
/// - Blocking operations (blpop, brpop)
/// - Pipelined operations (for efficiency)
/// - List manipulation (insert, set, remove)
#[derive(Clone)]
pub struct RedisList {
    conn: Arc<Mutex<Connection>>,
}

impl RedisList {
    /// Creates a new RedisList instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic List Operations

    /// Pushes an element to the head of the list
    pub fn lpush(&self, key: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.lpush(key, value)
    }

    /// Pushes multiple elements to the head of the list
    pub fn lpush_multiple(&self, key: &str, values: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.lpush_multiple(key, &values)
    }

    /// Pushes an element to the tail of the list
    pub fn rpush(&self, key: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.rpush(key, value)
    }

    /// Pushes multiple elements to the tail of the list
    pub fn rpush_multiple(&self, key: &str, values: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.rpush_multiple(key, &values)
    }

    /// Pops an element from the head of the list
    pub fn lpop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.lpop(key, None)
    }

    /// Pops multiple elements from the head of the list
    pub fn lpop_count(&self, key: &str, count: isize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.lpop(key, Some(count))
    }

    /// Pops an element from the tail of the list
    pub fn rpop(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.rpop(key, None)
    }

    /// Pops multiple elements from the tail of the list
    pub fn rpop_count(&self, key: &str, count: isize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.rpop(key, Some(count))
    }

    /// Gets the length of the list
    pub fn llen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.llen(key)
    }

    /// Gets a range of elements from the list
    pub fn lrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.lrange(key, start, stop)
    }

    /// Gets an element by index
    pub fn lindex(&self, key: &str, index: i64) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.lindex(key, index)
    }

    /// Sets the value of an element at a specific index
    pub fn lset(&self, key: &str, index: i64, value: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.lset(key, index, value)
    }

    /// Trims the list to the specified range
    pub fn ltrim(&self, key: &str, start: i64, stop: i64) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.ltrim(key, start, stop)
    }

    /// Inserts an element before or after another element
    pub fn linsert_before(&self, key: &str, pivot: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.linsert_before(key, pivot, value)
    }

    /// Inserts an element after another element
    pub fn linsert_after(&self, key: &str, pivot: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.linsert_after(key, pivot, value)
    }

    /// Removes elements from the list
    pub fn lrem(&self, key: &str, count: i64, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.lrem(key, count, value)
    }

    // Blocking Operations

    /// Blocking pop from the head of the list
    pub fn blpop(&self, keys: Vec<&str>, timeout: usize) -> RedisResult<Option<(String, String)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.blpop(&keys, timeout)
    }

    /// Blocking pop from the tail of the list
    pub fn brpop(&self, keys: Vec<&str>, timeout: usize) -> RedisResult<Option<(String, String)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.brpop(&keys, timeout)
    }

    /// Blocking pop from the tail of source and push to head of destination
    pub fn brpoplpush(&self, source: &str, destination: &str, timeout: usize) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.brpoplpush(source, destination, timeout)
    }

    // Advanced Operations

    /// Pops from the tail of source and pushes to head of destination
    pub fn rpoplpush(&self, source: &str, destination: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.rpoplpush(source, destination)
    }

    /// Moves an element from one list to another
    pub fn lmove(&self, source: &str, destination: &str, src_dir: &str, dest_dir: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("LMOVE")
            .arg(source)
            .arg(destination)
            .arg(src_dir)
            .arg(dest_dir)
            .query(&mut *conn)
    }

    /// Blocking move operation
    pub fn blmove(&self, source: &str, destination: &str, src_dir: &str, dest_dir: &str, timeout: f64) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BLMOVE")
            .arg(source)
            .arg(destination)
            .arg(src_dir)
            .arg(dest_dir)
            .arg(timeout)
            .query(&mut *conn)
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for list operations
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

    /// Batch push operations using pipeline
    pub fn lpush_many(&self, operations: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, values) in operations {
                for value in values {
                    pipe.cmd("LPUSH").arg(key).arg(value);
                }
            }
            pipe
        })
    }

    /// Batch pop operations using pipeline
    pub fn lpop_many(&self, keys: Vec<&str>) -> RedisResult<Vec<Option<String>>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("LPOP").arg(key);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if list is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let len = self.llen(key)?;
        Ok(len == 0)
    }

    /// Get all elements in the list
    pub fn get_all(&self, key: &str) -> RedisResult<Vec<String>> {
        self.lrange(key, 0, -1)
    }

    /// Clear the list (remove all elements)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Push only if list exists
    pub fn lpushx(&self, key: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("LPUSHX").arg(key).arg(value).query(&mut *conn)
    }

    /// Push to tail only if list exists
    pub fn rpushx(&self, key: &str, value: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("RPUSHX").arg(key).arg(value).query(&mut *conn)
    }

    /// Find positions of elements in list
    pub fn lpos(&self, key: &str, element: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("LPOS").arg(key).arg(element).query(&mut *conn)
    }

    /// Find multiple positions of elements in list
    pub fn lpos_count(&self, key: &str, element: &str, count: i64) -> RedisResult<Vec<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("LPOS")
            .arg(key)
            .arg(element)
            .arg("COUNT")
            .arg(count)
            .query(&mut *conn)
    }

    /// Find positions with rank (nth occurrence)
    pub fn lpos_rank(&self, key: &str, element: &str, rank: i64) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("LPOS")
            .arg(key)
            .arg(element)
            .arg("RANK")
            .arg(rank)
            .query(&mut *conn)
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
    fn test_basic_list_operations() {
        let conn = create_test_connection();
        let redis_list = RedisList::new(conn);

        // Test basic push/pop operations
        let _push_result = redis_list.lpush("test_list", "item1");
        let _push_result = redis_list.rpush("test_list", "item2");
        let _length = redis_list.llen("test_list");
        let _items = redis_list.lrange("test_list", 0, -1);
        
        // Clean up
        let _ = redis_list.clear("test_list");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_list_manipulation() {
        let conn = create_test_connection();
        let redis_list = RedisList::new(conn);

        // Test insert operations
        let _push_result = redis_list.lpush("test_list", "middle");
        let _insert_result = redis_list.linsert_before("test_list", "middle", "first");
        let _insert_result = redis_list.linsert_after("test_list", "middle", "last");
        
        // Clean up
        let _ = redis_list.clear("test_list");
    }
}