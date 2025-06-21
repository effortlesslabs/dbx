# DBX Crates

A comprehensive Rust library providing database adapters and utilities for the DBX project.

## Overview

DBX Crates provides type-safe, high-performance database adapters with support for:

- **Redis**: Full Redis adapter with string, hash, set operations
- **Connection Pooling**: Efficient connection management
- **Pipelines & Transactions**: Atomic operations and batch processing
- **Lua Scripts**: Custom Redis scripts for complex operations
- **Async Support**: Non-blocking operations with Tokio

## Features

### 🚀 Core Features

- **Type-safe operations** for all Redis data types
- **Connection pooling** for high-concurrency applications
- **Pipeline support** for batch operations
- **Transaction support** for atomic operations
- **Lua script integration** for complex business logic
- **Comprehensive error handling** with custom error types

### 📦 Redis Primitives

- **Strings**: Basic operations, counters, TTL management
- **Hashes**: Field-value operations, batch processing
- **Sets**: Set operations, intersections, unions
- **Lists**: (Coming soon)
- **Sorted Sets**: (Coming soon)

### 🔧 Advanced Features

- **Rate limiting** with Redis scripts
- **Batch operations** for performance optimization
- **Connection health checks** and automatic reconnection
- **Custom Lua scripts** for business-specific logic

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dbx-crates = { path = "./crates" }

# Optional features
dbx-crates = { path = "./crates", features = ["async", "connection-pool"] }
```

## Quick Start

```rust
use dbx_crates::adapter::redis::Redis;

// Create a Redis connection
let redis = Redis::from_url("redis://127.0.0.1:6379")?;

// String operations
let string_ops = redis.string();
string_ops.set("key", "value")?;
let value = string_ops.get("key")?;

// Hash operations
let hash_ops = redis.hash();
hash_ops.hset("user:1", "name", "Alice")?;
let name = hash_ops.hget("user:1", "name")?;

// Set operations
let set_ops = redis.set();
set_ops.sadd("tags", &["rust", "redis", "database"])?;
let members = set_ops.smembers("tags")?;

// Batch operations
let batch = Redis::batch();
batch.set_many(&redis, vec![("key1", "value1"), ("key2", "value2")])?;
```

## Usage Examples

### Basic Operations

```rust
use dbx_crates::adapter::redis::Redis;

async fn basic_example() -> Result<(), Box<dyn std::error::Error>> {
    let redis = Redis::from_url("redis://127.0.0.1:6379")?;

    // String operations
    let string_ops = redis.string();
    string_ops.set("user:1:name", "Alice")?;
    string_ops.set_with_expiry("session:123", "token", 3600)?;

    // Counter operations
    let visits = string_ops.incr("page:visits")?;
    let score = string_ops.incr_by("user:1:score", 10)?;

    Ok(())
}
```

### Pipeline Operations

```rust
use dbx_crates::adapter::redis::Redis;

fn pipeline_example() -> Result<(), Box<dyn std::error::Error>> {
    let redis = Redis::from_url("redis://127.0.0.1:6379")?;
    let string_ops = redis.string();

    // Execute multiple commands in a pipeline
    let results: (String, i64, Option<String>) = string_ops.with_pipeline(|pipe| {
        pipe.cmd("SET").arg("key1").arg("value1")
           .cmd("INCR").arg("counter")
           .cmd("GET").arg("key1")
    })?;

    println!("Pipeline results: {:?}", results);
    Ok(())
}
```

### Transactions

```rust
use dbx_crates::adapter::redis::Redis;

fn transaction_example() -> Result<(), Box<dyn std::error::Error>> {
    let redis = Redis::from_url("redis://127.0.0.1:6379")?;
    let string_ops = redis.string();

    // Execute multiple commands in a transaction
    let results: ((), i64, ()) = string_ops.transaction(|pipe| {
        pipe.cmd("SET").arg("account:1:balance").arg("100")
           .cmd("INCRBY").arg("account:1:balance").arg("-50")
           .cmd("EXPIRE").arg("account:1:balance").arg(3600)
    })?;

    println!("Transaction completed");
    Ok(())
}
```

### Lua Scripts

```rust
use dbx_crates::adapter::redis::Redis;

fn lua_script_example() -> Result<(), Box<dyn std::error::Error>> {
    let redis = Redis::from_url("redis://127.0.0.1:6379")?;
    let string_ops = redis.string();

    // Create a custom Lua script
    let rate_limiter = string_ops.create_script(r#"
        local key = KEYS[1]
        local limit = tonumber(ARGV[1])
        local window = tonumber(ARGV[2])

        local current = redis.call('GET', key)
        if current and tonumber(current) >= limit then
            return 0
        end

        redis.call('INCR', key)
        redis.call('EXPIRE', key, window)
        return 1
    "#);

    // Execute the script
    let allowed: i64 = string_ops.eval_script(&rate_limiter, &["rate:user:123"], &[5, 60])?;

    if allowed == 1 {
        println!("Request allowed");
    } else {
        println!("Rate limit exceeded");
    }

    Ok(())
}
```

### Connection Pooling

```rust
use dbx_crates::adapter::redis::Redis;

#[cfg(feature = "connection-pool")]
async fn pool_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Redis instance with connection pooling
    let redis = Redis::with_connection_pool("redis://127.0.0.1:6379", 10)?;

    // Get a connection from the pool
    let connection = redis.get_connection()?;

    // Use the connection
    let string_ops = redis.string();
    string_ops.set("pooled:key", "value")?;

    Ok(())
}
```

## API Reference

### Redis Client

```rust
pub struct Redis {
    client: RedisClient,
}

