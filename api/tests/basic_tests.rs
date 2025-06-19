use axum::{ body::Body, http::{ Request, StatusCode }, response::Response, Router };
use dbx_crates::adapter::redis::Redis;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use axum::body::to_bytes;

use dbx_api::{ config::{ Config, DatabaseType }, server::Server };

// Helper to create a test server
async fn create_test_server() -> (Router, Arc<Redis>) {
    let config = Config {
        database_type: DatabaseType::Redis,
        database_url: "redis://default:redispw@localhost:55000".to_string(),
        host: "127.0.0.1".to_string(),
        port: 3001,
        pool_size: 5,
    };
    let server = Server::new(config).await.expect("Failed to create test server");

    // Create Redis client directly for testing
    let redis = Arc::new(
        Redis::from_url("redis://default:redispw@localhost:55000").expect(
            "Failed to create Redis client"
        )
    );
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
    let health_response: Value = extract_json(response).await;
    assert_eq!(health_response["status"], "healthy");
    assert_eq!(health_response["service"], "dbx-api");
    assert!(health_response["timestamp"].is_string());
}

#[tokio::test]
async fn test_api_compilation() {
    // This test verifies that the API code compiles and can be imported
    use dbx_api::config::Config;
    use dbx_api::models::ApiResponse;

    let config = Config::new(DatabaseType::Redis);
    assert_eq!(config.port, 3000);

    let response = ApiResponse::<String>::success("test".to_string());
    assert!(response.success);
    assert_eq!(response.data.unwrap(), "test");
}
