# DBX Publishing Scripts

This directory contains optimized publishing scripts for the DBX project. All scripts now use shared functions and configuration for consistency and maintainability.

## 📁 Script Files

### Core Scripts

- **`publish-release.sh`** - Complete release automation (Docker + NPM + Git)
- **`publish-docker.sh`** - Docker-only image building and publishing
- **`publish-npm.sh`** - NPM-only TypeScript bindings publishing
- **`quick-publish.sh`** - Interactive release helper
- **`test-sequential.sh`** - Sequential testing (adapter → api → client)

### Testing Scripts

- **`test-with-server.sh`** - Complete test runner with server setup and cleanup
- **`test-simple.sh`** - Simple test runner for existing server

### Shared Files

- **`common.sh`** - Shared functions and utilities
- **`config.sh`** - Centralized configuration

## 🚀 Quick Start

### Environment Setup

Set up your environment variables for easier usage:

```bash
# Docker credentials
export DOCKER_USERNAME="your-username"
export DOCKER_PASSWORD="your-token"

# NPM credentials
export NPM_TOKEN="your-npm-token"

# Optional: Customize defaults
export DOCKER_REPO="your-repo-name"
export NPM_PACKAGE_NAME="dbx-redis-ts-bindings"
```

### Basic Usage

#### Full Release (Recommended)

```bash
# Interactive mode
./scripts/quick-publish.sh

# Command line mode
./scripts/publish-release.sh --version 1.0.0 --docker-password $DOCKER_TOKEN --npm-token $NPM_TOKEN
```

#### Docker Only

```bash
# Build locally
./scripts/publish-docker.sh --tag latest

# Build and push
./scripts/publish-docker.sh --tag v1.0.0 --push --password $DOCKER_TOKEN
```

#### NPM Only

```bash
# Publish current version
./scripts/publish-npm.sh --npm-token $NPM_TOKEN

# Publish with new version
./scripts/publish-npm.sh --version 1.0.0 --npm-token $NPM_TOKEN --update-version
```

#### Testing

```bash
# Test with server setup (recommended for CI)
./scripts/test-with-server.sh

# Test against existing server
./scripts/test-simple.sh

# Sequential testing (no server)
./scripts/test-sequential.sh
```

## 🧪 Testing Scripts

### test-with-server.sh

Complete test runner that sets up the entire testing environment:

**Features:**

- Starts Redis service automatically
- Builds and starts DBX API server
- Runs all crate tests against the running server
- Automatic cleanup of containers and services
- Environment variable management

**Usage:**

```bash
# Basic usage
./scripts/test-with-server.sh

# Custom environment
./scripts/test-with-server.sh --env-file .env.test --redis-url redis://localhost:6379

# Skip server start (assume it's running)
./scripts/test-with-server.sh --skip-server

# Keep server running after tests
./scripts/test-with-server.sh --skip-cleanup
```

### test-simple.sh

Lightweight test runner for existing server environments:

**Features:**

- Assumes server is already running
- Sets up environment variables for tests
- Runs crate tests sequentially
- Minimal dependencies

**Usage:**

```bash
# Test against default server
./scripts/test-simple.sh

# Custom server configuration
./scripts/test-simple.sh --redis-url redis://localhost:6379 --server-url http://localhost:3000

# Verbose output
./scripts/test-simple.sh --verbose
```

### test-sequential.sh

Original sequential test runner (no server required):

**Features:**

- Runs tests in dependency order (adapter → api → client)
- No server setup required
- Includes TypeScript tests
- Good for development and pre-publish testing

**Usage:**

```bash
# Run all tests
./scripts/test-sequential.sh

# Skip TypeScript tests
./scripts/test-sequential.sh --skip-typescript

# Verbose output
./scripts/test-sequential.sh --verbose
```

## 🔧 Features

### ✅ Optimizations Implemented

1. **Shared Functions** - Common utilities in `common.sh`
2. **Centralized Configuration** - All settings in `config.sh`
3. **Environment Variables** - Support for all credentials via env vars
4. **Better Error Handling** - Comprehensive error recovery and cleanup
5. **Pre-flight Checks** - Validate tools, files, and credentials
6. **Retry Logic** - Automatic retries for network operations
7. **Progress Indicators** - Visual feedback during long operations
8. **Version Validation** - Semantic versioning validation
9. **Backup/Restore** - Automatic backup of version files
10. **Debug/Verbose Modes** - Enhanced logging and troubleshooting
11. **Server Integration** - Test against running DBX API server
12. **Container Management** - Automatic Docker container lifecycle

### 🛡️ Safety Features

- **Dry-run mode** - Preview changes without executing
- **Version conflict detection** - Warns about existing versions
- **Credential validation** - Ensures required tokens are provided
- **File validation** - Checks for required files before starting
- **Tool validation** - Verifies required tools are installed
- **Automatic cleanup** - Removes temporary files on completion
- **Server health checks** - Validates server is running before tests
- **Container cleanup** - Ensures containers are stopped after tests

### 🔍 Debugging

Enable debug and verbose modes for troubleshooting:

```bash
# Debug mode (shows all commands)
./scripts/publish-release.sh --version 1.0.0 --debug

# Verbose mode (detailed output)
./scripts/publish-release.sh --version 1.0.0 --verbose

# Both modes
./scripts/publish-release.sh --version 1.0.0 --debug --verbose

# Test with verbose output
./scripts/test-with-server.sh --verbose
./scripts/test-simple.sh --verbose
```

## 📋 Script Comparison

