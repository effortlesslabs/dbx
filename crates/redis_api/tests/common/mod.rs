use reqwest::Client;
use serde_json::json;
use std::time::Duration;

// Constants
pub const BASE_URL: &str = "http://localhost:3000/redis";
pub const WS_BASE_URL: &str = "ws://localhost:3000/redis_ws/string/ws";
pub const WS_ADMIN_URL: &str = "ws://localhost:3000/redis_ws/admin/ws";

// Time delays
pub fn short_delay() -> Duration {
    Duration::from_millis(200)
}

pub fn ttl_delay() -> Duration {
    Duration::from_secs(2)
}

pub fn websocket_timeout() -> Duration {
    Duration::from_secs(15)
}

// Test data generators
pub fn generate_test_key(prefix: &str, index: Option<usize>) -> String {
    match index {
        Some(i) => format!("{}_{}", prefix, i),
        None => prefix.to_string(),
    }
}

pub fn generate_test_value(prefix: &str, index: Option<usize>) -> String {
    match index {
        Some(i) => format!("{}_{}", prefix, i),
        None => prefix.to_string(),
    }
}

pub fn generate_large_value(size: usize) -> String {
    "x".repeat(size)
}

pub fn generate_special_chars_value() -> String {
    "!@#$%^&*()_+-=[]{}|;':\",./<>?".to_string()
}

// HTTP client utilities
pub fn create_http_client() -> Client {
    Client::new()
}

pub async fn set_string(
    client: &Client,
    base_url: &str,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
        .post(&format!("{}/redis/string/{}", base_url, key))
        .json(&json!({"value": value}))
        .send()
        .await?;

    if res.status() == 200 {
        Ok(())
    } else {
        Err(format!("Failed to set string: {}", res.status()).into())
    }
}

pub async fn get_string(
    client: &Client,
    base_url: &str,
    key: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let res = client
        .get(&format!("{}/redis/string/{}", base_url, key))
        .send()
        .await?;

    if res.status() == 200 {
        let body: Option<String> = res.json().await?;
        Ok(body)
    } else {
        Err(format!("Failed to get string: {}", res.status()).into())
    }
}

pub async fn delete_string(
    client: &Client,
    base_url: &str,
    key: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let res = client
        .delete(&format!("{}/redis/string/{}", base_url, key))
        .send()
        .await?;

    if res.status() == 200 {
        let deleted: bool = res.json().await?;
        Ok(deleted)
    } else {
        Err(format!("Failed to delete string: {}", res.status()).into())
    }
}

// Common assertions
pub fn assert_status_ok(status: u16) {
    assert_eq!(status, 200, "Expected status 200, got {}", status);
}

pub fn assert_status_not_found(status: u16) {
    assert_eq!(status, 404, "Expected status 404, got {}", status);
}

pub fn assert_status_method_not_allowed(status: u16) {
    assert_eq!(status, 405, "Expected status 405, got {}", status);
}

pub fn assert_redis_info_structure(info: &serde_json::Value) {
    let required_fields = vec![
        "redis_version",
        "connected_clients",
        "used_memory_human",
        "uptime_in_seconds",
        "total_commands_processed",
        "total_connections_received",
        "keyspace_hits",
        "keyspace_misses",
        "expired_keys",
        "evicted_keys",
    ];

    for field in required_fields {
        assert!(info.get(field).is_some(), "Field {} is missing", field);
    }
}

// Batch operations
pub async fn batch_set_strings(
    client: &Client,
    base_url: &str,
    operations: Vec<(&str, &str)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let batch_ops: Vec<serde_json::Value> = operations
        .iter()
        .map(|(key, value)| json!({"key": key, "value": value}))
        .collect();

    let res = client
        .post(&format!("{}/redis/string/batch/set", base_url))
        .json(&json!({"operations": batch_ops}))
        .send()
        .await?;

    assert_status_ok(res.status().as_u16());
    Ok(())
}

pub async fn batch_get_strings(
    client: &Client,
    base_url: &str,
    keys: &[String],
) -> Result<Vec<Option<String>>, Box<dyn std::error::Error>> {
    let res = client
        .post(&format!("{}/redis/string/batch/get", base_url))
        .json(&json!({"keys": keys}))
        .send()
        .await?;

    assert_status_ok(res.status().as_u16());
    let values: Vec<Option<String>> = res.json().await?;
    Ok(values)
}

// Cleanup utilities
pub async fn cleanup_test_keys(client: &Client, base_url: &str, keys: &[&str]) {
    for key in keys {
        let _ = delete_string(client, base_url, key).await;
    }
}

// Test setup and teardown
pub struct TestContext {
    pub client: Client,
    pub base_url: String,
    pub test_keys: Vec<String>,
}

impl TestContext {
    pub fn new(base_url: String) -> Self {
        Self {
            client: create_http_client(),
            base_url,
            test_keys: Vec::new(),
        }
    }

    pub fn add_test_key(&mut self, key: String) {
        self.test_keys.push(key);
    }

    pub async fn cleanup(&self) {
        for key in &self.test_keys {
            let _ = delete_string(&self.client, &self.base_url, key).await;
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Note: This won't work in async context, but it's a fallback
        // In practice, call cleanup() explicitly in tests
    }
}
