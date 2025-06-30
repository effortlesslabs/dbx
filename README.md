# DBX

Lightweight API Proxy for Edge & Embedded Systems.

<p>
  <a href="https://www.npmjs.com/package/@0dbx/redis">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/npm/v/@0dbx/redis?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/npm/v/@0dbx/redis?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="Version">
    </picture>
  </a>
  <a href="https://hub.docker.com/r/effortlesslabs/dbx">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/v/effortlesslabs/dbx?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/docker/v/effortlesslabs/dbx?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="Docker Version">
    </picture>
  </a>
  <a href="LICENSE">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/license-MIT-blue.svg?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/badge/license-MIT-blue.svg?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="MIT License">
    </picture>
  </a>
</p>

DBX is a minimal and portable HTTP/WebSocket proxy that exposes Redis through a unified API layer. Built in Rust, DBX is optimized for edge runtimes like Cloudflare Workers, Raspberry Pi, and RISC-V boards. It enables fast, standardized access to Redis using REST and WebSocket, with language bindings (TypeScript, etc.) and pluggable backend support. Perfect for lightweight clients, embedded apps, and serverless environments.

## Quick Start

### Basic Usage

```bash
# Start the server
cargo run --bin dbx-redis-api

# Or use the convenience script
./scripts/run.sh --redis-url redis://localhost:6379
```

## Features

- **🚀 Lightweight**: Minimal footprint, perfect for edge computing
- **🔌 Redis-Focused**: Optimized Redis operations with connection pooling
- **🌐 Dual Interface**: HTTP REST API + WebSocket real-time updates
- **📱 TypeScript SDK**: Full client library with type safety via NAPI bindings
- **⚡ High Performance**: Built in Rust for maximum efficiency
- **🔧 Pluggable**: Easy to extend with new database backends
- **Redis Operations**: Full support for Redis string, hash, set, and admin operations
- **REST API**: HTTP endpoints for all Redis operations
- **WebSocket Support**: Real-time operations via WebSocket connections
- **Batch Operations**: Efficient batch processing for multiple keys
- **Pattern-based Operations**: Support for wildcard patterns in batch operations
- **Docker Support**: Easy deployment with Docker and Docker Compose

## TypeScript SDK

```bash
npm install @0dbx/redis
# or
yarn add @0dbx/redis
# or
pnpm add @0dbx/redis
```

```typescript
import { DbxRedisClient } from "@0dbx/redis";

// Create client
const client = new DbxRedisClient("http://localhost:3000");

// String operations
await client.string.set("my-key", "hello world", 3600); // with TTL
const value = await client.string.get("my-key");
console.log(value); // "hello world"

// Set operations
await client.set.addMember("tags", "redis");
const members = await client.set.getMembers("tags");

// WebSocket client
import { DbxWsClient } from "@0dbx/redis";
const wsClient = new DbxWsClient("ws://localhost:3000/redis_ws");
await wsClient.string.set("my-key", "hello world");
```

## Use Cases

- **Edge Computing**: Deploy on Cloudflare Workers, Vercel Edge Functions
- **IoT Devices**: Raspberry Pi, Arduino, RISC-V boards
- **Serverless**: AWS Lambda, Google Cloud Functions
- **Embedded Systems**: Resource-constrained environments
- **Microservices**: Lightweight database access layer

## Development

```bash
# Clone and build
git clone https://github.com/effortlesslabs/dbx.git
cd dbx
cargo build --release

# Run locally
cargo run --bin dbx-redis-api -- --redis-url redis://localhost:6379

# Run tests
cargo test
```

## Docker

```bash
# Build image
docker build -t effortlesslabs/dbx .

# Run with custom config
docker run -d --name dbx -p 8080:3000 \
  -e REDIS_URL=redis://user:pass@redis.com:6379 \
  -e PORT=3000 \
  -e LOG_LEVEL=DEBUG \
  effortlesslabs/dbx:latest
```

## Deployment

### Railway Deployment

For Railway deployment, use the AMD64-only Docker image tags to avoid "exec format error" issues:

```bash
# Use AMD64-only tag for Railway
effortlesslabs/dbx:latest-amd64-only
effortlesslabs/dbx:0.1.6-amd64-only
```

# Multi-arch (AMD64 + ARM64)

effortlesslabs/dbx:latest
effortlesslabs/dbx:0.1.6

### Docker Compose

```yaml
version: "3.8"
services:
  dbx-api:
    image: effortlesslabs/dbx:latest
    ports:
      - "3000:3000"
    environment:
      - REDIS_URL=redis://redis:6379
      - PORT=3000
      - LOG_LEVEL=INFO
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
```

## API Endpoints

### REST API

- `GET /redis/string/{key}` - Get string value
- `POST /redis/string/{key}` - Set string value
- `DELETE /redis/string/{key}` - Delete string value
- `GET /redis/hash/{key}/field/{field}` - Get hash field
- `POST /redis/hash/{key}/field/{field}` - Set hash field
- `GET /redis/set/{key}/members` - Get set members
- `POST /redis/set/{key}/members` - Add set members
- `GET /redis/admin/health` - Health check
- `GET /redis/admin/ping` - Ping server

### WebSocket API

- `ws://localhost:3000/redis_ws/string/ws` - String operations
- `ws://localhost:3000/redis_ws/hash/ws` - Hash operations
- `ws://localhost:3000/redis_ws/set/ws` - Set operations
- `ws://localhost:3000/redis_ws/admin/ws` - Admin operations

## Publishing

To publish new versions of DBX (Docker image and TypeScript SDK), see our comprehensive [Publishing Guide](PUBLISHING.md).

### Quick Publish

```bash
# Interactive publishing
./scripts/quick-publish.sh

# Manual publishing
./scripts/publish-release.sh --version 1.0.0 \
  --docker-username effortlesslabs \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN
```

### GitHub Actions

The easiest way to publish is using GitHub Actions:

1. Create a git tag: `git tag v1.0.0 && git push origin v1.0.0`
2. Or manually trigger the workflow from GitHub Actions

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these packages by you, as defined in the MIT license, shall be
licensed as above, without any additional terms or conditions.
</sub>

---

**🔗 Docker Hub**: [https://hub.docker.com/r/effortlesslabs/dbx](https://hub.docker.com/r/effortlesslabs/dbx)
