mod error;
mod connection;
mod commands;
mod transaction;
mod prepared;
mod metadata;
mod pubsub;
mod script;
mod pipeline;
mod stream;
mod hll;

pub use error::{ RedisError, RedisResult };
pub use connection::RedisConnection;
pub use commands::RedisCommands;
pub use transaction::RedisTransaction;
pub use prepared::RedisPrepared;
pub use metadata::RedisMetadata;
pub use pubsub::{ RedisPubSub, PubSubMessage };
pub use script::RedisScript;
pub use pipeline::RedisPipeline;
pub use stream::{ RedisStream, StreamMessage };
pub use hll::RedisHyperLogLog;

// Re-export commonly used types
pub use nucleus::{
    config::DbConfig,
    error::{ DbError, DbResult },
    metadata::{ DatabaseMetadata, TableMetadata, ColumnMetadata },
    query::{ QueryResult, PreparedQuery, QueryParam },
    traits::{ DbxDatabase, PreparedStatementSupport, TransactionSupport, ConnectionPoolSupport },
};
use redis::{ Client, AsyncCommands, aio::ConnectionManager };
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;
use serde_json::Value;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Connection error: {0}")] Connection(String),
    #[error("Query error: {0}")] Query(String),
    #[error("Invalid data type: {0}")] InvalidDataType(String),
    #[error("Not connected to Redis")]
    NotConnected,
    #[error("Transaction error: {0}")] Transaction(String),
    #[error("Prepared statement error: {0}")] PreparedStatement(String),
}

impl From<RedisError> for DbError {
    fn from(err: RedisError) -> Self {
        match err {
            RedisError::Connection(e) => DbError::Connection(e),
            RedisError::Query(e) => DbError::Query(e),
            RedisError::InvalidDataType(e) => DbError::DataType(e),
            RedisError::NotConnected => DbError::Connection("Not connected to Redis".to_string()),
            RedisError::Transaction(e) => DbError::Transaction(e),
            RedisError::PreparedStatement(e) => DbError::Query(e),
        }
    }
}

/// Redis database implementation
pub struct RedisDatabase {
    connection: Arc<Mutex<RedisConnection>>,
    commands: Arc<Mutex<RedisCommands>>,
    transaction: Arc<Mutex<RedisTransaction>>,
    prepared: Arc<Mutex<RedisPrepared>>,
    metadata: Arc<Mutex<RedisMetadata>>,
    pubsub: Arc<Mutex<RedisPubSub>>,
    script: Arc<Mutex<RedisScript>>,
    pipeline: Arc<Mutex<RedisPipeline>>,
    stream: Arc<Mutex<RedisStream>>,
    hll: Arc<Mutex<RedisHyperLogLog>>,
}

impl RedisDatabase {
    /// Create a new Redis database instance
    pub fn new(config: DbConfig) -> Self {
        let connection = Arc::new(Mutex::new(RedisConnection::new(config.clone())));
        let commands = Arc::new(Mutex::new(RedisCommands::new(connection.clone())));
        let transaction = Arc::new(Mutex::new(RedisTransaction::new(connection.clone())));
        let prepared = Arc::new(Mutex::new(RedisPrepared::new(connection.clone())));
        let metadata = Arc::new(Mutex::new(RedisMetadata::new(connection.clone())));
        let pubsub = Arc::new(Mutex::new(RedisPubSub::new(connection.clone())));
        let script = Arc::new(Mutex::new(RedisScript::new(connection.clone())));
        let pipeline = Arc::new(Mutex::new(RedisPipeline::new(connection.clone())));
        let stream = Arc::new(Mutex::new(RedisStream::new(connection.clone())));
        let hll = Arc::new(Mutex::new(RedisHyperLogLog::new(connection.clone())));

        Self {
            connection,
            commands,
            transaction,
            prepared,
            metadata,
            pubsub,
            script,
            pipeline,
            stream,
            hll,
        }
    }

    /// Get the Pub/Sub handler
    pub fn pubsub(&self) -> Arc<Mutex<RedisPubSub>> {
        self.pubsub.clone()
    }

    /// Get the Script handler
    pub fn script(&self) -> Arc<Mutex<RedisScript>> {
        self.script.clone()
    }

    /// Get the Pipeline handler
    pub fn pipeline(&self) -> Arc<Mutex<RedisPipeline>> {
        self.pipeline.clone()
    }

    /// Get the Stream handler
    pub fn stream(&self) -> Arc<Mutex<RedisStream>> {
        self.stream.clone()
    }

