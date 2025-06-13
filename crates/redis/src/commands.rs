use redis::{ AsyncCommands, aio::ConnectionManager };
use serde_json::Value;
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;
use dbx_core::query::{ QueryResult, QueryParam };

/// Handles Redis commands and operations
pub struct RedisCommands {
    connection: RedisConnection,
}

impl RedisCommands {
    /// Create a new Redis commands handler
    pub fn new(connection: RedisConnection) -> Self {
        Self { connection }
    }

    /// Execute a query and return results
    pub async fn query(&self, query: &str, params: &[QueryParam]) -> RedisResult<QueryResult> {
        let mut conn = self.connection.get_connection().await?;

        // Parse the query to determine the operation type
        let parts: Vec<&str> = query.split_whitespace().collect();
        if parts.is_empty() {
            return Err(RedisError::Query("Empty query".to_string()));
        }

        match parts[0].to_uppercase().as_str() {
            "GET" => self.handle_get(&mut conn, &parts[1..], params).await,
            "HGET" => self.handle_hget(&mut conn, &parts[1..], params).await,
            "LRANGE" => self.handle_lrange(&mut conn, &parts[1..], params).await,
            "SMEMBERS" => self.handle_smembers(&mut conn, &parts[1..], params).await,
            "ZRANGE" => self.handle_zrange(&mut conn, &parts[1..], params).await,
            _ => Err(RedisError::Query(format!("Unsupported query type: {}", parts[0]))),
        }
    }

    /// Handle GET command
    async fn handle_get(
        &self,
        conn: &mut ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.is_empty() {
            return Err(RedisError::Query("Missing key for GET command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let value: Option<String> = conn
            .get(&key).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: if let Some(v) = value {
                vec![vec![Value::String(v)]]
            } else {
                vec![]
            },
            columns: vec!["value".to_string()],
        })
    }

    /// Handle HGET command
    async fn handle_hget(
        &self,
        conn: &mut ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 2 {
            return Err(RedisError::Query("Missing key or field for HGET command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let field = self.resolve_param(args[1], params)?;
        let value: Option<String> = conn
            .hget(&key, &field).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: if let Some(v) = value {
                vec![vec![Value::String(v)]]
            } else {
                vec![]
            },
            columns: vec!["value".to_string()],
        })
    }

    /// Handle LRANGE command
    async fn handle_lrange(
        &self,
        conn: &mut ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 3 {
            return Err(
                RedisError::Query("Missing key, start, or stop for LRANGE command".to_string())
            );
        }

        let key = self.resolve_param(args[0], params)?;
        let start: isize = self
            .resolve_param(args[1], params)?
            .parse()
            .map_err(|_| RedisError::Query("Invalid start index".to_string()))?;
        let stop: isize = self
            .resolve_param(args[2], params)?
            .parse()
            .map_err(|_| RedisError::Query("Invalid stop index".to_string()))?;

        let values: Vec<String> = conn
            .lrange(&key, start, stop).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Handle SMEMBERS command
    async fn handle_smembers(
        &self,
        conn: &mut ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.is_empty() {
            return Err(RedisError::Query("Missing key for SMEMBERS command".to_string()));
        }

        let key = self.resolve_param(args[0], params)?;
        let values: Vec<String> = conn
            .smembers(&key).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Handle ZRANGE command
    async fn handle_zrange(
        &self,
        conn: &mut ConnectionManager,
        args: &[&str],
        params: &[QueryParam]
    ) -> RedisResult<QueryResult> {
        if args.len() < 3 {
            return Err(
                RedisError::Query("Missing key, start, or stop for ZRANGE command".to_string())
            );
        }

        let key = self.resolve_param(args[0], params)?;
        let start: isize = self
            .resolve_param(args[1], params)?
            .parse()
            .map_err(|_| RedisError::Query("Invalid start index".to_string()))?;
        let stop: isize = self
            .resolve_param(args[2], params)?
            .parse()
            .map_err(|_| RedisError::Query("Invalid stop index".to_string()))?;

        let values: Vec<String> = conn
            .zrange(&key, start, stop).await
            .map_err(|e| RedisError::Command(e.to_string()))?;

        Ok(QueryResult {
            rows: values
                .into_iter()
                .map(|v| vec![Value::String(v)])
                .collect(),
            columns: vec!["value".to_string()],
        })
    }

    /// Resolve a parameter value, handling both direct values and parameter references
    fn resolve_param(&self, param: &str, params: &[QueryParam]) -> RedisResult<String> {
        if param.starts_with('$') {
            let index: usize = param[1..]
                .parse()
                .map_err(|_| RedisError::Query(format!("Invalid parameter index: {}", param)))?;
            params
                .get(index)
                .ok_or_else(|| RedisError::Query(format!("Parameter not found: {}", param)))?
                .value.as_str()
                .ok_or_else(|| RedisError::Query(format!("Parameter is not a string: {}", param)))
                .map(|s| s.to_string())
        } else {
            Ok(param.to_string())
        }
    }
}
