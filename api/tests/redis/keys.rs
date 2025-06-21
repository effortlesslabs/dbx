//! Redis HTTP API key operation tests

use axum::http::StatusCode;
use serde_json::json;

use crate::common::{
    create_test_server,
    make_request,
    extract_json,
    assert_status_code,
    assert_success_response,
    assert_error_response,
    cleanup_test_keys,
    generate_test_key,
};
use dbx_api::models::{
    ApiResponse,
    StringValue,
    IntegerValue,
    BooleanValue,
    ExistsResponse,
    DeleteResponse,
    TtlResponse,
    KeysResponse,
};

#[tokio::test]
async fn test_redis_key_basic_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("key_basic");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test key doesn't exist initially
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test TTL on non-existent key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -2); // -2 means key doesn't exist

    // Create a key with TTL
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test key exists
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test TTL on existing key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert!(ttl_response.data.unwrap().ttl > 0);

    // Test DELETE operation
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/keys/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify key is deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_key_pattern_matching() {
    let (router, redis) = create_test_server().await;
    let prefix = generate_test_key("pattern");
    let key1 = format!("{}_key1", prefix);
    let key2 = format!("{}_key2", prefix);
    let key3 = format!("{}_key3", prefix);
    let other_key = generate_test_key("other");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &other_key]).await;

    // Create test keys
    let set_request = json!({
        "value": "value1",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", key1),
        Some(set_request.clone())
    ).await;

    assert_status_code(&response, StatusCode::OK);

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", key2),
        Some(set_request.clone())
    ).await;

    assert_status_code(&response, StatusCode::OK);

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", key3),
        Some(set_request.clone())
    ).await;

    assert_status_code(&response, StatusCode::OK);

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", other_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test KEYS pattern matching with wildcard
    let pattern = format!("{}*", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&key1));
    assert!(keys.contains(&key2));
    assert!(keys.contains(&key3));
    assert!(!keys.contains(&other_key));

    // Test KEYS pattern matching with specific pattern
    let pattern = format!("{}_key?", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&key1));
    assert!(keys.contains(&key2));
    assert!(keys.contains(&key3));

    // Test KEYS pattern matching with no matches
    let pattern = format!("{}_nonexistent*", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 0);

    // Test KEYS with empty pattern (should return all keys)
    let response = make_request(router.clone(), "GET", "/api/v1/redis/keys/list/", None).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert!(keys.len() >= 4); // Should include our test keys

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &other_key]).await;
}

#[tokio::test]
async fn test_redis_key_batch_operations() {
    let (router, redis) = create_test_server().await;
    let key1 = generate_test_key("batch1");
    let key2 = generate_test_key("batch2");
    let key3 = generate_test_key("batch3");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;

    // Create test keys
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    for key in &[&key1, &key2, &key3] {
        let response = make_request(
            router.clone(),
            "POST",
            &format!("/api/v1/redis/strings/{}", key),
            Some(set_request.clone())
        ).await;

        assert_status_code(&response, StatusCode::OK);
    }

    // Test batch EXISTS operation
    let batch_exists_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone(), "nonexistent_key"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/keys/batch/exists",
        Some(batch_exists_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_exists_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_exists_response);

    // Test batch TTL operation
    let batch_ttl_request = json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/keys/batch/ttl",
        Some(batch_ttl_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_ttl_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_ttl_response);

    // Test batch DELETE operation
    let batch_delete_request =
        json!({
        "keys": [key1.clone(), key2.clone(), key3.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/keys/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_delete_response);

    // Verify keys are deleted
    for key in &[&key1, &key2, &key3] {
        let response = make_request(
            router.clone(),
            "GET",
            &format!("/api/v1/redis/keys/{}/exists", key),
            None
        ).await;

        assert_status_code(&response, StatusCode::OK);
        let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
        assert_success_response(&exists_response);
        assert!(!exists_response.data.unwrap().exists);
    }

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3]).await;
}

