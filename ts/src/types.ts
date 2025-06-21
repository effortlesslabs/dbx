/**
 * Core API response structure
 */
export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

/**
 * String value response
 */
export interface StringValue {
  value: string;
}

/**
 * Integer value response
 */
export interface IntegerValue {
  value: number;
}

/**
 * Boolean value response
 */
export interface BooleanValue {
  value: boolean;
}

/**
 * Delete response
 */
export interface DeleteResponse {
  deleted_count: number;
}

/**
 * Exists response
 */
export interface ExistsResponse {
  exists: boolean;
}

/**
 * TTL response
 */
export interface TtlResponse {
  ttl: number;
}

/**
 * Key-value pairs response
 */
export interface KeyValues {
  key_values: Record<string, string>;
}

/**
 * Keys response
 */
export interface KeysResponse {
  keys: string[];
}

/**
 * Set request
 */
export interface SetRequest {
  value: string;
  ttl?: number;
}

/**
 * Increment by request
 */
export interface IncrByRequest {
  increment: number;
}

/**
 * Set if not exists request
 */
export interface SetIfNotExistsRequest {
  value: string;
  ttl?: number;
}

/**
 * Compare and set request
 */
export interface CompareAndSetRequest {
  expected_value: string;
  new_value: string;
  ttl?: number;
}

/**
 * Set many request
 */
export interface SetManyRequest {
  key_values: Record<string, string>;
  ttl?: number;
}

/**
 * Move set member request
 */
export interface MoveSetMemberRequest {
  member: string;
  destination: string;
}

/**
 * Set operation request (union, intersection, difference)
 */
export interface SetOperationRequest {
  keys: string[];
}

/**
 * Batch set members request
 */
export interface BatchSetMembersRequest {
  [key: string]: string[];
}

/**
 * Batch hash fields request
 */
export interface BatchHashFieldsRequest {
  [key: string]: Record<string, string>;
}

/**
 * Batch hash field check request
 */
export interface BatchHashFieldCheckRequest {
  [key: string]: string[];
}

/**
 * Batch hash field get request
 */
export interface BatchHashFieldGetRequest {
  [key: string]: string[];
}

/**
 * Batch hash field delete request
 */
export interface BatchHashFieldDeleteRequest {
  [key: string]: string[];
}

/**
 * Health check response
 */
export interface HealthResponse {
  status: string;
  redis_connected: boolean;
  timestamp: string;
}

/**
 * Server info response
 */
export interface ServerInfo {
  name: string;
  version: string;
  redis_url: string;
  pool_size: number;
}

/**
 * SDK configuration options
 */
export interface DbxConfig {
  baseUrl: string;
  timeout?: number;
  headers?: Record<string, string>;
}

/**
 * WebSocket command types
 */
export type WebSocketCommand =
  | { action: "get"; params: { key: string } }
  | { action: "set"; params: { key: string; value: string; ttl?: number } }
  | { action: "delete"; params: { key: string } }
  | { action: "exists"; params: { key: string } }
  | { action: "ttl"; params: { key: string } }
  | { action: "incr"; params: { key: string } }
  | { action: "incrby"; params: { key: string; increment: number } }
  | { action: "setnx"; params: { key: string; value: string; ttl?: number } }
  | {
      action: "cas";
      params: { key: string; expected_value: string; new_value: string; ttl?: number };
    }
  | { action: "batch_get"; params: { keys: string[] } }
  | { action: "batch_set"; params: { key_values: Record<string, string>; ttl?: number } }
  | { action: "batch_delete"; params: { keys: string[] } }
  | { action: "batch_incr"; params: { keys: string[] } }
  | { action: "batch_incrby"; params: { key_increments: [string, number][] } }
  | { action: "list_keys"; params: { pattern?: string } }
  | { action: "ping"; params: {} }
  | { action: "subscribe"; params: { channels: string[] } }
  | { action: "unsubscribe"; params: { channels: string[] } };

/**
 * WebSocket message wrapper
 */
export interface WebSocketMessage {
  id?: string;
  command: WebSocketCommand;
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
 * Database types supported by DBX
 */
export type DatabaseType = "redis" | "postgres" | "mongodb" | "sqlite";
