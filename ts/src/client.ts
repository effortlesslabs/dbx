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
  SetIfNotExistsRequest,
  SetManyRequest,
  SetRequest,
  StringValue,
  TtlResponse,
  HealthResponse,
  ServerInfo,
  DbxConfig,
  MoveSetMemberRequest,
  SetOperationRequest,
  BatchSetMembersRequest,
  BatchHashFieldsRequest,
  BatchHashFieldCheckRequest,
  BatchHashFieldGetRequest,
  BatchHashFieldDeleteRequest,
  WebSocketConfig,
  WebSocketMessage,
  WebSocketResponse,
  WebSocketCommand,
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
  async info(): Promise<ServerInfo> {
    return this.makeRequest<ServerInfo>(`${this.baseUrl}/info`);
  }

  /**
   * Get Redis database size
   */
  async dbSize(): Promise<number> {
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/dbsize`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get database size");
    }
    return response.data.value;
  }

  /**
   * Flush all databases
   */
  async flushAll(): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/flushall`,
      { method: "POST" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to flush all databases");
    }
    return response.data.value;
  }

  /**
   * Flush current database
   */
  async flushDb(): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/flushdb`,
      { method: "POST" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to flush current database");
    }
    return response.data.value;
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
   * Batch get multiple values
   */
  async batchGet(keys: string[]): Promise<Record<string, string>> {
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/get`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
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
        method: "POST",
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
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/incr`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch increment counters");
    }
    return response.data.values;
  }

  /**
   * Batch increment multiple counters by specific amounts
   */
  async batchIncrBy(keyIncrements: [string, number][]): Promise<number[]> {
    const response = await this.makeRequest<ApiResponse<{ values: number[] }>>(
      `${this.baseUrl}/api/v1/redis/strings/batch/incrby`,
      {
        method: "POST",
        body: JSON.stringify({ key_increments: keyIncrements }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch increment counters by amounts");
    }
    return response.data.values;
  }

  // Set Operations

  /**
   * Get all members of a set
   */
  async getSetMembers(key: string): Promise<string[]> {
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get set members");
    }
    const members = response.data.key_values[key];
    return members ? members.split(",").filter((m) => m.length > 0) : [];
  }

  /**
   * Add members to a set
   */
  async addSetMembers(key: string, members: string[]): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}`,
      {
        method: "POST",
        body: JSON.stringify({ value: members.join(",") }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to add set members");
    }
    return response.data.value;
  }

  /**
   * Delete a set
   */
  async deleteSet(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}`,
      { method: "DELETE" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to delete set");
    }
    return response.data.deleted_count;
  }

  /**
   * Check if a member exists in a set
   */
  async setMemberExists(key: string, member: string): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<ExistsResponse>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/exists?member=${encodeURIComponent(member)}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to check set member existence");
    }
    return response.data.exists;
  }

  /**
   * Get the cardinality (size) of a set
   */
  async getSetCardinality(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/cardinality`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get set cardinality");
    }
    return response.data.value;
  }

  /**
   * Get a random member from a set
   */
  async getRandomSetMember(key: string): Promise<string> {
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/random`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get random set member");
    }
    return response.data.value;
  }

  /**
   * Pop a random member from a set
   */
  async popRandomSetMember(key: string): Promise<string> {
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/pop`,
      { method: "POST" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to pop random set member");
    }
    return response.data.value;
  }

  /**
   * Move a member from one set to another
   */
  async moveSetMember(key: string, member: string, destination: string): Promise<boolean> {
    const payload: MoveSetMemberRequest = { member, destination };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/move`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to move set member");
    }
    return response.data.value;
  }

  /**
   * Get the union of multiple sets
   */
  async setUnion(key: string, otherKeys: string[]): Promise<string[]> {
    const payload: SetOperationRequest = { keys: otherKeys };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/union`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get set union");
    }
    const members = response.data.key_values[key];
    return members ? members.split(",").filter((m) => m.length > 0) : [];
  }

  /**
   * Get the intersection of multiple sets
   */
  async setIntersection(key: string, otherKeys: string[]): Promise<string[]> {
    const payload: SetOperationRequest = { keys: otherKeys };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/intersection`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get set intersection");
    }
    const members = response.data.key_values[key];
    return members ? members.split(",").filter((m) => m.length > 0) : [];
  }

  /**
   * Get the difference of multiple sets
   */
  async setDifference(key: string, otherKeys: string[]): Promise<string[]> {
    const payload: SetOperationRequest = { keys: otherKeys };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/sets/${encodeURIComponent(key)}/difference`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get set difference");
    }
    const members = response.data.key_values[key];
    return members ? members.split(",").filter((m) => m.length > 0) : [];
  }

  /**
   * Batch add members to multiple sets
   */
  async batchAddSetMembers(setMembers: BatchSetMembersRequest): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/sets/batch/add`,
      {
        method: "POST",
        body: JSON.stringify(setMembers),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch add set members");
    }
    return response.data.deleted_count;
  }

  /**
   * Batch remove members from multiple sets
   */
  async batchRemoveSetMembers(setMembers: BatchSetMembersRequest): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/sets/batch/remove`,
      {
        method: "POST",
        body: JSON.stringify(setMembers),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch remove set members");
    }
    return response.data.deleted_count;
  }

  /**
   * Batch get members from multiple sets
   */
  async batchGetSetMembers(keys: string[]): Promise<Record<string, string[]>> {
    const response = await this.makeRequest<ApiResponse<Record<string, string[]>>>(
      `${this.baseUrl}/api/v1/redis/sets/batch/members`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch get set members");
    }
    return response.data;
  }

  /**
   * Batch delete multiple sets
   */
  async batchDeleteSets(keys: string[]): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/sets/batch/delete`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch delete sets");
    }
    return response.data.deleted_count;
  }

  // Hash Operations

  /**
   * Get a hash field value
   */
  async getHashField(key: string, field: string): Promise<string> {
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get hash field");
    }
    return response.data.value;
  }

  /**
   * Set a hash field value
   */
  async setHashField(key: string, field: string, value: string): Promise<string> {
    const payload: SetRequest = { value };
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to set hash field");
    }
    return response.data.value;
  }

  /**
   * Delete a hash field
   */
  async deleteHashField(key: string, field: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}`,
      { method: "DELETE" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to delete hash field");
    }
    return response.data.deleted_count;
  }

  /**
   * Check if a hash field exists
   */
  async hashFieldExists(key: string, field: string): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}/exists`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to check hash field existence");
    }
    return response.data.value;
  }

  /**
   * Increment a hash field by a specific amount
   */
  async incrementHashField(key: string, field: string, increment: number): Promise<number> {
    const payload: IncrByRequest = { increment };
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}/incr`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to increment hash field");
    }
    return response.data.value;
  }

  /**
   * Set a hash field only if it doesn't exist
   */
  async setHashFieldNx(key: string, field: string, value: string): Promise<boolean> {
    const payload: SetRequest = { value };
    const response = await this.makeRequest<ApiResponse<BooleanValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/${encodeURIComponent(field)}/setnx`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to set hash field if not exists");
    }
    return response.data.value;
  }

  /**
   * Get the length (number of fields) of a hash
   */
  async getHashLength(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<IntegerValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/length`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get hash length");
    }
    return response.data.value;
  }

  /**
   * Get all field names of a hash
   */
  async getHashKeys(key: string): Promise<string[]> {
    const response = await this.makeRequest<ApiResponse<{ value: string }[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/keys`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get hash keys");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Get all field values of a hash
   */
  async getHashValues(key: string): Promise<string[]> {
    const response = await this.makeRequest<ApiResponse<{ value: string }[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/values`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get hash values");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Get a random field from a hash
   */
  async getRandomHashField(key: string): Promise<string> {
    const response = await this.makeRequest<ApiResponse<StringValue>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/random`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get random hash field");
    }
    return response.data.value;
  }

  /**
   * Get multiple hash fields
   */
  async getMultipleHashFields(key: string, fields: string[]): Promise<string[]> {
    const response = await this.makeRequest<ApiResponse<{ value: string }[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}/mget`,
      {
        method: "POST",
        body: JSON.stringify({ fields }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get multiple hash fields");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Get all fields and values of a hash
   */
  async getHashAll(key: string): Promise<Record<string, string>> {
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get all hash fields");
    }
    return response.data.key_values;
  }

  /**
   * Set multiple hash fields
   */
  async setHashMultiple(
    key: string,
    fields: Record<string, string>
  ): Promise<Record<string, string>> {
    const payload: SetManyRequest = { key_values: fields };
    const response = await this.makeRequest<ApiResponse<KeyValues>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to set multiple hash fields");
    }
    return response.data.key_values;
  }

  /**
   * Delete a hash
   */
  async deleteHash(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/hashes/${encodeURIComponent(key)}`,
      { method: "DELETE" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to delete hash");
    }
    return response.data.deleted_count;
  }

  /**
   * Batch set hash fields across multiple hashes
   */
  async batchSetHashFields(hashFields: BatchHashFieldsRequest): Promise<boolean[]> {
    const payload = Object.entries(hashFields).map(([key, fields]) => [
      key,
      Object.entries(fields).map(([field, value]) => [field, value]),
    ]);
    const response = await this.makeRequest<ApiResponse<BooleanValue[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/set`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch set hash fields");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Batch get hash fields across multiple hashes
   */
  async batchGetHashFields(hashFields: BatchHashFieldGetRequest): Promise<string[]> {
    const payload = Object.entries(hashFields).map(([key, fields]) => [key, fields]);
    const response = await this.makeRequest<ApiResponse<{ value: string }[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/get`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch get hash fields");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Batch delete hash fields across multiple hashes
   */
  async batchDeleteHashFields(hashFields: BatchHashFieldDeleteRequest): Promise<number[]> {
    const payload = Object.entries(hashFields).map(([key, fields]) => [key, fields]);
    const response = await this.makeRequest<ApiResponse<DeleteResponse[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/delete`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch delete hash fields");
    }
    return response.data.map((item) => item.deleted_count);
  }

  /**
   * Batch get all fields from multiple hashes
   */
  async batchGetHashAll(keys: string[]): Promise<Record<string, Record<string, string>>[]> {
    const response = await this.makeRequest<ApiResponse<KeyValues[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/all`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch get all hash fields");
    }
    return response.data.map((item) => ({ key_values: item.key_values }));
  }

  /**
   * Batch check if hash fields exist
   */
  async batchCheckHashFields(hashFields: BatchHashFieldCheckRequest): Promise<boolean[]> {
    const payload = Object.entries(hashFields).map(([key, fields]) => [key, fields]);
    const response = await this.makeRequest<ApiResponse<BooleanValue[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/exists`,
      {
        method: "POST",
        body: JSON.stringify(payload),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch check hash fields");
    }
    return response.data.map((item) => item.value);
  }

  /**
   * Batch get hash lengths
   */
  async batchGetHashLengths(keys: string[]): Promise<number[]> {
    const response = await this.makeRequest<ApiResponse<IntegerValue[]>>(
      `${this.baseUrl}/api/v1/redis/hashes/batch/lengths`,
      {
        method: "POST",
        body: JSON.stringify({ keys }),
      }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to batch get hash lengths");
    }
    return response.data.map((item) => item.value);
  }

  // Key Operations

  /**
   * List all keys (optionally with pattern)
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
   * Check if a key exists
   */
  async keyExists(key: string): Promise<boolean> {
    const response = await this.makeRequest<ApiResponse<ExistsResponse>>(
      `${this.baseUrl}/api/v1/redis/keys/${encodeURIComponent(key)}/exists`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to check key existence");
    }
    return response.data.exists;
  }

  /**
   * Get TTL for a key
   */
  async keyTtl(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<TtlResponse>>(
      `${this.baseUrl}/api/v1/redis/keys/${encodeURIComponent(key)}/ttl`
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to get key TTL");
    }
    return response.data.ttl;
  }

  /**
   * Delete a key
   */
  async deleteKey(key: string): Promise<number> {
    const response = await this.makeRequest<ApiResponse<DeleteResponse>>(
      `${this.baseUrl}/api/v1/redis/keys/${encodeURIComponent(key)}`,
      { method: "DELETE" }
    );
    if (!response.success || !response.data) {
      throw new Error("Failed to delete key");
    }
    return response.data.deleted_count;
  }

  // WebSocket Support

  /**
   * Create a WebSocket connection
   */
  createWebSocket(config: WebSocketConfig): any {
    const ws = new (globalThis as any).WebSocket(config.url);

    if (config.onOpen) {
      ws.onopen = config.onOpen;
    }

    if (config.onMessage) {
      ws.onmessage = (event: MessageEvent) => {
        try {
          const response: WebSocketResponse = JSON.parse(event.data);
          config.onMessage!(response);
        } catch (error) {
          console.error("Failed to parse WebSocket message:", error);
        }
      };
    }

    if (config.onError) {
      ws.onerror = config.onError;
    }

    if (config.onClose) {
      ws.onclose = config.onClose;
    }

    return ws;
  }

  /**
   * Send a WebSocket command
   */
  sendWebSocketCommand(ws: any, command: WebSocketCommand, id?: string): void {
    const message: WebSocketMessage = {
      ...(id && { id }),
      command,
    };
    ws.send(JSON.stringify(message));
  }
}
