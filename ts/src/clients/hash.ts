import { BaseClient } from "./base";

/**
 * Hash client for Redis hash operations
 */
export class HashClient extends BaseClient {
  /**
   * Get hash field value
   */
  async getField(key: string, field: string): Promise<string | null> {
    return this.makeRequest<string | null>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}`
    );
  }

  /**
   * Set hash field value
   */
  async setField(key: string, field: string, value: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}`,
      {
        method: "POST",
        data: JSON.stringify({ value }),
      }
    );
  }

  /**
   * Delete hash field
   */
  async deleteField(key: string, field: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}`,
      {
        method: "DELETE",
      }
    );
  }

  /**
   * Check if hash field exists
   */
  async fieldExists(key: string, field: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}/exists`
    );
  }

  /**
   * Increment hash field by integer amount
   */
  async incrementField(key: string, field: string, increment: number): Promise<number> {
    return this.makeRequest<number>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}/increment`,
      {
        method: "POST",
        data: JSON.stringify({ increment }),
      }
    );
  }

  /**
   * Increment hash field by float amount
   */
  async incrementFieldFloat(key: string, field: string, increment: number): Promise<number> {
    return this.makeRequest<number>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}/increment_float`,
      {
        method: "POST",
        data: JSON.stringify({ increment }),
      }
    );
  }

  /**
   * Set hash field if not exists
   */
  async setFieldNx(key: string, field: string, value: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/${encodeURIComponent(field)}/setnx`,
      {
        method: "POST",
        data: JSON.stringify({ value }),
      }
    );
  }

  /**
   * Get all hash fields
   */
  async getAll(key: string): Promise<Record<string, string>> {
    return this.makeRequest<Record<string, string>>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}`
    );
  }

  /**
   * Get multiple hash fields
   */
  async getFields(key: string, fields: string[]): Promise<(string | null)[]> {
    return this.makeRequest<(string | null)[]>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/fields`,
      {
        method: "POST",
        data: JSON.stringify({ fields }),
      }
    );
  }

  /**
   * Set multiple hash fields
   */
  async setMultiple(key: string, fields: Record<string, string>): Promise<void> {
    await this.makeRequest<void>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/batch`, {
      method: "POST",
      data: JSON.stringify({ fields }),
    });
  }

  /**
   * Get hash length (number of fields)
   */
  async getLength(key: string): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/length`);
  }

  /**
   * Get hash field keys
   */
  async getKeys(key: string): Promise<string[]> {
    return this.makeRequest<string[]>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/keys`);
  }

  /**
   * Get hash field values
   */
  async getValues(key: string): Promise<string[]> {
    return this.makeRequest<string[]>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/values`
    );
  }

  /**
   * Get random hash field
   */
  async getRandomField(key: string): Promise<string | null> {
    return this.makeRequest<string | null>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/random`
    );
  }

  /**
   * Get random hash fields
   */
  async getRandomFields(key: string, count: number): Promise<string[]> {
    return this.makeRequest<string[]>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/random_fields`,
      {
        method: "POST",
        data: JSON.stringify({ count }),
      }
    );
  }

  /**
   * Get random hash fields with values
   */
  async getRandomFieldsWithValues(key: string, count: number): Promise<Array<[string, string]>> {
    return this.makeRequest<Array<[string, string]>>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/random_fields_with_values`,
      {
        method: "POST",
        data: JSON.stringify({ count }),
      }
    );
  }

  /**
   * Delete hash
   */
  async delete(key: string): Promise<boolean> {
    return this.makeRequest<boolean>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}`, {
      method: "DELETE",
    });
  }

  /**
   * Check if hash exists
   */
  async exists(key: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/exists`
    );
  }

  /**
   * Get hash TTL
   */
  async getTtl(key: string): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/ttl`);
  }

  /**
   * Set hash TTL
   */
  async setTtl(key: string, ttl: number): Promise<boolean> {
    return this.makeRequest<boolean>(`${this.baseUrl}/redis/hash/${encodeURIComponent(key)}/ttl`, {
      method: "POST",
      data: JSON.stringify({ ttl }),
    });
  }

  /**
   * Batch get hash fields
   */
  async batchGetFields(hashFields: Array<[string, string]>): Promise<(string | null)[]> {
    return this.makeRequest<(string | null)[]>(`${this.baseUrl}/redis/hash/batch/get`, {
      method: "POST",
      data: JSON.stringify({ hash_fields: hashFields }),
    });
  }

  /**
   * Batch set hash fields
   */
  async batchSetFields(
    hashOperations: Array<[string, Array<[string, string]>]>
  ): Promise<boolean[]> {
    return this.makeRequest<boolean[]>(`${this.baseUrl}/redis/hash/batch/set`, {
      method: "POST",
      data: JSON.stringify({ hash_operations: hashOperations }),
    });
  }

  /**
   * Batch delete hash fields
   */
  async batchDeleteFields(hashFields: Array<[string, string[]]>): Promise<number[]> {
    return this.makeRequest<number[]>(`${this.baseUrl}/redis/hash/batch/delete`, {
      method: "POST",
      data: JSON.stringify({ hash_fields: hashFields }),
    });
  }

  /**
   * Batch check hash fields
   */
  async batchCheckFields(hashFields: Array<[string, string]>): Promise<boolean[]> {
    return this.makeRequest<boolean[]>(`${this.baseUrl}/redis/hash/batch/exists`, {
      method: "POST",
      data: JSON.stringify({ hash_fields: hashFields }),
    });
  }

  /**
   * Batch get hash lengths
   */
  async batchGetLengths(keys: string[]): Promise<number[]> {
    return this.makeRequest<number[]>(`${this.baseUrl}/redis/hash/batch/length`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }
}
