use async_trait::async_trait;
use crate::{ error::DbResult, traits::VectorStore };

/// Qdrant-specific configuration
#[derive(Debug, Clone)]
pub struct QdrantConfig {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
    pub timeout: u64,
    pub pool_size: u32,
    pub tls: bool,
}

/// Qdrant registry implementation
#[async_trait]
pub trait QdrantRegistry: VectorStore {
    /// Get Qdrant server info
    async fn info(&self) -> DbResult<serde_json::Value>;

    /// Get collection points count
    async fn count_points(&self, collection: &str) -> DbResult<u64>;

    /// Get collection points by IDs
    async fn get_points(
        &self,
        collection: &str,
        ids: &[String]
    ) -> DbResult<Vec<(Vec<f32>, serde_json::Value)>>;

    /// Scroll through collection points
    async fn scroll_points(
        &self,
        collection: &str,
        limit: u32,
        offset: Option<String>
    ) -> DbResult<(Vec<(Vec<f32>, serde_json::Value)>, Option<String>)>;

    /// Search points with filter
    async fn search_points_with_filter(
        &self,
        collection: &str,
        query_vector: &[f32],
        filter: &str,
        limit: u32
    ) -> DbResult<Vec<(f32, serde_json::Value)>>;

    /// Recommend points based on positive and negative examples
    async fn recommend_points(
        &self,
        collection: &str,
        positive: &[Vec<f32>],
        negative: &[Vec<f32>],
        limit: u32
    ) -> DbResult<Vec<(f32, serde_json::Value)>>;

    /// Create payload index
    async fn create_payload_index(
        &self,
        collection: &str,
        field_name: &str,
        field_schema: &str
    ) -> DbResult<()>;

    /// Delete payload index
    async fn delete_payload_index(&self, collection: &str, field_name: &str) -> DbResult<()>;

    /// List payload indexes
    async fn list_payload_indexes(&self, collection: &str) -> DbResult<Vec<serde_json::Value>>;

    /// Update collection parameters
    async fn update_collection_params(
        &self,
        collection: &str,
        params: &serde_json::Value
    ) -> DbResult<()>;

    /// Get collection parameters
    async fn get_collection_params(&self, collection: &str) -> DbResult<serde_json::Value>;

    /// Create snapshot
    async fn create_snapshot(&self, collection: &str) -> DbResult<String>;

    /// List snapshots
    async fn list_snapshots(&self, collection: &str) -> DbResult<Vec<String>>;

    /// Delete snapshot
    async fn delete_snapshot(&self, collection: &str, snapshot_name: &str) -> DbResult<()>;

    /// Recover from snapshot
    async fn recover_from_snapshot(&self, collection: &str, snapshot_name: &str) -> DbResult<()>;

    /// Get collection aliases
    async fn get_collection_aliases(&self, collection: &str) -> DbResult<Vec<String>>;

    /// Create collection alias
    async fn create_collection_alias(&self, collection: &str, alias: &str) -> DbResult<()>;

    /// Delete collection alias
    async fn delete_collection_alias(&self, alias: &str) -> DbResult<()>;

    /// Get collection operations status
    async fn get_operation_status(&self, operation_id: &str) -> DbResult<serde_json::Value>;

    /// Wait for operation completion
    async fn wait_for_operation(&self, operation_id: &str) -> DbResult<()>;
}
