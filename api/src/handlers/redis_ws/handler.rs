use axum::{ extract::{ ws::WebSocketUpgrade, State }, response::IntoResponse };
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

use super::connection::RedisWsConnection;
use crate::handlers::redis::RedisHandler;
use crate::models::{ RedisWsCommand, RedisWsMessage, RedisWsResponse };
use crate::handlers::redis_ws::commands::string::{
    handle_string_get,
    handle_string_set,
    handle_string_delete,
    handle_string_exists,
    handle_string_ttl,
    handle_string_incr,
    handle_string_incr_by,
    handle_string_set_nx,
    handle_string_compare_and_set,
};
use crate::handlers::redis_ws::commands::batch::{
    handle_batch_get,
    handle_batch_set,
    handle_batch_delete,
    handle_batch_incr,
    handle_batch_incr_by,
};
use crate::handlers::redis_ws::commands::set::{
    handle_set_sadd,
    handle_set_srem,
    handle_set_smembers,
    handle_set_scard,
    handle_set_sismember,
    handle_set_spop,
};
use crate::handlers::redis_ws::commands::hash::{
    handle_hash_hset,
    handle_hash_hget,
    handle_hash_hdel,
    handle_hash_hexists,
    handle_hash_hlen,
    handle_hash_hkeys,
    handle_hash_hvals,
    handle_hash_hgetall,
    handle_hash_hmset,
    handle_hash_hmget,
};
use crate::handlers::redis_ws::commands::key::{ handle_key_keys, handle_key_del };
use crate::handlers::redis_ws::commands::admin::{
    handle_admin_flush_all,
    handle_admin_flush_db,
    handle_admin_db_size,
    handle_admin_info,
};
use crate::handlers::redis_ws::commands::utility::{
    handle_utility_list_keys,
    handle_utility_ping,
    handle_utility_subscribe,
    handle_utility_unsubscribe,
};

/// RedisWs handler that processes JSON commands
#[derive(Clone)]
pub struct RedisWsHandler {
    pub redis_handler: Arc<Mutex<RedisHandler>>,
}

impl RedisWsHandler {
    /// Create a new RedisWs handler
    pub fn new(redis_handler: Arc<Mutex<RedisHandler>>) -> Self {
        Self {
            redis_handler,
        }
    }

    /// Handle RedisWs upgrade and connection
    pub async fn handle_redis_ws(
        ws: WebSocketUpgrade,
        State(handler): State<RedisWsHandler>
    ) -> impl IntoResponse {
        let connection_id = Uuid::new_v4().to_string();
        info!("RedisWs connection established: {}", connection_id);

        ws.on_upgrade(|socket| async move {
            RedisWsConnection::handle_connection(socket, handler, connection_id).await
        })
    }

    pub async fn handle_message(&self, message: RedisWsMessage) -> RedisWsResponse {
        self.process_command(message.command, message.id).await
    }

    pub async fn get_redis(&self) -> Result<dbx_crates::adapter::redis::Redis, redis::RedisError> {
        let handler = self.redis_handler.lock().await;
        handler.get_redis()
    }

    async fn process_command(
        &self,
        command: RedisWsCommand,
        id: Option<String>
    ) -> RedisWsResponse {
        match command {
            // String commands
            RedisWsCommand::Get { key } => {
                match handle_string_get(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Set { key, value, ttl } => {
                match handle_string_set(self, key, value, ttl).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Delete { key } => {
                match handle_string_delete(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Exists { key } => {
                match handle_string_exists(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Ttl { key } => {
                match handle_string_ttl(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Incr { key } => {
                match handle_string_incr(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::IncrBy { key, increment } => {
                match handle_string_incr_by(self, key, increment).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::SetNx { key, value, ttl } => {
                match handle_string_set_nx(self, key, value, ttl).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::CompareAndSet { key, expected_value, new_value, ttl } => {
                match
                    handle_string_compare_and_set(self, key, expected_value, new_value, ttl).await
                {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Batch commands
            RedisWsCommand::BatchGet { keys } => {
                match handle_batch_get(self, keys).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::BatchSet { key_values, ttl } => {
                match handle_batch_set(self, key_values, ttl).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::BatchDelete { keys } => {
                match handle_batch_delete(self, keys).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::BatchIncr { keys } => {
                match handle_batch_incr(self, keys).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::BatchIncrBy { key_increments } => {
                match handle_batch_incr_by(self, key_increments).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Set commands
            RedisWsCommand::Sadd { key, members } => {
                match handle_set_sadd(self, key, members).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Srem { key, members } => {
                match handle_set_srem(self, key, members).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Smembers { key } => {
                match handle_set_smembers(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Scard { key } => {
                match handle_set_scard(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Sismember { key, member } => {
                match handle_set_sismember(self, key, member).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Spop { key } => {
                match handle_set_spop(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Hash commands
            RedisWsCommand::Hset { key, field, value } => {
                match handle_hash_hset(self, key, field, value).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hget { key, field } => {
                match handle_hash_hget(self, key, field).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hdel { key, field } => {
                match handle_hash_hdel(self, key, field).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hexists { key, field } => {
                match handle_hash_hexists(self, key, field).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hlen { key } => {
                match handle_hash_hlen(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hkeys { key } => {
                match handle_hash_hkeys(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hvals { key } => {
                match handle_hash_hvals(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hgetall { key } => {
                match handle_hash_hgetall(self, key).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hmset { key, fields } => {
                match handle_hash_hmset(self, key, fields).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Hmget { key, fields } => {
                match handle_hash_hmget(self, key, fields).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Key commands
            RedisWsCommand::Keys { pattern } => {
                match handle_key_keys(self, pattern).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Del { keys } => {
                match handle_key_del(self, keys).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Admin commands
            RedisWsCommand::FlushAll => {
                match handle_admin_flush_all(self).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::FlushDb => {
                match handle_admin_flush_db(self).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::DbSize => {
                match handle_admin_db_size(self).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Info => {
                match handle_admin_info(self).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Utility commands
            RedisWsCommand::ListKeys { pattern } => {
                match handle_utility_list_keys(self, pattern).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Ping => {
                match handle_utility_ping(self).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Subscribe { channels } => {
                match handle_utility_subscribe(self, channels).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }
            RedisWsCommand::Unsubscribe { channels } => {
                match handle_utility_unsubscribe(self, channels).await {
                    Ok(response) => response,
                    Err(e) => RedisWsResponse::error(id, e.to_string()),
                }
            }

            // Placeholder for unimplemented commands
            _ => RedisWsResponse::error(id, "Command not yet implemented".to_string()),
        }
    }
}
