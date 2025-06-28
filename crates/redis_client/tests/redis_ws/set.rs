#![cfg(feature = "websocket")]

//! Tests for WebSocket Redis set operations

use redis_rs::{ WsClient, SetOperations, error::Result };
use crate::utils;

#[tokio::test]
async fn test_websocket_set_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;
    let test_key = utils::unique_key("ws_set_test");

    // Test add single member
    let added = set_client.add(&test_key, "ws_set_member1").await?;
    assert_eq!(added, 1);

    // Test add multiple members
    let added_many = set_client.add_many(
        &test_key,
        &["ws_set_member2", "ws_set_member3", "ws_set_member4"]
    ).await?;
    assert_eq!(added_many, 3);

    // Test cardinality
    let cardinality = set_client.cardinality(&test_key).await?;
    assert_eq!(cardinality, 4);

    // Test members
    let members = set_client.members(&test_key).await?;
    assert_eq!(members.len(), 4);
    assert!(members.contains(&"ws_set_member1".to_string()));
    assert!(members.contains(&"ws_set_member2".to_string()));
    assert!(members.contains(&"ws_set_member3".to_string()));
    assert!(members.contains(&"ws_set_member4".to_string()));

    // Test exists
    let exists = set_client.exists(&test_key, "ws_set_member1").await?;
    assert!(exists);

    let not_exists = set_client.exists(&test_key, "nonexistent").await?;
    assert!(!not_exists);

    // Test remove
    let removed = set_client.remove(&test_key, "ws_set_member1").await?;
    assert_eq!(removed, 1);

    // Verify removal
    let exists_after_remove = set_client.exists(&test_key, "ws_set_member1").await?;
    assert!(!exists_after_remove);

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_operations_multiple_sets() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;

    let set1_key = utils::unique_key("ws_set_set1");
    let set2_key = utils::unique_key("ws_set_set2");
    let set3_key = utils::unique_key("ws_set_set3");

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
async fn test_websocket_set_concurrent_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let mut client = client.clone();
            let key = format!("ws_set_concurrent_test_{}", i);
            let member = format!("ws_set_concurrent_member_{}", i);

            tokio::spawn(async move {
                let mut set_client = client.set().await?;
                set_client.add(&key, &member).await?;
                let exists = set_client.exists(&key, &member).await?;
                assert!(exists);
                set_client.remove(&key, &member).await?;
                Ok::<(), redis_rs::error::DbxError>(())
            })
        })
        .collect();

    // Wait for all operations to complete
    for handle in handles {
        handle.await.map_err(|e| redis_rs::error::DbxError::Other(anyhow::anyhow!("{}", e)))??;
    }

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_large_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;
    let test_key = utils::unique_key("ws_set_large_test");

    // Add many members
    let members: Vec<String> = (0..1000).map(|i| format!("ws_set_large_member_{}", i)).collect();
    let member_refs: Vec<&str> = members
        .iter()
        .map(|s| s.as_str())
        .collect();
    let added = set_client.add_many(&test_key, &member_refs).await?;
    assert_eq!(added, 1000);

    // Verify cardinality
    let cardinality = set_client.cardinality(&test_key).await?;
    assert_eq!(cardinality, 1000);

    // Verify all members exist
    for member in &member_refs {
        let exists = set_client.exists(&test_key, member).await?;
        assert!(exists);
    }

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_duplicate_handling() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;
    let test_key = utils::unique_key("ws_set_duplicate_test");
    let member = "ws_set_duplicate_member";

    // Add the same member twice
    let added1 = set_client.add(&test_key, member).await?;
    assert_eq!(added1, 1);

    let added2 = set_client.add(&test_key, member).await?;
    assert_eq!(added2, 0); // Should not add duplicate

    // Verify only one instance exists
    let cardinality = set_client.cardinality(&test_key).await?;
    assert_eq!(cardinality, 1);

    let members = set_client.members(&test_key).await?;
    assert_eq!(members.len(), 1);
    assert!(members.contains(&member.to_string()));

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_empty_operations() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;
    let test_key = utils::unique_key("ws_set_empty_test");

    // Test operations on empty set
    let cardinality = set_client.cardinality(&test_key).await?;
    assert_eq!(cardinality, 0);

    let members = set_client.members(&test_key).await?;
    assert_eq!(members.len(), 0);

    let exists = set_client.exists(&test_key, "any_member").await?;
    assert!(!exists);

    let removed = set_client.remove(&test_key, "any_member").await?;
    assert_eq!(removed, 0);

    Ok(())
}

#[tokio::test]
async fn test_websocket_set_error_handling() -> Result<()> {
    let mut client = WsClient::new(&utils::ws_test_url()).await?;
    let mut set_client = client.set().await?;

    // Test operations on non-existent keys
    let non_existent_key = utils::unique_key("ws_set_non_existent");
    let cardinality = set_client.cardinality(&non_existent_key).await?;
    assert_eq!(cardinality, 0);

    let members = set_client.members(&non_existent_key).await?;
    assert_eq!(members.len(), 0);

    Ok(())
}
