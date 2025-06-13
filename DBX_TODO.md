# 📘 DBX – Project TODO Guide

> Minimal API layer for all types of databases, portable across Workers, Raspberry Pi, and RISC-V boards. Written in Rust with bindings for TypeScript and other languages.

---

## ✅ Phase 1: Redis Productionization (Priority)

### 🔹 1. Redis Core Implementation

- [] Basic Redis driver implementation with `redis-rs`
- [] Enhance Redis driver with:
  - [] Connection pooling and retry logic
  - [] Proper error handling and custom error types
  - [] Support for all Redis data types (String, Hash, List, Set, Sorted Set)
  - [] Transaction support
  - [] Pub/Sub capabilities
  - [] Lua scripting support
  - [] Pipeline operations
  - [] Stream operations
  - [] HyperLogLog operations
- [] Add comprehensive unit tests
- [ ] Add integration tests with Redis server
- [ ] Add performance benchmarks

### 🔹 2. Redis TypeScript Bindings

- [ ] Complete NAPI-RS bindings for Node.js
- [ ] Add TypeScript type definitions
- [ ] Create async-safe wrappers for all Redis operations
- [ ] Add connection pooling in TypeScript layer
- [ ] Add example usage in Node.js
- [ ] Add TypeScript unit tests
- [ ] Add TypeScript integration tests

### 🔹 3. Redis API Layer

- [ ] Build Redis-specific endpoints:
  - [ ] `/redis/string` - String operations
  - [ ] `/redis/hash` - Hash operations
  - [ ] `/redis/list` - List operations
  - [ ] `/redis/set` - Set operations
  - [ ] `/redis/zset` - Sorted Set operations
  - [ ] `/redis/pubsub` - Pub/Sub operations
  - [ ] `/redis/script` - Lua script execution
  - [ ] `/redis/stream` - Stream operations
  - [ ] `/redis/hll` - HyperLogLog operations
- [ ] Add Redis-specific authentication
- [ ] Add Redis connection configuration
- [ ] Add Redis health check endpoint

### 🔹 4. Redis Documentation & Examples

- [ ] Add comprehensive API documentation
- [ ] Create example applications:
  - [ ] Basic key-value store
  - [ ] Caching layer
  - [ ] Pub/Sub messaging
  - [ ] Rate limiting
  - [ ] Session store
  - [ ] Stream processing
  - [ ] Cardinality estimation
- [ ] Add deployment guides for:
  - [ ] Cloudflare Workers
  - [ ] Raspberry Pi
  - [ ] Docker containers

---

## ✅ Phase 1: Core System in Rust

### 🔹 1. Project Setup

- [x] Create monorepo using `cargo` workspaces
- [x] Set up `core` crate for shared traits/types
- [ ] Choose lightweight HTTP server: `axum` / `salvo`
- [x] Define DBX core API contract (e.g. `/query`, `/insert`, `/metadata`)

### 🔹 2. Database Abstractions

- [x] Create `DbxDatabase` trait with:
  - [x] `connect()`
  - [x] `query(sql: &str)`
  - [x] `insert(...)`
  - [x] `get_metadata()`
- [ ] Implement drivers as separate crates:
  - [ ] `sqlite` using `rusqlite`
  - [ ] `postgres` using `sqlx`
  - [ ] `mongo` using `mongodb`
  - [x] `redis` using `redis-rs`
    - [x] Add connection pooling
    - [x] Add error handling
    - [x] Add data type support
    - [x] Add transaction support
    - [x] Add Pub/Sub support
    - [x] Add Lua scripting
    - [x] Add pipeline operations
    - [x] Add Stream support
    - [x] Add HyperLogLog support
    - [ ] Add NAPI-RS bindings for Node.js
    - [ ] Add TypeScript type definitions
    - [ ] Add async-safe wrappers for Redis operations
    - [ ] Add example usage in Node.js
- [ ] Add unit tests for each driver

---

## ✅ Phase 2: API Layer

### 🔹 3. Minimal HTTP API Server

- [ ] Build `/query`, `/insert`, `/describe` endpoints
- [ ] Use `serde_json` for input/output
- [ ] Add optional JWT authentication
- [ ] Add config loader for database credentials

### 🔹 4. WASM / Worker Compatibility

- [ ] Add feature flag: `wasm` vs `native`
- [ ] Replace incompatible crates (`tokio`, etc.) for WASM version
- [ ] Compile to WebAssembly target

---

### 🔹 6. NAPI-RS Native Binding

- [ ] Add `napi-rs` to expose Rust API to Node.js
- [ ] Create async-safe wrappers for DB operations
- [ ] Package as NPM `@dbx/native`
- [ ] Add example for Raspberry Pi usage

---

## ✅ Phase 4: CLI & Configuration

### 🔹 7. CLI Tool

- [ ] Create `dbx-cli` crate
- [ ] Commands:
  - `dbx serve` – start API server
  - `dbx connect` – test DB connection
  - `dbx inspect` – get metadata
- [ ] Add `.dbxrc` config file loader (TOML/YAML)

---

## ✅ Phase 5: Language Bindings (Optional)

### 🔹 8. FFI / C ABI for Other Languages

- [ ] Create `dbx-ffi` crate
- [ ] Expose core API via `extern "C"` functions
- [ ] Add:
  - [ ] Python wrapper (`ctypes`)
  - [ ] Ruby wrapper (`ffi`)
  - [ ] C# wrapper (`P/Invoke`)
  - [ ] Java JNI wrapper (optional)

---

## ✅ Phase 6: Testing + Deployment

### 🔹 9. Testing & QA

- [ ] Add integration tests across all endpoints
- [ ] Test with real DBs: SQLite, Postgres, Mongo, Redis
- [ ] Run on:
  - [ ] Cloudflare Workers
  - [ ] Raspberry Pi
  - [ ] RISC-V emulator / real board

### 🔹 10. CI/CD

- [ ] GitHub Actions for:
  - [ ] Linting & Formatting (`cargo fmt`, `clippy`)
  - [ ] Tests
  - [ ] WASM build & npm publish
- [ ] Auto-deploy Docs using `mdbook` or `Docusaurus`

---

## 🧩 Bonus: Dev Experience

- [ ] Create REST playground UI (Swagger / Redoc)
- [ ] Add OpenAPI spec generation (`utoipa` or `paperclip`)
- [ ] Add internal telemetry/logging hooks
- [ ] Add Redis or in-memory caching for frequent queries
