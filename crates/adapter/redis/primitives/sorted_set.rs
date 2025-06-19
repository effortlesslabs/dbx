use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Sorted Set (ZSet) data type with operations for manipulating ordered collections with scores.
///
/// This implementation supports:
/// - Basic sorted set operations (add, remove, range queries, etc.)
/// - Score-based operations (increment, rank, score retrieval)
/// - Range operations (by score, by rank, by lexicographical order)
/// - Set operations (union, intersection with weights)
/// - Pipelined operations (for efficiency)
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

    /// Adds a member with a score to a sorted set
    pub fn zadd(&self, key: &str, score: f64, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zadd(key, member, score)
    }

    /// Adds multiple members with scores to a sorted set
    pub fn zadd_multiple(&self, key: &str, items: Vec<(f64, &str)>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zadd_multiple(key, &items)
    }

    /// Adds a member only if it doesn't exist (NX)
    pub fn zadd_nx(&self, key: &str, score: f64, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZADD")
            .arg(key)
            .arg("NX")
            .arg(score)
            .arg(member)
            .query(&mut *conn)
    }

    /// Adds a member only if it already exists (XX)
    pub fn zadd_xx(&self, key: &str, score: f64, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZADD")
            .arg(key)
            .arg("XX")
            .arg(score)
            .arg(member)
            .query(&mut *conn)
    }

    /// Removes one or more members from a sorted set
    pub fn zrem(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrem(key, member)
    }

    /// Removes multiple members from a sorted set
    pub fn zrem_multiple(&self, key: &str, members: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrem_multiple(key, &members)
    }

    /// Returns the number of members in a sorted set
    pub fn zcard(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zcard(key)
    }

    /// Returns the score of a member
    pub fn zscore(&self, key: &str, member: &str) -> RedisResult<Option<f64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zscore(key, member)
    }

    /// Returns the scores of multiple members
    pub fn zmscore(&self, key: &str, members: Vec<&str>) -> RedisResult<Vec<Option<f64>>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZMSCORE")
            .arg(key)
            .arg(&members)
            .query(&mut *conn)
    }

    /// Increments the score of a member
    pub fn zincrby(&self, key: &str, increment: f64, member: &str) -> RedisResult<f64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zincr(key, member, increment)
    }

    /// Returns the rank of a member (0-based index in ascending order)
    pub fn zrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrank(key, member)
    }

    /// Returns the rank of a member (0-based index in descending order)
    pub fn zrevrank(&self, key: &str, member: &str) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrank(key, member)
    }

    // Range Operations

    /// Returns a range of members by rank (ascending order)
    pub fn zrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrange(key, start, stop)
    }

    /// Returns a range of members with scores by rank (ascending order)
    pub fn zrange_withscores(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrange_withscores(key, start, stop)
    }

    /// Returns a range of members by rank (descending order)
    pub fn zrevrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrange(key, start, stop)
    }

    /// Returns a range of members with scores by rank (descending order)
    pub fn zrevrange_withscores(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrange_withscores(key, start, stop)
    }

    /// Returns a range of members by score
    pub fn zrangebyscore(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore(key, min, max)
    }

    /// Returns a range of members with scores by score
    pub fn zrangebyscore_withscores(&self, key: &str, min: f64, max: f64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore_withscores(key, min, max)
    }

    /// Returns a range of members by score with limit
    pub fn zrangebyscore_limit(&self, key: &str, min: f64, max: f64, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrangebyscore_limit(key, min, max, offset, count)
    }

    /// Returns a range of members by score (reverse order)
    pub fn zrevrangebyscore(&self, key: &str, max: f64, min: f64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebyscore(key, max, min)
    }

    /// Returns a range of members with scores by score (reverse order)
    pub fn zrevrangebyscore_withscores(&self, key: &str, max: f64, min: f64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebyscore_withscores(key, max, min)
    }

    /// Returns a range of members by score with limit (reverse order)
    pub fn zrevrangebyscore_limit(&self, key: &str, max: f64, min: f64, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.zrevrangebyscore_limit(key, max, min, offset, count)
    }

    /// Returns the count of members with scores between min and max
    pub fn zcount(&self, key: &str, min: f64, max: f64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zcount(key, min, max)
    }

    // Lexicographical Operations

    /// Returns a range of members by lexicographical order
    pub fn zrangebylex(&self, key: &str, min: &str, max: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZRANGEBYLEX")
            .arg(key)
            .arg(min)
            .arg(max)
            .query(&mut *conn)
    }

    /// Returns a range of members by lexicographical order with limit
    pub fn zrangebylex_limit(&self, key: &str, min: &str, max: &str, offset: i64, count: i64) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZRANGEBYLEX")
            .arg(key)
            .arg(min)
            .arg(max)
            .arg("LIMIT")
            .arg(offset)
            .arg(count)
            .query(&mut *conn)
    }

    /// Returns a range of members by lexicographical order (reverse)
    pub fn zrevrangebylex(&self, key: &str, max: &str, min: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZREVRANGEBYLEX")
            .arg(key)
            .arg(max)
            .arg(min)
            .query(&mut *conn)
    }

    /// Returns the count of members between lexicographical range
    pub fn zlexcount(&self, key: &str, min: &str, max: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZLEXCOUNT")
            .arg(key)
            .arg(min)
            .arg(max)
            .query(&mut *conn)
    }

    // Remove Operations

    /// Removes members by rank range
    pub fn zremrangebyrank(&self, key: &str, start: i64, stop: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zremrangebyrank(key, start, stop)
    }

    /// Removes members by score range
    pub fn zremrangebyscore(&self, key: &str, min: f64, max: f64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.zremrangebyscore(key, min, max)
    }

    /// Removes members by lexicographical range
    pub fn zremrangebylex(&self, key: &str, min: &str, max: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZREMRANGEBYLEX")
            .arg(key)
            .arg(min)
            .arg(max)
            .query(&mut *conn)
    }

    // Pop Operations

    /// Removes and returns the member with the highest score
    pub fn zpopmax(&self, key: &str) -> RedisResult<Option<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZPOPMAX").arg(key).query(&mut *conn)
    }

    /// Removes and returns multiple members with the highest scores
    pub fn zpopmax_count(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZPOPMAX")
            .arg(key)
            .arg(count)
            .query(&mut *conn)
    }

    /// Removes and returns the member with the lowest score
    pub fn zpopmin(&self, key: &str) -> RedisResult<Option<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZPOPMIN").arg(key).query(&mut *conn)
    }

    /// Removes and returns multiple members with the lowest scores
    pub fn zpopmin_count(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZPOPMIN")
            .arg(key)
            .arg(count)
            .query(&mut *conn)
    }

    // Blocking Pop Operations

    /// Blocking pop max from multiple keys
    pub fn bzpopmax(&self, keys: Vec<&str>, timeout: f64) -> RedisResult<Option<(String, String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BZPOPMAX")
            .arg(&keys)
            .arg(timeout)
            .query(&mut *conn)
    }

    /// Blocking pop min from multiple keys
    pub fn bzpopmin(&self, keys: Vec<&str>, timeout: f64) -> RedisResult<Option<(String, String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BZPOPMIN")
            .arg(&keys)
            .arg(timeout)
            .query(&mut *conn)
    }

    // Set Operations

    /// Computes the union of multiple sorted sets and stores the result
    pub fn zunionstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZUNIONSTORE")
            .arg(destination)
            .arg(keys.len())
            .arg(&keys)
            .query(&mut *conn)
    }

    /// Computes the union of multiple sorted sets with weights and stores the result
    pub fn zunionstore_weights(&self, destination: &str, keys: Vec<&str>, weights: Vec<f64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZUNIONSTORE")
            .arg(destination)
            .arg(keys.len())
            .arg(&keys)
            .arg("WEIGHTS")
            .arg(&weights)
            .query(&mut *conn)
    }

    /// Computes the intersection of multiple sorted sets and stores the result
    pub fn zinterstore(&self, destination: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZINTERSTORE")
            .arg(destination)
            .arg(keys.len())
            .arg(&keys)
            .query(&mut *conn)
    }

    /// Computes the intersection of multiple sorted sets with weights and stores the result
    pub fn zinterstore_weights(&self, destination: &str, keys: Vec<&str>, weights: Vec<f64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZINTERSTORE")
            .arg(destination)
            .arg(keys.len())
            .arg(&keys)
            .arg("WEIGHTS")
            .arg(&weights)
            .query(&mut *conn)
    }

    /// Computes the union of multiple sorted sets without storing
    pub fn zunion(&self, keys: Vec<&str>) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZUNION")
            .arg(keys.len())
            .arg(&keys)
            .arg("WITHSCORES")
            .query(&mut *conn)
    }

    /// Computes the intersection of multiple sorted sets without storing
    pub fn zinter(&self, keys: Vec<&str>) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZINTER")
            .arg(keys.len())
            .arg(&keys)
            .arg("WITHSCORES")
            .query(&mut *conn)
    }

    /// Computes the difference of multiple sorted sets without storing
    pub fn zdiff(&self, keys: Vec<&str>) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZDIFF")
            .arg(keys.len())
            .arg(&keys)
            .arg("WITHSCORES")
            .query(&mut *conn)
    }

    // Random Operations

    /// Returns random members from the sorted set
    pub fn zrandmember(&self, key: &str, count: Option<i64>) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("ZRANDMEMBER");
        cmd.arg(key);
        if let Some(count) = count {
            cmd.arg(count);
        }
        cmd.query(&mut *conn)
    }

    /// Returns random members with scores from the sorted set
    pub fn zrandmember_withscores(&self, key: &str, count: i64) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("ZRANDMEMBER")
            .arg(key)
            .arg(count)
            .arg("WITHSCORES")
            .query(&mut *conn)
    }

    // Scanning

    /// Scans the sorted set for members and scores
    pub fn zscan(&self, key: &str, cursor: u64, pattern: Option<&str>, count: Option<u64>) -> RedisResult<(u64, Vec<(String, f64)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("ZSCAN");
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
            for (key, items) in operations {
                for (score, member) in items {
                    pipe.cmd("ZADD").arg(key).arg(score).arg(member);
                }
            }
            pipe
        })
    }

    /// Batch remove operations using pipeline
    pub fn zrem_many(&self, operations: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, members) in operations {
                pipe.cmd("ZREM").arg(key).arg(&members);
            }
            pipe
        })
    }

    /// Batch score operations using pipeline
    pub fn zscore_many(&self, operations: Vec<(&str, &str)>) -> RedisResult<Vec<Option<f64>>> {
        self.with_pipeline(|pipe| {
            for (key, member) in operations {
                pipe.cmd("ZSCORE").arg(key).arg(member);
            }
            pipe
        })
    }

    // Utility Methods

    /// Check if sorted set is empty
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let cardinality = self.zcard(key)?;
        Ok(cardinality == 0)
    }

    /// Clear the sorted set (remove all members)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Get all members with scores
    pub fn get_all_withscores(&self, key: &str) -> RedisResult<Vec<(String, f64)>> {
        self.zrange_withscores(key, 0, -1)
    }

    /// Get all members without scores
    pub fn get_all(&self, key: &str) -> RedisResult<Vec<String>> {
        self.zrange(key, 0, -1)
    }

    /// Get top N members by score
    pub fn get_top_n(&self, key: &str, n: i64) -> RedisResult<Vec<(String, f64)>> {
        self.zrevrange_withscores(key, 0, n - 1)
    }

    /// Get bottom N members by score
    pub fn get_bottom_n(&self, key: &str, n: i64) -> RedisResult<Vec<(String, f64)>> {
        self.zrange_withscores(key, 0, n - 1)
    }

    /// Check if member exists in sorted set
    pub fn contains_member(&self, key: &str, member: &str) -> RedisResult<bool> {
        match self.zscore(key, member)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Get members within score range
    pub fn get_members_in_score_range(&self, key: &str, min_score: f64, max_score: f64) -> RedisResult<Vec<(String, f64)>> {
        self.zrangebyscore_withscores(key, min_score, max_score)
    }

    /// Get members within rank range
    pub fn get_members_in_rank_range(&self, key: &str, start_rank: i64, end_rank: i64) -> RedisResult<Vec<(String, f64)>> {
        self.zrange_withscores(key, start_rank, end_rank)
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
        let _top_members = redis_zset.get_top_n("test_zset", 2);
        
        // Clean up
        let _ = redis_zset.clear("test_zset");
    }
}