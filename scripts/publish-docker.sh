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
# Usage: ./scripts/publish-docker.sh [options]
#
# Options:
#   --tag <tag>             Image tag (default: latest)
#   --push                  Push to Docker Hub after building
#   --platforms <platforms> Comma-separated list of platforms (default: linux/amd64,linux/arm64)
#   --username <username>   Docker Hub username (default: fnlog0)
#   --password <password>   Docker Hub password/token
#   --verbose              Enable verbose output
#   --debug                Enable debug mode
#   --help                  Show this help message

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Default values
TAG="$DOCKER_DEFAULT_TAG"
PUSH=false
USERNAME="$DOCKER_USERNAME"
REPO="$DOCKER_REPO"
PLATFORMS="$DOCKER_PLATFORMS"
DOCKER_PASSWORD=""

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
        --username)
            USERNAME="$2"
            shift 2
            ;;
        --password)
            DOCKER_PASSWORD="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --debug)
            DEBUG=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --tag <tag>             Image tag (default: $DOCKER_DEFAULT_TAG)"
            echo "  --push                  Push to Docker Hub after building"
            echo "  --platforms <platforms> Comma-separated list of platforms (default: $DOCKER_PLATFORMS)"
            echo "  --username <username>   Docker Hub username (default: $DOCKER_USERNAME)"
            echo "  --password <password>   Docker Hub password/token"
            echo "  --verbose              Enable verbose output"
            echo "  --debug                Enable debug mode"
            echo "  --help                  Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  DOCKER_USERNAME         Docker Hub username"
            echo "  DOCKER_PASSWORD         Docker Hub password/token"
            echo "  DOCKER_REPO             Docker repository name"
            echo "  DOCKER_PLATFORMS        Comma-separated platforms"
            echo "  DEBUG                   Enable debug mode"
            echo "  VERBOSE                 Enable verbose output"
            echo ""
            echo "Examples:"
            echo "  $0 --tag latest"
            echo "  $0 --tag v1.0.0 --push --password \$DOCKER_TOKEN"
            echo "  $0 --tag stable --push --platforms linux/arm64"
            echo "  $0 --tag multiarch --push --platforms linux/amd64,linux/arm64,linux/arm/v7"
            echo "  DOCKER_PASSWORD=\$TOKEN $0 --tag latest --push"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Pre-flight checks
log_info "🔍 Running pre-flight checks..."

# Check required tools
if ! check_required_tools "docker" "git"; then
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Dockerfile" ]; then
    log_error "Dockerfile not found. Are you in the correct directory?"
    exit 1
fi

# Validate Docker credentials if pushing
if [ "$PUSH" = true ] && [ -z "$DOCKER_PASSWORD" ]; then
    log_error "Docker password/token is required for pushing. Use --password <token> or set DOCKER_PASSWORD environment variable"
    exit 1
fi

IMAGE_NAME="$USERNAME/$REPO:$TAG"

log_info "🏗️  Building multi-platform Docker image: $IMAGE_NAME"
log_info "📦 Platforms: $PLATFORMS"
log_info "🐳 Repository: $USERNAME/$REPO"

if [ "$PUSH" = true ]; then
    log_info "🚀 Push mode: Images will be pushed to Docker Hub"
else
    log_info "🔨 Local mode: Images will be built locally only"
fi

echo ""

# Step 1: Setup Docker Buildx
log_step "Step 1: Setting up Docker Buildx"
if ! setup_docker_buildx "$DOCKER_BUILDER_NAME"; then
    log_error "Failed to setup Docker Buildx"
    exit 1
fi

# Step 2: Login to Docker Hub (if pushing)
if [ "$PUSH" = true ]; then
    log_step "Step 2: Authenticating with Docker Hub"
    show_progress "Logging in to Docker Hub" 2
    
    if ! echo "$DOCKER_PASSWORD" | docker login -u "$USERNAME" --password-stdin; then
        log_error "Failed to login to Docker Hub"
        exit 1
    fi
    log_success "Successfully authenticated with Docker Hub"
