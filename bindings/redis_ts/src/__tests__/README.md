# Redis TypeScript Bindings Tests

This directory contains comprehensive test suites for the Redis TypeScript bindings using Vitest.

## Test Structure

### Files

- **`setup.ts`** - Test configuration and utilities
- **`http-client.test.ts`** - HTTP client tests
- **`websocket-client.test.ts`** - WebSocket client tests
- **`integration.test.ts`** - Cross-protocol integration tests

### Test Coverage

#### HTTP Client Tests (`http-client.test.ts`)

- ✅ Client creation and configuration
- ✅ String operations (set, get, delete, info, batch operations)
- ✅ Set operations (add, remove, members, cardinality, set operations)

#### WebSocket Client Tests (`websocket-client.test.ts`)

- ✅ WebSocket client creation and configuration
- ✅ WebSocket string operations
- ✅ WebSocket set operations

#### Integration Tests (`integration.test.ts`)

- ✅ Cross-protocol operations (HTTP ↔ WebSocket)
- ✅ Error handling and edge cases
- ✅ Performance comparisons

## Running Tests

### Prerequisites

1. **Build the bindings first:**

   ```bash
   npm run build
   ```

2. **Set up test environment variables (optional):**
   ```bash
   export TEST_HTTP_URL="http://localhost:8080"
   export TEST_WS_URL="ws://localhost:8080/ws"
   ```

### Test Commands

```bash
# Run all tests
npm test

# Run tests once (CI mode)
npm run test:run

# Run tests with UI
npm run test:ui

# Run specific test file
npx vitest http-client.test.ts

# Run tests in watch mode
npx vitest --watch
```

## Test Configuration

The tests use the following configuration (from `setup.ts`):

```typescript
export const TEST_CONFIG = {
  HTTP_URL: process.env.TEST_HTTP_URL || "http://localhost:8080",
  WS_URL: process.env.TEST_WS_URL || "ws://localhost:8080/ws",
};
```

## Test Utilities

### Key Generation

```typescript
generateTestKey(prefix?: string): string
// Generates unique test keys with timestamps
```

### Value Generation

```typescript
generateTestValue(): string
// Generates unique test values with timestamps
```

## Test Patterns

### 1. Basic Operation Testing

```typescript
it("should set and get a string value", async () => {
  const key = generateTestKey("string");
  const value = generateTestValue();

  const setResult = await stringClient.set(key, value);
  expect(setResult).toBe(true);

  const getResult = await stringClient.get(key);
  expect(getResult).toBe(value);
});
```

### 2. Cross-Protocol Testing

```typescript
it("should set via HTTP and get via WebSocket", async () => {
  // Set via HTTP
  await httpClient.string().set(key, value);

  // Get via WebSocket
  const result = await wsClient.string().get(key);
  expect(result).toBe(value);
});
```

### 3. Error Handling

```typescript
it("should handle connection errors gracefully", async () => {
  try {
    const client = createClient("http://invalid-url:9999");
    await client.string().get("test");
    expect(true).toBe(false); // Should not reach here
  } catch (error) {
    expect(error).toBeDefined();
  }
});
```

## Expected Test Results

### ✅ Success Cases

- All CRUD operations work correctly
- Return values match expected types (boolean for operations, actual data for queries)
- Cross-protocol consistency
- Proper error handling

### ⚠️ Known Limitations

- Tests require a running Redis server
- WebSocket tests may be slower due to connection overhead
- Performance tests show relative timing, not absolute benchmarks

## Troubleshooting

### Common Issues

1. **"Cannot find module" errors**

   - Ensure bindings are built: `npm run build`
   - Check that `index.js` exists in the root directory

2. **Connection errors**

   - Verify Redis server is running
   - Check `TEST_HTTP_URL` and `TEST_WS_URL` environment variables
   - Ensure firewall/network allows connections

3. **Timeout errors**
   - Increase test timeout in `vitest.config.ts`
   - Check server responsiveness

### Debug Mode

Run tests with verbose output:

```bash
npx vitest --reporter=verbose
```

## Contributing

When adding new tests:

1. Use `generateTestKey()` and `generateTestValue()` for unique test data
2. Test both HTTP and WebSocket clients
3. Include error cases and edge cases
4. Add integration tests for cross-protocol scenarios
5. Update this README if adding new test patterns
