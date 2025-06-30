#!/bin/bash

# =============================================================================
# DBX DOCKER-ONLY PUBLISHING SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Focused Docker image building and publishing script for DBX. This script
#   only handles Docker operations and is optimized for multi-platform builds
#   with special consideration for Railway deployment compatibility.
#
# WHAT IT DOES:
#   1. Builds multi-platform Docker images (linux/amd64, linux/arm64)
#   2. Creates Railway-compatible AMD64-only tags
#   3. Pushes images to Docker Hub (optional)
#   4. Provides platform-specific usage examples
#
# WHEN TO USE:
#   - Quick Docker-only deployments
#   - Testing Docker builds
#   - When you only need Docker images (not NPM packages)
#   - Railway deployments (uses AMD64-only tags)
#
# Script to build and publish DBX API to Docker Hub with multi-platform support
# Default: linux/amd64,linux/arm64 (multi-arch, works for Railway, Apple Silicon, etc)
# Usage: ./scripts/publish.sh [options]
#
# Options:
#   --tag <tag>             Image tag (default: latest)
#   --push                  Push to Docker Hub after building
#   --platforms <platforms> Comma-separated list of platforms (default: linux/amd64,linux/arm64)
#   --help                  Show this help message

set -e

# Default values
TAG="latest"
PUSH=false
USERNAME="effortlesslabs"
REPO="dbx"
# Default: multi-arch for Railway and all major platforms
PLATFORMS="linux/amd64,linux/arm64"

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
        --platforms)
            PLATFORMS="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --tag <tag>             Image tag (default: latest)"
            echo "  --push                  Push to Docker Hub after building"
            echo "  --platforms <platforms> Comma-separated list of platforms (default: linux/amd64,linux/arm64)"
            echo "  --help                  Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --tag latest"
            echo "  $0 --tag v1.0.0 --push"
            echo "  $0 --tag stable --push --platforms linux/arm64"
            echo "  $0 --tag multiarch --push --platforms linux/amd64,linux/arm64,linux/arm/v7"
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

echo "🏗️  Building multi-platform Docker image: $IMAGE_NAME"
echo "📦 Platforms: $PLATFORMS"
echo ""

# Check if Docker Buildx is available
if ! docker buildx version > /dev/null 2>&1; then
    echo "❌ Error: Docker Buildx is not available. Please install Docker Buildx."
    echo "   You can install it with: docker buildx install"
    exit 1
fi

# Create a new builder instance if it doesn't exist
BUILDER_NAME="dbx-multiarch-builder"
if ! docker buildx inspect "$BUILDER_NAME" > /dev/null 2>&1; then
    echo "🔧 Creating new buildx builder: $BUILDER_NAME"
    docker buildx create --name "$BUILDER_NAME" --use
else
    echo "🔧 Using existing buildx builder: $BUILDER_NAME"
    docker buildx use "$BUILDER_NAME"
fi

# Build the multi-platform image
if [ "$PUSH" = true ]; then
    echo "🚀 Building and pushing multi-platform image..."
    docker buildx build \
        --platform "$PLATFORMS" \
        --tag "$IMAGE_NAME" \
        --tag "$USERNAME/$REPO:${TAG}-amd64" \
        --push \
        .
    
    # Also create an AMD64-only tag for Railway compatibility
    echo "🔧 Creating AMD64-only tag for Railway compatibility..."
    docker buildx build \
        --platform linux/amd64 \
        --tag "$USERNAME/$REPO:${TAG}-amd64-only" \
        --push \
        .
else
    echo "🔨 Building multi-platform image (local only)..."
    docker buildx build \
        --platform "$PLATFORMS" \
        --tag "$IMAGE_NAME" \
        --tag "$USERNAME/$REPO:${TAG}-amd64" \
        --load \
        .
    
    # Also create an AMD64-only tag for Railway compatibility
    echo "🔧 Creating AMD64-only tag for Railway compatibility..."
    docker buildx build \
        --platform linux/amd64 \
        --tag "$USERNAME/$REPO:${TAG}-amd64-only" \
        --load \
        .
fi

echo ""
echo "✅ Multi-platform image built successfully!"
echo "🐳 Image: $IMAGE_NAME"
echo "📦 Platforms: $PLATFORMS"
echo "🔧 AMD64-only tag: $USERNAME/$REPO:${TAG}-amd64-only (for Railway)"

if [ "$PUSH" = true ]; then
    echo ""
    echo "✅ Image pushed successfully to Docker Hub!"
    echo "📦 Users can now run on any supported platform:"
    echo "   docker run -d --name dbx-api -p 3000:3000 \
     -e DATABASE_URL=redis://localhost:6379 \
     $IMAGE_NAME"
    echo ""
    echo "🚂 For Railway deployment, use the AMD64-only tag:"
    echo "   $USERNAME/$REPO:${TAG}-amd64-only"
else
    echo ""
    echo "💡 To push to Docker Hub, run:"
    echo "   $0 --tag $TAG --push"
fi

echo ""
echo "📋 Platform-specific usage examples:"
echo ""
echo "1. On ARM64 (Apple Silicon, Raspberry Pi, etc.):"
echo "   docker run --platform linux/arm64 -d --name dbx-api -p 3000:3000 \
     -e DATABASE_URL=redis://localhost:6379 \
     $IMAGE_NAME"
echo ""
echo "2. On AMD64 (Intel/AMD):"
echo "   docker run --platform linux/amd64 -d --name dbx-api -p 3000:3000 \
     -e DATABASE_URL=redis://localhost:6379 \
     $IMAGE_NAME"
echo ""
echo "3. Using docker-compose (auto-detects platform):"
echo "   export REDIS_URL=redis://localhost:6379"
echo "   docker-compose up -d"
echo ""
echo "🔍 To inspect the image platforms:"
echo "   docker buildx imagetools inspect $IMAGE_NAME" 