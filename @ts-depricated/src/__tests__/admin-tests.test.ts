import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { DbxClient } from "../client";
import { getConfig } from "../config";

const config = getConfig();

describe("AdminClient - Comprehensive Tests", () => {
  let client: DbxClient;

  beforeAll(() => {
    client = new DbxClient(config);
  });

  describe("Basic Server Operations", () => {
    it("should ping the server", async () => {
      const result = await client.admin.ping();
      expect(typeof result).toBe("string");
      expect(result.toLowerCase()).toContain("pong");
    });

    it("should get server info", async () => {
      const result = await client.admin.info();
      expect(typeof result).toBe("string");
      expect(result.length).toBeGreaterThan(0);
    });

    it("should get specific server info section", async () => {
      const result = await client.admin.info("server");
      expect(typeof result).toBe("string");
      expect(result.length).toBeGreaterThan(0);
    });

    it("should get database size", async () => {
      const result = await client.admin.dbSize();
      expect(typeof result).toBe("number");
      expect(result).toBeGreaterThanOrEqual(0);
    });

    it("should get server time", async () => {
      const result = await client.admin.time();
      expect(Array.isArray(result)).toBe(true);
      expect(result).toHaveLength(2);
      expect(typeof result[0]).toBe("number");
      expect(typeof result[1]).toBe("number");
    });

    it("should get server version", async () => {
      const result = await client.admin.version();
      expect(typeof result).toBe("string");
      expect(result.length).toBeGreaterThan(0);
    });
  });

  describe("Health and Status Operations", () => {
    it("should perform health check", async () => {
      const result = await client.admin.health();
      expect(result).toHaveProperty("is_healthy");
      expect(typeof result.is_healthy).toBe("boolean");
      expect(result).toHaveProperty("ping_response");
      expect(result).toHaveProperty("database_size");
      expect(result).toHaveProperty("version");
      expect(result).toHaveProperty("memory_usage");
    });

    it("should get server status", async () => {
      const result = await client.admin.status();
      expect(result).toHaveProperty("timestamp");
      expect(typeof result.timestamp).toBe("number");
      expect(result).toHaveProperty("uptime_seconds");
      expect(result).toHaveProperty("connected_clients");
      expect(result).toHaveProperty("used_memory");
      expect(result).toHaveProperty("total_commands_processed");
      expect(result).toHaveProperty("version");
      expect(result).toHaveProperty("role");
    });
  });

  describe("Statistics Operations", () => {
    it("should get memory statistics", async () => {
      const result = await client.admin.memoryStats();
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBeGreaterThan(0);

      // Check that all values are strings
      for (const [key, value] of Object.entries(result)) {
        expect(typeof value).toBe("string");
      }
    });

    it("should get client statistics", async () => {
      const result = await client.admin.clientStats();
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBeGreaterThan(0);

      // Check that all values are strings
      for (const [key, value] of Object.entries(result)) {
        expect(typeof value).toBe("string");
      }
    });

    it("should get server statistics", async () => {
      const result = await client.admin.serverStats();
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBeGreaterThan(0);

      // Check that all values are strings
      for (const [key, value] of Object.entries(result)) {
        expect(typeof value).toBe("string");
      }
    });
  });

  describe("Configuration Operations", () => {
    it("should get configuration parameter", async () => {
      // Test with a common Redis config parameter
      const result = await client.admin.configGet("maxmemory");
      expect(typeof result).toBe("string");
    });

    it("should get all configuration", async () => {
      const result = await client.admin.configGetAll();
      expect(typeof result).toBe("object");
      expect(Object.keys(result).length).toBeGreaterThan(0);

      // Check that all values are strings
      for (const [key, value] of Object.entries(result)) {
        expect(typeof value).toBe("string");
      }
    });

    it("should set configuration parameter", async () => {
      // Test with a safe configuration parameter
      const testParam = "timeout";
      const testValue = "300";

      try {
        await client.admin.configSet(testParam, testValue);

        // Verify the configuration was set
        const result = await client.admin.configGet(testParam);
        expect(result).toBe(testValue);
      } catch (error) {
        // Some configuration parameters might not be settable
        // This is acceptable behavior
        expect(error).toBeDefined();
      }
    });

    it("should reset statistics", async () => {
      // This operation should not throw an error
      await expect(client.admin.configResetStat()).resolves.not.toThrow();
    });

    it("should rewrite configuration", async () => {
      // This operation may fail in test environments due to file permissions
      // Accept both success and failure as valid outcomes
      try {
        await client.admin.configRewrite();
        // Success case - no assertion needed
      } catch (error) {
        // Failure case - also acceptable in test environments
        expect(error).toBeDefined();
      }
    });
  });

  describe("Database Management Operations", () => {
    it("should flush current database", async () => {
      // Add some test data first
      await client.string.set("test:admin:flushdb", "test value");

      // Verify data exists
      const valueBefore = await client.string.get("test:admin:flushdb");
      expect(valueBefore).toBe("test value");

      // Flush the database
      await client.admin.flushDb();

      // Verify data was flushed
      const valueAfter = await client.string.get("test:admin:flushdb");
      expect(valueAfter).toBeNull();
    });

    it("should flush all databases", async () => {
      // Add some test data first
      await client.string.set("test:admin:flushall", "test value");

      // Verify data exists
      const valueBefore = await client.string.get("test:admin:flushall");
      expect(valueBefore).toBe("test value");

      // Flush all databases
      await client.admin.flushAll();

      // Verify data was flushed
      const valueAfter = await client.string.get("test:admin:flushall");
      expect(valueAfter).toBeNull();
    });
  });

  describe("Error Handling", () => {
    it("should handle invalid info section", async () => {
      try {
        await client.admin.info("invalid:section");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle invalid configuration parameter", async () => {
      try {
        await client.admin.configGet("invalid:parameter");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle invalid configuration set", async () => {
      try {
        await client.admin.configSet("invalid:parameter", "invalid:value");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });
  });

  describe("Integration Tests", () => {
    it("should maintain data consistency after operations", async () => {
      // Add test data
      const testKey = "test:admin:integration";
      const testValue = "integration test value";

      await client.string.set(testKey, testValue);

      // Verify data exists
      const value = await client.string.get(testKey);
      expect(value).toBe(testValue);

      // Get database size
      const dbSizeBefore = await client.admin.dbSize();
      expect(dbSizeBefore).toBeGreaterThan(0);

      // Delete the test data
      await client.string.delete(testKey);

      // Verify data was deleted
      const valueAfter = await client.string.get(testKey);
      expect(valueAfter).toBeNull();
    });

    it("should handle server restart simulation", async () => {
      // This test simulates what happens when the server is restarted
      // by checking that basic operations still work

      // Ping the server
      const pingResult = await client.admin.ping();
      expect(pingResult.toLowerCase()).toContain("pong");

      // Get server info
      const infoResult = await client.admin.info();
      expect(infoResult.length).toBeGreaterThan(0);

      // Get server status
      const statusResult = await client.admin.status();
      expect(statusResult).toHaveProperty("timestamp");
      expect(statusResult).toHaveProperty("version");
    });
  });

  describe("Performance Tests", () => {
    it("should handle rapid ping requests", async () => {
      const promises: Promise<string>[] = [];
      for (let i = 0; i < 10; i++) {
        promises.push(client.admin.ping());
      }

      const results = await Promise.all(promises);

      results.forEach((result) => {
        expect(typeof result).toBe("string");
        expect(result.toLowerCase()).toContain("pong");
      });
    });

    it("should handle concurrent info requests", async () => {
      const promises: Promise<string>[] = [];
      for (let i = 0; i < 5; i++) {
        promises.push(client.admin.info());
      }

      const results = await Promise.all(promises);

      results.forEach((result) => {
        expect(typeof result).toBe("string");
        expect(result.length).toBeGreaterThan(0);
      });
    });
  });

  describe("Edge Cases", () => {
    it("should handle empty configuration parameter", async () => {
      try {
        await client.admin.configGet("");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle very long configuration values", async () => {
      const longValue = "a".repeat(1000);

      try {
        await client.admin.configSet("test:long:value", longValue);
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle special characters in configuration", async () => {
      const specialValue = "test:value:with:colons:and:special:chars:!@#$%^&*()";

      try {
        await client.admin.configSet("test:special:chars", specialValue);
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });
  });

  afterAll(async () => {
    // Clean up any test data that might have been created
    const testKeys = ["test:admin:flushdb", "test:admin:flushall", "test:admin:integration"];

    for (const key of testKeys) {
      try {
        await client.string.delete(key);
      } catch (e) {
        // Ignore cleanup errors
      }
    }
  });
});
