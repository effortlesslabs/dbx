/**
 * Admin operation types
 */

/**
 * Admin WebSocket message types - matches backend AdminWsMessage
 */
export type AdminWebSocketMessage =
  // Basic Health & Status messages
  | { type: "ping" }
  | { type: "info"; data: { section?: string } }
  | { type: "dbsize" }
  | { type: "time" }
  | { type: "version" }

  // Health Check messages
  | { type: "health" }
  | { type: "status" }

  // Statistics messages
  | { type: "memory_stats" }
  | { type: "client_stats" }
  | { type: "server_stats" }

  // Configuration messages
  | { type: "config_set"; data: { parameter: string; value: string } }
  | { type: "config_get"; data: { parameter: string } }
  | { type: "config_get_all" }
  | { type: "config_resetstat" }
  | { type: "config_rewrite" }

  // Database Management messages
  | { type: "flushdb" }
  | { type: "flushall" }

  // Response messages
  | { type: "ping_result"; data: { response: string } }
  | { type: "info_result"; data: { info: string } }
  | { type: "dbsize_result"; data: { size: number } }
  | { type: "time_result"; data: { seconds: number; microseconds: number } }
  | { type: "version_result"; data: { version: string } }
  | { type: "health_result"; data: { health: HealthCheck } }
  | { type: "status_result"; data: { status: ServerStatus } }
  | { type: "memory_stats_result"; data: { stats: Record<string, string> } }
  | { type: "client_stats_result"; data: { stats: Record<string, string> } }
  | { type: "server_stats_result"; data: { stats: Record<string, string> } }
  | { type: "config_get_result"; data: { parameter: string; value: string } }
  | { type: "config_get_all_result"; data: { config: Record<string, string> } }
  | { type: "config_set_result"; data: { parameter: string; value: string } }
  | { type: "config_resetstat_result" }
  | { type: "config_rewrite_result" }
  | { type: "flushdb_result" }
  | { type: "flushall_result" }

  // Error message
  | { type: "error"; data: string };

/**
 * String WebSocket message types - matches backend StringWsMessage
 */
export type StringWebSocketMessage =
  | { type: "get"; data: { key: string } }
  | { type: "set"; data: { key: string; value: string; ttl?: number } }
  | { type: "del"; data: { key: string } }
  | { type: "info"; data: { key: string } }
  | { type: "batch_get"; data: { keys: string[] } }
  | { type: "batch_set"; data: { operations: StringOperation[] } }
  | { type: "result"; data: { key: string; value?: string } }
  | { type: "batch_result"; data: { keys: string[]; values: (string | null)[] } }
  | { type: "info_result"; data: { info?: StringInfo } }
  | { type: "deleted"; data: { key: string; deleted: boolean } }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

/**
 * Hash WebSocket message types - matches backend HashWsMessage
 */
export type HashWebSocketMessage =
  | { type: "get"; data: { key: string; field: string } }
  | { type: "set"; data: { key: string; field: string; value: string } }
  | { type: "get_all"; data: { key: string } }
  | { type: "del"; data: { key: string; field: string } }
  | { type: "exists"; data: { key: string; field: string } }
  | { type: "batch_set"; data: { key: string; fields: [string, string][] } }
  | { type: "result"; data: { key: string; field?: string; value?: string } }
  | { type: "all_result"; data: { key: string; fields: Record<string, string> } }
  | { type: "deleted"; data: { key: string; field: string; deleted: boolean } }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

/**
 * Set WebSocket message types - matches backend SetWsMessage
 */
export type SetWebSocketMessage =
  | { type: "add"; data: { key: string; member: string } }
  | { type: "remove"; data: { key: string; member: string } }
  | { type: "members"; data: { key: string } }
  | { type: "exists"; data: { key: string; member: string } }
  | { type: "cardinality"; data: { key: string } }
  | { type: "intersect"; data: { keys: string[] } }
  | { type: "union"; data: { keys: string[] } }
  | { type: "difference"; data: { keys: string[] } }
  | { type: "result"; data: { key: string; value?: any } }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

/**
 * WebSocket message wrapper
 */
export interface WebSocketMessage {
  id?: string;
  message:
    | AdminWebSocketMessage
    | StringWebSocketMessage
    | HashWebSocketMessage
    | SetWebSocketMessage;
}

/**
 * WebSocket response
 */
export interface WebSocketResponse {
  id?: string;
  success: boolean;
  data?: any;
  error?: string;
  timestamp: string;
}

/**
 * WebSocket client configuration
 */
export interface WebSocketConfig {
  url: string;
  onMessage?: (response: WebSocketResponse) => void;
  onError?: (error: Event) => void;
  onClose?: (event: Event) => void;
  onOpen?: (event: Event) => void;
}

/**
 * Health check response
 */
export interface HealthCheck {
  is_healthy: boolean;
  ping_response: string;
  database_size: number;
  version: string;
  memory_usage: Record<string, string>;
}

/**
 * Server status response
 */
export interface ServerStatus {
  timestamp: number;
  uptime_seconds: number;
  connected_clients: number;
  used_memory: number;
  total_commands_processed: number;
  keyspace_hits: number;
  keyspace_misses: number;
  version: string;
  role: string;
}

/**
 * String operation for batch operations
 */
export interface StringOperation {
  key: string;
  value: string;
  ttl?: number;
}

/**
 * String info response
 */
export interface StringInfo {
  key: string;
  value: string;
  ttl: number;
  type: string;
  encoding: string;
  refcount: number;
  idletime: number;
  freq: number;
}
