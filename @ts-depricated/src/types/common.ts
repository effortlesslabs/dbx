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
 * Health check response
 */
export interface HealthCheck {
  status: string;
  redis_connected: boolean;
  timestamp: string;
}

/**
 * Server status response
 */
export interface ServerStatus {
  status: string;
  uptime: string;
  connected_clients: number;
  used_memory: string;
  used_memory_peak: string;
  total_commands_processed: number;
  total_connections_received: number;
  keyspace_hits: number;
  keyspace_misses: number;
}

/**
 * SDK configuration options
 */
export interface DbxConfig {
  baseUrl?: string;
  timeout?: number;
  headers?: Record<string, string>;
}

/**
 * Database types supported by DBX
 */
export type DatabaseType = "redis" | "postgres" | "mongodb" | "sqlite";
