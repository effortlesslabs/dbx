# DBX

Lightweight API Proxy for Edge & Embedded Systems.

<p>
  <a href="https://www.npmjs.com/package/dbx-sdk">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/npm/v/dbx-sdk?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/npm/v/dbx-sdk?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="Version">
    </picture>
  </a>
  <a href="https://hub.docker.com/r/fnlog0/dbx">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/docker/v/fnlog0/dbx?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/docker/v/fnlog0/dbx?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="Docker Version">
    </picture>
  </a>
  <a href="LICENSE">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/license-MIT-blue.svg?colorA=21262d&colorB=21262d&style=flat">
      <img src="https://img.shields.io/badge/license-MIT-blue.svg?colorA=f6f8fa&colorB=f6f8fa&style=flat" alt="MIT License">
    </picture>
  </a>
</p>

DBX is a minimal and portable HTTP/WebSocket proxy that exposes Redis, Qdrant, and MDBX through a unified API layer. Built in Rust, DBX is optimized for edge runtimes like Cloudflare Workers, Raspberry Pi, and RISC-V boards. It enables fast, standardized access to multiple databases using REST and WebSocket, with language bindings (TypeScript, etc.) and pluggable backend support. Perfect for lightweight clients, embedded apps, and serverless environments.

## Quick Start

```bash
# Run with Docker
docker run -d --name dbx -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest

# Or use the convenience script
./scripts/run.sh --redis-url redis://localhost:6379
```

## Features

- **🚀 Lightweight**: Minimal footprint, perfect for edge computing
- **🔌 Multi-Database**: Support for Redis, Qdrant, and MDBX
- **🌐 Dual Interface**: HTTP REST API + WebSocket real-time updates
- **📱 TypeScript SDK**: Full client library with type safety
- **⚡ High Performance**: Built in Rust for maximum efficiency
- **🔧 Pluggable**: Easy to extend with new database backends

## TypeScript SDK

```bash
npm install dbx-sdk
# or
yarn add dbx-sdk
# or
pnpm add dbx-sdk
```

```typescript
import { DbxClient } from "dbx-sdk";

// Create client
const client = new DbxClient({
  baseUrl: "http://localhost:3000",
  timeout: 5000,
});

// String operations
await client.string.set("my-key", "hello world", 3600); // with TTL
const value = await client.string.get("my-key");
console.log(value); // "hello world"

// Hash operations
await client.hash.setField("user:1", "name", "John");
const name = await client.hash.getField("user:1", "name");

// Set operations
await client.set.addMember("tags", "redis");
const members = await client.set.getMembers("tags");

// Admin operations
const health = await client.admin.health();
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
cargo run -- --redis-url redis://localhost:6379

# Run tests
cargo test
```

## Docker

```bash
# Build image
docker build -t fnlog0/dbx .

# Run with custom config
docker run -d --name dbx -p 8080:3000 \
  -e DATABASE_URL=redis://user:pass@redis.com:6379 \
  -e PORT=3000 \
  -e LOG_LEVEL=DEBUG \
  fnlog0/dbx:latest
```

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

**🔗 Docker Hub**: [https://hub.docker.com/r/fnlog0/dbx](https://hub.docker.com/r/fnlog0/dbx)
