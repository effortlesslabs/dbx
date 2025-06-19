use redis::{Commands, Connection, FromRedisValue, Pipeline, RedisResult, ToRedisArgs};
use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis Bitmap data type with operations for manipulating bits.
///
/// This implementation supports:
/// - Basic bit operations (set, get, count, position)
/// - Bitwise operations (AND, OR, XOR, NOT)
/// - Efficient bit manipulation and analysis
/// - Pipeline operations for batch bit operations
#[derive(Clone)]
pub struct RedisBitmap {
    conn: Arc<Mutex<Connection>>,
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

    /// Sets or clears the bit at the specified offset
    pub fn setbit(&self, key: &str, offset: u64, value: bool) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let bit_value = if value { 1 } else { 0 };
        let result: i32 = conn.cmd("SETBIT")
            .arg(key)
            .arg(offset)
            .arg(bit_value)
            .query(&mut *conn)?;
        Ok(result == 1)
    }

    /// Gets the bit value at the specified offset
    pub fn getbit(&self, key: &str, offset: u64) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.cmd("GETBIT")
            .arg(key)
            .arg(offset)
            .query(&mut *conn)?;
        Ok(result == 1)
    }

    /// Counts the number of set bits (population counting) in a string
    pub fn bitcount(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITCOUNT").arg(key).query(&mut *conn)
    }

    /// Counts the number of set bits in a range
    pub fn bitcount_range(&self, key: &str, start: i64, end: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITCOUNT")
            .arg(key)
            .arg(start)
            .arg(end)
            .query(&mut *conn)
    }

    /// Finds the first set (1) or clear (0) bit in a string
    pub fn bitpos(&self, key: &str, bit: bool) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let bit_value = if bit { 1 } else { 0 };
        conn.cmd("BITPOS")
            .arg(key)
            .arg(bit_value)
            .query(&mut *conn)
    }

    /// Finds the first set (1) or clear (0) bit in a range
    pub fn bitpos_range(&self, key: &str, bit: bool, start: i64, end: Option<i64>) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        let bit_value = if bit { 1 } else { 0 };
        let mut cmd = conn.cmd("BITPOS");
        cmd.arg(key).arg(bit_value).arg(start);
        
        if let Some(end) = end {
            cmd.arg(end);
        }
        
        cmd.query(&mut *conn)
    }

    // Bitwise Operations

    /// Performs a bitwise AND operation between multiple keys
    pub fn bitop_and(&self, destkey: &str, keys: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITOP")
            .arg("AND")
            .arg(destkey)
            .arg(keys)
            .query(&mut *conn)
    }

    /// Performs a bitwise OR operation between multiple keys
    pub fn bitop_or(&self, destkey: &str, keys: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITOP")
            .arg("OR")
            .arg(destkey)
            .arg(keys)
            .query(&mut *conn)
    }

    /// Performs a bitwise XOR operation between multiple keys
    pub fn bitop_xor(&self, destkey: &str, keys: &[&str]) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITOP")
            .arg("XOR")
            .arg(destkey)
            .arg(keys)
            .query(&mut *conn)
    }

    /// Performs a bitwise NOT operation on a single key
    pub fn bitop_not(&self, destkey: &str, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.cmd("BITOP")
            .arg("NOT")
            .arg(destkey)
            .arg(key)
            .query(&mut *conn)
    }

    // Advanced Bit Operations

    /// Gets information about a bitmap using BITFIELD command
    pub fn bitfield_get(&self, key: &str, type_str: &str, offset: i64) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let results: Vec<Option<i64>> = conn.cmd("BITFIELD")
            .arg(key)
            .arg("GET")
            .arg(type_str)
            .arg(offset)
            .query(&mut *conn)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    /// Sets a value in a bitmap using BITFIELD command
    pub fn bitfield_set(&self, key: &str, type_str: &str, offset: i64, value: i64) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let results: Vec<Option<i64>> = conn.cmd("BITFIELD")
            .arg(key)
            .arg("SET")
            .arg(type_str)
            .arg(offset)
            .arg(value)
            .query(&mut *conn)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    /// Increments a value in a bitmap using BITFIELD command
    pub fn bitfield_incrby(&self, key: &str, type_str: &str, offset: i64, increment: i64) -> RedisResult<Option<i64>> {
        let mut conn = self.conn.lock().unwrap();
        let results: Vec<Option<i64>> = conn.cmd("BITFIELD")
            .arg(key)
            .arg("INCRBY")
            .arg(type_str)
            .arg(offset)
            .arg(increment)
            .query(&mut *conn)?;
        Ok(results.into_iter().next().unwrap_or(None))
    }

    /// Performs multiple BITFIELD operations atomically
    pub fn bitfield_multi(&self, key: &str, operations: &[BitfieldOperation]) -> RedisResult<Vec<Option<i64>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = conn.cmd("BITFIELD");
        cmd.arg(key);
        
        for op in operations {
            match op {
                BitfieldOperation::Get { type_str, offset } => {
                    cmd.arg("GET").arg(type_str).arg(*offset);
                }
                BitfieldOperation::Set { type_str, offset, value } => {
                    cmd.arg("SET").arg(type_str).arg(*offset).arg(*value);
                }
                BitfieldOperation::IncrBy { type_str, offset, increment } => {
                    cmd.arg("INCRBY").arg(type_str).arg(*offset).arg(*increment);
                }
            }
        }
        
        cmd.query(&mut *conn)
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
    pub fn setbit_many(&self, operations: Vec<(&str, u64, bool)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (key, offset, value) in operations {
                let bit_value = if value { 1 } else { 0 };
                pipe.cmd("SETBIT").arg(key).arg(offset).arg(bit_value);
            }
            pipe
        }).map(|results: Vec<i32>| {
            results.into_iter().map(|r| r == 1).collect()
        })
    }

    /// Batch get bit operations using pipeline
    pub fn getbit_many(&self, operations: Vec<(&str, u64)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (key, offset) in operations {
                pipe.cmd("GETBIT").arg(key).arg(offset);
            }
            pipe
        }).map(|results: Vec<i32>| {
            results.into_iter().map(|r| r == 1).collect()
        })
    }

    /// Batch bit count operations using pipeline
    pub fn bitcount_many(&self, keys: Vec<&str>) -> RedisResult<Vec<i64>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("BITCOUNT").arg(key);
            }
            pipe
        })
    }

    // Utility Methods

    /// Sets multiple bits at once
    pub fn set_bits(&self, key: &str, bit_positions: &[(u64, bool)]) -> RedisResult<Vec<bool>> {
        let operations: Vec<(&str, u64, bool)> = bit_positions
            .iter()
            .map(|(offset, value)| (key, *offset, *value))
            .collect();
        self.setbit_many(operations)
    }

    /// Gets multiple bits at once
    pub fn get_bits(&self, key: &str, offsets: &[u64]) -> RedisResult<Vec<bool>> {
        let operations: Vec<(&str, u64)> = offsets
            .iter()
            .map(|offset| (key, *offset))
            .collect();
        self.getbit_many(operations)
    }

    /// Checks if all specified bits are set
    pub fn all_bits_set(&self, key: &str, offsets: &[u64]) -> RedisResult<bool> {
        let bits = self.get_bits(key, offsets)?;
        Ok(bits.iter().all(|&bit| bit))
    }

    /// Checks if any of the specified bits are set
    pub fn any_bits_set(&self, key: &str, offsets: &[u64]) -> RedisResult<bool> {
        let bits = self.get_bits(key, offsets)?;
        Ok(bits.iter().any(|&bit| bit))
    }

    /// Counts set bits in multiple keys
    pub fn count_set_bits_multi(&self, keys: &[&str]) -> RedisResult<i64> {
        let counts = self.bitcount_many(keys.to_vec())?;
        Ok(counts.iter().sum())
    }

    /// Finds the first N set bits in a bitmap
    pub fn find_set_bits(&self, key: &str, limit: usize) -> RedisResult<Vec<i64>> {
        let mut positions = Vec::new();
        let mut offset = 0;
        
        while positions.len() < limit {
            match self.bitpos_range(key, true, offset, None)? {
                -1 => break, // No more set bits found
                pos => {
                    positions.push(pos);
                    offset = pos + 1;
                }
            }
        }
        
        Ok(positions)
    }

    /// Finds the first N clear bits in a bitmap
    pub fn find_clear_bits(&self, key: &str, limit: usize) -> RedisResult<Vec<i64>> {
        let mut positions = Vec::new();
        let mut offset = 0;
        
        while positions.len() < limit {
            match self.bitpos_range(key, false, offset, None)? {
                -1 => break, // No more clear bits found
                pos => {
                    positions.push(pos);
                    offset = pos + 1;
                }
            }
        }
        
        Ok(positions)
    }

    /// Clears all bits in a bitmap
    pub fn clear_all(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Sets a range of bits to a specific value
    pub fn set_bit_range(&self, key: &str, start_offset: u64, end_offset: u64, value: bool) -> RedisResult<Vec<bool>> {
        let offsets: Vec<u64> = (start_offset..=end_offset).collect();
        let bit_positions: Vec<(u64, bool)> = offsets.iter().map(|&offset| (offset, value)).collect();
        self.set_bits(key, &bit_positions)
    }

    /// Gets the bitmap as a byte array
    pub fn get_bitmap_bytes(&self, key: &str) -> RedisResult<Vec<u8>> {
        let mut conn = self.conn.lock().unwrap();
        let bytes: Vec<u8> = conn.get(key)?;
        Ok(bytes)
    }

    /// Sets the bitmap from a byte array
    pub fn set_bitmap_bytes(&self, key: &str, bytes: &[u8]) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.set(key, bytes)
    }

    /// Calculates the Hamming distance between two bitmaps
    pub fn hamming_distance(&self, key1: &str, key2: &str) -> RedisResult<i64> {
        // XOR the two keys and count the set bits
        let temp_key = format!("{}:{}:hamming_temp", key1, key2);
        self.bitop_xor(&temp_key, &[key1, key2])?;
        let distance = self.bitcount(&temp_key)?;
        
        // Clean up temporary key
        let mut conn = self.conn.lock().unwrap();
        let _: () = conn.del(&temp_key)?;
        
        Ok(distance)
    }

    /// Calculates the Jaccard index between two bitmaps
    pub fn jaccard_index(&self, key1: &str, key2: &str) -> RedisResult<f64> {
        let temp_and = format!("{}:{}:and_temp", key1, key2);
        let temp_or = format!("{}:{}:or_temp", key1, key2);
        
        // Calculate intersection (AND)
        self.bitop_and(&temp_and, &[key1, key2])?;
        let intersection = self.bitcount(&temp_and)?;
        
        // Calculate union (OR)
        self.bitop_or(&temp_or, &[key1, key2])?;
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

    /// Checks if the bitmap is empty (all bits are 0)
    pub fn is_empty(&self, key: &str) -> RedisResult<bool> {
        let count = self.bitcount(key)?;
        Ok(count == 0)
    }

    /// Gets bitmap statistics
    pub fn get_bitmap_stats(&self, key: &str) -> RedisResult<BitmapStats> {
        let total_bits = self.bitcount(key)?;
        let first_set_bit = if total_bits > 0 {
            match self.bitpos(key, true)? {
                -1 => None,
                pos => Some(pos as u64),
            }
        } else {
            None
        };
        
        let first_clear_bit = match self.bitpos(key, false)? {
            -1 => None,
            pos => Some(pos as u64),
        };
        
        Ok(BitmapStats {
            total_set_bits: total_bits,
            first_set_bit,
            first_clear_bit,
        })
    }
}

