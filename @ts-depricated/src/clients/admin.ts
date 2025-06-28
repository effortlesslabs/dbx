import { BaseClient } from "./base";
import { AdminHealthCheck as HealthCheck, AdminServerStatus as ServerStatus } from "../types";

/**
 * Admin client for server management operations
 */
export class AdminClient extends BaseClient {
  /**
   * Ping server
   */
  async ping(): Promise<string> {
    return this.makeRequest<string>(`${this.baseUrl}/redis/admin/ping`);
  }

  /**
   * Get server info
   */
  async info(section?: string): Promise<string> {
    const url = section
      ? `${this.baseUrl}/redis/admin/info/${encodeURIComponent(section)}`
      : `${this.baseUrl}/redis/admin/info`;
    return this.makeRequest<string>(url);
  }

  /**
   * Get Redis database size
   */
  async dbSize(): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/admin/dbsize`);
  }

  /**
   * Get server time
   */
  async time(): Promise<[number, number]> {
    return this.makeRequest<[number, number]>(`${this.baseUrl}/redis/admin/time`);
  }

  /**
   * Get server version
   */
  async version(): Promise<string> {
    return this.makeRequest<string>(`${this.baseUrl}/redis/admin/version`);
  }

  /**
   * Health check
   */
  async health(): Promise<HealthCheck> {
    return this.makeRequest<HealthCheck>(`${this.baseUrl}/redis/admin/health`);
  }

  /**
   * Get server status
   */
  async status(): Promise<ServerStatus> {
    return this.makeRequest<ServerStatus>(`${this.baseUrl}/redis/admin/status`);
  }

  /**
   * Get memory statistics
   */
  async memoryStats(): Promise<Record<string, string>> {
    return this.makeRequest<Record<string, string>>(`${this.baseUrl}/redis/admin/stats/memory`);
  }

  /**
   * Get client statistics
   */
  async clientStats(): Promise<Record<string, string>> {
    return this.makeRequest<Record<string, string>>(`${this.baseUrl}/redis/admin/stats/clients`);
  }

  /**
   * Get server statistics
   */
  async serverStats(): Promise<Record<string, string>> {
    return this.makeRequest<Record<string, string>>(`${this.baseUrl}/redis/admin/stats/server`);
  }

  /**
   * Set configuration parameter
   */
  async configSet(parameter: string, value: string): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/admin/config/set`, {
      method: "POST",
      data: JSON.stringify({ parameter, value }),
    });
  }

  /**
   * Get configuration parameter
   */
  async configGet(parameter: string): Promise<string> {
    return this.makeRequest<string>(
      `${this.baseUrl}/redis/admin/config/get/${encodeURIComponent(parameter)}`
    );
  }

  /**
   * Get all configuration
   */
  async configGetAll(): Promise<Record<string, string>> {
    return this.makeRequest<Record<string, string>>(`${this.baseUrl}/redis/admin/config/all`);
  }

  /**
   * Reset statistics
   */
  async configResetStat(): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/admin/config/resetstat`, {
      method: "POST",
    });
  }

  /**
   * Rewrite configuration
   */
  async configRewrite(): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/admin/config/rewrite`, {
      method: "POST",
    });
  }

  /**
   * Flush current database
   */
  async flushDb(): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/admin/flushdb`, {
      method: "DELETE",
    });
  }

  /**
   * Flush all databases
   */
  async flushAll(): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/admin/flushall`, {
      method: "DELETE",
    });
  }
}