#[tokio::test]
async fn test_redis_key_error_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("key_error");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EXISTS on non-existent key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test TTL on non-existent key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -2); // -2 means key doesn't exist

    // Test DELETE on non-existent key
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/keys/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 0);

    // Test invalid pattern for KEYS
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/keys/list/invalid[pattern",
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Test KEYS with very long pattern
    let long_pattern = "a".repeat(1000);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", long_pattern),
        None
    ).await;

    // Should handle gracefully
    assert!(response.status().is_success() || response.status().is_client_error());

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_key_edge_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("key_edge");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test key with special characters
    let special_key = format!("{}:with:colons", test_key);
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", special_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test EXISTS on key with special characters
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", special_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test TTL on key with special characters
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", special_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert!(ttl_response.data.unwrap().ttl > 0);

    // Test key with spaces (URL encoded)
    let space_key = format!("{} with spaces", test_key);
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", space_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test key with unicode characters
    let unicode_key = format!("{}_unicode_测试", test_key);
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", unicode_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test pattern matching with unicode
    let pattern = format!("{}_unicode*", test_key);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 1);
    assert!(keys.contains(&unicode_key));

    // Test very long key name
    let long_key = format!("{}_{}", test_key, "a".repeat(100));
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", long_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test EXISTS on very long key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", long_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Clean up
    cleanup_test_keys(&redis, &[&special_key, &space_key, &unicode_key, &long_key]).await;
}

#[tokio::test]
async fn test_redis_key_ttl_edge_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("key_ttl");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test key without TTL
    let set_request = json!({
        "value": "test_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test TTL on key without expiration
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -1); // -1 means no expiration

    // Test key with very short TTL
    let short_ttl_key = format!("{}_short", test_key);
    let set_request = json!({
        "value": "test_value",
        "ttl": 1
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", short_ttl_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test TTL on key with short expiration
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", short_ttl_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert!(ttl_response.data.unwrap().ttl <= 1);

    // Wait for key to expire
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Test TTL on expired key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", short_ttl_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -2); // -2 means key doesn't exist

    // Clean up
    cleanup_test_keys(&redis, &[&test_key, &short_ttl_key]).await;
}

#[tokio::test]
async fn test_redis_key_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("key_comprehensive");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test key doesn't exist initially
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test TTL on non-existent key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -2); // -2 means key doesn't exist

    // Create a key with TTL
    let set_request = json!({
        "value": "test_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test key exists
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test TTL on existing key
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert!(ttl_response.data.unwrap().ttl > 0);

    // Test DELETE operation
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/keys/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify key is deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_key_pattern_matching_comprehensive() {
    let (router, redis) = create_test_server().await;
    let prefix = generate_test_key("pattern");
    let key1 = format!("{}_key1", prefix);
    let key2 = format!("{}_key2", prefix);
    let key3 = format!("{}_key3", prefix);
    let key4 = format!("{}_sub_key4", prefix);
    let other_key = generate_test_key("other");

    // Clean up before test
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &key4, &other_key]).await;

    // Create test keys
    let set_request = json!({
        "value": "value1",
        "ttl": 3600
    });

    for key in &[&key1, &key2, &key3, &key4, &other_key] {
        let response = make_request(
            router.clone(),
            "POST",
            &format!("/api/v1/redis/strings/{}", key),
            Some(set_request.clone())
        ).await;
        assert_status_code(&response, StatusCode::OK);
    }

    // Test KEYS pattern matching with wildcard
    let pattern = format!("{}*", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 4);
    assert!(keys.contains(&key1));
    assert!(keys.contains(&key2));
    assert!(keys.contains(&key3));
    assert!(keys.contains(&key4));
    assert!(!keys.contains(&other_key));

    // Test KEYS pattern matching with specific suffix
    let pattern = format!("{}_key*", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&key1));
    assert!(keys.contains(&key2));
    assert!(keys.contains(&key3));
    assert!(!keys.contains(&key4));

    // Test KEYS pattern matching with exact match
    let pattern = key1.clone();
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], key1);

    // Test KEYS pattern matching with non-existent pattern
    let pattern = format!("{}_nonexistent*", prefix);
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/list/{}", pattern),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert_success_response(&keys_response);
    let keys = keys_response.data.unwrap().keys;
    assert_eq!(keys.len(), 0);

    // Clean up
    cleanup_test_keys(&redis, &[&key1, &key2, &key3, &key4, &other_key]).await;
}
