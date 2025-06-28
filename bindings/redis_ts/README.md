# DBX Redis TypeScript Bindings

High-performance TypeScript bindings for the DBX Redis API using NAPI-rs. This package provides native Rust performance with TypeScript/JavaScript ease of use.

## Features

- **Native Performance**: Built with Rust and NAPI-rs for maximum performance
- **TypeScript Support**: Full TypeScript definitions included
- **Async/Await**: Modern async/await API
- **String Operations**: Get, set, delete, batch operations, pattern search
- **Set Operations**: Add, remove, membership checks, set operations (intersect, union, difference)
- **Error Handling**: Comprehensive error handling with detailed messages
- **Cross-Platform**: Pre-built binaries for multiple platforms

## Installation

```bash
npm install dbx-redis-ts-bindings
```

## Quick Start

```typescript
import { createClient } from "dbx-redis-ts-bindings";

async function example() {
  // Create a client
  const client = createClient("http://localhost:8080");

  // String operations
  await client.string().setSimple("my_key", "my_value");
  const value = await client.string().get("my_key");
  console.log("Value:", value); // "my_value"

  // Set operations
  await client.set().addMany("my_set", ["member1", "member2"]);
  const members = await client.set().members("my_set");
  console.log("Members:", members); // ["member1", "member2"]
}
```

## API Reference

### Client Creation

```typescript
import { createClient, createClientWithTimeout } from "dbx-redis-ts-bindings";

// Basic client
const client = createClient("http://localhost:8080");

// Client with custom timeout (in milliseconds)
const client = createClientWithTimeout("http://localhost:8080", 30000);
```

### String Operations

#### Basic Operations

```typescript
const stringClient = client.string();

// Set a string value
await stringClient.setSimple("key", "value");

// Set with TTL (time to live in seconds)
await stringClient.set("key", "value", 3600);

// Get a string value
const value = await stringClient.get("key");
// Returns string | null - null if key doesn't exist

// Delete a string
const deleted = await stringClient.delete("key");
// Returns boolean indicating if key was deleted

// Get string information
const info = await stringClient.info("key");
// Returns StringInfo | null with metadata
```

#### Batch Operations

```typescript
// Batch get multiple strings
const keys = ["key1", "key2", "key3"];
const values = await stringClient.batchGet(keys);
// Returns (string | null)[]

// Batch set multiple strings
const operations = [
  { key: "key1", value: "value1" },
  { key: "key2", value: "value2", ttl: 3600 },
];
await stringClient.batchSet(operations);

// Get strings by patterns
const patterns = ["user:*", "session:*"];
const results = await stringClient.getByPatterns(patterns, true);
// Returns JSON string with grouped results
```

### Set Operations

#### Basic Operations

```typescript
const setClient = client.set();

// Add a single member
const added = await setClient.addOne("set_key", "member1");
// Returns number (number of new members added)

// Add multiple members
const added = await setClient.addMany("set_key", ["member1", "member2", "member3"]);

// Remove a member
const removed = await setClient.remove("set_key", "member1");
// Returns number (number of members removed)

// Get all members
const members = await setClient.members("set_key");
// Returns string[]

// Check if member exists
const exists = await setClient.contains("set_key", "member1");
// Returns boolean

// Get set size (cardinality)
const size = await setClient.size("set_key");
// Returns number
```

#### Set Operations

```typescript
// Intersect multiple sets
const keys = ["set1", "set2", "set3"];
const intersection = await setClient.intersect(keys);
// Returns string[] of common members

// Union multiple sets
const union = await setClient.union(keys);
// Returns string[] of all unique members

// Difference of sets (first set minus others)
const difference = await setClient.difference(keys);
// Returns string[] of members in first set but not in others
```

## Types

### StringInfo

```typescript
interface StringInfo {
  key: string;
  value: string;
  ttl?: number;
  type: string;
  encoding: string;
  size: number;
}
```

### StringOperation

```typescript
interface StringOperation {
  key: string;
  value?: string;
  ttl?: number;
}
```

## Error Handling

The bindings provide detailed error messages:

```typescript
try {
  await client.string().get("nonexistent_key");
} catch (error) {
  console.error("Error:", error.message);
  // Error messages include details about what went wrong
}
```

## Performance

This package provides significant performance improvements over HTTP-based clients:

- **Native Speed**: Direct Rust execution without HTTP overhead
- **Efficient Memory Usage**: Optimized memory management
- **Reduced Latency**: No network round-trips for local operations
- **Batch Operations**: Efficient batch processing

## Building from Source

If you need to build from source:

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

# Test the bindings
npm test
```

## License

MIT License - see the main project license for details.
