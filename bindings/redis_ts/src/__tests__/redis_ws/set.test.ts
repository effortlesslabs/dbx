import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
// Note: This import will work after the NAPI module is built
// For now, we'll use a placeholder import that will be resolved at runtime
const nativeBinding = require("../../../index.js");
const { DbxWsClient } = nativeBinding;

describe("Redis WebSocket Set Operations", () => {
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
      const setClient = client.set();
      console.log("setClient created:", setClient);
      expect(setClient).toBeDefined();

      // Test that the set client has the expected methods
      expect(typeof setClient.addOne).toBe("function");
      expect(typeof setClient.addMany).toBe("function");
      expect(typeof setClient.remove).toBe("function");
      expect(typeof setClient.members).toBe("function");
      expect(typeof setClient.cardinality).toBe("function");
      expect(typeof setClient.exists).toBe("function");
      expect(typeof setClient.contains).toBe("function");
      expect(typeof setClient.size).toBe("function");
      expect(typeof setClient.intersect).toBe("function");
      expect(typeof setClient.union).toBe("function");
      expect(typeof setClient.difference).toBe("function");

      console.log("WebSocket client methods are available");
    });
  });

  describe("Set Operations", () => {
    let setClient: any;
    const testKey = "test:set:ws";
    const testMember = "test:member:1";

    beforeEach(async () => {
      setClient = client.set();
      // Clean up before each test - remove all test members
      await setClient.remove(testKey, testMember);
      await setClient.remove(testKey, "member1");
      await setClient.remove(testKey, "member2");
      await setClient.remove(testKey, "member3");
      await setClient.remove(testKey, "member4");
    });

    afterAll(async () => {
      // Clean up after all tests
      const cleanupClient = client.set();
      await cleanupClient.remove(testKey, testMember);
      await cleanupClient.remove("test:set:ws:many", "member1");
      await cleanupClient.remove("test:set:ws:many", "member2");
      await cleanupClient.remove("test:set:ws:many", "member3");
      await cleanupClient.remove("test:set:ws:intersect1", "common");
      await cleanupClient.remove("test:set:ws:intersect1", "unique1");
      await cleanupClient.remove("test:set:ws:intersect2", "common");
      await cleanupClient.remove("test:set:ws:intersect2", "unique2");
      await cleanupClient.remove("test:set:ws:union1", "union1_only");
      await cleanupClient.remove("test:set:ws:union1", "common");
      await cleanupClient.remove("test:set:ws:union2", "union2_only");
      await cleanupClient.remove("test:set:ws:union2", "common");
      await cleanupClient.remove("test:set:ws:diff1", "diff1_only");
      await cleanupClient.remove("test:set:ws:diff1", "common");
      await cleanupClient.remove("test:set:ws:diff2", "diff2_only");
      await cleanupClient.remove("test:set:ws:diff2", "common");
    });

    it("should add one member to a set via WebSocket", async () => {
      console.log(`Testing addOne operation: key=${testKey}, member=${testMember}`);

      try {
        const result = await setClient.addOne(testKey, testMember);
        console.log("addOne result:", result);
        expect(result).toBe(true);

        // Verify the member was added
        const members = await setClient.members(testKey);
        console.log("members after addOne:", members);
        expect(members).toContain(testMember);

        console.log("✅ addOne operation successful!");
      } catch (error) {
        console.log("❌ addOne error:", error);
        throw error;
      }
    });

    it("should add multiple members to a set via WebSocket", async () => {
      const manyKey = "test:set:ws:many";
      const members = ["member1", "member2", "member3"];

      console.log(`Testing addMany operation: key=${manyKey}, members=${members.join(", ")}`);

      try {
        const result = await setClient.addMany(manyKey, members);
        console.log("addMany result:", result);
        expect(result).toBe(true);

        // Verify all members were added
        const retrievedMembers = await setClient.members(manyKey);
        console.log("members after addMany:", retrievedMembers);
        expect(retrievedMembers).toContain("member1");
        expect(retrievedMembers).toContain("member2");
        expect(retrievedMembers).toContain("member3");

        console.log("✅ addMany operation successful!");
      } catch (error) {
        console.log("❌ addMany error:", error);
        throw error;
      }
    });

    it("should remove a member from a set via WebSocket", async () => {
      console.log(`Testing remove operation: key=${testKey}, member=${testMember}`);

      try {
        // First add a member
        await setClient.addOne(testKey, testMember);

        // Verify it exists
        const membersBefore = await setClient.members(testKey);
        expect(membersBefore).toContain(testMember);

        // Remove the member
        const result = await setClient.remove(testKey, testMember);
        console.log("remove result:", result);
        expect(result).toBe(1); // Should return 1 member removed

        // Verify it's removed
        const membersAfter = await setClient.members(testKey);
        expect(membersAfter).not.toContain(testMember);

        console.log("✅ remove operation successful!");
      } catch (error) {
        console.log("❌ remove error:", error);
        throw error;
      }
    });

    it("should return 0 when removing non-existent member", async () => {
      const nonExistentMember = "non:existent:member";

      console.log(
        `Testing remove non-existent member: key=${testKey}, member=${nonExistentMember}`
      );

      try {
        const result = await setClient.remove(testKey, nonExistentMember);
        console.log("remove result:", result);
        expect(result).toBe(0); // Should return 0 members removed

        console.log("✅ remove non-existent member operation successful!");
      } catch (error) {
        console.log("❌ remove non-existent member error:", error);
        throw error;
      }
    });

    it("should get all members of a set via WebSocket", async () => {
      console.log(`Testing members operation: key=${testKey}`);

      try {
        // Add multiple members
        const testMembers = ["member1", "member2", "member3"];
        await setClient.addMany(testKey, testMembers);

        // Get all members
        const members = await setClient.members(testKey);
        console.log("members result:", members);

        expect(members).toBeDefined();
        expect(Array.isArray(members)).toBe(true);
        expect(members).toContain("member1");
        expect(members).toContain("member2");
        expect(members).toContain("member3");

        console.log("✅ members operation successful!");
      } catch (error) {
        console.log("❌ members error:", error);
        throw error;
      }
    });

    it("should return empty array for members of non-existent set", async () => {
      const nonExistentKey = "non:existent:set";

      console.log(`Testing members for non-existent set: key=${nonExistentKey}`);

      try {
        const members = await setClient.members(nonExistentKey);
        console.log("members result:", members);
        expect(members).toEqual([]);

        console.log("✅ members for non-existent set operation successful!");
      } catch (error) {
        console.log("❌ members for non-existent set error:", error);
        throw error;
      }
    });

    it("should get the cardinality of a set via WebSocket", async () => {
      console.log(`Testing cardinality operation: key=${testKey}`);

      try {
        // Add multiple members
        const testMembers = ["member1", "member2", "member3", "member4"];
        await setClient.addMany(testKey, testMembers);

        // Get cardinality
        const cardinality = await setClient.cardinality(testKey);
        console.log("cardinality result:", cardinality);

        expect(cardinality).toBe(4);

        console.log("✅ cardinality operation successful!");
      } catch (error) {
        console.log("❌ cardinality error:", error);
        throw error;
      }
    });

    it("should return 0 for cardinality of non-existent set", async () => {
      const nonExistentKey = "non:existent:set";

      console.log(`Testing cardinality for non-existent set: key=${nonExistentKey}`);

      try {
        const cardinality = await setClient.cardinality(nonExistentKey);
        console.log("cardinality result:", cardinality);
        expect(cardinality).toBe(0);

        console.log("✅ cardinality for non-existent set operation successful!");
      } catch (error) {
        console.log("❌ cardinality for non-existent set error:", error);
        throw error;
      }
    });

    it("should check if a member exists in a set via WebSocket", async () => {
      console.log(`Testing exists operation: key=${testKey}, member=${testMember}`);

      try {
        // First add a member
        await setClient.addOne(testKey, testMember);

        // Check if member exists
        const exists = await setClient.exists(testKey, testMember);
        console.log("exists result:", exists);
        expect(exists).toBe(true);

        // Check if non-existent member exists
        const nonExistentExists = await setClient.exists(testKey, "non:existent:member");
        expect(nonExistentExists).toBe(false);

        console.log("✅ exists operation successful!");
      } catch (error) {
        console.log("❌ exists error:", error);
        throw error;
      }
    });

    it("should check if a member contains in a set via WebSocket (alias for exists)", async () => {
      console.log(`Testing contains operation: key=${testKey}, member=${testMember}`);

      try {
        // First add a member
        await setClient.addOne(testKey, testMember);

        // Check if member contains
        const contains = await setClient.contains(testKey, testMember);
        console.log("contains result:", contains);
        expect(contains).toBe(true);

        // Check if non-existent member contains
        const nonExistentContains = await setClient.contains(testKey, "non:existent:member");
        expect(nonExistentContains).toBe(false);

        console.log("✅ contains operation successful!");
      } catch (error) {
        console.log("❌ contains error:", error);
        throw error;
      }
    });

    it("should get the size of a set via WebSocket", async () => {
      console.log(`Testing size operation: key=${testKey}`);

      try {
        // Clean up first to ensure empty set
        await setClient.remove(testKey, "member1");
        await setClient.remove(testKey, "member2");
        await setClient.remove(testKey, "member3");
        await setClient.remove(testKey, "member4");

        // Add exactly 3 members
        const testMembers = ["member1", "member2", "member3"];
        await setClient.addMany(testKey, testMembers);

        // Get size
        const size = await setClient.size(testKey);
        console.log("size result:", size);

        expect(size).toBe(3);

        console.log("✅ size operation successful!");
      } catch (error) {
        console.log("❌ size error:", error);
        throw error;
      }
    });

    it("should return 0 for size of non-existent set", async () => {
      const nonExistentKey = "non:existent:set";

      console.log(`Testing size for non-existent set: key=${nonExistentKey}`);

      try {
        const size = await setClient.size(nonExistentKey);
        console.log("size result:", size);
        expect(size).toBe(0);

        console.log("✅ size for non-existent set operation successful!");
      } catch (error) {
        console.log("❌ size for non-existent set error:", error);
        throw error;
      }
    });

    it("should get the intersection of multiple sets via WebSocket", async () => {
      const set1Key = "test:set:ws:intersect1";
      const set2Key = "test:set:ws:intersect2";

      console.log(`Testing intersect operation: keys=${set1Key}, ${set2Key}`);

      try {
        // Add members to first set
        await setClient.addMany(set1Key, ["common", "unique1"]);

        // Add members to second set
        await setClient.addMany(set2Key, ["common", "unique2"]);

        // Get intersection
        const intersection = await setClient.intersect([set1Key, set2Key]);
        console.log("intersect result:", intersection);

        expect(intersection).toBeDefined();
        expect(Array.isArray(intersection)).toBe(true);
        expect(intersection).toContain("common");
        expect(intersection).not.toContain("unique1");
        expect(intersection).not.toContain("unique2");

        console.log("✅ intersect operation successful!");
      } catch (error) {
        console.log("❌ intersect error:", error);
        throw error;
      }
    });

    it("should get the union of multiple sets via WebSocket", async () => {
      const set1Key = "test:set:ws:union1";
      const set2Key = "test:set:ws:union2";

      console.log(`Testing union operation: keys=${set1Key}, ${set2Key}`);

      try {
        // Add members to first set
        await setClient.addMany(set1Key, ["union1_only", "common"]);

        // Add members to second set
        await setClient.addMany(set2Key, ["union2_only", "common"]);

        // Get union
        const union = await setClient.union([set1Key, set2Key]);
        console.log("union result:", union);

        expect(union).toBeDefined();
        expect(Array.isArray(union)).toBe(true);
        expect(union).toContain("union1_only");
        expect(union).toContain("union2_only");
        expect(union).toContain("common");

        console.log("✅ union operation successful!");
      } catch (error) {
        console.log("❌ union error:", error);
        throw error;
      }
    });

    it("should get the difference of multiple sets via WebSocket", async () => {
      const set1Key = "test:set:ws:diff1";
      const set2Key = "test:set:ws:diff2";

      console.log(`Testing difference operation: keys=${set1Key}, ${set2Key}`);

      try {
        // Add members to first set
        await setClient.addMany(set1Key, ["diff1_only", "common"]);

        // Add members to second set
        await setClient.addMany(set2Key, ["diff2_only", "common"]);

        // Get difference (set1 - set2)
        const difference = await setClient.difference([set1Key, set2Key]);
        console.log("difference result:", difference);

        expect(difference).toBeDefined();
        expect(Array.isArray(difference)).toBe(true);
        expect(difference).toContain("diff1_only");
        expect(difference).not.toContain("diff2_only");
        expect(difference).not.toContain("common");

        console.log("✅ difference operation successful!");
      } catch (error) {
        console.log("❌ difference error:", error);
        throw error;
      }
    });

    it("should handle complex set operations workflow", async () => {
      console.log("Testing complex set operations workflow");

      try {
        // Create multiple sets
        await setClient.addMany("workflow:set1", ["a", "b", "c"]);
        await setClient.addMany("workflow:set2", ["b", "c", "d"]);
        await setClient.addMany("workflow:set3", ["c", "d", "e"]);

        // Test individual operations
        const members1 = await setClient.members("workflow:set1");
        expect(members1).toContain("a");
        expect(members1).toContain("b");
        expect(members1).toContain("c");

        const cardinality1 = await setClient.cardinality("workflow:set1");
        expect(cardinality1).toBe(3);

        const exists = await setClient.exists("workflow:set1", "a");
        expect(exists).toBe(true);

        // Test set operations
        const intersection = await setClient.intersect(["workflow:set1", "workflow:set2"]);
        expect(intersection).toContain("b");
        expect(intersection).toContain("c");
        expect(intersection).not.toContain("a");
        expect(intersection).not.toContain("d");

        const union = await setClient.union(["workflow:set1", "workflow:set2"]);
        expect(union).toContain("a");
        expect(union).toContain("b");
        expect(union).toContain("c");
        expect(union).toContain("d");

        const difference = await setClient.difference(["workflow:set1", "workflow:set2"]);
        expect(difference).toContain("a");
        expect(difference).not.toContain("b");
        expect(difference).not.toContain("c");
        expect(difference).not.toContain("d");

        // Test removal
        const removeResult = await setClient.remove("workflow:set1", "a");
        expect(removeResult).toBe(1); // Should return 1 member removed

        const membersAfterRemove = await setClient.members("workflow:set1");
        expect(membersAfterRemove).not.toContain("a");
        expect(membersAfterRemove).toContain("b");
        expect(membersAfterRemove).toContain("c");

        console.log("✅ complex set operations workflow successful!");
      } catch (error) {
        console.log("❌ complex set operations workflow error:", error);
        throw error;
      }
    });

    it("should handle edge cases and error conditions", async () => {
      console.log("Testing edge cases and error conditions");

      try {
        // Test with empty set
        const emptyMembers = await setClient.members("empty:set");
        expect(emptyMembers).toEqual([]);

        const emptyCardinality = await setClient.cardinality("empty:set");
        expect(emptyCardinality).toBe(0);

        const emptySize = await setClient.size("empty:set");
        expect(emptySize).toBe(0);

        // Test intersection with empty sets
        const emptyIntersection = await setClient.intersect(["empty:set1", "empty:set2"]);
        expect(emptyIntersection).toEqual([]);

        // Test union with empty sets
        const emptyUnion = await setClient.union(["empty:set1", "empty:set2"]);
        expect(emptyUnion).toEqual([]);

        // Test difference with empty sets
        const emptyDifference = await setClient.difference(["empty:set1", "empty:set2"]);
        expect(emptyDifference).toEqual([]);

        // Test adding duplicate members
        await setClient.addOne("duplicate:set", "member1");
        await setClient.addOne("duplicate:set", "member1"); // Duplicate
        const duplicateMembers = await setClient.members("duplicate:set");
        expect(duplicateMembers).toContain("member1");
        expect(duplicateMembers.filter((m) => m === "member1").length).toBe(1); // Should only appear once

        console.log("✅ edge cases and error conditions successful!");
      } catch (error) {
        console.log("❌ edge cases and error conditions error:", error);
        throw error;
      }
    });
  });
});
