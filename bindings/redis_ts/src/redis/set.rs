use dbx_redis_client::HttpClient;
use dbx_redis_client::SetOperations;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// NAPI wrapper for Set Operations
#[napi]
pub struct SetClient {
    client: Arc<HttpClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl SetClient {
    pub fn new(client: Arc<HttpClient>, runtime: Arc<Runtime>) -> Self {
        Self { client, runtime }
    }

    /// Add one member to a set
    #[napi]
    pub fn add_one(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .add_one(&key, &member)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Add multiple members to a set
    #[napi]
    pub fn add_many(&self, key: String, members: Vec<String>) -> Result<bool> {
        let client = self.client.clone();
        let member_refs: Vec<&str> = members.iter().map(|s| s.as_str()).collect();
        self.runtime.block_on(async move {
            client
                .set()
                .add_many(&key, &member_refs)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Remove a member from a set
    #[napi]
    pub fn remove(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .remove(&key, &member)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get all members of a set
    #[napi]
    pub fn members(&self, key: String) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .members(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the cardinality (size) of a set
    #[napi]
    pub fn cardinality(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .cardinality(&key)
                .await
                .map(|v| v as u32)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Check if a member exists in a set
    #[napi]
    pub fn exists(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .exists(&key, &member)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Check if a member exists in a set (alias for exists)
    #[napi]
    pub fn contains(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .contains(&key, &member)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the size of a set
    #[napi]
    pub fn size(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .size(&key)
                .await
                .map(|v| v as u32)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the intersection of multiple sets
    #[napi]
    pub fn intersect(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .intersect(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the union of multiple sets
    #[napi]
    pub fn union(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .union(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get the difference of multiple sets
    #[napi]
    pub fn difference(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .difference(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Delete a set by key
    #[napi]
    pub fn delete(&self, key: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            client
                .set()
                .delete(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }
}
