/**
 * Hash operation request types
 */

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
 * Hash operation types
 */

export interface HashOperation {
  key: string;
  field: string;
  value: string;
}