impl Redis {
    pub fn new(client: RedisClient) -> Self;
    pub fn from_url(url: &str) -> RedisResult<Self>;
    pub fn string(&self) -> RedisString;
    pub fn hash(&self) -> RedisHash;
    pub fn set(&self) -> RedisSet;
    pub fn batch() -> BatchOperations;
}
```

### String Operations

```rust
pub struct RedisString {
    conn: Arc<Mutex<Connection>>,
}

impl RedisString {
    // Basic operations
    pub fn set(&self, key: &str, value: &str) -> RedisResult<()>;
    pub fn get(&self, key: &str) -> RedisResult<Option<String>>;
    pub fn incr(&self, key: &str) -> RedisResult<i64>;
    pub fn decr(&self, key: &str) -> RedisResult<i64>;

    // Batch operations
    pub fn set_many(&self, kvs: Vec<(&str, &str)>) -> RedisResult<()>;
    pub fn get_many(&self, keys: Vec<&str>) -> RedisResult<Vec<Option<String>>>;

    // Pipeline and transaction
    pub fn with_pipeline<F, T>(&self, f: F) -> RedisResult<T>;
    pub fn transaction<F, T>(&self, f: F) -> RedisResult<T>;
}
```

### Hash Operations

```rust
pub struct RedisHash {
    conn: Arc<Mutex<Connection>>,
}

impl RedisHash {
    // Basic operations
    pub fn hset(&self, key: &str, field: &str, value: &str) -> RedisResult<bool>;
    pub fn hget(&self, key: &str, field: &str) -> RedisResult<Option<String>>;
    pub fn hgetall(&self, key: &str) -> RedisResult<HashMap<String, String>>;

    // Batch operations
    pub fn hset_many(&self, hash_fields: Vec<(&str, Vec<(&str, &str)>)>) -> RedisResult<Vec<bool>>;
    pub fn hget_many(&self, hash_fields: Vec<(&str, &str)>) -> RedisResult<Vec<Option<String>>>;
}
```

### Set Operations

```rust
pub struct RedisSet {
    conn: Arc<Mutex<Connection>>,
}

impl RedisSet {
    // Basic operations
    pub fn sadd(&self, key: &str, members: &[&str]) -> RedisResult<usize>;
    pub fn srem(&self, key: &str, members: &[&str]) -> RedisResult<usize>;
    pub fn smembers(&self, key: &str) -> RedisResult<Vec<String>>;
    pub fn sismember(&self, key: &str, member: &str) -> RedisResult<bool>;

    // Set operations
    pub fn sinter(&self, keys: &[&str]) -> RedisResult<Vec<String>>;
    pub fn sunion(&self, keys: &[&str]) -> RedisResult<Vec<String>>;
    pub fn sdiff(&self, keys: &[&str]) -> RedisResult<Vec<String>>;
}
```

## Error Handling

The library provides comprehensive error handling with custom error types:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
```

## Configuration

### Environment Variables

```bash
# Redis connection
DATABASE_URL=redis://127.0.0.1:6379

# Connection pool settings
POOL_SIZE=10

# Logging
LOG_LEVEL=INFO
```

### Features

Enable optional features in `Cargo.toml`:

```toml
[dependencies]
dbx-crates = { path = "./crates", features = ["async", "connection-pool"] }
```

Available features:

- `async`: Enable async operations with Tokio
- `connection-pool`: Enable connection pooling
- `default`: Basic functionality only

## Testing

Run the test suite:

```bash
# Run all tests
cargo test -p dbx-crates

# Run with verbose output
cargo test -p dbx-crates -- --nocapture

# Run specific test
cargo test -p dbx-crates test_name

# Run doc tests
cargo test -p dbx-crates --doc
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository
2. Install dependencies: `cargo build`
3. Run tests: `cargo test -p dbx-crates`
4. Check formatting: `cargo fmt`
5. Run clippy: `cargo clippy -p dbx-crates`

### Code Style

- Follow Rust conventions
- Use meaningful variable names
- Add comprehensive documentation
- Include tests for new features
- Update this README for API changes

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.

## Support

- **Issues**: [GitHub Issues](https://github.com/your-org/dbx/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/dbx/discussions)
- **Documentation**: [API Docs](https://docs.rs/dbx-crates)

## Roadmap

- [ ] List operations support
- [ ] Sorted set operations
- [ ] Stream operations
- [ ] Pub/Sub support
- [ ] Cluster support
- [ ] Sentinel support
- [ ] More database adapters (PostgreSQL, MongoDB)
