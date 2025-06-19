use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Bitmap data type with operations for managing bit-level data.
///
/// This implementation supports:
/// - Basic bit operations (set, get, count, position)
/// - Bitwise operations (AND, OR, XOR, NOT)
/// - Advanced bitfield operations
/// - Utility operations (statistics, range operations)
/// - Pipelined operations (for efficiency)
#[derive(Clone)]
pub struct RedisBitmap {
    conn: Arc<Mutex<Connection>>,
}

/// Operations for BITFIELD command
#[derive(Debug, Clone)]
pub enum BitfieldOperation {
    Get { ty: String, offset: i64 },
    Set { ty: String, offset: i64, value: i64 },
    Incrby { ty: String, offset: i64, increment: i64 },
}

/// Statistics for a Redis Bitmap
#[derive(Debug, Clone)]
pub struct BitmapStats {
    pub total_bits: i64,
    pub set_bits: i64,
    pub clear_bits: i64,
    pub memory_usage: i64,
}

impl RedisBitmap {
    /// Creates a new RedisBitmap instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    // Basic Bit Operations

    /// Sets or clears the bit at offset
    pub fn setbit(&self, key: &str, offset: i64, value: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("SETBIT");
        cmd.arg(key).arg(offset).arg(if value { 1 } else { 0 });
        cmd.query(&mut *conn)
    }

