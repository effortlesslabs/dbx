# DBX API Server

A REST API and WebSocket server for Redis operations built with Rust and Axum.

## Features

- **HTTP REST API**: Simple, one-off REST requests for database operations
- **WebSocket API**: Real-time, low-latency interface for batch commands and streaming
- **String Operations**: GET, SET, DELETE, INCR, INCRBY, EXISTS, TTL
- **Batch Operations**: Set/Get/Delete multiple keys at once
- **Advanced Operations**: SETNX, Compare-and-Set, Rate Limiting
- **Lua Scripts**: Pre-built scripts for complex operations
- **Health Checks**: Server and Redis connection monitoring
- **CORS Support**: Cross-origin resource sharing enabled
- **Comprehensive Error Handling**: Proper HTTP status codes and error messages

## Quick Start

### Prerequisites

- Rust 1.70+
- Redis server running (default: `redis://127.0.0.1:6379`)

### Installation

```bash
# Clone the repository
git clone https://github.com/effortlesslabs/dbx.git
cd dbx

# Build the API server
cargo build --bin dbx-api

# Run the server
cargo run --bin dbx-api
```

### Configuration

The server can be configured via command-line arguments or environment variables:

```bash
# Command-line arguments
cargo run --bin dbx-api -- --redis-url redis://localhost:6379 --port 3000

# Environment variables
export REDIS_URL=redis://localhost:6379
export PORT=3000
export HOST=127.0.0.1
export POOL_SIZE=10
cargo run --bin dbx-api
```

## API Endpoints

### Health & Info

#### GET /health

Check server and Redis connection status.

```bash
curl http://localhost:3000/health
```

Response:

```json
{
  "success": true,
  "data": {
    "status": "ok",
    "redis_connected": true,
    "timestamp": "2024-01-01T12:00:00Z"
  }
}
```

#### GET /info

Get server information.

```bash
curl http://localhost:3000/info
```

### String Operations

#### GET /api/v1/redis/strings/:key

Get a string value.

```bash
curl http://localhost:3000/api/v1/redis/strings/mykey
```

#### POST /api/v1/redis/strings/:key

Set a string value.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/mykey \
  -H "Content-Type: application/json" \
  -d '{"value": "hello world", "ttl": 3600}'
```

#### DELETE /api/v1/redis/strings/:key

Delete a string key.

```bash
curl -X DELETE http://localhost:3000/api/v1/redis/strings/mykey
```

#### GET /api/v1/redis/strings/:key/exists

Check if a key exists.

```bash
curl http://localhost:3000/api/v1/redis/strings/mykey/exists
```

#### GET /api/v1/redis/strings/:key/ttl

Get the TTL of a key.

```bash
curl http://localhost:3000/api/v1/redis/strings/mykey/ttl
```

#### POST /api/v1/redis/strings/:key/incr

Increment a numeric value.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/counter/incr
```

#### POST /api/v1/redis/strings/:key/incrby

Increment a numeric value by a specific amount.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/counter/incrby \
  -H "Content-Type: application/json" \
  -d '{"increment": 5}'
```

#### POST /api/v1/redis/strings/:key/setnx

Set a value only if the key doesn't exist.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/mykey/setnx \
  -H "Content-Type: application/json" \
  -d '{"value": "unique value", "ttl": 3600}'
```

#### POST /api/v1/redis/strings/:key/cas

Compare and set a value atomically.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/mykey/cas \
  -H "Content-Type: application/json" \
  -d '{"expected_value": "old value", "new_value": "new value", "ttl": 3600}'
```

### Batch Operations

#### POST /api/v1/redis/strings/batch/set

Set multiple keys at once.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/batch/set \
  -H "Content-Type: application/json" \
  -d '{
    "key_values": {
      "key1": "value1",
      "key2": "value2",
      "key3": "value3"
    },
    "ttl": 3600
  }'
```

#### POST /api/v1/redis/strings/batch/get

Get multiple keys at once.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/batch/get \
  -H "Content-Type: application/json" \
  -d '["key1", "key2", "key3"]'
```

#### POST /api/v1/redis/strings/batch/delete

Delete multiple keys at once.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/batch/delete \
  -H "Content-Type: application/json" \
  -d '["key1", "key2", "key3"]'
```

#### POST /api/v1/redis/strings/batch/incr

