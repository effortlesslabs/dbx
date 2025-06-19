use axum::{ body::Body, http::{ Request, StatusCode }, response::Response, Router };
use dbx_crates::adapter::redis::Redis;
use serde_json::{ json, Value };
use std::sync::Arc;
use tower::util::ServiceExt;
use axum::body::to_bytes;

use dbx_api::{
    config::Config,
    models::{
        ApiResponse,
        BooleanValue,
        CompareAndSetRequest,
        DeleteResponse,
        ExistsResponse,
        IncrByRequest,
        IntegerValue,
        KeyValues,
        KeysResponse,
        SetIfNotExistsRequest,
        SetManyRequest,
        SetRequest,
        StringValue,
        TtlResponse,
    },
    server::Server,
};

// Test helper to create a test server
async fn create_test_server() -> (Router, Arc<Redis>) {
    let config = Config {
        redis_url: "redis://127.0.0.1:6379".to_string(),
        host: "127.0.0.1".to_string(),
        port: 3000,
        pool_size: 5,
    };

    let server = Server::new(config).await.expect("Failed to create test server");
    let redis = server.redis().clone();
    let router = server.create_router();

    (router, redis)
}

// Test helper to make HTTP requests
async fn make_request(
    router: Router,
    method: &str,
    path: &str,
    body: Option<Value>
) -> Response<Body> {
    let mut request_builder = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");

    let request = if let Some(body) = body {
        request_builder.body(Body::from(body.to_string())).unwrap()
    } else {
        request_builder.body(Body::empty()).unwrap()
    };

    router.oneshot(request).await.unwrap()
}