    /// Returns the bit value at offset
    pub fn getbit(&self, key: &str, offset: i64) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("GETBIT");
        cmd.arg(key).arg(offset);
        let result: i64 = cmd.query(&mut *conn)?;
        Ok(result == 1)
    }

    /// Counts the number of set bits (population counting) in a string
    pub fn bitcount(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITCOUNT");
        cmd.arg(key);
        cmd.query(&mut *conn)
    }

    /// Counts the number of set bits in a range
    pub fn bitcount_range(&self, key: &str, start: i64, end: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITCOUNT");
        cmd.arg(key).arg(start).arg(end);
        cmd.query(&mut *conn)
    }

    /// Returns the position of the first bit set to 1 or 0 in a string
    pub fn bitpos(&self, key: &str, bit: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITPOS");
        cmd.arg(key).arg(if bit { 1 } else { 0 });
        cmd.query(&mut *conn)
    }

    /// Returns the position of the first bit set to 1 or 0 in a range
    pub fn bitpos_range(&self, key: &str, bit: bool, start: i64, end: Option<i64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITPOS");
        cmd.arg(key).arg(if bit { 1 } else { 0 }).arg(start);
        if let Some(e) = end {
            cmd.arg(e);
        }
        cmd.query(&mut *conn)
    }

    // Bitwise Operations

    /// Performs bitwise AND, OR, XOR, or NOT operation between strings
    pub fn bitop_and(&self, destkey: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITOP");
        cmd.arg("AND").arg(destkey).arg(keys);
        cmd.query(&mut *conn)
    }

    /// Performs bitwise OR operation between strings
    pub fn bitop_or(&self, destkey: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITOP");
        cmd.arg("OR").arg(destkey).arg(keys);
        cmd.query(&mut *conn)
    }

    /// Performs bitwise XOR operation between strings
    pub fn bitop_xor(&self, destkey: &str, keys: Vec<&str>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITOP");
        cmd.arg("XOR").arg(destkey).arg(keys);
        cmd.query(&mut *conn)
    }

    /// Performs bitwise NOT operation on a string
    pub fn bitop_not(&self, destkey: &str, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITOP");
        cmd.arg("NOT").arg(destkey).arg(key);
        cmd.query(&mut *conn)
    }

    // Advanced Bitfield Operations

    /// Performs multiple bitfield operations in a single command
    pub fn bitfield(&self, key: &str, operations: Vec<BitfieldOperation>) -> RedisResult<Vec<Option<i64>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("BITFIELD");
        cmd.arg(key);
        
        for op in operations {
            match op {
                BitfieldOperation::Get { ty, offset } => {
                    cmd.arg("GET").arg(ty).arg(offset);
                }
                BitfieldOperation::Set { ty, offset, value } => {
                    cmd.arg("SET").arg(ty).arg(offset).arg(value);
                }
                BitfieldOperation::Incrby { ty, offset, increment } => {
                    cmd.arg("INCRBY").arg(ty).arg(offset).arg(increment);
                }
            }
        }
        
        cmd.query(&mut *conn)
    }

    /// Gets a bitfield value
    pub fn bitfield_get(&self, key: &str, ty: &str, offset: i64) -> RedisResult<Option<i64>> {
        let operations = vec![BitfieldOperation::Get { ty: ty.to_string(), offset }];
        let results = self.bitfield(key, operations)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    /// Sets a bitfield value
    pub fn bitfield_set(&self, key: &str, ty: &str, offset: i64, value: i64) -> RedisResult<Option<i64>> {
        let operations = vec![BitfieldOperation::Set { ty: ty.to_string(), offset, value }];
        let results = self.bitfield(key, operations)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    /// Increments a bitfield value
    pub fn bitfield_incrby(&self, key: &str, ty: &str, offset: i64, increment: i64) -> RedisResult<Option<i64>> {
        let operations = vec![BitfieldOperation::Incrby { ty: ty.to_string(), offset, increment }];
        let results = self.bitfield(key, operations)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    // Pipeline Operations

    /// Executes a function with a pipeline for bitmap operations
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

    /// Batch set bit operations using pipeline
    pub fn setbit_many(&self, operations: Vec<(&str, Vec<(i64, bool)>)>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for (key, bits) in operations {
                for (offset, value) in bits {
                    pipe.cmd("SETBIT").arg(key).arg(offset).arg(if value { 1 } else { 0 }).ignore();
                }
            }
            pipe
        })
    }

    /// Batch get bit operations using pipeline
    pub fn getbit_many(&self, operations: Vec<(&str, i64)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (key, offset) in operations {
                pipe.cmd("GETBIT").arg(key).arg(offset);
            }
            pipe
        }).map(|results: Vec<i64>| results.into_iter().map(|r| r == 1).collect())
    }

    // Utility Methods

    /// Check if bitmap is empty (no bits set)
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let count = self.bitcount(key)?;
        Ok(count == 0)
    }

    /// Clear the bitmap (remove all bits)
    pub fn clear(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Set multiple bits at once
    pub fn set_bits(&self, key: &str, positions: Vec<i64>) -> RedisResult<Vec<i64>> {
        let mut results = Vec::new();
        for pos in positions {
            let result = self.setbit(key, pos, true)?;
            results.push(result);
        }
        Ok(results)
    }

    /// Clear multiple bits at once
    pub fn clear_bits(&self, key: &str, positions: Vec<i64>) -> RedisResult<Vec<i64>> {
        let mut results = Vec::new();
        for pos in positions {
            let result = self.setbit(key, pos, false)?;
            results.push(result);
        }
        Ok(results)
    }

    /// Toggle multiple bits at once
    pub fn toggle_bits(&self, key: &str, positions: Vec<i64>) -> RedisResult<Vec<bool>> {
        let mut results = Vec::new();
        for pos in positions {
            let current = self.getbit(key, pos)?;
            self.setbit(key, pos, !current)?;
            results.push(!current);
        }
        Ok(results)
    }

    /// Get bitmap statistics
    pub fn get_stats(&self, key: &str) -> RedisResult<BitmapStats> {
        let set_bits = self.bitcount(key)?;
        
        // Calculate memory usage (simplified)
        let memory_usage = self.memory_usage(key).unwrap_or(0);
        let total_bits = memory_usage * 8; // bytes to bits
        let clear_bits = total_bits - set_bits;

        Ok(BitmapStats {
            total_bits,
            set_bits,
            clear_bits,
            memory_usage,
        })
    }

    /// Get memory usage of the bitmap
    pub fn memory_usage(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("MEMORY");
        cmd.arg("USAGE").arg(key);
        cmd.query(&mut *conn)
    }

    /// Get the size of the bitmap in bytes
    pub fn strlen(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.strlen(key)
    }

    /// Find the first N set bits
    pub fn find_set_bits(&self, key: &str, count: usize) -> RedisResult<Vec<i64>> {
        let mut results = Vec::new();
        let mut pos = 0i64;
        
        while results.len() < count {
            match self.bitpos_range(key, true, pos, None) {
                Ok(found_pos) => {
                    if found_pos == -1 {
                        break; // No more set bits
                    }
                    results.push(found_pos);
                    pos = found_pos + 1;
                }
                Err(_) => break,
            }
        }
        
        Ok(results)
    }

    /// Find the first N clear bits
    pub fn find_clear_bits(&self, key: &str, count: usize) -> RedisResult<Vec<i64>> {
        let mut results = Vec::new();
        let mut pos = 0i64;
        
        while results.len() < count {
            match self.bitpos_range(key, false, pos, None) {
                Ok(found_pos) => {
                    if found_pos == -1 {
                        break; // No more clear bits
                    }
                    results.push(found_pos);
                    pos = found_pos + 1;
                }
                Err(_) => break,
            }
        }
        
        Ok(results)
    }

    /// Calculate Hamming distance between two bitmaps
    pub fn hamming_distance(&self, key1: &str, key2: &str) -> RedisResult<i64> {
        let temp_key = format!("{}:{}:hamming_temp", key1, key2);
        let _xor_result = self.bitop_xor(&temp_key, vec![key1, key2])?;
        let distance = self.bitcount(&temp_key)?;
        
        // Clean up temporary key
        let mut conn = self.conn.lock().unwrap();
        let _: () = conn.del(&temp_key)?;
        
        Ok(distance)
    }

    /// Calculate Jaccard index (similarity) between two bitmaps
    pub fn jaccard_index(&self, key1: &str, key2: &str) -> RedisResult<f64> {
        let temp_and = format!("{}:{}:and_temp", key1, key2);
        let temp_or = format!("{}:{}:or_temp", key1, key2);
        
        let _and_result = self.bitop_and(&temp_and, vec![key1, key2])?;
        let _or_result = self.bitop_or(&temp_or, vec![key1, key2])?;
        
        let intersection = self.bitcount(&temp_and)?;
        let union = self.bitcount(&temp_or)?;
        
        // Clean up temporary keys
        let mut conn = self.conn.lock().unwrap();
        let _: () = conn.del(&temp_and)?;
        let _: () = conn.del(&temp_or)?;
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok(intersection as f64 / union as f64)
        }
    }

    /// Set a range of bits to a specific value
    pub fn set_range(&self, key: &str, start: i64, end: i64, value: bool) -> RedisResult<i64> {
        let mut count = 0i64;
        for pos in start..=end {
            let result = self.setbit(key, pos, value)?;
            count += result;
        }
        Ok(count)
    }

    /// Get a range of bits as a vector
    pub fn get_range(&self, key: &str, start: i64, end: i64) -> RedisResult<Vec<bool>> {
        let mut results = Vec::new();
        for pos in start..=end {
            let bit = self.getbit(key, pos)?;
            results.push(bit);
        }
        Ok(results)
    }

    /// Copy bitmap from one key to another
    pub fn copy_bitmap(&self, source: &str, destination: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let _: () = conn.set(destination, "")?;  // Ensure destination exists
        self.bitop_or(destination, vec![source]).map(|_| ())
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
    fn test_basic_bitmap_operations() {
        let conn = create_test_connection();
        let redis_bitmap = RedisBitmap::new(conn);

        // Test basic bit operations
        let _set_result = redis_bitmap.setbit("test_bitmap", 1, true);
        let _set_result = redis_bitmap.setbit("test_bitmap", 3, true);
        let _get_result = redis_bitmap.getbit("test_bitmap", 1);
        let _count = redis_bitmap.bitcount("test_bitmap");
        let _pos = redis_bitmap.bitpos("test_bitmap", true);
        
        // Clean up
        let _ = redis_bitmap.clear("test_bitmap");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_bitwise_operations() {
        let conn = create_test_connection();
        let redis_bitmap = RedisBitmap::new(conn);

        // Setup test data
        let _set_result = redis_bitmap.setbit("bitmap1", 1, true);
        let _set_result = redis_bitmap.setbit("bitmap1", 3, true);
        let _set_result = redis_bitmap.setbit("bitmap2", 1, true);
        let _set_result = redis_bitmap.setbit("bitmap2", 2, true);
        
        // Test bitwise operations
        let _and_result = redis_bitmap.bitop_and("result_and", vec!["bitmap1", "bitmap2"]);
        let _or_result = redis_bitmap.bitop_or("result_or", vec!["bitmap1", "bitmap2"]);
        let _xor_result = redis_bitmap.bitop_xor("result_xor", vec!["bitmap1", "bitmap2"]);
        let _not_result = redis_bitmap.bitop_not("result_not", "bitmap1");
        
        // Clean up
        let _ = redis_bitmap.clear("bitmap1");
        let _ = redis_bitmap.clear("bitmap2");
        let _ = redis_bitmap.clear("result_and");
        let _ = redis_bitmap.clear("result_or");
        let _ = redis_bitmap.clear("result_xor");
        let _ = redis_bitmap.clear("result_not");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_bitmap_utility_operations() {
        let conn = create_test_connection();
        let redis_bitmap = RedisBitmap::new(conn);

        // Test utility methods
        let _set_result = redis_bitmap.set_bits("test_bitmap", vec![1, 3, 5]);
        
        let offsets = vec![1, 2, 3, 4, 5];
        let _get_result = redis_bitmap.getbit_many(offsets.iter().map(|&offset| ("test_bitmap", offset)).collect());
        
        let _stats = redis_bitmap.get_stats("test_bitmap");
        
        // Clean up
        let _ = redis_bitmap.clear("test_bitmap");
    }
}