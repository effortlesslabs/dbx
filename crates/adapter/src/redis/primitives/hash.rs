use redis::{ Commands, Connection, FromRedisValue, Pipeline, RedisResult, Script, ToRedisArgs };

use std::sync::Arc;
use std::sync::Mutex;

/// Represents a Redis hash data type with operations for manipulating hash values.
///
/// This implementation supports:
/// - Individual commands (hset, hget, hdel, hgetall, etc.)
/// - Pipelined operations (for efficiency)
/// - Transactions (for atomicity)
/// - Lua script execution (for complex operations)
#[derive(Clone)]
pub struct RedisHash {
    conn: Arc<Mutex<Connection>>,
}

/// Core implementation with basic hash operations
impl RedisHash {
    /// Creates a new RedisHash instance with the provided connection
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Gets the connection reference for direct usage
    pub fn connection(&self) -> &Arc<Mutex<Connection>> {
        &self.conn
    }

    /// Sets a field in a hash
    pub fn hset(&self, key: &str, field: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.hset(key, field, value)?;
        Ok(result == 1)
    }

    /// Sets multiple fields in a hash
    pub fn hmset(&self, key: &str, field_values: &[(&str, &str)]) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.hset_multiple(key, field_values)
    }

    /// Gets a field from a hash
    pub fn hget(&self, key: &str, field: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hget(key, field)
    }

    /// Gets multiple fields from a hash
    pub fn hmget(&self, key: &str, fields: &[&str]) -> RedisResult<Vec<Option<String>>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HMGET");
        cmd.arg(key);
        for field in fields {
            cmd.arg(field);
        }
        cmd.query(&mut *conn)
    }

    /// Gets all fields and values from a hash
    pub fn hgetall(&self, key: &str) -> RedisResult<std::collections::HashMap<String, String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hgetall(key)
    }

    /// Gets all field names from a hash
    pub fn hkeys(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hkeys(key)
    }

    /// Gets all values from a hash
    pub fn hvals(&self, key: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.hvals(key)
    }

    /// Gets the number of fields in a hash
    pub fn hlen(&self, key: &str) -> RedisResult<usize> {
        let mut conn = self.conn.lock().unwrap();
        conn.hlen(key)
    }

    /// Checks if a field exists in a hash
    pub fn hexists(&self, key: &str, field: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.hexists(key, field)?;
        Ok(result == 1)
    }

    /// Deletes one or more fields from a hash
    pub fn hdel(&self, key: &str, fields: &[&str]) -> RedisResult<usize> {
        let mut conn = self.conn.lock().unwrap();
        conn.hdel(key, fields)
    }

    /// Increments a numeric field in a hash
    pub fn hincrby(&self, key: &str, field: &str, increment: i64) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.hincr(key, field, increment)
    }

    /// Increments a float field in a hash
    pub fn hincrbyfloat(&self, key: &str, field: &str, increment: f64) -> RedisResult<f64> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HINCRBYFLOAT");
        cmd.arg(key).arg(field).arg(increment);
        cmd.query(&mut *conn)
    }

    /// Sets a field only if it doesn't exist
    pub fn hsetnx(&self, key: &str, field: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.hset_nx(key, field, value)?;
        Ok(result == 1)
    }

    /// Gets a random field from a hash
    pub fn hrandfield(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key);
        cmd.query(&mut *conn)
    }

    /// Gets multiple random fields from a hash
    pub fn hrandfield_count(&self, key: &str, count: isize) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key).arg(count);
        cmd.query(&mut *conn)
    }

    /// Gets a random field with value from a hash
    pub fn hrandfield_withvalues(
        &self,
        key: &str,
        count: isize
    ) -> RedisResult<Vec<(String, String)>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HRANDFIELD");
        cmd.arg(key).arg(count).arg("WITHVALUES");
        cmd.query(&mut *conn)
    }

    /// Scans hash fields with pattern matching
    pub fn hscan(
        &self,
        key: &str,
        cursor: usize,
        pattern: Option<&str>,
        count: Option<usize>
    ) -> RedisResult<(usize, Vec<(String, String)>)> {
        let mut conn = self.conn.lock().unwrap();
        let mut cmd = redis::cmd("HSCAN");
        cmd.arg(key).arg(cursor);
        if let Some(p) = pattern {
            cmd.arg("MATCH").arg(p);
        }
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }
        cmd.query(&mut *conn)
    }

    /// Deletes a hash
    pub fn del(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.conn.lock().unwrap();
        conn.del(key)
    }

    /// Checks if a hash exists
    pub fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.exists(key)?;
        Ok(result == 1)
    }

    /// Gets the TTL of a hash in seconds
    pub fn ttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.conn.lock().unwrap();
        conn.ttl(key)
    }

    /// Sets the TTL of a hash in seconds
    pub fn expire(&self, key: &str, seconds: u64) -> RedisResult<bool> {
        let mut conn = self.conn.lock().unwrap();
        let result: i32 = conn.expire(key, seconds as usize)?;
        Ok(result == 1)
    }

    /// Gets keys matching a pattern
    pub fn keys(&self, pattern: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.conn.lock().unwrap();
        conn.keys(pattern)
    }
}

