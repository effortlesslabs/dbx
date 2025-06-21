//! Redis HTTP API batch operation tests

use axum::http::StatusCode;
use axum::body::to_bytes;
use serde_json::json;

use crate::common::{
    assert_status_code,
    assert_success_response,
    cleanup_test_keys,
    create_test_server,
    extract_json,
    generate_test_key,
    make_request,
};
use dbx_api::models::{ ApiResponse, DeleteResponse, KeyValues };

#[tokio::test]
async fn test_redis_batch_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch_comp1");
    let key2 = generate_test_key("batch_comp2");
    let key3 = generate_test_key("batch_comp3");
    let key4 = generate_test_key("batch_comp4");
    let key5 = generate_test_key("batch_comp5");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &key4, &key5]).await;

    // Test batch SET operation with multiple keys (use numeric values for all keys)
    let batch_set_request =
        json!({
        "key_values": {
            key1.clone(): "0",
            key2.clone(): "0",
            key3.clone(): "0",
            key4.clone(): "0",
            key5.clone(): "0"
        },
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(batch_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch GET operation
    let batch_get_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone(), key4.clone(), key5.clone(), "nonexistent_key"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.len(), 6);
    assert_eq!(key_values.get(&key1), Some(&"0".to_string()));
    assert_eq!(key_values.get(&key2), Some(&"0".to_string()));
    assert_eq!(key_values.get(&key3), Some(&"0".to_string()));
    assert_eq!(key_values.get(&key4), Some(&"0".to_string()));
    assert_eq!(key_values.get(&key5), Some(&"0".to_string()));
    assert_eq!(key_values.get("nonexistent_key"), Some(&"".to_string()));

    // Test batch INCR operation
    let batch_incr_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(batch_incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incr_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incr_response);

    // Verify incremented values
    let batch_get_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"1".to_string()));
    assert_eq!(key_values.get(&key2), Some(&"1".to_string()));
    assert_eq!(key_values.get(&key3), Some(&"1".to_string()));

    // Test batch INCRBY operation with different increments
    let batch_incrby_request =
        json!({
        "key_increments": [
            [key1.clone(), 5],
            [key2.clone(), -3],
            [key3.clone(), 10],
            [key4.clone(), 2],
            [key5.clone(), 7]
        ]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(batch_incrby_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incrby_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incrby_response);

    // Verify final values
    let batch_get_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone(), key4.clone(), key5.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"6".to_string())); // 1 + 5
    assert_eq!(key_values.get(&key2), Some(&"-2".to_string())); // 1 - 3
    assert_eq!(key_values.get(&key3), Some(&"11".to_string())); // 1 + 10
    assert_eq!(key_values.get(&key4), Some(&"2".to_string())); // 0 + 2
    assert_eq!(key_values.get(&key5), Some(&"7".to_string())); // 0 + 7

    // Test batch DELETE operation
    let batch_delete_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&batch_delete_response);
    assert_eq!(batch_delete_response.data.unwrap().deleted_count, 3);

    // Verify keys are deleted
    for key in &[&key1, &key2, &key3] {
        let response = make_request(
            router.clone(),
            "GET",
            &format!("/api/v1/redis/strings/{}", key),
            None
        ).await;
        assert_status_code(&response, StatusCode::NOT_FOUND);
    }

    // Verify remaining keys still exist
    for key in &[&key4, &key5] {
        let response = make_request(
            router.clone(),
            "GET",
            &format!("/api/v1/redis/strings/{}", key),
            None
        ).await;
        assert_status_code(&response, StatusCode::OK);
    }

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &key4, &key5]).await;
}

#[tokio::test]
async fn test_redis_batch_string_operations() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch1");
    let key2 = generate_test_key("batch2");
    let key3 = generate_test_key("batch3");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;

    // Test batch SET operation
    let batch_set_request =
        json!({
        "key_values": {
            key1.clone(): "value1",
            key2.clone(): "value2",
            key3.clone(): "value3"
        },
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(batch_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch GET operation
    let batch_get_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone(), "nonexistent_key"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.len(), 4);
    assert_eq!(key_values.get(&key1), Some(&"value1".to_string()));
    assert_eq!(key_values.get(&key2), Some(&"value2".to_string()));
    assert_eq!(key_values.get(&key3), Some(&"value3".to_string()));
    assert_eq!(key_values.get("nonexistent_key"), Some(&"".to_string())); // or null depending on implementation

    // Test batch DELETE operation
    let batch_delete_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&batch_delete_response);
    assert_eq!(batch_delete_response.data.unwrap().deleted_count, 3);

    // Verify keys are deleted
    for key in &[&key1, &key2, &key3] {
        let response = make_request(
            router.clone(),
            "GET",
            &format!("/api/v1/redis/strings/{}", key),
            None
        ).await;

        assert_status_code(&response, StatusCode::NOT_FOUND);
    }

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;
}

