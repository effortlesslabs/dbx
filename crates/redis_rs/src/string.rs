use crate::{ error::{ DbxError, Result }, types::* };
use reqwest::Client;
use url::Url;

/// Client for string operations
pub struct StringClient {
    client: Client,
    base_url: Url,
}

impl StringClient {
    pub(crate) fn new(client: Client, base_url: Url) -> Self {
        Self { client, base_url }
    }

    /// Get the base URL for this client
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Get a string value by key
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let url = self.base_url.join(&format!("string/{}", key))?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let value: Option<String> = response.json().await?;
            Ok(value)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to get string for key: {}", key),
            })
        }
    }

    /// Set a string value
    pub async fn set(&self, key: &str, value: &str, ttl: Option<u64>) -> Result<()> {
        let url = self.base_url.join(&format!("string/{}", key))?;
        let request = SetStringRequest {
            value: value.to_string(),
            ttl,
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to set string for key: {}", key),
            })
        }
    }

    /// Delete a string value
    pub async fn delete(&self, key: &str) -> Result<bool> {
        let url = self.base_url.join(&format!("string/{}", key))?;
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            let deleted: bool = response.json().await?;
            Ok(deleted)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to delete string for key: {}", key),
            })
        }
    }

    /// Get string information
    pub async fn info(&self, key: &str) -> Result<Option<StringInfo>> {
        let url = self.base_url.join(&format!("string/{}/info", key))?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let info: Option<StringInfo> = response.json().await?;
            Ok(info)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to get string info for key: {}", key),
            })
        }
    }

    /// Batch get multiple strings
    pub async fn batch_get(&self, keys: &[String]) -> Result<Vec<Option<String>>> {
        let url = self.base_url.join("string/batch/get")?;
        let request = BatchGetRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let values: Vec<Option<String>> = response.json().await?;
            Ok(values)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to batch get strings".to_string(),
            })
        }
    }

    /// Batch set multiple strings
    pub async fn batch_set(&self, operations: &[StringOperation]) -> Result<()> {
        let url = self.base_url.join("string/batch/set")?;
        let request = BatchSetRequest {
            operations: operations.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to batch set strings".to_string(),
            })
        }
    }

    /// Get strings by patterns
    pub async fn get_by_patterns(
        &self,
        patterns: &[String],
        grouped: Option<bool>
    ) -> Result<serde_json::Value> {
        let url = self.base_url.join("string/batch/patterns")?;
        let request = BatchGetPatternsRequest {
            patterns: patterns.to_vec(),
            grouped,
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let results: serde_json::Value = response.json().await?;
            Ok(results)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to get strings by patterns".to_string(),
            })
        }
    }

    /// Convenience method to set a string without TTL
    pub async fn set_simple(&self, key: &str, value: &str) -> Result<()> {
        self.set(key, value, None).await
    }

    /// Convenience method to set a string with TTL
    pub async fn set_with_ttl(&self, key: &str, value: &str, ttl: u64) -> Result<()> {
        self.set(key, value, Some(ttl)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::DbxClient;

    #[tokio::test]
    async fn test_string_client_creation() {
        let client = DbxClient::new("http://localhost:8080").unwrap();
        let string_client = client.string();
        assert_eq!(string_client.base_url.as_str(), "http://localhost:8080/");
    }

    #[tokio::test]
    async fn test_string_operations() {
        let client = DbxClient::new("http://localhost:8080").unwrap();
        let string_client = client.string();

        // These would fail in tests since there's no server running
        // but they test the API structure
        let _ = string_client.set_simple("test_key", "test_value").await;
        let _ = string_client.get("test_key").await;
        let _ = string_client.delete("test_key").await;
    }
}
