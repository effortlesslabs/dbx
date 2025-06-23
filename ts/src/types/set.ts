/**
 * Set operation request types
 */

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
 * Set operation types
 */

export interface SetOperation {
  key: string;
  member: string;
}
