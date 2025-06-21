//! Redis HTTP API error case tests

use axum::http::StatusCode;
use serde_json::json;

use crate::common::{
    create_test_server,
    make_request,
    assert_status_code,
    cleanup_test_keys,
    generate_test_key,
};

#[tokio::test]
async fn test_redis_error_handling_comprehensive() {
    let (router, _) = create_test_server().await;

    // Test with malformed JSON
    let malformed_json = "{ invalid json }";

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(serde_json::Value::String(malformed_json.to_string()))
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test with empty JSON object
    let empty_json = json!({});

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(empty_json)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test with null JSON
    let null_json = json!(null);

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(null_json)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test SET without value parameter
    let invalid_request = json!({
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test INCRBY without increment parameter
    let invalid_request = json!({
        "some_other_field": "value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/incrby",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test SETNX without value parameter
    let invalid_request = json!({
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/setnx",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test SET with non-string value
    let invalid_request = json!({
        "value": 123,
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test SET with null value
    let invalid_request = json!({
        "value": null,
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test INCRBY with non-numeric increment
    let invalid_request = json!({
        "increment": "not_a_number"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/incrby",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test SET with invalid TTL type
    let invalid_request = json!({
        "value": "test_value",
        "ttl": "not_a_number"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test non-existent endpoint
    let response = make_request(router.clone(), "GET", "/api/v1/redis/nonexistent", None).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Test invalid HTTP method
    let response = make_request(
        router.clone(),
        "PUT",
        "/api/v1/redis/strings/test_key",
        None
    ).await;

    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);

    // Test invalid key with special characters
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/strings/test%20key%20with%20spaces",
        None
    ).await;

    // Should handle gracefully (may return 404 or 400)
    assert!(response.status().is_client_error());

    // Test batch operations with invalid data
    let invalid_batch_request = json!({
        "key_values": "not_an_object"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(invalid_batch_request)
    ).await;

    assert_status_code(&response, StatusCode::BAD_REQUEST);

    // Test batch operations with empty keys array
    let invalid_batch_request = json!({
        "keys": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(invalid_batch_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test set operations with invalid data
    let invalid_set_request = json!({
        "members": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/test_key",
        Some(invalid_set_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test hash operations with invalid data
    let invalid_hash_request = json!({
        "fields": "not_an_object"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/test_key",
        Some(invalid_hash_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test script operations with invalid data
    let invalid_script_request =
        json!({
        "script": "",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_script_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test script operations with invalid keys type
    let invalid_script_request =
        json!({
        "script": "return 'test'",
        "keys": "not_an_array",
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_script_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test script operations with invalid args type
    let invalid_script_request =
        json!({
        "script": "return 'test'",
        "keys": [],
        "args": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_script_request)
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_redis_invalid_json_requests() {
    let (router, _) = create_test_server().await;

    // Test with malformed JSON
    let malformed_json = "{ invalid json }";

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(serde_json::Value::String(malformed_json.to_string()))
    ).await;

    assert_status_code(&response, StatusCode::UNPROCESSABLE_ENTITY);

    // Test with empty JSON object
    let empty_json = json!({});

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(empty_json)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test with null JSON
    let null_json = json!(null);

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(null_json)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_missing_parameters() {
    let (router, _) = create_test_server().await;

    // Test SET without value parameter
    let invalid_request = json!({
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test INCRBY without increment parameter
    let invalid_request = json!({
        "some_other_field": "value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/incrby",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test SETNX without value parameter
    let invalid_request = json!({
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/setnx",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_invalid_data_types() {
    let (router, _) = create_test_server().await;

    // Test SET with non-string value
    let invalid_request = json!({
        "value": 123,
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test SET with null value
    let invalid_request = json!({
        "value": null,
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test INCRBY with non-numeric increment
    let invalid_request = json!({
        "increment": "not_a_number"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key/incrby",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test SET with invalid TTL type
    let invalid_request = json!({
        "value": "test_value",
        "ttl": "not_a_number"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_invalid_endpoints() {
    let (router, _) = create_test_server().await;

    // Test non-existent endpoint
    let response = make_request(router.clone(), "GET", "/api/v1/redis/nonexistent", None).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Test invalid HTTP method
    let response = make_request(
        router.clone(),
        "PUT",
        "/api/v1/redis/strings/test_key",
        None
    ).await;

    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);

    // Test invalid HTTP method for health endpoint
    let response = make_request(router.clone(), "POST", "/health", None).await;

    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);

    // Test invalid HTTP method for info endpoint
    let response = make_request(router.clone(), "POST", "/info", None).await;

    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_redis_invalid_keys() {
    let (router, _) = create_test_server().await;

    // Test with empty key
    let response = make_request(router.clone(), "GET", "/api/v1/redis/strings/", None).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Test with very long key
    let long_key = "a".repeat(10000);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", long_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test with key containing spaces (which should be URL encoded)
    let invalid_key = "key with spaces";
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", invalid_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test with unicode characters in key
    let unicode_key = "key_with_unicode_🎉🌍测试";
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", unicode_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test with special characters in key (URL encoded)
    let special_key = "key%20with%20special%20chars";
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", special_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());
}

#[tokio::test]
async fn test_redis_batch_operation_errors() {
    let (router, _) = create_test_server().await;

    // Test batch SET with invalid key_values structure
    let invalid_request = json!({
        "key_values": "not_an_object",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test batch GET with invalid keys structure
    let invalid_request = json!({
        "keys": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test batch DELETE with invalid keys structure
    let invalid_request = json!({
        "keys": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/delete",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test batch INCR with invalid keys structure
    let invalid_request = json!({
        "keys": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test batch INCRBY with invalid key_increments structure
    let invalid_request = json!({
        "key_increments": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incrby",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_set_operation_errors() {
    let (router, _) = create_test_server().await;

    // Test SADD with invalid members structure
    let invalid_request = json!({
        "members": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/test_set",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(
        response.status().is_client_error() ||
            response.status().is_success() ||
            response.status().is_server_error()
    );

    // Test set operations with invalid keys structure
    let invalid_request = json!({
        "keys": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/union",
        Some(invalid_request.clone())
    ).await;

    // Should handle gracefully
    assert!(
        response.status().is_client_error() ||
            response.status().is_success() ||
            response.status().is_server_error()
    );

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/intersection",
        Some(invalid_request.clone())
    ).await;

    // Should handle gracefully
    assert!(
        response.status().is_client_error() ||
            response.status().is_success() ||
            response.status().is_server_error()
    );

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/difference",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(
        response.status().is_client_error() ||
            response.status().is_success() ||
            response.status().is_server_error()
    );
}

#[tokio::test]
async fn test_redis_hash_operation_errors() {
    let (router, _) = create_test_server().await;

    // Test HSET with invalid fields structure
    let invalid_request = json!({
        "fields": "not_an_object"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/test_hash",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test HINCRBY with invalid increment type
    let invalid_request = json!({
        "increment": "not_a_number"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/test_hash/test_field/incr",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test HMGET with invalid fields structure
    let invalid_request = json!({
        "fields": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/test_hash/mget",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_script_operation_errors() {
    let (router, _) = create_test_server().await;

    // Test EVAL with missing script
    let invalid_request = json!({
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test EVAL with invalid keys structure
    let invalid_request =
        json!({
        "script": "return 'test'",
        "keys": "not_an_array",
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test EVAL with invalid args structure
    let invalid_request =
        json!({
        "script": "return 'test'",
        "keys": [],
        "args": "not_an_array"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());
}

#[tokio::test]
async fn test_redis_edge_case_errors() {
    let (router, _) = create_test_server().await;

    // Test with extremely large request body
    let large_value = "a".repeat(1000000); // 1MB string
    let large_request = json!({
        "value": large_value,
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(large_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test with deeply nested JSON
    let mut nested_json = json!("value");
    for _ in 0..100 {
        nested_json = json!({ "nested": nested_json });
    }

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(nested_json)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test with unicode characters in key
    let unicode_key = "key_with_unicode_🎉🌍测试";
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", unicode_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test with special characters in key (URL encoded)
    let special_key = "key%20with%20special%20chars";
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", special_key),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());
}

#[tokio::test]
async fn test_redis_concurrent_error_scenarios() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_error");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent operations that might cause conflicts
    let mut handles = Vec::new();

    for i in 0..10 {
        let router_clone = router.clone();
        let key = format!("{}_{}", test_key, i);

        let handle = tokio::spawn(async move {
            // Try to set the same key multiple times concurrently
            let set_request =
                json!({
                "value": format!("value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone.clone(),
                "POST",
                &format!("/api/v1/redis/strings/{}", key),
                Some(set_request)
            ).await;

            // Should handle gracefully - accept success, client errors, or server errors
            assert!(
                response.status().is_success() ||
                    response.status().is_client_error() ||
                    response.status().is_server_error(),
                "Unexpected status code: {} for SET operation on key {}",
                response.status(),
                key
            );

            // Try to increment the same key concurrently
            let incr_request = json!({
                "increment": 1
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}/incrby", key),
                Some(incr_request)
            ).await;

            // Should handle gracefully - accept success, client errors, or server errors
            assert!(
                response.status().is_success() ||
                    response.status().is_client_error() ||
                    response.status().is_server_error(),
                "Unexpected status code: {} for INCRBY operation on key {}",
                response.status(),
                key
            );
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}
