use dbx_redis_client::redis_ws::WsClient;
use dbx_redis_client::SetOperations;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// NAPI wrapper for WebSocket Set Operations
#[napi]
pub struct WsSetClient {
    client: Arc<WsClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl WsSetClient {
    pub fn new(client: Arc<WsClient>, runtime: Arc<Runtime>) -> Self {
        Self { client, runtime }
    }

    /// Add one member to a set via WebSocket
    #[napi]
    pub fn add_one(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .add_one(&key, &member)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Add multiple members to a set via WebSocket
    #[napi]
    pub fn add_many(&self, key: String, members: Vec<String>) -> Result<bool> {
        let client = self.client.clone();
        let member_refs: Vec<&str> = members.iter().map(|s| s.as_str()).collect();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .add_many(&key, &member_refs)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Remove a member from a set via WebSocket
    #[napi]
    pub fn remove(&self, key: String, member: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .remove(&key, &member)
                .await
                .map(|v| v as u32)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get all members of a set via WebSocket
    #[napi]
    pub fn members(&self, key: String) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .members(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the cardinality (size) of a set via WebSocket
    #[napi]
    pub fn cardinality(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .cardinality(&key)
                .await
                .map(|v| v as u32)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Check if a member exists in a set via WebSocket
    #[napi]
    pub fn exists(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .exists(&key, &member)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Check if a member exists in a set via WebSocket (alias for exists)
    #[napi]
    pub fn contains(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .contains(&key, &member)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the size of a set via WebSocket
    #[napi]
    pub fn size(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .size(&key)
                .await
                .map(|v| v as u32)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the intersection of multiple sets via WebSocket
    #[napi]
    pub fn intersect(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .intersect(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the union of multiple sets via WebSocket
    #[napi]
    pub fn union(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .union(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the difference of multiple sets via WebSocket
    #[napi]
    pub fn difference(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut set_client = ws_client
                .set()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            set_client
                .difference(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }
}
