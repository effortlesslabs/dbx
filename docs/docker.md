# Docker Configuration for DBX

This document provides instructions for running DBX using Docker.

## Quick Start

### Setup and Run

1. **Setup Docker environment:**

   ```bash
   ./scripts/docker-setup.sh
   ```

2. **Manual setup:**

   ```bash
   # Build and start services
   docker-compose up -d

   # View logs
   docker-compose logs -f
   ```

## Services

### DBX API

- **Port**: 3000
- **Health Check**: `GET /health`
- **Environment Variables**: See `api/env.example`

### Redis

- **Port**: 6379
- **Persistence**: AOF enabled
- **Web UI**: Redis Commander (port 8081)

### Redis Commander

- **Port**: 8081
- **Purpose**: Web-based Redis management interface
- **URL**: http://localhost:8081

## Configuration

### Environment Variables

The Docker setup automatically configures environment variables:

```bash
REDIS_URL=redis://redis:6379
DATABASE_TYPE=redis
HOST=0.0.0.0
PORT=3000
POOL_SIZE=10
LOG_LEVEL=INFO
```

### Custom Configuration

To customize the configuration:

1. **Modify environment variables in `docker-compose.yml`**
2. **Create a `.env` file** in the root directory
3. **Update the Dockerfile** for build-time changes

## Usage

### Basic Commands

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild images
docker-compose build --no-cache

# Clean up (removes volumes)
docker-compose down -v
```

### Accessing Services

- **DBX API**: http://localhost:3000
- **Redis**: localhost:6379
- **Redis Commander**: http://localhost:8081

### Health Checks

```bash
# Check API health
curl http://localhost:3000/health

# Check Redis health
docker exec dbx-redis redis-cli ping
```

## Development

### Local Development with Docker

```bash
# Start services
docker-compose up -d

# Run tests
docker-compose exec dbx-api cargo test

# Access container shell
docker exec -it dbx-api bash
```

### Building Locally

```bash
# Build the image
docker build -t dbx-api .

# Run the container
docker run -p 3000:3000 dbx-api
```

## Troubleshooting

### Common Issues

1. **Port conflicts**

   ```bash
   # Check port usage
   netstat -tulpn | grep :3000

   # Change ports in docker-compose.yml
   ports:
     - "3001:3000"
   ```

2. **Redis connection issues**

   ```bash
   # Test Redis connection
   redis-cli ping

   # Check Redis logs
   docker-compose logs redis
   ```

3. **Build issues**

   ```bash
   # Clean build
   docker-compose build --no-cache

   # Check Dockerfile
   docker build -t dbx-api . --progress=plain
   ```

### Debugging

```bash
# View all logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f dbx-api

# Access container shell
docker exec -it dbx-api bash

# Check container status
docker-compose ps
```

## File Structure

```
dbx/
├── Dockerfile              # Main Docker image
├── docker-compose.yml      # Service orchestration
├── .dockerignore          # Docker build exclusions
├── scripts/
│   └── docker-setup.sh    # Setup script
├── docs/
│   └── docker.md          # This documentation
└── api/                   # API source code
    ├── src/
    ├── Cargo.toml
    └── env.example
```

## Best Practices

1. **Always use the setup script** for initial configuration
2. **Check service health** after starting
3. **Use volume mounts** for persistent data
4. **Monitor logs** for debugging
5. **Clean up unused resources** regularly
