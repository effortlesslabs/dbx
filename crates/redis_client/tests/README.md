# redis_rs Test Suite

This directory contains comprehensive tests for the `redis_rs` crate, covering both HTTP and WebSocket client functionality, organized by operation type.

## Test Structure

```
tests/
├── mod.rs                 # Main test module and utilities
├── common/                # Common functionality tests
│   └── mod.rs            # Serialization and type tests
├── redis/                 # HTTP client tests
│   ├── mod.rs            # HTTP client-specific tests
│   ├── string.rs         # HTTP string operations tests
│   └── set.rs            # HTTP set operations tests
├── redis_ws/              # WebSocket client tests
│   ├── mod.rs            # WebSocket client-specific tests
│   ├── string.rs         # WebSocket string operations tests
│   └── set.rs            # WebSocket set operations tests
└── README.md             # This file
```

## Test Categories

### 1. Common Tests (`common/`)

- **Purpose**: Test shared functionality and data types
- **Features**: No specific features required
- **Tests**:
  - Serialization/deserialization of request/response types
  - String and set type validation
  - API response handling
  - Pattern results processing

### 2. HTTP Client Tests (`redis/`)

- **Purpose**: Test HTTP-based Redis operations
- **Features**: `http`
- **Structure**:
  - `mod.rs`: Client creation, configuration, and error handling
  - `string.rs`: All string operations (get, set, delete, batch, patterns, TTL, etc.)
  - `set.rs`: All set operations (add, remove, cardinality, members, set operations, etc.)

### 3. WebSocket Client Tests (`redis_ws/`)

- **Purpose**: Test WebSocket-based Redis operations
- **Features**: `websocket`
- **Structure**:
  - `mod.rs`: Client creation, configuration, and error handling
  - `string.rs`: All string operations (get, set, delete, batch, patterns, TTL, etc.)
  - `set.rs`: All set operations (add, remove, cardinality, members, set operations, etc.)

## Running Tests

### Using Cargo Directly

```bash
# Run all tests with all features
cargo test --all-features

# Run tests with specific features
cargo test --features http
cargo test --features websocket
cargo test --features "http,websocket"

# Run specific test modules
cargo test --all-features common
cargo test --features http redis
cargo test --features websocket redis_ws

# Run specific operation types
cargo test --features http redis::string
cargo test --features http redis::set
cargo test --features websocket redis_ws::string
cargo test --features websocket redis_ws::set

# Run specific test functions
cargo test --all-features test_string_operations
cargo test --features http test_http_client_creation
```

### Test Categories by Operation Type

#### String Operations Tests

- Basic operations (get, set, delete)
- Batch operations (batch_get, batch_set)
- Pattern-based searches
- TTL operations
- Large payload handling
- Concurrent operations
- Error handling
- Connection reuse (WebSocket)

#### Set Operations Tests

- Basic operations (add, remove, exists, members, cardinality)
- Multiple set operations (intersection, union, difference)
- Large set handling
- Duplicate handling
- Empty set operations
- Concurrent operations
- Error handling

## Test Environment

### Environment Variables

- `TEST_HTTP_URL`: HTTP server URL (default: `http://localhost:8080`)
- `TEST_WS_URL`: WebSocket server URL (default: `ws://localhost:8080/ws`)

### Prerequisites

1. **Running DBX Server**: Tests require a running DBX Redis server
2. **Network Access**: Tests need network access to the test endpoints
3. **Dependencies**: All required dependencies are specified in `Cargo.toml`

## Test Utilities

### Key Generation

```rust
use crate::utils;

let test_key = utils::unique_key("my_test");
// Generates: my_test_1703123456789_12345
```

### URL Configuration

```rust
use crate::utils;

let http_url = utils::http_test_url();
let ws_url = utils::ws_test_url();
```

## Test Patterns

### 1. Setup and Teardown

```rust
#[tokio::test]
async fn test_example() -> Result<()> {
    // Setup
    let client = HttpClient::new(&utils::http_test_url())?;
    let test_key = utils::unique_key("test");

    // Test operations
    client.string().set(&test_key, "value", None).await?;
    let result = client.string().get(&test_key).await?;
    assert_eq!(result, Some("value".to_string()));

    // Cleanup
    client.string().delete(&test_key).await?;

    Ok(())
}
```

### 2. Error Testing

```rust
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    // Test invalid URLs
    let invalid_client = HttpClient::new("http://invalid-url");
    assert!(invalid_client.is_err());

    // Test non-existent keys
    let client = HttpClient::new(&utils::http_test_url())?;
    let result = client.string().get("non_existent").await?;
    assert_eq!(result, None);

    Ok(())
}
```

### 3. Concurrent Testing

```rust
#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    let client = HttpClient::new(&utils::http_test_url())?;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let mut client = client.clone();
            tokio::spawn(async move {
                // Concurrent operations
                Ok::<(), DbxError>(())
            })
        })
        .collect();

    for handle in handles {
        handle.await.map_err(|e| DbxError::Other(anyhow::anyhow!("{}", e)))??;
    }

    Ok(())
}
```

## Test Organization Benefits

### 1. Easy Navigation

- Find string-related tests in `string.rs` files
- Find set-related tests in `set.rs` files
- Client-specific tests in `mod.rs` files

### 2. Focused Testing

- Run only string operations: `cargo test --features http redis::string`
- Run only set operations: `cargo test --features websocket redis_ws::set`
- Run only client tests: `cargo test --features http redis`

### 3. Maintainability

- Related tests are grouped together
- Easy to add new test cases to the appropriate category
- Clear separation of concerns

## Troubleshooting

### Common Issues

1. **Connection Errors**: Ensure the DBX server is running
2. **Feature Flag Errors**: Check that required features are enabled
3. **Key Conflicts**: Use `utils::unique_key()` for unique test keys

### Debug Mode

```bash
cargo test --all-features -- --nocapture
```

### Single Test Debugging

```bash
cargo test test_specific_function --all-features -- --nocapture
```
