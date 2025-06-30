# DBX Testing Workflow

This document explains the new testing workflow that ensures all crate tests run against a properly configured DBX server with the correct environment variables.

## 🎯 Overview

The new testing workflow addresses the requirement that all crate tests need to run against a running DBX server with proper environment configuration. This ensures that:

1. **Server is running** - Tests run against an actual DBX API server
2. **Environment is set** - All required environment variables are properly configured
3. **Dependencies are met** - Redis is available and server is healthy
4. **Tests are sequential** - Tests run in the correct dependency order (adapter → api → client)

## 📁 New Files

### Scripts

- **`scripts/test-with-server.sh`** - Complete test runner with server setup
- **`scripts/test-simple.sh`** - Simple test runner for existing server

### Workflows

- **`.github/workflows/crates-tests.yml`** - Updated workflow using test-with-server.sh
- **`.github/workflows/crates-tests-simple.yml`** - Alternative workflow using test-simple.sh

## 🚀 Usage

### Local Development

#### Option 1: Complete Setup (Recommended)

```bash
# Run tests with automatic server setup
./scripts/test-with-server.sh

# With custom environment
./scripts/test-with-server.sh --env-file .env.test --redis-url redis://localhost:6379

# Keep server running for debugging
./scripts/test-with-server.sh --skip-cleanup
```

#### Option 2: Manual Server + Simple Tests

```bash
# Start server manually
cargo run -p redis_api --release &
SERVER_PID=$!

# Wait for server
sleep 5

# Run tests
./scripts/test-simple.sh

# Stop server
kill $SERVER_PID
```

#### Option 3: Existing Server

```bash
# If you have a server running elsewhere
./scripts/test-simple.sh --server-url http://localhost:3000 --redis-url redis://localhost:6379
```

### CI/CD Pipeline

The GitHub Actions workflows automatically:

1. **Set up Redis** - Uses GitHub Actions Redis service
2. **Create environment** - Generates `.env` file with proper configuration
3. **Start server** - Builds and starts DBX API server
4. **Run tests** - Executes all crate tests against the running server
5. **Clean up** - Stops server and cleans up resources

## 🔧 Configuration

### Environment Variables

The scripts automatically set these environment variables for tests:

```bash
# Database
REDIS_URL=redis://localhost:6379

# Server
DBX_BASE_URL=http://localhost:3000
DBX_WS_HOST_URL=ws://localhost:3000/redis_ws

# Optional (from .env file)
HOST=0.0.0.0
PORT=3000
POOL_SIZE=10
LOG_LEVEL=INFO
```

### .env File

Create a `.env` file for custom configuration:

```bash
# Database Configuration
REDIS_URL=redis://localhost:6379

# Server Configuration
HOST=0.0.0.0
PORT=3000
POOL_SIZE=10

# Logging Configuration
LOG_LEVEL=INFO
```

## 📋 Test Execution Order

All tests run in this sequential order to respect dependencies:

1. **Adapter tests** (`crates/adapter`) - Foundation layer
2. **API tests** (`crates/redis_api`) - Depends on adapter
3. **Client tests** (`crates/redis_client`) - Depends on adapter and API

## 🛠️ Script Options

### test-with-server.sh

```bash
--env-file <path>       # Path to .env file (default: .env)
--redis-url <url>       # Redis connection URL (overrides .env)
--server-port <port>    # Server port (default: 3000)
--skip-server           # Skip starting server (assume it's already running)
--skip-cleanup          # Don't stop server after tests
--verbose               # Enable verbose output
--help                  # Show help message
```

### test-simple.sh

```bash
--redis-url <url>       # Redis connection URL (default: redis://localhost:6379)
--server-url <url>      # Server base URL (default: http://localhost:3000)
--verbose               # Enable verbose output
--help                  # Show help message
```

## 🔍 Troubleshooting

### Common Issues

#### Server Not Starting

```bash
# Check if port is in use
lsof -i :3000

# Check server logs
docker logs dbx-test-server

# Try different port
./scripts/test-with-server.sh --server-port 3001
```

#### Redis Connection Issues

```bash
# Check if Redis is running
docker ps | grep redis

# Test Redis connection
redis-cli ping

# Use different Redis URL
./scripts/test-with-server.sh --redis-url redis://localhost:6380
```

#### Test Failures

```bash
# Run with verbose output
./scripts/test-with-server.sh --verbose

# Check server health
curl http://localhost:3000/redis/admin/ping

# Run individual test suites
cd crates/adapter && cargo test
cd ../redis_api && cargo test
cd ../redis_client && cargo test
```

### Debug Mode

Enable debug output to see all commands:

```bash
# Set debug environment variable
DEBUG=true ./scripts/test-with-server.sh

# Or use verbose mode
./scripts/test-with-server.sh --verbose
```

## 🔄 Migration from Old Workflow

### Before (Old Workflow)

```yaml
- name: Run unit tests
  run: cargo test -p dbx-crates --features "${{ matrix.features }}" --verbose --lib
  env:
    REDIS_URL: redis://localhost:6379
```

### After (New Workflow)

```yaml
- name: Run tests with server
  run: |
    # Create .env file for testing
    cat > .env << EOF
    REDIS_URL=redis://localhost:6379
    HOST=0.0.0.0
    PORT=3000
    POOL_SIZE=10
    LOG_LEVEL=INFO
    EOF

    # Run tests using the script
    ./scripts/test-with-server.sh --verbose
  env:
    REDIS_URL: redis://localhost:6379
```

## 📊 Benefits

### ✅ Advantages

1. **Realistic Testing** - Tests run against actual server implementation
2. **Environment Consistency** - All tests use the same environment configuration
3. **Dependency Validation** - Tests verify actual integration between components
4. **CI/CD Ready** - Automated setup and teardown in CI environments
5. **Flexible Configuration** - Support for custom environments and configurations
6. **Error Isolation** - Clear separation between server issues and test issues
7. **Resource Management** - Automatic cleanup prevents resource leaks

### 🔧 Technical Improvements

1. **Server Health Checks** - Validates server is ready before running tests
2. **Container Management** - Proper Docker container lifecycle management
3. **Environment Variables** - Automatic setup of all required environment variables
4. **Sequential Execution** - Tests run in correct dependency order
5. **Error Handling** - Comprehensive error handling and cleanup
6. **Logging** - Detailed logging for debugging and monitoring

## 🚀 Next Steps

1. **Update CI/CD** - Use the new workflows in your CI/CD pipeline
2. **Local Development** - Use the new scripts for local testing
3. **Documentation** - Update team documentation with new testing procedures
4. **Monitoring** - Monitor test results and server health in CI
5. **Optimization** - Fine-tune server configuration based on test performance

## 📚 Related Documentation

- [Scripts README](scripts/README.md) - Complete script documentation
- [GitHub Workflows](.github/workflows/) - CI/CD workflow files
- [Environment Configuration](env.example) - Environment variable reference
