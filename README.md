# DBX - Redis API Gateway

A high-performance Redis API gateway that provides both HTTP and WebSocket interfaces for Redis operations.

## Quick Start with Published Docker Image

**📦 Available on Docker Hub: [fnlog0/dbx](https://hub.docker.com/r/fnlog0/dbx)**

### Option 1: Using the provided script (Recommended)

```bash
# Basic usage with just Redis URL
./scripts/run.sh --redis-url redis://localhost:6379

# Full configuration with all variables inline
./scripts/run.sh \
  --redis-url redis://user:password@redis.example.com:6379 \
  --host 0.0.0.0 \
  --port 8080 \
  --pool-size 20 \
  --log-level DEBUG

# Using a specific image version
./scripts/run.sh \
  --redis-url redis://localhost:6379 \
  --image fnlog0/dbx:v1.0.0

# Show all options
./scripts/run.sh --help
```

### Option 2: Direct docker run

```bash
# Basic usage
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest

# With all options
docker run -d --name dbx-api -p 8080:3000 \
  -e DATABASE_URL=redis://user:pass@redis.com:6379 \
  -e POOL_SIZE=20 \
  -e LOG_LEVEL=DEBUG \
  fnlog0/dbx:latest
```

### Option 3: Using docker-compose

```bash
# Set your Redis URL
export REDIS_URL=redis://your-redis-url:6379

# Run the container
docker-compose up -d
```

## Publishing to Docker Hub

### Build and Publish

```bash
# Build and push to Docker Hub
./scripts/publish.sh --username yourusername --tag v1.0.0 --push

# Build only (without pushing)
./scripts/publish.sh --username yourusername --tag v1.0.0

# Show help
./scripts/publish.sh --help
```

### Manual Publishing

```bash
# Build the image
docker build -t yourusername/dbx-api:latest .

# Login to Docker Hub
docker login

# Push the image
docker push yourusername/dbx-api:latest
```

## Available Endpoints

Once running, the following endpoints will be available:

### HTTP API

- **Main API**: http://localhost:3000
- **Health Check**: http://localhost:3000/redis/admin/ping
- **Redis String Operations**: http://localhost:3000/redis/string/:key
- **Redis Hash Operations**: http://localhost:3000/redis/hash/:key
- **Redis Set Operations**: http://localhost:3000/redis/set/:key
- **Redis Admin**: http://localhost:3000/redis/admin/\*

### WebSocket API

- **String WebSocket**: ws://localhost:3000/redis_ws/string/ws
- **Hash WebSocket**: ws://localhost:3000/redis_ws/hash/ws
- **Set WebSocket**: ws://localhost:3000/redis_ws/set/ws
- **Admin WebSocket**: ws://localhost:3000/redis_ws/admin/ws

## Environment Variables

| Variable        | Default                  | Description                                    |
| --------------- | ------------------------ | ---------------------------------------------- |
| `DATABASE_URL`  | `redis://localhost:6379` | Your Redis connection URL                      |
| `DATABASE_TYPE` | `redis`                  | Database type (currently only redis supported) |
| `HOST`          | `0.0.0.0`                | Server host address                            |
| `PORT`          | `3000`                   | Server port                                    |
| `POOL_SIZE`     | `10`                     | Connection pool size                           |
| `LOG_LEVEL`     | `INFO`                   | Logging level                                  |

## Redis URL Formats

The `DATABASE_URL` supports various Redis connection formats:

```
# Basic local Redis
redis://localhost:6379

# Redis with password
redis://:password@localhost:6379

# Redis with username and password
redis://username:password@localhost:6379

# Remote Redis
redis://redis.example.com:6379

# Redis Cloud
redis://default:password@redis-12345.c123.us-east-1-1.ec2.cloud.redislabs.com:12345

# Redis with database number
redis://localhost:6379/1
```

## Management Commands

```bash
# View logs (docker run)
docker logs -f dbx-api

# View logs (docker-compose)
docker-compose logs -f

# Stop the service (docker run)
docker stop dbx-api && docker rm dbx-api

# Stop the service (docker-compose)
docker-compose down

# Restart the service (docker-compose)
docker-compose restart

# Check health
curl http://localhost:3000/redis/admin/ping
```

## Troubleshooting

### Connection Issues

- Ensure your Redis URL is correct and accessible from the Docker container
- Check if Redis requires authentication
- Verify network connectivity between Docker and your Redis instance

### Port Conflicts

- If port 3000 is already in use, change the `PORT` environment variable
- Update the port mapping in docker run: `-p 3001:3000`

### Health Check Failures

- The container includes a health check that pings Redis
- Check logs for connection errors: `docker logs dbx-api`

## Examples

### Local Development

```bash
# Start local Redis (if you have it installed)
redis-server

# Run DBX API with published image
./scripts/run.sh --redis-url redis://localhost:6379 --port 3001
```

### Production with Redis Cloud

```bash
# Run with Redis Cloud and custom configuration
./scripts/run.sh \
  --redis-url redis://default:your-password@redis-12345.c123.us-east-1-1.ec2.cloud.redislabs.com:12345 \
  --port 8080 \
  --pool-size 20 \
  --log-level INFO
```

### Docker Network

```bash
# If Redis is in another Docker container
./scripts/run.sh --redis-url redis://redis-container:6379
```

### One-liner docker run

```bash
# Complete inline configuration
docker run -d --name dbx-api -p 8080:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  -e DATABASE_TYPE=redis \
  -e HOST=0.0.0.0 \
  -e PORT=3000 \
  -e POOL_SIZE=15 \
  -e LOG_LEVEL=DEBUG \
  yourusername/dbx-api:latest
```

## Docker Hub Usage

Once published to Docker Hub, users can easily run your image:

```bash
# Pull and run
docker pull fnlog0/dbx:latest
docker run -d --name dbx-api -p 3000:3000 \
  -e DATABASE_URL=redis://localhost:6379 \
  fnlog0/dbx:latest
```

The image is designed to be user-friendly with sensible defaults and clear environment variable documentation.

**🔗 Docker Hub Repository: [https://hub.docker.com/r/fnlog0/dbx](https://hub.docker.com/r/fnlog0/dbx)**
