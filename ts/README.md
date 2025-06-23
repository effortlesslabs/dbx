# DBX TypeScript Client

A comprehensive TypeScript client for the DBX Redis API, supporting both HTTP and WebSocket connections.

## Features

- **Full API Coverage**: Supports all Redis operations available in the Rust API
- **Type Safety**: Complete TypeScript definitions for all operations
- **WebSocket Support**: Real-time operations via WebSocket connections
- **Modular Design**: Separate clients for different Redis data types
- **Batch Operations**: Efficient batch operations for multiple keys
- **Error Handling**: Comprehensive error handling and type safety

## Installation

```bash
npm install dbx-sdk
# or
yarn add dbx-sdk
# or
pnpm add dbx-sdk
```

## Quick Start

```typescript
import { DbxClient } from "dbx-sdk";

// Create client
const client = new DbxClient({
  baseUrl: "http://localhost:8080",
  timeout: 5000,
});

// String operations
await client.string.set("key", "value", 3600); // with TTL
const value = await client.string.get("key");
const info = await client.string.info("key");

// Hash operations
await client.hash.setField("user:1", "name", "John");
const name = await client.hash.getField("user:1", "name");
const allFields = await client.hash.getAll("user:1");

// Set operations
await client.set.addMember("tags", "redis");
const members = await client.set.getMembers("tags");
const exists = await client.set.memberExists("tags", "redis");

// Admin operations
const health = await client.admin.health();
const info = await client.admin.info();
const stats = await client.admin.memoryStats();
```

## WebSocket Usage

```typescript
// Create WebSocket client
const wsClient = client.createWebSocket({
  url: "ws://localhost:8080/redis/string/ws",
  onMessage: (message) => {
    console.log("Received:", message);
  },
  onError: (error) => {
    console.error("WebSocket error:", error);
  },
});

// Connect and send messages
await wsClient.connect();

// String operations via WebSocket
wsClient.sendMessage({ type: "get", key: "mykey" });
wsClient.sendMessage({ type: "set", key: "mykey", value: "myvalue", ttl: 3600 });

// Hash operations via WebSocket
wsClient.sendMessage({ type: "get", key: "user:1", field: "name" });
wsClient.sendMessage({ type: "set", key: "user:1", field: "name", value: "John" });

// Set operations via WebSocket
wsClient.sendMessage({ type: "add", key: "tags", member: "redis" });
wsClient.sendMessage({ type: "members", key: "tags" });

// Admin operations via WebSocket
wsClient.sendMessage({ type: "ping" });
wsClient.sendMessage({ type: "health" });
wsClient.sendMessage({ type: "info" });
```

## API Reference

### String Operations

```typescript
// Basic operations
await client.string.get(key: string): Promise<string | null>
await client.string.set(key: string, value: string, ttl?: number): Promise<void>
await client.string.delete(key: string): Promise<boolean>
await client.string.info(key: string): Promise<StringInfo | null>

// Batch operations
await client.string.batchGet(keys: string[]): Promise<(string | null)[]>
await client.string.batchSet(operations: StringOperation[]): Promise<void>
```

### Hash Operations

```typescript
// Field operations
await client.hash.getField(key: string, field: string): Promise<string | null>
await client.hash.setField(key: string, field: string, value: string): Promise<boolean>
await client.hash.deleteField(key: string, field: string): Promise<boolean>
await client.hash.fieldExists(key: string, field: string): Promise<boolean>

// Increment operations
await client.hash.incrementField(key: string, field: string, increment: number): Promise<number>
await client.hash.incrementFieldFloat(key: string, field: string, increment: number): Promise<number>

// Hash operations
await client.hash.getAll(key: string): Promise<Record<string, string>>
await client.hash.getFields(key: string, fields: string[]): Promise<(string | null)[]>
await client.hash.setMultiple(key: string, fields: Record<string, string>): Promise<void>
await client.hash.getLength(key: string): Promise<number>
await client.hash.getKeys(key: string): Promise<string[]>
await client.hash.getValues(key: string): Promise<string[]>

// Random operations
await client.hash.getRandomField(key: string): Promise<string | null>
await client.hash.getRandomFields(key: string, count: number): Promise<string[]>
await client.hash.getRandomFieldsWithValues(key: string, count: number): Promise<Array<[string, string]>>

// Hash management
await client.hash.delete(key: string): Promise<boolean>
await client.hash.exists(key: string): Promise<boolean>
await client.hash.getTtl(key: string): Promise<number>
await client.hash.setTtl(key: string, ttl: number): Promise<boolean>

// Batch operations
await client.hash.batchGetFields(hashFields: Array<[string, string]>): Promise<(string | null)[]>
await client.hash.batchSetFields(hashOperations: Array<[string, Array<[string, string]>]>): Promise<boolean[]>
await client.hash.batchDeleteFields(hashFields: Array<[string, string[]]>): Promise<number[]>
await client.hash.batchCheckFields(hashFields: Array<[string, string]>): Promise<boolean[]>
await client.hash.batchGetLengths(keys: string[]): Promise<number[]>
```

