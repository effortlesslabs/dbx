# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## About DBX

DBX is a lightweight API proxy for edge and embedded systems that exposes Redis, Qdrant, and MDBX through a unified API layer. Built in Rust, DBX is optimized for edge runtimes like Cloudflare Workers, Raspberry Pi, and RISC-V boards. It enables fast, standardized access to multiple databases using REST and WebSocket, with language bindings (TypeScript, etc.) and pluggable backend support.

## Required Dependencies

- **Rust**: `>= 1.70`
- **Cargo**: `>= 1.70`
- **Docker**: `>= 20.10`
- **Node.js**: `>= 18.0`

## Commands

### Rust Backend
```bash
# Build the API server
cargo build --release

# Run the API server locally
cargo run --bin dbx-api -- --redis-url redis://localhost:6379

# Run tests
cargo test

# Run specific test suite
cargo test --package dbx-api
cargo test --package dbx-crates

# Code quality
cargo clippy
cargo fmt
```

### TypeScript SDK
```bash
# Navigate to TypeScript directory
cd ts

# Install dependencies
npm install

# Build SDK
npm run build

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Lint and format
npm run lint
npm run format
```

### Docker Operations
```bash
# Build Docker image
docker build -t fnlog0/dbx .

# Run with convenience script
./scripts/run.sh --redis-url redis://localhost:6379

# Run with custom configuration
docker run -d --name dbx -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  -e PORT=3000 \
  -e LOG_LEVEL=DEBUG \
  fnlog0/dbx:latest

# Check logs
docker logs -f dbx-api

# Stop and clean up
docker stop dbx-api && docker rm dbx-api
```

### Development Workflow
```bash
# Start development environment
./scripts/run.sh --redis-url redis://localhost:6379 --log-level DEBUG

# Test Docker build
docker build -t dbx-test .
```

## Project Structure

### Workspace Organization
- `api/` - Rust API server implementation
  - `src/main.rs` - Application entry point
  - `src/server.rs` - HTTP/WebSocket server setup
  - `src/routes/` - API route handlers (redis/, redis_ws/, common/)
  - `src/models.rs` - Data structures and types
  - `src/config.rs` - Configuration management
  - `src/middleware.rs` - HTTP middleware
  - `tests/` - Integration tests

- `crates/` - Core Rust library
  - `adapter/redis/` - Redis adapter with primitives
  - `adapter/redis/primitives/` - Core Redis operations (string, hash, set, admin)

- `ts/` - TypeScript SDK
  - `src/client.ts` - Main DBX client class
  - `src/clients/` - Specialized clients (base.ts, string.ts, hash.ts, set.ts, admin.ts, websocket.ts)
  - `src/types/` - TypeScript type definitions
  - `src/__tests__/` - Test files

- `scripts/` - Development and deployment scripts
- `docs/` - Documentation
- `.github/workflows/` - CI/CD workflows

## API Endpoints

### HTTP REST API
- **String Operations**: `/redis/string/*`
- **Hash Operations**: `/redis/hash/*`
- **Set Operations**: `/redis/set/*`
- **Admin Operations**: `/redis/admin/*`

### WebSocket API
- **String WebSocket**: `/redis_ws/string/ws`
- **Hash WebSocket**: `/redis_ws/hash/ws`
- **Set WebSocket**: `/redis_ws/set/ws`
- **Admin WebSocket**: `/redis_ws/admin/ws`

## Environment Configuration

### Required Environment Variables
```bash
DATABASE_URL=redis://localhost:6379  # Redis connection URL (required)
DATABASE_TYPE=redis                  # Database type (default: redis)
HOST=0.0.0.0                        # Server host (default: 0.0.0.0)
PORT=3000                           # Server port (default: 3000)
POOL_SIZE=10                        # Connection pool size (default: 10)
LOG_LEVEL=INFO                      # Logging level (default: INFO)
```

## Development Guidelines

### PR Requirements
- All Rust tests must pass (`cargo test`)
- All TypeScript tests must pass (`cd ts && npm test`)
- Code must be properly formatted (`cargo fmt` and `npm run format`)
- Linting must pass (`cargo clippy` and `npm run lint`)
- Build must succeed (`cargo build --release` and `npm run build`)
- Docker build must succeed (`docker build -t dbx-test .`)

### Branch Naming
Create feature branches against `main`, named `claude/feature-name`

### Testing Strategy
- **Unit Tests**: Rust tests in `api/tests/` and `crates/tests/`
- **Integration Tests**: End-to-end API tests
- **TypeScript Tests**: Vitest with coverage reporting
- **Docker Tests**: Containerized testing with Redis

### Common Development Tasks

#### Adding a New Database Adapter
1. Create new adapter in `crates/adapter/[database_name]/`
2. Implement required traits and primitives
3. Add routes in `api/src/routes/[database_name]/`
4. Add tests in `api/tests/[database_name]/`
5. Update configuration to support new database type

#### Adding New API Endpoints
1. Define request/response models in `api/src/models.rs`
2. Add route handlers in appropriate `api/src/routes/` directory
3. Add integration tests in `api/tests/`
4. Update TypeScript SDK types and clients if needed

#### Updating TypeScript SDK
1. Update types in `ts/src/types/`
2. Add client methods in `ts/src/clients/`
3. Update main client in `ts/src/client.ts`
4. Add tests in `ts/src/__tests__/`
5. Update documentation and examples

## Docker Deployment Notes

- Multi-platform builds (linux/amd64, linux/arm64)
- Railway deployment requires AMD64-only tags: `fnlog0/dbx:latest-amd64-only`
- Multi-stage builds with minimal runtime
- Health checks and non-root user execution

## Publishing

```bash
# Interactive publishing
./scripts/quick-publish.sh

# Manual publishing
./scripts/publish-release.sh --version 1.0.0 \
  --docker-username fnlog0 \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN
```

- Automated via GitHub Actions
- Docker Hub + NPM registry
- Git tag-based releases