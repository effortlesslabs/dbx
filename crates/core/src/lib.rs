pub mod config;
pub mod error;
pub mod traits;
pub mod registries;

// Re-export commonly used types
pub use serde_json::Value;
pub use tokio::sync::OnceCell;
pub use async_trait::async_trait;

// Re-export configuration types
pub use config::DbConfig;
pub use error::{ DbError, DbResult };

// Re-export base traits
pub use traits::{ DbxDatabase, KeyValueStore, VectorStore };

// Re-export registry traits and configs
pub use registries::{
    RedisRegistry,
    QdrantRegistry,
    PostgresRegistry,
    RedisConfig,
    QdrantConfig,
    PostgresConfig,
};

/// Global database instance
pub static DB: OnceCell<Box<dyn DbxDatabase>> = OnceCell::const_new();

/// Database factory for creating database instances
pub struct DbxFactory;

impl DbxFactory {
    /// Create a new database instance based on configuration
    pub async fn create(config: &DbConfig) -> DbResult<Box<dyn DbxDatabase>> {
        match config.db_type {
            config::DbType::Redis => {
                // Create Redis instance
                let redis_config = config.as_redis_config();
                let redis = dbx_redis::RedisConnection::new(config.clone());
                Ok(Box::new(redis) as Box<dyn DbxDatabase>)
            }
            config::DbType::Qdrant => {
                // Create Qdrant instance
                let qdrant_config = config.as_qdrant_config();
                // TODO: Implement Qdrant instance creation
                Err(DbError::Driver("Qdrant implementation not available".into()))
            }
            config::DbType::Postgres => {
                // Create PostgreSQL instance
                let postgres_config = config.as_postgres_config();
                // TODO: Implement PostgreSQL instance creation
                Err(DbError::Driver("PostgreSQL implementation not available".into()))
            }
            config::DbType::MySQL => {
                Err(DbError::Driver("MySQL implementation not available".into()))
            }
            config::DbType::SQLite => {
                Err(DbError::Driver("SQLite implementation not available".into()))
            }
            config::DbType::MongoDB => {
                Err(DbError::Driver("MongoDB implementation not available".into()))
            }
            config::DbType::Custom(ref db_type) => {
                Err(DbError::Driver(format!("Custom database type '{}' not implemented", db_type)))
            }
        }
    }

    /// Initialize the global database instance
    pub async fn init(config: &DbConfig) -> DbResult<()> {
        let db = Self::create(config).await?;
        DB.set(db).map_err(|_|
            DbError::Driver("Failed to initialize global database instance".into())
        )
    }

    /// Get the global database instance
    pub fn get() -> Option<&'static Box<dyn DbxDatabase>> {
        DB.get()
    }

    /// Get the global database instance as a specific type
    pub fn get_as<T: 'static + DbxDatabase>() -> Option<&'static T> {
        DB.get().and_then(|db| {
            let boxed = db.as_ref();
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Box<dyn DbxDatabase>>() {
                Some(unsafe { &*(boxed as *const dyn DbxDatabase as *const T) })
            } else {
                None
            }
        })
    }
}

/// Extension trait for downcasting database instances
pub trait DbxDatabaseExt: DbxDatabase {
    /// Downcast to Any type
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: DbxDatabase + 'static> DbxDatabaseExt for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Extension trait for Redis operations
pub trait RedisExt: DbxDatabaseExt {
    /// Get Redis-specific operations
    fn as_redis(&self) -> Option<&dyn RedisRegistry>;
}

/// Extension trait for Qdrant operations
pub trait QdrantExt: DbxDatabaseExt {
    /// Get Qdrant-specific operations
    fn as_qdrant(&self) -> Option<&dyn QdrantRegistry>;
}

/// Extension trait for PostgreSQL operations
pub trait PostgresExt: DbxDatabaseExt {
    /// Get PostgreSQL-specific operations
    fn as_postgres(&self) -> Option<&dyn PostgresRegistry>;
}

impl<T: DbxDatabaseExt> RedisExt for T {
    fn as_redis(&self) -> Option<&dyn RedisRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn RedisRegistry>>()
            .map(|b| b.as_ref())
    }
}

impl<T: DbxDatabaseExt> QdrantExt for T {
    fn as_qdrant(&self) -> Option<&dyn QdrantRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn QdrantRegistry>>()
            .map(|b| b.as_ref())
    }
}

impl<T: DbxDatabaseExt> PostgresExt for T {
    fn as_postgres(&self) -> Option<&dyn PostgresRegistry> {
        self.as_any()
            .downcast_ref::<Box<dyn PostgresRegistry>>()
            .map(|b| b.as_ref())
    }
}
