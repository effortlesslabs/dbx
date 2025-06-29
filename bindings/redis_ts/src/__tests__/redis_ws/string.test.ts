import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
// Note: This import will work after the NAPI module is built
// For now, we'll use a placeholder import that will be resolved at runtime
const nativeBinding = require("../../../index.js");
const { DbxWsClient } = nativeBinding;

describe("Redis WebSocket String Operations", () => {
  let client: any;
  const TEST_WS_URL = process.env.REDIS_WS_URL || "ws://localhost:3000/redis_ws";

  beforeAll(async () => {
    try {
      console.log("Creating WebSocket client with URL:", TEST_WS_URL);
      client = new DbxWsClient(TEST_WS_URL);
      console.log("WebSocket client created successfully");
    } catch (error) {
      console.error("Error creating WebSocket client:", error);
      throw error;
    }
  });

  describe("Basic Functionality", () => {
    it("should handle ping pong via WebSocket", async () => {
      // Test basic NAPI functionality
      console.log("Testing basic NAPI functionality...");
      const testResult = client.testMethod();
      console.log("testMethod result:", testResult);
      expect(testResult).toBe("hello from napi");

      // Test WebSocket client creation
      const stringClient = client.string();
      console.log("stringClient created:", stringClient);
      expect(stringClient).toBeDefined();

      // Test that the string client has the expected methods
      expect(typeof stringClient.get).toBe("function");
      expect(typeof stringClient.set).toBe("function");
      expect(typeof stringClient.setSimple).toBe("function");
      expect(typeof stringClient.setWithTtl).toBe("function");
      expect(typeof stringClient.delete).toBe("function");
      expect(typeof stringClient.info).toBe("function");
      expect(typeof stringClient.batchGet).toBe("function");
      expect(typeof stringClient.batchSet).toBe("function");
      expect(typeof stringClient.getByPatterns).toBe("function");

      console.log("WebSocket client methods are available");
    });
  });

  describe("String Operations", () => {
    let stringClient: any;
    const testKey = "test:string:ws";
    const testValue = "test_value_ws";

    beforeEach(async () => {
      stringClient = client.string();
      // Clean up before each test
      await stringClient.delete(testKey);
    });

    afterAll(async () => {
      // Clean up after all tests
      const cleanupClient = client.string();
      await cleanupClient.delete(testKey);
      await cleanupClient.delete("test:string:ws:ttl");
      await cleanupClient.delete("test:string:ws:batch1");
      await cleanupClient.delete("test:string:ws:batch2");
      await cleanupClient.delete("test:string:ws:batch3");
    });

    it("should get a string value via WebSocket", async () => {
      console.log(`Testing get operation: key=${testKey}`);

      try {
        const result = await stringClient.get(testKey);
        console.log("get result:", result);
        // The result should be null if the key doesn't exist
        expect(result).toBeNull();
        console.log("✅ get operation successful!");
      } catch (error) {
        console.log("❌ get error:", error);
        throw error;
      }
    });

    it("should set and get a string value via WebSocket", async () => {
      console.log(`Testing set and get operations: key=${testKey}, value=${testValue}`);

      try {
        // Set the value
        const setResult = await stringClient.setSimple(testKey, testValue);
        console.log("setSimple result:", setResult);
        expect(setResult).toBe(true);

        // Get the value
        const getResult = await stringClient.get(testKey);
        console.log("get result:", getResult);
        expect(getResult).toBe(testValue);

        console.log("✅ set and get operations successful!");
      } catch (error) {
        console.log("❌ set/get error:", error);
        throw error;
      }
    });

    it("should set a string value with TTL via WebSocket", async () => {
      const ttlKey = "test:string:ws:ttl";
      const ttlValue = "ttl_test_value";
      const ttl = 10; // 10 seconds

      console.log(`Testing setWithTtl operation: key=${ttlKey}, value=${ttlValue}, ttl=${ttl}`);

      try {
        // Set the value with TTL
        const setResult = await stringClient.setWithTtl(ttlKey, ttlValue, ttl);
        console.log("setWithTtl result:", setResult);
        expect(setResult).toBe(true);

        // Get the value immediately
        const getResult = await stringClient.get(ttlKey);
        console.log("get result:", getResult);
        expect(getResult).toBe(ttlValue);

        console.log("✅ setWithTtl operation successful!");
      } catch (error) {
        console.log("❌ setWithTtl error:", error);
        throw error;
      }
    });

    it("should set a string value with optional TTL via WebSocket", async () => {
      const optionalTtlKey = "test:string:ws:optional_ttl";
      const optionalTtlValue = "optional_ttl_test_value";
      const optionalTtl = 5; // 5 seconds

      console.log(
        `Testing set with optional TTL: key=${optionalTtlKey}, value=${optionalTtlValue}, ttl=${optionalTtl}`
      );

      try {
        // Set the value with optional TTL
        const setResult = await stringClient.set(optionalTtlKey, optionalTtlValue, optionalTtl);
        console.log("set result:", setResult);
        expect(setResult).toBe(true);

        // Get the value immediately
        const getResult = await stringClient.get(optionalTtlKey);
        console.log("get result:", getResult);
        expect(getResult).toBe(optionalTtlValue);

        console.log("✅ set with optional TTL operation successful!");
      } catch (error) {
        console.log("❌ set with optional TTL error:", error);
        throw error;
      }
    });

    it("should delete a string value via WebSocket", async () => {
      console.log(`Testing delete operation: key=${testKey}`);

      try {
        // First set a value
        await stringClient.setSimple(testKey, testValue);

        // Verify it exists
        const getResult = await stringClient.get(testKey);
        expect(getResult).toBe(testValue);

        // Delete the value
        const deleteResult = await stringClient.delete(testKey);
        console.log("delete result:", deleteResult);
        expect(deleteResult).toBe(true);

        // Verify it's deleted
        const getAfterDelete = await stringClient.get(testKey);
        expect(getAfterDelete).toBeNull();

        console.log("✅ delete operation successful!");
      } catch (error) {
        console.log("❌ delete error:", error);
        throw error;
      }
    });

    it("should return false when deleting non-existent key", async () => {
      const nonExistentKey = "non:existent:key:ws";

      console.log(`Testing delete non-existent key: key=${nonExistentKey}`);

      try {
        const deleteResult = await stringClient.delete(nonExistentKey);
        console.log("delete result:", deleteResult);
        expect(deleteResult).toBe(false);

        console.log("✅ delete non-existent key operation successful!");
      } catch (error) {
        console.log("❌ delete non-existent key error:", error);
        throw error;
      }
    });

    it("should get string information via WebSocket", async () => {
      console.log(`Testing info operation: key=${testKey}`);

      try {
        // First set a value
        await stringClient.setSimple(testKey, testValue);

        // Get string info
        const infoResult = await stringClient.info(testKey);
        console.log("info result:", infoResult);

        expect(infoResult).toBeDefined();
        expect(infoResult.key).toBe(testKey);
        expect(infoResult.value).toBe(testValue);
        expect(infoResult.type).toBe("string");
        expect(typeof infoResult.size).toBe("number");

        console.log("✅ info operation successful!");
      } catch (error) {
        console.log("❌ info error:", error);
        throw error;
      }
    });

    it("should return null for info of non-existent key", async () => {
      const nonExistentKey = "non:existent:info:key:ws";

      console.log(`Testing info for non-existent key: key=${nonExistentKey}`);

      try {
        const infoResult = await stringClient.info(nonExistentKey);
        console.log("info result:", infoResult);
        expect(infoResult).toBeNull();

        console.log("✅ info for non-existent key operation successful!");
      } catch (error) {
        console.log("❌ info for non-existent key error:", error);
        throw error;
      }
    });

    it("should batch get multiple strings via WebSocket", async () => {
      const keys = ["test:string:ws:batch1", "test:string:ws:batch2", "test:string:ws:batch3"];
      const values = ["batch_value_1", "batch_value_2", "batch_value_3"];

      console.log(`Testing batchGet operation: keys=${keys.join(", ")}`);

      try {
        // Set multiple values
        for (let i = 0; i < keys.length; i++) {
          await stringClient.setSimple(keys[i], values[i]);
        }

        // Batch get the values
        const batchGetResult = await stringClient.batchGet(keys);
        console.log("batchGet result:", batchGetResult);

        expect(batchGetResult).toBeDefined();
        expect(batchGetResult.length).toBe(keys.length);
        expect(batchGetResult[0]).toBe(values[0]);
        expect(batchGetResult[1]).toBe(values[1]);
        expect(batchGetResult[2]).toBe(values[2]);

        console.log("✅ batchGet operation successful!");
      } catch (error) {
        console.log("❌ batchGet error:", error);
        throw error;
      }
    });

    it("should batch set multiple strings via WebSocket", async () => {
      const operations = [
        { key: "test:string:ws:batch1", value: "new_batch_value_1", ttl: 60 },
        { key: "test:string:ws:batch2", value: "new_batch_value_2" },
        { key: "test:string:ws:batch3", value: "new_batch_value_3", ttl: 120 },
      ];

      console.log(`Testing batchSet operation: ${operations.length} operations`);

      try {
        // Batch set the values
        await stringClient.batchSet(operations);

        // Verify the values were set
        const keys = operations.map((op) => op.key);
        const batchGetResult = await stringClient.batchGet(keys);

        expect(batchGetResult[0]).toBe("new_batch_value_1");
        expect(batchGetResult[1]).toBe("new_batch_value_2");
        expect(batchGetResult[2]).toBe("new_batch_value_3");

        console.log("✅ batchSet operation successful!");
      } catch (error) {
        console.log("❌ batchSet error:", error);
        throw error;
      }
    });

    it("should get strings by patterns via WebSocket", async () => {
      console.log(`Testing getByPatterns operation`);

      try {
        // Set some test values with patterns
        await stringClient.setSimple("test:pattern:1", "pattern_value_1");
        await stringClient.setSimple("test:pattern:2", "pattern_value_2");
        await stringClient.setSimple("other:pattern:3", "other_value_3");

        // Get strings by pattern
        const patterns = ["test:pattern:*"];
        const grouped = true;
        const patternResult = await stringClient.getByPatterns(patterns, grouped);
        console.log("getByPatterns result:", patternResult);

        expect(patternResult).toBeDefined();
        expect(typeof patternResult).toBe("string");

        // Parse the JSON result
        const parsedResult = JSON.parse(patternResult);
        expect(parsedResult).toBeDefined();

        console.log("✅ getByPatterns operation successful!");
      } catch (error) {
        console.log("❌ getByPatterns error:", error);
        throw error;
      }
    });

    it("should handle mixed operations workflow", async () => {
      console.log("Testing mixed operations workflow");

      try {
        // Set multiple values
        await stringClient.setSimple("workflow:key1", "workflow_value_1");
        await stringClient.setWithTtl("workflow:key2", "workflow_value_2", 30);
        await stringClient.set("workflow:key3", "workflow_value_3", 60);

        // Get individual values
        const value1 = await stringClient.get("workflow:key1");
        const value2 = await stringClient.get("workflow:key2");
        const value3 = await stringClient.get("workflow:key3");

        expect(value1).toBe("workflow_value_1");
        expect(value2).toBe("workflow_value_2");
        expect(value3).toBe("workflow_value_3");

        // Get info for one key
        const info = await stringClient.info("workflow:key1");
        expect(info.key).toBe("workflow:key1");
        expect(info.value).toBe("workflow_value_1");

        // Batch get all keys
        const batchResult = await stringClient.batchGet([
          "workflow:key1",
          "workflow:key2",
          "workflow:key3",
        ]);
        expect(batchResult.length).toBe(3);
        expect(batchResult[0]).toBe("workflow_value_1");
        expect(batchResult[1]).toBe("workflow_value_2");
        expect(batchResult[2]).toBe("workflow_value_3");

        // Delete one key
        const deleteResult = await stringClient.delete("workflow:key1");
        expect(deleteResult).toBe(true);

        // Verify it's deleted
        const deletedValue = await stringClient.get("workflow:key1");
        expect(deletedValue).toBeNull();

        console.log("✅ mixed operations workflow successful!");
      } catch (error) {
        console.log("❌ mixed operations workflow error:", error);
        throw error;
      }
    });
  });
});
