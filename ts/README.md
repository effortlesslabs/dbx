# DBX TypeScript Client

A comprehensive TypeScript client for the DBX API, providing easy access to Redis operations through HTTP and WebSocket interfaces.

## Installation

```bash
npm install @effortlesslabs/dbx
```

## Quick Start

```typescript
import { createDbxClient } from "@effortlesslabs/dbx";

// Create a client
const client = createDbxClient("http://localhost:8080");

// Basic operations
await client.setString("key", "value", 3600); // Set with TTL
const value = await client.getString("key");
console.log(value); // 'value'
```

## Configuration

```typescript
import { DbxClient } from "@effortlesslabs/dbx";

const client = new DbxClient({
  baseUrl: "http://localhost:8080",
  timeout: 10000, // 10 seconds
  headers: {
    Authorization: "Bearer your-token",
  },
});
```

## API Reference

### Server Operations

#### Health Check

```typescript
const health = await client.health();
// Returns: { status: string, redis_connected: boolean, timestamp: string }
```

#### Server Info

```typescript
const info = await client.info();
// Returns: { name: string, version: string, redis_url: string, pool_size: number }
```

#### Database Size

```typescript
const size = await client.dbSize();
// Returns: number
```

#### Flush Operations

```typescript
await client.flushAll(); // Flush all databases
await client.flushDb(); // Flush current database
```

### String Operations

#### Basic String Operations

```typescript
// Set a string value
await client.setString("key", "value", 3600); // Optional TTL in seconds

// Get a string value
const value = await client.getString("key");

// Delete a string
const deleted = await client.deleteString("key");

// Check if key exists
const exists = await client.exists("key");

// Get TTL
const ttl = await client.getTtl("key");
```

#### Counter Operations

```typescript
// Increment by 1
const newValue = await client.incr("counter");

// Increment by specific amount
const newValue = await client.incrBy("counter", 5);
```

#### Conditional Operations

```typescript
// Set only if key doesn't exist
const set = await client.setNx("key", "value", 3600);

// Compare and set (CAS)
const success = await client.compareAndSet("key", "expected", "new-value", 3600);
```

#### Batch String Operations

```typescript
// Batch set multiple key-value pairs
await client.batchSet(
  {
    key1: "value1",
    key2: "value2",
  },
  3600
); // Optional TTL

// Batch get multiple values
const values = await client.batchGet(["key1", "key2", "key3"]);

// Batch delete multiple keys
const deleted = await client.batchDelete(["key1", "key2"]);

// Batch increment multiple counters
const newValues = await client.batchIncr(["counter1", "counter2"]);

// Batch increment by specific amounts
const newValues = await client.batchIncrBy([
  ["counter1", 5],
  ["counter2", 10],
]);
```

### Set Operations

#### Basic Set Operations

```typescript
// Add members to a set
await client.addSetMembers("set1", ["member1", "member2", "member3"]);

// Get all members of a set
const members = await client.getSetMembers("set1");

// Delete a set
const deleted = await client.deleteSet("set1");

// Check if member exists
const exists = await client.setMemberExists("set1", "member1");

// Get set cardinality (size)
const size = await client.getSetCardinality("set1");
```

#### Set Member Operations

```typescript
// Get a random member
const member = await client.getRandomSetMember("set1");

// Pop a random member
const member = await client.popRandomSetMember("set1");

// Move a member to another set
const moved = await client.moveSetMember("set1", "member1", "set2");
```

#### Set Operations

```typescript
// Union of multiple sets
const union = await client.setUnion("result", ["set1", "set2", "set3"]);

// Intersection of multiple sets
const intersection = await client.setIntersection("result", ["set1", "set2"]);

// Difference of multiple sets
const difference = await client.setDifference("result", ["set1", "set2"]);
```

#### Batch Set Operations

```typescript
// Batch add members to multiple sets
await client.batchAddSetMembers({
  set1: ["member1", "member2"],
  set2: ["member3", "member4"],
});

// Batch remove members from multiple sets
await client.batchRemoveSetMembers({
  set1: ["member1"],
  set2: ["member3"],
});

// Batch get members from multiple sets
const allMembers = await client.batchGetSetMembers(["set1", "set2"]);

// Batch delete multiple sets
const deleted = await client.batchDeleteSets(["set1", "set2"]);
```

### Hash Operations

#### Basic Hash Operations

```typescript
// Set a hash field
await client.setHashField("hash1", "field1", "value1");

// Get a hash field
const value = await client.getHashField("hash1", "field1");

// Delete a hash field
const deleted = await client.deleteHashField("hash1", "field1");

// Check if hash field exists
const exists = await client.hashFieldExists("hash1", "field1");

// Delete entire hash
const deleted = await client.deleteHash("hash1");
```

