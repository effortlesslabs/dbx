/// API endpoints and paths
pub struct ApiEndpoints;

impl ApiEndpoints {
    /// Health check endpoint
    pub const HEALTH: &'static str = "/health";

    /// Server info endpoint
    pub const INFO: &'static str = "/info";

    /// WebSocket endpoint
    pub const WEBSOCKET: &'static str = "/ws";

    /// Redis API base path
    pub const REDIS_API_BASE: &'static str = "/api/v1/redis";

    /// Strings endpoint
    pub const STRINGS: &'static str = "/strings";

    /// Scripts endpoint
    pub const SCRIPTS: &'static str = "/scripts";

    /// Keys endpoint
    pub const KEYS: &'static str = "/keys";
}

/// API response values
pub struct ApiResponses;

impl ApiResponses {
    /// Success status
    pub const SUCCESS_STATUS: &'static str = "ok";

    /// Database type field name
    pub const DATABASE_TYPE_FIELD: &'static str = "database_type";

    /// Redis connected field name
    pub const REDIS_CONNECTED_FIELD: &'static str = "redis_connected";

    /// Timestamp field name
    pub const TIMESTAMP_FIELD: &'static str = "timestamp";

    /// Name field name
    pub const NAME_FIELD: &'static str = "name";

    /// Version field name
    pub const VERSION_FIELD: &'static str = "version";

    /// Redis URL field name
    pub const REDIS_URL_FIELD: &'static str = "redis_url";

    /// Pool size field name
    pub const POOL_SIZE_FIELD: &'static str = "pool_size";
}
