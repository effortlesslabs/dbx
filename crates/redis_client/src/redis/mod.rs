//! HTTP client for DBX Redis API

use crate::error::Result;
#[cfg(feature = "http")]
use reqwest::Client;
use std::time::Duration;
use url::Url;

#[cfg(feature = "set")]
pub mod set;
#[cfg(feature = "string")]
pub mod string;

#[cfg(feature = "set")]
pub use set::HttpSetClient;
#[cfg(feature = "string")]
pub use string::HttpStringClient;

/// HTTP client for interacting with the DBX Redis API
#[cfg(feature = "http")]
pub struct HttpClient {
    client: Client,
    base_url: Url,
}

#[cfg(feature = "http")]
impl HttpClient {
    /// Create a new HTTP client with the given base URL
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self { client, base_url })
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(base_url: &str, timeout: Duration) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = Client::builder().timeout(timeout).build()?;

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
    pub fn http_client(&self) -> &Client {
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

#[cfg(all(test, feature = "http"))]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new("http://localhost:8080").unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8080/");
    }

    #[test]
    fn test_http_client_with_timeout() {
        let timeout = Duration::from_secs(60);
        let client = HttpClient::with_timeout("http://localhost:8080", timeout).unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8080/");
    }

    #[test]
    fn test_http_client_clone() {
        let client1 = HttpClient::new("http://localhost:8080").unwrap();
        let client2 = client1.clone();
        assert_eq!(client1.base_url(), client2.base_url());
    }
}
