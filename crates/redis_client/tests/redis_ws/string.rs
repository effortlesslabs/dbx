//! Tests for WebSocket Redis string operations
#![cfg(feature = "websocket")]

use crate::utils;
use dbx_redis_client::{error::Result, StringOperations, WsClient};

#[tokio::test]
async fn test_websocket_string_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;
    let test_key = utils::unique_key("ws_string_test");
    let test_value = "ws_string_value_123";

    // Test set operation
    string_client.set(&test_key, test_value, None).await?;

    // Test get operation
    let retrieved = string_client.get(&test_key).await?;
    assert_eq!(retrieved, Some(test_value.to_string()));

    // Test delete operation
    let deleted = string_client.delete(&test_key).await?;
    assert!(deleted);

    // Verify deletion
    let retrieved_after_delete = string_client.get(&test_key).await?;
    assert_eq!(retrieved_after_delete, None);

    Ok(())
}

#[tokio::test]
async fn test_websocket_string_batch_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let keys = vec![
        utils::unique_key("ws_string_batch_key1"),
        utils::unique_key("ws_string_batch_key2"),
        utils::unique_key("ws_string_batch_key3"),
    ];

    let operations = vec![
        dbx_redis_client::StringOperation {
            key: keys[0].clone(),
            value: Some("ws_string_value1".to_string()),
            ttl: Some(3600),
        },
        dbx_redis_client::StringOperation {
            key: keys[1].clone(),
            value: Some("ws_string_value2".to_string()),
            ttl: None,
        },
        dbx_redis_client::StringOperation {
            key: keys[2].clone(),
            value: Some("ws_string_value3".to_string()),
            ttl: Some(1800),
        },
    ];

    // Test batch set
    string_client.batch_set(&operations).await?;

    // Test batch get
    let retrieved = string_client.batch_get(&keys).await?;
    assert_eq!(retrieved.len(), 3);
    assert_eq!(retrieved[0], Some("ws_string_value1".to_string()));
    assert_eq!(retrieved[1], Some("ws_string_value2".to_string()));
    assert_eq!(retrieved[2], Some("ws_string_value3".to_string()));

    // Clean up
    for key in &keys {
        string_client.delete(key).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_websocket_string_pattern_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let prefix = utils::unique_key("ws_string_pattern_test");
    let keys = vec![
        format!("{}_key1", prefix),
        format!("{}_key2", prefix),
        format!("{}_key3", prefix),
    ];

    // Set some test data
    for (i, key) in keys.iter().enumerate() {
        string_client
            .set(key, &format!("ws_string_value{}", i), None)
            .await?;
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
async fn test_websocket_string_concurrent_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let mut client = client.clone();
            let key = format!("ws_string_concurrent_test_{}", i);
            let value = format!("ws_string_concurrent_value_{}", i);

            tokio::spawn(async move {
                let mut string_client = client.string().await?;
                string_client.set(&key, &value, None).await?;
                let retrieved = string_client.get(&key).await?;
                assert_eq!(retrieved, Some(value));
                string_client.delete(&key).await?;
                Ok::<(), dbx_redis_client::error::DbxError>(())
            })
        })
        .collect();

    // Wait for all operations to complete
    for handle in handles {
        handle
            .await
            .map_err(|e| dbx_redis_client::error::DbxError::Other(anyhow::anyhow!("{}", e)))??;
    }

    Ok(())
}

#[tokio::test]
async fn test_websocket_string_large_payload() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_string_large_payload");

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
async fn test_websocket_string_connection_reuse() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_string_connection_test");
    let test_value = "ws_string_connection_test_value";

    // Perform multiple operations to test connection reuse
    for i in 0..5 {
        let key = format!("{}_{}", test_key, i);
        let value = format!("{}_{}", test_value, i);

        string_client.set(&key, &value, None).await?;
        let retrieved = string_client.get(&key).await?;
        assert_eq!(retrieved, Some(value));
        string_client.delete(&key).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_websocket_string_ttl_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_string_ttl_test");
    let test_value = "ws_string_ttl_value";

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
async fn test_websocket_string_error_handling() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    // Test operations on non-existent keys
    let non_existent_key = utils::unique_key("ws_string_non_existent");
    let retrieved = string_client.get(&non_existent_key).await?;
    assert_eq!(retrieved, None);

    Ok(())
}