/// Pipeline operations
impl RedisHash {
    /// Executes a function with a pipeline
    ///
    /// # Example
    /// ```ignore
    /// # use redis::{Connection, RedisResult};
    /// # use std::sync::{Arc, Mutex};
    /// # use dbx_crates::adapter::redis::primitives::hash::RedisHash;
    /// # fn example(conn: Connection) -> RedisResult<()> {
    /// let redis_hash = RedisHash::new(Arc::new(Mutex::new(conn)));
    /// let results: (bool, Option<String>) = redis_hash.with_pipeline(|pipe| {
    ///     pipe.cmd("HSET").arg("hash1").arg("field1").arg("value1")
    ///        .cmd("HGET").arg("hash1").arg("field1")
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_pipeline<F, T>(&self, f: F) -> RedisResult<T>
        where F: FnOnce(&mut Pipeline) -> &mut Pipeline, T: FromRedisValue
    {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        let result = f(&mut pipe).query(&mut *conn)?;
        Ok(result)
    }

    /// Helper: batch set multiple fields in multiple hashes using pipeline
    pub fn hset_many(&self, hash_fields: Vec<(&str, Vec<(&str, &str)>)>) -> RedisResult<Vec<bool>> {
        let raw_results: Vec<i32> = self.with_pipeline(|pipe| {
            for (hash_key, fields) in hash_fields {
                for (field, value) in fields {
                    pipe.cmd("HSET").arg(hash_key).arg(field).arg(value);
                }
            }
            pipe
        })?;

        // Convert raw integer results to booleans (1 = true, 0 = false)
        Ok(
            raw_results
                .into_iter()
                .map(|result| result == 1)
                .collect()
        )
    }

    /// Helper: batch get multiple fields from multiple hashes using pipeline
    pub fn hget_many(&self, hash_fields: Vec<(&str, &str)>) -> RedisResult<Vec<Option<String>>> {
        self.with_pipeline(|pipe| {
            for (hash_key, field) in hash_fields {
                pipe.cmd("HGET").arg(hash_key).arg(field);
            }
            pipe
        })
    }

