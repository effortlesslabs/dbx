/**
 * Types module index - exports all types from modular structure
 */

// Common types
export * from "./common";

// Operation-specific types
export * from "./string";
export * from "./set";
export * from "./hash";

// Admin types - explicitly re-export to avoid conflicts
export {
  AdminWebSocketMessage,
  StringWebSocketMessage,
  HashWebSocketMessage,
  SetWebSocketMessage,
  WebSocketMessage,
  WebSocketResponse,
  // Re-export with aliases to avoid conflicts
  HealthCheck as AdminHealthCheck,
  ServerStatus as AdminServerStatus,
  StringOperation as AdminStringOperation,
  StringInfo as AdminStringInfo,
} from "./admin";

// WebSocket types - explicitly re-export to avoid conflicts
export {
  WebSocketConfig as WsConfig,
  StringWsMessage,
  HashWsMessage,
  SetWsMessage,
  AdminWsMessage,
} from "./websocket";
