use crate::constants::redis_ws;

/// WebSocket error messages
pub const ERROR_BINARY_MESSAGES: &str = "Binary messages not supported";

/// WebSocket welcome message
pub const WELCOME_MESSAGE: &str = "Connected to DBX WebSocket API";
pub const CONNECTION_ID_FIELD: &str = "connection_id";
pub const SUPPORTED_COMMANDS_FIELD: &str = "supported_commands";

/// Get all supported commands as string slice
pub fn get_supported_commands() -> Vec<&'static str> {
    redis_ws::get_supported_commands()
}

/// Supported WebSocket commands (for backward compatibility)
pub const SUPPORTED_COMMANDS: &[&str] = &[
    // String commands
    "get",
    "set",
    "delete",
    "exists",
    "ttl",
    "incr",
    "incrby",
    "setnx",
    "cas",
    // Batch commands
    "batch_get",
    "batch_set",
    "batch_delete",
    "batch_incr",
    "batch_incrby",
    // Set commands
    "sadd",
    "srem",
    "smembers",
    "scard",
    "sismember",
    "spop",
    // Hash commands
    "hset",
    "hget",
    "hdel",
    "hexists",
    "hlen",
    "hkeys",
    "hvals",
    "hgetall",
    "hmset",
    "hmget",
    // Key commands
    "keys",
    "del",
    // Admin commands
    "flush_all",
    "flush_db",
    "db_size",
    "info",
    // Utility commands
    "list_keys",
    "ping",
    "subscribe",
    "unsubscribe",
];
