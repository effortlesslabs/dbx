import { DbxClient } from "./client";
export { DbxClient } from "./client";
export * from "./types";

// Convenience function to create a new DBX client
export function createDbxClient(
  baseUrl: string,
  options?: {
    timeout?: number;
    headers?: Record<string, string>;
  }
) {
  return new DbxClient({
    baseUrl,
    ...options,
  });
}
