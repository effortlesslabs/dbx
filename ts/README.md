# DBX TypeScript SDK

A TypeScript SDK for DBX - A minimal API layer for databases (fetch-based).

## Installation

```bash
npm install @dbx/sdk
```

## Configuration

The SDK supports environment variables for configuration. Create a `.env` file in your project root:

```env
# API Configuration
HOST_URL=http://127.0.0.1:3000
WS_HOST_URL=ws://127.0.0.1:3000/redis_ws
REDIS_URL=redis://127.0.0.1:6379
```

## Basic Usage

```typescript
import { createDbxClient } from "@dbx/sdk";

// Create client using environment variables
const client = createDbxClient();

// Or specify a custom base URL
const client = createDbxClient("http://localhost:8080");

// Health check
const health = await client.health();
console.log("Health:", health);

// Set a value
await client.setString("key", "value", 3600); // with TTL

// Get a value
const value = await client.getString("key");
console.log("Value:", value);
```

## WebSocket Usage

```typescript
import { createDbxClient, getConfig } from "@dbx/sdk";

const config = getConfig();
const client = createDbxClient();

// Create WebSocket connection
const ws = client.createWebSocket({
  url: config.wsHostUrl, // Uses WS_HOST_URL from .env
  onOpen: () => {
    console.log("Connected!");

    // Send commands
    client.sendWebSocketCommand(
      ws,
      {
        action: "set",
        params: { key: "test", value: "Hello WebSocket!" },
      },
      "cmd1"
    );
  },
  onMessage: (response) => {
    console.log("Response:", response);
  },
  onError: (error) => {
    console.error("Error:", error);
  },
  onClose: () => {
    console.log("Disconnected");
  },
});
```

## Configuration API

```typescript
import { getConfig, getConfigWithOverrides } from "@dbx/sdk";

// Get configuration from environment variables
const config = getConfig();
console.log("Host URL:", config.hostUrl);
console.log("WebSocket URL:", config.wsHostUrl);

// Get configuration with custom overrides
const customConfig = getConfigWithOverrides({
  hostUrl: "http://custom-host:3000",
});
```

## Available Operations

### String Operations

- `getString(key)` - Get a string value
- `setString(key, value, ttl?)` - Set a string value
- `deleteString(key)` - Delete a string value
- `exists(key)` - Check if key exists
- `getTtl(key)` - Get TTL for a key
- `incr(key)` - Increment a counter
- `incrBy(key, increment)` - Increment by specific amount
- `setNx(key, value, ttl?)` - Set only if not exists
- `compareAndSet(key, expectedValue, newValue, ttl?)` - Compare and set

### Set Operations

- `getSetMembers(key)` - Get all set members
- `addSetMembers(key, members)` - Add members to set
- `deleteSet(key)` - Delete entire set
- `setMemberExists(key, member)` - Check if member exists
- `getSetCardinality(key)` - Get set size
- `getRandomSetMember(key)` - Get random member
- `popRandomSetMember(key)` - Remove and return random member
- `moveSetMember(key, member, destination)` - Move member to another set
- `setUnion(key, otherKeys)` - Get union of sets
- `setIntersection(key, otherKeys)` - Get intersection of sets
- `setDifference(key, otherKeys)` - Get difference of sets

### Hash Operations

- `getHashField(key, field)` - Get hash field value
- `setHashField(key, field, value)` - Set hash field
- `deleteHashField(key, field)` - Delete hash field
- `hashFieldExists(key, field)` - Check if field exists
- `incrementHashField(key, field, increment)` - Increment hash field
- `setHashFieldNx(key, field, value)` - Set field only if not exists
- `getHashLength(key)` - Get number of hash fields
- `getHashKeys(key)` - Get all hash field names
- `getHashValues(key)` - Get all hash field values
- `getRandomHashField(key)` - Get random hash field
- `getMultipleHashFields(key, fields)` - Get multiple fields
- `getHashAll(key)` - Get all hash fields
- `setHashMultiple(key, fields)` - Set multiple hash fields
- `deleteHash(key)` - Delete entire hash

### Batch Operations

- `batchSet(keyValues, ttl?)` - Set multiple keys
- `batchGet(keys)` - Get multiple keys
- `batchDelete(keys)` - Delete multiple keys
- `batchIncr(keys)` - Increment multiple counters
- `batchIncrBy(keyIncrements)` - Increment multiple counters by specific amounts
- `batchAddSetMembers(setMembers)` - Add members to multiple sets
- `batchRemoveSetMembers(setMembers)` - Remove members from multiple sets
- `batchGetSetMembers(keys)` - Get members from multiple sets
- `batchDeleteSets(keys)` - Delete multiple sets
- `batchSetHashFields(hashFields)` - Set fields in multiple hashes
- `batchGetHashFields(hashFields)` - Get fields from multiple hashes
- `batchDeleteHashFields(hashFields)` - Delete fields from multiple hashes
- `batchGetHashAll(keys)` - Get all fields from multiple hashes
- `batchCheckHashFields(hashFields)` - Check if fields exist in multiple hashes
- `batchGetHashLengths(keys)` - Get lengths of multiple hashes

### Key Operations

- `listKeys(pattern?)` - List keys matching pattern
- `keyExists(key)` - Check if key exists
- `keyTtl(key)` - Get TTL for key
- `deleteKey(key)` - Delete a key

### Server Operations

- `health()` - Health check
- `info()` - Server info
- `dbSize()` - Get database size
- `flushAll()` - Flush all databases
- `flushDb()` - Flush current database

## WebSocket Commands

The WebSocket API supports all the same operations as the HTTP API. Commands are sent using the `sendWebSocketCommand` method:

```typescript
client.sendWebSocketCommand(
  ws,
  {
    action: "set",
    params: { key: "test", value: "Hello!", ttl: 3600 },
  },
  "command-id"
);
```

Available actions: `get`, `set`, `delete`, `exists`, `ttl`, `incr`, `incrby`, `setnx`, `cas`, `batch_set`, `batch_get`, `batch_delete`, `batch_incr`, `batch_incrby`, `get_set_members`, `add_set_members`, `delete_set`, `set_member_exists`, `get_set_cardinality`, `get_random_set_member`, `pop_random_set_member`, `move_set_member`, `set_union`, `set_intersection`, `set_difference`, `batch_add_set_members`, `batch_remove_set_members`, `batch_get_set_members`, `batch_delete_sets`, `get_hash_field`, `set_hash_field`, `delete_hash_field`, `hash_field_exists`, `increment_hash_field`, `set_hash_field_nx`, `get_hash_length`, `get_hash_keys`, `get_hash_values`, `get_random_hash_field`, `get_multiple_hash_fields`, `get_hash_all`, `set_hash_multiple`, `delete_hash`, `batch_set_hash_fields`, `batch_get_hash_fields`, `batch_delete_hash_fields`, `batch_get_hash_all`, `batch_check_hash_fields`, `batch_get_hash_lengths`, `list_keys`, `key_exists`, `key_ttl`, `delete_key`, `ping`.

## License

MIT
