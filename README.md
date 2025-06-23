# DBX – Lightweight API Proxy for Edge & Embedded Systems

![DBX Banner](banner.png)

DBX is a minimal and portable HTTP/WebSocket proxy that exposes Redis, Qdrant, and MDBX through a unified API layer. Built in Rust, DBX is optimized for edge runtimes like Cloudflare Workers, Raspberry Pi, and RISC-V boards. It enables fast, standardized access to multiple databases using REST and WebSocket, with language bindings (TypeScript, etc.) and pluggable backend support. Perfect for lightweight clients, embedded apps, and serverless environments.

## Quick Start

**📦 Available on Docker Hub: [fnlog0/dbx](https://hub.docker.com/r/fnlog0/dbx)**

```bash
# Run with Redis
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

**📦 Available on npm: [dbx-sdk](https://www.npmjs.com/package/dbx-sdk)**

```bash
npm install dbx-sdk
# or
yarn add dbx-sdk
# or
pnpm add dbx-sdk
```

## Use Cases

- **Edge Computing**: Deploy on Cloudflare Workers, Vercel Edge Functions
- **IoT Devices**: Raspberry Pi, Arduino, RISC-V boards
- **Serverless**: AWS Lambda, Google Cloud Functions
- **Embedded Systems**: Resource-constrained environments
- **Microservices**: Lightweight database access layer

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

[Add your license here]

---

**🔗 Docker Hub**: [https://hub.docker.com/r/fnlog0/dbx](https://hub.docker.com/r/fnlog0/dbx)
