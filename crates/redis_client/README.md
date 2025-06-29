# Redis Client SDK

A comprehensive Rust SDK for interacting with the DBX Redis API. This SDK provides a high-level, idiomatic Rust interface for Redis operations through both HTTP and WebSocket protocols.

## Overview

The Redis Client SDK offers a modern, async-first approach to Redis operations with strong type safety and comprehensive error handling. It supports both HTTP REST API and WebSocket real-time communication, making it suitable for various use cases from simple caching to real-time applications.

## Features

### Core Capabilities

- **Dual Protocol Support**: HTTP REST API and WebSocket real-time communication
- **String Operations**: Complete CRUD operations for Redis strings with TTL support
- **Set Operations**: Full set manipulation including membership, cardinality, and set algebra
- **Batch Operations**: Efficient bulk operations for improved performance
- **Pattern Matching**: Advanced pattern-based retrieval for complex queries

### Technical Features

- **Async/Await**: Built on Tokio for high-performance async operations
- **Type Safety**: Strongly typed API with compile-time guarantees
- **Error Handling**: Comprehensive error types with detailed context
- **Connection Management**: Automatic connection pooling and reuse
- **Timeout Support**: Configurable timeouts for all operations

### String Operations

- Get, set, and delete individual string values
- Set values with configurable time-to-live (TTL)
- Batch operations for multiple keys
- Pattern-based retrieval with wildcard support
- String metadata and information retrieval

### Set Operations

- Add and remove individual or multiple members
- Check membership and retrieve cardinality
- Get all members of a set
- Set algebra operations: intersection, union, and difference
- Efficient batch operations for large datasets

## Architecture

### Client Types

- **HttpClient**: Traditional REST API client for standard operations
- **WsClient**: WebSocket client for real-time, bidirectional communication

### Error Handling

The SDK provides a unified error handling system with detailed error types:

- API errors with HTTP status codes and messages
- Network and connection errors
- Serialization and parsing errors
- Timeout and configuration errors

### Connection Management

- Automatic connection establishment and cleanup
- Connection pooling for HTTP clients
- Persistent WebSocket connections with automatic reconnection
- Configurable timeouts and retry logic

## Installation

Add the SDK to your project dependencies:

```toml
[dependencies]
redis_client = { path = "../crates/redis_client" }
tokio = { version = "1.0", features = ["full"] }
```

## Usage Overview

### HTTP Client

The HTTP client provides traditional REST API access with automatic connection management and efficient request handling. It's ideal for standard Redis operations where real-time communication isn't required.

### WebSocket Client

The WebSocket client enables real-time, bidirectional communication with the Redis API. It's perfect for applications requiring live updates, event-driven architectures, or low-latency operations.

### Error Handling

All operations return a `Result` type that provides detailed error information, allowing for robust error handling and recovery strategies.

### Batch Operations

The SDK supports efficient batch operations for both strings and sets, reducing network overhead and improving performance for bulk operations.

## Performance Considerations

- **Connection Reuse**: Both client types efficiently reuse connections
- **Batch Operations**: Use batch operations for multiple related operations
- **Async Operations**: All operations are async for optimal resource utilization
- **Memory Efficiency**: Streaming responses for large datasets

## Testing

The SDK includes comprehensive test suites covering both HTTP and WebSocket protocols. The tests are organized by protocol and operation type to ensure complete coverage.

### Running All Tests

To run the complete test suite for both HTTP and WebSocket clients:

```bash
# From the redis_client directory
cd crates/redis_client
cargo test

# Or from the project root
cargo test -p redis_client
```

### Testing HTTP Client

The HTTP client tests cover all REST API operations:

```bash
# Test only HTTP operations
cargo test --features http

# Test specific HTTP string operations
cargo test redis::string

# Test specific HTTP set operations
cargo test redis::set

# Test HTTP client creation and configuration
cargo test redis::test_http_client
```

### Testing WebSocket Client

The WebSocket client tests require the Redis API server to be running:

```bash
# Start the Redis API server first
cd crates/redis_api
cargo run

# In another terminal, test WebSocket operations
cd crates/redis_client
cargo test --features websocket

# Test specific WebSocket string operations
cargo test redis_ws::string

# Test specific WebSocket set operations
cargo test redis_ws::set

# Test WebSocket client creation and connection
cargo test redis_ws::test_websocket_client
```

### Testing Specific Features

```bash
# Test only string operations (both HTTP and WebSocket)
cargo test --features string

# Test only set operations (both HTTP and WebSocket)
cargo test --features set

# Test with all features enabled
cargo test --features http,websocket,string,set

# Test with verbose output
cargo test -- --nocapture

# Test with specific test pattern
cargo test test_websocket_string_operations
```

### Integration Testing

For integration tests that verify end-to-end functionality:

```bash
# Run integration tests with server
cd crates/redis_client
cargo test --test integration

# Run tests with specific server URL
RUST_LOG=debug cargo test -- --nocapture
```

### Test Coverage

The test suite includes:

- **Unit Tests**: Individual function and method testing
- **Integration Tests**: End-to-end API interaction testing
- **WebSocket Tests**: Connection, message handling, and real-time communication
- **Error Handling**: Comprehensive error scenario testing
- **Performance Tests**: Large payload and concurrent operation testing
- **Edge Cases**: Boundary conditions and error recovery testing

### Debugging Tests

For debugging test failures:

```bash
# Run with debug logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific failing test
cargo test test_websocket_error_handling -- --nocapture

# Run tests with timeout
cargo test -- --timeout 30
```

## License

MIT License - see the main project license for details.
