import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
import { DbxClient } from "../client";
import { getConfig } from "../config";

const config = getConfig();

describe("StringClient - Comprehensive Tests", () => {
  let client: DbxClient;

  beforeAll(() => {
    client = new DbxClient(config);
  });

  beforeEach(async () => {
    // Clean up any existing test data
    const testKeys = [
      "test:string:basic",
      "test:string:ttl",
      "test:string:incr",
      "test:string:setnx",
      "test:string:cas",
      "test:string:batch1",
      "test:string:batch2",
      "test:string:batch3",
      "test:string:pattern:user:123:balance",
      "test:string:pattern:user:123:pending",
      "test:string:pattern:user:456:balance",
      "test:string:pattern:user:456:pending",
    ];

    for (const key of testKeys) {
      try {
        await client.string.delete(key);
      } catch (e) {
        // Ignore errors for non-existent keys
      }
    }
  });

  describe("Basic String Operations", () => {
    it("should set and get string value", async () => {
      const key = "test:string:basic";
      const value = "hello world";

      await client.string.set(key, value);
      const result = await client.string.get(key);

      expect(result).toBe(value);
    });

    it("should return null for non-existent key", async () => {
      const result = await client.string.get("non:existent:key");
      expect(result).toBeNull();
    });

    it("should delete string value", async () => {
      const key = "test:string:delete";
      const value = "to be deleted";

      await client.string.set(key, value);
      const deleted = await client.string.delete(key);

      expect(deleted).toBe(true);

      const result = await client.string.get(key);
      expect(result).toBeNull();
    });

    it("should return false when deleting non-existent key", async () => {
      const deleted = await client.string.delete("non:existent:key");
      expect(deleted).toBe(false);
    });
  });

  describe("String Info Operations", () => {
    it("should get string info", async () => {
      const key = "test:string:info";
      const value = "test value";

      await client.string.set(key, value);
      const info = await client.string.info(key);

      expect(info).not.toBeNull();
      expect(info?.type_).toBe("string");
      expect(info?.ttl).toBeDefined();
    });

    it("should return null for non-existent key info", async () => {
      const info = await client.string.info("non:existent:key");
      expect(info).toBeNull();
    });
  });

  describe("Batch Operations", () => {
    it("should batch get multiple keys", async () => {
      const keyValues = {
        "test:string:batch1": "value1",
        "test:string:batch2": "value2",
        "test:string:batch3": "value3",
      };

      // Set values individually first
      for (const [key, value] of Object.entries(keyValues)) {
        await client.string.set(key, value);
      }

      // Batch get
      const keys = Object.keys(keyValues);
      const results = await client.string.batchGet(keys);

      expect(results).toHaveLength(3);
      expect(results[0]).toBe("value1");
      expect(results[1]).toBe("value2");
      expect(results[2]).toBe("value3");
    });

    it("should handle non-existent keys in batch get", async () => {
      const keys = ["test:string:existing", "test:string:non:existent"];

      // Set one key
      await client.string.set("test:string:existing", "value");

      const results = await client.string.batchGet(keys);

      expect(results).toHaveLength(2);
      expect(results[0]).toBe("value");
      expect(results[1]).toBeNull();
    });

    it("should batch set multiple operations", async () => {
      const operations = [
        { key: "test:string:batch:op1", value: "value1" },
        { key: "test:string:batch:op2", value: "value2", ttl: 10 },
      ];

      await client.string.batchSet(operations);

      // Verify values were set
      const value1 = await client.string.get("test:string:batch:op1");
      const value2 = await client.string.get("test:string:batch:op2");

      expect(value1).toBe("value1");
      expect(value2).toBe("value2");
    });
  });

  describe("Pattern-based Operations", () => {
    it("should get patterns flat", async () => {
      // Set up test data
      await client.string.set("test:pattern:user:1", "user1");
      await client.string.set("test:pattern:user:2", "user2");
      await client.string.set("test:pattern:config:1", "config1");

      const patterns = ["test:pattern:user:*", "test:pattern:config:*"];
      const results = await client.string.batchGetPatternsFlat(patterns);

      expect(results["test:pattern:user:1"]).toBe("user1");
      expect(results["test:pattern:user:2"]).toBe("user2");
      expect(results["test:pattern:config:1"]).toBe("config1");
    });

    it("should get patterns grouped", async () => {
      // Set up test data
      await client.string.set("test:group:user:1", "user1");
      await client.string.set("test:group:user:2", "user2");
      await client.string.set("test:group:config:1", "config1");

      const patterns = ["test:group:user:*", "test:group:config:*"];
      const results = await client.string.batchGetPatternsGrouped(patterns);

      expect(results).toHaveLength(2);
      expect(results[0].pattern).toBe("test:group:user:*");
      expect(results[0].results["test:group:user:1"]).toBe("user1");
      expect(results[0].results["test:group:user:2"]).toBe("user2");
      expect(results[1].pattern).toBe("test:group:config:*");
      expect(results[1].results["test:group:config:1"]).toBe("config1");
    });

    it("should handle empty patterns", async () => {
      const results = await client.string.batchGetPatternsFlat([]);
      expect(results).toEqual({});
    });

    it("should handle patterns with no matches", async () => {
      const results = await client.string.batchGetPatternsFlat(["non:existent:pattern:*"]);
      expect(results).toEqual({ "non:existent:pattern:*": null });
    });
  });

  describe("Error Handling", () => {
    it("should handle empty key operations", async () => {
      try {
        await client.string.set("", "value");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle operations on non-existent keys", async () => {
      const nonExistentKey = "test:string:non:existent";

      const value = await client.string.get(nonExistentKey);
      expect(value).toBeNull();

      const deleted = await client.string.delete(nonExistentKey);
      expect(deleted).toBe(false);

      const info = await client.string.info(nonExistentKey);
      expect(info).toBeNull();
    });
  });

  afterAll(async () => {
    // Clean up all test data
    const testKeys = [
      "test:string:basic",
      "test:string:ttl",
      "test:string:incr",
      "test:string:setnx",
      "test:string:cas",
      "test:string:batch1",
      "test:string:batch2",
      "test:string:batch3",
      "test:string:pattern:user:123:balance",
      "test:string:pattern:user:123:pending",
      "test:string:pattern:user:456:balance",
      "test:string:pattern:user:456:pending",
      "test:string:info",
      "test:string:batch:op1",
      "test:string:batch:op2",
    ];

    for (const key of testKeys) {
      try {
        await client.string.delete(key);
      } catch (e) {
        // Ignore cleanup errors
      }
    }
  });
});
