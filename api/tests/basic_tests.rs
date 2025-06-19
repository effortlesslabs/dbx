use axum::{ body::Body, http::{ Request, StatusCode }, response::Response, Router };
use dbx_crates::adapter::redis::Redis;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use axum::body::to_bytes;

use dbx_api::{ config::Config, models::ApiResponse, server::Server };

// Helper to create a test server
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

// Helper to make HTTP requests
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

// Helper to extract JSON response
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
async fn test_api_compilation() {
    // This test verifies that the API code compiles and can be imported
    use dbx_api::config::Config;
    use dbx_api::models::ApiResponse;

    let config = Config::new();
    assert_eq!(config.port, 3000);

    let response = ApiResponse::<String>::success("test".to_string());
    assert!(response.success);
    assert_eq!(response.data.unwrap(), "test");
}
