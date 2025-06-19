/// WebSocket command actions
pub struct WebSocketActions;

impl WebSocketActions {
    /// Get action
    pub const GET: &'static str = "get";

    /// Set action
    pub const SET: &'static str = "set";

    /// Delete action
    pub const DELETE: &'static str = "delete";

    /// Exists action
    pub const EXISTS: &'static str = "exists";

    /// TTL action
    pub const TTL: &'static str = "ttl";

    /// Increment action
    pub const INCR: &'static str = "incr";

    /// Increment by action
    pub const INCRBY: &'static str = "incrby";

    /// Set if not exists action
    pub const SETNX: &'static str = "setnx";

    /// Compare and set action
    pub const CAS: &'static str = "cas";

    /// Batch get action
    pub const BATCH_GET: &'static str = "batch_get";

    /// Batch set action
    pub const BATCH_SET: &'static str = "batch_set";

    /// Batch delete action
    pub const BATCH_DELETE: &'static str = "batch_delete";

    /// Batch increment action
    pub const BATCH_INCR: &'static str = "batch_incr";

    /// Batch increment by action
    pub const BATCH_INCRBY: &'static str = "batch_incrby";

    /// List keys action
    pub const LIST_KEYS: &'static str = "list_keys";

    /// Ping action
    pub const PING: &'static str = "ping";

    /// Subscribe action
    pub const SUBSCRIBE: &'static str = "subscribe";

    /// Unsubscribe action
    pub const UNSUBSCRIBE: &'static str = "unsubscribe";
}

/// WebSocket message fields
pub struct WebSocketFields;

impl WebSocketFields {
    /// ID field
    pub const ID: &'static str = "id";

    /// Command field
    pub const COMMAND: &'static str = "command";

    /// Action field
    pub const ACTION: &'static str = "action";

    /// Params field
    pub const PARAMS: &'static str = "params";

    /// Success field
    pub const SUCCESS: &'static str = "success";

    /// Data field
    pub const DATA: &'static str = "data";

    /// Error field
    pub const ERROR: &'static str = "error";

    /// Timestamp field
    pub const TIMESTAMP: &'static str = "timestamp";
}

/// WebSocket welcome message
pub struct WebSocketWelcome;

impl WebSocketWelcome {
    /// Welcome message
    pub const MESSAGE: &'static str = "Connected to DBX WebSocket API";

    /// Connection ID field
    pub const CONNECTION_ID_FIELD: &'static str = "connection_id";

    /// Supported commands field
    pub const SUPPORTED_COMMANDS_FIELD: &'static str = "supported_commands";

    /// Pong response value
    pub const PONG_VALUE: bool = true;
}
