import { DbxClient } from "./client";
import { getConfig } from "./config";
export { DbxClient } from "./client";
export { getConfig, getConfigWithOverrides, type DbxConfig } from "./config";
export * from "./types";

// Convenience function to create a new DBX client
export function createDbxClient(
  baseUrl?: string,
  options?: {
    timeout?: number;
    headers?: Record<string, string>;
  }
) {
  const config = getConfig();
  return new DbxClient({
    baseUrl: baseUrl || config.hostUrl,
    ...options,
  });
}
