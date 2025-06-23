/**
 * WebSocket configuration and message types
 */

export interface WebSocketConfig {
  url: string;
  onOpen?: (event: Event) => void;
  onMessage?: (message: any) => void;
  onError?: (error: Event) => void;
  onClose?: (event: Event) => void;
}

// String WebSocket Messages
export type StringWsMessage =
  | { type: "get"; key: string }
  | { type: "set"; key: string; value: string; ttl?: number }
  | { type: "del"; key: string }
  | { type: "info"; key: string }
  | { type: "batch_get"; keys: string[] }
  | { type: "batch_set"; operations: Array<{ key: string; value: string; ttl?: number }> }
  | { type: "result"; key: string; value?: string }
  | { type: "batch_result"; keys: string[]; values: (string | null)[] }
  | { type: "info_result"; info?: { ttl: number; type: string } }
  | { type: "deleted"; key: string; deleted: boolean }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

// Hash WebSocket Messages
export type HashWsMessage =
  | { type: "get"; key: string; field: string }
  | { type: "set"; key: string; field: string; value: string }
  | { type: "get_all"; key: string }
  | { type: "del"; key: string; field: string }
  | { type: "exists"; key: string; field: string }
  | { type: "batch_set"; key: string; fields: Array<[string, string]> }
  | { type: "result"; key: string; field?: string; value?: string }
  | { type: "all_result"; key: string; fields: Record<string, string> }
  | { type: "deleted"; key: string; field: string; deleted: boolean }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

// Set WebSocket Messages
export type SetWsMessage =
  | { type: "add"; key: string; member: string }
  | { type: "remove"; key: string; member: string }
  | { type: "members"; key: string }
  | { type: "exists"; key: string; member: string }
  | { type: "cardinality"; key: string }
  | { type: "intersect"; keys: string[] }
  | { type: "union"; keys: string[] }
  | { type: "difference"; keys: string[] }
  | { type: "result"; key: string; value?: any }
  | { type: "error"; data: string }
  | { type: "ping" }
  | { type: "pong" };

// Admin WebSocket Messages
export type AdminWsMessage =
  | { type: "ping" }
  | { type: "info"; section?: string }
  | { type: "dbsize" }
  | { type: "time" }
  | { type: "version" }
  | { type: "health" }
  | { type: "status" }
  | { type: "memory_stats" }
  | { type: "client_stats" }
  | { type: "server_stats" }
  | { type: "config_set"; parameter: string; value: string }
  | { type: "config_get"; parameter: string }
  | { type: "config_get_all" }
  | { type: "config_resetstat" }
  | { type: "config_rewrite" }
  | { type: "flushdb" }
  | { type: "flushall" }
  | { type: "ping_result"; response: string }
  | { type: "info_result"; info: string }
  | { type: "dbsize_result"; size: number }
  | { type: "time_result"; seconds: number; microseconds: number }
  | { type: "version_result"; version: string }
  | { type: "health_result"; health: { status: string; uptime: number; version: string } }
  | {
      type: "status_result";
      status: { connected_clients: number; used_memory: number; total_commands_processed: number };
    }
  | { type: "memory_stats_result"; stats: Record<string, string> }
  | { type: "client_stats_result"; stats: Record<string, string> }
  | { type: "server_stats_result"; stats: Record<string, string> }
  | { type: "config_get_result"; parameter: string; value: string }
  | { type: "config_get_all_result"; config: Record<string, string> }
  | { type: "config_set_result"; parameter: string; value: string }
  | { type: "config_resetstat_result" }
  | { type: "config_rewrite_result" }
  | { type: "flushdb_result" }
  | { type: "flushall_result" }
  | { type: "error"; data: string };
