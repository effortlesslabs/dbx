use crate::{middleware::handle_redis_error, models::WebSocketResponse};
use serde_json;
use std::collections::HashMap;

impl super::WebSocketCommandProcessor {
    /// Handle BATCH GET command
    pub async fn handle_batch_get_command(
        &self,
        command_id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut results = HashMap::new();
        for key in keys {
            if let Ok(Some(value)) = redis_handler.redis.string().get(&key) {
                results.insert(key, value);
            }
        }
        WebSocketResponse::success(command_id, serde_json::json!({ "key_values": results }))
    }

    /// Handle BATCH SET command
    pub async fn handle_batch_set_command(
        &self,
        command_id: Option<String>,
        key_values: HashMap<String, String>,
        ttl: Option<u64>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut results = HashMap::new();
        for (key, value) in key_values {
            let result = if let Some(ttl) = ttl {
                redis_handler.redis.string().set_with_expiry(
                    &key,
                    &value,
                    ttl.try_into().unwrap_or(usize::MAX),
                )
            } else {
                redis_handler.redis.string().set(&key, &value)
            };

            if result.is_ok() {
                results.insert(key, value);
            }
        }
        WebSocketResponse::success(command_id, serde_json::json!({ "key_values": results }))
    }

    /// Handle BATCH DELETE command
    pub async fn handle_batch_delete_command(
        &self,
        command_id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut deleted_count = 0;
        for key in keys {
            if redis_handler.redis.string().del(&key).is_ok() {
                deleted_count += 1;
            }
        }
        WebSocketResponse::success(
            command_id,
            serde_json::json!({ "deleted_count": deleted_count }),
        )
    }

    /// Handle BATCH INCR command
    pub async fn handle_batch_incr_command(
        &self,
        command_id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut results = Vec::new();
        for key in keys {
            if let Ok(value) = redis_handler.redis.string().incr(&key) {
                results.push(serde_json::json!({ "key": key, "value": value }));
            }
        }
        WebSocketResponse::success(command_id, serde_json::json!({ "results": results }))
    }

    /// Handle BATCH INCRBY command
    pub async fn handle_batch_incr_by_command(
        &self,
        command_id: Option<String>,
        key_increments: Vec<(String, i64)>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut results = Vec::new();
        for (key, increment) in key_increments {
            if let Ok(value) = redis_handler.redis.string().incr_by(&key, increment) {
                results.push(serde_json::json!({ "key": key, "value": value }));
            }
        }
        WebSocketResponse::success(command_id, serde_json::json!({ "results": results }))
    }
}
