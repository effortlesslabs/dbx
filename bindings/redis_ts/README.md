# @0dbx/redis – TypeScript SDK for DBX Redis API

High-performance TypeScript/JavaScript SDK for the DBX Redis API, powered by native Rust (NAPI) bindings. Provides modern, type-safe, and efficient access to Redis via HTTP and WebSocket APIs.

## Features

- **Native Performance**: Built with Rust and NAPI-rs for maximum speed
- **TypeScript Support**: Full type definitions and type safety
- **Async/Await**: Modern async/await API
- **REST & WebSocket**: Unified API for both HTTP and WebSocket
- **String, Set, Hash, Admin**: Full Redis data type support
- **Comprehensive Error Handling**
- **Cross-Platform**: Pre-built binaries for major platforms

## Installation

```bash
npm install @0dbx/redis
# or
yarn add @0dbx/redis
# or
pnpm add @0dbx/redis
```

## Quick Start

### REST Client Example

```typescript
import { DbxRedisClient } from "@0dbx/redis";

const client = new DbxRedisClient("http://localhost:3000");

// String operations
await client.string.set("my-key", "hello world", 3600); // with TTL
const value = await client.string.get("my-key");
console.log(value); // "hello world"

// Set operations
await client.set.addMember("tags", "redis");
const members = await client.set.getMembers("tags");
console.log(members); // ["redis"]

// Hash operations
await client.hash.setField("user:1", "name", "Alice");
const name = await client.hash.getField("user:1", "name");
console.log(name); // "Alice"

// Admin operations
const health = await client.admin.health();
console.log(health); // { status: "ok", ... }
```

### WebSocket Client Example

```typescript
import { DbxWsClient } from "@0dbx/redis";

const wsClient = new DbxWsClient("ws://localhost:3000/redis_ws");

// String operations
await wsClient.string.set("my-key", "hello world");
const value = await wsClient.string.get("my-key");

// Set operations
await wsClient.set.addMember("tags", "redis");
const members = await wsClient.set.getMembers("tags");

// Hash operations
await wsClient.hash.setField("user:1", "name", "Alice");
const name = await wsClient.hash.getField("user:1", "name");

// Admin operations
const health = await wsClient.admin.health();
```

## API Reference

### Client Creation

```typescript
import { DbxRedisClient, DbxWsClient } from "@0dbx/redis";

// REST client
const client = new DbxRedisClient("http://localhost:3000");

// WebSocket client
const wsClient = new DbxWsClient("ws://localhost:3000/redis_ws");
```

### String Operations

```typescript
// Set a string value (with optional TTL in seconds)
await client.string.set("key", "value", 3600);

// Get a string value
const value = await client.string.get("key");

// Delete a string
await client.string.delete("key");
```

### Set Operations

```typescript
// Add a member to a set
await client.set.addMember("set_key", "member1");

// Get all members
const members = await client.set.getMembers("set_key");

// Remove a member
await client.set.removeMember("set_key", "member1");
```

### Hash Operations

```typescript
// Set a field in a hash
await client.hash.setField("user:1", "name", "Alice");

// Get a field
const name = await client.hash.getField("user:1", "name");

// Delete a field
await client.hash.deleteField("user:1", "name");
```

### Admin Operations

```typescript
// Health check
const health = await client.admin.health();

// Ping
const ping = await client.admin.ping();
```

## Error Handling

All methods throw errors with descriptive messages if something goes wrong:

```typescript
try {
  await client.string.get("nonexistent_key");
} catch (error) {
  console.error("Error:", error.message);
}
```

## Building from Source

If you want to build the SDK from source:

```bash
git clone https://github.com/effortlesslabs/dbx.git
cd dbx/bindings/redis_ts
npm install
npm run build
```

## Platform Support

Pre-built binaries are available for:

- **Windows**: x64, x86
- **macOS**: x64, ARM64
- **Linux**: x64, ARM64, ARMv7
- **FreeBSD**: x64

## Development

```bash
# Install dependencies
npm install

# Build in debug mode
npm run build:debug

# Build for release
npm run build

# Test the SDK
npm test
```

## License

MIT License – see the main project license for details.
