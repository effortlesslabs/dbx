import { describe, it, expect } from "vitest";

// Import the bindings
let createClient: any;
let createWsClient: any;
let DbxRedisClient: any;
let DbxWsClient: any;

try {
  const bindings = require("../../index.js");
  createClient = bindings.createClient;
  createWsClient = bindings.createWsClient;
  DbxRedisClient = bindings.DbxRedisClient;
  DbxWsClient = bindings.DbxWsClient;
} catch (error) {
  console.error("Failed to load bindings:", error);
  throw error;
}

describe("Basic Bindings Test", () => {
  it("should load bindings successfully", () => {
    expect(createClient).toBeDefined();
    expect(createWsClient).toBeDefined();
    expect(DbxRedisClient).toBeDefined();
    expect(DbxWsClient).toBeDefined();
  });

  it("should create HTTP client instance", () => {
    const client = createClient("http://localhost:8080");
    expect(client).toBeDefined();
    expect(typeof client.get_base_url).toBe("function");
  });

  it("should create WebSocket client instance", () => {
    const client = createWsClient("ws://localhost:8080/ws");
    expect(client).toBeDefined();
    expect(typeof client.get_base_url).toBe("function");
  });

  it("should create HTTP client with constructor", () => {
    const client = new DbxRedisClient("http://localhost:8080");
    expect(client).toBeDefined();
    expect(typeof client.get_base_url).toBe("function");
  });

  it("should create WebSocket client with constructor", () => {
    const client = new DbxWsClient("ws://localhost:8080/ws");
    expect(client).toBeDefined();
    expect(typeof client.get_base_url).toBe("function");
  });

  it("should get base URL from HTTP client", () => {
    const client = createClient("http://localhost:8080");
    const baseUrl = client.get_base_url();
    expect(baseUrl).toBe("http://localhost:8080");
  });

  it("should get base URL from WebSocket client", () => {
    const client = createWsClient("ws://localhost:8080/ws");
    const baseUrl = client.get_base_url();
    expect(baseUrl).toBe("ws://localhost:8080/ws");
  });
});
