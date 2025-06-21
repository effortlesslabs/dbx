use axum::http::StatusCode;
use serde_json::Value;

use crate::common::{ create_test_server, make_request, extract_json, assert_status_code };

#[tokio::test]
async fn test_health_endpoint() {
    let (router, _) = create_test_server().await;

    let response = make_request(router, "GET", "/health", None).await;
    assert_status_code(&response, StatusCode::OK);

    let health_response: Value = extract_json(response).await;
    assert_eq!(health_response["status"], "healthy");
    assert_eq!(health_response["service"], "dbx-api");
    assert!(health_response["timestamp"].is_string());
}

#[tokio::test]
async fn test_info_endpoint() {
    let (router, _) = create_test_server().await;

    let response = make_request(router, "GET", "/info", None).await;
    assert_status_code(&response, StatusCode::OK);

    let info_response: Value = extract_json(response).await;
    assert_eq!(info_response["service"], "dbx-api");
    assert!(info_response["version"].is_string());
    assert!(info_response["database_url"].is_string());
}

#[tokio::test]
async fn test_health_endpoint_method_not_allowed() {
    let (router, _) = create_test_server().await;

    // Test POST method (should not be allowed)
    let response = make_request(router, "POST", "/health", None).await;
    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_info_endpoint_method_not_allowed() {
    let (router, _) = create_test_server().await;

    // Test POST method (should not be allowed)
    let response = make_request(router, "POST", "/info", None).await;
    assert_status_code(&response, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_health_endpoint_with_body() {
    let (router, _) = create_test_server().await;

    // Test with body (should ignore body)
    let body = serde_json::json!({"test": "data"});
    let response = make_request(router, "GET", "/health", Some(body)).await;
    assert_status_code(&response, StatusCode::OK);

    let health_response: Value = extract_json(response).await;
    assert_eq!(health_response["status"], "healthy");
}
