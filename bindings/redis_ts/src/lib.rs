use dbx_redis_client::redis_ws::WsClient;
use dbx_redis_client::HttpClient;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::runtime::Runtime;

// Import modules
pub mod redis;
pub mod redis_ws;

// Re-export WebSocket types at crate root for NAPI
pub use redis_ws::set::WsSetClient;
pub use redis_ws::string::WsStringClient;

/// NAPI wrapper for DBX Redis Client (HTTP)
#[napi]
pub struct DbxRedisClient {
    client: Arc<HttpClient>,
    runtime: Arc<Runtime>,
}

/// NAPI wrapper for DBX Redis WebSocket Client
#[napi]
pub struct DbxWsClient {
    client: Arc<WsClient>,
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
            HttpClient::with_timeout(&base_url, timeout)
                .map_err(|e| Error::from_reason(e.to_string()))
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
    #[napi]
    pub fn string(&self) -> redis::string::StringClient {
        redis::string::StringClient::new(self.client.clone(), self.runtime.clone())
    }

    /// Get access to set operations
    #[napi]
    pub fn set(&self) -> redis::set::SetClient {
        redis::set::SetClient::new(self.client.clone(), self.runtime.clone())
    }
}

#[napi]
impl DbxWsClient {
    /// Create a new DBX Redis WebSocket client
    #[napi(constructor)]
    pub fn new(ws_url: String) -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|e| Error::from_reason(e.to_string()))?);
        let client = runtime.block_on(async {
            WsClient::new(&ws_url)
                .await
                .map_err(|e| Error::from_reason(e.to_string()))
        })?;

        Ok(Self {
            client: Arc::new(client),
            runtime,
        })
    }

    /// Test method for NAPI export
    #[napi]
    pub fn test_method(&self) -> String {
        "hello from napi".to_string()
    }

    /// Get the base URL of the WebSocket client
    pub fn get_base_url(&self) -> String {
        self.client.base_url().to_string()
    }

    /// Get access to WebSocket string operations
    #[napi]
    pub fn string(&self) -> WsStringClient {
        WsStringClient::new(self.client.clone(), self.runtime.clone())
    }

    /// Get access to WebSocket set operations
    #[napi]
    pub fn set(&self) -> WsSetClient {
        WsSetClient::new(self.client.clone(), self.runtime.clone())
    }
}

/// Factory function to create a new DBX Redis client
#[napi]
pub fn create_client(base_url: String) -> Result<DbxRedisClient> {
    DbxRedisClient::new(base_url)
}

/// Factory function to create a new DBX Redis client with timeout
#[napi]
pub fn create_client_with_timeout(base_url: String, timeout_ms: u32) -> Result<DbxRedisClient> {
    DbxRedisClient::with_timeout(base_url, timeout_ms)
}

/// Factory function to create a new DBX Redis WebSocket client
#[napi]
pub fn create_ws_client(ws_url: String) -> Result<DbxWsClient> {
    DbxWsClient::new(ws_url)
}
