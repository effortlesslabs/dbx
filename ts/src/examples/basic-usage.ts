import { createDbxClient } from "../index";

async function main() {
  // Create a DBX client (uses HOST_URL from .env file)
  const client = createDbxClient();

  try {
    // Health check
    console.log("Health check:", await client.health());

    // Server info
    console.log("Server info:", await client.info());

    // Database size
    console.log("Database size:", await client.dbSize());

    // String operations
    console.log("\n=== String Operations ===");

    await client.setString("user:1", "John Doe", 3600); // Set with TTL
    console.log("Set user:1:", await client.getString("user:1"));

    await client.incr("counter:1");
    console.log("Incremented counter:", await client.incrBy("counter:1", 5));

    console.log("Key exists:", await client.exists("user:1"));
    console.log("TTL:", await client.getTtl("user:1"));

    // Set operations
    console.log("\n=== Set Operations ===");

    await client.addSetMembers("tags:post:1", ["redis", "typescript", "api"]);
    console.log("Set members:", await client.getSetMembers("tags:post:1"));

    console.log("Member exists:", await client.setMemberExists("tags:post:1", "redis"));
    console.log("Cardinality:", await client.getSetCardinality("tags:post:1"));

    const randomMember = await client.getRandomSetMember("tags:post:1");
    console.log("Random member:", randomMember);

    // Hash operations
    console.log("\n=== Hash Operations ===");

    await client.setHashField("user:profile:1", "name", "John Doe");
    await client.setHashField("user:profile:1", "email", "john@example.com");
    await client.setHashField("user:profile:1", "age", "30");

    console.log("Hash field:", await client.getHashField("user:profile:1", "name"));
    console.log("All hash fields:", await client.getHashAll("user:profile:1"));

    console.log("Hash keys:", await client.getHashKeys("user:profile:1"));
    console.log("Hash length:", await client.getHashLength("user:profile:1"));

    // Batch operations
    console.log("\n=== Batch Operations ===");

    const batchData = {
      "user:2": "Jane Smith",
      "user:3": "Bob Johnson",
      "user:4": "Alice Brown",
    };

    await client.batchSet(batchData, 1800); // Set with TTL
    console.log("Batch get:", await client.batchGet(["user:2", "user:3", "user:4"]));

    // Key operations
    console.log("\n=== Key Operations ===");

    console.log("All keys:", await client.listKeys());
    console.log("User keys:", await client.listKeys("user:*"));

    console.log("Key exists:", await client.keyExists("user:1"));
    console.log("Key TTL:", await client.keyTtl("user:1"));

    // WebSocket example
    console.log("\n=== WebSocket Example ===");

    const ws = client.createWebSocket({
      url: "ws://localhost:8080/ws",
      onOpen: () => {
        console.log("WebSocket connected");

        // Send a get command
        client.sendWebSocketCommand(
          ws,
          {
            action: "get",
            params: { key: "user:1" },
          },
          "cmd1"
        );

        // Send a set command
        client.sendWebSocketCommand(
          ws,
          {
            action: "set",
            params: { key: "ws:test", value: "Hello WebSocket!", ttl: 300 },
          },
          "cmd2"
        );
      },
      onMessage: (response) => {
        console.log("WebSocket response:", response);
      },
      onError: (error) => {
        console.error("WebSocket error:", error);
      },
      onClose: () => {
        console.log("WebSocket closed");
      },
    });

    // Clean up
    console.log("\n=== Cleanup ===");

    await client.deleteString("user:1");
    await client.deleteSet("tags:post:1");
    await client.deleteHash("user:profile:1");
    await client.batchDelete(["user:2", "user:3", "user:4"]);

    console.log("Cleanup completed");
  } catch (error) {
    console.error("Error:", error);
  }
}

// Run the example
main().catch(console.error);
