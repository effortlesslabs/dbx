import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
import { DbxClient } from "../client";
import { getConfig } from "../config";

const config = getConfig();

describe("SetClient - Comprehensive Tests", () => {
  let client: DbxClient;

  beforeAll(() => {
    client = new DbxClient(config);
  });

  beforeEach(async () => {
    // Clean up any existing test data
    const testKeys = [
      "test:set:basic",
      "test:set:members",
      "test:set:members:empty",
      "test:set:cardinality",
      "test:set:operations:set1",
      "test:set:operations:set2",
      "test:set:operations:set3",
      "test:set:operations:empty",
      "test:set:non:existent",
    ];

    for (const key of testKeys) {
      try {
        // Try to remove any existing members
        const members = await client.set.getMembers(key);
        for (const member of members) {
          await client.set.removeMember(key, member);
        }
      } catch (e) {
        // Ignore errors for non-existent keys
      }
    }
  });

  describe("Basic Set Member Operations", () => {
    it("should add member to set", async () => {
      const key = "test:set:basic";
      const member = "member1";

      const result = await client.set.addMember(key, member);
      expect(result).toBe(1);

      // Verify member was added
      const members = await client.set.getMembers(key);
      expect(members).toContain(member);
    });

    it("should not add duplicate member", async () => {
      const key = "test:set:basic";
      const member = "member1";

      // Add member
      const result1 = await client.set.addMember(key, member);
      console.log("addMember first result:", result1);
      // Add same member again
      const result2 = await client.set.addMember(key, member);
      console.log("addMember duplicate result:", result2);
      // Verify only one member exists
      const members = await client.set.getMembers(key);
      console.log("getMembers after adds:", members);
      expect(result1).toBe(1);
      expect(result2).toBe(0);
      expect(members).toHaveLength(1);
      expect(members).toContain(member);
    });

    it("should remove member from set", async () => {
      const key = `test:set:basic:${Date.now()}`;
      const member = "member1";

      // Add member
      const addResult = await client.set.addMember(key, member);
      console.log("addMember result:", addResult);

      // Remove member
      const result = await client.set.removeMember(key, member);
      console.log("removeMember result:", result);

      // Verify member was removed
      const members = await client.set.getMembers(key);
      console.log("getMembers after remove:", members);
      expect(result).toBe(1);
      expect(members).not.toContain(member);
    });

    it("should return 0 when removing non-existent member", async () => {
      const key = "test:set:basic";
      const member = "non:existent:member";

      const result = await client.set.removeMember(key, member);
      expect(result).toBe(0);
    });

    it("should check if member exists in set", async () => {
      const key = "test:set:basic";
      const member = "member1";

      // Member doesn't exist initially
      const existsBefore = await client.set.memberExists(key, member);
      expect(existsBefore).toBe(false);

      // Add member
      await client.set.addMember(key, member);

      // Member should exist now
      const existsAfter = await client.set.memberExists(key, member);
      expect(existsAfter).toBe(true);
    });
  });

  describe("Set Retrieval Operations", () => {
    it("should get all set members", async () => {
      const key = "test:set:members";
      const members = ["member1", "member2", "member3"];

      // Add members
      for (const member of members) {
        await client.set.addMember(key, member);
      }

      // Get all members
      const result = await client.set.getMembers(key);

      expect(result).toHaveLength(3);
      expect(result).toContain("member1");
      expect(result).toContain("member2");
      expect(result).toContain("member3");
    });

    it("should return empty array for empty set", async () => {
      const key = "test:set:members:empty";

      const result = await client.set.getMembers(key);
      expect(result).toEqual([]);
    });

    it("should get set cardinality", async () => {
      const key = "test:set:cardinality";

      // Empty set should have cardinality 0
      const cardinalityBefore = await client.set.getCardinality(key);
      expect(cardinalityBefore).toBe(0);

      // Add members
      await client.set.addMember(key, "member1");
      await client.set.addMember(key, "member2");
      await client.set.addMember(key, "member3");

      // Check cardinality
      const cardinalityAfter = await client.set.getCardinality(key);
      expect(cardinalityAfter).toBe(3);
    });
  });

  describe("Set Operations", () => {
    beforeEach(async () => {
      // Set up test data for set operations
      await client.set.addMember("test:set:operations:set1", "member1");
      await client.set.addMember("test:set:operations:set1", "member2");
      await client.set.addMember("test:set:operations:set1", "member3");

      await client.set.addMember("test:set:operations:set2", "member2");
      await client.set.addMember("test:set:operations:set2", "member3");
      await client.set.addMember("test:set:operations:set2", "member4");

      await client.set.addMember("test:set:operations:set3", "member3");
      await client.set.addMember("test:set:operations:set3", "member4");
      await client.set.addMember("test:set:operations:set3", "member5");
    });

    it("should intersect sets", async () => {
      const keys = [
        "test:set:operations:set1",
        "test:set:operations:set2",
        "test:set:operations:set3",
      ];

      const result = await client.set.intersect(keys);

      expect(result).toHaveLength(1);
      expect(result).toContain("member3");
    });

    it("should union sets", async () => {
      const keys = ["test:set:operations:set1", "test:set:operations:set2"];

      const result = await client.set.union(keys);

      expect(result).toHaveLength(4);
      expect(result).toContain("member1");
      expect(result).toContain("member2");
      expect(result).toContain("member3");
      expect(result).toContain("member4");
    });

    it("should get difference of sets", async () => {
      const keys = ["test:set:operations:set1", "test:set:operations:set2"];

      const result = await client.set.difference(keys);

      expect(result).toHaveLength(1);
      expect(result).toContain("member1");
    });

    it("should handle empty sets in operations", async () => {
      const keys = ["test:set:operations:empty", "test:set:operations:set1"];

      const intersectResult = await client.set.intersect(keys);
      expect(intersectResult).toEqual([]);

      const unionResult = await client.set.union(keys);
      expect(unionResult).toHaveLength(3);

      const differenceResult = await client.set.difference(keys);
      expect(differenceResult).toEqual([]);
    });
  });

  describe("Error Handling", () => {
    it("should handle empty key operations", async () => {
      try {
        await client.set.addMember("", "member");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle operations on non-existent sets", async () => {
      const nonExistentKey = "test:set:non:existent";

      // These operations should work on non-existent sets
      const members = await client.set.getMembers(nonExistentKey);
      expect(members).toEqual([]);

      const cardinality = await client.set.getCardinality(nonExistentKey);
      expect(cardinality).toBe(0);

      const exists = await client.set.memberExists(nonExistentKey, "member");
      expect(exists).toBe(false);
    });
  });

  afterAll(async () => {
    // Clean up all test data
    const testKeys = [
      "test:set:basic",
      "test:set:members",
      "test:set:members:empty",
      "test:set:cardinality",
      "test:set:operations:set1",
      "test:set:operations:set2",
      "test:set:operations:set3",
      "test:set:operations:empty",
      "test:set:non:existent",
    ];

    for (const key of testKeys) {
      try {
        // Try to remove any existing members
        const members = await client.set.getMembers(key);
        for (const member of members) {
          await client.set.removeMember(key, member);
        }
      } catch (e) {
        // Ignore cleanup errors
      }
    }
  });
});
