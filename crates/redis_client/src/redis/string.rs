use crate::{
    error::Result,
    common::{ StringOperations, HttpClientBase, client::http },
    SetStringRequest,
    BatchGetRequest,
    BatchSetRequest,
    BatchGetPatternsRequest,
    StringOperation,
    StringInfo,
};
#[cfg(feature = "http")]
use reqwest::Client;
use url::Url;

/// HTTP client for string operations
#[cfg(feature = "http")]
pub struct HttpStringClient {
    client: Client,
    base_url: Url,
}

#[cfg(feature = "http")]
impl HttpStringClient {
    pub(crate) fn new(client: Client, base_url: Url) -> Self {
        Self { client, base_url }
    }
}

#[cfg(feature = "http")]
impl HttpClientBase for HttpStringClient {
    /// Get the base URL for this client
    fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[cfg(feature = "http")]
impl StringOperations for HttpStringClient {
    /// Get a string value by key
    async fn get(&mut self, key: &str) -> Result<Option<String>> {
        let url = self.base_url.join(&format!("redis/string/{}", key))?;
        let response = self.client.get(url).send().await?;
        http::handle_response(response, &format!("get string for key: {}", key)).await
    }

    /// Set a string value
    async fn set(&mut self, key: &str, value: &str, ttl: Option<u64>) -> Result<()> {
        let url = self.base_url.join(&format!("redis/string/{}", key))?;
        let request = SetStringRequest {
            value: value.to_string(),
            ttl,
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_empty_response(response, &format!("set string for key: {}", key)).await
    }

    /// Delete a string value
    async fn delete(&mut self, key: &str) -> Result<bool> {
        let url = self.base_url.join(&format!("redis/string/{}", key))?;
        let response = self.client.delete(url).send().await?;
        http::handle_response(response, &format!("delete string for key: {}", key)).await
    }

    /// Get string information
    async fn info(&mut self, key: &str) -> Result<Option<StringInfo>> {
        let url = self.base_url.join(&format!("redis/string/{}/info", key))?;
        let response = self.client.get(url).send().await?;
        http::handle_response(response, &format!("get string info for key: {}", key)).await
    }

    /// Batch get multiple strings
    async fn batch_get(&mut self, keys: &[String]) -> Result<Vec<Option<String>>> {
        let url = self.base_url.join("redis/string/batch/get")?;
        let request = BatchGetRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, "batch get strings").await
    }

    /// Batch set multiple strings
    async fn batch_set(&mut self, operations: &[StringOperation]) -> Result<()> {
        let url = self.base_url.join("redis/string/batch/set")?;
        let request = BatchSetRequest {
            operations: operations.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_empty_response(response, "batch set strings").await
    }

    /// Get strings by patterns
    async fn get_by_patterns(
        &mut self,
        patterns: &[String],
        grouped: Option<bool>
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("redis/string/batch/patterns")?;
        let request = BatchGetPatternsRequest {
            patterns: patterns.to_vec(),
            grouped,
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, "get strings by patterns").await
    }
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_string_client_creation() {
        let client = reqwest::Client::new();
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let string_client = HttpStringClient::new(client, base_url);
        assert_eq!(string_client.base_url().as_str(), "http://localhost:8080/");
    }
}
