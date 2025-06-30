use crate::{
    common::{client::http, HttpClientBase, SetOperations},
    error::Result,
    SetKeysRequest, SetMemberRequest, SetMembersRequest,
};
#[cfg(feature = "http")]
use reqwest::Client;
use url::Url;

/// HTTP client for set operations
#[cfg(feature = "http")]
pub struct HttpSetClient {
    client: Client,
    base_url: Url,
}

#[cfg(feature = "http")]
impl HttpSetClient {
    pub(crate) fn new(client: Client, base_url: Url) -> Self {
        Self { client, base_url }
    }
}

#[cfg(feature = "http")]
impl HttpClientBase for HttpSetClient {
    /// Get the base URL for this client
    fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[cfg(feature = "http")]
impl SetOperations for HttpSetClient {
    /// Add a member to a set
    async fn add(&mut self, key: &str, member: &str) -> Result<usize> {
        let url = self.base_url.join(&format!("redis/set/{}", key))?;
        let request = SetMemberRequest {
            member: member.to_string(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, &format!("add member to set: {}", key)).await
    }

    /// Add multiple members to a set
    async fn add_many(&mut self, key: &str, members: &[&str]) -> Result<usize> {
        let url = self.base_url.join(&format!("redis/set/{}/many", key))?;
        let request = SetMembersRequest {
            members: members.iter().map(|&s| s.to_string()).collect(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, &format!("add members to set: {}", key)).await
    }

    /// Remove a member from a set
    async fn remove(&mut self, key: &str, member: &str) -> Result<usize> {
        let url = self
            .base_url
            .join(&format!("redis/set/{}/{}", key, member))?;
        let response = self.client.delete(url).send().await?;
        http::handle_response(response, &format!("remove member from set: {}", key)).await
    }

    /// Get all members of a set
    async fn members(&mut self, key: &str) -> Result<Vec<String>> {
        let url = self.base_url.join(&format!("redis/set/{}/members", key))?;
        let response = self.client.get(url).send().await?;
        http::handle_response(response, &format!("get members of set: {}", key)).await
    }

    /// Get the cardinality (size) of a set
    async fn cardinality(&mut self, key: &str) -> Result<usize> {
        let url = self
            .base_url
            .join(&format!("redis/set/{}/cardinality", key))?;
        let response = self.client.get(url).send().await?;
        http::handle_response(response, &format!("get cardinality of set: {}", key)).await
    }

    /// Check if a member exists in a set
    async fn exists(&mut self, key: &str, member: &str) -> Result<bool> {
        let url = self
            .base_url
            .join(&format!("redis/set/{}/{}/exists", key, member))?;
        let response = self.client.get(url).send().await?;
        http::handle_response(response, &format!("check member existence in set: {}", key)).await
    }

    /// Intersect multiple sets
    async fn intersect(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("redis/set/intersect")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, "intersect sets").await
    }

    /// Union multiple sets
    async fn union(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("redis/set/union")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, "union sets").await
    }

    /// Get the difference of multiple sets (first set minus others)
    async fn difference(&mut self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("redis/set/difference")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;
        http::handle_response(response, "get set difference").await
    }

    /// Delete a set by key
    async fn delete(&mut self, key: &str) -> Result<bool> {
        let url = self.base_url.join(&format!("redis/set/{}", key))?;
        let response = self.client.delete(url).send().await?;
        http::handle_response(response, &format!("delete set: {}", key)).await
    }
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_set_client_creation() {
        let client = reqwest::Client::new();
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let set_client = HttpSetClient::new(client, base_url);
        assert_eq!(set_client.base_url().as_str(), "http://localhost:8080/");
    }
}
