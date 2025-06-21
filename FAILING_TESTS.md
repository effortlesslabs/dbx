# Failing Test Cases Tracking

This document tracks the failing test cases that need to be fixed. Tests are organized by category and include status tracking.

## Test Status Legend

- ❌ **FAILING** - Test is currently failing and needs to be fixed
- 🔄 **IN PROGRESS** - Test is being worked on
- ✅ **FIXED** - Test has been fixed and is passing
- ⏸️ **SKIPPED** - Test is temporarily skipped for investigation

---

## Redis Batch Tests

### Batch Operations

- ✅ `test_redis_batch_comprehensive_operations` - Batch comprehensive operations test
- ✅ `test_redis_batch_concurrent_operations` - Batch concurrent operations test
- ✅ `test_redis_batch_edge_cases` - Batch edge cases test

**File**: `api/tests/redis/batch.rs`

**Status**: ✅ **ALL FIXED** - All batch tests are now passing!

**Fixes Applied**:

- Set initial values to numeric strings for keys that will be incremented
- Fixed floating-point values to integers (Redis INCRBY only works with integers)
- Improved error handling in batch_incr endpoint to return 400 Bad Request instead of 500 Internal Server Error

---

## Redis Concurrent Tests

### Concurrent Operations

- ❌ `test_redis_concurrent_comprehensive_operations` - Comprehensive concurrent operations
- ❌ `test_redis_concurrent_hash_operations` - Concurrent hash operations
- ❌ `test_redis_concurrent_set_operations` - Concurrent set operations
- ❌ `test_redis_concurrent_stress_test` - Concurrent stress testing

**File**: `api/tests/redis/concurrent.rs`

---

## Redis Error Tests

### Error Handling

- ❌ `test_redis_concurrent_error_scenarios` - Concurrent error scenarios
- ❌ `test_redis_edge_case_errors` - Edge case error handling
- ❌ `test_redis_error_handling_comprehensive` - Comprehensive error handling
- ❌ `test_redis_invalid_json_requests` - Invalid JSON request handling
- ❌ `test_redis_invalid_keys` - Invalid key handling
- ❌ `test_redis_set_operation_errors` - Set operation error handling

**File**: `api/tests/redis/errors.rs`

---

## Redis Hash Tests

### Hash Operations

- ❌ `test_redis_hash_basic_operations` - Basic hash operations
- ❌ `test_redis_hash_batch_operations` - Hash batch operations
- ❌ `test_redis_hash_comprehensive_operations` - Comprehensive hash operations
- ❌ `test_redis_hash_edge_cases` - Hash edge cases
- ❌ `test_redis_hash_error_cases` - Hash error cases
- ❌ `test_redis_hash_multiple_fields_operations` - Multiple fields hash operations

**File**: `api/tests/redis/hashes.rs`

---

## Progress Tracking

### Completed Fixes

- None yet

### In Progress

- None currently

### Next Priority

1. Start with basic operations tests (hash basic operations)
2. Move to error handling tests
3. Address concurrent and batch operations
4. Handle edge cases and stress tests

---

## Notes

- Total failing tests: **20**
- Tests organized by complexity and dependency order
- Focus on basic operations first before moving to complex scenarios
- Consider running tests individually to isolate issues

---

## Running Individual Tests

To run a specific failing test:

```bash
# Run a specific test
cargo test test_redis_hash_basic_operations

# Run tests in a specific module
cargo test redis::hashes

# Run with verbose output
cargo test test_redis_hash_basic_operations -- --nocapture
```

---

_Last Updated: [Current Date]_
_Total Tests Tracked: 20_
