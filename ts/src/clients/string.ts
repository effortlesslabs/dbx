import { BaseClient } from "./base";
import {
  ApiResponse,
  IntegerValue,
  BooleanValue,
  DeleteResponse,
  ExistsResponse,
  TtlResponse,
  IncrByRequest,
  SetIfNotExistsRequest,
  CompareAndSetRequest,
  SetManyRequest,
  StringOperation,
  StringInfo,
  BatchGetPatternsRequest,
  BatchGetPatternsResponse,
} from "../types";

/**
 * String client for Redis string operations
 */
export class StringClient extends BaseClient {
  /**
   * Get a string value by key
   */
  async get(key: string): Promise<string | null> {
    return this.makeRequest<string | null>(
      `${this.baseUrl}/redis/string/${encodeURIComponent(key)}`
    );
  }

  /**
   * Set a string value
   */
  async set(key: string, value: string, ttl?: number): Promise<void> {
    const payload: { value: string; ttl?: number } = { value };
    if (ttl !== undefined) {
      payload.ttl = ttl;
    }
    await this.makeRequest<void>(`${this.baseUrl}/redis/string/${encodeURIComponent(key)}`, {
      method: "POST",
      data: JSON.stringify(payload),
    });
  }

  /**
   * Delete a string value
   */
  async delete(key: string): Promise<boolean> {
    return this.makeRequest<boolean>(`${this.baseUrl}/redis/string/${encodeURIComponent(key)}`, {
      method: "DELETE",
    });
  }

  /**
   * Check if a key exists
   */
  async exists(key: string): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<ExistsResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/exists`
    );
    return this.handleApiResponse(response).exists;
  }

  /**
   * Get TTL for a key
   */
  async getTtl(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<TtlResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/ttl`
    );
    return this.handleApiResponse(response).ttl;
  }

  /**
   * Increment a numeric value
   */
  async incr(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/incr`,
      {
        method: "POST",
      }
    );
    return this.handleApiResponse(response).value;
  }

  /**
   * Increment by a specific amount
   */
  async incrBy(key: string, increment: number): Promise<number> {
    const payload: IncrByRequest = { increment };
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/incrby`,
      {
        method: "POST",
        data: JSON.stringify(payload),
      }
    );
    return this.handleApiResponse(response).value;
  }

  /**
   * Set if not exists
   */
  async setNx(key: string, value: string, ttl?: number): Promise<boolean> {
    const payload: SetIfNotExistsRequest = { value, ...(ttl && { ttl }) };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/setnx`,
      {
        method: "POST",
        data: JSON.stringify(payload),
      }
    );
    return this.handleApiResponse(response).value;
  }

  /**
   * Compare and set
   */
  async compareAndSet(
    key: string,
    expectedValue: string,
    newValue: string,
    ttl?: number
  ): Promise<boolean> {
    const payload: CompareAndSetRequest = {
      expected_value: expectedValue,
      new_value: newValue,
      ...(ttl && { ttl }),
    };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/cas`,
      {
        method: "POST",
        data: JSON.stringify(payload),
      }
    );
    return this.handleApiResponse(response).value;
  }

  /**
   * Batch set multiple key-value pairs
   */
  async batchSet(keyValues: Record<string, string>, ttl?: number): Promise<Record<string, string>> {
    const payload: SetManyRequest = { key_values: keyValues, ...(ttl && { ttl }) };
    const response = await this.makeRequest<ApiResponse<{ key_values: Record<string, string> }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/set`,
      {
        method: "POST",
        data: JSON.stringify(payload),
      }
    );
    return this.handleApiResponse(response).key_values;
  }

  /**
   * Batch get multiple keys
   */
  async batchGet(keys: string[]): Promise<(string | null)[]> {
    return this.makeRequest<(string | null)[]>(`${this.baseUrl}/redis/string/batch/get`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }

  /**
   * Batch delete multiple keys
   */
  async batchDelete(keys: string[]): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/delete`,
      {
        method: "POST",
        data: JSON.stringify({ keys }),
      }
    );
    return this.handleApiResponse(response).deleted_count;
  }

  /**
   * Batch increment multiple keys
   */
  async batchIncr(keys: string[]): Promise<number[]> {
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/incr`,
      {
        method: "POST",
        data: JSON.stringify({ keys }),
      }
    );
    return this.handleApiResponse(response).values;
  }

  /**
   * Batch increment by specific amounts
   */
  async batchIncrBy(keyIncrements: [string, number][]): Promise<number[]> {
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/incrby`,
      {
        method: "POST",
        data: JSON.stringify({ key_increments: keyIncrements }),
      }
    );
    return this.handleApiResponse(response).values;
  }

  /**
   * Get string info
   */
  async info(key: string): Promise<StringInfo | null> {
    return this.makeRequest<StringInfo | null>(
      `${this.baseUrl}/redis/string/${encodeURIComponent(key)}/info`
    );
  }

  /**
   * Batch set multiple operations
   */
  async batchSetOperations(operations: StringOperation[]): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/string/batch/set`, {
      method: "POST",
      data: JSON.stringify({ operations }),
    });
  }

  /**
   * Batch get strings by patterns (supports wildcards like *)
   */
  async batchGetPatterns(
    patterns: string[],
    grouped: boolean = false
  ): Promise<BatchGetPatternsResponse> {
    const payload: BatchGetPatternsRequest = { patterns, grouped };
    return this.makeRequest<BatchGetPatternsResponse>(
      `${this.baseUrl}/redis/string/batch/patterns`,
      {
        method: "POST",
        data: JSON.stringify(payload),
      }
    );
  }

  /**
   * Batch get strings by patterns, returning flat results
   */
  async batchGetPatternsFlat(patterns: string[]): Promise<Record<string, string | null>> {
    const response = await this.batchGetPatterns(patterns, false);
    return response.results as Record<string, string | null>;
  }

  /**
   * Batch get strings by patterns, returning grouped results
   */
  async batchGetPatternsGrouped(
    patterns: string[]
  ): Promise<Array<{ pattern: string; results: Record<string, string | null> }>> {
    const response = await this.batchGetPatterns(patterns, true);
    return response.results as Array<{ pattern: string; results: Record<string, string | null> }>;
  }
}