/// Represents a BITFIELD operation
#[derive(Debug, Clone)]
pub enum BitfieldOperation {
    Get { type_str: String, offset: i64 },
    Set { type_str: String, offset: i64, value: i64 },
    IncrBy { type_str: String, offset: i64, increment: i64 },
}

/// Bitmap statistics structure
#[derive(Debug, Clone)]
pub struct BitmapStats {
    pub total_set_bits: i64,
    pub first_set_bit: Option<u64>,
    pub first_clear_bit: Option<u64>,
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
        let _ = redis_bitmap.clear_all("test_bitmap");
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
        let _and_result = redis_bitmap.bitop_and("result_and", &["bitmap1", "bitmap2"]);
        let _or_result = redis_bitmap.bitop_or("result_or", &["bitmap1", "bitmap2"]);
        let _xor_result = redis_bitmap.bitop_xor("result_xor", &["bitmap1", "bitmap2"]);
        let _not_result = redis_bitmap.bitop_not("result_not", "bitmap1");
        
        // Clean up
        let _ = redis_bitmap.clear_all("bitmap1");
        let _ = redis_bitmap.clear_all("bitmap2");
        let _ = redis_bitmap.clear_all("result_and");
        let _ = redis_bitmap.clear_all("result_or");
        let _ = redis_bitmap.clear_all("result_xor");
        let _ = redis_bitmap.clear_all("result_not");
    }

    #[test]
    #[ignore = "Requires Redis server"]
    fn test_bitmap_utilities() {
        let conn = create_test_connection();
        let redis_bitmap = RedisBitmap::new(conn);

        // Test utility methods
        let bit_positions = [(1u64, true), (3u64, true), (5u64, false)];
        let _set_result = redis_bitmap.set_bits("test_bitmap", &bit_positions);
        
        let offsets = [1, 2, 3, 4, 5];
        let _get_result = redis_bitmap.get_bits("test_bitmap", &offsets);
        
        let _all_set = redis_bitmap.all_bits_set("test_bitmap", &[1, 3]);
        let _any_set = redis_bitmap.any_bits_set("test_bitmap", &[2, 4]);
        let _stats = redis_bitmap.get_bitmap_stats("test_bitmap");
        
        // Clean up
        let _ = redis_bitmap.clear_all("test_bitmap");
    }
}