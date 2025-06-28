const { createClient } = require("./index.js");

async function example() {
  try {
    console.log("=== DBX Redis TypeScript Bindings Example ===\n");

    // Create a client
    const client = createClient("http://localhost:8080");
    console.log("✓ Client created with base URL:", client.getBaseUrl());

    // String operations
    console.log("\n1. String Operations:");

    // Set simple string
    await client.string().setSimple("user:1:name", "Alice");
    console.log("  ✓ Set user:1:name = Alice");

    // Set string with TTL
    await client.string().setWithTtl("user:1:session", "abc123", 3600);
    console.log("  ✓ Set user:1:session = abc123 (TTL: 3600s)");

    // Get values
    const name = await client.string().get("user:1:name");
    console.log("  ✓ Retrieved user:1:name =", name);

    const session = await client.string().get("user:1:session");
    console.log("  ✓ Retrieved user:1:session =", session);

    // Get string info
    const info = await client.string().info("user:1:name");
    if (info) {
      console.log("  ✓ String info: key=" + info.key + ", size=" + info.size + ", ttl=" + info.ttl);
    }

    // Batch operations
    console.log("\n2. Batch String Operations:");

    const operations = [
      { key: "user:2:name", value: "Bob" },
      { key: "user:2:email", value: "bob@example.com" },
      { key: "user:2:age", value: "30", ttl: 7200 },
    ];

    await client.string().batchSet(operations);
    console.log("  ✓ Batch set 3 values");

    // Batch get
    const keys = ["user:2:name", "user:2:email", "user:2:age"];
    const values = await client.string().batchGet(keys);
    console.log("  ✓ Batch retrieved:", values);

    // Pattern search
    console.log("\n3. Pattern Search:");
    const patterns = ["user:*:name"];
    const results = await client.string().getByPatterns(patterns, false);
    console.log("  ✓ Pattern search results:", results);

    // Set operations
    console.log("\n4. Set Operations:");

    // Create sets
    await client.set().addMany("users:online", ["user:1", "user:2", "user:3"]);
    console.log("  ✓ Added 3 users to online set");

    await client.set().addMany("users:premium", ["user:1", "user:4"]);
    console.log("  ✓ Added 2 users to premium set");

    await client.set().addMany("users:admin", ["user:1", "user:5"]);
    console.log("  ✓ Added 2 users to admin set");

    // Get set members
    const onlineUsers = await client.set().members("users:online");
    console.log("  ✓ Online users:", onlineUsers);

    const premiumUsers = await client.set().members("users:premium");
    console.log("  ✓ Premium users:", premiumUsers);

    // Check membership
    const isPremium = await client.set().contains("users:premium", "user:1");
    console.log("  ✓ User 1 is premium:", isPremium);

    const isAdmin = await client.set().contains("users:admin", "user:2");
    console.log("  ✓ User 2 is admin:", isAdmin);

    // Get set sizes
    const onlineCount = await client.set().size("users:online");
    console.log("  ✓ Online users count:", onlineCount);

    // Set operations
    console.log("\n5. Set Operations (Intersect, Union, Difference):");

    const premiumOnline = await client.set().intersect(["users:online", "users:premium"]);
    console.log("  ✓ Premium online users (intersection):", premiumOnline);

    const allUsers = await client.set().union(["users:online", "users:premium", "users:admin"]);
    console.log("  ✓ All users (union):", allUsers);

    const onlineOnly = await client.set().difference(["users:online", "users:premium"]);
    console.log("  ✓ Online only (difference):", onlineOnly);

    // Remove a member
    const removed = await client.set().remove("users:online", "user:3");
    console.log("  ✓ Removed user:3 from online set (" + removed + " removed)");

    // Cleanup
    console.log("\n6. Cleanup:");

    const deleted = await client.string().delete("user:1:name");
    console.log("  ✓ Deleted user:1:name:", deleted);

    const deleted2 = await client.string().delete("user:1:session");
    console.log("  ✓ Deleted user:1:session:", deleted2);

    const deleted3 = await client.string().delete("user:2:name");
    console.log("  ✓ Deleted user:2:name:", deleted3);

    const deleted4 = await client.string().delete("user:2:email");
    console.log("  ✓ Deleted user:2:email:", deleted4);

    const deleted5 = await client.string().delete("user:2:age");
    console.log("  ✓ Deleted user:2:age:", deleted5);

    console.log("\n=== Example completed successfully! ===");
  } catch (error) {
    console.error("Error:", error.message);
    process.exit(1);
  }
}

// Run the example
example();