// Test helper to extract JSON response
async fn extract_json<T: serde::de::DeserializeOwned>(response: Response<Body>) -> T {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn test_health_endpoint() {
    let (router, _) = create_test_server().await;

    let response = make_request(router, "GET", "/health", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let health_response: ApiResponse<Value> = extract_json(response).await;
    assert!(health_response.success);
    assert!(health_response.data.is_some());

    let health_data = health_response.data.unwrap();
    assert_eq!(health_data["status"], "ok");
    assert!(health_data["timestamp"].is_string());
}

#[tokio::test]
async fn test_info_endpoint() {
    let (router, _) = create_test_server().await;

    let response = make_request(router, "GET", "/info", None).await;
    assert_eq!(response.status(), StatusCode::OK);

    let info_response: ApiResponse<Value> = extract_json(response).await;
    assert!(info_response.success);
    assert!(info_response.data.is_some());

    let info_data = info_response.data.unwrap();
    assert_eq!(info_data["name"], "DBX API");
    assert!(info_data["version"].is_string());
    assert_eq!(info_data["redis_url"], "redis://127.0.0.1:6379");
}

#[tokio::test]
async fn test_string_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = "test_string_key";

    // Clean up before test
    let _ = redis.string().del(test_key);

    // Test SET operation
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

    assert_eq!(response.status(), StatusCode::OK);
    let set_response: ApiResponse<StringValue> = extract_json(response).await;
    assert!(set_response.success);
    assert_eq!(set_response.data.unwrap().value, "test_value");

    // Test GET operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert!(get_response.success);
    assert_eq!(get_response.data.unwrap().value, "test_value");

    // Test EXISTS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}/exists", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert!(exists_response.success);
    assert!(exists_response.data.unwrap().exists);

    // Test TTL operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}/ttl", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert!(ttl_response.success);
    assert!(ttl_response.data.unwrap().ttl > 0);

    // Test DELETE operation
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert!(delete_response.success);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify key is deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_increment_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = "test_counter_key";

    // Clean up before test
    let _ = redis.string().del(test_key);

    // Test INCR operation
    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/incr", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert!(incr_response.success);
    assert_eq!(incr_response.data.unwrap().value, 1);

    // Test INCRBY operation
    let incr_by_request = json!({
        "increment": 5
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/incrby", test_key),
        Some(incr_by_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let incr_by_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert!(incr_by_response.success);
    assert_eq!(incr_by_response.data.unwrap().value, 6);

    // Clean up
    let _ = redis.string().del(test_key);
}

#[tokio::test]
async fn test_set_if_not_exists() {
    let (router, redis) = create_test_server().await;
    let test_key = "test_setnx_key";

    // Clean up before test
    let _ = redis.string().del(test_key);

    // Test SETNX with new key
    let setnx_request = json!({
        "value": "new_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/setnx", test_key),
        Some(setnx_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let setnx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert!(setnx_response.success);
    assert!(setnx_response.data.unwrap().value);

    // Test SETNX with existing key
    let setnx_request2 = json!({
        "value": "another_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/setnx", test_key),
        Some(setnx_request2)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let setnx_response2: ApiResponse<BooleanValue> = extract_json(response).await;
    assert!(setnx_response2.success);
    assert!(!setnx_response2.data.unwrap().value);

    // Verify original value is still there
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_eq!(get_response.data.unwrap().value, "new_value");

    // Clean up
    let _ = redis.string().del(test_key);
}

#[tokio::test]
async fn test_compare_and_set() {
    let (router, redis) = create_test_server().await;
    let test_key = "test_cas_key";

    // Clean up before test
    let _ = redis.string().del(test_key);

    // Set initial value
    let set_request = json!({
        "value": "initial_value"
    });

    let _ = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    // Test CAS with correct expected value
    let cas_request =
        json!({
        "expected_value": "initial_value",
        "new_value": "updated_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/cas", test_key),
        Some(cas_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let cas_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert!(cas_response.success);
    assert!(cas_response.data.unwrap().value);

    // Test CAS with incorrect expected value
    let cas_request2 =
        json!({
        "expected_value": "wrong_value",
        "new_value": "another_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/cas", test_key),
        Some(cas_request2)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let cas_response2: ApiResponse<BooleanValue> = extract_json(response).await;
    assert!(cas_response2.success);
    assert!(!cas_response2.data.unwrap().value);

    // Verify value was updated correctly
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_eq!(get_response.data.unwrap().value, "updated_value");

    // Clean up
    let _ = redis.string().del(test_key);
}

#[tokio::test]
async fn test_batch_operations() {
    let (router, redis) = create_test_server().await;
    let test_keys = vec!["batch_key1", "batch_key2", "batch_key3"];

    // Clean up before test
    for key in &test_keys {
        let _ = redis.string().del(key);
    }

    // Test batch SET
    let batch_set_request =
        json!({
        "key_values": {
            "batch_key1": "value1",
            "batch_key2": "value2",
            "batch_key3": "value3"
        },
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/set",
        Some(batch_set_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let batch_set_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert!(batch_set_response.success);

    // Test batch GET
    let batch_get_request = json!(["batch_key1", "batch_key2", "batch_key3"]);

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/get",
        Some(batch_get_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let batch_get_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert!(batch_get_response.success);

    let key_values = batch_get_response.data.unwrap().key_values;
    assert_eq!(key_values["batch_key1"], "value1");
    assert_eq!(key_values["batch_key2"], "value2");
    assert_eq!(key_values["batch_key3"], "value3");

    // Test batch INCR
    let batch_incr_request = json!(["batch_key1", "batch_key2"]);

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/incr",
        Some(batch_incr_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let batch_incr_response: ApiResponse<Vec<IntegerValue>> = extract_json(response).await;
    assert!(batch_incr_response.success);

    // Test batch DELETE
    let batch_delete_request = json!(["batch_key1", "batch_key2", "batch_key3"]);

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let batch_delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert!(batch_delete_response.success);
    assert_eq!(batch_delete_response.data.unwrap().deleted_count, 3);
}

#[tokio::test]
async fn test_key_operations() {
    let (router, redis) = create_test_server().await;
    let test_keys = vec!["key_op1", "key_op2", "key_op3"];

    // Clean up before test
    for key in &test_keys {
        let _ = redis.string().del(key);
    }

    // Set some test keys
    for key in &test_keys {
        let set_request = json!({
            "value": format!("value_for_{}", key)
        });

        let _ = make_request(
            router.clone(),
            "POST",
            &format!("/api/v1/redis/strings/{}", key),
            Some(set_request)
        ).await;
    }

    // Test list keys
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/keys?pattern=key_op*",
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let keys_response: ApiResponse<KeysResponse> = extract_json(response).await;
    assert!(keys_response.success);
    assert!(keys_response.data.unwrap().keys.len() >= 3);

    // Test key exists
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/keys/key_op1/exists",
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert!(exists_response.success);
    assert!(exists_response.data.unwrap().exists);

    // Test key TTL
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/keys/key_op1/ttl",
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert!(ttl_response.success);

    // Test delete key
    let response = make_request(router.clone(), "DELETE", "/api/v1/redis/keys/key_op1", None).await;

    assert_eq!(response.status(), StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert!(delete_response.success);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify key is deleted
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/keys/key_op1/exists",
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert!(!exists_response.data.unwrap().exists);

    // Clean up remaining keys
    for key in &test_keys[1..] {
        let _ = redis.string().del(key);
    }
}

#[tokio::test]
async fn test_lua_script_operations() {
    let (router, redis) = create_test_server().await;

    // Test rate limiter script
    let rate_limiter_request =
        json!({
        "key": "rate_limit_test",
        "limit": 5,
        "window": 60
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/rate-limiter",
        Some(rate_limiter_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let rate_limiter_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert!(rate_limiter_response.success);
    assert!(rate_limiter_response.data.unwrap().value);

    // Test multi-counter script
    let multi_counter_request =
        json!({
        "counters": [
            ["counter1", 1],
            ["counter2", 2],
            ["counter3", 3]
        ]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/multi-counter",
        Some(multi_counter_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let multi_counter_response: ApiResponse<Vec<IntegerValue>> = extract_json(response).await;
    assert!(multi_counter_response.success);
    assert_eq!(multi_counter_response.data.unwrap().len(), 3);

    // Test multi-set-ttl script
    let multi_set_ttl_request =
        json!({
        "key_values": {
            "lua_key1": "lua_value1",
            "lua_key2": "lua_value2"
        },
        "ttl": 300
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/multi-set-ttl",
        Some(multi_set_ttl_request)
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let multi_set_ttl_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert!(multi_set_ttl_response.success);

    // Clean up
    let _ = redis.string().del("rate_limit_test");
    let _ = redis.string().del("counter1");
    let _ = redis.string().del("counter2");
    let _ = redis.string().del("counter3");
    let _ = redis.string().del("lua_key1");
    let _ = redis.string().del("lua_key2");
}

#[tokio::test]
async fn test_error_handling() {
    let (router, _) = create_test_server().await;

    // Test invalid JSON
    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(json!("invalid json"))
    ).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Test missing required fields
    let invalid_request = json!({
        "missing_field": "value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // Test non-existent endpoint
    let response = make_request(router.clone(), "GET", "/api/v1/redis/nonexistent", None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_concurrent_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = "concurrent_test_key";

    // Clean up before test
    let _ = redis.string().del(test_key);

    // Set initial value
    let set_request = json!({
        "value": "0"
    });

    let _ = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    // Perform concurrent increments
    let mut handles = vec![];

    for _ in 0..10 {
        let router_clone = router.clone();
        let key = test_key.to_string();

        let handle = tokio::spawn(async move {
            make_request(
                router_clone,
                "POST",
                &format!("/api/v1/redis/strings/{}/incr", key),
                None
            ).await
        });

        handles.push(handle);
    }

    // Wait for all operations to complete
    let mut responses = vec![];
    for handle in handles {
        let response = handle.await.unwrap();
        responses.push(response);
    }

    // Verify all responses are successful
    for response in responses {
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Verify final value
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_eq!(response.status(), StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_eq!(get_response.data.unwrap().value, "10");

    // Clean up
    let _ = redis.string().del(test_key);
}
