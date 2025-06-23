# DBX Crates Agent Contribution Guide

This document provides comprehensive information for AI agents to contribute effectively to the DBX Crates library. It includes folder structure, feature status, and detailed contribution guidelines for the core Rust database adapters.

## 📁 Crates Structure

```
crates/
├── adapter/                 # Database adapters
│   └── redis/              # Redis adapter implementation
│       ├── primitives/     # Redis data type operations
│       │   ├── string.rs   # String operations (21KB, 629 lines)
│       │   ├── hash.rs     # Hash operations (17KB, 516 lines)
│       │   ├── set.rs      # Set operations (25KB, 741 lines)
│       │   └── admin.rs    # Admin operations (25KB, 818 lines)
│       ├── client.rs       # Redis client implementation (5.2KB, 187 lines)
│       └── mod.rs          # Redis adapter module (8.0KB, 277 lines)
├── lib.rs                  # Library entry point
├── Cargo.toml              # Rust dependencies
├── README.md               # Library documentation
├── CONTRIBUTING.md         # General contribution guidelines
└── CHANGELOG.md            # Version history
```

## 🚀 Feature Status

### ✅ Completed Features

#### Redis Adapter (adapter/redis/)

- **String Operations**: Complete CRUD, counters, TTL management, Lua scripts
- **Hash Operations**: Full CRUD, batch operations, pipeline support, transactions
- **Set Operations**: Complete set operations, intersections, unions, batch processing
- **Admin Operations**: Server management, configuration, monitoring, health checks
- **Client Management**: Connection handling, health checks, connection pooling
- **Pipeline Support**: Batch command execution with atomic operations
- **Transaction Support**: Multi-command atomic transactions
- **Lua Scripts**: Custom script execution and predefined utility scripts
- **Error Handling**: Comprehensive error types with thiserror

### 🔄 In Progress Features

#### Infrastructure

- **Connection Pooling**: Enhanced connection management with configurable pools
- **Async Support**: Full async/await patterns with Tokio integration
- **Mock Support**: Testing utilities with mockall integration

### 📋 Planned Features

#### High Priority (0-3 months)

- **Redis Lists**: List data type operations (lpush, lpop, lrange, etc.)
- **Redis Sorted Sets**: Sorted set operations (zadd, zrange, zscore, etc.)
- **Redis Streams**: Stream data type support (xadd, xread, xrange, etc.)
- **PubSub**: Publish/Subscribe functionality
- **Cluster Support**: Redis cluster operations
- **Enhanced Error Handling**: More specific error types and recovery strategies

#### Medium Priority (3-6 months)

- **Query Builder**: Type-safe query construction interface
- **Migration Support**: Database schema management utilities
- **Backup/Restore**: Data management and persistence utilities
- **Performance Monitoring**: Metrics and performance tracking

#### Long Term (6+ months)

- **Multi-database Support**: Unified interface for multiple databases
- **WASM Compatibility**: WebAssembly support for browser environments
- **Embedded Systems**: Support for resource-constrained environments

## 🤖 AI Contribution Guidelines

### Understanding the Architecture

DBX Crates follows a modular adapter pattern:

1. **Adapter Layer** (`adapter/`): Database-specific implementations
2. **Primitives** (`adapter/redis/primitives/`): Data type-specific operations
3. **Client** (`adapter/redis/client.rs`): Connection and client management
4. **Module** (`adapter/redis/mod.rs`): Public API and trait implementations

### Adding New Redis Data Types

#### 1. Create New Primitive File

