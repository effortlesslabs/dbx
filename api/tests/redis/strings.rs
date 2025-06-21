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
    TtlResponse,
    DeleteResponse,
};

#[tokio::test]
async fn test_redis_string_basic_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("string_basic");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

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

    assert_status_code(&response, StatusCode::OK);
    let set_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&set_response);
    assert_eq!(set_response.data.unwrap().value, "test_value");

    // Test GET operation
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

    // Test EXISTS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test TTL operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}/ttl", test_key),
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
        &format!("/api/v1/redis/strings/{}", test_key),
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
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_string_increment_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("string_incr");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test INCR operation
    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/incr", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 1);

    // Test INCR again
    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/incr", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let incr_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_response);
    assert_eq!(incr_response.data.unwrap().value, 2);

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

    assert_status_code(&response, StatusCode::OK);
    let incr_by_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&incr_by_response);
    assert_eq!(incr_by_response.data.unwrap().value, 7);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_string_set_if_not_exists() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("string_setnx");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test SETNX operation
    let set_nx_request = json!({
        "value": "first_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/setnx", test_key),
        Some(set_nx_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let set_nx_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&set_nx_response);
    assert!(set_nx_response.data.unwrap().value);

    // Try to set again (should fail)
    let set_nx_request = json!({
        "value": "second_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/setnx", test_key),
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
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "first_value");

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_string_compare_and_set() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("string_cas");

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

    assert_status_code(&response, StatusCode::OK);
    let cas_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&cas_response);
    assert!(cas_response.data.unwrap().value);

    // Verify value was updated
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "updated_value");

    // Test CAS with incorrect expected value
    let cas_request =
        json!({
        "expected_value": "wrong_value",
        "new_value": "another_value",
        "ttl": 3600
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/strings/{}/cas", test_key),
        Some(cas_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let cas_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&cas_response);
    assert!(!cas_response.data.unwrap().value);

    // Verify value was not changed
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let get_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&get_response);
    assert_eq!(get_response.data.unwrap().value, "updated_value");

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_string_operations_without_ttl() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("string_no_ttl");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test SET operation without TTL
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
    let set_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&set_response);

    // Test GET operation
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

    // Test TTL operation (should return -1 for no TTL)
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/strings/{}/ttl", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let ttl_response: ApiResponse<TtlResponse> = extract_json(response).await;
    assert_success_response(&ttl_response);
    assert_eq!(ttl_response.data.unwrap().ttl, -1);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_string_operations_error_cases() {
    let (router, _) = create_test_server().await;

    // Test GET non-existent key
    let response = make_request(
        router.clone(),
        "GET",
        "/api/v1/redis/strings/non_existent_key",
        None
    ).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Test invalid request body
    let invalid_request = json!({
        "invalid_field": "invalid_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/strings/test_key",
        Some(invalid_request)
    ).await;

    assert_status_code(&response, StatusCode::BAD_REQUEST);
}
