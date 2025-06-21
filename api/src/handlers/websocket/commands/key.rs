use crate::models::WebSocketResponse;

impl super::WebSocketCommandProcessor {
    pub async fn handle_expire_command(
        &self,
        id: Option<String>,
        key: String,
        seconds: i64,
    ) -> WebSocketResponse {
        WebSocketResponse::error(id, "Key commands not yet implemented".to_string())
    }

    pub async fn handle_keys_command(
        &self,
        id: Option<String>,
        pattern: String,
    ) -> WebSocketResponse {
        WebSocketResponse::error(id, "Key commands not yet implemented".to_string())
    }

    pub async fn handle_rename_command(
        &self,
        id: Option<String>,
        key: String,
        new_key: String,
    ) -> WebSocketResponse {
        WebSocketResponse::error(id, "Key commands not yet implemented".to_string())
    }

    pub async fn handle_type_command(&self, id: Option<String>, key: String) -> WebSocketResponse {
        WebSocketResponse::error(id, "Key commands not yet implemented".to_string())
    }
}
