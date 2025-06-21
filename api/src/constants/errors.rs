/// Error messages used throughout the application
pub struct ErrorMessages;

impl ErrorMessages {
    /// Not found error
    pub const NOT_FOUND: &'static str = "Not found";

    /// Internal server error
    pub const INTERNAL_SERVER_ERROR: &'static str = "Internal server error";

    /// Redis ping failed
    pub const REDIS_PING_FAILED: &'static str = "Redis ping failed";

    /// Failed to connect to Redis
    pub const REDIS_CONNECTION_FAILED: &'static str = "Failed to connect to Redis: ";

    /// Failed to create Redis client
    pub const REDIS_CLIENT_CREATION_FAILED: &'static str = "Failed to create Redis client";
}
