# DBX TypeScript SDK

A TypeScript SDK for DBX - A minimal API layer for databases. Built with native fetch API for modern environments.

## Requirements

- Node.js 18.0.0 or higher (for native fetch support)
- TypeScript 5.0.0 or higher

## Installation

```bash
npm install @dbx/sdk
```

## Quick Start

```typescript
import { DbxClient } from "@dbx/sdk";

// Create a new client
const client = new DbxClient({
  baseUrl: "http://localhost:3000",
  timeout: 5000,
});

// Basic operations
await client.setString("my-key", "my-value", 3600); // Set with 1 hour TTL
const value = await client.getString("my-key");
const exists = await client.exists("my-key");
const counter = await client.incr("my-counter");
```

## Features

- **Fetch-based**: Uses native fetch API for HTTP requests (no external dependencies)
- **Full TypeScript Support**: Complete type definitions for all API operations
- **String Operations**: Get, set, delete, increment, TTL management
- **Batch Operations**: Efficient bulk operations for multiple keys
- **Key Management**: List, check existence, TTL, delete keys
- **Lua Scripts**: Execute rate limiting and multi-counter scripts
- **Error Handling**: Comprehensive error handling with meaningful messages
- **Health & Info**: Server health checks and information
- **Timeout Support**: Configurable request timeouts with AbortController

## API Reference

### Client Configuration

```typescript
interface DbxConfig {
  baseUrl: string;
  timeout?: number; // Default: 10000ms
  headers?: Record<string, string>;
}
```

### String Operations

```typescript
// Basic string operations
await client.setString(key: string, value: string, ttl?: number): Promise<string>
await client.getString(key: string): Promise<string>
await client.deleteString(key: string): Promise<number>

// Counter operations
await client.incr(key: string): Promise<number>
await client.incrBy(key: string, increment: number): Promise<number>

// Conditional operations
await client.setNx(key: string, value: string, ttl?: number): Promise<boolean>
await client.compareAndSet(key: string, expectedValue: string, newValue: string, ttl?: number): Promise<boolean>

// Utility operations
await client.exists(key: string): Promise<boolean>
await client.getTtl(key: string): Promise<number>
```

### Batch Operations

```typescript
// Batch operations for efficiency
await client.batchSet(keyValues: Record<string, string>, ttl?: number): Promise<Record<string, string>>
await client.batchGet(keys: string[]): Promise<Record<string, string>>
await client.batchDelete(keys: string[]): Promise<number>
await client.batchIncr(keys: string[]): Promise<number[]>
```

### Key Operations

```typescript
// Key management
await client.listKeys(pattern?: string): Promise<string[]>
await client.keyExists(key: string): Promise<boolean>
await client.keyTtl(key: string): Promise<number>
await client.deleteKey(key: string): Promise<number>
```

### Script Operations

```typescript
// Lua script execution
await client.rateLimiter(key: string, limit: number, window: number): Promise<boolean>
await client.multiCounter(counters: [string, number][]): Promise<number[]>
await client.multiSetTtl(keyValues: Record<string, string>, ttl: number): Promise<Record<string, string>>
```

### Server Information

```typescript
// Server health and info
await client.health(): Promise<HealthResponse>
await client.info(): Promise<InfoResponse>
```

## Examples

### Basic Usage

```typescript
import { DbxClient } from "@dbx/sdk";

const client = new DbxClient({
  baseUrl: "http://localhost:3000",
});

async function example() {
  // Set and get values
  await client.setString("user:123", "John Doe", 3600);
  const user = await client.getString("user:123");

  // Counter operations
  const views = await client.incr("page:views");

  // Batch operations
  await client.batchSet({
    "session:1": "active",
    "session:2": "inactive",
    "session:3": "active",
  });

  // Rate limiting
  const allowed = await client.rateLimiter("api:user:123", 100, 3600);
}
```

### Error Handling

```typescript
try {
  const value = await client.getString("non-existent-key");
} catch (error) {
  if (error.message === "Key not found") {
    console.log("Key does not exist");
  } else {
    console.error("Unexpected error:", error);
  }
}
```

### Advanced Usage

```typescript
// Compare and set for atomic updates
const success = await client.compareAndSet("user:balance", "100", "90", 3600);

// Multi-counter for analytics
const results = await client.multiCounter([
  ["page:views", 1],
  ["user:clicks", 5],
  ["api:requests", 1],
]);

// Pattern-based key listing
const userKeys = await client.listKeys("user:*");
const sessionKeys = await client.listKeys("session:*");
```

## Development

### Building

```bash
npm run build
```

### Testing

```bash
npm test
```

### Linting

```bash
npm run lint
```

## License

MIT
