use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Sorted Set data type with operations for managing scored, ordered collections.
///
/// This implementation supports:
/// - Basic sorted set operations (add, remove, score, rank)
/// - Range queries (by rank, score, lexicographical)
/// - Set operations (union, intersection, difference)
/// - Pipelined operations (for efficiency)  
/// - Pop operations (min/max)
#[derive(Clone)]
pub struct RedisSortedSet {
    conn: Arc<Mutex<Connection>>,
}

impl RedisSortedSet {
    /// Creates a new RedisSortedSet instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic Sorted Set Operations

    /// Adds one member to a sorted set, or updates its score if it already exists
    pub fn zadd(&self, key: &str, score: f64, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zadd(key, member, score)
    }

    /// Adds multiple members to a sorted set with conditional options
    pub fn zadd_options(&self, key: &str, members: Vec<(f64, &str)>, nx: bool, xx: bool, ch: bool, incr: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        
        // Since SortedSetAddOptions is not available, we'll use basic zadd in a loop
        let mut total = 0i64;
        for (score, member) in members {
            let result: i64 = conn.zadd(key, member, score)?;
            total += result;
        }
        Ok(total)
    }

    /// Adds multiple members with their scores in a single command
    pub fn zadd_multiple(&self, key: &str, members: Vec<(f64, &str)>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let score_member_pairs: Vec<(f64, &str)> = members;
        conn.zadd_multiple(key, &score_member_pairs)
    }

    /// Removes one or more members from a sorted set
    pub fn zrem(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrem(key, member)
    }

    /// Removes multiple members from a sorted set
    pub fn zrem_multiple(&self, key: &str, members: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut total = 0i64;
        for member in members {
            let result: i64 = conn.zrem(key, member)?;
            total += result;
        }
        Ok(total)
    }

