//! Redis HTTP API set operation tests

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
    IntegerValue,
    BooleanValue,
    ExistsResponse,
    DeleteResponse,
    StringValue,
    KeyValues,
};

#[tokio::test]
async fn test_redis_set_comprehensive_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("set_comprehensive");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test SADD with multiple members
    let add_request =
        json!({
        "members": ["member1", "member2", "member3", "member4", "member5"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(add_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 5);

    // Test SMEMBERS to get all members
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let members_response: ApiResponse<Vec<StringValue>> = extract_json(response).await;
    assert_success_response(&members_response);
    let members = members_response.data.unwrap();
    assert_eq!(members.len(), 5);
    assert!(members.iter().any(|m| m.value == "member1"));
    assert!(members.iter().any(|m| m.value == "member2"));
    assert!(members.iter().any(|m| m.value == "member3"));
    assert!(members.iter().any(|m| m.value == "member4"));
    assert!(members.iter().any(|m| m.value == "member5"));

    // Test SISMEMBER for existing member
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/member1/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test SISMEMBER for non-existing member
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/nonexistent/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test SCARD (cardinality)
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/cardinality", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let card_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&card_response);
    assert_eq!(card_response.data.unwrap().value, 5);

    // Test SRANDMEMBER multiple times
    for _ in 0..3 {
        let response = make_request(
            router.clone(),
            "GET",
            &format!("/api/v1/redis/sets/{}/random", test_key),
            None
        ).await;

        assert_status_code(&response, StatusCode::OK);
        let random_response: ApiResponse<StringValue> = extract_json(response).await;
        assert_success_response(&random_response);
        assert!(!random_response.data.unwrap().value.is_empty());
    }

    // Test SPOP
    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}/pop", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let pop_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&pop_response);
    assert!(!pop_response.data.unwrap().value.is_empty());

    // Verify cardinality decreased
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/cardinality", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let card_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&card_response);
    assert_eq!(card_response.data.unwrap().value, 4);

    // Test SREM for single member
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/sets/{}/members/member2", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let remove_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&remove_response);
    assert_eq!(remove_response.data.unwrap().value, 1);

    // Verify member was removed
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/member2/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test DELETE entire set
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/sets/{}", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let delete_response: ApiResponse<DeleteResponse> = extract_json(response).await;
    assert_success_response(&delete_response);
    assert_eq!(delete_response.data.unwrap().deleted_count, 1);

    // Verify set is deleted
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::NOT_FOUND);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_set_basic_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("set_basic");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test SADD operation
    let add_request = json!({
        "members": ["member1", "member2", "member3"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(add_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 3);

    // Test SMEMBERS operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let members_response: ApiResponse<Vec<StringValue>> = extract_json(response).await;
    assert_success_response(&members_response);
    let members = members_response.data.unwrap();
    assert_eq!(members.len(), 3);
    assert!(members.iter().any(|m| m.value == "member1"));
    assert!(members.iter().any(|m| m.value == "member2"));
    assert!(members.iter().any(|m| m.value == "member3"));

    // Test SISMEMBER operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/member1/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(exists_response.data.unwrap().exists);

    // Test SCARD operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/cardinality", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let card_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&card_response);
    assert_eq!(card_response.data.unwrap().value, 3);

    // Test SRANDMEMBER operation
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/random", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let random_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&random_response);
    assert!(!random_response.data.unwrap().value.is_empty());

    // Test SREM operation
    let remove_request = json!({
        "members": ["member1", "member2"]
    });

    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/sets/{}/members/member1", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let remove_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&remove_response);
    assert_eq!(remove_response.data.unwrap().value, 1);

    // Verify member was removed
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/member1/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test SPOP operation
    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}/pop", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let pop_response: ApiResponse<StringValue> = extract_json(response).await;
    assert_success_response(&pop_response);
    assert!(!pop_response.data.unwrap().value.is_empty());

    // Test DELETE operation
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/sets/{}", test_key),
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
async fn test_redis_set_operations() {
    let (router, redis) = create_test_server().await;
    let set1_key = generate_test_key("set1");
    let set2_key = generate_test_key("set2");
    let dest_key = generate_test_key("dest");

    // Clean up before test
    cleanup_test_keys(&redis, &[&set1_key, &set2_key, &dest_key]).await;

    // Create first set
    let add_request1 = json!({
        "members": ["a", "b", "c", "d"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", set1_key),
        Some(add_request1)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Create second set
    let add_request2 = json!({
        "members": ["c", "d", "e", "f"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", set2_key),
        Some(add_request2)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test UNION operation
    let union_request = json!({
        "keys": [set1_key.clone(), set2_key.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/union",
        Some(union_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let union_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&union_response);
    let union_data = union_response.data.unwrap();
    // The union result should be in the key_values map
    assert!(!union_data.key_values.is_empty());

    // Test INTERSECTION operation
    let intersection_request = json!({
        "keys": [set1_key.clone(), set2_key.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/intersection",
        Some(intersection_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let intersection_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&intersection_response);
    let intersection_data = intersection_response.data.unwrap();
    // The intersection result should be in the key_values map
    assert!(!intersection_data.key_values.is_empty());

    // Test DIFFERENCE operation
    let difference_request = json!({
        "keys": [set1_key.clone(), set2_key.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/difference",
        Some(difference_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let difference_response: ApiResponse<KeyValues> = extract_json(response).await;
    assert_success_response(&difference_response);
    let difference_data = difference_response.data.unwrap();
    // The difference result should be in the key_values map
    assert!(!difference_data.key_values.is_empty());

    // Test SMOVE operation
    let move_request = json!({
        "destination": dest_key.clone()
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}/move", set1_key),
        Some(move_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let move_response: ApiResponse<BooleanValue> = extract_json(response).await;
    assert_success_response(&move_response);

    // Clean up
    cleanup_test_keys(&redis, &[&set1_key, &set2_key, &dest_key]).await;
}

#[tokio::test]
async fn test_redis_set_batch_operations() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("set_batch");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test batch add
    let batch_add_request =
        json!({
        "members": ["batch1", "batch2", "batch3", "batch4", "batch5"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/batch/add",
        Some(batch_add_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&batch_add_response);
    assert_eq!(batch_add_response.data.unwrap().value, 5);

    // Test batch remove
    let batch_remove_request = json!({
        "members": ["batch1", "batch3", "batch5"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/batch/remove",
        Some(batch_remove_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_remove_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&batch_remove_response);
    assert_eq!(batch_remove_response.data.unwrap().value, 3);

    // Verify remaining members
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let members_response: ApiResponse<Vec<StringValue>> = extract_json(response).await;
    assert_success_response(&members_response);
    let members = members_response.data.unwrap();
    assert_eq!(members.len(), 2);
    assert!(members.iter().any(|m| m.value == "batch2"));
    assert!(members.iter().any(|m| m.value == "batch4"));

    // Test batch delete
    let batch_delete_request = json!({
        "keys": [test_key.clone()]
    });

    let response = make_request(
        router.clone(),
        "POST",
        "/api/v1/redis/sets/batch/delete",
        Some(batch_delete_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let batch_delete_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&batch_delete_response);
    assert_eq!(batch_delete_response.data.unwrap().value, 1);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_set_error_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("set_error");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test adding to non-existent set (should work)
    let add_request = json!({
        "members": ["test_member"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(add_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);

    // Test checking member existence in non-existent set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/members/nonexistent/exists", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let exists_response: ApiResponse<ExistsResponse> = extract_json(response).await;
    assert_success_response(&exists_response);
    assert!(!exists_response.data.unwrap().exists);

    // Test getting cardinality of non-existent set
    let response = make_request(
        router.clone(),
        "GET",
        &format!("/api/v1/redis/sets/{}/cardinality", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let card_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&card_response);
    assert_eq!(card_response.data.unwrap().value, 1);

    // Test removing non-existent member
    let response = make_request(
        router.clone(),
        "DELETE",
        &format!("/api/v1/redis/sets/{}/members/nonexistent", test_key),
        None
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let remove_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&remove_response);
    assert_eq!(remove_response.data.unwrap().value, 0);

    // Test invalid request body
    let invalid_request = json!({
        "invalid_field": "invalid_value"
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(invalid_request)
    ).await;

    // Should handle gracefully
    assert!(response.status().is_client_error() || response.status().is_success());

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}

#[tokio::test]
async fn test_redis_set_edge_cases() {
    let (router, redis) = create_test_server().await;
    let test_key = generate_test_key("set_edge");

    // Clean up before test
    cleanup_test_keys(&redis, &[&test_key]).await;

    // Test empty set operations
    let empty_request = json!({
        "members": []
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(empty_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 0);

    // Test duplicate members
    let duplicate_request = json!({
        "members": ["duplicate", "duplicate", "unique"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(duplicate_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 2); // Only unique members added

    // Test large set operations
    let large_members: Vec<String> = (0..100).map(|i| format!("member_{}", i)).collect();
    let large_request = json!({
        "members": large_members
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(large_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 100);

    // Test special characters in member names
    let special_request =
        json!({
        "members": ["member with spaces", "member-with-dashes", "member_with_underscores", "member:with:colons"]
    });

    let response = make_request(
        router.clone(),
        "POST",
        &format!("/api/v1/redis/sets/{}", test_key),
        Some(special_request)
    ).await;

    assert_status_code(&response, StatusCode::OK);
    let add_response: ApiResponse<IntegerValue> = extract_json(response).await;
    assert_success_response(&add_response);
    assert_eq!(add_response.data.unwrap().value, 4);

    // Clean up
    cleanup_test_keys(&redis, &[&test_key]).await;
}
