import { createDbxClient, getConfig } from "../index";

async function websocketExample() {
  const config = getConfig();
  const client = createDbxClient();

  // Create WebSocket connection using WS_HOST_URL from .env
  const ws = client.createWebSocket({
    url: config.wsHostUrl,
    onOpen: () => {
      console.log("WebSocket connected successfully");

      // Send various commands to test the WebSocket API
      setTimeout(() => {
        // Test basic operations
        client.sendWebSocketCommand(
          ws,
          {
            action: "set",
            params: { key: "ws:test", value: "Hello from WebSocket!", ttl: 300 },
          },
          "set1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "get",
            params: { key: "ws:test" },
          },
          "get1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "exists",
            params: { key: "ws:test" },
          },
          "exists1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "ttl",
            params: { key: "ws:test" },
          },
          "ttl1"
        );

        // Test counter operations
        client.sendWebSocketCommand(
          ws,
          {
            action: "incr",
            params: { key: "ws:counter" },
          },
          "incr1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "incrby",
            params: { key: "ws:counter", increment: 5 },
          },
          "incrby1"
        );

        // Test conditional operations
        client.sendWebSocketCommand(
          ws,
          {
            action: "setnx",
            params: { key: "ws:unique", value: "Only set once", ttl: 600 },
          },
          "setnx1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "cas",
            params: {
              key: "ws:cas",
              expected_value: "Hello from WebSocket!",
              new_value: "Updated via CAS",
              ttl: 300,
            },
          },
          "cas1"
        );

        // Test batch operations
        client.sendWebSocketCommand(
          ws,
          {
            action: "batch_set",
            params: {
              key_values: {
                "ws:batch1": "value1",
                "ws:batch2": "value2",
                "ws:batch3": "value3",
              },
              ttl: 1800,
            },
          },
          "batch_set1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "batch_get",
            params: { keys: ["ws:batch1", "ws:batch2", "ws:batch3"] },
          },
          "batch_get1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "batch_incr",
            params: { keys: ["ws:counter1", "ws:counter2"] },
          },
          "batch_incr1"
        );

        client.sendWebSocketCommand(
          ws,
          {
            action: "batch_incrby",
            params: {
              key_increments: [
                ["ws:counter1", 10],
                ["ws:counter2", 20],
              ],
            },
          },
          "batch_incrby1"
        );

        // Test key operations
        client.sendWebSocketCommand(
          ws,
          {
            action: "list_keys",
            params: { pattern: "ws:*" },
          },
          "list_keys1"
        );

        // Test ping
        client.sendWebSocketCommand(
          ws,
          {
            action: "ping",
            params: {},
          },
          "ping1"
        );

        // Clean up after a delay
        setTimeout(() => {
          client.sendWebSocketCommand(
            ws,
            {
              action: "batch_delete",
              params: {
                keys: [
                  "ws:test",
                  "ws:counter",
                  "ws:unique",
                  "ws:cas",
                  "ws:batch1",
                  "ws:batch2",
                  "ws:batch3",
                  "ws:counter1",
                  "ws:counter2",
                ],
              },
            },
            "cleanup"
          );

          // Close connection after cleanup
          setTimeout(() => {
            ws.close();
          }, 1000);
        }, 2000);
      }, 1000);
    },
    onMessage: (response) => {
      console.log(`[${response.id}] Response:`, {
        success: response.success,
        data: response.data,
        error: response.error,
        timestamp: response.timestamp,
      });
    },
    onError: (error) => {
      console.error("WebSocket error:", error);
    },
    onClose: (event) => {
      console.log("WebSocket connection closed:", event);
    },
  });

  // Handle process termination
  process.on("SIGINT", () => {
    console.log("\nClosing WebSocket connection...");
    ws.close();
    process.exit(0);
  });
}

// Run the WebSocket example
websocketExample().catch(console.error);