    /// Get the HyperLogLog handler
    pub fn hll(&self) -> Arc<Mutex<RedisHyperLogLog>> {
        self.hll.clone()
    }

    async fn get_connection(&self) -> Result<ConnectionManager, RedisError> {
        let manager_guard = self.connection.lock().await;
        manager_guard.as_ref().cloned().ok_or(RedisError::NotConnected)
    }

    async fn execute_command<F, T>(&self, f: F) -> Result<T, RedisError>
        where F: FnOnce(&mut ConnectionManager) -> redis::RedisFuture<T>
    {
        let mut manager = self.get_connection().await?;
        f(&mut manager).await.map_err(|e| RedisError::Query(e.to_string()))
    }

    async fn execute_transaction<F, T>(&self, f: F) -> Result<T, RedisError>
        where F: FnOnce(&mut ConnectionManager) -> redis::RedisFuture<T>
    {
        let mut manager = self.get_connection().await?;
        let mut transaction = manager.multi();
        let result = f(&mut transaction).await?;
        transaction.exec().await.map_err(|e| RedisError::Transaction(e.to_string()))?;
        Ok(result)
    }
}

#[async_trait::async_trait]
impl DbxDatabase for RedisDatabase {
    async fn connect(&self) -> DbResult<()> {
        let mut conn = self.connection.lock().await;
        conn.connect().await?;
        Ok(())
    }

    async fn disconnect(&self) -> DbResult<()> {
        let mut conn = self.connection.lock().await;
        conn.disconnect().await?;
        Ok(())
    }

    async fn query(&self, query: &str) -> DbResult<QueryResult> {
        let mut commands = self.commands.lock().await;
        commands.query(query).await
    }

    async fn insert(&self, table: &str, data: &[(&str, &str)]) -> DbResult<u64> {
        let mut commands = self.commands.lock().await;
        commands.insert(table, data).await
    }

    async fn update(&self, table: &str, data: &[(&str, &str)], condition: &str) -> DbResult<u64> {
        let mut commands = self.commands.lock().await;
        commands.update(table, data, condition).await
    }

    async fn delete(&self, table: &str, condition: &str) -> DbResult<u64> {
        let mut commands = self.commands.lock().await;
        commands.delete(table, condition).await
    }

    async fn get_metadata(&self) -> DbResult<DatabaseMetadata> {
        let mut metadata = self.metadata.lock().await;
        metadata.get_metadata().await
    }
}

#[async_trait::async_trait]
impl PreparedStatementSupport for RedisDatabase {
    async fn prepare(&self, query: &str) -> DbResult<PreparedQuery> {
        let mut prepared = self.prepared.lock().await;
        prepared.prepare(query).await
    }

    async fn execute_prepared(
        &self,
        name: &str,
        params: &[QueryParam]
    ) -> Result<QueryResult, DbError> {
        self.prepared.lock().await.execute(name, params).await.map_err(DbError::from)
    }

    async fn remove_prepared(&self, name: &str) -> Result<(), DbError> {
        self.prepared.lock().await.remove(name).map_err(DbError::from)
    }

    async fn list_prepared(&self) -> Result<Vec<String>, DbError> {
        Ok(self.prepared.lock().await.list())
    }
}

#[async_trait::async_trait]
impl TransactionSupport for RedisDatabase {
    async fn begin(&self) -> DbResult<()> {
        let mut transaction = self.transaction.lock().await;
        transaction.begin().await
    }

    async fn commit(&self) -> DbResult<()> {
        let mut transaction = self.transaction.lock().await;
        transaction.commit().await
    }

    async fn rollback(&self) -> DbResult<()> {
        let mut transaction = self.transaction.lock().await;
        transaction.rollback().await
    }

    async fn is_transaction_active(&self) -> bool {
        *self.transaction.lock().await
    }
}

#[async_trait::async_trait]
impl ConnectionPoolSupport for RedisDatabase {
    async fn pool_size(&self) -> DbResult<u32> {
        // Redis doesn't expose pool size directly, so we'll return a default
        Ok(10)
    }

    async fn active_connections(&self) -> DbResult<u32> {
        let info: String = self.execute_command(|conn| {
            Box::pin(async move { redis::cmd("INFO").query_async(conn).await })
        }).await?;

        Ok(
            info
                .lines()
                .find(|line| line.starts_with("connected_clients:"))
                .and_then(|line| line.split(':').nth(1)?.trim().parse().ok())
                .unwrap_or(0)
        )
    }

    async fn idle_connections(&self) -> DbResult<u32> {
        // Redis doesn't expose idle connections directly, so we'll return a default
        Ok(0)
    }
}
