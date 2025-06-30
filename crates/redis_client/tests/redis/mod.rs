//! Tests for HTTP Redis client functionality

use dbx_redis_client::{ HttpClient, error::Result, StringOperations };
use crate::utils;

// Import string and set test modules
pub mod string;
pub mod set;

#[tokio::test]
async fn test_http_client_creation() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    assert_eq!(client.base_url().as_str(), &format!("{}/", utils::http_test_url()));
    Ok(())
}

#[tokio::test]
async fn test_http_client_with_timeout() -> Result<()> {
    let timeout = std::time::Duration::from_secs(60);
    let client = HttpClient::with_timeout(&utils::http_test_url(), timeout)?;
    assert_eq!(client.base_url().as_str(), &format!("{}/", utils::http_test_url()));
    Ok(())
}

#[tokio::test]
async fn test_http_client_clone() -> Result<()> {
    let client1 = HttpClient::new(&utils::http_test_url())?;
    let client2 = client1.clone();
    assert_eq!(client1.base_url(), client2.base_url());
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    // Test with invalid URL (should fail gracefully)
    let invalid_client = HttpClient::new("not-a-valid-url");
    assert!(invalid_client.is_err());

    // Test operations on non-existent keys
    let non_existent_key = utils::unique_key("non_existent");
    let retrieved = string_client.get(&non_existent_key).await?;
    assert_eq!(retrieved, None);

    Ok(())
}
