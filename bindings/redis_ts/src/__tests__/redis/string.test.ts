import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
// Note: This import will work after the NAPI module is built
// For now, we'll use a placeholder import that will be resolved at runtime
const { DbxRedisClient } = require("../../../index.js");

describe("Redis HTTP String Operations", () => {
  let client: any;
  const TEST_BASE_URL = process.env.REDIS_HTTP_URL || "http://localhost:3000";

  beforeAll(async () => {
    client = new DbxRedisClient(TEST_BASE_URL);
  });

  afterAll(async () => {
    // Clean up test data
    const stringClient = client.string();
    await stringClient.delete("test:string:1");
    await stringClient.delete("test:string:2");
    await stringClient.delete("test:string:3");
    await stringClient.delete("test:string:4");
    await stringClient.delete("test:string:5");
    await stringClient.delete("test:pattern:*");
  });

  beforeEach(async () => {
    // Clear test strings before each test
    const stringClient = client.string();
    await stringClient.delete("test:string:1");
    await stringClient.delete("test:string:2");
    await stringClient.delete("test:string:3");
    await stringClient.delete("test:string:4");
    await stringClient.delete("test:string:5");
  });

  describe("set", () => {
    it("should set a string value without TTL", async () => {
      const stringClient = client.string();
      const result = await stringClient.set("test:string:1", "value1", undefined);
      expect(result).toBe(true);
    });

    it("should set a string value with TTL", async () => {
      const stringClient = client.string();
      const result = await stringClient.set("test:string:1", "value1", 60);
      expect(result).toBe(true);
    });

    it("should overwrite existing value", async () => {
      const stringClient = client.string();
      await stringClient.set("test:string:1", "value1", undefined);
      const result = await stringClient.set("test:string:1", "value2", undefined);
      expect(result).toBe(true);
    });
  });

  describe("set_simple", () => {
    it("should set a string value without TTL", async () => {
      const stringClient = client.string();
      const result = await stringClient.setSimple("test:string:1", "value1");
      expect(result).toBe(true);
    });

    it("should overwrite existing value", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:string:1", "value1");
      const result = await stringClient.setSimple("test:string:1", "value2");
      expect(result).toBe(true);
    });
  });

  describe("set_with_ttl", () => {
    it("should set a string value with TTL", async () => {
      const stringClient = client.string();
      const result = await stringClient.setWithTtl("test:string:1", "value1", 60);
      expect(result).toBe(true);
    });

    it("should overwrite existing value with new TTL", async () => {
      const stringClient = client.string();
      await stringClient.setWithTtl("test:string:1", "value1", 30);
      const result = await stringClient.setWithTtl("test:string:1", "value2", 60);
      expect(result).toBe(true);
    });
  });

  describe("get", () => {
    it("should get a string value", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:string:1", "value1");
      const value = await stringClient.get("test:string:1");
      expect(value).toBe("value1");
    });

    it("should return null for non-existent key", async () => {
      const stringClient = client.string();
      const value = await stringClient.get("non-existent:key");
      expect(value).toBeNull();
    });
  });

  describe("delete", () => {
    it("should delete a string value", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:string:1", "value1");
      const result = await stringClient.delete("test:string:1");
      expect(result).toBe(true);

      // Verify it's deleted
      const value = await stringClient.get("test:string:1");
      expect(value).toBeNull();
    });

    it("should return false when deleting non-existent key", async () => {
      const stringClient = client.string();
      const result = await stringClient.delete("non-existent:key");
      expect(result).toBe(false);
    });
  });

  describe("info", () => {
    it("should get string information", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:string:1", "value1");
      const info = await stringClient.info("test:string:1");

      expect(info).toBeDefined();
      expect(info.key).toBe("test:string:1");
      expect(info.value).toBe("value1");
      expect(info.type).toBe("string");
      expect(info.size).toBeGreaterThan(0);
    });

    it("should return null for non-existent key", async () => {
      const stringClient = client.string();
      const info = await stringClient.info("non-existent:key");
      expect(info).toBeNull();
    });
  });

  describe("batch_get", () => {
    it("should get multiple string values", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:string:1", "value1");
      await stringClient.setSimple("test:string:2", "value2");
      await stringClient.setSimple("test:string:3", "value3");

      const values = await stringClient.batchGet([
        "test:string:1",
        "test:string:2",
        "test:string:3",
        "non-existent",
      ]);

      expect(values).toHaveLength(4);
      expect(values[0]).toBe("value1");
      expect(values[1]).toBe("value2");
      expect(values[2]).toBe("value3");
      expect(values[3]).toBeNull();
    });

    it("should handle empty array of keys", async () => {
      const stringClient = client.string();
      const values = await stringClient.batchGet([]);
      expect(values).toEqual([]);
    });
  });

  describe("batch_set", () => {
    it("should set multiple string values", async () => {
      const stringClient = client.string();
      const operations = [
        { key: "test:string:1", value: "value1", ttl: undefined },
        { key: "test:string:2", value: "value2", ttl: 60 },
        { key: "test:string:3", value: "value3", ttl: undefined },
      ];

      await stringClient.batchSet(operations);

      // Verify all values were set
      const value1 = await stringClient.get("test:string:1");
      const value2 = await stringClient.get("test:string:2");
      const value3 = await stringClient.get("test:string:3");

      expect(value1).toBe("value1");
      expect(value2).toBe("value2");
      expect(value3).toBe("value3");
    });

    it("should handle empty array of operations", async () => {
      const stringClient = client.string();
      await stringClient.batchSet([]);
      // Should not throw an error
    });
  });

  describe("get_by_patterns", () => {
    it("should get strings by patterns", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:pattern:1", "value1");
      await stringClient.setSimple("test:pattern:2", "value2");
      await stringClient.setSimple("test:pattern:3", "value3");
      await stringClient.setSimple("other:key", "other_value");

      const result = await stringClient.getByPatterns(["test:pattern:*"], false);
      const parsed = JSON.parse(result);

      expect(parsed).toBeDefined();
      expect(parsed.grouped).toBe(false);
      expect(typeof parsed.results).toBe("object");
      expect(Object.keys(parsed.results).length).toBeGreaterThanOrEqual(3);
    });

    it("should get strings by patterns with grouping", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:pattern:1", "value1");
      await stringClient.setSimple("test:pattern:2", "value2");

      const result = await stringClient.getByPatterns(["test:pattern:*"], true);
      const parsed = JSON.parse(result);

      expect(parsed).toBeDefined();
      expect(parsed.grouped).toBe(true);
      expect(Array.isArray(parsed.results)).toBe(true);
    });

    it("should handle multiple patterns", async () => {
      const stringClient = client.string();
      await stringClient.setSimple("test:pattern:1", "value1");
      await stringClient.setSimple("other:pattern:1", "other_value");

      const result = await stringClient.getByPatterns(["test:pattern:*", "other:pattern:*"], false);
      const parsed = JSON.parse(result);

      expect(parsed).toBeDefined();
      expect(parsed.grouped).toBe(false);
      expect(typeof parsed.results).toBe("object");
      expect(Object.keys(parsed.results).length).toBeGreaterThanOrEqual(2);
    });

    it("should handle empty patterns array", async () => {
      const stringClient = client.string();
      const result = await stringClient.getByPatterns([], false);
      const parsed = JSON.parse(result);

      expect(parsed).toBeDefined();
      expect(parsed.grouped).toBe(false);
      expect(Array.isArray(parsed.results)).toBe(true);
      expect(parsed.results.length).toBe(0);
    });
  });
});
