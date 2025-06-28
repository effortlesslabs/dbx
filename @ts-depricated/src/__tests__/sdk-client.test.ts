import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { DbxClient } from "../client";
import { getConfig } from "../config";
import { WebSocketClient } from "../clients/websocket";
import type { WebSocketConfig } from "../types/admin";

const config = getConfig();

// HTTP Test

describe("DBX SDK - HTTP", () => {
  let client: DbxClient;

  beforeAll(() => {
    client = new DbxClient(config);
  });

  it("should ping the server", async () => {
    const res = await client.admin.ping();
    expect(typeof res).toBe("string");
    expect(res.toLowerCase()).toContain("pong");
  });

  // WebSocket Tests

  describe("DBX SDK - WebSocket", () => {
    describe("Admin WebSocket", () => {
      let wsClient: WebSocketClient | null = null;
      let pongReceived = false;

      beforeAll(async () => {
        wsClient = client.createAdminWebSocket({
          onMessage: (msg: any) => {
            console.log("Admin WebSocket message:", msg);
            if (msg && msg.type === "ping_result") pongReceived = true;
          },
          onError: (error) => {
            console.error("Admin WebSocket error:", error);
          },
          onClose: () => {},
          onOpen: () => {},
        });

        try {
          await wsClient.connect();
        } catch (e) {
          console.error("Admin WebSocket connection failed:", e);
          wsClient = null;
        }
      });

      afterAll(() => {
        if (wsClient) wsClient.close();
      });

      it("should connect and receive pong on ping", async function () {
        expect(wsClient).not.toBeNull();
        pongReceived = false;

        wsClient!.sendMessage({ type: "ping" });

        await new Promise((resolve) => setTimeout(resolve, 1000));
        expect(pongReceived).toBe(true);
      });
    });

    describe("String WebSocket", () => {
      let wsClient: WebSocketClient | null = null;
      let pongReceived = false;

      beforeAll(async () => {
        wsClient = client.createStringWebSocket({
          onMessage: (msg: any) => {
            console.log("String WebSocket message:", msg);
            if (msg && msg.type === "pong") pongReceived = true;
          },
          onError: (error) => {
            console.error("String WebSocket error:", error);
          },
          onClose: () => {},
          onOpen: () => {},
        });

        try {
          await wsClient.connect();
        } catch (e) {
          console.error("String WebSocket connection failed:", e);
          wsClient = null;
        }
      });

      afterAll(() => {
        if (wsClient) wsClient.close();
      });

      it("should connect and receive pong on ping", async function () {
        expect(wsClient).not.toBeNull();
        pongReceived = false;

        wsClient!.sendMessage({ type: "ping" });

        await new Promise((resolve) => setTimeout(resolve, 1000));
        expect(pongReceived).toBe(true);
      });
    });

    describe("Hash WebSocket", () => {
      let wsClient: WebSocketClient | null = null;
      let pongReceived = false;

      beforeAll(async () => {
        wsClient = client.createHashWebSocket({
          onMessage: (msg: any) => {
            console.log("Hash WebSocket message:", msg);
            if (msg && msg.type === "pong") pongReceived = true;
          },
          onError: (error) => {
            console.error("Hash WebSocket error:", error);
          },
          onClose: () => {},
          onOpen: () => {},
        });

        try {
          await wsClient.connect();
        } catch (e) {
          console.error("Hash WebSocket connection failed:", e);
          wsClient = null;
        }
      });

      afterAll(() => {
        if (wsClient) wsClient.close();
      });

      it("should connect and receive pong on ping", async function () {
        expect(wsClient).not.toBeNull();
        pongReceived = false;

        wsClient!.sendMessage({ type: "ping" });

        await new Promise((resolve) => setTimeout(resolve, 1000));
        expect(pongReceived).toBe(true);
      });
    });

    describe("Set WebSocket", () => {
      let wsClient: WebSocketClient | null = null;
      let pongReceived = false;

      beforeAll(async () => {
        wsClient = client.createSetWebSocket({
          onMessage: (msg: any) => {
            console.log("Set WebSocket message:", msg);
            if (msg && msg.type === "pong") pongReceived = true;
          },
          onError: (error) => {
            console.error("Set WebSocket error:", error);
          },
          onClose: () => {},
          onOpen: () => {},
        });

        try {
          await wsClient.connect();
        } catch (e) {
          console.error("Set WebSocket connection failed:", e);
          wsClient = null;
        }
      });

      afterAll(() => {
        if (wsClient) wsClient.close();
      });

      it("should connect and receive pong on ping", async function () {
        expect(wsClient).not.toBeNull();
        pongReceived = false;

        wsClient!.sendMessage({ type: "ping" });

        await new Promise((resolve) => setTimeout(resolve, 1000));
        expect(pongReceived).toBe(true);
      });
    });
  });

  describe("StringClient - Pattern-based operations", () => {
    it("should support pattern-based batch operations", async () => {
      // Set up test data
      await client.string.set("tokenBalance:0x123:ethereum:100", "100.5");
      await client.string.set("tokenBalance:0x123:ethereum:200", "200.0");
      await client.string.set("tokenBalancePending:0x123:ethereum:50", "50.25");
      await client.string.set("tokenBalancePending:0x123:ethereum:75", "75.75");
      await client.string.set("otherKey:0x456:ethereum:300", "300.0");

      // Test flat pattern matching
      const flatResults = await client.string.batchGetPatternsFlat([
        "tokenBalance:0x123:ethereum:*",
        "tokenBalancePending:0x123:ethereum:*",
      ]);

      expect(flatResults).toEqual({
        "tokenBalance:0x123:ethereum:100": "100.5",
        "tokenBalance:0x123:ethereum:200": "200.0",
        "tokenBalancePending:0x123:ethereum:50": "50.25",
        "tokenBalancePending:0x123:ethereum:75": "75.75",
      });

      // Test grouped pattern matching
      const groupedResults = await client.string.batchGetPatternsGrouped([
        "tokenBalance:0x123:ethereum:*",
        "tokenBalancePending:0x123:ethereum:*",
      ]);

      expect(groupedResults).toHaveLength(2);
      expect(groupedResults[0].pattern).toBe("tokenBalance:0x123:ethereum:*");
      expect(groupedResults[0].results).toEqual({
        "tokenBalance:0x123:ethereum:100": "100.5",
        "tokenBalance:0x123:ethereum:200": "200.0",
      });
      expect(groupedResults[1].pattern).toBe("tokenBalancePending:0x123:ethereum:*");
      expect(groupedResults[1].results).toEqual({
        "tokenBalancePending:0x123:ethereum:50": "50.25",
        "tokenBalancePending:0x123:ethereum:75": "75.75",
      });

      // Cleanup
      await client.string.delete("tokenBalance:0x123:ethereum:100");
      await client.string.delete("tokenBalance:0x123:ethereum:200");
      await client.string.delete("tokenBalancePending:0x123:ethereum:50");
      await client.string.delete("tokenBalancePending:0x123:ethereum:75");
      await client.string.delete("otherKey:0x456:ethereum:300");
    });
  });
});
