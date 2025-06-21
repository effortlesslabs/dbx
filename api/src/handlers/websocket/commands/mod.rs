pub mod batch;
pub mod set;
pub mod string;
pub mod utility;
pub mod hash;
pub mod key;
pub mod admin;

use crate::{
    handlers::redis::RedisHandler,
    models::{ WebSocketCommand, WebSocketMessage, WebSocketResponse },
    middleware::handle_redis_error,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// WebSocket command processor
pub struct WebSocketCommandProcessor {
    redis_handler: Arc<Mutex<RedisHandler>>,
}

impl WebSocketCommandProcessor {
    /// Create a new command processor
    pub fn new(redis_handler: Arc<Mutex<RedisHandler>>) -> Self {
        Self { redis_handler }
    }

    /// Process a WebSocket command and return a response
    pub async fn process_command(&self, ws_message: WebSocketMessage) -> WebSocketResponse {
        let command_id = ws_message.id.clone();

        match ws_message.command {
            // String commands
            WebSocketCommand::Set { key, value, ttl } => {
                self.handle_set_command(command_id, key, value, ttl).await
            }
            WebSocketCommand::Get { key } => { self.handle_get_command(command_id, key).await }
            WebSocketCommand::Delete { key } => {
                self.handle_delete_command(command_id, key).await
            }
            WebSocketCommand::Exists { key } => {
                self.handle_exists_command(command_id, key).await
            }
            WebSocketCommand::Ttl { key } => { self.handle_ttl_command(command_id, key).await }
            WebSocketCommand::Incr { key } => { self.handle_incr_command(command_id, key).await }
            WebSocketCommand::IncrBy { key, increment } => {
                self.handle_incr_by_command(command_id, key, increment).await
            }
            WebSocketCommand::SetNx { key, value, ttl } => {
                self.handle_set_nx_command(command_id, key, value, ttl).await
            }
            WebSocketCommand::CompareAndSet { key, expected_value, new_value, ttl } => {
                self.handle_compare_and_set_command(
                    command_id,
                    key,
                    expected_value,
                    new_value,
                    ttl
                ).await
            }

            // Set commands
            WebSocketCommand::Sadd { key, members } => {
                self.handle_sadd_command(command_id, key, members).await
            }
            WebSocketCommand::Srem { key, members } => {
                self.handle_srem_command(command_id, key, members).await
            }
            WebSocketCommand::Smembers { key } => {
                self.handle_smembers_command(command_id, key).await
            }
            WebSocketCommand::Sismember { key, member } => {
                self.handle_sismember_command(command_id, key, member).await
            }
            WebSocketCommand::Scard { key } => { self.handle_scard_command(command_id, key).await }
            WebSocketCommand::Spop { key } => { self.handle_spop_command(command_id, key).await }
            WebSocketCommand::Srandmember { key } => {
                self.handle_srandmember_command(command_id, key).await
            }
            WebSocketCommand::Sdiff { keys } => {
                self.handle_sdiff_command(command_id, keys).await
            }
            WebSocketCommand::Sinter { keys } => {
                self.handle_sinter_command(command_id, keys).await
            }
            WebSocketCommand::Sunion { keys } => {
                self.handle_sunion_command(command_id, keys).await
            }
            WebSocketCommand::Smove { source, destination, member } => {
                self.handle_smove_command(command_id, source, destination, member).await
            }

            // Hash commands
            WebSocketCommand::Hset { key, field, value } => {
                self.handle_hset_command(command_id, key, field, value).await
            }
            WebSocketCommand::Hget { key, field } => {
                self.handle_hget_command(command_id, key, field).await
            }
            WebSocketCommand::Hdel { key, field } => {
                self.handle_hdel_command(command_id, key, field).await
            }
            WebSocketCommand::Hexists { key, field } => {
                self.handle_hexists_command(command_id, key, field).await
            }
            WebSocketCommand::Hlen { key } => { self.handle_hlen_command(command_id, key).await }
            WebSocketCommand::Hkeys { key } => { self.handle_hkeys_command(command_id, key).await }
            WebSocketCommand::Hvals { key } => { self.handle_hvals_command(command_id, key).await }
            WebSocketCommand::Hgetall { key } => {
                self.handle_hgetall_command(command_id, key).await
            }
            WebSocketCommand::Hmset { key, fields } => {
                self.handle_hmset_command(command_id, key, fields).await
            }
            WebSocketCommand::Hmget { key, fields } => {
                self.handle_hmget_command(command_id, key, fields).await
            }

            // Key commands
            WebSocketCommand::Keys { pattern } => {
                self.handle_keys_command(
                    command_id,
                    pattern.unwrap_or_else(|| "*".to_string())
                ).await
            }
            WebSocketCommand::Del { keys } => { self.handle_del_command(command_id, keys).await }

            // Admin commands
            WebSocketCommand::Ping => { self.handle_ping_command(command_id).await }
            WebSocketCommand::Info => { self.handle_info_command(command_id).await }
            WebSocketCommand::FlushDb => { self.handle_flushdb_command(command_id).await }
            WebSocketCommand::FlushAll => { self.handle_flushall_command(command_id).await }
            WebSocketCommand::DbSize => { self.handle_dbsize_command(command_id).await }

            // Batch commands
            WebSocketCommand::BatchGet { keys } => {
                self.handle_batch_get_command(command_id, keys).await
            }
            WebSocketCommand::BatchSet { key_values, ttl } => {
                self.handle_batch_set_command(command_id, key_values, ttl).await
            }
            WebSocketCommand::BatchDelete { keys } => {
                self.handle_batch_delete_command(command_id, keys).await
            }
            WebSocketCommand::BatchIncr { keys } => {
                self.handle_batch_incr_command(command_id, keys).await
            }
            WebSocketCommand::BatchIncrBy { key_increments } => {
                self.handle_batch_incr_by_command(command_id, key_increments).await
            }

            // Utility commands
            WebSocketCommand::ListKeys { pattern: _ } => {
                self.handle_list_keys_command(command_id).await
            }
            WebSocketCommand::Subscribe { channels: _ } => {
                self.handle_subscribe_command(command_id).await
            }
            WebSocketCommand::Unsubscribe { channels: _ } => {
                self.handle_unsubscribe_command(command_id).await
            }
        }
    }

    // Helper method to get a locked RedisHandler
    async fn get_redis_handler(&self) -> tokio::sync::MutexGuard<RedisHandler> {
        self.redis_handler.lock().await
    }
}
