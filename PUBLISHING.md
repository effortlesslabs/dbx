# Publishing Guide

This guide covers the process of publishing new versions of DBX, including the Docker image and TypeScript SDK.

## Overview

DBX consists of multiple components that need to be published:

1. **Docker Image** - The main DBX server
2. **TypeScript SDK** - NAPI bindings for Node.js/TypeScript
3. **GitHub Release** - Source code and binaries

## Prerequisites

### Required Accounts

- **Docker Hub**: [effortlesslabs/0dbx_redis](https://hub.docker.com/r/effortlesslabs/0dbx_redis)
- **NPM**: [@0dbx](https://www.npmjs.com/org/0dbx)
- **GitHub**: [effortlesslabs/dbx](https://github.com/effortlesslabs/dbx)

### Required Tokens

```bash
# Docker Hub token
export DOCKER_TOKEN="your-docker-hub-token"

# NPM token
export NPM_TOKEN="your-npm-token"

# GitHub token (for releases)
export GITHUB_TOKEN="your-github-token"
```

## Version Management

### Version Format

DBX uses semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Version Locations

```bash
# Main workspace version
Cargo.toml: [workspace.package].version = "0.1.6"

# TypeScript SDK version
bindings/redis_ts/package.json: "version": "0.1.6"

# Docker image tags
effortlesslabs/0dbx_redis:0.1.6
effortlesslabs/0dbx_redis:latest
```

## Publishing Process

### 1. Update Version

```bash
# Update version in Cargo.toml
sed -i 's/version = "0.1.5"/version = "0.1.6"/' Cargo.toml

# Update version in TypeScript package.json
sed -i 's/"version": "0.1.5"/"version": "0.1.6"/' bindings/redis_ts/package.json

# Update version in Dockerfile
sed -i 's/LABEL version="0.1.5"/LABEL version="0.1.6"/' Dockerfile
```

### 2. Build and Test

```bash
# Build all components
cargo build --release

# Build TypeScript SDK
cd bindings/redis_ts
npm run build
cd ../..

# Run tests
cargo test
cd bindings/redis_ts && npm test && cd ../..

# Test Docker build
docker build -t effortlesslabs/0dbx_redis:test .
```

### 3. Publish TypeScript SDK

```bash
# Navigate to TypeScript bindings
cd bindings/redis_ts

# Login to NPM
npm login --scope=@0dbx

# Publish package
npm publish

# Verify publication
npm view @0dbx/redis version
```

### 4. Build and Push Docker Image

```bash
# Build multi-arch image
docker buildx build --platform linux/amd64,linux/arm64 \
  -t effortlesslabs/0dbx_redis:0.1.6 \
  -t effortlesslabs/0dbx_redis:latest \
  --push .

# Build AMD64-only image (for Railway)
docker buildx build --platform linux/amd64 \
  -t effortlesslabs/0dbx_redis:0.1.6-amd64-only \
  -t effortlesslabs/0dbx_redis:latest-amd64-only \
  --push .
```

### 5. Create GitHub Release

```bash
# Create git tag
git tag v0.1.6
git push origin v0.1.6

# Or use GitHub CLI
gh release create v0.1.6 \
  --title "DBX v0.1.6" \
  --notes "Release notes here" \
  --draft
```

## Automated Publishing

### Using Scripts

DBX provides several publishing scripts:

```bash
# Interactive publishing
./scripts/quick-publish.sh

# Manual publishing
./scripts/publish-release.sh --version 0.1.6 \
  --docker-username effortlesslabs \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN

# Quick publish (latest)
./scripts/publish.sh
```

### GitHub Actions

The easiest way to publish is using GitHub Actions:

1. **Create a git tag**: `git tag v0.1.6 && git push origin v0.1.6`
2. **Or manually trigger** the workflow from GitHub Actions

## Publishing Scripts

### Quick Publish Script

```bash
#!/bin/bash
# scripts/quick-publish.sh

echo "🚀 DBX Quick Publish"
echo "=================="

# Get current version
CURRENT_VERSION=$(grep 'version = ' Cargo.toml | cut -d'"' -f2)
echo "Current version: $CURRENT_VERSION"

# Prompt for new version
read -p "New version: " NEW_VERSION

# Update versions
sed -i "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$NEW_VERSION\"/" bindings/redis_ts/package.json
sed -i "s/LABEL version=\"$CURRENT_VERSION\"/LABEL version=\"$NEW_VERSION\"/" Dockerfile

# Build and publish
./scripts/publish-release.sh --version $NEW_VERSION
```

### Manual Publish Script

```bash
#!/bin/bash
# scripts/publish-release.sh

set -e

VERSION=""
DOCKER_USERNAME=""
DOCKER_PASSWORD=""
NPM_TOKEN=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --docker-username)
      DOCKER_USERNAME="$2"
      shift 2
      ;;
    --docker-password)
      DOCKER_PASSWORD="$2"
      shift 2
      ;;
    --npm-token)
      NPM_TOKEN="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  echo "Error: Version is required"
  exit 1
fi

echo "🚀 Publishing DBX v$VERSION"

# Build TypeScript SDK
echo "📦 Building TypeScript SDK..."
cd bindings/redis_ts
npm run build
npm publish --access public
cd ../..

# Build and push Docker image
echo "🐳 Building and pushing Docker image..."
docker buildx build --platform linux/amd64,linux/arm64 \
  -t effortlesslabs/0dbx_redis:$VERSION \
  -t effortlesslabs/0dbx_redis:latest \
  --push .

# Build AMD64-only image
docker buildx build --platform linux/amd64 \
  -t effortlesslabs/0dbx_redis:$VERSION-amd64-only \
  -t effortlesslabs/0dbx_redis:latest-amd64-only \
  --push .

echo "✅ Published DBX v$VERSION successfully!"
```

## Package-Specific Publishing

### TypeScript SDK (@0dbx/redis)

```bash
cd bindings/redis_ts

# Build the package
npm run build

# Test the build
npm test

# Publish to NPM
npm publish --access public

# Verify publication
npm view @0dbx/redis version
```

### Docker Image (effortlesslabs/0dbx_redis)

```bash
# Build multi-arch image
docker buildx build --platform linux/amd64,linux/arm64 \
  -t effortlesslabs/0dbx_redis:0.1.6 \
  -t effortlesslabs/0dbx_redis:latest \
  --push .

# Build AMD64-only image (for Railway)
docker buildx build --platform linux/amd64 \
  -t effortlesslabs/0dbx_redis:0.1.6-amd64-only \
  -t effortlesslabs/0dbx_redis:latest-amd64-only \
  --push .
```

## Release Notes Template

````markdown
# DBX v0.1.6

## 🚀 New Features

- Feature 1
- Feature 2

## 🐛 Bug Fixes

- Fix 1
- Fix 2

## 🔧 Improvements

- Improvement 1
- Improvement 2

## 📦 Installation

### Docker

```bash
docker pull effortlesslabs/0dbx_redis:0.1.6
```
````

````

### TypeScript SDK

```bash
npm install @0dbx/redis@0.1.6
````

## 🔗 Links

- [Documentation](https://docs.dbx.dev)
- [GitHub Repository](https://github.com/effortlesslabs/dbx)
- [Docker Hub](https://hub.docker.com/r/effortlesslabs/0dbx_redis)
- [NPM Package](https://www.npmjs.com/package/@0dbx/redis)
