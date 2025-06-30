//! Integration tests for redis_rs crate

pub mod common;
pub mod redis;
pub mod redis_ws;

/// Test utilities and common functionality

/// Test utilities and helpers
pub mod utils {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    /// Get test HTTP server URL
    pub fn http_test_url() -> String {
        std::env::var("TEST_HTTP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    }

    /// Get test WebSocket server URL
    pub fn ws_test_url() -> String {
        std::env::var("TEST_WS_URL").unwrap_or_else(|_| "ws://localhost:3000/redis_ws".to_string())
    }

    /// Generate a unique test key
    pub fn unique_key(prefix: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("{}_{}_{}", prefix, timestamp, rand::random::<u32>())
    }

    /// Wait for a short duration (useful for async tests)
    pub async fn wait_for(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    /// Mock HTTP server for testing
    pub async fn start_mock_http_server() -> Result<(), Box<dyn std::error::Error>> {
        // This would start a mock HTTP server for testing
        // For now, we'll assume the real server is running
        Ok(())
    }

    /// Mock WebSocket server for testing
    pub async fn start_mock_ws_server() -> Result<(), Box<dyn std::error::Error>> {
        // This would start a mock WebSocket server for testing
        // For now, we'll assume the real server is running
        Ok(())
    }
}