    /// Helper: batch get all fields from multiple hashes using pipeline
    pub fn hgetall_many(
        &self,
        keys: Vec<&str>
    ) -> RedisResult<Vec<std::collections::HashMap<String, String>>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("HGETALL").arg(key);
            }
            pipe
        })
    }

    /// Helper: batch delete multiple fields from multiple hashes using pipeline
    pub fn hdel_many(&self, hash_fields: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<usize>> {
        self.with_pipeline(|pipe| {
            for (hash_key, fields) in hash_fields {
                let mut cmd = pipe.cmd("HDEL").arg(hash_key);
                for field in fields {
                    cmd = cmd.arg(field);
                }
            }
            pipe
        })
    }

    /// Helper: batch check if fields exist in hashes using pipeline
    pub fn hexists_many(&self, hash_fields: Vec<(&str, &str)>) -> RedisResult<Vec<bool>> {
        self.with_pipeline(|pipe| {
            for (hash_key, field) in hash_fields {
                pipe.cmd("HEXISTS").arg(hash_key).arg(field);
            }
            pipe
        })
    }

    /// Helper: batch get hash lengths using pipeline
    pub fn hlen_many(&self, keys: Vec<&str>) -> RedisResult<Vec<usize>> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("HLEN").arg(key);
            }
            pipe
        })
    }

    /// Helper: batch delete multiple hashes using pipeline
    pub fn del_many(&self, keys: Vec<&str>) -> RedisResult<()> {
        self.with_pipeline(|pipe| {
            for key in keys {
                pipe.cmd("DEL").arg(key);
            }
            pipe
        })
    }
}

/// Transaction operations (MULTI/EXEC)
///
/// Transactions in Redis are atomic command blocks executed with MULTI/EXEC.
/// Unlike pipelines, transactions guarantee atomicity - either all commands
/// execute or none do.
impl RedisHash {
    /// Executes a function within a transaction
    ///
    /// # Example
    /// ```ignore
    /// # use redis::{Connection, RedisResult};
    /// # use std::sync::{Arc, Mutex};
    /// # use dbx_crates::adapter::redis::primitives::hash::RedisHash;
    /// # fn example(conn: Connection) -> RedisResult<()> {
    /// let redis_hash = RedisHash::new(Arc::new(Mutex::new(conn)));
    /// let results: (bool, Option<String>) = redis_hash.with_transaction(|pipe| {
    ///     pipe.cmd("HSET").arg("hash1").arg("field1").arg("value1")
    ///        .cmd("HGET").arg("hash1").arg("field1")
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_transaction<F, T>(&self, f: F) -> RedisResult<T>
        where F: FnOnce(&mut Pipeline) -> &mut Pipeline, T: FromRedisValue
    {
        let mut conn = self.conn.lock().unwrap();
        let mut pipe = redis::pipe();
        pipe.atomic();
        let result = f(&mut pipe).query(&mut *conn)?;
        Ok(result)
    }

    /// Helper: atomically set multiple fields in multiple hashes
    pub fn hset_many_atomic(
        &self,
        hash_fields: Vec<(&str, Vec<(&str, &str)>)>
    ) -> RedisResult<Vec<bool>> {
        self.with_transaction(|pipe| {
            for (hash_key, fields) in hash_fields {
                for (field, value) in fields {
                    pipe.cmd("HSET").arg(hash_key).arg(field).arg(value);
                }
            }
            pipe
        })
    }

    /// Helper: atomically delete multiple fields from multiple hashes
    pub fn hdel_many_atomic(&self, hash_fields: Vec<(&str, Vec<&str>)>) -> RedisResult<Vec<usize>> {
        self.with_transaction(|pipe| {
            for (hash_key, fields) in hash_fields {
                let mut cmd = pipe.cmd("HDEL").arg(hash_key);
                for field in fields {
                    cmd = cmd.arg(field);
                }
            }
            pipe
        })
    }
}

/// Lua script operations
impl RedisHash {
    /// Creates a new Lua script
    ///
    /// # Example
    /// ```ignore
    /// use redis::Script;
    /// use dbx_crates::adapter::redis::primitives::hash::RedisHash;
    ///
    /// let script = RedisHash::create_script(r#"
    ///     local fields = redis.call('HGETALL', KEYS[1])
    ///     redis.call('HSET', KEYS[1], ARGV[1], ARGV[2])
    ///     return #fields / 2
    /// "#);
    /// ```
    pub fn create_script(script_source: &str) -> Script {
        Script::new(script_source)
    }

