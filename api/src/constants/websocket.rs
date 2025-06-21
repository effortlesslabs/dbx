/// WebSocket message constants
pub const WS_ENDPOINT: &str = "/redis_ws";
pub const WS_PROTOCOL: &str = "ws";

/// WebSocket message types
pub const MESSAGE_TYPE_COMMAND: &str = "command";
pub const MESSAGE_TYPE_RESPONSE: &str = "response";
pub const MESSAGE_TYPE_ERROR: &str = "error";

/// WebSocket connection constants
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB
pub const CONNECTION_TIMEOUT: u64 = 300; // 5 minutes
pub const PING_INTERVAL: u64 = 30; // 30 seconds

/// WebSocket error messages
pub const ERROR_INVALID_MESSAGE: &str = "Invalid message format";
pub const ERROR_UNSUPPORTED_COMMAND: &str = "Unsupported command";
pub const ERROR_REDIS_CONNECTION: &str = "Redis connection error";
pub const ERROR_SERIALIZATION: &str = "Message serialization error";
pub const ERROR_BINARY_MESSAGES: &str = "Binary messages not supported";

/// WebSocket welcome message
pub const WELCOME_MESSAGE: &str = "Connected to DBX WebSocket API";
pub const CONNECTION_ID_FIELD: &str = "connection_id";
pub const SUPPORTED_COMMANDS_FIELD: &str = "supported_commands";

/// Supported WebSocket commands
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
