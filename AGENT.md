# CLAUDE.md - AI Assistant Guide for DBX

This file provides essential information for Claude AI (and other AI assistants) working with the DBX codebase.

## About DBX

DBX is a lightweight API proxy for edge and embedded systems that exposes Redis, Qdrant, and MDBX through a unified API layer. Built in Rust, DBX is optimized for edge runtimes like Cloudflare Workers, Raspberry Pi, and RISC-V boards. It enables fast, standardized access to multiple databases using REST and WebSocket, with language bindings (TypeScript, etc.) and pluggable backend support.

## Helpful Links

- [DBX Repository](https://github.com/effortlesslabs/dbx)
- [Docker Hub](https://hub.docker.com/r/fnlog0/dbx)
- [NPM Package](https://www.npmjs.com/package/dbx-sdk)

## Binaries

### Required

- **[Rust](https://rustup.rs/):** `>= 1.70`
- **[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):** `>= 1.70`
- **[Docker](https://docs.docker.com/get-docker):** `>= 20.10`
- **[Node.js](https://nodejs.org/en/download/):** `>= 18.0`

### Optional: For Development

- **[pnpm](https://pnpm.io/installation):** `>= 8.0` (for TypeScript SDK)
- **[Redis](https://redis.io/download):** `>= 6.0` (for testing)

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
```

### TypeScript SDK

```bash
# Navigate to TypeScript directory
cd ts

# Install dependencies
npm install
# or
pnpm install

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
```

### Development Workflow

```bash
# Start development environment
./scripts/run.sh --redis-url redis://localhost:6379 --log-level DEBUG

# Check logs
docker logs -f dbx-api

# Stop and clean up
docker stop dbx-api && docker rm dbx-api
```

## Directory Structure

- `api/`: Rust API server implementation
  - `src/`: Source code
    - `main.rs`: Application entry point
    - `lib.rs`: Library exports
    - `server.rs`: HTTP/WebSocket server setup
    - `routes/`: API route handlers
      - `redis/`: Redis-specific endpoints
      - `redis_ws/`: WebSocket endpoints
      - `common/`: Shared endpoint logic
    - `models.rs`: Data structures and types
    - `config.rs`: Configuration management
    - `middleware.rs`: HTTP middleware
  - `tests/`: Integration tests
- `crates/`: Core Rust library
  - `adapter/`: Database adapter implementations
    - `redis/`: Redis adapter with primitives
- `ts/`: TypeScript SDK
  - `src/`: Source code
    - `client.ts`: Main client implementation
    - `clients/`: Client implementations for different operations
    - `types/`: TypeScript type definitions
    - `config.ts`: Configuration types
  - `src/__tests__/`: Test files
- `scripts/`: Development and deployment scripts
  - `run.sh`: Docker run script with configuration
  - `publish.sh`: Publishing script
- `docs/`: Documentation
- `static/`: Static assets
- `.github/workflows/`: CI/CD workflows

## Project Overview

### Rust Backend (`api/`)

#### Core Dependencies

- **`axum`** - Web framework for Rust
- **`tokio`** - Async runtime
- **`redis`** - Redis client for Rust
- **`serde`** - Serialization framework
- **`tracing`** - Application tracing

#### Key Modules

- **`server.rs`** - HTTP/WebSocket server setup and routing
- **`routes/`** - API endpoint handlers organized by database type
- **`models.rs`** - Request/response data structures
- **`config.rs`** - Configuration management and environment variables
- **`middleware.rs`** - CORS, logging, and other HTTP middleware

#### Database Adapters (`crates/adapter/`)

- **`redis/`** - Redis adapter implementation
  - **`primitives/`** - Core Redis operations (string, hash, set, admin)
  - **`client.rs`** - Redis client wrapper

### TypeScript SDK (`ts/`)

#### Core Dependencies

- **`axios`** - HTTP client for API requests
- **`dotenv`** - Environment variable management

#### Key Modules

- **`client.ts`** - Main DBX client class
- **`clients/`** - Specialized clients for different operations
  - **`base.ts`** - Base client functionality
  - **`string.ts`** - String operations
  - **`hash.ts`** - Hash operations
  - **`set.ts`** - Set operations
  - **`admin.ts`** - Administrative operations
  - **`websocket.ts`** - WebSocket client
- **`types/`** - TypeScript type definitions
- **`config.ts`** - Client configuration types

## Development Workflow

### Making Changes

1. **Create a feature branch**: Against `main`, named `claude/feature-name`
2. **Develop features**:
   - For Rust changes: Ensure to add tests in `api/tests/` or `crates/tests/`
   - For TypeScript changes: Add tests in `ts/src/__tests__/`
3. **Run tests**:
   - Rust: `cargo test`
   - TypeScript: `cd ts && npm test`
4. **Check code quality**:
   - Rust: `cargo clippy` and `cargo fmt`
   - TypeScript: `cd ts && npm run lint && npm run format`
5. **Build and validate**:
   - Rust: `cargo build --release`
   - TypeScript: `cd ts && npm run build`
6. **Test Docker build**: `docker build -t dbx-test .`
7. **Submit a PR**: Ensure PR title is in conventional commit format (e.g. `feat: add new database adapter`) and PR description is detailed

### Testing Strategy

- **Unit Tests**: Rust tests in `api/tests/` and `crates/tests/`
- **Integration Tests**: End-to-end API tests
- **TypeScript Tests**: Vitest with coverage reporting
- **Docker Tests**: Containerized testing with Redis

### PR Requirements

- All Rust tests must pass (`cargo test`)
- All TypeScript tests must pass (`cd ts && npm test`)
- Code must be properly formatted (`cargo fmt` and `npm run format`)
- Linting must pass (`cargo clippy` and `npm run lint`)
- Build must succeed (`cargo build --release` and `npm run build`)
- Docker build must succeed (`docker build -t dbx-test .`)

### PR Template

AI Assistants must follow the PR template:

```markdown
### Summary

<!-- Brief summary of the PR. -->

### Details

<!-- Detailed list of changes in bullet point format. -->

### Areas Touched

<!--
Contextual list of areas of the project touched.

Example:
- API Server (`api/`)
- Core Library (`crates/`)
- TypeScript SDK (`ts/`)
- Documentation (`docs/`)
- Scripts (`scripts/`)

-->

### Testing

<!-- Describe how the changes were tested. -->

### Breaking Changes

<!-- List any breaking changes and migration instructions. -->
```

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

## Configuration

### Environment Variables

- `DATABASE_URL`: Redis connection URL (required)
- `DATABASE_TYPE`: Database type (default: redis)
- `HOST`: Server host (default: 0.0.0.0)
- `PORT`: Server port (default: 3000)
- `POOL_SIZE`: Connection pool size (default: 10)
- `LOG_LEVEL`: Logging level (default: INFO)

### Docker Configuration

```bash
# Example docker-compose.yml
version: '3.8'
services:
  dbx:
    image: fnlog0/dbx:latest
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=redis://redis:6379
      - LOG_LEVEL=DEBUG
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
```

## Documentation Writing Style

When writing or editing documentation files, follow these style guidelines:

### Voice and Perspective

#### Use Third Person for Technical Documentation

- ✅ "DBX is a lightweight API proxy designed for edge computing"
- ✅ "The TypeScript SDK provides type-safe database operations"
- ✅ "Redis operations are exposed through REST and WebSocket APIs"
- ❌ "We built DBX to..." or "Our proxy allows you to..."

#### Use Second Person for Instructions

When giving direct instructions to developers, use second person:

- ✅ "You can get started with DBX by running the Docker container"
- ✅ "After you have set up Redis, you can configure DBX"

#### Avoid First Person

Never use "we," "our," or "I" in technical documentation:

- ❌ "We implemented Redis support..."
- ✅ "Redis support was implemented..."
- ❌ "we don't want to think about it"
- ✅ "this adds unnecessary complexity"

### Examples

**Before (First Person):**

```markdown
We built DBX to solve the problem of database access in edge environments. We implemented Redis support first because we found it was the most common use case.
```

**After (Third Person):**

```markdown
DBX was built to solve the problem of database access in edge environments. Redis support was implemented first as it represents the most common use case.
```

## PR Review Comments

- @claude should **Always wrap PR review comments in `<details>` tags**
- Use descriptive summary text in the `<summary>` tag
- This improves PR browsability by allowing users to easily scan through activity without long review comments cluttering the view

Example format:

```html
<details>
  <summary>🔍 Code Review: [Brief description]</summary>

  [Your detailed review comments here]
</details>
```

## Common Development Tasks

### Adding a New Database Adapter

1. Create new adapter in `crates/adapter/[database_name]/`
2. Implement required traits and primitives
3. Add routes in `api/src/routes/[database_name]/`
4. Add tests in `api/tests/[database_name]/`
5. Update configuration to support new database type

### Adding New API Endpoints

1. Define request/response models in `api/src/models.rs`
2. Add route handlers in appropriate `api/src/routes/` directory
3. Add integration tests in `api/tests/`
4. Update TypeScript SDK types and clients if needed

### Updating TypeScript SDK

1. Update types in `ts/src/types/`
2. Add client methods in `ts/src/clients/`
3. Update main client in `ts/src/client.ts`
4. Add tests in `ts/src/__tests__/`
5. Update documentation and examples

This file should be updated when major architectural changes are made to the codebase.
