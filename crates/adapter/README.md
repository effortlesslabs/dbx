# DBX Adapter

A standardized database adapter library for DBX that provides consistent interfaces for different database systems.

## Features

- **Standardized Interfaces**: Common traits for database operations
- **Redis Support**: Full Redis adapter with all data types
- **Async Support**: Built-in async/await support
- **Connection Pooling**: Optional connection pooling for high-performance applications
- **Error Handling**: Comprehensive error types and handling
- **Extensible**: Easy to add new database adapters

## Usage

### Basic Usage

```rust
use dbx_adapter::{redis::Redis, traits::DatabaseAdapter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Redis adapter
    let redis = Redis::from_url("redis://localhost:6379")?;

    // Check connection
    let is_alive = redis.ping().await?;
    println!("Redis is alive: {}", is_alive);

    // Use string operations
    let string_ops = redis.string();
    string_ops.set("my_key", "my_value").await?;
    let value = string_ops.get("my_key").await?;
    println!("Value: {:?}", value);

    Ok(())
}
```

### Using Connection Pooling

```rust
use dbx_adapter::redis::Redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Redis adapter with connection pool
    let redis = Redis::with_connection_pool("redis://localhost:6379", 10)?;

    // Get a connection from the pool
    let connection = redis.get_connection().await?;

    // Use the connection
    let string_ops = connection.string();
    string_ops.set("pooled_key", "pooled_value").await?;

    Ok(())
}
```

### Using Different Data Types

```rust
use dbx_adapter::redis::Redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis = Redis::from_url("redis://localhost:6379")?;

    // String operations
    let string_ops = redis.string();
    string_ops.set("user:1:name", "John Doe").await?;

    // Hash operations
    let hash_ops = redis.hash();
    hash_ops.hset("user:1", "email", "john@example.com").await?;
    hash_ops.hset("user:1", "age", "30").await?;

    // Set operations
    let set_ops = redis.set();
    set_ops.sadd("user:1:roles", "admin").await?;
    set_ops.sadd("user:1:roles", "user").await?;

    // Get all user data
    let name = string_ops.get("user:1:name").await?;
    let user_data = hash_ops.hgetall("user:1").await?;
    let roles = set_ops.smembers("user:1:roles").await?;

    println!("Name: {:?}", name);
    println!("User data: {:?}", user_data);
    println!("Roles: {:?}", roles);

    Ok(())
}
```

## Adapter Traits

The library provides several standard traits that adapters can implement:

### DatabaseAdapter

Basic database operations:

```rust
use dbx_adapter::traits::DatabaseAdapter;

#[async_trait::async_trait]
impl DatabaseAdapter for MyAdapter {
    type Error = MyError;

    async fn ping(&self) -> Result<bool, Self::Error> {
        // Implementation
    }

    async fn close(&self) -> Result<(), Self::Error> {
        // Implementation
    }

    fn is_connected(&self) -> bool {
        // Implementation
    }
}
```

### KeyValueAdapter

Key-value operations:

```rust
use dbx_adapter::traits::KeyValueAdapter;

#[async_trait::async_trait]
impl KeyValueAdapter for MyAdapter {
    async fn get<K: AsRef<str> + Send>(&self, key: K) -> Result<Option<String>, Self::Error> {
        // Implementation
    }

    async fn set<K: AsRef<str> + Send, V: AsRef<str> + Send>(
        &self,
        key: K,
        value: V,
    ) -> Result<(), Self::Error> {
        // Implementation
    }

    // ... other methods
}
```

## Error Handling

The library provides comprehensive error types:

```rust
use dbx_adapter::error::{AdapterError, ConnectionError, OperationError};

match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(AdapterError::Connection(ConnectionError::Timeout(msg))) => {
        println!("Connection timeout: {}", msg);
    }
    Err(AdapterError::Operation(OperationError::KeyNotFound(key))) => {
        println!("Key not found: {}", key);
    }
    Err(err) => println!("Other error: {}", err),
}
```

## Features

- `default`: Basic functionality
- `async`: Async support with tokio
- `connection-pool`: Connection pooling support

## Adding New Adapters

To add a new database adapter:

1. Create a new module in `src/`
2. Implement the relevant traits from `traits.rs`
3. Use the error types from `error.rs`
4. Add the module to `src/mod.rs`

Example:

```rust
// src/postgres/mod.rs
use crate::traits::{DatabaseAdapter, KeyValueAdapter};
use crate::error::AdapterError;

pub struct PostgresAdapter {
    // Implementation details
}

#[async_trait::async_trait]
impl DatabaseAdapter for PostgresAdapter {
    type Error = AdapterError;

    // Implement required methods
}

#[async_trait::async_trait]
impl KeyValueAdapter for PostgresAdapter {
    // Implement required methods
}
```

## License

MIT OR Apache-2.0
