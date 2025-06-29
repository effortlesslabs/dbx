# Publishing Guide

This guide explains how to publish new versions of DBX, including both the Docker image and TypeScript SDK.

## Docker Image Architecture (Default)

**By default, all Docker images are built and published as multi-architecture (linux/amd64, linux/arm64).**

- This ensures compatibility with Railway, AWS, GCP, Azure, Apple Silicon, and more.
- You do **not** need to specify `--platform` unless you want a custom build.
- The default `./scripts/publish-docker.sh` will always build for both `amd64` and `arm64`.

## Prerequisites

Before publishing, ensure you have:

1. **Docker Hub Account**: Access to push images to Docker Hub
2. **NPM Account**: Access to publish packages to NPM
3. **Git Access**: Ability to push tags to the repository
4. **Docker Buildx**: For multi-platform image builds

## Required Credentials

### Docker Hub

- **Username**: Your Docker Hub username (default: `effortlesslabs`)
- **Password/Token**: Docker Hub access token (preferred over password)

### NPM

- **Token**: NPM access token with publish permissions

## Publishing Methods

### Method 1: Automated GitHub Actions (Recommended)

The easiest way to publish is using GitHub Actions:

1. **Create a new tag**:

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Manual workflow dispatch**:
   - Go to GitHub Actions → "Publish Release"
   - Click "Run workflow"
   - Fill in the required parameters:
     - Version: `1.0.0`
     - Docker username: `effortlesslabs` (or your username)
     - NPM token: Your NPM token

### Method 2: Quick Publishing Script

Use the interactive script for immediate publishing:

```bash
./scripts/quick-publish.sh
```

This script will:

- Prompt for the new version
- Ask for Docker Hub credentials
- Ask for NPM token
- Confirm before proceeding
- Execute the full publishing process

### Method 3: Manual Publishing Script

For more control, use the manual script:

```bash
./scripts/publish-release.sh \
  --version 1.0.0 \
  --docker-username effortlesslabs \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN
```

### Method 4: Docker Image Only (Multi-Arch by Default)

To publish only the Docker image (multi-arch):

```bash
./scripts/publish-docker.sh --tag 1.0.0 --push
```

### Method 5: NPM Package Only

To publish only the TypeScript SDK to NPM:

```bash
./scripts/publish-npm.sh --version 1.0.0 --npm-token $NPM_TOKEN
```

## Publishing Scripts Overview

### `publish-release.sh` - Full Release Script

- **Purpose**: Complete release process (Docker + NPM + Git tags)
- **Features**: Version bumping, testing, building, publishing, git tagging
- **Usage**: `./scripts/publish-release.sh --version 1.0.0 --docker-username user --docker-password token --npm-token token`

### `publish-docker.sh` - Docker Only Script

- **Purpose**: Docker image building and publishing only
- **Features**: Multi-arch builds, version tagging, Docker Hub push
- **Usage**: `./scripts/publish-docker.sh --tag 1.0.0 --push`

### `publish-npm.sh` - NPM Only Script

- **Purpose**: TypeScript SDK building and publishing only
- **Features**: Clean build, testing, NPM publishing, version management
- **Usage**: `./scripts/publish-npm.sh --version 1.0.0 --npm-token token`

### `quick-publish.sh` - Interactive Wrapper

- **Purpose**: User-friendly interactive publishing
- **Features**: Prompts for credentials, confirmation, calls full release script
- **Usage**: `./scripts/quick-publish.sh`

## Publishing Process

The publishing process includes the following steps:

1. **Version Update**: Updates version in `Cargo.toml`, `ts/package.json`, and `Dockerfile`
2. **Testing**: Runs all Rust and TypeScript tests
3. **TypeScript SDK Build**: Cleans previous builds and builds the TypeScript SDK
4. **NPM Publishing**: Publishes the TypeScript SDK to NPM
5. **Docker Build**: Builds multi-platform Docker image (linux/amd64, linux/arm64)
6. **Docker Push**: Pushes image to Docker Hub
7. **Git Tag**: Creates and pushes git tag

## Version Management

### Semantic Versioning

Follow semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Version Files

The following files are automatically updated:

