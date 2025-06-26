#!/bin/bash

# Script to publish a new release of DBX
# This script handles version bumping, Docker image publishing, and TypeScript SDK publishing
# Usage: ./scripts/publish-release.sh [options]
#
# Options:
#   --version <version>     Version to publish (e.g., 1.0.0)
#   --docker-username <user> Docker Hub username (default: fnlog0)
#   --docker-password <pass> Docker Hub password/token
#   --npm-token <token>     NPM token for publishing
#   --dry-run              Show what would be done without executing
#   --help                 Show this help message

set -e

# Default values
VERSION=""
DOCKER_USERNAME="fnlog0"
DOCKER_PASSWORD=""
NPM_TOKEN=""
DRY_RUN=false
REPO="dbx"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Parse command line arguments
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
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --version <version>     Version to publish (e.g., 1.0.0)"
            echo "  --docker-username <user> Docker Hub username (default: fnlog0)"
            echo "  --docker-password <pass> Docker Hub password/token"
            echo "  --npm-token <token>     NPM token for publishing"
            echo "  --dry-run              Show what would be done without executing"
            echo "  --help                 Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --version 1.0.0 --docker-password \$DOCKER_TOKEN --npm-token \$NPM_TOKEN"
            echo "  $0 --version 1.1.0 --dry-run"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ -z "$VERSION" ]; then
    log_error "Version is required. Use --version <version>"
    exit 1
fi

if [ "$DRY_RUN" = false ]; then
    if [ -z "$DOCKER_PASSWORD" ]; then
        log_error "Docker password/token is required for publishing. Use --docker-password <token>"
        exit 1
    fi

    if [ -z "$NPM_TOKEN" ]; then
        log_error "NPM token is required for publishing. Use --npm-token <token>"
        exit 1
    fi
fi

# Validate version format
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

log_info "🚀 Starting DBX release process for version $VERSION"
log_info "📦 Docker Hub: $DOCKER_USERNAME/$REPO"
log_info "📦 NPM Package: dbx-sdk"

if [ "$DRY_RUN" = true ]; then
    log_warning "DRY RUN MODE - No changes will be made"
fi

echo ""

# Step 1: Update version in Cargo.toml
log_info "Step 1: Updating version in Cargo.toml"
if [ "$DRY_RUN" = true ]; then
    echo "Would update workspace version to $VERSION in Cargo.toml"
else
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    log_success "Updated workspace version to $VERSION"
fi

# Step 2: Update version in TypeScript package.json
log_info "Step 2: Updating version in TypeScript package.json"
if [ "$DRY_RUN" = true ]; then
    echo "Would update version to $VERSION in ts/package.json"
else
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" ts/package.json
    rm ts/package.json.bak
    log_success "Updated TypeScript SDK version to $VERSION"
fi

# Step 3: Update Dockerfile label
log_info "Step 3: Updating Dockerfile version label"
if [ "$DRY_RUN" = true ]; then
    echo "Would update version label to $VERSION in Dockerfile"
else
    sed -i.bak "s/LABEL version=\".*\"/LABEL version=\"$VERSION\"/" Dockerfile
    rm Dockerfile.bak
    log_success "Updated Dockerfile version label to $VERSION"
fi

# Step 4: Run tests
log_info "Step 4: Running tests"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cargo test --all"
    echo "Would run: cd ts && npm run test:run"
else
    # Run Rust tests
    log_info "Running Rust tests..."
    cargo test --all
    log_success "Rust tests passed"

    # Run TypeScript tests
    log_info "Running TypeScript tests..."
    cd ts
    npm run test:run
    cd ..
    log_success "TypeScript tests passed"
fi

# Step 5: Build TypeScript SDK
log_info "Step 5: Building TypeScript SDK"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cd ts && npm run build"
else
    cd ts
    npm run build
    cd ..
    log_success "TypeScript SDK built successfully"
fi

# Step 6: Publish TypeScript SDK to NPM
log_info "Step 6: Publishing TypeScript SDK to NPM"
if [ "$DRY_RUN" = true ]; then
    echo "Would publish dbx-sdk@$VERSION to NPM"
else
    cd ts
    echo "//registry.npmjs.org/:_authToken=$NPM_TOKEN" > ~/.npmrc
    npm publish --access public
    cd ..
    log_success "TypeScript SDK published to NPM as dbx-sdk@$VERSION"
fi

# Step 7: Build and push Docker image
log_info "Step 7: Building and pushing Docker image"
if [ "$DRY_RUN" = true ]; then
    echo "Would build and push Docker image: $DOCKER_USERNAME/$REPO:$VERSION"
else
    # Check if Docker Buildx is available
    if ! docker buildx version > /dev/null 2>&1; then
        log_error "Docker Buildx is not available. Please install Docker Buildx."
        exit 1
    fi

    # Login to Docker Hub
    echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin

    # Create a new builder instance if it doesn't exist
    BUILDER_NAME="dbx-multiarch-builder"
    if ! docker buildx inspect "$BUILDER_NAME" > /dev/null 2>&1; then
        log_info "Creating new buildx builder: $BUILDER_NAME"
        docker buildx create --name "$BUILDER_NAME" --use
    else
        log_info "Using existing buildx builder: $BUILDER_NAME"
        docker buildx use "$BUILDER_NAME"
    fi

    # Build and push multi-platform image
    IMAGE_NAME="$DOCKER_USERNAME/$REPO:$VERSION"
    log_info "Building and pushing multi-platform image: $IMAGE_NAME"
    
    docker buildx build \
        --platform linux/amd64,linux/arm64 \
        --tag "$IMAGE_NAME" \
        --tag "$DOCKER_USERNAME/$REPO:latest" \
        --push \
        .

    log_success "Docker image published: $IMAGE_NAME"
fi

# Step 8: Create git tag
log_info "Step 8: Creating git tag"
if [ "$DRY_RUN" = true ]; then
    echo "Would create git tag: v$VERSION"
    echo "Would push tag to remote"
else
    git add .
    git commit -m "Release version $VERSION"
    git tag "v$VERSION"
    git push origin main
    git push origin "v$VERSION"
    log_success "Git tag v$VERSION created and pushed"
fi

echo ""
log_success "🎉 Release $VERSION published successfully!"
echo ""
echo "📦 Published artifacts:"
echo "   • Docker Image: $DOCKER_USERNAME/$REPO:$VERSION"
echo "   • TypeScript SDK: dbx-sdk@$VERSION"
echo ""
echo "🔗 Installation commands:"
echo "   # Docker"
echo "   docker pull $DOCKER_USERNAME/$REPO:$VERSION"
echo ""
echo "   # TypeScript SDK"
echo "   npm install dbx-sdk@$VERSION"
echo ""
echo "📋 Next steps:"
echo "   1. Verify the release on Docker Hub: https://hub.docker.com/r/$DOCKER_USERNAME/$REPO"
echo "   2. Verify the package on NPM: https://www.npmjs.com/package/dbx-sdk"
echo "   3. Create a GitHub release with release notes"
echo "   4. Update documentation if needed" 