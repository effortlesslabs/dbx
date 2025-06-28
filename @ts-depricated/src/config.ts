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
}

/**
 * Default configuration values
 */
const DEFAULT_CONFIG: DbxConfig = {
  hostUrl: "http://127.0.0.1:3000",
  wsHostUrl: "ws://127.0.0.1:3000/redis_ws",
};

/**
 * Get configuration with environment variable overrides
 */
export function getConfig(): DbxConfig {
  return {
    hostUrl: process.env["HOST_URL"] || DEFAULT_CONFIG.hostUrl,
    wsHostUrl: process.env["WS_HOST_URL"] || DEFAULT_CONFIG.wsHostUrl,
  };
}

/**
 * Get configuration with custom overrides
 */
export function getConfigWithOverrides(overrides: Partial<DbxConfig>): DbxConfig {
  const config = getConfig();
  return { ...config, ...overrides };
}

// SDK configuration file

export const API_BASE_URL = process.env["DBX_API_URL"] || "http://127.0.0.1:3000";

/**
 * Convert config.ts DbxConfig to SDK DbxConfig (types/common)
 */
export function toSdkConfig(config: DbxConfig): import("./types/common").DbxConfig {
  return {
    baseUrl: config.hostUrl,
    // Optionally add timeout/headers if needed
  };
}