Increment multiple counters at once.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/batch/incr \
  -H "Content-Type: application/json" \
  -d '["counter1", "counter2", "counter3"]'
```

#### POST /api/v1/redis/strings/batch/incrby

Increment multiple counters by specific amounts.

```bash
curl -X POST http://localhost:3000/api/v1/redis/strings/batch/incrby \
  -H "Content-Type: application/json" \
  -d '[["counter1", 5], ["counter2", 10], ["counter3", 15]]'
```

### Key Operations

#### GET /api/v1/redis/keys

List keys matching a pattern.

```bash
# List all keys
curl http://localhost:3000/api/v1/redis/keys

# List keys matching a pattern
curl "http://localhost:3000/api/v1/redis/keys?pattern=user:*"
```

#### DELETE /api/v1/redis/keys/:key

Delete a key.

```bash
curl -X DELETE http://localhost:3000/api/v1/redis/keys/mykey
```

#### GET /api/v1/redis/keys/:key/exists

Check if a key exists.

```bash
curl http://localhost:3000/api/v1/redis/keys/mykey/exists
```

#### GET /api/v1/redis/keys/:key/ttl

Get the TTL of a key.

```bash
curl http://localhost:3000/api/v1/redis/keys/mykey/ttl
```

### Lua Script Operations

#### POST /api/v1/redis/scripts/rate-limiter

Implement rate limiting.

```bash
curl -X POST http://localhost:3000/api/v1/redis/scripts/rate-limiter \
  -H "Content-Type: application/json" \
  -d '{
    "key": "rate_limit:user:123",
    "limit": 10,
    "window": 60
  }'
```

#### POST /api/v1/redis/scripts/multi-counter

Increment multiple counters atomically.

```bash
curl -X POST http://localhost:3000/api/v1/redis/scripts/multi-counter \
  -H "Content-Type: application/json" \
  -d '{
    "counters": [
      ["counter1", 5],
      ["counter2", 10],
      ["counter3", 15]
    ]
  }'
```

#### POST /api/v1/redis/scripts/multi-set-ttl

Set multiple keys with TTL atomically.

```bash
curl -X POST http://localhost:3000/api/v1/redis/scripts/multi-set-ttl \
  -H "Content-Type: application/json" \
  -d '{
    "key_values": {
      "key1": "value1",
      "key2": "value2",
      "key3": "value3"
    },
    "ttl": 3600
  }'