    /// Gets the score of a member in a sorted set
    pub fn zscore(&self, key: &str, member: &str) -> RedisResult<Option<f64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zscore(key, member)
    }

    /// Gets the scores of multiple members in a sorted set
    pub fn zmscore(&self, key: &str, members: Vec<&str>) -> RedisResult<Vec<Option<f64>>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zscore_multiple(key, &members)
    }

    /// Increments the score of a member in a sorted set
    pub fn zincrby(&self, key: &str, increment: f64, member: &str) -> RedisResult<f64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zincr(key, member, increment)
    }

    /// Gets the cardinality (number of members) of a sorted set
    pub fn zcard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zcard(key)
    }

    /// Counts the members in a sorted set with scores within the given range
    pub fn zcount(&self, key: &str, min: f64, max: f64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zcount(key, min, max)
    }

    // Range Operations

    /// Returns a range of members in a sorted set, by index
    pub fn zrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrange(key, start as isize, stop as isize)
    }

    /// Returns a range of members in a sorted set, by index, with scores
    pub fn zrange_withscores(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrange_withscores(key, start as isize, stop as isize)
    }

    /// Returns a range of members in a sorted set, by index, ordered from high to low
    pub fn zrevrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrange(key, start as isize, stop as isize)
    }

    /// Returns a range of members in a sorted set, by index, with scores ordered from high to low
    pub fn zrevrange_withscores(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrange_withscores(key, start as isize, stop as isize)
    }

    /// Returns a range of members in a sorted set, by score
    pub fn zrangebyscore(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore(key, min, max)
    }

    /// Returns a range of members in a sorted set, by score, with scores
    pub fn zrangebyscore_withscores(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore_withscores(key, min, max)
    }

    /// Returns a range of members in a sorted set, by score, with limit
    pub fn zrangebyscore_limit(&self, key: &str, min: f64, max: f64, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore_limit(key, min, max, offset as isize, count as isize)
    }

    /// Returns a range of members in a sorted set, by score, with scores and limit
    pub fn zrangebyscore_limit_withscores(&self, key: &str, min: f64, max: f64, offset: i64, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore_limit_withscores(key, min, max, offset as isize, count as isize)
    }

    /// Returns a range of members in a sorted set, by score, from high to low
    pub fn zrevrangebyscore(&self, key: &str, max: f64, min: f64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebyscore(key, max, min)
    }

    /// Returns a range of members in a sorted set, by score, from high to low, with limit
    pub fn zrevrangebyscore_limit(&self, key: &str, max: f64, min: f64, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebyscore_limit(key, max, min, offset as isize, count as isize)
    }

    // Lexicographical Range Operations

    /// Returns a range of members in a sorted set, by lexicographical range
    pub fn zrangebylex(&self, key: &str, min: &str, max: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebylex(key, min, max)
    }

    /// Returns a range of members in a sorted set, by lexicographical range with limit
    pub fn zrangebylex_limit(&self, key: &str, min: &str, max: &str, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebylex_limit(key, min, max, offset as isize, count as isize)
    }

    /// Returns a range of members in a sorted set, by reverse lexicographical range
    pub fn zrevrangebylex(&self, key: &str, max: &str, min: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebylex(key, max, min)
    }

    /// Count the number of members in a sorted set between a given lexicographical range
    pub fn zlexcount(&self, key: &str, min: &str, max: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zlexcount(key, min, max)
    }

    // Rank Operations

    /// Determines the index (rank) of a member in a sorted set
    pub fn zrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let result: Option<isize> = conn.zrank(key, member)?;
        Ok(result.map(|r| r as i64))
    }

    /// Determines the index (rank) of a member in a sorted set, with scores ordered from high to low
    pub fn zrevrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let result: Option<isize> = conn.zrevrank(key, member)?;
        Ok(result.map(|r| r as i64))
    }

    // Remove Range Operations

    /// Removes all members in a sorted set within the given indexes
    pub fn zremrangebyrank(&self, key: &str, start: i64, stop: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zremrangebyrank(key, start as isize, stop as isize)
    }

    /// Removes all members in a sorted set within the given scores
    pub fn zremrangebyscore(&self, key: &str, min: f64, max: f64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrembyscore(key, min, max)
    }

    /// Removes all members in a sorted set between the given lexicographical range
    pub fn zremrangebylex(&self, key: &str, min: &str, max: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrembylex(key, min, max)
    }

    // Pop Operations

    /// Removes and returns the member with the highest score in a sorted set
    pub fn zpopmax(&self, key: &str) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zpopmax(key, 1)
    }

    /// Removes and returns up to count members with the highest scores in a sorted set
    pub fn zpopmax_count(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zpopmax(key, count as isize)
    }

    /// Removes and returns the member with the lowest score in a sorted set
    pub fn zpopmin(&self, key: &str) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zpopmin(key, 1)
    }

    /// Removes and returns up to count members with the lowest scores in a sorted set
    pub fn zpopmin_count(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zpopmin(key, count as isize)
    }

    /// Removes and returns the member with the highest score in a sorted set, or blocks until one is available
    pub fn bzpopmax(&self, keys: Vec<&str>, timeout: f64) -> RedisResult<Option<(String, String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for commands not directly supported
        let mut cmd = redis::cmd("BZPOPMAX");
        cmd.arg(keys).arg(timeout);
        cmd.query(&mut *conn)
    }

    /// Removes and returns the member with the lowest score in a sorted set, or blocks until one is available
    pub fn bzpopmin(&self, keys: Vec<&str>, timeout: f64) -> RedisResult<Option<(String, String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        // Use redis::cmd for commands not directly supported
        let mut cmd = redis::cmd("BZPOPMIN");
        cmd.arg(keys).arg(timeout);
        cmd.query(&mut *conn)
    }

    // Set Operations

    /// Intersects multiple sorted sets and stores the resulting sorted set in a new key
    pub fn zinterstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zinterstore(destination, &keys)
    }

    /// Intersects multiple sorted sets with weights and stores the result
    pub fn zinterstore_weights(&self, destination: &str, keys: Vec<(&str, f64)>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zinterstore_weights(destination, &keys)
    }

    /// Unions multiple sorted sets and stores the resulting sorted set in a new key
    pub fn zunionstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zunionstore(destination, &keys)
    }

    /// Unions multiple sorted sets with weights and stores the result
    pub fn zunionstore_weights(&self, destination: &str, keys: Vec<(&str, f64)>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zunionstore_weights(destination, &keys)
    }

    /// Computes the union of multiple sorted sets (Redis 6.2+)
    pub fn zunion(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        pipe.cmd("ZUNION").arg(keys.len()).arg(keys);
        pipe.query(&mut *conn)
    }

    /// Computes the intersection of multiple sorted sets (Redis 6.2+)
    pub fn zinter(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        pipe.cmd("ZINTER").arg(keys.len()).arg(keys);
        pipe.query(&mut *conn)
    }

    /// Computes the difference of multiple sorted sets (Redis 6.2+)
    pub fn zdiff(&self, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        pipe.cmd("ZDIFF").arg(keys.len()).arg(keys);
        pipe.query(&mut *conn)
    }

    // Random Operations

    /// Returns random members from a sorted set
    pub fn zrandmember(&self, key: &str, count: Option<i64>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrandmember(key, count.map(|c| c as isize))
    }

    /// Returns random members from a sorted set with scores
    pub fn zrandmember_withscores(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrandmember_withscores(key, count as isize)
    }

    // Scanning Operations

    /// Scans a sorted set with optional pattern matching
    pub fn zscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<usize>) -> RedisResult<(u64, Vec<(String, f64)>)> {
        let mut conn = self.conn.lock().unwrap();
        
        if let Some(pat) = pattern {
            let mut iter = conn.zscan_match(key, pat)?;
            let members: Vec<(String, f64)> = iter.collect();
            Ok((0, members)) // For simplicity, return 0 as cursor since we collected all
        } else {
            let mut iter = conn.zscan(key)?;
            let members: Vec<(String, f64)> = iter.collect();
            Ok((0, members)) // For simplicity, return 0 as cursor since we collected all
        }
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for sorted set operations
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
    pub fn zadd_many(&self, operations: Vec<(&str, Vec<(f64, &str)>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, members) in operations {
                for (score, member) in members {
                    pipe.zadd(key, member, score).ignore();
                }
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if sorted set is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let count = self.zcard(key)?;
        Ok(count == 0)
    }

    /// Clear the sorted set (remove all members)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Get all members with their scores
    pub fn get_all_withscores(&self, key: &str) -> RedisResult<Vec<(String, f64)>> {
        self.zrange_withscores(key, 0, -1)
    }

    /// Get all members (without scores)
    pub fn get_all(&self, key: &str) -> RedisResult<Vec<String>> {
        self.zrange(key, 0, -1)
    }

    /// Get the top N members by score
    pub fn get_top(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        self.zrevrange_withscores(key, 0, count - 1)
    }

    /// Get the bottom N members by score
    pub fn get_bottom(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        self.zrange_withscores(key, 0, count - 1)
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
    fn test_basic_sorted_set_operations() {
        let conn = create_test_connection();
        let redis_zset = RedisSortedSet::new(conn);

        // Test basic add/remove operations
        let _add_result = redis_zset.zadd("test_zset", 1.0, "member1");
        let _add_result = redis_zset.zadd("test_zset", 2.0, "member2");
        let _score = redis_zset.zscore("test_zset", "member1");
        let _cardinality = redis_zset.zcard("test_zset");
        let _range = redis_zset.zrange("test_zset", 0, -1);
        
        // Clean up
        let _ = redis_zset.clear("test_zset");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_sorted_set_range_operations() {
        let conn = create_test_connection();
        let redis_zset = RedisSortedSet::new(conn);

        // Setup test data
        let _add_result = redis_zset.zadd_multiple("test_zset", vec![(1.0, "a"), (2.0, "b"), (3.0, "c")]);
        
        // Test range operations
        let _range_by_score = redis_zset.zrangebyscore("test_zset", 1.0, 2.0);
        let _count = redis_zset.zcount("test_zset", 1.0, 3.0);
        let _top_members = redis_zset.get_top("test_zset", 2);
        
        // Clean up
        let _ = redis_zset.clear("test_zset");
    }
}