#### Hash Field Operations

```typescript
// Increment a hash field
const newValue = await client.incrementHashField("hash1", "counter", 5);

// Set hash field only if it doesn't exist
const set = await client.setHashFieldNx("hash1", "field1", "value1");

// Get random hash field
const field = await client.getRandomHashField("hash1");
```

#### Hash Information

```typescript
// Get hash length (number of fields)
const length = await client.getHashLength("hash1");

// Get all hash field names
const keys = await client.getHashKeys("hash1");

// Get all hash field values
const values = await client.getHashValues("hash1");

// Get all hash fields and values
const all = await client.getHashAll("hash1");
```

#### Multiple Hash Fields

```typescript
// Set multiple hash fields
await client.setHashMultiple("hash1", {
  field1: "value1",
  field2: "value2",
  field3: "value3",
});

// Get multiple hash fields
const values = await client.getMultipleHashFields("hash1", ["field1", "field2"]);
```

#### Batch Hash Operations

```typescript
// Batch set hash fields across multiple hashes
await client.batchSetHashFields({
  hash1: { field1: "value1", field2: "value2" },
  hash2: { field3: "value3" },
});

// Batch get hash fields across multiple hashes
const values = await client.batchGetHashFields({
  hash1: ["field1", "field2"],
  hash2: ["field3"],
});

// Batch delete hash fields across multiple hashes
const deleted = await client.batchDeleteHashFields({
  hash1: ["field1"],
  hash2: ["field3"],
});

// Batch get all fields from multiple hashes
const allFields = await client.batchGetHashAll(["hash1", "hash2"]);

// Batch check if hash fields exist
const exists = await client.batchCheckHashFields({
  hash1: ["field1", "field2"],
  hash2: ["field3"],
});

// Batch get hash lengths
const lengths = await client.batchGetHashLengths(["hash1", "hash2"]);
```

### Key Operations

```typescript
// List all keys
const keys = await client.listKeys();

// List keys matching pattern
const keys = await client.listKeys("user:*");

// Check if key exists
const exists = await client.keyExists("key1");

// Get key TTL
const ttl = await client.keyTtl("key1");

// Delete a key
const deleted = await client.deleteKey("key1");
```

### WebSocket Support

The client provides WebSocket support for real-time operations:

```typescript
// Create WebSocket connection
const ws = client.createWebSocket({
  url: "ws://localhost:8080/ws",
  onOpen: () => {
    console.log("Connected to WebSocket");
  },
  onMessage: (response) => {
    console.log("Received:", response);
  },
  onError: (error) => {
    console.error("WebSocket error:", error);
  },
  onClose: () => {
    console.log("WebSocket closed");
  },
});

// Send commands
client.sendWebSocketCommand(
  ws,
  {
    action: "get",
    params: { key: "my-key" },
  },
  "cmd1"
);

client.sendWebSocketCommand(
  ws,
  {
    action: "set",
    params: { key: "my-key", value: "my-value", ttl: 3600 },
  },
  "cmd2"
);

client.sendWebSocketCommand(
  ws,
  {
    action: "batch_get",
    params: { keys: ["key1", "key2", "key3"] },
  },
  "cmd3"
);
```

#### Supported WebSocket Commands

- `get` - Get a string value
- `set` - Set a string value
- `delete` - Delete a key
- `exists` - Check if key exists
- `ttl` - Get key TTL
- `incr` - Increment counter
- `incrby` - Increment by amount
- `setnx` - Set if not exists
- `cas` - Compare and set
- `batch_get` - Batch get multiple values
- `batch_set` - Batch set multiple values
- `batch_delete` - Batch delete multiple keys
- `batch_incr` - Batch increment counters
- `batch_incrby` - Batch increment by amounts
- `list_keys` - List keys with optional pattern
- `ping` - Ping the server
- `subscribe` - Subscribe to channels
- `unsubscribe` - Unsubscribe from channels

## Error Handling

The client throws errors for failed operations:

```typescript
try {
  const value = await client.getString("non-existent-key");
} catch (error) {
  console.error("Operation failed:", error.message);
}
```

## Examples

See the `examples/` directory for complete usage examples:

- `basic-usage.ts` - Comprehensive example showing all operations
- WebSocket usage examples

## TypeScript Support

The library is written in TypeScript and provides full type safety:

```typescript
import { DbxClient, ApiResponse, StringValue } from "@effortlesslabs/dbx";

const client: DbxClient = createDbxClient("http://localhost:8080");
const response: ApiResponse<StringValue> = await client.getString("key");
```

## License

MIT
