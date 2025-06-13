use redis::{ AsyncCommands, aio::ConnectionManager, Pipeline };
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use serde_json::Value;

/// Handles Redis pipeline operations
pub struct RedisPipeline {
    connection: RedisConnection,
    pipeline: Pipeline,
}

impl RedisPipeline {
    /// Create a new Redis pipeline handler
    pub fn new(connection: RedisConnection) -> Self {
        Self {
            connection,
            pipeline: Pipeline::new(),
        }
    }

    /// Add a command to the pipeline
    pub fn cmd(&mut self, cmd: &str) -> &mut Self {
        self.pipeline.add_command(redis::cmd(cmd));
        self
    }

    /// Add a command with arguments to the pipeline
    pub fn cmd_with_args(&mut self, cmd: &str, args: &[&str]) -> &mut Self {
        let mut command = redis::cmd(cmd);
        for arg in args {
            command.arg(arg);
        }
        self.pipeline.add_command(command);
        self
    }

    /// Execute the pipeline and get all results
    pub async fn execute(&mut self) -> RedisResult<Vec<Value>> {
        let mut conn = self.connection.get_connection().await?;
        let results: Vec<Value> = self.pipeline
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Pipeline(e.to_string()))?;
        Ok(results)
    }

    /// Execute the pipeline and get a single result
    pub async fn execute_single(&mut self) -> RedisResult<Value> {
        let results = self.execute().await?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| RedisError::Pipeline("No results returned".to_string()))
    }

    /// Clear all commands from the pipeline
    pub fn clear(&mut self) -> &mut Self {
        self.pipeline = Pipeline::new();
        self
    }

    /// Get the number of commands in the pipeline
    pub fn len(&self) -> usize {
        self.pipeline.commands().len()
    }

    /// Check if the pipeline is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add a GET command to the pipeline
    pub fn get(&mut self, key: &str) -> &mut Self {
        self.cmd_with_args("GET", &[key])
    }

    /// Add a SET command to the pipeline
    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        self.cmd_with_args("SET", &[key, value])
    }

    /// Add a DEL command to the pipeline
    pub fn del(&mut self, key: &str) -> &mut Self {
        self.cmd_with_args("DEL", &[key])
    }

    /// Add an HSET command to the pipeline
    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> &mut Self {
        self.cmd_with_args("HSET", &[key, field, value])
    }

    /// Add an HGET command to the pipeline
    pub fn hget(&mut self, key: &str, field: &str) -> &mut Self {
        self.cmd_with_args("HGET", &[key, field])
    }

    /// Add an LPUSH command to the pipeline
    pub fn lpush(&mut self, key: &str, value: &str) -> &mut Self {
        self.cmd_with_args("LPUSH", &[key, value])
    }

    /// Add an RPUSH command to the pipeline
    pub fn rpush(&mut self, key: &str, value: &str) -> &mut Self {
        self.cmd_with_args("RPUSH", &[key, value])
    }

    /// Add an SADD command to the pipeline
    pub fn sadd(&mut self, key: &str, member: &str) -> &mut Self {
        self.cmd_with_args("SADD", &[key, member])
    }

    /// Add a ZADD command to the pipeline
    pub fn zadd(&mut self, key: &str, score: f64, member: &str) -> &mut Self {
        self.cmd_with_args("ZADD", &[key, &score.to_string(), member])
    }
}
