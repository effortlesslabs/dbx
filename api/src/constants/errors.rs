/// Error messages used throughout the application
pub struct ErrorMessages;

impl ErrorMessages {
    /// Key not found error
    pub const KEY_NOT_FOUND: &'static str = "Key not found";

    /// Not found error
    pub const NOT_FOUND: &'static str = "Not found";

    /// Internal server error
    pub const INTERNAL_SERVER_ERROR: &'static str = "Internal server error";

    /// Binary messages not supported
    pub const BINARY_MESSAGES_NOT_SUPPORTED: &'static str = "Binary messages not supported";

    /// Invalid JSON error prefix
    pub const INVALID_JSON_PREFIX: &'static str = "Invalid JSON: ";

    /// List keys not yet implemented
    pub const LIST_KEYS_NOT_IMPLEMENTED: &'static str = "List keys not yet implemented";

    /// Subscribe not yet implemented
    pub const SUBSCRIBE_NOT_IMPLEMENTED: &'static str = "Subscribe not yet implemented";

    /// Unsubscribe not yet implemented
    pub const UNSUBSCRIBE_NOT_IMPLEMENTED: &'static str = "Unsubscribe not yet implemented";

    /// Redis ping failed
    pub const REDIS_PING_FAILED: &'static str = "Redis ping failed";

    /// Failed to connect to Redis
    pub const REDIS_CONNECTION_FAILED: &'static str = "Failed to connect to Redis: ";

    /// Failed to create Redis client
    pub const REDIS_CLIENT_CREATION_FAILED: &'static str = "Failed to create Redis client";
}