```rust
// crates/adapter/redis/primitives/list.rs
use crate::adapter::redis::client::RedisClient;
use crate::adapter::redis::RedisResult;
use std::sync::Arc;

/// Provides list operations for Redis.
/
pub struct ListOperations {
    client: Arc<RedisClient>,
}

impl ListOperations {
    /// Creates a new instance of `ListOperations`.

    pub fn new(client: Arc<RedisClient>) -> Self {
        Self { client }
    }

    /// Pushes values to the left side of a list.

    pub fn lpush(&self, key: &str, values: &[&str]) -> RedisResult<i64> {
        let mut cmd = redis::cmd("LPUSH");
        cmd.arg(key);
        for value in values {
            cmd.arg(value);
        }
        self.client.execute(cmd)
    }

    /// Pops and returns the leftmost element of a list.
    pub fn lpop(&self, key: &str) -> RedisResult<Option<String>> {
        let cmd = redis::cmd("LPOP").arg(key);
        self.client.execute(cmd)
    }

    /// Returns a range of elements from a list.
    pub fn lrange(&self, key: &str, start: i64, stop: i64) -> RedisResult<Vec<String>> {
        let cmd = redis::cmd("LRANGE").arg(key).arg(start).arg(stop);
        self.client.execute(cmd)
    }

    /// Gets the length of a list.
    pub fn llen(&self, key: &str) -> RedisResult<i64> {
        let cmd = redis::cmd("LLEN").arg(key);
        self.client.execute(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::redis::Redis;

    #[test]
    fn test_lpush_operation() {
        let redis = Redis::from_url("redis://127.0.0.1:6379").unwrap();
        let list = ListOperations::new(redis.client.clone());

        // Test lpush
        let result = list.lpush("test:list", &["value1", "value2"]).unwrap();
        assert_eq!(result, 2);

        // Test llen
        let length = list.llen("test:list").unwrap();
        assert_eq!(length, 2);

        // Test lrange
        let range = list.lrange("test:list", 0, -1).unwrap();
        assert_eq!(range, vec!["value2", "value1"]);

        // Test lpop
        let popped = list.lpop("test:list").unwrap();
        assert_eq!(popped, Some("value2".to_string()));
    }

    #[tokio::test]
    async fn test_async_list_operations() {
        let redis = Redis::from_url("redis://127.0.0.1:6379").unwrap();
        let list = ListOperations::new(redis.client.clone());

        // Async operations would go here
        let result = list.lpush("async:test:list", &["async_value"]).unwrap();
        assert_eq!(result, 1);
    }
}
```

#### 2. Add to Module System

```rust
// crates/adapter/redis/primitives/mod.rs
pub mod string;
pub mod hash;
pub mod set;
pub mod admin;
pub mod list; // Add this line

pub use string::StringOperations;
pub use hash::HashOperations;
pub use set::SetOperations;
pub use admin::AdminOperations;
pub use list::ListOperations; // Add this line
```

#### 3. Add to Main Redis Struct

```rust
// crates/adapter/redis/mod.rs
use crate::adapter::redis::primitives::ListOperations;

impl Redis {
    // ... existing methods ...

    /// Returns a new ListOperations instance for list operations.
    pub fn list(&self) -> ListOperations {
        ListOperations::new(self.client.clone())
    }
}
```

### Adding New Features to Existing Primitives

#### Example: Adding TTL Support to Lists

```rust
// In crates/adapter/redis/primitives/list.rs
impl ListOperations {
    // ... existing methods ...

    /// Pushes values to a list with expiration.
    pub fn lpush_with_expiry(&self, key: &str, values: &[&str], expiry: u64) -> RedisResult<i64> {
        let mut pipe = redis::pipe();

        // Add LPUSH command
        let mut cmd = redis::cmd("LPUSH");
        cmd.arg(key);
        for value in values {
            cmd.arg(value);
        }
        pipe.add_command(cmd);

        // Add EXPIRE command
        pipe.cmd("EXPIRE").arg(key).arg(expiry);

        let results: (i64, i64) = pipe.query(&mut self.client.get_connection()?)?;
        Ok(results.0)
    }
}
```

### Code Quality Standards

#### Rust Conventions

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and structs
- Use `SCREAMING_SNAKE_CASE` for constants
- Document all public APIs with comprehensive `///` comments
- Include detailed usage examples in documentation
- Follow Rust API guidelines for naming and structure

#### Error Handling

```rust
use thiserror::Error;

/// Comprehensive error types for Redis operations.
///
/// This enum provides detailed error information for different types of
/// Redis operation failures, enabling better error handling and debugging.
/// Each variant includes specific context about the failure type and cause.
#[derive(Error, Debug)]
pub enum RedisError {
    /// Connection-related errors including network issues and authentication failures.
    /// This variant is used when the Redis client cannot establish or maintain
    /// a connection to the Redis server.
    #[error("Connection failed: {0}")]
    Connection(String),

    /// Operation-specific errors including command failures and timeouts.
    /// This variant is used when Redis commands fail due to server-side issues,
    /// invalid commands, or operation timeouts.
    #[error("Operation failed: {0}")]
    Operation(String),

    /// Invalid argument errors for malformed input parameters.
    /// This variant is used when function arguments don't meet the required
    /// format or constraints.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Key not found errors for operations on non-existent keys.
    /// This variant is used when operations are performed on keys that
    /// don't exist in the Redis database.
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    /// Type mismatch errors when operations are performed on wrong data types.
    /// This variant provides detailed information about expected vs actual
    /// data types for better debugging.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    /// Serialization errors for complex data types.
    /// This variant is used when data cannot be properly serialized or
    /// deserialized for Redis operations.
    #[error("Serialization failed: {0}")]
    Serialization(String),
}

/// Type alias for Redis operation results.
///
/// This provides a convenient way to handle Redis operation results
/// with consistent error handling across the library. All Redis operations
/// should return this type for consistency and better error handling.
pub type RedisResult<T> = Result<T, RedisError>;
```

#### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic functionality of the operation.
    ///
    /// This test verifies that the operation works correctly under normal conditions
    /// with valid input parameters. It should cover the happy path and ensure
    /// that basic functionality operates as expected.
    #[test]
    fn test_basic_operation() {
        // Test implementation with valid inputs
        // Verify expected behavior
        // Check return values
        // Ensure no side effects on other operations
    }

    /// Tests error conditions and edge cases.
    ///
    /// This test ensures that the operation handles error conditions gracefully
    /// and returns appropriate error types. It should cover various failure
    /// scenarios and edge cases that might occur in production.
    #[test]
    fn test_error_conditions() {
        // Test with invalid inputs
        // Test with non-existent keys
        // Test with wrong data types
        // Test with malformed parameters
        // Verify error messages and types
        // Ensure errors are properly propagated
    }

    /// Tests async operation behavior.
    ///
    /// This test verifies that async operations work correctly in concurrent
    /// environments and handle async-specific scenarios. It should test
    /// concurrent access, async error handling, and performance characteristics.
    #[tokio::test]
    async fn test_async_operation() {
        // Test async implementation
        // Test concurrent operations
        // Test async error handling
        // Test cancellation scenarios
        // Verify async performance characteristics
    }

    /// Tests edge cases and boundary conditions.
    ///
    /// This test covers edge cases such as empty inputs, maximum values,
    /// and boundary conditions that might cause issues. It should ensure
    /// robustness and prevent potential bugs in production.
    #[test]
    fn test_edge_cases() {
        // Test with empty inputs
        // Test with maximum values
        // Test with boundary conditions
        // Test with special characters
        // Test with very large datasets
        // Test with concurrent modifications
    }

    /// Tests performance characteristics.
    ///
    /// This test verifies that operations meet performance expectations
    /// and don't have significant performance regressions. It should
    /// measure operation times and ensure they remain within acceptable limits.
    #[test]
    fn test_performance() {
        // Test with large datasets
        // Measure operation times
        // Verify performance characteristics
        // Test memory usage patterns
        // Ensure no memory leaks
        // Test scalability with concurrent operations
    }

    /// Tests integration with other components.
    ///
    /// This test verifies that the operation works correctly when integrated
    /// with other parts of the system. It should test real-world usage
    /// scenarios and ensure compatibility with existing functionality.
    #[test]
    fn test_integration() {
        // Test with other Redis operations
        // Test with connection pooling
        // Test with error handling systems
        // Test with monitoring and logging
        // Verify end-to-end functionality
    }
}
```

### Performance Considerations

#### Connection Pooling

```rust
use std::sync::Arc;
use redis::ConnectionManager;
use std::time::Duration;

/// Thread-safe Redis client with connection pooling.
///
/// This struct provides efficient connection management with automatic
/// connection pooling, retry logic, and connection health monitoring.
/// All operations are thread-safe and can be shared across multiple threads.
/// The connection pool automatically handles connection lifecycle, including
/// creation, reuse, and cleanup of connections.
pub struct RedisClient {
    /// Connection manager for handling pooled connections.
    /// This manages a pool of Redis connections that can be reused
    /// across multiple operations, improving performance and reducing
    /// connection overhead.
    manager: Arc<ConnectionManager>,

    /// Configuration for connection behavior.
    /// This contains settings for pool size, timeouts, retry policies,
    /// and other connection-related parameters.
    config: RedisConfig,
}

