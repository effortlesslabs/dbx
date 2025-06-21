import dotenv from "dotenv";

// Load environment variables from .env file
dotenv.config();

/**
 * Configuration for DBX client
 */
export interface DbxConfig {
  /** Base URL for API endpoints */
  hostUrl: string;
  /** WebSocket URL for real-time connections */
  wsHostUrl: string;
  /** Redis connection URL */
  redisUrl: string;
}

/**
 * Default configuration values
 */
const DEFAULT_CONFIG: DbxConfig = {
  hostUrl: "http://127.0.0.1:3000",
  wsHostUrl: "ws://127.0.0.1:3000/redis_ws",
  redisUrl: "redis://127.0.0.1:6379",
};

/**
 * Get configuration with environment variable overrides
 */
export function getConfig(): DbxConfig {
  return {
    hostUrl: process.env["HOST_URL"] || DEFAULT_CONFIG.hostUrl,
    wsHostUrl: process.env["WS_HOST_URL"] || DEFAULT_CONFIG.wsHostUrl,
    redisUrl: process.env["REDIS_URL"] || DEFAULT_CONFIG.redisUrl,
  };
}

/**
 * Get configuration with custom overrides
 */
export function getConfigWithOverrides(overrides: Partial<DbxConfig>): DbxConfig {
  const config = getConfig();
  return { ...config, ...overrides };
}
