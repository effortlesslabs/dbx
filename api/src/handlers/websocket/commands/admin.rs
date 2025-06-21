use crate::models::WebSocketResponse;

impl super::WebSocketCommandProcessor {
    pub async fn handle_info_command(&self, id: Option<String>) -> WebSocketResponse {
        WebSocketResponse::error(id, "Admin commands not yet implemented".to_string())
    }

    pub async fn handle_flushdb_command(&self, id: Option<String>) -> WebSocketResponse {
        WebSocketResponse::error(id, "Admin commands not yet implemented".to_string())
    }

    pub async fn handle_flushall_command(&self, id: Option<String>) -> WebSocketResponse {
        WebSocketResponse::error(id, "Admin commands not yet implemented".to_string())
    }

    pub async fn handle_dbsize_command(&self, id: Option<String>) -> WebSocketResponse {
        WebSocketResponse::error(id, "Admin commands not yet implemented".to_string())
    }

    pub async fn handle_select_command(&self, id: Option<String>, db: i32) -> WebSocketResponse {
        WebSocketResponse::error(id, "Admin commands not yet implemented".to_string())
    }
}
