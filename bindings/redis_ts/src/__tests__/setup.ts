import { beforeAll, afterAll } from "vitest";

// Test configuration
export const TEST_CONFIG = {
  // Update these URLs to match your test environment
  HTTP_URL: process.env.TEST_HTTP_URL || "http://0.0.0.0:3000",
  WS_URL: process.env.TEST_WS_URL || "ws://0.0.0.0:3000/redis_ws",
};

// Test utilities
export function generateTestKey(prefix: string = "test"): string {
  return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

export function generateTestValue(): string {
  return `value_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

// Global setup and teardown
beforeAll(() => {
  console.log("🧪 Starting Redis TypeScript Bindings Tests");
  console.log(`📡 HTTP URL: ${TEST_CONFIG.HTTP_URL}`);
  console.log(`🔌 WebSocket URL: ${TEST_CONFIG.WS_URL}`);
});

afterAll(() => {
  console.log("✅ Redis TypeScript Bindings Tests completed");
});
