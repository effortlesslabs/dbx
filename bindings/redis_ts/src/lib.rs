use napi::bindgen_prelude::*;
use napi_derive::napi;
use redis_client::HttpClient;
use redis_client::{ StringOperations, SetOperations };
use redis_client::common::string::{
    StringOperation,
    SetStringRequest,
    BatchGetRequest,
    BatchSetRequest,
    BatchGetPatternsRequest,
};
use redis_client::common::set::{ SetMemberRequest, SetMembersRequest, SetKeysRequest };
use std::sync::Arc;
use tokio::runtime::Runtime;

/// NAPI wrapper for DBX Redis Client
#[napi]
pub struct DbxRedisClient {
    client: Arc<HttpClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl DbxRedisClient {
    /// Create a new DBX Redis client
    #[napi(constructor)]
    pub fn new(base_url: String) -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|e| Error::from_reason(e.to_string()))?);
        let client = runtime.block_on(async {
            HttpClient::new(&base_url).map_err(|e| Error::from_reason(e.to_string()))
        })?;

        Ok(Self {
            client: Arc::new(client),
            runtime,
        })
    }

    /// Create a new DBX Redis client with custom timeout
    #[napi(factory)]
    pub fn with_timeout(base_url: String, timeout_ms: u32) -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|e| Error::from_reason(e.to_string()))?);
        let timeout = std::time::Duration::from_millis(timeout_ms as u64);
        let client = runtime.block_on(async {
            HttpClient::with_timeout(&base_url, timeout).map_err(|e|
                Error::from_reason(e.to_string())
            )
        })?;

        Ok(Self {
            client: Arc::new(client),
            runtime,
        })
    }

    /// Get the base URL of the client
    pub fn get_base_url(&self) -> String {
        self.client.base_url().to_string()
    }

    /// Get access to string operations
    pub fn string(&self) -> StringClient {
        StringClient {
            client: self.client.clone(),
            runtime: self.runtime.clone(),
        }
    }

    /// Get access to set operations
    pub fn set(&self) -> SetClient {
        SetClient {
            client: self.client.clone(),
            runtime: self.runtime.clone(),
        }
    }
}

/// NAPI wrapper for String Operations
#[napi]
pub struct StringClient {
    client: Arc<HttpClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl StringClient {
    /// Get a string value by key
    #[napi]
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .string()
                .get(&key).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Set a string value
    #[napi]
    pub fn set(&self, key: String, value: String, ttl: Option<u32>) -> Result<()> {
        let client = self.client.clone();
        let ttl = ttl.map(|t| t as u64);
        self.runtime.block_on(async move { client
                .string()
                .set(&key, &value, ttl).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Set a string value without TTL
    #[napi]
    pub fn set_simple(&self, key: String, value: String) -> Result<()> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .string()
                .set_simple(&key, &value).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Set a string value with TTL
    #[napi]
    pub fn set_with_ttl(&self, key: String, value: String, ttl: u32) -> Result<()> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .string()
                .set_with_ttl(&key, &value, ttl as u64).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Delete a string value
    #[napi]
    pub fn delete(&self, key: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .string()
                .delete(&key).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Get string information
    #[napi]
    pub fn info(&self, key: String) -> Result<Option<StringInfoJs>> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let info = client
                .string()
                .info(&key).await
                .map_err(|e| Error::from_reason(e.to_string()))?;
            Ok(
                info.map(|i| StringInfoJs {
                    key: i.key,
                    value: i.value,
                    ttl: i.ttl.map(|t| t as i64),
                    type_: i.type_,
                    encoding: i.encoding,
                    size: i.size as u32,
                })
            )
        })
    }

    /// Batch get multiple strings
    #[napi]
    pub fn batch_get(&self, keys: Vec<String>) -> Result<Vec<Option<String>>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .string()
                .batch_get(&keys).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Batch set multiple strings
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

        self.runtime.block_on(async move { client
                .string()
                .batch_set(&operations).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Get strings by patterns
    #[napi]
    pub fn get_by_patterns(&self, patterns: Vec<String>, grouped: Option<bool>) -> Result<String> {
        let client = self.client.clone();
        self.runtime.block_on(async move {
            let result = client
                .string()
                .get_by_patterns(&patterns, grouped).await
                .map_err(|e| Error::from_reason(e.to_string()))?;
            serde_json::to_string(&result).map_err(|e| Error::from_reason(e.to_string()))
        })
    }
}

/// NAPI wrapper for Set Operations
#[napi]
pub struct SetClient {
    client: Arc<HttpClient>,
    runtime: Arc<Runtime>,
}

#[napi]
impl SetClient {
    /// Add a single member to a set
    #[napi]
    pub fn add_one(&self, key: String, member: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .add_one(&key, &member).await
                .map(|r| r as u32)
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Add multiple members to a set
    #[napi]
    pub fn add_many(&self, key: String, members: Vec<String>) -> Result<u32> {
        let client = self.client.clone();
        let member_refs: Vec<&str> = members
            .iter()
            .map(|s| s.as_str())
            .collect();
        self.runtime.block_on(async move { client
                .set()
                .add_many(&key, &member_refs).await
                .map(|r| r as u32)
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Remove a member from a set
    #[napi]
    pub fn remove(&self, key: String, member: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .remove(&key, &member).await
                .map(|r| r as u32)
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Get all members of a set
    #[napi]
    pub fn members(&self, key: String) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .members(&key).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Get the cardinality (size) of a set
    #[napi]
    pub fn cardinality(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .cardinality(&key).await
                .map(|r| r as u32)
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Check if a member exists in a set
    #[napi]
    pub fn exists(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .exists(&key, &member).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Convenience method to check if a member exists
    #[napi]
    pub fn contains(&self, key: String, member: String) -> Result<bool> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .contains(&key, &member).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Convenience method to get set size
    #[napi]
    pub fn size(&self, key: String) -> Result<u32> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .size(&key).await
                .map(|r| r as u32)
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Intersect multiple sets
    #[napi]
    pub fn intersect(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .intersect(&keys).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Union multiple sets
    #[napi]
    pub fn union(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .union(&keys).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }

    /// Get the difference of multiple sets
    #[napi]
    pub fn difference(&self, keys: Vec<String>) -> Result<Vec<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async move { client
                .set()
                .difference(&keys).await
                .map_err(|e| Error::from_reason(e.to_string())) })
    }
}

/// JavaScript-compatible types
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

/// Module exports
#[napi]
pub fn create_client(base_url: String) -> Result<DbxRedisClient> {
    DbxRedisClient::new(base_url)
}

#[napi]
pub fn create_client_with_timeout(base_url: String, timeout_ms: u32) -> Result<DbxRedisClient> {
    DbxRedisClient::with_timeout(base_url, timeout_ms)
}