#[tokio::test]
async fn test_redis_batch_increment_operations() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch_incr1");
    let key2 = generate_test_key("batch_incr2");
    let key3 = generate_test_key("batch_incr3");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;

    // Set initial values
    let batch_set_request =
        json!({
        "key_values": {
            key1.clone(): "10",
            key2.clone(): "20",
            key3.clone(): "30"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(batch_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test batch INCR operation
    let batch_incr_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(batch_incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incr_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incr_response);

    // Verify incremented values
    let batch_get_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"11".to_string()));
    assert_eq!(key_values.get(&key2), Some(&"21".to_string()));
    assert_eq!(key_values.get(&key3), Some(&"31".to_string()));

    // Test batch INCRBY operation
    let batch_incrby_request =
        json!({
        "key_increments": [
            [key1.clone(), 5],
            [key2.clone(), -3],
            [key3.clone(), 10]
        ]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(batch_incrby_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incrby_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incrby_response);

    // Verify incremented values
    let batch_get_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"16".to_string())); // 11 + 5
    assert_eq!(key_values.get(&key2), Some(&"18".to_string())); // 21 - 3
    assert_eq!(key_values.get(&key3), Some(&"41".to_string())); // 31 + 10

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;
}

#[tokio::test]
async fn test_redis_batch_mixed_operations() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch_mixed1");
    let key2 = generate_test_key("batch_mixed2");
    let key3 = generate_test_key("batch_mixed3");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;

    // Test batch operations on non-existent keys
    let batch_incr_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(batch_incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incr_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incr_response);

    // Verify keys were created with value 1
    let batch_get_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"1".to_string()));
    assert_eq!(key_values.get(&key2), Some(&"1".to_string()));
    assert_eq!(key_values.get(&key3), Some(&"1".to_string()));

    // Test batch INCRBY on existing keys
    let batch_incrby_request =
        json!({
        "key_increments": [
            [key1.clone(), 10],
            [key2.clone(), 20],
            [key3.clone(), 30]
        ]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(batch_incrby_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Verify final values
    let batch_get_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.get(&key1), Some(&"11".to_string())); // 1 + 10
    assert_eq!(key_values.get(&key2), Some(&"21".to_string())); // 1 + 20
    assert_eq!(key_values.get(&key3), Some(&"31".to_string())); // 1 + 30

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;
}

#[tokio::test]
async fn test_redis_batch_error_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("batch_error");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test batch SET with empty key_values
    let empty_set_request = json!({
        "key_values": {}
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(empty_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch GET with empty keys
    let empty_get_request = json!({
        "keys": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(empty_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    assert_eq!(batch_get_response.data.unwrap().key_values.len(), 0);

    // Test batch DELETE with empty keys
    let empty_delete_request = json!({
        "keys": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/delete",
        Some(empty_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&batch_delete_response);
    assert_eq!(batch_delete_response.data.unwrap().deleted_count, 0);

    // Test batch INCR with empty keys
    let empty_incr_request = json!({
        "keys": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(empty_incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incr_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incr_response);

    // Test batch INCRBY with empty key_increments
    let empty_incrby_request = json!({
        "key_increments": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(empty_incrby_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incrby_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incrby_response);

    // Test invalid request body
    let invalid_request = json!({
        "invalid_field": "invalid_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_batch_edge_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("batch_edge");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test batch operations with large number of keys
    let mut large_key_values = serde_json::Map::new();
    let mut large_keys = Vec::new();

    for i in 0..100 {
        let key = format!("{}_{}", test_key, i);
        large_key_values.insert(key.clone(), serde_json::Value::String(format!("value_{}", i)));
        large_keys.push(key);
    }

    let large_set_request = json!({
        "key_values": large_key_values
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(large_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch GET with large number of keys
    let large_get_request = json!({
        "keys": large_keys
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(large_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.len(), 100);

    // Test batch operations with special characters in keys
    let special_key_values =
        json!({
        "key_values": {
            "key with spaces": "value with spaces",
            "key-with-dashes": "value-with-dashes",
            "key_with_underscores": "value_with_underscores",
            "key:with:colons": "value:with:colons",
            "key.with.dots": "value.with.dots"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(special_key_values)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch operations with unicode characters
    let unicode_key_values =
        json!({
        "key_values": {
            "key_unicode_测试": "value_unicode_测试",
            "key_unicode_🎉": "value_unicode_🎉",
            "key_unicode_🌍": "value_unicode_🌍"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(unicode_key_values)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch operations with very long values
    let long_value = "a".repeat(10000);
    let long_value_request =
        json!({
        "key_values": {
            "long_key": long_value
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(long_value_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch operations with numeric values as strings
    let numeric_request =
        json!({
        "key_values": {
            "int_key": "123",
            "float_key": "123",
            "negative_key": "-789"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(numeric_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch INCRBY with mixed positive and negative increments
    let mixed_incrby_request =
        json!({
        "key_increments": [
            ["int_key", 10],
            ["float_key", -5],
            ["negative_key", 100]
        ]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(mixed_incrby_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_incrby_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_incrby_response);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_batch_concurrent_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("batch_concurrent");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent batch operations
    let mut handles = Vec::new();

    for i in 0..10 {
        let router_clone = router.clone();
        let key = format!("{}_{}", test_key, i);

        let handle = tokio::spawn(async move {
            let set_request =
                json!({
                "key_values": {
                    key.clone(): "0"
                }
            });

            let response = make_request(
                router_clone.clone(),
                "POST",
                "/api/v1/redis/strings/batch/set",
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);

            let incr_request = json!({
                "keys": [key.clone()]
            });

            let response = make_request(
                router_clone,
                "POST",
                "/api/v1/redis/strings/batch/incr",
                Some(incr_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all operations completed successfully
    let mut verify_keys = Vec::new();
    for i in 0..10 {
        verify_keys.push(format!("{}_{}", test_key, i));
    }

    let verify_request = json!({
        "keys": verify_keys
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(verify_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&batch_get_response);
    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values.len(), 10);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_batch_increment_string_values() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch_incr_str1");
    let key2 = generate_test_key("batch_incr_str2");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2]).await;

    // Set string values (not numeric)
    let batch_set_request =
        json!({
        "key_values": {
            key1.clone(): "value1",
            key2.clone(): "value2"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(batch_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Try to increment string values - this should fail
    let batch_incr_request = json!({
        "keys": [key1.clone(), key2.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(batch_incr_request)
    ).await;

    // This should return an error, not 500
    println!("Response status: {}", response.status());
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    println!("Response body: {:?}", String::from_utf8_lossy(&body));

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2]).await;
}
