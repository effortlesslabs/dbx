# Multi-Platform Docker Support

DBX now supports multi-platform Docker builds, allowing you to run the application on both AMD64 and ARM64 architectures.

## Supported Platforms

- **linux/amd64** - Intel/AMD 64-bit processors
- **linux/arm64** - ARM 64-bit processors (Apple Silicon, Raspberry Pi 4, etc.)

## Building Multi-Platform Images

### Using the Multi-Arch Script

```bash
# Build for both AMD64 and ARM64 (default)
./scripts/publish-multiarch.sh --tag latest --push

# Build for ARM64 only
./scripts/publish-multiarch.sh --tag arm64-only --push --platforms linux/arm64

# Build for specific platforms
./scripts/publish-multiarch.sh --tag custom --push --platforms linux/amd64,linux/arm64,linux/arm/v7
```

### Manual Build with Docker Buildx

```bash
# Create a new builder instance
docker buildx create --name multiarch-builder --use

# Build and push multi-platform image
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  --tag fnlog0/dbx:latest \
  --push \
  .
```

## Running on Different Platforms

### ARM64 (Apple Silicon, Raspberry Pi, etc.)

```bash
# Explicitly specify ARM64 platform
docker run --platform linux/arm64 -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest
```

### AMD64 (Intel/AMD)

```bash
# Explicitly specify AMD64 platform
docker run --platform linux/amd64 -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest
```

### Auto-Detection (Recommended)

```bash
# Docker will automatically select the correct platform
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest
```

## Using with Docker Compose

The `docker-compose.yml` file automatically uses the correct platform:

```bash
export REDIS_URL=redis://localhost:6379
docker-compose up -d
```

## Inspecting Image Platforms

To see which platforms are available for an image:

```bash
docker buildx imagetools inspect fnlog0/dbx:latest
```

## GitHub Actions

Multi-platform builds are automatically triggered on:

- Push to main/master branch
- Tag creation (v\*)
- Pull requests (build only, no push)

The workflow builds for both AMD64 and ARM64 platforms and pushes to Docker Hub.

## Prerequisites

### Local Development

1. **Docker Buildx**: Ensure you have Docker Buildx installed

   ```bash
   docker buildx version
   ```

2. **Docker Hub Access**: For pushing images, you need Docker Hub credentials
   ```bash
   docker login
   ```

### GitHub Actions

Set up these secrets in your GitHub repository:

- `DOCKER_USERNAME`: Your Docker Hub username
- `DOCKER_PASSWORD`: Your Docker Hub password or access token

## Performance Considerations

- **ARM64 builds** may take longer than AMD64 builds
- **Cross-platform builds** require emulation, which can be slower
- **Local builds** are faster than CI/CD builds for testing

## Troubleshooting

### Buildx Not Available

```bash
# Install Docker Buildx
docker buildx install
```

### Platform Not Supported

If you encounter platform-specific issues:

1. Check if your Docker version supports the platform
2. Ensure you're using the latest Docker Buildx
3. Try building for a single platform first

### Memory Issues

For large builds, you may need to increase Docker's memory limit:

- Docker Desktop: Settings → Resources → Memory
- Docker Engine: Modify daemon.json

## Examples

### Raspberry Pi Deployment

```bash
# On Raspberry Pi (ARM64)
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://192.168.1.100:6379 \
  -e LOG_LEVEL=INFO \
  fnlog0/dbx:latest
```

### Apple Silicon Mac

```bash
# On Apple Silicon Mac (ARM64)
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  -e POOL_SIZE=20 \
  fnlog0/dbx:latest
```

### Production Deployment

```bash
# Production deployment with all options
docker run -d --name dbx-api \
  --restart unless-stopped \
  -p 8080:3000 \
  -e DATABASE_URL=redis://user:pass@redis.example.com:6379 \
  -e POOL_SIZE=50 \
  -e LOG_LEVEL=WARN \
  fnlog0/dbx:latest
```
