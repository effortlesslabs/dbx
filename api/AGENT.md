# API Agent Implementation Guide

## Overview

This document provides instructions for implementing new Redis data type features following the existing **crates -> redis -> redis_ws** pattern. The API currently has string operations implemented and uses the Redis adapter from `crates/adapter/redis/` module.

## Current Implementation Status

### ✅ **Already Implemented**

- **String Operations**: Complete HTTP REST API and WebSocket implementation
- **Redis Primitives Available**: string, hash, set, admin (in `crates/adapter/redis/primitives/`)

### 🔄 **To Be Implemented**

- **Hash Operations**: HTTP REST API and WebSocket endpoints
- **Set Operations**: HTTP REST API and WebSocket endpoints
- **List Operations**: HTTP REST API and WebSocket endpoints (primitives not yet available)
- **Sorted Set Operations**: HTTP REST API and WebSocket endpoints (primitives not yet available)

## Existing Code Structure Analysis

### 1. **Crates Layer (Foundation)** - `crates/adapter/redis/`

**Available Primitives:**

- `crates/adapter/redis/primitives/string.rs` - ✅ Implemented and used
- `crates/adapter/redis/primitives/hash.rs` - ✅ Available, not yet used in API
- `crates/adapter/redis/primitives/set.rs` - ✅ Available, not yet used in API
- `crates/adapter/redis/primitives/admin.rs` - ✅ Available, not yet used in API

### 2. **Common Layer** - `api/src/routes/common/`

**Current Structure:**

- `api/src/routes/common/mod.rs` - Declares string module
- `api/src/routes/common/string.rs` - ✅ String operations implementation

**Pattern to Follow:**

- Create `api/src/routes/common/hash.rs` for hash operations
- Create `api/src/routes/common/set.rs` for set operations
- Update `api/src/routes/common/mod.rs` to include new modules

### 3. **HTTP REST API Layer** - `api/src/routes/redis/`

**Current Structure:**

- `api/src/routes/redis/mod.rs` - Declares string module
- `api/src/routes/redis/string.rs` - ✅ String HTTP endpoints

**Pattern to Follow:**

- Create `api/src/routes/redis/hash.rs` for hash HTTP endpoints
- Create `api/src/routes/redis/set.rs` for set HTTP endpoints
- Update `api/src/routes/redis/mod.rs` to include new modules

### 4. **WebSocket API Layer** - `api/src/routes/redis_ws/`

**Current Structure:**

- `api/src/routes/redis_ws/mod.rs` - Declares string module
- `api/src/routes/redis_ws/string.rs` - ✅ String WebSocket endpoints

**Pattern to Follow:**

- Create `api/src/routes/redis_ws/hash.rs` for hash WebSocket endpoints
- Create `api/src/routes/redis_ws/set.rs` for set WebSocket endpoints
- Update `api/src/routes/redis_ws/mod.rs` to include new modules

### 5. **Server Integration** - `api/src/server.rs`

**Current Integration:**

- Lines 58-65: Redis string routes are integrated
- Uses `RedisPool` from `dbx_crates::adapter::redis::client::RedisPool`

## Implementation Instructions for New Features

### Step 1: Implement Hash Operations

#### 1.1 Create Common Hash Operations

**File:** `api/src/routes/common/hash.rs`
**Template:** Follow the pattern from `api/src/routes/common/string.rs`

**Required Functions:**

- `get_hash_field(conn, key, field)` - Get single hash field
- `set_hash_field(conn, key, field, value)` - Set single hash field
- `get_all_hash_fields(conn, key)` - Get all hash fields
- `set_multiple_hash_fields(conn, key, fields)` - Set multiple hash fields
- `delete_hash_field(conn, key, field)` - Delete hash field
- `hash_exists(conn, key, field)` - Check if hash field exists
- `get_hash_length(conn, key)` - Get hash length

