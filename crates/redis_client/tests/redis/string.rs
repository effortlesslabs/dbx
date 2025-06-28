//! Tests for HTTP Redis string operations

use redis_client::{ HttpClient, error::Result, StringOperations };
use crate::utils;

#[tokio::test]
async fn test_string_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();
    let test_key = utils::unique_key("test_string");
    let test_value = "test_value_123";

    // Test set operation
    string_client.set(&test_key, test_value, None).await?;

    // Test get operation
    let retrieved = string_client.get(&test_key).await?;
    assert_eq!(retrieved, Some(test_value.to_string()));

    // Test set with TTL
    let ttl_key = utils::unique_key("test_string_ttl");
    string_client.set_with_ttl(&ttl_key, test_value, 3600).await?;

    // Test delete operation
    let deleted = string_client.delete(&test_key).await?;
    assert!(deleted);

    // Verify deletion
    let retrieved_after_delete = string_client.get(&test_key).await?;
    assert_eq!(retrieved_after_delete, None);

    Ok(())
}

#[tokio::test]
async fn test_string_batch_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    let keys = vec![
        utils::unique_key("batch_key1"),
        utils::unique_key("batch_key2"),
        utils::unique_key("batch_key3")
    ];

    let operations = vec![
        redis_client::StringOperation {
            key: keys[0].clone(),
            value: Some("value1".to_string()),
            ttl: Some(3600),
        },
        redis_client::StringOperation {
            key: keys[1].clone(),
            value: Some("value2".to_string()),
            ttl: None,
        },
        redis_client::StringOperation {
            key: keys[2].clone(),
            value: Some("value3".to_string()),
            ttl: Some(1800),
        }
    ];

    // Test batch set
    string_client.batch_set(&operations).await?;

    // Test batch get
    let retrieved = string_client.batch_get(&keys).await?;
    assert_eq!(retrieved.len(), 3);
    assert_eq!(retrieved[0], Some("value1".to_string()));
    assert_eq!(retrieved[1], Some("value2".to_string()));
    assert_eq!(retrieved[2], Some("value3".to_string()));

    // Clean up
    for key in &keys {
        string_client.delete(key).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_string_pattern_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    let prefix = utils::unique_key("pattern_test");
    let keys = vec![
        format!("{}_key1", prefix),
        format!("{}_key2", prefix),
        format!("{}_key3", prefix)
    ];

    // Set some test data
    for (i, key) in keys.iter().enumerate() {
        string_client.set(key, &format!("value{}", i), None).await?;
    }

    // Test pattern search
    let patterns = vec![format!("{}_*", prefix)];
    let results = string_client.get_by_patterns(&patterns, Some(true)).await?;

    // Verify we got results
    assert!(results.is_object());

    // Clean up
    for key in &keys {
        string_client.delete(key).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_string_concurrent_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;

    let test_key = utils::unique_key("concurrent_test");
    let test_value = "concurrent_value";

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            let key = format!("{}_{}", test_key, i);
            let value = format!("{}_{}", test_value, i);

            tokio::spawn(async move {
                let mut string_client = client.string();
                string_client.set(&key, &value, None).await?;
                let retrieved = string_client.get(&key).await?;
                assert_eq!(retrieved, Some(value));
                string_client.delete(&key).await?;
                Ok::<(), redis_client::error::DbxError>(())
            })
        })
        .collect();

    // Wait for all operations to complete
    for handle in handles {
        handle.await.map_err(|e| redis_client::error::DbxError::Other(anyhow::anyhow!("{}", e)))??;
    }

    Ok(())
}

#[tokio::test]
async fn test_string_large_payload() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    let test_key = utils::unique_key("large_payload");

    // Create a large payload (1MB)
    let large_value: String = "x".repeat(1024 * 1024);

    // Test setting large payload
    string_client.set(&test_key, &large_value, None).await?;

    // Test getting large payload
    let retrieved = string_client.get(&test_key).await?;
    assert_eq!(retrieved, Some(large_value));

    // Clean up
    string_client.delete(&test_key).await?;

    Ok(())
}

#[tokio::test]
async fn test_string_ttl_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    let test_key = utils::unique_key("ttl_test");
    let test_value = "ttl_value";

    // Test set with TTL
    string_client.set_with_ttl(&test_key, test_value, 5).await?;

    // Verify value is set
    let retrieved = string_client.get(&test_key).await?;
    assert_eq!(retrieved, Some(test_value.to_string()));

    // Wait for TTL to expire (in a real test, you might want to mock this)
    // For now, we'll just verify the value was set correctly
    string_client.delete(&test_key).await?;

    Ok(())
}

#[tokio::test]
async fn test_string_error_handling() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;
    let mut string_client = client.string();

    // Test operations on non-existent keys
    let non_existent_key = utils::unique_key("non_existent");
    let retrieved = string_client.get(&non_existent_key).await?;
    assert_eq!(retrieved, None);

    // Test delete on non-existent key
    let deleted = string_client.delete(&non_existent_key).await?;
    assert!(!deleted);

    Ok(())
}