- `Cargo.toml` - Workspace version
- `ts/package.json` - TypeScript SDK version
- `Dockerfile` - Version label

## Docker Image Details

### Multi-Platform Support (Default)

The Docker image is built for multiple platforms by default:

- `linux/amd64` - Intel/AMD 64-bit (Railway, most cloud)
- `linux/arm64` - ARM 64-bit (Apple Silicon, Raspberry Pi, etc.)

### Image Tags

Each release creates multiple tags:

- `effortlesslabs/0dbx_redis:1.0.0` - Specific version
- `effortlesslabs/0dbx_redis:latest` - Latest version
- `effortlesslabs/0dbx_redis:1.0` - Major.minor version
- `effortlesslabs/0dbx_redis:1` - Major version

### Usage

```bash
# Pull specific version
docker pull effortlesslabs/0dbx_redis:1.0.0

# Run with Redis
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  effortlesslabs/0dbx_redis:1.0.0

# Using docker-compose
docker-compose up -d
```

## TypeScript SDK Details

### Package Information

- **Name**: `@0dbx/redis`
- **Registry**: NPM
- **Access**: Public

### Installation

```bash
# Install specific version
npm install @0dbx/redis@1.0.0

# Install latest
npm install @0dbx/redis
```

### Usage

```typescript
import { createClient } from "@0dbx/redis";

const client = createClient({
  baseUrl: "http://localhost:3000",
  timeout: 5000,
});

// String operations
await client.string.set("key", "value");
const value = await client.string.get("key");
```

## Verification

After publishing, verify the release:

### Docker Image

```bash
# Check image exists
docker pull effortlesslabs/0dbx_redis:1.0.0

# Test the image
docker run --rm -p 3000:3000 effortlesslabs/0dbx_redis:1.0.0
```

### TypeScript SDK

```bash
# Check package exists
npm view dbx-sdk@1.0.0

# Test installation
npm install dbx-sdk@1.0.0
```

### Online Verification

- **Docker Hub**: https://hub.docker.com/r/fnlog0/dbx
- **NPM**: https://www.npmjs.com/package/dbx-sdk

## Troubleshooting

### Common Issues

1. **Docker Buildx Not Available**

   ```bash
   docker buildx install
   ```

2. **Authentication Errors**

   - Verify Docker Hub credentials
   - Check NPM token permissions
   - Ensure tokens are not expired

3. **Version Already Exists**

   - Check if version already exists on Docker Hub/NPM
   - Use a different version number

4. **Build Failures**
   - Check Docker daemon is running
   - Verify sufficient disk space
   - Check network connectivity

### Environment Variables

You can set credentials as environment variables:

```bash
export DOCKER_PASSWORD="your-docker-token"
export NPM_TOKEN="your-npm-token"
```

### Dry Run

Test the publishing process without actually publishing:

```bash
./scripts/publish-release.sh \
  --version 1.0.0 \
  --docker-username fnlog0 \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN \
  --dry-run
```

## Script Optimizations

The publishing scripts have been optimized with:

- **Shared Functions**: Common utilities in `scripts/common.sh`
- **Centralized Config**: Configuration management in `scripts/config.sh`
- **Better Error Handling**: Comprehensive error checking and recovery
- **Progress Indicators**: Visual feedback during long operations
- **Parallel Processing**: Concurrent operations where possible
- **Caching**: Build artifact caching for faster rebuilds
- **Validation**: Input validation and pre-flight checks
- **Enhanced Logging**: Detailed logging with different verbosity levels

## Security Considerations

1. **Tokens**: Use access tokens instead of passwords
2. **Scope**: Use minimal required permissions for tokens
3. **Rotation**: Regularly rotate access tokens
4. **Secrets**: Store tokens securely (use GitHub Secrets for CI/CD)

## Release Notes

After publishing, consider:

1. **GitHub Release**: Create a release with notes
2. **Documentation**: Update documentation if needed
3. **Announcement**: Announce the release to users
4. **Monitoring**: Monitor for any issues

## Support

For publishing issues:

1. Check the troubleshooting section
2. Review GitHub Actions logs
3. Verify credentials and permissions
4. Contact the maintainers if needed
