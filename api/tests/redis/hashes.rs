//! Redis HTTP API hash operation tests

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
    KeyValues,
};

#[tokio::test]
async fn test_redis_hash_basic_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_basic");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test HSET operation (single field)
    let set_request = json!({
        "field": "field1",
        "value": "value1"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_response);

    // Test HGET operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "value1");

    // Test HSET operation (multiple fields)
    let set_multiple_request =
        json!({
        "fields": {
            "field2": "value2",
            "field3": "value3",
            "field4": "value4"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(set_multiple_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_multiple_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_multiple_response);
    assert_eq!(set_multiple_response.data.unwrap().value, 3);

    // Test HGETALL operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_all_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&get_all_response);
    let fields = get_all_response.data.unwrap().key_values;
    assert_eq!(fields.len(), 4);
    assert_eq!(fields.get("field1"), Some(&"value1".to_string()));
    assert_eq!(fields.get("field2"), Some(&"value2".to_string()));
    assert_eq!(fields.get("field3"), Some(&"value3".to_string()));
    assert_eq!(fields.get("field4"), Some(&"value4".to_string()));

    // Test HEXISTS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test HLEN operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/length", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let len_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&len_response);
    assert_eq!(len_response.data.unwrap().value, 4);

    // Test HKEYS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/keys", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&keys_response);
    let response_data = keys_response.data.unwrap();
    let keys = response_data["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 4);
    assert!(keys.iter().any(|k| k.as_str() == Some("field1")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field2")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field3")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field4")));

    // Test HVALS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/values", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let values_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&values_response);
    let response_data = values_response.data.unwrap();
    let values = response_data["values"].as_array().unwrap();
    assert_eq!(values.len(), 4);
    assert!(values.iter().any(|v| v.as_str() == Some("value1")));
    assert!(values.iter().any(|v| v.as_str() == Some("value2")));
    assert!(values.iter().any(|v| v.as_str() == Some("value3")));
    assert!(values.iter().any(|v| v.as_str() == Some("value4")));

    // Test HRANDFIELD operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/random", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let random_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&random_response);
    let random_data = random_response.data.unwrap();
    assert!(random_data["field"].is_string());
    assert!(random_data["value"].is_string());

    // Test HDEL operation
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let del_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&del_response);
    assert_eq!(del_response.data.unwrap().value, 1);

    // Verify field was deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test DELETE operation (entire hash)
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_numeric_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_numeric");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Set initial numeric field
    let set_request = json!({
        "field": "counter",
        "value": "10"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "counter"),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test HINCRBY operation
    let incr_request = json!({
        "increment": 5
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/incr", test_key, "counter"),
        Some(incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 15);

    // Test HINCRBY with negative value
    let incr_request = json!({
        "increment": -3
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/incr", test_key, "counter"),
        Some(incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 12);

    // Test HINCRBY on non-existent field
    let incr_request = json!({
        "increment": 7
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/incr", test_key, "new_counter"),
        Some(incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 7);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_conditional_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_conditional");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test HSETNX operation (field doesn't exist)
    let set_nx_request =
        json!({
        "field": "unique_field",
        "value": "unique_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/setnx", test_key, "unique_field"),
        Some(set_nx_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_nx_response);
    assert!(set_nx_response.data.unwrap().value);

    // Test HSETNX operation (field already exists)
    let set_nx_request =
        json!({
        "field": "unique_field",
        "value": "different_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/setnx", test_key, "unique_field"),
        Some(set_nx_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_nx_response);
    assert!(!set_nx_response.data.unwrap().value);

    // Verify original value is still there
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "unique_field"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "unique_value");

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_multiple_fields_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_multiple");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Set multiple fields
    let set_multiple_request =
        json!({
        "fields": {
            "field1": "value1",
            "field2": "value2",
            "field3": "value3",
            "field4": "value4",
            "field5": "value5"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(set_multiple_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test HMGET operation
    let mget_request = json!({
        "fields": ["field1", "field3", "field5", "nonexistent"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/mget", test_key),
        Some(mget_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let mget_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&mget_response);
    let response_data = mget_response.data.unwrap();
    let field_values = response_data["field_values"].as_object().unwrap();
    assert_eq!(field_values.len(), 4);

    // Check that existing fields have values and non-existent field is null
    assert_eq!(field_values.get("field1").unwrap().as_str(), Some("value1"));
    assert_eq!(field_values.get("field3").unwrap().as_str(), Some("value3"));
    assert_eq!(field_values.get("field5").unwrap().as_str(), Some("value5"));
    assert_eq!(field_values.get("nonexistent").unwrap().as_str(), Some("")); // or null depending on implementation

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_batch_operations() {
    let (router, redis) = create_test_server().await;
    let test_key1 = generate_test_key("hash_batch1");
    let test_key2 = generate_test_key("hash_batch2");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key1, &test_key2]).await;

    // Test batch set hash fields
    let batch_set_request =
        json!({
        "hash_fields": {
            "hash1": {
                "field1": "value1",
                "field2": "value2"
            },
            "hash2": {
                "field3": "value3",
                "field4": "value4"
            }
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/set",
        Some(batch_set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_set_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_set_response);

    // Test batch get hash fields
    let batch_get_request =
        json!({
        "hash_fields": {
            "hash1": ["field1", "field2"],
            "hash2": ["field3", "field4"]
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/get",
        Some(batch_get_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_get_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_get_response);

    // Test batch delete hash fields
    let batch_delete_request =
        json!({
        "hash_fields": {
            "hash1": ["field1"],
            "hash2": ["field3"]
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_delete_response);

    // Test batch get all hash fields
    let batch_all_request = json!({
        "keys": ["hash1", "hash2"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/all",
        Some(batch_all_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_all_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_all_response);

    // Test batch check hash field existence
    let batch_exists_request =
        json!({
        "hash_fields": {
            "hash1": ["field2"],
            "hash2": ["field4"]
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/exists",
        Some(batch_exists_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_exists_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_exists_response);

    // Test batch get hash lengths
    let batch_lengths_request = json!({
        "keys": ["hash1", "hash2"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/hashes/batch/lengths",
        Some(batch_lengths_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_lengths_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&batch_lengths_response);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key1, &test_key2]).await;
}

#[tokio::test]
async fn test_redis_hash_error_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_error");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test getting non-existent field
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "nonexistent"),
        None
    ).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Test checking existence of field in non-existent hash
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test getting length of non-existent hash
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/length", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let len_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&len_response);
    assert_eq!(len_response.data.unwrap().value, 0);

    // Test deleting non-existent field
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "nonexistent"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let del_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&del_response);
    assert_eq!(del_response.data.unwrap().value, 0);

    // Test invalid request body
    let invalid_request = json!({
        "invalid_field": "invalid_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_edge_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_edge");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test empty hash operations
    let empty_request = json!({
        "fields": {}
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(empty_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_response);
    assert_eq!(set_response.data.unwrap().value, 0);

    // Test special characters in field names and values
    let special_request =
        json!({
        "fields": {
            "field with spaces": "value with spaces",
            "field-with-dashes": "value-with-dashes",
            "field_with_underscores": "value_with_underscores",
            "field:with:colons": "value:with:colons",
            "field.with.dots": "value.with.dots"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(special_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_response);
    assert_eq!(set_response.data.unwrap().value, 5);

    // Test large hash operations
    let mut large_fields = serde_json::Map::new();
    for i in 0..50 {
        large_fields.insert(
            format!("field_{}", i),
            serde_json::Value::String(format!("value_{}", i))
        );
    }
    let large_request = json!({
        "fields": large_fields
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(large_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_response);
    assert_eq!(set_response.data.unwrap().value, 50);

    // Test numeric values as strings
    let numeric_request =
        json!({
        "fields": {
            "int_field": "123",
            "float_field": "123.456",
            "negative_field": "-789"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(numeric_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_response);
    assert_eq!(set_response.data.unwrap().value, 3);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_hash_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("hash_comprehensive");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test HSET single field
    let set_request = json!({
        "field": "field1",
        "value": "value1"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_response);

    // Test HGET single field
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "value1");

    // Test HSET multiple fields
    let set_multiple_request =
        json!({
        "fields": {
            "field2": "value2",
            "field3": "value3",
            "field4": "value4",
            "field5": "value5"
        }
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}", test_key),
        Some(set_multiple_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_multiple_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&set_multiple_response);
    assert_eq!(set_multiple_response.data.unwrap().value, 4);

    // Test HGETALL
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_all_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&get_all_response);
    let fields = get_all_response.data.unwrap().key_values;
    assert_eq!(fields.len(), 5);
    assert_eq!(fields.get("field1"), Some(&"value1".to_string()));
    assert_eq!(fields.get("field2"), Some(&"value2".to_string()));
    assert_eq!(fields.get("field3"), Some(&"value3".to_string()));
    assert_eq!(fields.get("field4"), Some(&"value4".to_string()));
    assert_eq!(fields.get("field5"), Some(&"value5".to_string()));

    // Test HEXISTS for existing field
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test HEXISTS for non-existing field
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "nonexistent"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test HLEN
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/length", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let len_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&len_response);
    assert_eq!(len_response.data.unwrap().value, 5);

    // Test HKEYS
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/keys", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let keys_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&keys_response);
    let response_data = keys_response.data.unwrap();
    let keys = response_data["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 5);
    assert!(keys.iter().any(|k| k.as_str() == Some("field1")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field2")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field3")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field4")));
    assert!(keys.iter().any(|k| k.as_str() == Some("field5")));

    // Test HVALS
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/values", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let values_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&values_response);
    let response_data = values_response.data.unwrap();
    let values = response_data["values"].as_array().unwrap();
    assert_eq!(values.len(), 5);
    assert!(values.iter().any(|v| v.as_str() == Some("value1")));
    assert!(values.iter().any(|v| v.as_str() == Some("value2")));
    assert!(values.iter().any(|v| v.as_str() == Some("value3")));
    assert!(values.iter().any(|v| v.as_str() == Some("value4")));
    assert!(values.iter().any(|v| v.as_str() == Some("value5")));

    // Test HRANDFIELD
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/random", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let random_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&random_response);
    let random_data = random_response.data.unwrap();
    assert!(random_data["field"].is_string());
    assert!(random_data["value"].is_string());

    // Test HINCRBY
    let incr_request = json!({
        "increment": 10
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/incr", test_key, "numeric_field"),
        Some(incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 10);

    // Test HINCRBY again
    let incr_request = json!({
        "increment": 5
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/incr", test_key, "numeric_field"),
        Some(incr_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 15);

    // Test HSETNX for new field
    let set_nx_request = json!({
        "field": "new_field",
        "value": "new_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/setnx", test_key, "new_field"),
        Some(set_nx_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_nx_response);
    assert!(set_nx_response.data.unwrap().value);

    // Test HSETNX for existing field (should fail)
    let set_nx_request = json!({
        "field": "field1",
        "value": "overwrite_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/{}/setnx", test_key, "field1"),
        Some(set_nx_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_nx_response);
    assert!(!set_nx_response.data.unwrap().value);

    // Test HMGET
    let mget_request = json!({
        "fields": ["field1", "field2", "nonexistent", "field3"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/hashes/{}/mget", test_key),
        Some(mget_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let mget_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&mget_response);
    let mget_data = mget_response.data.unwrap();
    assert_eq!(mget_data["field1"], "value1");
    assert_eq!(mget_data["field2"], "value2");
    assert_eq!(mget_data["field3"], "value3");
    assert_eq!(mget_data["nonexistent"], "");

    // Test HDEL single field
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/hashes/{}/{}", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let del_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&del_response);
    assert_eq!(del_response.data.unwrap().value, 1);

    // Verify field was deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}/{}/exists", test_key, "field1"),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test DELETE entire hash
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify hash is deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/hashes/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}
