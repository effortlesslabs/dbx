use crate::{ error::Result, common::types::* };
use std::time::Duration;
use url::Url;

// =====================
// HTTP Client
// =====================
#[cfg(feature = "http")]
use reqwest::Client as ReqwestClient;
#[cfg(feature = "string")]
use crate::redis::string::HttpStringClient;
#[cfg(feature = "set")]
use crate::redis::set::HttpSetClient;

/// HTTP client for interacting with the DBX Redis API
#[cfg(feature = "http")]
pub struct HttpClient {
    client: ReqwestClient,
    base_url: Url,
}

#[cfg(feature = "http")]
impl HttpClient {
    /// Create a new HTTP client with the given base URL
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = ReqwestClient::builder().timeout(Duration::from_secs(30)).build()?;
        Ok(Self { client, base_url })
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(base_url: &str, timeout: Duration) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = ReqwestClient::builder().timeout(timeout).build()?;
        Ok(Self { client, base_url })
    }

    /// Get access to string operations
    #[cfg(feature = "string")]
    pub fn string(&self) -> HttpStringClient {
        HttpStringClient::new(self.client.clone(), self.base_url.clone())
    }

    /// Get access to set operations
    #[cfg(feature = "set")]
    pub fn set(&self) -> HttpSetClient {
        HttpSetClient::new(self.client.clone(), self.base_url.clone())
    }

    /// Get the underlying HTTP client
    pub fn http_client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Get the base URL
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[cfg(feature = "http")]
impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

// =====================
// WebSocket Client
// =====================
#[cfg(feature = "websocket")]
use tokio_tungstenite::{ connect_async, WebSocketStream, MaybeTlsStream };
#[cfg(feature = "websocket")]
use futures_util::{ SinkExt, StreamExt };
#[cfg(feature = "websocket")]
use serde_json::{ json, Value };
#[cfg(feature = "websocket")]
use tokio::net::TcpStream;
#[cfg(feature = "string")]
use crate::redis_ws::string::WsStringClient;
#[cfg(feature = "set")]
use crate::redis_ws::set::WsSetClient;

/// WebSocket client for interacting with the DBX Redis API
#[cfg(feature = "websocket")]
pub struct WsClient {
    base_url: Url,
}

#[cfg(feature = "websocket")]
impl WsClient {
    /// Create a new WebSocket client with the given URL
    pub async fn new(ws_url: &str) -> Result<Self> {
        let url = Url::parse(ws_url)?;
        Ok(Self { base_url: url })
    }

    /// Create a new WebSocket client with custom timeout
    pub async fn with_timeout(ws_url: &str, _timeout: Duration) -> Result<Self> {
        // Note: WebSocket timeout is handled differently than HTTP
        Self::new(ws_url).await
    }

    /// Get access to string operations
    #[cfg(feature = "string")]
    pub async fn string(&mut self) -> Result<WsStringClient> {
        let ws_url = self.base_url.join("string/ws")?;
        let (stream, _) = connect_async(ws_url).await?;
        Ok(WsStringClient::new(stream, self.base_url.clone()))
    }

    /// Get access to set operations
    #[cfg(feature = "set")]
    pub async fn set(&mut self) -> Result<WsSetClient> {
        let ws_url = self.base_url.join("set/ws")?;
        let (stream, _) = connect_async(ws_url).await?;
        Ok(WsSetClient::new(stream, self.base_url.clone()))
    }

    /// Get the base URL
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[cfg(feature = "websocket")]
impl Clone for WsClient {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
        }
    }
}
