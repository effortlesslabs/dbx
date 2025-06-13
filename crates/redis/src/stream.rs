use redis::{ AsyncCommands, aio::ConnectionManager };
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use serde_json::Value;
use std::collections::HashMap;

/// Represents a message in a Redis Stream
#[derive(Debug, Clone)]
pub struct StreamMessage {
    pub id: String,
    pub fields: HashMap<String, String>,
}

/// Handles Redis Stream operations
pub struct RedisStream {
    connection: RedisConnection,
}

impl RedisStream {
    /// Create a new Redis Stream handler
    pub fn new(connection: RedisConnection) -> Self {
        Self { connection }
    }

    /// Add a message to a stream
    pub async fn add(&self, key: &str, fields: &[(&str, &str)]) -> RedisResult<String> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("XADD").arg(key).arg("*");

        for (field, value) in fields {
            cmd.arg(field).arg(value);
        }

        let id: String = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(id)
    }

    /// Read messages from a stream
    pub async fn read(
        &self,
        key: &str,
        start_id: &str,
        count: Option<usize>
    ) -> RedisResult<Vec<StreamMessage>> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis
            ::cmd("XREAD")
            .arg("COUNT")
            .arg(count.unwrap_or(1))
            .arg("STREAMS")
            .arg(key)
            .arg(start_id);

        let result: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        let mut messages = Vec::new();
        for (_, entries) in result {
            for (id, fields) in entries {
                let mut field_map = HashMap::new();
                for (field, value) in fields {
                    field_map.insert(field, value);
                }
                messages.push(StreamMessage { id, fields: field_map });
            }
        }

        Ok(messages)
    }

    /// Read messages from multiple streams
    pub async fn read_many(
        &self,
        keys_and_ids: &[(&str, &str)],
        count: Option<usize>
    ) -> RedisResult<HashMap<String, Vec<StreamMessage>>> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("XREAD").arg("COUNT").arg(count.unwrap_or(1)).arg("STREAMS");

        for (key, _) in keys_and_ids {
            cmd.arg(key);
        }
        for (_, id) in keys_and_ids {
            cmd.arg(id);
        }

        let result: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        let mut messages = HashMap::new();
        for (key, entries) in result {
            let mut stream_messages = Vec::new();
            for (id, fields) in entries {
                let mut field_map = HashMap::new();
                for (field, value) in fields {
                    field_map.insert(field, value);
                }
                stream_messages.push(StreamMessage { id, fields: field_map });
            }
            messages.insert(key, stream_messages);
        }

        Ok(messages)
    }

    /// Get the length of a stream
    pub async fn len(&self, key: &str) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let len: u64 = redis
            ::cmd("XLEN")
            .arg(key)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(len)
    }

    /// Trim a stream to a maximum length
    pub async fn trim(&self, key: &str, max_len: u64) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let removed: u64 = redis
            ::cmd("XTRIM")
            .arg(key)
            .arg("MAXLEN")
            .arg("~")
            .arg(max_len)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(removed)
    }

    /// Delete messages from a stream
    pub async fn del(&self, key: &str, ids: &[&str]) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("XDEL").arg(key);

        for id in ids {
            cmd.arg(id);
        }

        let removed: u64 = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(removed)
    }

    /// Get stream information
    pub async fn info(&self, key: &str) -> RedisResult<HashMap<String, Value>> {
        let mut conn = self.connection.get_connection().await?;
        let info: HashMap<String, Value> = redis
            ::cmd("XINFO")
            .arg("STREAM")
            .arg(key)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(info)
    }

    /// Create a consumer group
    pub async fn create_group(&self, key: &str, group: &str, start_id: &str) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("XGROUP")
            .arg("CREATE")
            .arg(key)
            .arg(group)
            .arg(start_id)
            .arg("MKSTREAM")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(())
    }

    /// Read messages from a consumer group
    pub async fn read_group(
        &self,
        key: &str,
        group: &str,
        consumer: &str,
        count: Option<usize>
    ) -> RedisResult<Vec<StreamMessage>> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis
            ::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(group)
            .arg(consumer)
            .arg("COUNT")
            .arg(count.unwrap_or(1))
            .arg("STREAMS")
            .arg(key)
            .arg(">");

        let result: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        let mut messages = Vec::new();
        for (_, entries) in result {
            for (id, fields) in entries {
                let mut field_map = HashMap::new();
                for (field, value) in fields {
                    field_map.insert(field, value);
                }
                messages.push(StreamMessage { id, fields: field_map });
            }
        }

        Ok(messages)
    }

    /// Acknowledge messages in a consumer group
    pub async fn ack(&self, key: &str, group: &str, ids: &[&str]) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("XACK").arg(key).arg(group);

        for id in ids {
            cmd.arg(id);
        }

        let acked: u64 = cmd
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Stream(e.to_string()))?;

        Ok(acked)
    }
}
