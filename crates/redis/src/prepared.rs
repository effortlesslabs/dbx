use std::collections::HashMap;
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use dbx_core::query::{ PreparedQuery, QueryParam, QueryResult };

/// Handles Redis prepared statements
pub struct RedisPrepared {
    connection: RedisConnection,
    statements: HashMap<String, PreparedQuery>,
}

impl RedisPrepared {
    /// Create a new Redis prepared statements handler
    pub fn new(connection: RedisConnection) -> Self {
        Self {
            connection,
            statements: HashMap::new(),
        }
    }

    /// Prepare a new query
    pub fn prepare(&mut self, name: &str, query: &str) -> RedisResult<()> {
        if self.statements.contains_key(name) {
            return Err(RedisError::Prepared(format!("Statement '{}' already exists", name)));
        }

        // Parse the query to validate it
        let parts: Vec<&str> = query.split_whitespace().collect();
        if parts.is_empty() {
            return Err(RedisError::Prepared("Empty query".to_string()));
        }

        // Validate the command
        match parts[0].to_uppercase().as_str() {
            "GET" | "HGET" | "LRANGE" | "SMEMBERS" | "ZRANGE" => (),
            _ => {
                return Err(RedisError::Prepared(format!("Unsupported command: {}", parts[0])));
            }
        }

        // Create and store the prepared query
        let prepared = PreparedQuery {
            query: query.to_string(),
            param_count: query.matches('$').count(),
        };
        self.statements.insert(name.to_string(), prepared);

        Ok(())
    }

    /// Execute a prepared query
    pub async fn execute(&self, name: &str, params: &[QueryParam]) -> RedisResult<QueryResult> {
        let prepared = self.statements
            .get(name)
            .ok_or_else(|| RedisError::Prepared(format!("Statement '{}' not found", name)))?;

        if params.len() != prepared.param_count {
            return Err(
                RedisError::Prepared(
                    format!("Expected {} parameters, got {}", prepared.param_count, params.len())
                )
            );
        }

        // Execute the query using the commands module
        let mut conn = self.connection.get_connection().await?;
        let parts: Vec<&str> = prepared.query.split_whitespace().collect();

        match parts[0].to_uppercase().as_str() {
            "GET" => self.execute_get(&mut conn, &parts[1..], params).await,
            "HGET" => self.execute_hget(&mut conn, &parts[1..], params).await,
            "LRANGE" => self.execute_lrange(&mut conn, &parts[1..], params).await,
            "SMEMBERS" => self.execute_smembers(&mut conn, &parts[1..], params).await,
            "ZRANGE" => self.execute_zrange(&mut conn, &parts[1..], params).await,
            _ => Err(RedisError::Prepared(format!("Unsupported command: {}", parts[0]))),
        }
    }

    /// Execute a prepared GET query
    async fn execute_get(
        &self,
        conn: &mut redis::aio::ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.is_empty() {
            return Err(RedisError::Prepared("Missing key for GET command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let value: Option<String> = conn
            .get(&key).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: if let Some(v) = value {
                vec![vec![serde_json::Value::String(v)]]
            } else {
                vec![]
            },
            columns: vec!["value".to_string()],
        })
    }

    /// Execute a prepared HGET query
    async fn execute_hget(
        &self,
        conn: &mut redis::aio::ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 2 {
            return Err(RedisError::Prepared("Missing key or field for HGET command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let field = self.resolve_param(args[1], params)?;
        let value: Option<String> = conn
            .hget(&key, &field).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: if let Some(v) = value {
                vec![vec![serde_json::Value::String(v)]]
            } else {
                vec![]
            },
            columns: vec!["value".to_string()],
        })
    }

    /// Execute a prepared LRANGE query
    async fn execute_lrange(
        &self,
        conn: &mut redis::aio::ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 3 {
            return Err(
                RedisError::Prepared("Missing key, start, or stop for LRANGE command".to_string())
            );
        }

        let key = self.resolve_param(args[0], params)?;
        let start: isize = self
            .resolve_param(args[1], params)?
            .parse()
            .map_err(|_| RedisError::Prepared("Invalid start index".to_string()))?;
        let stop: isize = self
            .resolve_param(args[2], params)?
            .parse()
            .map_err(|_| RedisError::Prepared("Invalid stop index".to_string()))?;

        let values: Vec<String> = conn
            .lrange(&key, start, stop).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![serde_json::Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Execute a prepared SMEMBERS query
    async fn execute_smembers(
        &self,
        conn: &mut redis::aio::ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.is_empty() {
            return Err(RedisError::Prepared("Missing key for SMEMBERS command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let values: Vec<String> = conn
            .smembers(&key).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![serde_json::Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Execute a prepared ZRANGE query
    async fn execute_zrange(
        &self,
        conn: &mut redis::aio::ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 3 {
            return Err(
                RedisError::Prepared("Missing key, start, or stop for ZRANGE command".to_string())
            );
        }

        let key = self.resolve_param(args[0], params)?;
        let start: isize = self
            .resolve_param(args[1], params)?
            .parse()
            .map_err(|_| RedisError::Prepared("Invalid start index".to_string()))?;
        let stop: isize = self
            .resolve_param(args[2], params)?
            .parse()
            .map_err(|_| RedisError::Prepared("Invalid stop index".to_string()))?;

        let values: Vec<String> = conn
            .zrange(&key, start, stop).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![serde_json::Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Resolve a parameter value
    fn resolve_param(&self, param: &str, params: &[QueryParam]) -> RedisResult<String> {
        if param.starts_with('$') {
            let index: usize = param[1..]
                .parse()
                .map_err(|_| RedisError::Prepared(format!("Invalid parameter index: {}", param)))?;
            params
                .get(index)
                .ok_or_else(|| RedisError::Prepared(format!("Parameter not found: {}", param)))?
                .value.as_str()
                .ok_or_else(||
                    RedisError::Prepared(format!("Parameter is not a string: {}", param))
                )
                .map(|s| s.to_string())
        } else {
            Ok(param.to_string())
        }
    }

    /// Remove a prepared statement
    pub fn remove(&mut self, name: &str) -> RedisResult<()> {
        if !self.statements.contains_key(name) {
            return Err(RedisError::Prepared(format!("Statement '{}' not found", name)));
        }

        self.statements.remove(name);
        Ok(())
    }

    /// List all prepared statements
    pub fn list(&self) -> Vec<String> {
        self.statements.keys().cloned().collect()
    }
}
