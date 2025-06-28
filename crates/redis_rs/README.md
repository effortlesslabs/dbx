# Redis RS SDK

A Rust SDK for interacting with the DBX Redis API. This SDK provides a high-level, idiomatic Rust interface for Redis operations through the DBX API.

## Features

- **String Operations**: Get, set, delete, and batch operations for Redis strings
- **Set Operations**: Add, remove, check membership, and set operations (intersect, union, difference)
- **Async/Await**: Full async support with Tokio
- **Error Handling**: Comprehensive error types with detailed error messages
- **Type Safety**: Strongly typed API with compile-time guarantees
- **Batch Operations**: Efficient batch operations for multiple keys

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
redis_rs = { path = "../crates/redis_rs" }
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use redis_rs::DbxClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = DbxClient::new("http://localhost:8080")?;

    // String operations
    client.string().set_simple("my_key", "my_value").await?;
    let value = client.string().get("my_key").await?;
    println!("Value: {:?}", value);

    // Set operations
    client.set().add_one("my_set", "member1").await?;
    let members = client.set().members("my_set").await?;
    println!("Members: {:?}", members);

    Ok(())
}
```

## API Reference

### Client Creation

```rust
// Basic client
let client = DbxClient::new("http://localhost:8080")?;

// Client with custom timeout
let client = DbxClient::with_timeout("http://localhost:8080", Duration::from_secs(60))?;
```

### String Operations

#### Basic Operations

```rust
let string_client = client.string();

// Set a string value
string_client.set_simple("key", "value").await?;

// Set with TTL (time to live in seconds)
string_client.set_with_ttl("key", "value", 3600).await?;

// Get a string value
let value = string_client.get("key").await?;
// Returns Option<String> - None if key doesn't exist

// Delete a string
let deleted = string_client.delete("key").await?;
// Returns bool indicating if key was deleted

// Get string information
let info = string_client.info("key").await?;
// Returns Option<StringInfo> with metadata
```

#### Batch Operations

```rust
// Batch get multiple strings
let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
let values = string_client.batch_get(&keys).await?;
// Returns Vec<Option<String>>

// Batch set multiple strings
let operations = vec![
    StringOperation {
        key: "key1".to_string(),
        value: Some("value1".to_string()),
        ttl: None,
    },
    StringOperation {
        key: "key2".to_string(),
        value: Some("value2".to_string()),
        ttl: Some(3600),
    },
];
string_client.batch_set(&operations).await?;

// Get strings by patterns
let patterns = vec!["user:*".to_string(), "session:*".to_string()];
let results = string_client.get_by_patterns(&patterns, Some(true)).await?;
// Returns serde_json::Value with grouped results
```

### Set Operations

#### Basic Operations

```rust
let set_client = client.set();

// Add a single member
let added = set_client.add_one("set_key", "member1").await?;
// Returns usize (number of new members added)

// Add multiple members
let added = set_client.add_many("set_key", &["member1", "member2", "member3"]).await?;

// Remove a member
let removed = set_client.remove("set_key", "member1").await?;
// Returns usize (number of members removed)

// Get all members
let members = set_client.members("set_key").await?;
// Returns Vec<String>

// Check if member exists
let exists = set_client.contains("set_key", "member1").await?;
// Returns bool

// Get set size (cardinality)
let size = set_client.size("set_key").await?;
// Returns usize
```

#### Set Operations

```rust
// Intersect multiple sets
let keys = vec!["set1".to_string(), "set2".to_string(), "set3".to_string()];
let intersection = set_client.intersect(&keys).await?;
// Returns Vec<String> of common members

// Union multiple sets
let union = set_client.union(&keys).await?;
// Returns Vec<String> of all unique members

// Difference of sets (first set minus others)
let difference = set_client.difference(&keys).await?;
// Returns Vec<String> of members in first set but not in others
```

## Error Handling

The SDK uses a custom `DbxError` type that provides detailed error information:

```rust
use redis_rs::{DbxError, Result};

async fn handle_errors() -> Result<()> {
    let client = DbxClient::new("http://localhost:8080")?;

    match client.string().get("nonexistent_key").await {
        Ok(value) => println!("Value: {:?}", value),
        Err(DbxError::Api { status, message }) => {
            println!("API Error {}: {}", status, message);
        }
        Err(DbxError::Request(e)) => {
            println!("Network error: {}", e);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    Ok(())
}
```

## Types

### String Types

- `StringOperation`: Represents a string operation for batch operations
- `StringInfo`: Contains metadata about a string value
- `SetStringRequest`: Request payload for setting strings
- `BatchGetRequest`: Request payload for batch getting
- `BatchSetRequest`: Request payload for batch setting
- `BatchGetPatternsRequest`: Request payload for pattern-based getting

### Set Types

- `SetOperation`: Represents a set operation for batch operations
- `SetInfo`: Contains metadata about a set
- `SetMemberRequest`: Request payload for adding a member
- `SetMembersRequest`: Request payload for adding multiple members
- `SetKeysRequest`: Request payload for set operations

## Examples

### Complete Example

```rust
use redis_rs::{DbxClient, StringOperation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DbxClient::new("http://localhost:8080")?;

    // String operations
    println!("=== String Operations ===");

    // Set some values
    client.string().set_simple("user:1:name", "Alice").await?;
    client.string().set_with_ttl("user:1:session", "abc123", 3600).await?;

    // Get values
    let name = client.string().get("user:1:name").await?;
    println!("User name: {:?}", name);

    let session = client.string().get("user:1:session").await?;
    println!("Session: {:?}", session);

    // Batch operations
    let operations = vec![
        StringOperation {
            key: "user:2:name".to_string(),
            value: Some("Bob".to_string()),
            ttl: None,
        },
        StringOperation {
            key: "user:2:email".to_string(),
            value: Some("bob@example.com".to_string()),
            ttl: None,
        },
    ];
    client.string().batch_set(&operations).await?;

    // Set operations
    println!("\n=== Set Operations ===");

    // Create sets
    client.set().add_many("users:online", &["user:1", "user:2", "user:3"]).await?;
    client.set().add_many("users:premium", &["user:1", "user:4"]).await?;

    // Get members
    let online_users = client.set().members("users:online").await?;
    println!("Online users: {:?}", online_users);

    // Check membership
    let is_premium = client.set().contains("users:premium", "user:1").await?;
    println!("User 1 is premium: {}", is_premium);

    // Set operations
    let premium_online = client.set().intersect(&[
        "users:online".to_string(),
        "users:premium".to_string(),
    ]).await?;
    println!("Premium online users: {:?}", premium_online);

    Ok(())
}
```

## Testing

The SDK includes comprehensive tests. Run them with:

```bash
cd crates/redis_rs
cargo test
```

## License

MIT License - see the main project license for details.
