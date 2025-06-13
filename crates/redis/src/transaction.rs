use redis::{ AsyncCommands, aio::ConnectionManager };
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use nucleus::query::{ QueryResult, QueryParam };

/// Handles Redis transactions
pub struct RedisTransaction {
    connection: RedisConnection,
    in_transaction: bool,
}

impl RedisTransaction {
    /// Create a new Redis transaction handler
    pub fn new(connection: RedisConnection) -> Self {
        Self {
            connection,
            in_transaction: false,
        }
    }

    /// Begin a new transaction
    pub async fn begin(&mut self) -> RedisResult<()> {
        if self.in_transaction {
            return Err(RedisError::Transaction("Already in a transaction".to_string()));
        }

        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("MULTI")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Transaction(e.to_string()))?;

        self.in_transaction = true;
        Ok(())
    }

    /// Commit the current transaction
    pub async fn commit(&mut self) -> RedisResult<()> {
        if !self.in_transaction {
            return Err(RedisError::Transaction("Not in a transaction".to_string()));
        }

        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("EXEC")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Transaction(e.to_string()))?;

        self.in_transaction = false;
        Ok(())
    }

    /// Rollback the current transaction
    pub async fn rollback(&mut self) -> RedisResult<()> {
        if !self.in_transaction {
            return Err(RedisError::Transaction("Not in a transaction".to_string()));
        }

        let mut conn = self.connection.get_connection().await?;
        redis
            ::cmd("DISCARD")
            .query_async(&mut conn).await
            .map_err(|e| RedisError::Transaction(e.to_string()))?;

        self.in_transaction = false;
        Ok(())
    }

    /// Execute a command within the current transaction
    pub async fn execute_command(&self, command: &str, params: &[QueryParam]) -> RedisResult<()> {
        if !self.in_transaction {
            return Err(RedisError::Transaction("Not in a transaction".to_string()));
        }

        let mut conn = self.connection.get_connection().await?;
        let cmd = self.build_command(command, params)?;
        cmd.query_async(&mut conn).await.map_err(|e| RedisError::Transaction(e.to_string()))?;

        Ok(())
    }

    /// Check if currently in a transaction
    pub fn is_in_transaction(&self) -> bool {
        self.in_transaction
    }

    /// Build a Redis command from the given command string and parameters
    fn build_command(&self, command: &str, params: &[QueryParam]) -> RedisResult<redis::Cmd> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(RedisError::Transaction("Empty command".to_string()));
        }

        let mut cmd = redis::cmd(parts[0]);
        for part in &parts[1..] {
            if part.starts_with('$') {
                let index: usize = part[1..]
                    .parse()
                    .map_err(|_|
                        RedisError::Transaction(format!("Invalid parameter index: {}", part))
                    )?;
                let param = params
                    .get(index)
                    .ok_or_else(||
                        RedisError::Transaction(format!("Parameter not found: {}", part))
                    )?;
                cmd.arg(&param.value);
            } else {
                cmd.arg(part);
            }
        }

        Ok(cmd)
    }
}