impl RedisClient {
    /// Executes a Redis command asynchronously with connection pooling.
    ///
    /// This method automatically manages connections from the pool,
    /// handles connection failures, and provides retry logic for
    /// transient errors. It ensures optimal performance by reusing
    /// connections and handling connection lifecycle automatically.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The Redis command to execute
    ///
    /// # Returns
    ///
    /// The result of the Redis operation
    ///
    /// # Errors
    ///
    /// Returns an error if the command fails or connection issues occur.
    /// Connection errors are automatically retried based on the configured
    /// retry policy.
    ///
    /// # Performance Characteristics
    ///
    /// * Connection reuse: Connections are automatically reused from the pool
    /// * Automatic retry: Transient failures are automatically retried
    /// * Connection health: Unhealthy connections are automatically replaced
    /// * Thread safety: Safe for concurrent access from multiple threads
    pub async fn execute<T>(&self, cmd: redis::Cmd) -> RedisResult<T>
    where
        T: redis::FromRedisValue,
    {
        let mut conn = self.manager.clone().get().await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        cmd.query_async(&mut conn).await
            .map_err(|e| RedisError::Operation(e.to_string()))
    }
}
```

#### Batch Operations

````rust
impl ListOperations {
    /// Performs multiple list operations in a single atomic transaction.
    ///
    /// This method executes multiple list operations atomically using Redis pipelines,
    /// providing better performance than individual operations and ensuring
    /// consistency across all operations. The entire batch either succeeds
    /// or fails as a unit, maintaining data consistency.
    ///
    /// # Arguments
    ///
    /// * `operations` - Array of tuples containing (key, values) for each operation.
    ///   Each tuple represents a separate list operation to be performed.
    ///
    /// # Returns
    ///
    /// Vector of results for each operation in the same order as input.
    /// Each result represents the new length of the corresponding list.
    ///
    /// # Errors
    ///
    /// Returns an error if any operation fails, causing the entire batch to fail.
    /// This ensures atomicity - either all operations succeed or none do.
    ///
    /// # Performance Characteristics
    ///
    /// * Network efficiency: Single round-trip for all operations
    /// * Atomicity: All operations succeed or fail together
    /// * Reduced latency: Fewer network round-trips compared to individual operations
    /// * Memory efficiency: Operations are batched to reduce memory overhead
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dbx_crates::adapter::redis::Redis;
    /// let redis = Redis::from_url("redis://127.0.0.1:6379")?;
    /// let list = redis.list();
    ///
    /// let operations = vec![
    ///     ("list1", &["a", "b"] as &[&str]),
    ///     ("list2", &["c", "d"] as &[&str]),
    /// ];
    ///
    /// let results = list.lpush_many(&operations)?;
    /// assert_eq!(results, vec![2, 2]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn lpush_many(&self, operations: &[(&str, &[&str])]) -> RedisResult<Vec<i64>> {
        let mut pipe = redis::pipe();

        for (key, values) in operations {
            let mut cmd = redis::cmd("LPUSH");
            cmd.arg(key);
            for value in values {
                cmd.arg(value);
            }
            pipe.add_command(cmd);
        }

        pipe.query(&mut self.client.get_connection()?)
            .map_err(|e| RedisError::Operation(e.to_string()))
    }
}
````

### Documentation Requirements

#### Comprehensive Function Documentation

All public functions must include detailed documentation that covers:

1. **Purpose and Behavior**: Clear description of what the function does
2. **Arguments**: Detailed explanation of each parameter with types and constraints
3. **Returns**: Description of return values and their meaning
4. **Errors**: Comprehensive list of possible error conditions
5. **Performance**: Time and space complexity information
6. **Thread Safety**: Concurrency considerations
7. **Examples**: Multiple usage examples including basic, error handling, and advanced cases

#### Example Documentation Template

```rust
/// Pushes values to the left side of a list with comprehensive error handling.
pub fn lpush(&self, key: &str, values: &[&str]) -> RedisResult<i64> {
    // Implementation with comprehensive error handling
    // and performance optimizations
}
```

#### Struct Documentation

All public structs should include:

```rust
/// Provides list operations for Redis.
pub struct ListOperations {
    client: Arc<RedisClient>,
}
```

#### Error Documentation

Error types should be thoroughly documented:

```rust
/// Comprehensive error types for Redis operations.
///
/// This enum provides detailed error information for different types of
/// Redis operation failures, enabling better error handling and debugging.
/// Each variant includes specific context about the failure type and cause.
#[derive(Error, Debug)]
pub enum RedisError {
    /// Connection-related errors including network issues and authentication failures.
    /// This variant is used when the Redis client cannot establish or maintain
    /// a connection to the Redis server.
    #[error("Connection failed: {0}")]
    Connection(String),

