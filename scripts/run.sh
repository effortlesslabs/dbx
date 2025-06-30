#!/bin/bash

# Script to run DBX API with published Docker image
# Usage: ./scripts/run.sh [options]
# 
# Options:
#   --redis-url <url>        Redis connection URL (required)
#   --host <host>           Server host (default: 0.0.0.0)
#   --port <port>           Server port (default: 3000)
#   --pool-size <size>      Connection pool size (default: 10)
#   --log-level <level>     Log level (default: INFO)
#   --image <image>         Docker image name (default: build from local Dockerfile)
#   --help                  Show this help message

set -e

# Default values
REDIS_URL=""
HOST="0.0.0.0"
PORT="3000"
POOL_SIZE="10"
LOG_LEVEL="INFO"
IMAGE=""
BUILD_LOCAL=true

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --redis-url)
            REDIS_URL="$2"
            shift 2
            ;;
        --host)
            HOST="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        --pool-size)
            POOL_SIZE="$2"
            shift 2
            ;;
        --log-level)
            LOG_LEVEL="$2"
            shift 2
            ;;
        --image)
            IMAGE="$2"
            BUILD_LOCAL=false
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --redis-url <url>        Redis connection URL (required)"
            echo "  --host <host>           Server host (default: 0.0.0.0)"
            echo "  --port <port>           Server port (default: 3000)"
            echo "  --pool-size <size>      Connection pool size (default: 10)"
            echo "  --log-level <level>     Log level (default: INFO)"
            echo "  --image <image>         Docker image name (default: build from local Dockerfile)"
            echo "  --help                  Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --redis-url redis://localhost:6379"
            echo "  $0 --redis-url redis://user:pass@redis.com:6379 --port 8080"
            echo "  $0 --redis-url redis://localhost:6379 --image effortlesslabs/0dbx_redis:latest"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if Redis URL is provided
if [ -z "$REDIS_URL" ]; then
    echo "Error: --redis-url is required"
    echo "Use --help for usage information"
    exit 1
fi

# Set image name based on whether we're building locally or using published image
if [ "$BUILD_LOCAL" = true ]; then
    IMAGE_NAME="dbx-api:local"
    echo "🚀 Starting DBX API with configuration:"
    echo "   🐳 Image: $IMAGE_NAME (building from local Dockerfile)"
    echo "   📡 Redis URL: $REDIS_URL"
    echo "   🌐 Host: $HOST"
    echo "   🔌 Port: $PORT"
    echo "   🔗 Pool Size: $POOL_SIZE"
    echo "   📝 Log Level: $LOG_LEVEL"
    echo ""
    
    # Build the image locally
    echo "🏗️  Building Docker image..."
    docker build -t "$IMAGE_NAME" .
    echo "✅ Image built successfully!"
else
    IMAGE_NAME="$IMAGE"
    echo "🚀 Starting DBX API with configuration:"
    echo "   🐳 Image: $IMAGE_NAME"
    echo "   📡 Redis URL: $REDIS_URL"
    echo "   🌐 Host: $HOST"
    echo "   🔌 Port: $PORT"
    echo "   🔗 Pool Size: $POOL_SIZE"
    echo "   📝 Log Level: $LOG_LEVEL"
    echo ""
fi

# Stop and remove existing container if it exists
docker stop dbx-api 2>/dev/null || true
docker rm dbx-api 2>/dev/null || true

# Run the container with all variables inline
docker run -d \
  --name dbx-api \
  -p "$PORT:3000" \
  -e DATABASE_URL="$REDIS_URL" \
  -e DATABASE_TYPE=redis \
  -e HOST=0.0.0.0 \
  -e PORT=3000 \
  -e POOL_SIZE="$POOL_SIZE" \
  -e LOG_LEVEL="$LOG_LEVEL" \
  "$IMAGE_NAME"

echo ""
echo "✅ DBX API is starting up..."
echo "🔍 Check logs with: docker logs -f dbx-api"
echo "🛑 Stop with: docker stop dbx-api && docker rm dbx-api"
echo ""
echo "📋 Available endpoints:"
echo "   • HTTP API: http://$HOST:$PORT"
echo "   • Health check: http://$HOST:$PORT/redis/admin/ping"
echo "   • WebSocket: ws://$HOST:$PORT/redis_ws/string/ws"
echo "   • Admin WebSocket: ws://$HOST:$PORT/redis_ws/admin/ws" 