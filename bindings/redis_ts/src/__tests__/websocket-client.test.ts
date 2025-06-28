import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { TEST_CONFIG, generateTestKey, generateTestValue } from "./setup";

// Import the bindings
let createWsClient: any;
let DbxWsClient: any;

try {
  const bindings = require("../../index.js");
  createWsClient = bindings.createWsClient;
  DbxWsClient = bindings.DbxWsClient;
} catch (error) {
  console.error("Failed to load WebSocket bindings:", error);
  throw error;
}

describe("WebSocket Redis Client", () => {
  let client: any;

  beforeAll(() => {
    client = createWsClient(TEST_CONFIG.WS_URL);
    expect(client).toBeDefined();
  });

  afterAll(() => {
    // Cleanup if needed
  });

  describe("Client Creation", () => {
    it("should create a WebSocket client with default constructor", () => {
      const newClient = new DbxWsClient(TEST_CONFIG.WS_URL);
      expect(newClient).toBeDefined();
      expect(newClient.get_base_url()).toBe(TEST_CONFIG.WS_URL);
    });

    it("should get base URL", () => {
      expect(client.get_base_url()).toBe(TEST_CONFIG.WS_URL);
    });
  });

  describe("WebSocket String Operations", () => {
    const stringClient = client.string();

    it("should set and get a string value via WebSocket", async () => {
      const key = generateTestKey("ws_string");
      const value = generateTestValue();

      // Set value
      const setResult = await stringClient.set(key, value);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should set and get a string value with TTL via WebSocket", async () => {
      const key = generateTestKey("ws_string_ttl");
      const value = generateTestValue();

      // Set value with TTL
      const setResult = await stringClient.set_with_ttl(key, value, 60);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should set and get a string value without TTL via WebSocket", async () => {
      const key = generateTestKey("ws_string_simple");
      const value = generateTestValue();

      // Set value without TTL
      const setResult = await stringClient.set_simple(key, value);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should delete a string value via WebSocket", async () => {
      const key = generateTestKey("ws_string_delete");
      const value = generateTestValue();

      // Set value
      await stringClient.set(key, value);

      // Delete value
      const deleteResult = await stringClient.delete(key);
      expect(deleteResult).toBe(true);

      // Verify deletion
      const getResult = await stringClient.get(key);
      expect(getResult).toBeNull();
    });

    it("should get string information via WebSocket", async () => {
      const key = generateTestKey("ws_string_info");
      const value = generateTestValue();

      // Set value
      await stringClient.set(key, value);

      // Get info
      const info = await stringClient.info(key);
      expect(info).toBeDefined();
      expect(info.key).toBe(key);
      expect(info.value).toBe(value);
      expect(info.type).toBe("string");
    });

    it("should batch get multiple strings via WebSocket", async () => {
      const keys = [
        generateTestKey("ws_batch1"),
        generateTestKey("ws_batch2"),
        generateTestKey("ws_batch3"),
      ];
      const values = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Set values
      for (let i = 0; i < keys.length; i++) {
        await stringClient.set(keys[i], values[i]);
      }

      // Batch get
      const results = await stringClient.batch_get(keys);
      expect(results).toHaveLength(3);
      expect(results[0]).toBe(values[0]);
      expect(results[1]).toBe(values[1]);
      expect(results[2]).toBe(values[2]);
    });

    it("should batch set multiple strings via WebSocket", async () => {
      const operations = [
        { key: generateTestKey("ws_batch_set1"), value: generateTestValue(), ttl: null },
        { key: generateTestKey("ws_batch_set2"), value: generateTestValue(), ttl: 60 },
        { key: generateTestKey("ws_batch_set3"), value: generateTestValue(), ttl: null },
      ];

      // Batch set
      const setResult = await stringClient.batch_set(operations);
      expect(setResult).toBe(true);

      // Verify all values were set
      for (const op of operations) {
        const value = await stringClient.get(op.key);
        expect(value).toBe(op.value);
      }
    });

    it("should return null for non-existent key via WebSocket", async () => {
      const key = generateTestKey("ws_non_existent");
      const result = await stringClient.get(key);
      expect(result).toBeNull();
    });
  });

  describe("WebSocket Set Operations", () => {
    const setClient = client.set();

    it("should add one member to a set via WebSocket", async () => {
      const key = generateTestKey("ws_set");
      const member = generateTestValue();

      const result = await setClient.add_one(key, member);
      expect(result).toBe(true);

      // Verify member exists
      const exists = await setClient.exists(key, member);
      expect(exists).toBe(true);
    });

    it("should add multiple members to a set via WebSocket", async () => {
      const key = generateTestKey("ws_set_many");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      const result = await setClient.add_many(key, members);
      expect(result).toBe(true);

      // Verify all members exist
      for (const member of members) {
        const exists = await setClient.exists(key, member);
        expect(exists).toBe(true);
      }
    });

    it("should remove a member from a set via WebSocket", async () => {
      const key = generateTestKey("ws_set_remove");
      const member = generateTestValue();

      // Add member
      await setClient.add_one(key, member);

      // Remove member
      const result = await setClient.remove(key, member);
      expect(result).toBe(true);

      // Verify member was removed
      const exists = await setClient.exists(key, member);
      expect(exists).toBe(false);
    });

    it("should get all members of a set via WebSocket", async () => {
      const key = generateTestKey("ws_set_members");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get members
      const result = await setClient.members(key);
      expect(result).toHaveLength(3);
      expect(result).toEqual(expect.arrayContaining(members));
    });

    it("should get set cardinality via WebSocket", async () => {
      const key = generateTestKey("ws_set_cardinality");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get cardinality
      const result = await setClient.cardinality(key);
      expect(result).toBe(3);
    });

    it("should check if member exists via WebSocket", async () => {
      const key = generateTestKey("ws_set_exists");
      const member = generateTestValue();

      // Member doesn't exist initially
      let exists = await setClient.exists(key, member);
      expect(exists).toBe(false);

      // Add member
      await setClient.add_one(key, member);

      // Member exists now
      exists = await setClient.exists(key, member);
      expect(exists).toBe(true);
    });

    it("should check if member contains via WebSocket (alias for exists)", async () => {
      const key = generateTestKey("ws_set_contains");
      const member = generateTestValue();

      // Add member
      await setClient.add_one(key, member);

      // Check contains
      const contains = await setClient.contains(key, member);
      expect(contains).toBe(true);
    });

    it("should get set size via WebSocket", async () => {
      const key = generateTestKey("ws_set_size");
      const members = [generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get size
      const result = await setClient.size(key);
      expect(result).toBe(2);
    });

    it("should get intersection of sets via WebSocket", async () => {
      const key1 = generateTestKey("ws_set_intersect1");
      const key2 = generateTestKey("ws_set_intersect2");
      const commonMember = generateTestValue();

      // Add members to both sets
      await setClient.add_many(key1, [commonMember, generateTestValue()]);
      await setClient.add_many(key2, [commonMember, generateTestValue()]);

      // Get intersection
      const result = await setClient.intersect([key1, key2]);
      expect(result).toContain(commonMember);
    });

    it("should get union of sets via WebSocket", async () => {
      const key1 = generateTestKey("ws_set_union1");
      const key2 = generateTestKey("ws_set_union2");
      const member1 = generateTestValue();
      const member2 = generateTestValue();

      // Add members to sets
      await setClient.add_one(key1, member1);
      await setClient.add_one(key2, member2);

      // Get union
      const result = await setClient.union([key1, key2]);
      expect(result).toContain(member1);
      expect(result).toContain(member2);
    });

    it("should get difference of sets via WebSocket", async () => {
      const key1 = generateTestKey("ws_set_diff1");
      const key2 = generateTestKey("ws_set_diff2");
      const member1 = generateTestValue();
      const member2 = generateTestValue();

      // Add members to sets
      await setClient.add_many(key1, [member1, member2]);
      await setClient.add_one(key2, member2);

      // Get difference
      const result = await setClient.difference([key1, key2]);
      expect(result).toContain(member1);
      expect(result).not.toContain(member2);
    });
  });
});