### Set Operations

```typescript
// Member operations
await client.set.addMember(key: string, member: string): Promise<number>
await client.set.removeMember(key: string, member: string): Promise<number>
await client.set.getMembers(key: string): Promise<string[]>
await client.set.memberExists(key: string, member: string): Promise<boolean>
await client.set.getCardinality(key: string): Promise<number>

// Set operations
await client.set.intersect(keys: string[]): Promise<string[]>
await client.set.union(keys: string[]): Promise<string[]>
await client.set.difference(keys: string[]): Promise<string[]>
```

### Admin Operations

```typescript
// Health and status
await client.admin.ping(): Promise<string>
await client.admin.info(section?: string): Promise<string>
await client.admin.dbSize(): Promise<number>
await client.admin.time(): Promise<[number, number]>
await client.admin.version(): Promise<string>
await client.admin.health(): Promise<HealthCheck>
await client.admin.status(): Promise<ServerStatus>

// Statistics
await client.admin.memoryStats(): Promise<Record<string, string>>
await client.admin.clientStats(): Promise<Record<string, string>>
await client.admin.serverStats(): Promise<Record<string, string>>

// Configuration
await client.admin.configSet(parameter: string, value: string): Promise<void>
await client.admin.configGet(parameter: string): Promise<string>
await client.admin.configGetAll(): Promise<Record<string, string>>
await client.admin.configResetStat(): Promise<void>
await client.admin.configRewrite(): Promise<void>

// Database management
await client.admin.flushDb(): Promise<void>
await client.admin.flushAll(): Promise<void>
```

## WebSocket Message Types

### String WebSocket Messages

```typescript
// Request messages
{ type: 'get'; key: string }
{ type: 'set'; key: string; value: string; ttl?: number }
{ type: 'del'; key: string }
{ type: 'info'; key: string }
{ type: 'batch_get'; keys: string[] }
{ type: 'batch_set'; operations: Array<{ key: string; value: string; ttl?: number }> }

// Response messages
{ type: 'result'; key: string; value?: string }
{ type: 'batch_result'; keys: string[]; values: (string | null)[] }
{ type: 'info_result'; info?: { ttl: number; type: string } }
{ type: 'deleted'; key: string; deleted: boolean }
{ type: 'error'; data: string }
{ type: 'ping' | 'pong' }
```

### Hash WebSocket Messages

```typescript
// Request messages
{ type: 'get'; key: string; field: string }
{ type: 'set'; key: string; field: string; value: string }
{ type: 'get_all'; key: string }
{ type: 'del'; key: string; field: string }
{ type: 'exists'; key: string; field: string }
{ type: 'batch_set'; key: string; fields: Array<[string, string]> }

// Response messages
{ type: 'result'; key: string; field?: string; value?: string }
{ type: 'all_result'; key: string; fields: Record<string, string> }
{ type: 'deleted'; key: string; field: string; deleted: boolean }
{ type: 'error'; data: string }
{ type: 'ping' | 'pong' }
```

### Set WebSocket Messages

```typescript
// Request messages
{ type: 'add'; key: string; member: string }
{ type: 'remove'; key: string; member: string }
{ type: 'members'; key: string }
{ type: 'exists'; key: string; member: string }
{ type: 'cardinality'; key: string }
{ type: 'intersect'; keys: string[] }
{ type: 'union'; keys: string[] }
{ type: 'difference'; keys: string[] }

// Response messages
{ type: 'result'; key: string; value?: any }
{ type: 'error'; data: string }
{ type: 'ping' | 'pong' }
```

