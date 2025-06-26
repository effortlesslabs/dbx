# DBX Publishing Scripts

This directory contains optimized publishing scripts for the DBX project. All scripts now use shared functions and configuration for consistency and maintainability.

## 📁 Script Files

### Core Scripts

- **`publish-release.sh`** - Complete release automation (Docker + NPM + Git)
- **`publish-docker.sh`** - Docker-only image building and publishing
- **`publish-npm.sh`** - NPM-only TypeScript SDK publishing
- **`quick-publish.sh`** - Interactive release helper

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
export NPM_PACKAGE_NAME="your-package-name"
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

### 🛡️ Safety Features

- **Dry-run mode** - Preview changes without executing
- **Version conflict detection** - Warns about existing versions
- **Credential validation** - Ensures required tokens are provided
- **File validation** - Checks for required files before starting
- **Tool validation** - Verifies required tools are installed
- **Automatic cleanup** - Removes temporary files on completion

### 🔍 Debugging

Enable debug and verbose modes for troubleshooting:

```bash
# Debug mode (shows all commands)
./scripts/publish-release.sh --version 1.0.0 --debug

# Verbose mode (detailed output)
./scripts/publish-release.sh --version 1.0.0 --verbose

# Both modes
./scripts/publish-release.sh --version 1.0.0 --debug --verbose
```

## 📋 Script Comparison

| Feature          | Full Release | Docker Only | NPM Only | Quick Publish |
| ---------------- | ------------ | ----------- | -------- | ------------- |
| Version Updates  | ✅           | ❌          | ✅       | ✅            |
| Rust Tests       | ✅           | ❌          | ❌       | ✅            |
| TypeScript Tests | ✅           | ❌          | ✅       | ✅            |
| Docker Build     | ✅           | ✅          | ❌       | ✅            |
| NPM Publish      | ✅           | ❌          | ✅       | ✅            |
| Git Operations   | ✅           | ❌          | ❌       | ✅            |
| Interactive      | ❌           | ❌          | ❌       | ✅            |
| Environment Vars | ✅           | ✅          | ✅       | ✅            |
| Dry Run          | ✅           | ❌          | ✅       | ❌            |

## ⚙️ Configuration

### Environment Variables

All scripts support these environment variables:

```bash
# Docker Configuration
DOCKER_USERNAME="fnlog0"           # Docker Hub username
DOCKER_PASSWORD=""                 # Docker Hub password/token
DOCKER_REPO="dbx"                  # Docker repository name
DOCKER_PLATFORMS="linux/amd64,linux/arm64"  # Target platforms

# NPM Configuration
NPM_TOKEN=""                       # NPM authentication token
NPM_PACKAGE_NAME="dbx-sdk"         # NPM package name
NPM_PACKAGE_ACCESS="public"        # Package access level

# Build Configuration
TYPESCRIPT_BUILD_DIR="ts"          # TypeScript build directory
RUST_BUILD_DIR="."                 # Rust build directory

# Testing Configuration
ENABLE_PARALLEL_TESTS="true"       # Enable parallel test execution
RUST_TEST_CMD="cargo test --all"   # Rust test command
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
# 1. Test changes
cargo test --all
cd ts && npm run test:run && cd ..

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
   npm view dbx-sdk versions
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

## 📚 Advanced Usage

### Custom Platforms

```bash
./scripts/publish-docker.sh \
  --platforms linux/amd64,linux/arm64,linux/arm/v7 \
  --tag multiarch
```

### Custom Package Name

```bash
NPM_PACKAGE_NAME="my-custom-package" \
./scripts/publish-npm.sh --npm-token $NPM_TOKEN
```

### Parallel Testing

```bash
ENABLE_PARALLEL_TESTS=true \
./scripts/publish-release.sh --version 1.0.0 --dry-run
```

## 🤝 Contributing

When modifying these scripts:

1. **Update shared functions** in `common.sh`
2. **Add new configuration** to `config.sh`
3. **Test all scripts** after changes
4. **Update this README** with new features
5. **Follow the existing patterns** for consistency

## 📄 License

These scripts are part of the DBX project and follow the same license terms.
