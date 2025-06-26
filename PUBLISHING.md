# Publishing Guide

This guide explains how to publish new versions of DBX, including both the Docker image and TypeScript SDK.

## Prerequisites

Before publishing, ensure you have:

1. **Docker Hub Account**: Access to push images to Docker Hub
2. **NPM Account**: Access to publish packages to NPM
3. **Git Access**: Ability to push tags to the repository
4. **Docker Buildx**: For multi-platform image builds

## Required Credentials

### Docker Hub

- **Username**: Your Docker Hub username (default: `fnlog0`)
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
     - Docker username: `fnlog0` (or your username)
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
  --docker-username fnlog0 \
  --docker-password $DOCKER_TOKEN \
  --npm-token $NPM_TOKEN
```

### Method 4: Docker Image Only

To publish only the Docker image:

```bash
./scripts/publish.sh --tag 1.0.0 --push
```

## Publishing Process

The publishing process includes the following steps:

1. **Version Update**: Updates version in `Cargo.toml`, `ts/package.json`, and `Dockerfile`
2. **Testing**: Runs all Rust and TypeScript tests
3. **TypeScript SDK Build**: Builds the TypeScript SDK
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

### Multi-Platform Support

The Docker image is built for multiple platforms:

- `linux/amd64` - Intel/AMD 64-bit
- `linux/arm64` - ARM 64-bit (Apple Silicon, Raspberry Pi, etc.)

### Image Tags

Each release creates multiple tags:

- `fnlog0/dbx:1.0.0` - Specific version
- `fnlog0/dbx:latest` - Latest version
- `fnlog0/dbx:1.0` - Major.minor version
- `fnlog0/dbx:1` - Major version

### Usage

```bash
# Pull specific version
docker pull fnlog0/dbx:1.0.0

# Run with Redis
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:1.0.0

# Using docker-compose
docker-compose up -d
```

## TypeScript SDK Details

### Package Information

- **Name**: `dbx-sdk`
- **Registry**: NPM
- **Access**: Public

### Installation

```bash
# Install specific version
npm install dbx-sdk@1.0.0

# Install latest
npm install dbx-sdk
```

### Usage

```typescript
import { DBX } from "dbx-sdk";

const dbx = new DBX({
  baseUrl: "http://localhost:3000",
  databaseUrl: "redis://localhost:6379",
});

// String operations
const stringOps = dbx.string();
await stringOps.set("key", "value");
const value = await stringOps.get("key");

// Hash operations
const hashOps = dbx.hash();
await hashOps.hset("user:1", "name", "Alice");
const name = await hashOps.hget("user:1", "name");
```

## Verification

After publishing, verify the release:

### Docker Image

```bash
# Check image exists
docker pull fnlog0/dbx:1.0.0

# Test the image
docker run --rm -p 3000:3000 fnlog0/dbx:1.0.0
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
