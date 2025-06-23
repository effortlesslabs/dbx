import { BaseClient } from "./base";

/**
 * Common client for shared Redis operations
 */
export class CommonClient extends BaseClient {
  /**
   * List keys with optional pattern
   */
  async listKeys(pattern?: string): Promise<string[]> {
    const url = pattern
      ? `${this.baseUrl}/redis/keys?pattern=${encodeURIComponent(pattern)}`
      : `${this.baseUrl}/redis/keys`;

    return this.makeRequest<string[]>(url);
  }

  /**
   * Check if key exists
   */
  async keyExists(key: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/keys/${encodeURIComponent(key)}/exists`
    );
  }

  /**
   * Get key TTL
   */
  async keyTtl(key: string): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/keys/${encodeURIComponent(key)}/ttl`);
  }

  /**
   * Delete key
   */
  async deleteKey(key: string): Promise<boolean> {
    return this.makeRequest<boolean>(`${this.baseUrl}/redis/keys/${encodeURIComponent(key)}`, {
      method: "DELETE",
    });
  }
}
