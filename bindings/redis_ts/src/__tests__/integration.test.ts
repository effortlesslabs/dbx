import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { TEST_CONFIG, generateTestKey, generateTestValue } from "./setup";

// Import the bindings
let createClient: any;
let createWsClient: any;
let DbxRedisClient: any;
let DbxWsClient: any;

try {
  const bindings = require("../../index.js");
  createClient = bindings.createClient;
  createWsClient = bindings.createWsClient;
  DbxRedisClient = bindings.DbxRedisClient;
  DbxWsClient = bindings.DbxWsClient;
} catch (error) {
  console.error("Failed to load bindings:", error);
  throw error;
}

describe("Integration Tests - HTTP vs WebSocket", () => {
  let httpClient: any;
  let wsClient: any;

  beforeAll(() => {
    httpClient = createClient(TEST_CONFIG.HTTP_URL);
    wsClient = createWsClient(TEST_CONFIG.WS_URL);
    expect(httpClient).toBeDefined();
    expect(wsClient).toBeDefined();
  });

  afterAll(() => {
    // Cleanup if needed
  });

  describe("Cross-Protocol String Operations", () => {
    it("should set via HTTP and get via WebSocket", async () => {
      const key = generateTestKey("cross_http_set_ws_get");
      const value = generateTestValue();

      // Set via HTTP
      const setResult = await httpClient.string().set(key, value);
      expect(setResult).toBe(true);

      // Get via WebSocket
      const getResult = await wsClient.string().get(key);
      expect(getResult).toBe(value);
    });

    it("should set via WebSocket and get via HTTP", async () => {
      const key = generateTestKey("cross_ws_set_http_get");
      const value = generateTestValue();

      // Set via WebSocket
      const setResult = await wsClient.string().set(key, value);
      expect(setResult).toBe(true);

      // Get via HTTP
      const getResult = await httpClient.string().get(key);
      expect(getResult).toBe(value);
    });

    it("should set with TTL via HTTP and verify via WebSocket", async () => {
      const key = generateTestKey("cross_http_ttl_ws_verify");
      const value = generateTestValue();

      // Set with TTL via HTTP
      const setResult = await httpClient.string().set_with_ttl(key, value, 60);
      expect(setResult).toBe(true);

      // Get via WebSocket
      const getResult = await wsClient.string().get(key);
      expect(getResult).toBe(value);

      // Get info via WebSocket
      const info = await wsClient.string().info(key);
      expect(info).toBeDefined();
      expect(info.key).toBe(key);
      expect(info.value).toBe(value);
      expect(info.ttl).toBeGreaterThan(0);
    });

    it("should batch set via HTTP and batch get via WebSocket", async () => {
      const operations = [
        { key: generateTestKey("cross_batch1"), value: generateTestValue(), ttl: null },
        { key: generateTestKey("cross_batch2"), value: generateTestValue(), ttl: 60 },
        { key: generateTestKey("cross_batch3"), value: generateTestValue(), ttl: null },
      ];

      // Batch set via HTTP
      const setResult = await httpClient.string().batch_set(operations);
      expect(setResult).toBe(true);

      // Batch get via WebSocket
      const keys = operations.map((op) => op.key);
      const results = await wsClient.string().batch_get(keys);
      expect(results).toHaveLength(3);
      expect(results[0]).toBe(operations[0].value);
      expect(results[1]).toBe(operations[1].value);
      expect(results[2]).toBe(operations[2].value);
    });
  });

  describe("Cross-Protocol Set Operations", () => {
    it("should add members via HTTP and check via WebSocket", async () => {
      const key = generateTestKey("cross_set_http_add_ws_check");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members via HTTP
      const addResult = await httpClient.set().add_many(key, members);
      expect(addResult).toBe(true);

      // Check members via WebSocket
      for (const member of members) {
        const exists = await wsClient.set().exists(key, member);
        expect(exists).toBe(true);
      }

      // Get all members via WebSocket
      const allMembers = await wsClient.set().members(key);
      expect(allMembers).toHaveLength(3);
      expect(allMembers).toEqual(expect.arrayContaining(members));
    });

    it("should add members via WebSocket and check via HTTP", async () => {
      const key = generateTestKey("cross_set_ws_add_http_check");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members via WebSocket
      const addResult = await wsClient.set().add_many(key, members);
      expect(addResult).toBe(true);

      // Check members via HTTP
      for (const member of members) {
        const exists = await httpClient.set().exists(key, member);
        expect(exists).toBe(true);
      }

      // Get cardinality via HTTP
      const cardinality = await httpClient.set().cardinality(key);
      expect(cardinality).toBe(3);
    });

    it("should perform set operations across protocols", async () => {
      const key1 = generateTestKey("cross_set_op1");
      const key2 = generateTestKey("cross_set_op2");
      const commonMember = generateTestValue();

      // Add members via HTTP
      await httpClient.set().add_many(key1, [commonMember, generateTestValue()]);
      await httpClient.set().add_many(key2, [commonMember, generateTestValue()]);

      // Get intersection via WebSocket
      const intersection = await wsClient.set().intersect([key1, key2]);
      expect(intersection).toContain(commonMember);

      // Get union via WebSocket
      const union = await wsClient.set().union([key1, key2]);
      expect(union.length).toBeGreaterThan(1);
      expect(union).toContain(commonMember);
    });
  });

  describe("Error Handling", () => {
    it("should handle connection errors gracefully", async () => {
      // Test with invalid URLs
      try {
        const invalidHttpClient = createClient("http://invalid-url:9999");
        await invalidHttpClient.string().get("test");
        // Should not reach here
        expect(true).toBe(false);
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle invalid operations consistently", async () => {
      const key = generateTestKey("error_test");
      const value = generateTestValue();

      // Set a string value
      await httpClient.string().set(key, value);

      // Try to use string key for set operations (should work but return expected results)
      const exists = await httpClient.set().exists(key, value);
      expect(exists).toBe(false); // String key doesn't exist as set member
    });
  });

  describe("Performance Comparison", () => {
    it("should measure basic operation performance", async () => {
      const key = generateTestKey("perf_test");
      const value = generateTestValue();

      // HTTP performance
      const httpStart = Date.now();
      await httpClient.string().set(key, value);
      const httpSetTime = Date.now() - httpStart;

      const httpGetStart = Date.now();
      await httpClient.string().get(key);
      const httpGetTime = Date.now() - httpGetStart;

      // WebSocket performance
      const wsStart = Date.now();
      await wsClient.string().set(key, value);
      const wsSetTime = Date.now() - wsStart;

      const wsGetStart = Date.now();
      await wsClient.string().get(key);
      const wsGetTime = Date.now() - wsGetStart;

      // Log performance (not asserting specific times as they vary)
      console.log(`HTTP Set: ${httpSetTime}ms, HTTP Get: ${httpGetTime}ms`);
      console.log(`WS Set: ${wsSetTime}ms, WS Get: ${wsGetTime}ms`);

      // Just ensure operations complete
      expect(httpSetTime).toBeGreaterThan(0);
      expect(httpGetTime).toBeGreaterThan(0);
      expect(wsSetTime).toBeGreaterThan(0);
      expect(wsGetTime).toBeGreaterThan(0);
    });
  });
});
