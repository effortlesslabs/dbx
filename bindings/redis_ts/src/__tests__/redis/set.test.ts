import { describe, it, expect, beforeAll, afterAll, beforeEach } from "vitest";
const { DbxRedisClient } = require("../../../index.js");

describe("Redis HTTP Set Operations", () => {
  let client: any;
  const TEST_BASE_URL = process.env.REDIS_HTTP_URL || "http://localhost:3000";

  beforeAll(async () => {
    client = new DbxRedisClient(TEST_BASE_URL);
  });

  afterAll(async () => {
    const setClient = client.set();
    await setClient.delete("test:set:1");
    await setClient.delete("test:set:2");
    await setClient.delete("test:set:3");
  });

  beforeEach(async () => {
    const setClient = client.set();
    await setClient.delete("test:set:1");
    await setClient.delete("test:set:2");
    await setClient.delete("test:set:3");
  });

  describe("add_one", () => {
    it("should add a single member to a set", async () => {
      const setClient = client.set();
      const result = await setClient.addOne("test:set:1", "member1");
      expect(result).toBe(true);
    });

    it("should return true when adding duplicate member", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");
      const result = await setClient.addOne("test:set:1", "member1");
      expect(result).toBe(true);
    });
  });

  describe("add_many", () => {
    it("should add multiple members to a set", async () => {
      const setClient = client.set();
      const members = ["member1", "member2", "member3"];
      const result = await setClient.addMany("test:set:1", members);
      expect(result).toBe(true);
    });

    it("should handle empty array of members", async () => {
      const setClient = client.set();
      const result = await setClient.addMany("test:set:1", []);
      expect(result).toBe(true);
    });

    it("should handle duplicate members in array", async () => {
      const setClient = client.set();
      const members = ["member1", "member1", "member2"];
      const result = await setClient.addMany("test:set:1", members);
      expect(result).toBe(true);
    });
  });

  describe("remove", () => {
    it("should remove a member from a set", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");
      const result = await setClient.remove("test:set:1", "member1");
      expect(result).toBe(true);
    });

    it("should return true when removing non-existent member", async () => {
      const setClient = client.set();
      const result = await setClient.remove("test:set:1", "non-existent");
      expect(result).toBe(true);
    });
  });

  describe("members", () => {
    it("should get all members of a set", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2", "member3"]);
      const members = await setClient.members("test:set:1");
      expect(members).toContain("member1");
      expect(members).toContain("member2");
      expect(members).toContain("member3");
      expect(members).toHaveLength(3);
    });

    it("should return empty array for non-existent set", async () => {
      const setClient = client.set();
      const members = await setClient.members("non-existent:set");
      expect(members).toEqual([]);
    });
  });

  describe("cardinality", () => {
    it("should return correct cardinality of a set", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2", "member3"]);
      const cardinality = await setClient.cardinality("test:set:1");
      expect(cardinality).toBe(3);
    });

    it("should return 0 for empty set", async () => {
      const setClient = client.set();
      const cardinality = await setClient.cardinality("test:set:1");
      expect(cardinality).toBe(0);
    });
  });

  describe("exists", () => {
    it("should return true for existing member", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");
      const exists = await setClient.exists("test:set:1", "member1");
      expect(exists).toBe(true);
    });

    it("should return false for non-existing member", async () => {
      const setClient = client.set();
      const exists = await setClient.exists("test:set:1", "non-existent");
      expect(exists).toBe(false);
    });
  });

  describe("contains", () => {
    it("should return true for existing member (alias for exists)", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");
      const contains = await setClient.contains("test:set:1", "member1");
      expect(contains).toBe(true);
    });

    it("should return false for non-existing member", async () => {
      const setClient = client.set();
      const contains = await setClient.contains("test:set:1", "non-existent");
      expect(contains).toBe(false);
    });
  });

  describe("size", () => {
    it("should return correct size of a set", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2", "member3"]);
      const size = await setClient.size("test:set:1");
      expect(size).toBe(3);
    });

    it("should return 0 for empty set", async () => {
      const setClient = client.set();
      const size = await setClient.size("test:set:1");
      expect(size).toBe(0);
    });
  });

  describe("intersect", () => {
    it("should return intersection of multiple sets", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2", "member3"]);
      await setClient.addMany("test:set:2", ["member2", "member3", "member4"]);
      await setClient.addMany("test:set:3", ["member1", "member4", "member5"]);

      const intersection = await setClient.intersect(["test:set:1", "test:set:2"]);
      expect(intersection).toContain("member2");
      expect(intersection).toContain("member3");
      expect(intersection).toHaveLength(2);
    });

    it("should return empty array when no intersection", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");
      await setClient.addOne("test:set:2", "member2");

      const intersection = await setClient.intersect(["test:set:1", "test:set:2"]);
      expect(intersection).toEqual([]);
    });
  });

  describe("union", () => {
    it("should return union of multiple sets", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2"]);
      await setClient.addMany("test:set:2", ["member2", "member3"]);

      const union = await setClient.union(["test:set:1", "test:set:2"]);
      expect(union).toContain("member1");
      expect(union).toContain("member2");
      expect(union).toContain("member3");
      expect(union).toHaveLength(3);
    });

    it("should handle empty sets in union", async () => {
      const setClient = client.set();
      await setClient.addOne("test:set:1", "member1");

      const union = await setClient.union(["test:set:1", "test:set:2"]);
      expect(union).toContain("member1");
      expect(union).toHaveLength(1);
    });
  });

  describe("difference", () => {
    it("should return difference of multiple sets", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2", "member3"]);
      await setClient.addMany("test:set:2", ["member2", "member3"]);

      // Debug: print members
      const members1 = await setClient.members("test:set:1");
      const members2 = await setClient.members("test:set:2");
      console.log("test:set:1 members:", members1);
      console.log("test:set:2 members:", members2);

      const difference = await setClient.difference(["test:set:1", "test:set:2"]);
      console.log("difference:", difference);
      expect(difference).toContain("member1");
      expect(difference).toHaveLength(1);
    });

    it("should return empty array when no difference", async () => {
      const setClient = client.set();
      await setClient.addMany("test:set:1", ["member1", "member2"]);
      await setClient.addMany("test:set:2", ["member1", "member2"]);

      const difference = await setClient.difference(["test:set:1", "test:set:2"]);
      expect(difference).toEqual([]);
    });
  });
});
