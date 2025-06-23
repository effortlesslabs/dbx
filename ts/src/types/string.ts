/**
 * String operation types
 */

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
  ttl: number;
  type: string;
}
