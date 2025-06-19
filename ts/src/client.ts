import {
  ApiResponse,
  BooleanValue,
  CompareAndSetRequest,
  DeleteResponse,
  ExistsResponse,
  IncrByRequest,
  IntegerValue,
  KeyValues,
  KeysResponse,
  MultiCounterRequest,
  MultiSetTtlRequest,
  RateLimiterRequest,
  SetIfNotExistsRequest,
  SetManyRequest,
  SetRequest,
  StringValue,
  TtlResponse,
  HealthResponse,
  InfoResponse,
  DbxConfig,
} from "./types";

/**
 * DBX Client - TypeScript SDK for DBX API
 */
export class DbxClient {
  private baseUrl: string;
  private timeout: number;
  private headers: Record<string, string>;

  constructor(config: DbxConfig) {
    this.baseUrl = config.baseUrl.replace(/\/$/, ""); // Remove trailing slash
    this.timeout = config.timeout || 10000;
    this.headers = {
      "Content-Type": "application/json",
      ...config.headers,
    };
  }

  /**
   * Make HTTP request with fetch
   */
  private async makeRequest<T>(url: string, options: RequestInit = {}): Promise<T> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...this.headers,
          ...options.headers,
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        const errorMessage =
          typeof errorData === "object" && errorData !== null && "error" in errorData
            ? String(errorData.error)
            : `HTTP ${response.status}: ${response.statusText}`;
        throw new Error(errorMessage);
      }

      const data = await response.json();
      return data as T;
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof Error) {
        throw error;
      }
      throw new Error("Request failed");
    }
  }

  /**
   * Health check endpoint
   */
  async health(): Promise<HealthResponse> {
    return this.makeRequest<HealthResponse>(`${this.baseUrl}/health`);
  }

  /**
   * Info endpoint
   */
  async info(): Promise<InfoResponse> {
    return this.makeRequest<InfoResponse>(`${this.baseUrl}/info`);
  }

  // String Operations

  /**
   * Get a string value by key
   */
  async getString(key: string): Promise<string> {
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get string value");
    }
    return response.data.value;
  }

  /**
   * Set a string value
   */
  async setString(key: string, value: string, ttl?: number): Promise<string> {
    const payload: SetRequest = { value, ...(ttl && { ttl }) };
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to set string value");
    }
    return response.data.value;
  }

  /**
   * Delete a string value
   */
  async deleteString(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}`,
      {
        method: "DELETE",
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to delete string value");
    }
    return response.data.deleted_count;
  }

  /**
   * Check if a key exists
   */
  async exists(key: string): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<ExistsResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/exists`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to check key existence");
    }
    return response.data.exists;
  }

  /**
   * Get TTL for a key
   */
  async getTtl(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<TtlResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/ttl`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get TTL");
    }
    return response.data.ttl;
  }

  /**
   * Increment a counter
   */
  async incr(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/incr`,
      {
        method: "POST",
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to increment counter");
    }
    return response.data.value;
  }

  /**
   * Increment a counter by a specific amount
   */
  async incrBy(key: string, increment: number): Promise<number> {
    const payload: IncrByRequest = { increment };
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/incrby`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to increment counter by amount");
    }
    return response.data.value;
  }

  /**
   * Set a value only if the key doesn't exist
   */
  async setNx(key: string, value: string, ttl?: number): Promise<boolean> {
    const payload: SetIfNotExistsRequest = { value, ...(ttl && { ttl }) };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/strings/${encodeURIComponent(key)}/setnx`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to set value if not exists");
    }
    return response.data.value;
  }

  /**
   * Compare and set a value
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
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to compare and set value");
    }
    return response.data.value;
  }

  /**
   * Batch set multiple key-value pairs
   */
  async batchSet(keyValues: Record<string, string>, ttl?: number): Promise<Record<string, string>> {
    const payload: SetManyRequest = { key_values: keyValues, ...(ttl && { ttl }) };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/set`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch set values");
    }
    return response.data.key_values;
  }

  /**
   * Batch get multiple values by keys
   */
  async batchGet(keys: string[]): Promise<Record<string, string>> {
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/get?${keys.map((k) => `keys=${encodeURIComponent(k)}`).join("&")}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch get values");
    }
    return response.data.key_values;
  }

  /**
   * Batch delete multiple keys
   */
  async batchDelete(keys: string[]): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/delete`,
      {
        method: "DELETE",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch delete keys");
    }
    return response.data.deleted_count;
  }

  /**
   * Batch increment multiple counters
   */
  async batchIncr(keys: string[]): Promise<number[]> {
    const payload: MultiCounterRequest = { counters: keys.map((k) => [k, 1]) };
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/incr`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch increment counters");
    }
    return response.data.values;
  }

  /**
   * List keys matching a pattern
   */
  async listKeys(pattern?: string): Promise<string[]> {
    const url = pattern
      ? `${this.baseUrl}/api/v1/redis/keys?pattern=${encodeURIComponent(pattern)}`
      : `${this.baseUrl}/api/v1/redis/keys`;
    const response = await this.makeRequest<ApiResponse<KeysResponse>>(url);
    if (!response.success || !response.data) {
      throw new Error("Failed to list keys");
    }
    return response.data.keys;
  }

  /**
   * Check if a key exists (alias for exists)
   */
  async keyExists(key: string): Promise<boolean> {
    return this.exists(key);
  }

  /**
   * Get TTL for a key (alias for getTtl)
   */
  async keyTtl(key: string): Promise<number> {
    return this.getTtl(key);
  }

  /**
   * Delete a key (alias for deleteString)
   */
  async deleteKey(key: string): Promise<number> {
    return this.deleteString(key);
  }

  /**
   * Rate limiter implementation
   */
  async rateLimiter(key: string, limit: number, window: number): Promise<boolean> {
    const payload: RateLimiterRequest = { key, limit, window };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/rate-limiter`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to check rate limiter");
    }
    return response.data.value;
  }

  /**
   * Multi-counter operations
   */
  async multiCounter(counters: [string, number][]): Promise<number[]> {
    const payload: MultiCounterRequest = { counters };
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/multi-counter`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to perform multi-counter operation");
    }
    return response.data.values;
  }

  /**
   * Multi-set with TTL
   */
  async multiSetTtl(
    keyValues: Record<string, string>,
    ttl: number
  ): Promise<Record<string, string>> {
    const payload: MultiSetTtlRequest = { key_values: keyValues, ttl };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/multi-set-ttl`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to perform multi-set TTL operation");
    }
    return response.data.key_values;
  }
}
