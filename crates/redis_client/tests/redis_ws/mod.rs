#![cfg(feature = "websocket")]
//! Tests for WebSocket Redis client functionality

use redis_client::{ WsClient, StringOperations, SetOperations, error::Result };
use crate::utils;

// Import string and set test modules
pub mod string;
pub mod set;

#[tokio::test]
async fn test_websocket_client_creation() -> Result<()> {
    let client = WsClient::new(&utils::ws_test_url()).await?;
    assert_eq!(client.base_url().as_str(), utils::ws_test_url());
    Ok(())
}

#[tokio::test]
async fn test_websocket_client_with_timeout() -> Result<()> {
    let timeout = std::time::Duration::from_secs(60);
    let client = WsClient::with_timeout(&utils::ws_test_url(), timeout).await?;
    assert_eq!(client.base_url().as_str(), utils::ws_test_url());
    Ok(())
}

#[tokio::test]
async fn test_websocket_client_clone() -> Result<()> {
    let client1 = WsClient::new(&utils::ws_test_url()).await?;
    let client2 = client1.clone();
    assert_eq!(client1.base_url(), client2.base_url());
    Ok(())
}

#[tokio::test]
async fn test_websocket_string_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;
    let test_key = utils::unique_key("ws_test_string");
    let test_value = "ws_test_value_123";

    // Test set operation
    string_client.set(&test_key, test_value, None).await?;

    // Test get operation
    let retrieved = string_client.get(&test_key).await?;
    assert_eq!(retrieved, Some(test_value.to_string()));

    // Test set with TTL
    let ttl_key = utils::unique_key("ws_test_string_ttl");
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
async fn test_websocket_string_batch_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let keys = vec![
        utils::unique_key("ws_batch_key1"),
        utils::unique_key("ws_batch_key2"),
        utils::unique_key("ws_batch_key3")
    ];

    let operations = vec![
        redis_client::StringOperation {
            key: keys[0].clone(),
            value: Some("ws_value1".to_string()),
            ttl: Some(3600),
        },
        redis_client::StringOperation {
            key: keys[1].clone(),
            value: Some("ws_value2".to_string()),
            ttl: None,
        },
        redis_client::StringOperation {
            key: keys[2].clone(),
            value: Some("ws_value3".to_string()),
            ttl: Some(1800),
        }
    ];

    // Test batch set
    string_client.batch_set(&operations).await?;

    // Test batch get
    let retrieved = string_client.batch_get(&keys).await?;
    assert_eq!(retrieved.len(), 3);
    assert_eq!(retrieved[0], Some("ws_value1".to_string()));
    assert_eq!(retrieved[1], Some("ws_value2".to_string()));
    assert_eq!(retrieved[2], Some("ws_value3".to_string()));

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

    let prefix = utils::unique_key("ws_pattern_test");
    let keys = vec![
        format!("{}_key1", prefix),
        format!("{}_key2", prefix),
        format!("{}_key3", prefix)
    ];

    // Set some test data
    for (i, key) in keys.iter().enumerate() {
        string_client.set(key, &format!("ws_value{}", i), None).await?;
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
async fn test_websocket_set_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;
    let test_key = utils::unique_key("ws_test_set");

    // Test add single member
    let added = set_client.add(&test_key, "ws_member1").await?;
    assert_eq!(added, 1);

    // Test add multiple members
    let added_many = set_client.add_many(
        &test_key,
        &["ws_member2", "ws_member3", "ws_member4"]
    ).await?;
    assert_eq!(added_many, 3);

    // Test cardinality
    let cardinality = set_client.cardinality(&test_key).await?;
    assert_eq!(cardinality, 4);

    // Test members
    let members = set_client.members(&test_key).await?;
    assert_eq!(members.len(), 4);
    assert!(members.contains(&"ws_member1".to_string()));
    assert!(members.contains(&"ws_member2".to_string()));
    assert!(members.contains(&"ws_member3".to_string()));
    assert!(members.contains(&"ws_member4".to_string()));

    // Test exists
    let exists = set_client.exists(&test_key, "ws_member1").await?;
    assert!(exists);

    let not_exists = set_client.exists(&test_key, "nonexistent").await?;
    assert!(!not_exists);

    // Test remove
    let removed = set_client.remove(&test_key, "ws_member1").await?;
    assert_eq!(removed, 1);

    // Verify removal
    let exists_after_remove = set_client.exists(&test_key, "ws_member1").await?;
    assert!(!exists_after_remove);

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_operations_multiple_sets() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;

    let set1_key = utils::unique_key("ws_set1");
    let set2_key = utils::unique_key("ws_set2");
    let set3_key = utils::unique_key("ws_set3");

    // Populate set1: {a, b, c, d}
    set_client.add_many(&set1_key, &["a", "b", "c", "d"]).await?;

    // Populate set2: {b, c, e, f}
    set_client.add_many(&set2_key, &["b", "c", "e", "f"]).await?;

    // Populate set3: {c, d, g, h}
    set_client.add_many(&set3_key, &["c", "d", "g", "h"]).await?;

    let keys = vec![set1_key.clone(), set2_key.clone(), set3_key.clone()];

    // Test intersection: {c}
    let intersection = set_client.intersect(&keys).await?;
    assert_eq!(intersection.len(), 1);
    assert!(intersection.contains(&"c".to_string()));

    // Test union: {a, b, c, d, e, f, g, h}
    let union = set_client.union(&keys).await?;
    assert_eq!(union.len(), 8);
    assert!(union.contains(&"a".to_string()));
    assert!(union.contains(&"b".to_string()));
    assert!(union.contains(&"c".to_string()));
    assert!(union.contains(&"d".to_string()));
    assert!(union.contains(&"e".to_string()));
    assert!(union.contains(&"f".to_string()));
    assert!(union.contains(&"g".to_string()));
    assert!(union.contains(&"h".to_string()));

    // Test difference (set1 - set2 - set3): {a}
    let difference = set_client.difference(&keys).await?;
    assert_eq!(difference.len(), 1);
    assert!(difference.contains(&"a".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_websocket_connection_reuse() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_connection_test");
    let test_value = "connection_test_value";

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
async fn test_websocket_error_handling() -> Result<()> {
    // Test with invalid WebSocket URL (should fail gracefully)
    let invalid_client = WsClient::new("ws://invalid-url-that-does-not-exist.com");
    assert!(invalid_client.await.is_err());

    // Test operations on non-existent keys
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let non_existent_key = utils::unique_key("ws_non_existent");
    let retrieved = string_client.get(&non_existent_key).await?;
    assert_eq!(retrieved, None);

    Ok(())
}

#[tokio::test]
async fn test_websocket_concurrent_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let mut client = client.clone();
            let key = format!("ws_concurrent_test_{}", i);
            let value = format!("concurrent_value_{}", i);

            tokio::spawn(async move {
                let mut string_client = client.string().await?;
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
async fn test_websocket_large_payload() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_large_payload");

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
async fn test_websocket_ttl_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut string_client = client.string().await?;

    let test_key = utils::unique_key("ws_ttl_test");
    let test_value = "ws_ttl_value";

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
