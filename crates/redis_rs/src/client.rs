use crate::{ error::Result, string::StringClient, set::SetClient };
use reqwest::Client;
use std::time::Duration;
use url::Url;

/// Main client for interacting with the DBX Redis API
pub struct DbxClient {
    client: Client,
    base_url: Url,
}

impl DbxClient {
    /// Create a new DBX client with the given base URL
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self { client, base_url })
    }

    /// Create a new DBX client with custom timeout
    pub fn with_timeout(base_url: &str, timeout: Duration) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let client = Client::builder().timeout(timeout).build()?;

        Ok(Self { client, base_url })
    }

    /// Get access to string operations
    pub fn string(&self) -> StringClient {
        StringClient::new(self.client.clone(), self.base_url.clone())
    }

    /// Get access to set operations
    pub fn set(&self) -> SetClient {
        SetClient::new(self.client.clone(), self.base_url.clone())
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

impl Clone for DbxClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DbxClient::new("http://localhost:8080").unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8080/");
    }

    #[test]
    fn test_client_with_timeout() {
        let timeout = Duration::from_secs(60);
        let client = DbxClient::with_timeout("http://localhost:8080", timeout).unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8080/");
    }

    #[test]
    fn test_client_clone() {
        let client1 = DbxClient::new("http://localhost:8080").unwrap();
        let client2 = client1.clone();
        assert_eq!(client1.base_url(), client2.base_url());
    }
}
