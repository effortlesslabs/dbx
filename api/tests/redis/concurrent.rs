//! Redis HTTP API concurrent operation tests

use axum::http::StatusCode;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Barrier;

use crate::common::{
    assert_status_code,
    assert_success_response,
    cleanup_test_keys,
    create_test_server,
    extract_json,
    generate_test_key,
    make_request,
};
use dbx_api::models::{ ApiResponse, BooleanValue, IntegerValue, KeyValues, StringValue };

#[tokio::test]
async fn test_redis_concurrent_string_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_string");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent SET operations on the same key
    let mut handles = Vec::new();
    let num_operations = 10;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let set_request =
                json!({
                "value": format!("value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}", key),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let set_response: ApiResponse<StringValue> = extract_json(response).await;
            assert_success_response(&set_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify the final value (should be the last one set)
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    // The final value should be one of the values set (not necessarily the last one due to race conditions)

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_increment_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_incr");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Set initial value
    let set_request = json!({
        "value": "0",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test concurrent INCR operations
    let mut handles = Vec::new();
    let num_operations = 50;

    for _ in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}/incr", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
            assert_success_response(&incr_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify the final value
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, num_operations.to_string());

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_batch_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_batch");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent batch SET operations
    let mut handles = Vec::new();
    let num_batches = 5;
    let keys_per_batch = 10;

    for batch_id in 0..num_batches {
        let router_clone = router.clone();
        let base_key = format!("{}_{}", test_key, batch_id);

        let handle = tokio::spawn(async move {
            let mut key_values = serde_json::Map::new();
            for i in 0..keys_per_batch {
                let key = format!("{}_{}", base_key, i);
                key_values.insert(
                    key,
                    serde_json::Value::String(format!("value_{}_{}", batch_id, i))
                );
            }

            let batch_set_request =
                json!({
                "key_values": key_values,
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                "/api/v1/redis/strings/batch/set",
                Some(batch_set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
            assert_success_response(&batch_set_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all keys were set
    let mut all_keys = Vec::new();
    for batch_id in 0..num_batches {
        for i in 0..keys_per_batch {
            all_keys.push(format!("{}_{}_{}", test_key, batch_id, i));
        }
    }

    let batch_get_request = json!({
        "keys": all_keys
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
    assert_eq!(key_values.len(), num_batches * keys_per_batch);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_set_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_set");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent SADD operations
    let mut handles = Vec::new();
    let num_operations = 20;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let add_request =
                json!({
                "members": [format!("member_{}", i)]
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/sets/{}", key),
                Some(add_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
            assert_success_response(&add_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all members were added
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members", test_key.clone()),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let members_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&members_response);
    let response_data = members_response.data.unwrap();
    let members = response_data["members"].as_array().unwrap();
    assert_eq!(members.len(), num_operations);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_hash_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_hash");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent HSET operations
    let mut handles = Vec::new();
    let num_operations = 15;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let set_request =
                json!({
                "field": format!("field_{}", i),
                "value": format!("value_{}", i)
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/hashes/{}/{}", key.clone(), format!("field_{}", i)),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let set_response: ApiResponse<BooleanValue> = extract_json(response).await;
            assert_success_response(&set_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all fields were set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let hash_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&hash_response);
    let fields = hash_response.data.unwrap().key_values;
    assert_eq!(fields.len(), num_operations);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_read_write_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_rw");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Set initial value
    let set_request = json!({
        "value": "initial_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test concurrent read and write operations
    let mut handles = Vec::new();
    let num_operations = 30;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            if i % 2 == 0 {
                // Write operation
                let set_request =
                    json!({
                    "value": format!("value_{}", i),
                    "ttl": 3600
                });

                let response = make_request(
                    router_clone,
                    "POST",
                    &format!("/api/v1/redis/strings/{}", key),
                    Some(set_request)
                ).await;

                assert_status_code(&response, StatusCode::OK);
            } else {
                // Read operation
                let response = make_request(
                    router_clone,
                    "GET",
                    &format!("/api/v1/redis/strings/{}", key),
                    None
                ).await;

                // Read might succeed or fail depending on timing
                assert!(response.status().is_success() || response.status().is_client_error());
            }
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

#[tokio::test]
async fn test_redis_concurrent_stress_test() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_stress");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Stress test with many concurrent operations
    let mut handles = Vec::new();
    let num_operations = 100;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = format!("{}_{}", test_key, i);

        let handle = tokio::spawn(async move {
            // Set operation
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

            assert_status_code(&response, StatusCode::OK);

            // Get operation
            let response = make_request(
                router_clone.clone(),
                "GET",
                &format!("/api/v1/redis/strings/{}", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);

            // Increment operation
            let response = make_request(
                router_clone.clone(),
                "POST",
                &format!("/api/v1/redis/strings/{}/incr", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);

            // Delete operation
            let response = make_request(
                router_clone,
                "DELETE",
                &format!("/api/v1/redis/strings/{}", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);
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

#[tokio::test]
async fn test_redis_concurrent_barrier_synchronization() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_barrier");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test synchronization using barriers
    let barrier = Arc::new(Barrier::new(5));
    let mut handles = Vec::new();

    for i in 0..5 {
        let router_clone = router.clone();
        let key = test_key.clone();
        let barrier_clone = barrier.clone();

        let handle = tokio::spawn(async move {
            // Wait for all threads to reach this point
            barrier_clone.wait().await;

            // All threads will execute this simultaneously
            let set_request =
                json!({
                "value": format!("value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}", key),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify one of the values was set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert!(get_response.data.unwrap().value.starts_with("value_"));

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_race_conditions() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_race");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test race condition with SETNX
    let mut handles = Vec::new();
    let num_operations = 10;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let set_nx_request =
                json!({
                "value": format!("value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}/setnx", key),
                Some(set_nx_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
            assert_success_response(&set_nx_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify only one value was set (due to SETNX)
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert!(get_response.data.unwrap().value.starts_with("value_"));

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_concurrent_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("concurrent_comp");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test concurrent SET operations on the same key
    let mut handles = Vec::new();
    let num_operations = 20;

    for i in 0..num_operations {
        let router_clone = router.clone();
        let key = test_key.clone();

        let handle = tokio::spawn(async move {
            let set_request =
                json!({
                "value": format!("value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}", key),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let set_response: ApiResponse<StringValue> = extract_json(response).await;
            assert_success_response(&set_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify the final value (should be one of the values set)
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    let final_value = get_response.data.unwrap().value;
    assert!(final_value.starts_with("value_"));

    // Test concurrent INCR operations
    let mut handles = Vec::new();
    let num_incr_operations = 50;

    for _ in 0..num_incr_operations {
        let router_clone = router.clone();
        let key = format!("{}_incr", test_key);

        let handle = tokio::spawn(async move {
            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}/incr", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
            assert_success_response(&incr_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify the final value
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}_incr", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, num_incr_operations.to_string());

    // Test concurrent batch operations
    let mut handles = Vec::new();
    let num_batches = 10;
    let keys_per_batch = 5;

    for batch_id in 0..num_batches {
        let router_clone = router.clone();
        let base_key = format!("{}_batch_{}", test_key, batch_id);

        let handle = tokio::spawn(async move {
            let mut key_values = serde_json::Map::new();
            for i in 0..keys_per_batch {
                let key = format!("{}_{}", base_key, i);
                key_values.insert(
                    key,
                    serde_json::Value::String(format!("value_{}_{}", batch_id, i))
                );
            }

            let batch_set_request =
                json!({
                "key_values": key_values,
                "ttl": 3600
            });

            let response = make_request(
                router_clone,
                "POST",
                "/api/v1/redis/strings/batch/set",
                Some(batch_set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
            assert_success_response(&batch_set_response);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Test concurrent set operations
    let mut handles = Vec::new();
    let num_sets = 5;
    let members_per_set = 10;

    for set_id in 0..num_sets {
        let router_clone = router.clone();
        let set_key = format!("{}_set_{}", test_key, set_id);

        let handle = tokio::spawn(async move {
            let members: Vec<String> = (0..members_per_set)
                .map(|i| format!("member_{}_{}", set_id, i))
                .collect();

            let add_request = json!({
                "members": members
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/sets/{}", set_key),
                Some(add_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
            assert_success_response(&add_response);
            assert_eq!(add_response.data.unwrap().value, members_per_set as i64);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Test concurrent hash operations
    let mut handles = Vec::new();
    let num_hashes = 5;
    let fields_per_hash = 10;

    for hash_id in 0..num_hashes {
        let router_clone = router.clone();
        let hash_key = format!("{}_hash_{}", test_key, hash_id);

        let handle = tokio::spawn(async move {
            let mut fields = serde_json::Map::new();
            for i in 0..fields_per_hash {
                let field = format!("field_{}_{}", hash_id, i);
                let value = format!("value_{}_{}", hash_id, i);
                fields.insert(field, serde_json::Value::String(value));
            }

            let set_request = json!({
                "fields": fields
            });

            let response = make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/hashes/{}", hash_key),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let set_response: ApiResponse<IntegerValue> = extract_json(response).await;
            assert_success_response(&set_response);
            assert_eq!(set_response.data.unwrap().value, fields_per_hash as i64);
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Test concurrent read/write operations
    let mut handles = Vec::new();
    let num_read_write = 20;

    for i in 0..num_read_write {
        let router_clone = router.clone();
        let key = format!("{}_rw_{}", test_key, i);

        let handle = tokio::spawn(async move {
            // Write operation
            let set_request =
                json!({
                "value": format!("rw_value_{}", i),
                "ttl": 3600
            });

            let response = make_request(
                router_clone.clone(),
                "POST",
                &format!("/api/v1/redis/strings/{}", key),
                Some(set_request)
            ).await;

            assert_status_code(&response, StatusCode::OK);

            // Read operation
            let response = make_request(
                router_clone,
                "GET",
                &format!("/api/v1/redis/strings/{}", key),
                None
            ).await;

            assert_status_code(&response, StatusCode::OK);
            let get_response: ApiResponse<StringValue> = extract_json(response).await;
            assert_success_response(&get_response);
            assert_eq!(get_response.data.unwrap().value, format!("rw_value_{}", i));
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Clean up
    let keys_to_cleanup: Vec<String> = (0..num_batches)
        .flat_map(|batch_id| {
            let test_key_clone = test_key.clone();
            (0..keys_per_batch).map(move |i| format!("{}_batch_{}_{}", test_key_clone, batch_id, i))
        })
        .chain(
            (0..num_sets).map(|set_id| {
                let test_key_clone = test_key.clone();
                format!("{}_set_{}", test_key_clone, set_id)
            })
        )
        .chain(
            (0..num_hashes).map(|hash_id| {
                let test_key_clone = test_key.clone();
                format!("{}_hash_{}", test_key_clone, hash_id)
            })
        )
        .chain(
            (0..num_read_write).map(|i| {
                let test_key_clone = test_key.clone();
                format!("{}_rw_{}", test_key_clone, i)
            })
        )
        .collect();

    let keys_refs: Vec<&str> = keys_to_cleanup
        .iter()
        .map(|s| s.as_str())
        .collect();
    cleanup_test_keys(&redis, &keys_refs).await;
    cleanup_test_keys(&redis, &[&test_key, &format!("{}_incr", test_key.clone())]).await;
}