    /// Executes a Lua script with the given keys and arguments
    ///
    /// # Example
    /// ```ignore
    /// # use redis::{Connection, RedisResult, Script};
    /// # use std::sync::{Arc, Mutex};
    /// # use dbx_crates::adapter::redis::primitives::hash::RedisHash;
    /// # fn example(conn: Connection) -> RedisResult<()> {
    /// let redis_hash = RedisHash::new(Arc::new(Mutex::new(conn)));
    /// let script = RedisHash::create_script("return redis.call('HLEN', KEYS[1])");
    ///
    /// // Execute the script with "myhash" as the key and no arguments
    /// let result: usize = redis_hash.eval_script::<usize, _, _>(&script, &["myhash"], &[""])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn eval_script<T, K, A>(&self, script: &Script, keys: K, args: A) -> RedisResult<T>
        where T: FromRedisValue, K: ToRedisArgs, A: ToRedisArgs
    {
        let mut conn = self.conn.lock().unwrap();
        script.key(keys).arg(args).invoke(&mut *conn)
    }
}

/// Utility functions for common hash operations with Lua scripts
///
/// These predefined scripts provide common atomic operations that can be reused
/// across your application.
impl RedisHash {
    /// Gets a script that atomically sets a field and returns the previous value
    ///
    /// # Example
    /// ```ignore
    /// # use redis::{Connection, RedisResult};
    /// # use std::sync::{Arc, Mutex};
    /// # use dbx_crates::adapter::redis::primitives::hash::RedisHash;
    /// # fn example(conn: Connection) -> RedisResult<()> {
    /// let redis_hash = RedisHash::new(Arc::new(Mutex::new(conn)));
    /// let script = RedisHash::get_set_script();
    ///
    /// // Atomically set a field and get the previous value
    /// let previous_value: Option<String> = redis_hash.eval_script(
    ///     &script,
    ///     &["my_hash"],  // KEYS[1]
    ///     &["field1", "new_value"] // ARGV[1], ARGV[2]
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_set_script() -> Script {
        Script::new(
            r#"
            local previous = redis.call('HGET', KEYS[1], ARGV[1])
            redis.call('HSET', KEYS[1], ARGV[1], ARGV[2])
            return previous
            "#
        )
    }

    /// Gets a script that conditionally sets a field if it doesn't exist
    pub fn set_if_not_exists_script() -> Script {
        Script::new(
            r#"
            local exists = redis.call('HEXISTS', KEYS[1], ARGV[1])
            if exists == 0 then
                redis.call('HSET', KEYS[1], ARGV[1], ARGV[2])
                return 1
            else
                return 0
            end
            "#
        )
    }

    /// Gets a script that removes a field and returns whether it existed
    pub fn remove_and_check_script() -> Script {
        Script::new(
            r#"
            local removed = redis.call('HDEL', KEYS[1], ARGV[1])
            return removed
            "#
        )
    }

    /// Gets a script that atomically increments a field and returns the new value
    pub fn increment_and_get_script() -> Script {
        Script::new(
            r#"
            local new_value = redis.call('HINCRBY', KEYS[1], ARGV[1], ARGV[2])
            return new_value
            "#
        )
    }

    /// Gets a script that atomically sets multiple fields
    pub fn multi_set_script() -> Script {
        Script::new(
            r#"
            local count = 0
            for i = 1, #ARGV, 2 do
                local field = ARGV[i]
                local value = ARGV[i + 1]
                redis.call('HSET', KEYS[1], field, value)
                count = count + 1
            end
            return count
            "#
        )
    }

    /// Gets a script that atomically deletes multiple fields
    pub fn multi_delete_script() -> Script {
        Script::new(
            r#"
            local deleted = 0
            for i = 1, #ARGV do
                local field = ARGV[i]
                deleted = deleted + redis.call('HDEL', KEYS[1], field)
            end
            return deleted
            "#
        )
    }
}