| Feature          | Full Release | Docker Only | NPM Only | Quick Publish | Test Sequential | Test with Server | Test Simple |
| ---------------- | ------------ | ----------- | -------- | ------------- | --------------- | ---------------- | ----------- |
| Version Updates  | ✅           | ❌          | ✅       | ✅            | ❌              | ❌               | ❌          |
| Rust Tests       | ✅           | ❌          | ❌       | ✅            | ✅              | ✅               | ✅          |
| TypeScript Tests | ✅           | ❌          | ✅       | ✅            | ✅              | ❌               | ❌          |
| Docker Build     | ✅           | ✅          | ❌       | ✅            | ❌              | ✅               | ❌          |
| NPM Publish      | ✅           | ❌          | ✅       | ✅            | ❌              | ❌               | ❌          |
| Git Operations   | ✅           | ❌          | ❌       | ✅            | ❌              | ❌               | ❌          |
| Interactive      | ❌           | ❌          | ❌       | ✅            | ❌              | ❌               | ❌          |
| Environment Vars | ✅           | ✅          | ✅       | ✅            | ✅              | ✅               | ✅          |
| Dry Run          | ✅           | ❌          | ✅       | ❌            | ❌              | ❌               | ❌          |
| Sequential Tests | ✅           | ❌          | ❌       | ✅            | ✅              | ✅               | ✅          |
| Server Setup     | ❌           | ❌          | ❌       | ❌            | ❌              | ✅               | ❌          |
| Container Mgmt   | ❌           | ❌          | ❌       | ❌            | ❌              | ✅               | ❌          |
| CI/CD Ready      | ✅           | ✅          | ✅       | ❌            | ✅              | ✅               | ✅          |

## ⚙️ Configuration

### Environment Variables

All scripts support these environment variables:

```bash
# Docker Configuration
DOCKER_USERNAME="effortlesslabs"           # Docker Hub username
DOCKER_PASSWORD=""                 # Docker Hub password/token
DOCKER_REPO="dbx"                  # Docker repository name
DOCKER_PLATFORMS="linux/amd64,linux/arm64"  # Target platforms

# NPM Configuration
NPM_TOKEN=""                       # NPM authentication token
NPM_PACKAGE_NAME="dbx-redis-ts-bindings"         # NPM package name
NPM_PACKAGE_ACCESS="public"        # Package access level

# Build Configuration
TYPESCRIPT_BUILD_DIR="bindings/redis_ts"          # TypeScript build directory
RUST_BUILD_DIR="."                 # Rust build directory

# Testing Configuration
ENABLE_SEQUENTIAL_TESTS="true"     # Enable sequential test execution (adapter → api → client)
RUST_TEST_CMD_ADAPTER="cd crates/adapter && cargo test"  # Adapter test command
RUST_TEST_CMD_API="cd crates/redis_api && cargo test"    # API test command
RUST_TEST_CMD_CLIENT="cd crates/redis_client && cargo test"  # Client test command
TYPESCRIPT_TEST_CMD="npm run test:run"  # TypeScript test command

# Error Handling
MAX_RETRIES="3"                    # Maximum retry attempts
RETRY_DELAY="5"                    # Delay between retries
ENABLE_AUTO_BACKUP="true"          # Auto-backup version files
ENABLE_AUTO_RESTORE="true"         # Auto-restore on failure

# Logging
DEBUG="false"                      # Enable debug mode
VERBOSE="false"                    # Enable verbose output
LOG_LEVEL="info"                   # Log level (debug, info, warning, error)
```

### Configuration File

You can create a `.env` file in the project root to set these variables:

```bash
# .env
DOCKER_USERNAME=your-username
DOCKER_PASSWORD=your-token
NPM_TOKEN=your-npm-token
DEBUG=false
VERBOSE=false
```

## 🔄 Workflow Examples

### Development Workflow

```bash
# 1. Test changes (sequential order)
./scripts/test-sequential.sh

# Or test manually in order:
cd crates/adapter && cargo test && cd ../redis_api && cargo test && cd ../redis_client && cargo test
cd bindings/redis_ts && npm run test:run && cd ../..

# 2. Quick NPM publish for testing
./scripts/publish-npm.sh --version 0.1.6 --npm-token $NPM_TOKEN --update-version

# 3. Full release when ready
./scripts/quick-publish.sh
```

### CI/CD Workflow

```bash
# Automated release in CI
./scripts/publish-release.sh \
  --version $VERSION \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN
```

### Railway Deployment

```bash
# Build Railway-compatible image
./scripts/publish-docker.sh \
  --tag railway-deploy \
  --push \
  --password $DOCKER_TOKEN
```

## 🐛 Troubleshooting

### Common Issues

1. **Permission Denied**

   ```bash
   chmod +x scripts/*.sh
   ```

2. **Missing Tools**

   ```bash
   # Install required tools
   brew install docker buildx  # macOS
   npm install -g npm          # Update npm
   ```

3. **Authentication Errors**

   ```bash
   # Verify credentials
   docker login
   npm whoami
   ```

4. **Version Conflicts**
   ```bash
   # Check existing versions
   npm view dbx-redis-ts-bindings versions
   git tag -l
   ```

### Debug Mode

Enable debug mode to see exactly what's happening:

```bash
DEBUG=true ./scripts/publish-release.sh --version 1.0.0 --dry-run
```

### Verbose Output

Get detailed information about each step:

```bash
./scripts/publish-release.sh --version 1.0.0 --verbose --dry-run
```
