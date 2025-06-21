use serde_json;
use crate::{ models::WebSocketResponse, middleware::handle_redis_error };

impl super::WebSocketCommandProcessor {
    /// Handle GET command
    pub async fn handle_get_command(
        &self,
        command_id: Option<String>,
        key: String
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().get(&key) {
            Ok(Some(value)) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "value": value }))
            }
            Ok(None) => { WebSocketResponse::error(command_id, "Key not found".to_string()) }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle SET command
    pub async fn handle_set_command(
        &self,
        command_id: Option<String>,
        key: String,
        value: String,
        ttl: Option<u64>
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let result = if let Some(ttl) = ttl {
            redis_handler.redis
                .string()
                .set_with_expiry(&key, &value, ttl.try_into().unwrap_or(usize::MAX))
        } else {
            redis_handler.redis.string().set(&key, &value)
        };

        match result {
            Ok(_) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "value": value }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle DELETE command
    pub async fn handle_delete_command(
        &self,
        command_id: Option<String>,
        key: String
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().del(&key) {
            Ok(_) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "deleted_count": 1 }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle EXISTS command
    pub async fn handle_exists_command(
        &self,
        command_id: Option<String>,
        key: String
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().exists(&key) {
            Ok(exists) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "exists": exists }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle TTL command
    pub async fn handle_ttl_command(
        &self,
        command_id: Option<String>,
        key: String
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().ttl(&key) {
            Ok(ttl) => { WebSocketResponse::success(command_id, serde_json::json!({ "ttl": ttl })) }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle INCR command
    pub async fn handle_incr_command(
        &self,
        command_id: Option<String>,
        key: String
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().incr(&key) {
            Ok(value) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "value": value }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle INCRBY command
    pub async fn handle_incr_by_command(
        &self,
        command_id: Option<String>,
        key: String,
        increment: i64
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.string().incr_by(&key, increment) {
            Ok(value) => {
                WebSocketResponse::success(command_id, serde_json::json!({ "value": value }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                )
            }
        }
    }

    /// Handle SETNX command
    pub async fn handle_set_nx_command(
        &self,
        command_id: Option<String>,
        key: String,
        value: String,
        ttl: Option<u64>
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let script =
            dbx_crates::adapter::redis::primitives::string::RedisString::set_if_not_exists_script();
        let result: i32 = match
            redis_handler.redis.string().eval_script(&script, &[&key], &[&value])
        {
            Ok(result) => result,
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                return WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                );
            }
        };

        let success = result == 1;
        if let Some(ttl) = ttl {
            if success {
                let _ = redis_handler.redis.string().expire(&key, ttl);
            }
        }

        WebSocketResponse::success(command_id, serde_json::json!({ "success": success }))
    }

    /// Handle COMPARE AND SET command
    pub async fn handle_compare_and_set_command(
        &self,
        command_id: Option<String>,
        key: String,
        expected_value: String,
        new_value: String,
        ttl: Option<u64>
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let script =
            dbx_crates::adapter::redis::primitives::string::RedisString::compare_and_set_with_ttl_script();
        let ttl = ttl.unwrap_or(0);
        let result: i32 = match
            redis_handler.redis
                .string()
                .eval_script(&script, &[&key], &[&expected_value, &new_value, &ttl.to_string()])
        {
            Ok(result) => result,
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                return WebSocketResponse::error(
                    command_id,
                    error_response.error.clone().unwrap_or_default()
                );
            }
        };

        let success = result == 1;
        WebSocketResponse::success(command_id, serde_json::json!({ "success": success }))
    }

    /// Handle DEL command for multiple keys
    pub async fn handle_del_command(
        &self,
        command_id: Option<String>,
        keys: Vec<String>
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
            serde_json::json!({ "deleted_count": deleted_count })
        )
    }

    /// Handle EXISTS command for multiple keys
    pub async fn handle_exists_command_multiple(
        &self,
        command_id: Option<String>,
        keys: Vec<String>
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let mut exists_count = 0;
        for key in keys {
            if let Ok(exists) = redis_handler.redis.string().exists(&key) {
                if exists {
                    exists_count += 1;
                }
            }
        }
        WebSocketResponse::success(command_id, serde_json::json!({ "exists_count": exists_count }))
    }
}
