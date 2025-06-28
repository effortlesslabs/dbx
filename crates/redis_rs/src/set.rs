use crate::{ error::{ DbxError, Result }, types::* };
use reqwest::Client;
use url::Url;

/// Client for set operations
pub struct SetClient {
    client: Client,
    base_url: Url,
}

impl SetClient {
    pub(crate) fn new(client: Client, base_url: Url) -> Self {
        Self { client, base_url }
    }

    /// Get the base URL for this client
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Add a member to a set
    pub async fn add(&self, key: &str, member: &str) -> Result<usize> {
        let url = self.base_url.join(&format!("set/{}", key))?;
        let request = SetMemberRequest {
            member: member.to_string(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let added: usize = response.json().await?;
            Ok(added)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to add member to set: {}", key),
            })
        }
    }

    /// Add multiple members to a set
    pub async fn add_many(&self, key: &str, members: &[&str]) -> Result<usize> {
        let url = self.base_url.join(&format!("set/{}", key))?;
        let request = SetMembersRequest {
            members: members
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let added: usize = response.json().await?;
            Ok(added)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to add members to set: {}", key),
            })
        }
    }

    /// Remove a member from a set
    pub async fn remove(&self, key: &str, member: &str) -> Result<usize> {
        let url = self.base_url.join(&format!("set/{}/{}", key, member))?;
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            let removed: usize = response.json().await?;
            Ok(removed)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to remove member from set: {}", key),
            })
        }
    }

    /// Get all members of a set
    pub async fn members(&self, key: &str) -> Result<Vec<String>> {
        let url = self.base_url.join(&format!("set/{}/members", key))?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let members: Vec<String> = response.json().await?;
            Ok(members)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to get members of set: {}", key),
            })
        }
    }

    /// Get the cardinality (size) of a set
    pub async fn cardinality(&self, key: &str) -> Result<usize> {
        let url = self.base_url.join(&format!("set/{}/cardinality", key))?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let cardinality: usize = response.json().await?;
            Ok(cardinality)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to get cardinality of set: {}", key),
            })
        }
    }

    /// Check if a member exists in a set
    pub async fn exists(&self, key: &str, member: &str) -> Result<bool> {
        let url = self.base_url.join(&format!("set/{}/{}/exists", key, member))?;
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let exists: bool = response.json().await?;
            Ok(exists)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: format!("Failed to check member existence in set: {}", key),
            })
        }
    }

    /// Intersect multiple sets
    pub async fn intersect(&self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("set/intersect")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let result: Vec<String> = response.json().await?;
            Ok(result)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to intersect sets".to_string(),
            })
        }
    }

    /// Union multiple sets
    pub async fn union(&self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("set/union")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let result: Vec<String> = response.json().await?;
            Ok(result)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to union sets".to_string(),
            })
        }
    }

    /// Get the difference of multiple sets (first set minus others)
    pub async fn difference(&self, keys: &[String]) -> Result<Vec<String>> {
        let url = self.base_url.join("set/difference")?;
        let request = SetKeysRequest {
            keys: keys.to_vec(),
        };

        let response = self.client.post(url).json(&request).send().await?;

        if response.status().is_success() {
            let result: Vec<String> = response.json().await?;
            Ok(result)
        } else {
            Err(DbxError::Api {
                status: response.status().as_u16(),
                message: "Failed to get set difference".to_string(),
            })
        }
    }

    /// Convenience method to add a single member
    pub async fn add_one(&self, key: &str, member: &str) -> Result<usize> {
        self.add(key, member).await
    }

    /// Convenience method to check if a member exists
    pub async fn contains(&self, key: &str, member: &str) -> Result<bool> {
        self.exists(key, member).await
    }

    /// Convenience method to get set size
    pub async fn size(&self, key: &str) -> Result<usize> {
        self.cardinality(key).await
    }
}

#[cfg(test)]
mod tests {
    use crate::DbxClient;

    #[tokio::test]
    async fn test_set_client_creation() {
        let client = DbxClient::new("http://localhost:8080").unwrap();
        let set_client = client.set();
        assert_eq!(set_client.base_url.as_str(), "http://localhost:8080/");
    }

    #[tokio::test]
    async fn test_set_operations() {
        let client = DbxClient::new("http://localhost:8080").unwrap();
        let set_client = client.set();

        // These would fail in tests since there's no server running
        // but they test the API structure
        let _ = set_client.add_one("test_set", "member1").await;
        let _ = set_client.members("test_set").await;
        let _ = set_client.contains("test_set", "member1").await;
    }
}