**Data Structures:**

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashOperation {
    pub key: String,
    pub field: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashInfo {
    pub key: String,
    pub field: String,
    pub value: String,
    pub ttl: Option<i64>,
}
```

#### 1.2 Create Hash HTTP Routes

**File:** `api/src/routes/redis/hash.rs`
**Template:** Follow the pattern from `api/src/routes/redis/string.rs`

**Required Endpoints:**

- `GET /redis/hash/:key/:field` - Get hash field
- `POST /redis/hash/:key/:field` - Set hash field
- `GET /redis/hash/:key` - Get all hash fields
- `POST /redis/hash/:key/batch` - Set multiple hash fields
- `DELETE /redis/hash/:key/:field` - Delete hash field
- `GET /redis/hash/:key/:field/exists` - Check if hash field exists

#### 1.3 Create Hash WebSocket Routes

**File:** `api/src/routes/redis_ws/hash.rs`
**Template:** Follow the pattern from `api/src/routes/redis_ws/string.rs`

**Required WebSocket Messages:**

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum HashWsMessage {
    #[serde(rename = "get")] Get { key: String, field: String },
    #[serde(rename = "set")] Set { key: String, field: String, value: String },
    #[serde(rename = "get_all")] GetAll { key: String },
    #[serde(rename = "del")] Del { key: String, field: String },
    #[serde(rename = "exists")] Exists { key: String, field: String },
    // ... result messages
}
```

### Step 2: Implement Set Operations

#### 2.1 Create Common Set Operations

**File:** `api/src/routes/common/set.rs`
**Template:** Follow the pattern from `api/src/routes/common/string.rs`

**Required Functions:**

- `add_to_set(conn, key, member)` - Add member to set
- `remove_from_set(conn, key, member)` - Remove member from set
- `get_set_members(conn, key)` - Get all set members
- `set_exists(conn, key, member)` - Check if member exists in set
- `get_set_cardinality(conn, key)` - Get set size
- `intersect_sets(conn, keys)` - Intersect multiple sets
- `union_sets(conn, keys)` - Union multiple sets
- `difference_sets(conn, keys)` - Difference of sets

#### 2.2 Create Set HTTP Routes

**File:** `api/src/routes/redis/set.rs`
**Template:** Follow the pattern from `api/src/routes/redis/string.rs`

**Required Endpoints:**

- `POST /redis/set/:key` - Add member to set
- `DELETE /redis/set/:key/:member` - Remove member from set
- `GET /redis/set/:key/members` - Get all set members
- `GET /redis/set/:key/cardinality` - Get set size
- `POST /redis/set/intersect` - Intersect sets
- `POST /redis/set/union` - Union sets
- `POST /redis/set/difference` - Difference of sets

#### 2.3 Create Set WebSocket Routes

**File:** `api/src/routes/redis_ws/set.rs`
**Template:** Follow the pattern from `api/src/routes/redis_ws/string.rs`

### Step 3: Update Module Declarations

#### 3.1 Update Common Module

**File:** `api/src/routes/common/mod.rs`

```rust
pub mod string;
pub mod hash;  // Add this
pub mod set;   // Add this
```

#### 3.2 Update Redis HTTP Module

**File:** `api/src/routes/redis/mod.rs`

```rust
pub mod string;
pub mod hash;  // Add this
pub mod set;   // Add this
```

#### 3.3 Update Redis WebSocket Module

**File:** `api/src/routes/redis_ws/mod.rs`

```rust
pub mod string;
pub mod hash;  // Add this
pub mod set;   // Add this
```

### Step 4: Update Server Integration

#### 4.1 Update Server Router

**File:** `api/src/server.rs`
**Lines 58-65:** Add new route integrations following the existing pattern:

```rust
// Add Redis admin routes if Redis pool is available
if let Some(pool) = &self.redis_pool {
    let redis_string_routes = crate::routes::redis::string::create_redis_string_routes(pool.clone());
    let redis_hash_routes = crate::routes::redis::hash::create_redis_hash_routes(pool.clone());  // Add this
    let redis_set_routes = crate::routes::redis::set::create_redis_set_routes(pool.clone());    // Add this

    let redis_ws_string_routes = crate::routes::redis_ws::string::create_redis_ws_string_routes(pool.clone());
    let redis_ws_hash_routes = crate::routes::redis_ws::hash::create_redis_ws_hash_routes(pool.clone());  // Add this
    let redis_ws_set_routes = crate::routes::redis_ws::set::create_redis_ws_set_routes(pool.clone());    // Add this

    router = router
        .nest("/redis", redis_string_routes)
        .nest("/redis", redis_hash_routes)    // Add this
        .nest("/redis", redis_set_routes)     // Add this
        .nest("/redis_ws", redis_ws_string_routes)
        .nest("/redis_ws", redis_ws_hash_routes)  // Add this
        .nest("/redis_ws", redis_ws_set_routes);  // Add this
}
```

## Testing Implementation

### Step 1: Create Test Files

#### 1.1 HTTP Tests

**Files to Create:**

- `api/tests/redis/hash.rs` - Hash HTTP endpoint tests
- `api/tests/redis/set.rs` - Set HTTP endpoint tests

**Template:** Follow the pattern from existing test structure in `api/tests/mod.rs`

#### 1.2 WebSocket Tests

**Files to Create:**

- `api/tests/redis_ws/hash.rs` - Hash WebSocket tests
- `api/tests/redis_ws/set.rs` - Set WebSocket tests

**Template:** Follow the pattern from existing test structure

### Step 2: Update Test Modules

#### 2.1 Update Redis Tests Module

**File:** `api/tests/redis/mod.rs`

```rust
pub mod string;
pub mod hash;  // Add this
pub mod set;   // Add this
```

#### 2.2 Update Redis WebSocket Tests Module

**File:** `api/tests/redis_ws/mod.rs`

```rust
pub mod string;
pub mod hash;  // Add this
pub mod set;   // Add this
```

## Key Implementation Patterns

### 1. **Common Operations Pattern**

- Use `Arc<Mutex<Connection>>` for thread-safe Redis connections
- Return `redis::RedisResult<T>` for error handling
- Use the corresponding Redis primitive from `crates/adapter/redis/primitives/`

### 2. **HTTP Route Pattern**

- Use `State<Arc<RedisPool>>` for connection pool
- Return `Result<Json<T>, StatusCode>` for responses
- Use `Path` and `Json` extractors for parameters
- Follow the exact handler pattern from `api/src/routes/redis/string.rs`

### 3. **WebSocket Pattern**

- Use `WebSocketUpgrade` and `WebSocket` for WebSocket handling
- Define enum messages with `#[serde(tag = "type", content = "data")]`
- Use `futures::{StreamExt, SinkExt}` for async WebSocket operations
- Follow the exact pattern from `api/src/routes/redis_ws/string.rs`

### 4. **Error Handling Pattern**

- Use `StatusCode::INTERNAL_SERVER_ERROR` for Redis errors
- Use `map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?` pattern
- Follow the error handling pattern from existing string operations

## Implementation Checklist

### Hash Operations

- [ ] Create `api/src/routes/common/hash.rs` with all hash operations
- [ ] Create `api/src/routes/redis/hash.rs` with HTTP endpoints
- [ ] Create `api/src/routes/redis_ws/hash.rs` with WebSocket handlers
- [ ] Update `api/src/routes/common/mod.rs` to include hash module
- [ ] Update `api/src/routes/redis/mod.rs` to include hash module
- [ ] Update `api/src/routes/redis_ws/mod.rs` to include hash module
- [ ] Update `api/src/server.rs` to integrate hash routes
- [ ] Create `api/tests/redis/hash.rs` with HTTP tests
- [ ] Create `api/tests/redis_ws/hash.rs` with WebSocket tests
- [ ] Update test modules to include hash tests

### Set Operations

- [ ] Create `api/src/routes/common/set.rs` with all set operations
- [ ] Create `api/src/routes/redis/set.rs` with HTTP endpoints
- [ ] Create `api/src/routes/redis_ws/set.rs` with WebSocket handlers
- [ ] Update `api/src/routes/common/mod.rs` to include set module
- [ ] Update `api/src/routes/redis/mod.rs` to include set module
- [ ] Update `api/src/routes/redis_ws/mod.rs` to include set module
- [ ] Update `api/src/server.rs` to integrate set routes
- [ ] Create `api/tests/redis/set.rs` with HTTP tests
- [ ] Create `api/tests/redis_ws/set.rs` with WebSocket tests
- [ ] Update test modules to include set tests

## Existing Files Reference

### Core Implementation Files

- `api/src/routes/common/string.rs` - Template for common operations
- `api/src/routes/redis/string.rs` - Template for HTTP endpoints
- `api/src/routes/redis_ws/string.rs` - Template for WebSocket handlers
- `api/src/server.rs` - Server integration pattern
- `api/tests/mod.rs` - Test server setup pattern

### Redis Primitives (Available)

- `crates/adapter/redis/primitives/string.rs` - String operations
- `crates/adapter/redis/primitives/hash.rs` - Hash operations
- `crates/adapter/redis/primitives/set.rs` - Set operations
- `crates/adapter/redis/primitives/admin.rs` - Admin operations

### Configuration Files

- `api/src/config.rs` - Configuration structure
- `api/src/models.rs` - API response models
- `api/src/constants/errors.rs` - Error constants

This implementation guide follows the exact patterns established in the existing codebase, ensuring consistency and maintainability when adding new Redis data type support.
