use crate::common::{
    assert_status_ok,
    assert_status_not_found,
    assert_status_method_not_allowed,
    set_string,
    get_string,
    delete_string,
    generate_test_key,
    generate_test_value,
    generate_large_value,
    generate_special_chars_value,
    create_http_client,
    TestContext,
};
use crate::get_test_base_url;
use serde_json::json;

#[tokio::test]
async fn test_set_get_string_basic() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("basic", None);
    let test_value = generate_test_value("value", None);

    ctx.add_test_key(test_key.clone());

    // Set string
    set_string(&ctx.client, &ctx.base_url, &test_key, &test_value).await.unwrap();

    // Get string
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(test_value));

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_set_get_string_with_special_chars() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("special", None);
    let test_value = generate_special_chars_value();

    ctx.add_test_key(test_key.clone());

    // Set string with special characters
    set_string(&ctx.client, &ctx.base_url, &test_key, &test_value).await.unwrap();

    // Get string
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(test_value));

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_set_get_large_string() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("large", None);
    let test_value = generate_large_value(10000); // 10KB string

    ctx.add_test_key(test_key.clone());

    // Set large string
    set_string(&ctx.client, &ctx.base_url, &test_key, &test_value).await.unwrap();

    // Get large string
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(test_value));

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_nonexistent_string() {
    let ctx = TestContext::new(get_test_base_url().await);
    let nonexistent_key = generate_test_key("nonexistent", None);

    // Try to get nonexistent string
    let result = get_string(&ctx.client, &ctx.base_url, &nonexistent_key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_delete_string() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("delete", None);
    let test_value = generate_test_value("value", None);

    ctx.add_test_key(test_key.clone());

    // Set string
    set_string(&ctx.client, &ctx.base_url, &test_key, &test_value).await.unwrap();

    // Verify it exists
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(test_value));

    // Delete string
    let deleted = delete_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert!(deleted);

    // Verify it's gone
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_delete_nonexistent_string() {
    let ctx = TestContext::new(get_test_base_url().await);
    let nonexistent_key = generate_test_key("nonexistent", None);

    // Try to delete nonexistent string
    let deleted = delete_string(&ctx.client, &ctx.base_url, &nonexistent_key).await.unwrap();
    assert!(!deleted);
}

#[tokio::test]
async fn test_string_overwrite() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("overwrite", None);
    let value1 = generate_test_value("value1", None);
    let value2 = generate_test_value("value2", None);

    ctx.add_test_key(test_key.clone());

    // Set initial value
    set_string(&ctx.client, &ctx.base_url, &test_key, &value1).await.unwrap();
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(value1.clone()));

    // Overwrite with new value
    set_string(&ctx.client, &ctx.base_url, &test_key, &value2).await.unwrap();
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(value2));

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_concurrent_string_operations() {
    let base_url = get_test_base_url().await;
    let client = create_http_client();
    let mut handles = vec![];

    // Spawn multiple concurrent operations
    for i in 0..10 {
        let client = client.clone();
        let base_url = base_url.clone();
        let test_key = generate_test_key("concurrent", Some(i));
        let test_value = generate_test_value("value", Some(i));

        let handle = tokio::spawn(async move {
            // Set string
            set_string(&client, &base_url, &test_key, &test_value).await.unwrap();

            // Get string
            let result = get_string(&client, &base_url, &test_key).await.unwrap();
            assert_eq!(result, Some(test_value));

            // Delete string
            let deleted = delete_string(&client, &base_url, &test_key).await.unwrap();
            assert!(deleted);
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_string_operations_with_ttl() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let test_key = generate_test_key("ttl", None);
    let test_value = generate_test_value("value", None);

    ctx.add_test_key(test_key.clone());

    // Set string with TTL
    let res = ctx.client
        .post(&format!("{}/redis/string/{}", ctx.base_url, test_key))
        .json(
            &json!({
            "value": test_value,
            "ttl": 1 // 1 second TTL
        })
        )
        .send().await
        .unwrap();

    assert_status_ok(res.status().as_u16());

    // Verify it exists immediately
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, Some(test_value));

    // Wait for TTL to expire
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Verify it's expired
    let result = get_string(&ctx.client, &ctx.base_url, &test_key).await.unwrap();
    assert_eq!(result, None);
}

#[tokio::test]
async fn test_batch_string_operations() {
    let mut ctx = TestContext::new(get_test_base_url().await);
    let operations: Vec<(&str, &str)> = vec![
        ("batch_key_1", "batch_value_1"),
        ("batch_key_2", "batch_value_2"),
        ("batch_key_3", "batch_value_3")
    ];

    // Add keys for cleanup
    for (key, _) in &operations {
        ctx.add_test_key(key.to_string());
    }

    // Batch set strings
    let batch_ops: Vec<serde_json::Value> = operations
        .iter()
        .map(|(key, value)| json!({"key": key, "value": value}))
        .collect();

    let res = ctx.client
        .post(&format!("{}/redis/string/batch/set", ctx.base_url))
        .json(&json!({"operations": batch_ops}))
        .send().await
        .unwrap();

    assert_status_ok(res.status().as_u16());

    // Batch get strings
    let keys: Vec<String> = operations
        .iter()
        .map(|(key, _)| key.to_string())
        .collect();
    let res = ctx.client
        .post(&format!("{}/redis/string/batch/get", ctx.base_url))
        .json(&json!({"keys": keys}))
        .send().await
        .unwrap();

    assert_status_ok(res.status().as_u16());
    let values: Vec<Option<String>> = res.json().await.unwrap();

    // Verify all values
    for (i, (_, expected_value)) in operations.iter().enumerate() {
        assert_eq!(values[i], Some(expected_value.to_string()));
    }

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_string_error_handling() {
    let ctx = TestContext::new(get_test_base_url().await);

    // Test invalid endpoint
    let res = ctx.client.get(&format!("{}/redis/invalid", ctx.base_url)).send().await.unwrap();
    assert_status_not_found(res.status().as_u16());

    // Test unsupported HTTP method (PUT) for string endpoint
    let res = ctx.client
        .put(&format!("{}/redis/string/test_key", ctx.base_url))
        .send().await
        .unwrap();
    assert_status_method_not_allowed(res.status().as_u16());

    // Test unsupported HTTP method (PATCH) for string endpoint
    let res = ctx.client
        .patch(&format!("{}/redis/string/test_key", ctx.base_url))
        .send().await
        .unwrap();
    assert_status_method_not_allowed(res.status().as_u16());
}
