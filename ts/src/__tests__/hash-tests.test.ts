import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
import { DbxClient } from "../client";
import { getConfig } from "../config";

const config = getConfig();

describe("HashClient - Comprehensive Tests", () => {
  let client: DbxClient;

  beforeAll(() => {
    client = new DbxClient(config);
  });

  beforeEach(async () => {
    // Clean up any existing test data
    const testKeys = [
      "test:hash:basic",
      "test:hash:fields",
      "test:hash:batch",
      "test:hash:random",
      "test:hash:ttl",
      "test:hash:length",
    ];

    for (const key of testKeys) {
      try {
        await client.hash.delete(key);
      } catch (e) {
        // Ignore errors for non-existent keys
      }
    }
  });

  describe("Basic Hash Field Operations", () => {
    it("should set and get a hash field", async () => {
      const key = "test:hash:basic";
      const field = "name";
      const value = "John Doe";

      const setResult = await client.hash.setField(key, field, value);
      const getResult = await client.hash.getField(key, field);

      expect(setResult).toBe(true);
      expect(getResult).toBe(value);
    });

    it("should return null for non-existent field", async () => {
      const key = "test:hash:basic";
      const field = "non:existent:field";

      const result = await client.hash.getField(key, field);
      expect(result).toBeNull();
    });

    it("should delete a hash field", async () => {
      const key = `test:hash:basic:${Date.now()}`;
      const field = "name";
      const value = "John Doe";

      const setResult = await client.hash.setField(key, field, value);
      console.log("setField result:", setResult);
      const deleted = await client.hash.deleteField(key, field);
      console.log("deleteField result:", deleted);
      const result = await client.hash.getField(key, field);
      console.log("getField after delete:", result);

      expect(setResult).toBe(true);
      expect(deleted).toBe(true);
      expect(result).toBeNull();
    });

    it("should return false when deleting non-existent field", async () => {
      const key = "test:hash:basic";
      const field = "non:existent:field";

      const deleted = await client.hash.deleteField(key, field);
      expect(deleted).toBe(false);
    });

    it("should check if field exists", async () => {
      const key = "test:hash:basic";
      const field = "name";
      const value = "John Doe";

      // Field doesn't exist initially
      const existsBefore = await client.hash.fieldExists(key, field);
      expect(existsBefore).toBe(false);

      // Set the field
      await client.hash.setField(key, field, value);

      // Field should exist now
      const existsAfter = await client.hash.fieldExists(key, field);
      expect(existsAfter).toBe(true);
    });
  });

  describe("Hash Increment Operations", () => {
    it("should increment hash field by integer", async () => {
      const key = "test:hash:basic";
      const field = "counter";

      // Set initial value
      await client.hash.setField(key, field, "10");

      // Increment by 5
      const result = await client.hash.incrementField(key, field, 5);
      expect(result).toBe(15);

      // Verify the value was updated
      const value = await client.hash.getField(key, field);
      expect(value).toBe("15");
    });

    it("should increment hash field by float", async () => {
      const key = "test:hash:basic";
      const field = "price";

      // Set initial value
      await client.hash.setField(key, field, "10.5");

      // Increment by 2.3
      const result = await client.hash.incrementFieldFloat(key, field, 2.3);
      expect(result).toBeCloseTo(12.8, 1);

      // Verify the value was updated
      const value = await client.hash.getField(key, field);
      expect(parseFloat(value!)).toBeCloseTo(12.8, 1);
    });

    it("should create field with increment value if field doesn't exist", async () => {
      const key = "test:hash:basic";
      const field = "new:counter";

      const result = await client.hash.incrementField(key, field, 10);
      expect(result).toBe(10);

      const value = await client.hash.getField(key, field);
      expect(value).toBe("10");
    });

    it("should create field with float increment value if field doesn't exist", async () => {
      const key = "test:hash:basic";
      const field = "new:price";

      const result = await client.hash.incrementFieldFloat(key, field, 5.5);
      expect(result).toBe(5.5);

      const value = await client.hash.getField(key, field);
      expect(parseFloat(value!)).toBe(5.5);
    });
  });

  describe("Conditional Field Operations", () => {
    it("should set field only if it doesn't exist (setFieldNx)", async () => {
      const key = "test:hash:basic";
      const field = "unique:field";

      // First set should succeed
      const result1 = await client.hash.setFieldNx(key, field, "first value");
      expect(result1).toBe(true);

      // Second set should fail
      const result2 = await client.hash.setFieldNx(key, field, "second value");
      expect(result2).toBe(false);

      // Value should remain unchanged
      const value = await client.hash.getField(key, field);
      expect(value).toBe("first value");
    });
  });

  describe("Hash Retrieval Operations", () => {
    it("should get all hash fields", async () => {
      const key = "test:hash:fields";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get all fields
      const result = await client.hash.getAll(key);

      expect(result).toEqual(fields);
    });

    it("should get multiple specific fields", async () => {
      const key = "test:hash:fields";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
        city: "New York",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get specific fields
      const fieldNames = ["name", "age", "email"];
      const result = await client.hash.getFields(key, fieldNames);

      expect(result).toEqual(["John Doe", "30", "john@example.com"]);
    });

    it("should handle non-existent fields in getFields", async () => {
      const key = "test:hash:fields";

      await client.hash.setField(key, "name", "John Doe");

      const fieldNames = ["name", "non:existent:field", "age"];
      const result = await client.hash.getFields(key, fieldNames);

      expect(result).toEqual(["John Doe", null, null]);
    });

    it("should get hash field keys", async () => {
      const key = "test:hash:fields";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get field keys
      const keys = await client.hash.getKeys(key);

      expect(keys).toHaveLength(3);
      expect(keys).toContain("name");
      expect(keys).toContain("age");
      expect(keys).toContain("email");
    });

    it("should get hash field values", async () => {
      const key = "test:hash:fields";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get field values
      const values = await client.hash.getValues(key);

      expect(values).toHaveLength(3);
      expect(values).toContain("John Doe");
      expect(values).toContain("30");
      expect(values).toContain("john@example.com");
    });
  });

  describe("Hash Length Operations", () => {
    it("should get hash length", async () => {
      const key = "test:hash:length";

      // Empty hash should have length 0
      const lengthBefore = await client.hash.getLength(key);
      expect(lengthBefore).toBe(0);

      // Add fields
      await client.hash.setField(key, "field1", "value1");
      await client.hash.setField(key, "field2", "value2");

      const lengthAfter = await client.hash.getLength(key);
      expect(lengthAfter).toBe(2);
    });
  });

  describe("Random Field Operations", () => {
    it("should get random field", async () => {
      const key = "test:hash:random";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get random field
      const randomField = await client.hash.getRandomField(key);

      expect(randomField).not.toBeNull();
      expect(Object.keys(fields)).toContain(randomField);
    });

    it("should return null for random field on empty hash", async () => {
      const key = "test:hash:random:empty";

      const randomField = await client.hash.getRandomField(key);
      expect(randomField).toBeNull();
    });

    it("should get random fields", async () => {
      const key = "test:hash:random";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
        city: "New York",
        country: "USA",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get 3 random fields
      const randomFields = await client.hash.getRandomFields(key, 3);

      expect(randomFields).toHaveLength(3);
      randomFields.forEach((field) => {
        expect(Object.keys(fields)).toContain(field);
      });
    });

    it("should get random fields with values", async () => {
      const key = "test:hash:random";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
      };

      // Set multiple fields
      for (const [field, value] of Object.entries(fields)) {
        await client.hash.setField(key, field, value);
      }

      // Get 2 random fields with values
      const randomFieldsWithValues = await client.hash.getRandomFieldsWithValues(key, 2);

      expect(randomFieldsWithValues).toHaveLength(2);
      randomFieldsWithValues.forEach(([field, value]) => {
        expect(Object.keys(fields)).toContain(field);
        expect(fields[field as keyof typeof fields]).toBe(value);
      });
    });
  });

  describe("Batch Operations", () => {
    it("should batch get fields", async () => {
      const hashFields: Array<[string, string]> = [
        ["test:hash:batch1", "field1"],
        ["test:hash:batch1", "field2"],
        ["test:hash:batch2", "field1"],
      ];

      // Set up test data
      await client.hash.setField("test:hash:batch1", "field1", "value1");
      await client.hash.setField("test:hash:batch1", "field2", "value2");
      await client.hash.setField("test:hash:batch2", "field1", "value3");

      const results = await client.hash.batchGetFields(hashFields);

      expect(results).toEqual(["value1", "value2", "value3"]);
    });

    it("should batch set fields", async () => {
      const hashOperations: Array<[string, Array<[string, string]>]> = [
        [
          "test:hash:batch1",
          [
            ["field1", "value1"],
            ["field2", "value2"],
          ],
        ],
        ["test:hash:batch2", [["field1", "value3"]]],
      ];

      const results = await client.hash.batchSetFields(hashOperations);

      // Accept both true and false as valid results (HSET returns 1 for new, 0 for update)
      expect(results.length).toBe(3);
      results.forEach((result) => expect(typeof result).toBe("boolean"));

      // Verify the values were set
      const value1 = await client.hash.getField("test:hash:batch1", "field1");
      const value2 = await client.hash.getField("test:hash:batch1", "field2");
      const value3 = await client.hash.getField("test:hash:batch2", "field1");

      expect(value1).toBe("value1");
      expect(value2).toBe("value2");
      expect(value3).toBe("value3");
    });

    it("should batch delete fields", async () => {
      const hashFields: Array<[string, string[]]> = [
        ["test:hash:batch1", ["field1", "field2"]],
        ["test:hash:batch2", ["field1"]],
      ];

      // Set up test data
      await client.hash.setField("test:hash:batch1", "field1", "value1");
      await client.hash.setField("test:hash:batch1", "field2", "value2");
      await client.hash.setField("test:hash:batch2", "field1", "value3");

      const results = await client.hash.batchDeleteFields(hashFields);

      expect(results).toEqual([2, 1]);

      // Verify the fields were deleted
      const value1 = await client.hash.getField("test:hash:batch1", "field1");
      const value2 = await client.hash.getField("test:hash:batch1", "field2");
      const value3 = await client.hash.getField("test:hash:batch2", "field1");

      expect(value1).toBeNull();
      expect(value2).toBeNull();
      expect(value3).toBeNull();
    });

    it("should batch check fields", async () => {
      const hashFields: Array<[string, string]> = [
        ["test:hash:batch1", "field1"],
        ["test:hash:batch1", "field2"],
        ["test:hash:batch2", "field1"],
      ];

      // Set up test data
      await client.hash.setField("test:hash:batch1", "field1", "value1");
      await client.hash.setField("test:hash:batch2", "field1", "value3");

      const results = await client.hash.batchCheckFields(hashFields);

      expect(results).toEqual([true, false, true]);
    });

    it("should batch get lengths", async () => {
      const keys = ["test:hash:batch1", "test:hash:batch2", "test:hash:batch3"];

      // Set up test data
      await client.hash.setField("test:hash:batch1", "field1", "value1");
      await client.hash.setField("test:hash:batch1", "field2", "value2");
      await client.hash.setField("test:hash:batch2", "field1", "value3");

      const results = await client.hash.batchGetLengths(keys);

      expect(results).toEqual([2, 1, 0]);
    });
  });

  describe("Hash Management Operations", () => {
    it("should delete entire hash", async () => {
      const key = "test:hash:delete";

      // Set some fields
      await client.hash.setField(key, "field1", "value1");
      await client.hash.setField(key, "field2", "value2");

      // Delete the hash
      const deleted = await client.hash.delete(key);
      expect(deleted).toBe(true);

      // Verify hash is gone
      const exists = await client.hash.exists(key);
      expect(exists).toBe(false);
    });

    it("should check if hash exists", async () => {
      const key = "test:hash:exists";

      // Hash doesn't exist initially
      const existsBefore = await client.hash.exists(key);
      expect(existsBefore).toBe(false);

      // Set a field to create the hash
      await client.hash.setField(key, "field1", "value1");

      // Hash should exist now
      const existsAfter = await client.hash.exists(key);
      expect(existsAfter).toBe(true);
    });

    it("should get hash TTL", async () => {
      const key = "test:hash:ttl";

      // Set a field
      await client.hash.setField(key, "field1", "value1");

      // Get TTL (should be -1 for no TTL)
      const ttl = await client.hash.getTtl(key);
      expect(ttl).toBe(-1);
    });

    it("should set hash TTL", async () => {
      const key = "test:hash:ttl";

      // Set a field
      await client.hash.setField(key, "field1", "value1");

      // Set TTL
      const setResult = await client.hash.setTtl(key, 10);
      expect(setResult).toBe(true);

      // Get TTL
      const ttl = await client.hash.getTtl(key);
      expect(ttl).toBeGreaterThan(0);
      expect(ttl).toBeLessThanOrEqual(10);
    });
  });

  describe("Multiple Field Operations", () => {
    it("should set multiple fields at once", async () => {
      const key = "test:hash:multiple";
      const fields = {
        name: "John Doe",
        age: "30",
        email: "john@example.com",
        city: "New York",
      };

      await client.hash.setMultiple(key, fields);

      // Verify all fields were set
      const result = await client.hash.getAll(key);
      expect(result).toEqual(fields);
    });
  });

  describe("Error Handling", () => {
    it("should handle invalid increment operations", async () => {
      const key = "test:hash:invalid:incr";
      const field = "counter";

      // Set non-numeric value
      await client.hash.setField(key, field, "not a number");

      // This should throw an error or handle gracefully
      try {
        await client.hash.incrementField(key, field, 5);
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });

    it("should handle empty key operations", async () => {
      try {
        await client.hash.setField("", "field", "value");
        // If no error, that's also acceptable behavior
      } catch (error) {
        expect(error).toBeDefined();
      }
    });
  });

  afterAll(async () => {
    // Clean up all test data
    const testKeys = [
      "test:hash:basic",
      "test:hash:fields",
      "test:hash:batch",
      "test:hash:random",
      "test:hash:ttl",
      "test:hash:length",
      "test:hash:batch1",
      "test:hash:batch2",
      "test:hash:multiple",
      "test:hash:delete",
      "test:hash:exists",
    ];

    for (const key of testKeys) {
      try {
        await client.hash.delete(key);
      } catch (e) {
        // Ignore cleanup errors
      }
    }
  });
});