    /// Operation-specific errors including command failures and timeouts.
    /// This variant is used when Redis commands fail due to server-side issues,
    /// invalid commands, or operation timeouts.
    #[error("Operation failed: {0}")]
    Operation(String),

    /// Invalid argument errors for malformed input parameters.
    /// This variant is used when function arguments don't meet the required
    /// format or constraints.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Key not found errors for operations on non-existent keys.
    /// This variant is used when operations are performed on keys that
    /// don't exist in the Redis database.
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    /// Type mismatch errors when operations are performed on wrong data types.
    /// This variant provides detailed information about expected vs actual
    /// data types for better debugging.
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    /// Serialization errors for complex data types.
    /// This variant is used when data cannot be properly serialized or
    /// deserialized for Redis operations.
    #[error("Serialization failed: {0}")]
    Serialization(String),
}
```

### Development Workflow

1. **Create Feature Branch**: `git checkout -b feature/redis-lists`
2. **Implement Feature**: Follow the comprehensive patterns above
3. **Add Tests**: Include unit tests for all public functions with edge cases
4. **Update Documentation**: Add detailed docs with multiple examples
5. **Run Quality Checks**:
   ```bash
   cargo fmt
   cargo clippy -p dbx-crates -- -D warnings
   cargo test -p dbx-crates
   cargo test -p dbx-crates --doc
   cargo doc --no-deps --open
   ```
6. **Submit PR**: Create pull request with clear description and examples

### File Naming Conventions

- **Primitive files**: `{data_type}.rs` (e.g., `list.rs`, `sorted_set.rs`)
- **Test files**: Tests go in the same file as the implementation
- **Module files**: `mod.rs` for module organization
- **Client files**: `client.rs` for connection management

### Dependencies Management

Add new dependencies to `crates/Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...
new_dependency = "1.0.0"

[dev-dependencies]
# For testing dependencies...
test_dependency = "1.0.0"
```

### Common Patterns

#### Async Operations

```rust
/// Performs an asynchronous operation with proper error handling.
pub async fn async_operation(&self, key: &str) -> RedisResult<String> {
    let client = self.client.clone();
    let key = key.to_string();

    tokio::spawn(async move {
        // Async implementation with proper error handling
        // and connection management
    }).await.map_err(|e| RedisError::Operation(e.to_string()))?
}
```

#### Configuration Management

```rust
use std::time::Duration;

/// Configuration for Redis client behavior.
///
/// This struct provides comprehensive configuration options for
/// Redis client behavior including connection settings, timeouts,
/// and retry policies. All settings are designed to provide optimal
/// performance and reliability for different use cases.
#[derive(Debug, Clone)]
pub struct RedisConfig {
    /// Redis server URL in standard format.
    /// Supports various connection schemes including redis://, rediss://,
    /// and unix:// for different deployment scenarios.
    pub url: String,

    /// Maximum number of connections in the pool.
    /// This setting balances resource usage with performance.
    /// Higher values provide better concurrency but use more memory.
    pub pool_size: usize,

    /// Connection timeout for operations.
    /// This prevents operations from hanging indefinitely and
    /// ensures timely error reporting for connection issues.
    pub timeout: Duration,

    /// Number of retry attempts for failed operations.
    /// This provides resilience against transient network issues
    /// and temporary Redis server problems.
    pub retry_attempts: u32,

    /// Retry delay between attempts.
    /// This prevents overwhelming the server during recovery
    /// and provides exponential backoff for better resilience.
    pub retry_delay: Duration,

    /// Whether to enable connection health checks.
    /// Health checks ensure that connections in the pool are
    /// still valid and automatically replace unhealthy connections.
    pub health_check_enabled: bool,
}

impl Default for RedisConfig {
    /// Creates a default configuration with sensible defaults.
    ///
    /// The default configuration is suitable for most development
    /// and production environments with reasonable performance
    /// and reliability characteristics. It provides a good balance
    /// between resource usage and performance.
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            pool_size: 10,
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
            health_check_enabled: true,
        }
    }
}
```

This guide enables AI agents to effectively contribute to the DBX Crates library by understanding the architecture, following established patterns, and maintaining high code quality standards with comprehensive documentation.
