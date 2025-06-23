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
});