```

## WebSocket API

The WebSocket API provides a real-time, low-latency interface for database operations. It's ideal for:

- **Batch Commands**: Execute multiple operations efficiently
- **Low-Latency Operations**: Minimal overhead for frequent requests
- **Real-time Streaming**: Subscribe to database changes (coming soon)
- **Persistent Connections**: Maintain connection state across requests

### WebSocket Endpoint

```
ws://localhost:3000/ws
```

### Message Format

All WebSocket messages use JSON format with the following structure:

```json
{
  "id": "optional-request-id",
  "command": {
    "action": "command_name",
    "params": {
      // command-specific parameters
    }
  }
}
```

### Supported Commands

#### Basic Operations

**Get Value**

```json
{
  "id": "get-1",
  "command": {
    "action": "get",
    "params": {
      "key": "mykey"
    }
  }
}
```

**Set Value**

```json
{
  "id": "set-1",
  "command": {
    "action": "set",
    "params": {
      "key": "mykey",
      "value": "myvalue",
      "ttl": 3600
    }
  }
}
```

**Delete Key**

```json
{
  "id": "delete-1",
  "command": {
    "action": "delete",
    "params": {
      "key": "mykey"
    }
  }
}
```

**Check Exists**

```json
{
  "id": "exists-1",
  "command": {
    "action": "exists",
    "params": {
      "key": "mykey"
    }
  }
}
```

**Get TTL**

```json
{
  "id": "ttl-1",
  "command": {
    "action": "ttl",
    "params": {
      "key": "mykey"
    }
  }
}
```

#### Numeric Operations

**Increment**

```json
{
  "id": "incr-1",
  "command": {
    "action": "incr",
    "params": {
      "key": "counter"
    }
  }
}
```

**Increment By**

```json
{
  "id": "incrby-1",
  "command": {
    "action": "incrby",
    "params": {
      "key": "counter",
      "increment": 5
    }
  }
}
```

#### Advanced Operations

**Set If Not Exists**

```json
{
  "id": "setnx-1",
  "command": {
    "action": "setnx",
    "params": {
      "key": "unique_key",
      "value": "unique_value",
      "ttl": 3600
    }
  }
}
```

**Compare and Set**

```json
{
  "id": "cas-1",
  "command": {
    "action": "cas",
    "params": {
      "key": "mykey",
      "expected_value": "old_value",
      "new_value": "new_value",
      "ttl": 3600
    }
  }
}
```

#### Batch Operations

**Batch Get**

```json
{
  "id": "batch-get-1",
  "command": {
    "action": "batch_get",
    "params": {
      "keys": ["key1", "key2", "key3"]
    }
  }
}
```

**Batch Set**

```json
{
  "id": "batch-set-1",
  "command": {
    "action": "batch_set",
    "params": {
      "key_values": {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3"
      },
      "ttl": 1800
    }
  }
}
```

**Batch Delete**

```json
{
  "id": "batch-delete-1",
  "command": {
    "action": "batch_delete",
    "params": {
      "keys": ["key1", "key2", "key3"]
    }
  }
}
```

**Batch Increment**

```json
{
  "id": "batch-incr-1",
  "command": {
    "action": "batch_incr",
    "params": {
      "keys": ["counter1", "counter2", "counter3"]
    }
  }
}
```

**Batch Increment By**

```json
{
  "id": "batch-incrby-1",
  "command": {
    "action": "batch_incrby",
    "params": {
      "key_increments": [
        ["counter1", 1],
        ["counter2", 5],
        ["counter3", 10]
      ]
    }
  }
}
```

#### Utility Commands

**Ping**

```json
{
  "id": "ping-1",
  "command": {
    "action": "ping"
  }
}
```

### Response Format

All WebSocket responses follow this format:

```json
{
  "id": "request-id",
  "success": true,
  "data": {
    // response data
  },
  "error": null,
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### Example Usage

#### JavaScript Client

```javascript
const ws = new WebSocket("ws://localhost:3000/ws");

ws.onopen = function () {
  console.log("Connected to DBX WebSocket API");

  // Send a set command
  ws.send(
    JSON.stringify({
      id: "set-1",
      command: {
        action: "set",
        params: {
          key: "test_key",
          value: "test_value",
          ttl: 3600,
        },
      },
    })
  );
};

ws.onmessage = function (event) {
  const response = JSON.parse(event.data);
  console.log("Received:", response);

  if (response.id === "set-1" && response.success) {
    console.log("Value set successfully:", response.data);
  }
};
```

#### Rust Client Example

Run the included WebSocket client example:

```bash
cargo run --example websocket_client
```

This example demonstrates:

- Connecting to the WebSocket API
- Sending various commands (ping, set, get, batch operations)
- Receiving and processing responses

### Error Handling

WebSocket responses include error information when operations fail:

```json
{
  "id": "get-1",
  "success": false,
  "data": null,
  "error": "Key not found",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

Common error scenarios:

- **Key not found**: When trying to get a non-existent key
- **Invalid JSON**: When message format is incorrect
- **Redis errors**: When underlying Redis operations fail
- **Unsupported commands**: When using commands not yet implemented

### Connection Management

- **Automatic Reconnection**: Clients should implement reconnection logic
- **Heartbeat**: Use ping/pong for connection health monitoring
- **Graceful Shutdown**: Server sends close frames on shutdown
- **Connection Limits**: Consider implementing connection limits for production

## Error Handling

The API returns consistent error responses:

```json
{
  "success": false,
  "error": "Error message describing what went wrong"
}
```

Common HTTP status codes:

- `200 OK`: Operation successful
- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Key not found
- `409 Conflict`: Operation conflict (e.g., compare-and-set failed)
- `500 Internal Server Error`: Server error
- `503 Service Unavailable`: Redis connection issues

## Development

### Running Tests

```bash
cargo test --bin dbx-api
```

### Running with Docker

```bash
# Build the Docker image
docker build -t dbx-api .

# Run the container
docker run -p 3000:3000 -e REDIS_URL=redis://host.docker.internal:6379 dbx-api
```

### Logging

The server uses structured logging with different levels:

```bash
# Set log level
cargo run --bin dbx-api -- --log-level debug
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT OR Apache-2.0
