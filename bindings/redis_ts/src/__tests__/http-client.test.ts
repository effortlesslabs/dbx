import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { TEST_CONFIG, generateTestKey, generateTestValue } from "./setup";

// Import the bindings
let createClient: any;
let createClientWithTimeout: any;
let DbxRedisClient: any;

try {
  const bindings = require("../../index.js");
  createClient = bindings.createClient;
  createClientWithTimeout = bindings.createClientWithTimeout;
  DbxRedisClient = bindings.DbxRedisClient;
} catch (error) {
  console.error("Failed to load bindings:", error);
  throw error;
}

describe("HTTP Redis Client", () => {
  let client: any;

  beforeAll(() => {
    client = createClient(TEST_CONFIG.HTTP_URL);
    expect(client).toBeDefined();
  });

  afterAll(() => {
    // Cleanup if needed
  });

  describe("Client Creation", () => {
    it("should create a client with default constructor", () => {
      const newClient = new DbxRedisClient(TEST_CONFIG.HTTP_URL);
      expect(newClient).toBeDefined();
      expect(newClient.get_base_url()).toBe(TEST_CONFIG.HTTP_URL);
    });

    it("should create a client with timeout", () => {
      const newClient = createClientWithTimeout(TEST_CONFIG.HTTP_URL, 5000);
      expect(newClient).toBeDefined();
      expect(newClient.get_base_url()).toBe(TEST_CONFIG.HTTP_URL);
    });

    it("should get base URL", () => {
      expect(client.get_base_url()).toBe(TEST_CONFIG.HTTP_URL);
    });
  });

  describe("String Operations", () => {
    const stringClient = client.string();

    it("should set and get a string value", async () => {
      const key = generateTestKey("string");
      const value = generateTestValue();

      // Set value
      const setResult = await stringClient.set(key, value);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should set and get a string value with TTL", async () => {
      const key = generateTestKey("string_ttl");
      const value = generateTestValue();

      // Set value with TTL
      const setResult = await stringClient.set_with_ttl(key, value, 60);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should set and get a string value without TTL", async () => {
      const key = generateTestKey("string_simple");
      const value = generateTestValue();

      // Set value without TTL
      const setResult = await stringClient.set_simple(key, value);
      expect(setResult).toBe(true);

      // Get value
      const getResult = await stringClient.get(key);
      expect(getResult).toBe(value);
    });

    it("should delete a string value", async () => {
      const key = generateTestKey("string_delete");
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

    it("should get string information", async () => {
      const key = generateTestKey("string_info");
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

    it("should batch get multiple strings", async () => {
      const keys = [
        generateTestKey("batch1"),
        generateTestKey("batch2"),
        generateTestKey("batch3"),
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

    it("should batch set multiple strings", async () => {
      const operations = [
        { key: generateTestKey("batch_set1"), value: generateTestValue(), ttl: null },
        { key: generateTestKey("batch_set2"), value: generateTestValue(), ttl: 60 },
        { key: generateTestKey("batch_set3"), value: generateTestValue(), ttl: null },
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

    it("should return null for non-existent key", async () => {
      const key = generateTestKey("non_existent");
      const result = await stringClient.get(key);
      expect(result).toBeNull();
    });
  });

  describe("Set Operations", () => {
    const setClient = client.set();

    it("should add one member to a set", async () => {
      const key = generateTestKey("set");
      const member = generateTestValue();

      const result = await setClient.add_one(key, member);
      expect(result).toBe(true);

      // Verify member exists
      const exists = await setClient.exists(key, member);
      expect(exists).toBe(true);
    });

    it("should add multiple members to a set", async () => {
      const key = generateTestKey("set_many");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      const result = await setClient.add_many(key, members);
      expect(result).toBe(true);

      // Verify all members exist
      for (const member of members) {
        const exists = await setClient.exists(key, member);
        expect(exists).toBe(true);
      }
    });

    it("should remove a member from a set", async () => {
      const key = generateTestKey("set_remove");
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

    it("should get all members of a set", async () => {
      const key = generateTestKey("set_members");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get members
      const result = await setClient.members(key);
      expect(result).toHaveLength(3);
      expect(result).toEqual(expect.arrayContaining(members));
    });

    it("should get set cardinality", async () => {
      const key = generateTestKey("set_cardinality");
      const members = [generateTestValue(), generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get cardinality
      const result = await setClient.cardinality(key);
      expect(result).toBe(3);
    });

    it("should check if member exists", async () => {
      const key = generateTestKey("set_exists");
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

    it("should check if member contains (alias for exists)", async () => {
      const key = generateTestKey("set_contains");
      const member = generateTestValue();

      // Add member
      await setClient.add_one(key, member);

      // Check contains
      const contains = await setClient.contains(key, member);
      expect(contains).toBe(true);
    });

    it("should get set size", async () => {
      const key = generateTestKey("set_size");
      const members = [generateTestValue(), generateTestValue()];

      // Add members
      await setClient.add_many(key, members);

      // Get size
      const result = await setClient.size(key);
      expect(result).toBe(2);
    });

    it("should get intersection of sets", async () => {
      const key1 = generateTestKey("set_intersect1");
      const key2 = generateTestKey("set_intersect2");
      const commonMember = generateTestValue();

      // Add members to both sets
      await setClient.add_many(key1, [commonMember, generateTestValue()]);
      await setClient.add_many(key2, [commonMember, generateTestValue()]);

      // Get intersection
      const result = await setClient.intersect([key1, key2]);
      expect(result).toContain(commonMember);
    });

    it("should get union of sets", async () => {
      const key1 = generateTestKey("set_union1");
      const key2 = generateTestKey("set_union2");
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

    it("should get difference of sets", async () => {
      const key1 = generateTestKey("set_diff1");
      const key2 = generateTestKey("set_diff2");
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