else
    log_info "Step 2: Skipping Docker Hub authentication (not pushing)"
fi

# Step 3: Build multi-platform image
log_step "Step 3: Building multi-platform Docker image"
show_progress "Building Docker image" 3

# Prepare additional tags
ADDITIONAL_TAGS=(
    "$USERNAME/$REPO:${TAG}-amd64"
)

# Build the main multi-platform image
if ! build_docker_image "$IMAGE_NAME" "$PLATFORMS" "$PUSH" "${ADDITIONAL_TAGS[*]}"; then
    log_error "Failed to build multi-platform Docker image"
    exit 1
fi

# Step 4: Create Railway-compatible AMD64-only tag
log_step "Step 4: Creating Railway-compatible AMD64-only tag"
RAILWAY_TAG="$USERNAME/$REPO:${TAG}-${DOCKER_RAILWAY_TAG_SUFFIX}"

if ! build_docker_image "$RAILWAY_TAG" "linux/amd64" "$PUSH"; then
    log_error "Failed to build Railway-compatible AMD64-only tag"
    exit 1
fi

# Step 5: Verify builds
log_step "Step 5: Verifying Docker builds"
show_progress "Verifying image builds" 2

# Check if images were created successfully
if [ "$PUSH" = false ]; then
    if docker images "$IMAGE_NAME" --format "table {{.Repository}}:{{.Tag}}" | grep -q "$IMAGE_NAME"; then
        log_success "Main image verified locally"
    else
        log_warning "Main image not found locally (may be multi-platform)"
    fi
    
    if docker images "$RAILWAY_TAG" --format "table {{.Repository}}:{{.Tag}}" | grep -q "$RAILWAY_TAG"; then
        log_success "Railway-compatible image verified locally"
    else
        log_warning "Railway-compatible image not found locally"
    fi
else
    log_info "Images pushed to Docker Hub - verification will be available shortly"
fi

echo ""
log_success "🎉 Multi-platform Docker image built successfully!"
echo ""
echo "📦 Built images:"
echo "   • Main image: $IMAGE_NAME"
echo "   • AMD64 variant: $USERNAME/$REPO:${TAG}-amd64"
echo "   • Railway-compatible: $RAILWAY_TAG"
echo "📦 Platforms: $PLATFORMS"

if [ "$PUSH" = true ]; then
    echo ""
    echo "✅ Images pushed successfully to Docker Hub!"
    echo "📦 Users can now run on any supported platform:"
    echo "   docker run -d --name dbx-api -p 3000:3000 \\"
    echo "     -e DATABASE_URL=redis://localhost:6379 \\"
    echo "     $IMAGE_NAME"
    echo ""
    echo "🚂 For Railway deployment, use the AMD64-only tag:"
    echo "   $RAILWAY_TAG"
else
    echo ""
    echo "💡 To push to Docker Hub, run:"
    echo "   $0 --tag $TAG --push --password \$DOCKER_TOKEN"
fi

echo ""
echo "📋 Platform-specific usage examples:"
echo ""
echo "1. On ARM64 (Apple Silicon, Raspberry Pi, etc.):"
echo "   docker run --platform linux/arm64 -d --name dbx-api -p 3000:3000 \\"
echo "     -e DATABASE_URL=redis://localhost:6379 \\"
echo "     $IMAGE_NAME"
echo ""
echo "2. On AMD64 (Intel/AMD):"
echo "   docker run --platform linux/amd64 -d --name dbx-api -p 3000:3000 \\"
echo "     -e DATABASE_URL=redis://localhost:6379 \\"
echo "     $IMAGE_NAME"
echo ""
echo "3. Using docker-compose (auto-detects platform):"
echo "   export REDIS_URL=redis://localhost:6379"
echo "   docker-compose up -d"
echo ""
echo "🔍 To inspect the image platforms:"
echo "   docker buildx imagetools inspect $IMAGE_NAME"

# Cleanup
if [ "$CLEANUP_TEMP_FILES" = "true" ]; then
    cleanup_temp_files
fi

log_info "✨ Docker publishing process completed successfully!" 