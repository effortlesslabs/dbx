//! Redis HTTP API script operation tests

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
use dbx_api::models::{ ApiResponse, StringValue, IntegerValue, BooleanValue };

#[tokio::test]
async fn test_redis_script_basic_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("script_basic");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EVAL with simple script
    let eval_request =
        json!({
        "script": "return 'Hello, World!'",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "Hello, World!");

    // Test EVAL with script that uses keys and arguments
    let eval_request =
        json!({
        "script": "return redis.call('SET', KEYS[1], ARGV[1])",
        "keys": [test_key.clone()],
        "args": ["test_value"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "OK");

    // Verify the key was set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "test_value");

    // Test EVAL with script that returns a number
    let eval_request =
        json!({
        "script": "return 42",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, 42);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_complex_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("script_complex");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EVAL with script that performs multiple operations
    let complex_script =
        r#"
        local key = KEYS[1]
        local value = ARGV[1]
        local ttl = tonumber(ARGV[2])
        
        -- Set the key
        redis.call('SET', key, value)
        
        -- Set TTL if provided
        if ttl and ttl > 0 then
            redis.call('EXPIRE', key, ttl)
        end
        
        -- Return the value that was set
        return redis.call('GET', key)
    "#;

    let eval_request =
        json!({
        "script": complex_script,
        "keys": [test_key.clone()],
        "args": ["complex_value", "3600"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "complex_value");

    // Verify the key was set with TTL
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/keys/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert!(ttl_response.data.unwrap()["ttl"].as_i64().unwrap() > 0);

    // Test EVAL with script that returns a table
    let table_script =
        r#"
        local result = {}
        for i = 1, #KEYS do
            result[i] = redis.call('GET', KEYS[i])
        end
        return result
    "#;

    let eval_request =
        json!({
        "script": table_script,
        "keys": [test_key.clone()],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&eval_response);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_conditional_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("script_conditional");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EVAL with conditional logic
    let conditional_script =
        r#"
        local key = KEYS[1]
        local expected_value = ARGV[1]
        local new_value = ARGV[2]
        
        local current_value = redis.call('GET', key)
        
        if current_value == expected_value then
            redis.call('SET', key, new_value)
            return 1
        else
            return 0
        end
    "#;

    // First, set the key
    let set_request = json!({
        "value": "expected_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test conditional update with matching value
    let eval_request =
        json!({
        "script": conditional_script,
        "keys": [test_key.clone()],
        "args": ["expected_value", "new_value"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, 1);

    // Verify the value was updated
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "new_value");

    // Test conditional update with non-matching value
    let eval_request =
        json!({
        "script": conditional_script,
        "keys": [test_key.clone()],
        "args": ["wrong_value", "another_value"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, 0);

    // Verify the value was not changed
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "new_value");

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_error_handling() {
    let (router, redis) = create_test_server().await;

    // Test EVAL with invalid Lua script
    let invalid_script = "invalid lua code";

    let eval_request =
        json!({
        "script": invalid_script,
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test EVAL with script that calls non-existent Redis command
    let invalid_command_script = "return redis.call('INVALID_COMMAND')";

    let eval_request =
        json!({
        "script": invalid_command_script,
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test EVAL with script that accesses non-existent key
    let test_key = generate_test_key("script_error");
    let non_existent_key_script = "return redis.call('GET', KEYS[1])";

    let eval_request =
        json!({
        "script": non_existent_key_script,
        "keys": [test_key.clone()],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&eval_response);
    // Should return nil for non-existent key

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_edge_cases() {
    let (router, redis) = create_test_server().await;

    // Test EVAL with empty script
    let empty_script = "";

    let eval_request =
        json!({
        "script": empty_script,
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Test EVAL with very long script
    let long_script = "return 'test' -- ".repeat(1000);

    let eval_request =
        json!({
        "script": long_script,
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "test");

    // Test EVAL with many keys and arguments
    let test_key = generate_test_key("script_edge");
    let many_keys_script =
        r#"
        local result = {}
        for i = 1, #KEYS do
            result[i] = KEYS[i] .. ":" .. ARGV[i]
        end
        return result
    "#;

    let keys: Vec<String> = (0..10).map(|i| format!("{}_{}", test_key, i)).collect();
    let args: Vec<String> = (0..10).map(|i| format!("arg_{}", i)).collect();

    let eval_request =
        json!({
        "script": many_keys_script,
        "keys": keys,
        "args": args
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&eval_response);

    // Test EVAL with script that returns different data types
    let mixed_types_script =
        r#"
        return {
            "string",  -- string
            42,        -- number
            true,      -- boolean
            nil,       -- nil
            {1, 2, 3}  -- table
        }
    "#;

    let eval_request =
        json!({
        "script": mixed_types_script,
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&eval_response);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_performance() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("script_perf");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EVAL with script that performs multiple operations efficiently
    let performance_script =
        r#"
        local key = KEYS[1]
        local count = tonumber(ARGV[1])
        local result = 0
        
        for i = 1, count do
            redis.call('INCR', key)
            result = result + redis.call('GET', key)
        end
        
        return result
    "#;

    let eval_request =
        json!({
        "script": performance_script,
        "keys": [test_key.clone()],
        "args": ["100"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert!(eval_response.data.unwrap().value > 0);

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
    assert_eq!(get_response.data.unwrap().value, "100");

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_script_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("script_comp");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test EVAL with simple string return
    let eval_request =
        json!({
        "script": "return 'Hello, Redis!'",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "Hello, Redis!");

    // Test EVAL with numeric return
    let eval_request =
        json!({
        "script": "return 42",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, 42);

    // Test EVAL with boolean return
    let eval_request =
        json!({
        "script": "return true",
        "keys": [],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert!(eval_response.data.unwrap().value);

    // Test EVAL with key and argument usage
    let eval_request =
        json!({
        "script": "return redis.call('SET', KEYS[1], ARGV[1])",
        "keys": [test_key.clone()],
        "args": ["script_value"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "OK");

    // Verify the key was set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "script_value");

    // Test EVAL with complex logic
    let complex_script =
        r#"
        local key = KEYS[1]
        local value = ARGV[1]
        local ttl = tonumber(ARGV[2])
        
        -- Set the key
        redis.call('SET', key, value)
        
        -- Set TTL if provided
        if ttl and ttl > 0 then
            redis.call('EXPIRE', key, ttl)
        end
        
        -- Return the value that was set
        return redis.call('GET', key)
    "#;

    let eval_request =
        json!({
        "script": complex_script,
        "keys": [format!("{}_complex", test_key)],
        "args": ["complex_value", "3600"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "complex_value");

    // Test EVAL with array return
    let array_script =
        r#"
        local result = {}
        for i = 1, #KEYS do
            result[i] = redis.call('GET', KEYS[i])
        end
        return result
    "#;

    let eval_request =
        json!({
        "script": array_script,
        "keys": [test_key.clone(), format!("{}_complex", test_key)],
        "args": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<serde_json::Value> = extract_json(response).await;
    assert_success_response(&eval_response);

    // Test EVAL with conditional logic
    let conditional_script =
        r#"
        local key = KEYS[1]
        local threshold = tonumber(ARGV[1])
        local current_value = tonumber(redis.call('GET', key) or 0)
        
        if current_value >= threshold then
            return "above_threshold"
        else
            return "below_threshold"
        end
    "#;

    // Set initial value
    let set_request = json!({
        "value": "5",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}_conditional", test_key),
        Some(set_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test conditional script
    let eval_request =
        json!({
        "script": conditional_script,
        "keys": [format!("{}_conditional", test_key)],
        "args": ["10"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "below_threshold");

    // Test with higher threshold
    let eval_request =
        json!({
        "script": conditional_script,
        "keys": [format!("{}_conditional", test_key)],
        "args": ["3"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/scripts/eval",
        Some(eval_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let eval_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&eval_response);
    assert_eq!(eval_response.data.unwrap().value, "above_threshold");

    // Clean up
    cleanup_test_keys(
        &redis,
        &[&test_key, &format!("{}_complex", test_key), &format!("{}_conditional", test_key)]
    ).await;
}
