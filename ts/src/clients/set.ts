import { BaseClient } from "./base";
import { MoveSetMemberRequest, BatchSetMembersRequest } from "../types";

/**
 * Set client for Redis set operations
 */
export class SetClient extends BaseClient {
  /**
   * Add member to set
   */
  async addMember(key: string, member: string): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/set/${encodeURIComponent(key)}`, {
      method: "POST",
      data: JSON.stringify({ member }),
    });
  }

  /**
   * Remove member from set
   */
  async removeMember(key: string, member: string): Promise<number> {
    return this.makeRequest<number>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/${encodeURIComponent(member)}`,
      {
        method: "DELETE",
      }
    );
  }

  /**
   * Get set members
   */
  async getMembers(key: string): Promise<string[]> {
    return this.makeRequest<string[]>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/members`
    );
  }

  /**
   * Check if member exists in set
   */
  async memberExists(key: string, member: string): Promise<boolean> {
    return this.makeRequest<boolean>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/${encodeURIComponent(member)}/exists`
    );
  }

  /**
   * Get set cardinality (number of members)
   */
  async getCardinality(key: string): Promise<number> {
    return this.makeRequest<number>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/cardinality`
    );
  }

  /**
   * Get random set member
   */
  async getRandomMember(key: string): Promise<string | null> {
    return this.makeRequest<string | null>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/random`
    );
  }

  /**
   * Pop random set member
   */
  async popRandomMember(key: string): Promise<string | null> {
    return this.makeRequest<string | null>(
      `${this.baseUrl}/redis/set/${encodeURIComponent(key)}/pop`,
      {
        method: "POST",
      }
    );
  }

  /**
   * Move set member to another set
   */
  async moveMember(key: string, member: string, destination: string): Promise<boolean> {
    const payload: MoveSetMemberRequest = { member, destination };
    return this.makeRequest<boolean>(`${this.baseUrl}/redis/set/${encodeURIComponent(key)}/move`, {
      method: "POST",
      data: JSON.stringify(payload),
    });
  }

  /**
   * Intersect sets
   */
  async intersect(keys: string[]): Promise<string[]> {
    return this.makeRequest<string[]>(`${this.baseUrl}/redis/set/intersect`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }

  /**
   * Union sets
   */
  async union(keys: string[]): Promise<string[]> {
    return this.makeRequest<string[]>(`${this.baseUrl}/redis/set/union`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }

  /**
   * Difference of sets
   */
  async difference(keys: string[]): Promise<string[]> {
    return this.makeRequest<string[]>(`${this.baseUrl}/redis/set/difference`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }

  /**
   * Batch add set members
   */
  async batchAddMembers(setMembers: BatchSetMembersRequest): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/set/batch/add`, {
      method: "POST",
      data: JSON.stringify(setMembers),
    });
  }

  /**
   * Batch remove set members
   */
  async batchRemoveMembers(setMembers: BatchSetMembersRequest): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/set/batch/remove`, {
      method: "POST",
      data: JSON.stringify(setMembers),
    });
  }

  /**
   * Batch get set members
   */
  async batchGetMembers(keys: string[]): Promise<Record<string, string[]>> {
    return this.makeRequest<Record<string, string[]>>(`${this.baseUrl}/redis/set/batch/get`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }

  /**
   * Batch delete sets
   */
  async batchDelete(keys: string[]): Promise<number> {
    return this.makeRequest<number>(`${this.baseUrl}/redis/set/batch/delete`, {
      method: "POST",
      data: JSON.stringify({ keys }),
    });
  }
}
