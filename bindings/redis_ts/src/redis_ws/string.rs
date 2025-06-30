use dbx_redis_client::common::string::StringOperation;
use dbx_redis_client::redis_ws::WsClient;
use dbx_redis_client::StringOperations;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// NAPI wrapper for WebSocket String Operations
#[napi]
pub struct WsStringClient {
    client: Arc<WsClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl WsStringClient {
    pub fn new(client: Arc<WsClient>, runtime: Arc<Runtime>) -> Self {
        Self { client, runtime }
    }

    /// Get a string value by key via WebSocket
    #[napi]
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .get(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Set a string value via WebSocket
    #[napi]
    pub fn set(&self, key: String, value: String, ttl: Option<u32>) -> Result<bool> {
        let client = self.client.clone();
        let ttl = ttl.map(|t| t as u64);
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .set(&key, &value, ttl)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Set a string value without TTL via WebSocket
    #[napi]
    pub fn set_simple(&self, key: String, value: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .set_simple(&key, &value)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Set a string value with TTL via WebSocket
    #[napi]
    pub fn set_with_ttl(&self, key: String, value: String, ttl: u32) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .set_with_ttl(&key, &value, ttl as u64)
                .await
                .map(|_| true)
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Delete a string value via WebSocket
    #[napi]
    pub fn delete(&self, key: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .delete(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get string information via WebSocket
    #[napi]
    pub fn info(&self, key: String) -> Result<Option<StringInfoJs>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            let info = string_client
                .info(&key)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            Ok(info.map(|i| StringInfoJs {
                key: i.key,
                value: i.value,
                ttl: i.ttl,
                type_: i.type_,
                encoding: i.encoding,
                size: i.size as u32,
            }))
        })
    }

    /// Batch get multiple strings via WebSocket
    #[napi]
    pub fn batch_get(&self, keys: Vec<String>) -> Result<Vec<Option<String>>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .batch_get(&keys)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Batch set multiple strings via WebSocket
    #[napi]
    pub fn batch_set(&self, operations: Vec<StringOperationJs>) -> Result<()> {
        let client = self.client.clone();
        let operations: Vec<StringOperation> = operations
            .into_iter()
            .map(|op| StringOperation {
                key: op.key,
                value: op.value,
                ttl: op.ttl.map(|t| t as u64),
            })
            .collect();

        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            string_client
                .batch_set(&operations)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })
    }

    /// Get strings by patterns via WebSocket
    #[napi]
    pub fn get_by_patterns(&self, patterns: Vec<String>, grouped: Option<bool>) -> Result<String> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let mut ws_client = client.as_ref().clone();
            let mut string_client = ws_client
                .string()
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            let result = string_client
                .get_by_patterns(&patterns, grouped)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))?;

            serde_json::to_string(&result).map_err(|e| Error::from_reason(e.to_string()))
        })
    }
}

#[napi(object)]
pub struct StringInfoJs {
    pub key: String,
    pub value: String,
    pub ttl: Option<i64>,
    #[napi(js_name = "type")]
    pub type_: String,
    pub encoding: String,
    pub size: u32,
}

#[napi(object)]
pub struct StringOperationJs {
    pub key: String,
    pub value: Option<String>,
    pub ttl: Option<u32>,
}
