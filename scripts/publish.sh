#!/bin/bash

# Script to build and publish DBX API to Docker Hub
# Usage: ./scripts/publish.sh [options]
#
# Options:
#   --tag <tag>             Image tag (default: latest)
#   --push                  Push to Docker Hub after building
#   --help                  Show this help message

set -e

# Default values
TAG="latest"
PUSH=false
USERNAME="fnlog0"
REPO="dbx"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --tag)
            TAG="$2"
            shift 2
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --tag <tag>             Image tag (default: latest)"
            echo "  --push                  Push to Docker Hub after building"
            echo "  --help                  Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --tag latest"
            echo "  $0 --tag v1.0.0 --push"
            echo "  $0 --tag stable --push"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

IMAGE_NAME="$USERNAME/$REPO:$TAG"

echo "🏗️  Building Docker image: $IMAGE_NAME"
echo ""

# Build the image
docker build -t "$IMAGE_NAME" .

echo ""
echo "✅ Image built successfully!"
echo "🐳 Image: $IMAGE_NAME"

if [ "$PUSH" = true ]; then
    echo ""
    echo "🚀 Pushing to Docker Hub..."
    
    # Login to Docker Hub if not already logged in
    if ! docker info | grep -q "Username"; then
        echo "Please login to Docker Hub:"
        docker login
    fi
    
    # Push the image
    docker push "$IMAGE_NAME"
    
    echo ""
    echo "✅ Image pushed successfully to Docker Hub!"
    echo "📦 Users can now run:"
    echo "   docker run -d --name dbx-api -p 3000:3000 \\"
    echo "     -e DATABASE_URL=redis://localhost:6379 \\"
    echo "     $IMAGE_NAME"
else
    echo ""
    echo "💡 To push to Docker Hub, run:"
    echo "   $0 --tag $TAG --push"
fi

echo ""
echo "📋 Usage examples for users:"
echo ""
echo "1. Basic usage:"
echo "   docker run -d --name dbx-api -p 3000:3000 \\"
echo "     -e DATABASE_URL=redis://localhost:6379 \\"
echo "     $IMAGE_NAME"
echo ""
echo "2. With all options:"
echo "   docker run -d --name dbx-api -p 8080:3000 \\"
echo "     -e DATABASE_URL=redis://user:pass@redis.com:6379 \\"
echo "     -e POOL_SIZE=20 \\"
echo "     -e LOG_LEVEL=DEBUG \\"
echo "     $IMAGE_NAME"
echo ""
echo "3. Using docker-compose:"
echo "   export REDIS_URL=redis://localhost:6379"
echo "   docker-compose up -d" 