### Admin WebSocket Messages

```typescript
// Request messages
{ type: 'ping' }
{ type: 'info'; section?: string }
{ type: 'dbsize' }
{ type: 'time' }
{ type: 'version' }
{ type: 'health' }
{ type: 'status' }
{ type: 'memory_stats' }
{ type: 'client_stats' }
{ type: 'server_stats' }
{ type: 'config_set'; parameter: string; value: string }
{ type: 'config_get'; parameter: string }
{ type: 'config_get_all' }
{ type: 'config_resetstat' }
{ type: 'config_rewrite' }
{ type: 'flushdb' }
{ type: 'flushall' }

// Response messages
{ type: 'ping_result'; response: string }
{ type: 'info_result'; info: string }
{ type: 'dbsize_result'; size: number }
{ type: 'time_result'; seconds: number; microseconds: number }
{ type: 'version_result'; version: string }
{ type: 'health_result'; health: HealthCheck }
{ type: 'status_result'; status: ServerStatus }
{ type: 'memory_stats_result'; stats: Record<string, string> }
{ type: 'client_stats_result'; stats: Record<string, string> }
{ type: 'server_stats_result'; stats: Record<string, string> }
{ type: 'config_get_result'; parameter: string; value: string }
{ type: 'config_get_all_result'; config: Record<string, string> }
{ type: 'config_set_result'; parameter: string; value: string }
{ type: 'config_resetstat_result' }
{ type: 'config_rewrite_result' }
{ type: 'flushdb_result' }
{ type: 'flushall_result' }
{ type: 'error'; data: string }
```

## Error Handling

The client provides comprehensive error handling:

```typescript
try {
  const value = await client.string.get("nonexistent");
  console.log(value); // null
} catch (error) {
  console.error("Error:", error);
}

// WebSocket error handling
const wsClient = client.createWebSocket({
  url: "ws://localhost:8080/redis/string/ws",
  onError: (error) => {
    console.error("WebSocket error:", error);
  },
});
```

## Configuration

```typescript
interface DbxConfig {
  baseUrl: string;
  timeout?: number;
  headers?: Record<string, string>;
}

interface WebSocketConfig {
  url: string;
  onOpen?: (event: Event) => void;
  onMessage?: (message: any) => void;
  onError?: (error: Event) => void;
  onClose?: (event: Event) => void;
}
```

## Examples

### Batch Operations

```typescript
// Batch get multiple keys
const values = await client.string.batchGet(["key1", "key2", "key3"]);

// Batch set multiple hash fields
await client.hash.batchSetFields([
  [
    "user:1",
    [
      ["name", "John"],
      ["email", "john@example.com"],
    ],
  ],
  [
    "user:2",
    [
      ["name", "Jane"],
      ["email", "jane@example.com"],
    ],
  ],
]);

// Batch check hash fields
const exists = await client.hash.batchCheckFields([
  ["user:1", "name"],
  ["user:1", "email"],
  ["user:2", "name"],
]);
```

### Set Operations

```typescript
// Add members to sets
await client.set.addMember("tags", "redis");
await client.set.addMember("tags", "typescript");
await client.set.addMember("tags", "api");

// Get intersection of multiple sets
const commonTags = await client.set.intersect(["tags", "popular_tags", "trending_tags"]);

// Get union of multiple sets
const allTags = await client.set.union(["tags", "popular_tags", "trending_tags"]);
```

### Real-time Monitoring

```typescript
const adminWs = client.createWebSocket({
  url: "ws://localhost:8080/redis/admin/ws",
  onMessage: (message) => {
    if (message.type === "health_result") {
      console.log("Health status:", message.health);
    } else if (message.type === "memory_stats_result") {
      console.log("Memory usage:", message.stats);
    }
  },
});

await adminWs.connect();

// Monitor health every 30 seconds
setInterval(() => {
  adminWs.sendMessage({ type: "health" });
  adminWs.sendMessage({ type: "memory_stats" });
}, 30000);
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
