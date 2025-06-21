use crate::{middleware::handle_redis_error, models::WebSocketResponse};
use serde_json;

impl super::WebSocketCommandProcessor {
    pub async fn handle_sadd_command(
        &self,
        id: Option<String>,
        key: String,
        members: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let member_refs: Vec<&str> = members.iter().map(|s| s.as_str()).collect();
        match redis_handler.redis.set().sadd(&key, &member_refs) {
            Ok(_) => WebSocketResponse::success(id, serde_json::json!({ "added": members.len() })),
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_srem_command(
        &self,
        id: Option<String>,
        key: String,
        members: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let member_refs: Vec<&str> = members.iter().map(|s| s.as_str()).collect();
        match redis_handler.redis.set().srem(&key, &member_refs) {
            Ok(_) => {
                WebSocketResponse::success(id, serde_json::json!({ "removed": members.len() }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_smembers_command(
        &self,
        id: Option<String>,
        key: String,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.set().smembers(&key) {
            Ok(members) => {
                WebSocketResponse::success(id, serde_json::json!({ "members": members }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_scard_command(&self, id: Option<String>, key: String) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.set().scard(&key) {
            Ok(cardinality) => {
                WebSocketResponse::success(id, serde_json::json!({ "cardinality": cardinality }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_sismember_command(
        &self,
        id: Option<String>,
        key: String,
        member: String,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.set().sismember(&key, &member) {
            Ok(exists) => WebSocketResponse::success(id, serde_json::json!({ "exists": exists })),
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_spop_command(&self, id: Option<String>, key: String) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.set().spop(&key) {
            Ok(Some(member)) => {
                WebSocketResponse::success(id, serde_json::json!({ "member": member }))
            }
            Ok(None) => WebSocketResponse::success(id, serde_json::json!({ "member": null })),
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_srandmember_command(
        &self,
        id: Option<String>,
        key: String,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler.redis.set().srandmember(&key) {
            Ok(Some(member)) => {
                WebSocketResponse::success(id, serde_json::json!({ "member": member }))
            }
            Ok(None) => WebSocketResponse::success(id, serde_json::json!({ "member": null })),
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_smove_command(
        &self,
        id: Option<String>,
        source: String,
        destination: String,
        member: String,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        match redis_handler
            .redis
            .set()
            .smove(&source, &destination, &member)
        {
            Ok(moved) => WebSocketResponse::success(id, serde_json::json!({ "moved": moved })),
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_sunion_command(
        &self,
        id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match redis_handler.redis.set().sunion(&key_refs) {
            Ok(members) => {
                WebSocketResponse::success(id, serde_json::json!({ "members": members }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_sinter_command(
        &self,
        id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match redis_handler.redis.set().sinter(&key_refs) {
            Ok(members) => {
                WebSocketResponse::success(id, serde_json::json!({ "members": members }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }

    pub async fn handle_sdiff_command(
        &self,
        id: Option<String>,
        keys: Vec<String>,
    ) -> WebSocketResponse {
        let redis_handler = self.get_redis_handler().await;
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match redis_handler.redis.set().sdiff(&key_refs) {
            Ok(members) => {
                WebSocketResponse::success(id, serde_json::json!({ "members": members }))
            }
            Err(e) => {
                let (_, error_response) = handle_redis_error(e);
                WebSocketResponse::error(id, error_response.error.clone().unwrap_or_default())
            }
        }
    }
}
