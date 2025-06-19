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
 * Rate limiter request
 */
export interface RateLimiterRequest {
  key: string;
  limit: number;
  window: number;
}

/**
 * Multi-counter request
 */
export interface MultiCounterRequest {
  counters: [string, number][];
}

/**
 * Multi-set TTL request
 */
export interface MultiSetTtlRequest {
  key_values: Record<string, string>;
  ttl: number;
}

/**
 * Health check response
 */
export interface HealthResponse {
  service: string;
  status: string;
  timestamp: string;
}

/**
 * Info response
 */
export interface InfoResponse {
  service: string;
  version: string;
  database_type: string;
  database_url: string;
  host: string;
  port: number;
  pool_size: number;
  timestamp: string;
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
 * Database types supported by DBX
 */
export type DatabaseType = 'redis' | 'postgres' | 'mongodb' | 'sqlite'; 