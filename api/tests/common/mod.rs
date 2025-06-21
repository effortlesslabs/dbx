use axum::body::to_bytes;
use axum::{ body::Body, http::{ Request, StatusCode }, response::Response, Router };
use dbx_crates::adapter::redis::Redis;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{ connect_async, WebSocketStream, MaybeTlsStream };
use futures_util::{ SinkExt, StreamExt };

use dbx_api::{
    config::{ Config, DatabaseType },
    constants::database::DatabaseUrls,
    server::Server,
};

/// Test helper to create a test server
pub async fn create_test_server() -> (Router, Arc<Redis>) {
    let config = Config {
        database_type: DatabaseType::Redis,
        database_url: DatabaseUrls::redis_test_url(),
        host: "127.0.0.1".to_string(),
        port: 3001,
        pool_size: 5,
    };

    let server = Server::new(config).await.expect("Failed to create test server");

    // Create Redis client directly for testing
    let redis = Arc::new(
        Redis::from_url(&DatabaseUrls::redis_test_url()).expect("Failed to create Redis client")
    );
    let router = server.create_router();

    (router, redis)
}

/// Test helper to make HTTP requests
pub async fn make_request(
    router: Router,
    method: &str,
    path: &str,
    body: Option<Value>
) -> Response<Body> {
    let request_builder = Request::builder()
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

/// Test helper to extract JSON response
pub async fn extract_json<T: serde::de::DeserializeOwned>(response: Response<Body>) -> T {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

/// Test helper to create WebSocket connection
pub async fn create_websocket_connection() -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let url = "ws://127.0.0.1:3001/redis_ws";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to WebSocket");
    ws_stream
}

/// Test helper to send WebSocket message and get response
pub async fn send_websocket_message(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    message: Value
) -> Value {
    ws_stream
        .send(tokio_tungstenite::tungstenite::Message::Text(message.to_string())).await
        .expect("Failed to send WebSocket message");

    let response = ws_stream.next().await.expect("Failed to receive WebSocket response");
    let response_text = response.unwrap().into_text().unwrap();
    serde_json::from_str(&response_text).unwrap()
}

/// Test helper to clean up test keys
pub async fn cleanup_test_keys(redis: &Redis, keys: &[&str]) {
    for key in keys {
        let _ = redis.string().del(key);
    }
}

/// Test helper to generate unique test keys
pub fn generate_test_key(prefix: &str) -> String {
    use std::time::{ SystemTime, UNIX_EPOCH };
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("test_{}_{}", prefix, timestamp)
}

/// Test helper to wait for async operations
pub async fn wait_for_async() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

/// Assert response is successful
pub fn assert_success_response<T>(response: &dbx_api::models::ApiResponse<T>) {
    assert!(response.success, "Expected successful response, got error: {:?}", response.error);
}

/// Assert response is an error
pub fn assert_error_response<T>(response: &dbx_api::models::ApiResponse<T>) {
    assert!(!response.success, "Expected error response, got success");
    assert!(response.error.is_some(), "Expected error message");
}

/// Assert HTTP status code
pub fn assert_status_code(response: &Response<Body>, expected: StatusCode) {
    assert_eq!(
        response.status(),
        expected,
        "Expected status code {}, got {}",
        expected,
        response.status()
    );
}
