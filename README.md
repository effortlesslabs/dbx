<img src="banner.png" alt="DBX Banner" width="100%">

# DBX 🧩

**A lightweight HTTP/WebSocket API proxy for all types of databases.**  
Built in Rust. Deploy anywhere — Cloudflare Workers, Raspberry Pi, RISC-V boards, or embedded systems.

---

## ✨ Features

- ⚙️ **Unified API Layer** – Interact with Redis, PostgreSQL, MongoDB, and libmdbx using one consistent interface.
- 🌐 **Supports HTTP + WebSocket** – Low-latency communication, real-time streaming, and bidirectional data flow.
- 🧠 **Pluggable Backend** – Easily add support for new databases via traits and modular architecture.
- 🚀 **Portable Runtime** – Built to run on Cloudflare Workers, edge devices, or local machines.
- 📦 **Bindings for TypeScript** – Ready-to-use client SDKs with full type safety.
- 🔄 **Multi-Database Support** – Switch between different database engines seamlessly.
- ⚡ **High Performance** – Optimized for low-latency operations and high-throughput workloads.
- 🛡️ **Type Safety** – Strong typing across all database operations and responses.
- 🔌 **Real-time Sync** – WebSocket-based real-time data synchronization and updates.
- 📊 **Query Optimization** – Intelligent query routing and caching strategies.

---

## 📚 Supported Databases

| Database   | Status     | Features                                |
| ---------- | ---------- | --------------------------------------- |
| Redis      | ✅ Full    | Strings, Hashes, Lists, Streams, PubSub |
| PostgreSQL | 🔜 WIP     | SQL queries, transactions, JSON support |
| MongoDB    | 🔜 WIP     | Document storage, aggregation, indexing |
| libmdbx    | 🔜 Planned | Embedded, transactional key-value store |

---

## Quick Start

### Docker (Recommended)

```bash
# Setup and start
./scripts/docker-setup.sh

# Or manually
docker-compose up -d
```

### Local Development

```bash
# Clone and setup
git clone <repository-url>
cd dbx
cp api/env.example api/.env

# Run
cargo run --bin dbx-api
```

## API Usage

### HTTP API

**Set/Get Key-Value:**

```bash
# Set
curl -X POST http://localhost:3000/api/redis/set \
  -H "Content-Type: application/json" \
  -d '{"key": "user:123", "value": "John Doe"}'

# Get
curl http://localhost:3000/api/redis/get/user:123
```

**Health Check:**

```bash
curl http://localhost:3000/health
```

### WebSocket API

```javascript
const ws = new WebSocket("ws://localhost:3000/ws");

ws.send(
  JSON.stringify({
    id: "1",
    command: "SET",
    args: { key: "user:123", value: "John Doe" },
  })
);
```

## Configuration

| Variable    | Default                  | Description          |
| ----------- | ------------------------ | -------------------- |
| `REDIS_URL` | `redis://localhost:6379` | Redis connection URL |
| `HOST`      | `127.0.0.1`              | Server host          |
| `PORT`      | `3000`                   | Server port          |

## Development

```bash
# Tests
cargo test

# Format & lint
cargo fmt
cargo clippy
```

## Project Structure

```
dbx/
├── api/          # Rust API server
├── crates/       # Shared libraries
├── ts/           # TypeScript client
└── scripts/      # Setup scripts
```

## License

MIT License
