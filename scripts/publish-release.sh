#!/bin/bash

# =============================================================================
# DBX FULL RELEASE SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Complete release automation script for DBX that handles the entire release
#   process from version bumping to publishing both Docker images and TypeScript SDK.
#   This is the main script used for official releases and CI/CD pipelines.
#
# WHAT IT DOES:
#   1. Updates version numbers in Cargo.toml, package.json, and Dockerfile
#   2. Runs comprehensive tests (Rust + TypeScript)
#   3. Builds TypeScript SDK
#   4. Publishes TypeScript SDK to NPM
#   5. Builds and pushes multi-platform Docker images
#   6. Creates git tags and commits
#
# WHEN TO USE:
#   - Official releases
#   - CI/CD pipelines
#   - When you have all credentials ready
#   - When you need both Docker and NPM publishing
#
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
#   --verbose              Enable verbose output
#   --debug                Enable debug mode
#   --help                 Show this help message

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Default values
VERSION=""
DOCKER_USERNAME="$DOCKER_USERNAME"
DOCKER_PASSWORD=""
NPM_TOKEN=""
DRY_RUN=false

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
            echo "  --version <version>     Version to publish (e.g., 1.0.0)"
            echo "  --docker-username <user> Docker Hub username (default: $DOCKER_USERNAME)"
            echo "  --docker-password <pass> Docker Hub password/token"
            echo "  --npm-token <token>     NPM token for publishing"
            echo "  --dry-run              Show what would be done without executing"
            echo "  --verbose              Enable verbose output"
            echo "  --debug                Enable debug mode"
            echo "  --help                 Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  DOCKER_USERNAME         Docker Hub username"
            echo "  DOCKER_PASSWORD         Docker Hub password/token"
            echo "  NPM_TOKEN              NPM authentication token"
            echo "  DEBUG                  Enable debug mode"
            echo "  VERBOSE                Enable verbose output"
            echo ""
            echo "Examples:"
            echo "  $0 --version 1.0.0 --docker-password \$DOCKER_TOKEN --npm-token \$NPM_TOKEN"
            echo "  $0 --version 1.1.0 --dry-run --verbose"
            echo "  DOCKER_PASSWORD=\$DOCKER_TOKEN NPM_TOKEN=\$NPM_TOKEN $0 --version 1.0.0"
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
if ! check_required_tools "docker" "cargo" "npm" "node" "git"; then
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "ts/package.json" ] || [ ! -f "Dockerfile" ]; then
    log_error "Required files not found. Are you in the correct directory?"
    log_error "Expected: Cargo.toml, ts/package.json, Dockerfile"
    exit 1
fi

# Validate required arguments
if [ -z "$VERSION" ]; then
    log_error "Version is required. Use --version <version>"
    exit 1
fi

if [ "$DRY_RUN" = false ]; then
    if [ -z "$DOCKER_PASSWORD" ]; then
        log_error "Docker password/token is required for publishing. Use --docker-password <token> or set DOCKER_PASSWORD environment variable"
        exit 1
    fi

    if [ -z "$NPM_TOKEN" ]; then
        log_error "NPM token is required for publishing. Use --npm-token <token> or set NPM_TOKEN environment variable"
        exit 1
    fi
fi

# Validate version format
if ! validate_version_format "$VERSION"; then
    exit 1
fi

if ! validate_semver "$VERSION"; then
    exit 1
fi

# Get current versions for comparison
CURRENT_CARGO_VERSION=$(get_current_version "Cargo.toml")
CURRENT_NPM_VERSION=$(get_current_version "ts/package.json")

log_info "🚀 Starting DBX release process for version $VERSION"
log_info "📦 Docker Hub: $DOCKER_USERNAME/$DOCKER_REPO"
log_info "📦 NPM Package: $NPM_PACKAGE_NAME"
log_info "📦 Current versions - Cargo: $CURRENT_CARGO_VERSION, NPM: $CURRENT_NPM_VERSION"

if [ "$DRY_RUN" = true ]; then
    log_warning "DRY RUN MODE - No changes will be made"
fi

echo ""

# Step 1: Update version in Cargo.toml
log_step "Step 1: Updating version in Cargo.toml"
if [ "$DRY_RUN" = true ]; then
    echo "Would update workspace version to $VERSION in Cargo.toml"
else
    if update_version_in_file "Cargo.toml" "$VERSION"; then
        log_success "Updated workspace version to $VERSION"
    else
        log_error "Failed to update version in Cargo.toml"
        exit 1
    fi
fi

# Step 2: Update version in TypeScript package.json
log_step "Step 2: Updating version in TypeScript package.json"
if [ "$DRY_RUN" = true ]; then
    echo "Would update version to $VERSION in ts/package.json"
else
    if update_version_in_file "ts/package.json" "$VERSION"; then
        log_success "Updated TypeScript SDK version to $VERSION"
    else
        log_error "Failed to update version in ts/package.json"
        exit 1
    fi
fi

# Step 3: Update Dockerfile label
log_step "Step 3: Updating Dockerfile version label"
if [ "$DRY_RUN" = true ]; then
    echo "Would update version label to $VERSION in Dockerfile"
else
    if update_version_in_file "Dockerfile" "$VERSION"; then
        log_success "Updated Dockerfile version label to $VERSION"
    else
        log_error "Failed to update version in Dockerfile"
        exit 1
    fi
fi

# Step 4: Run comprehensive tests
log_step "Step 4: Running comprehensive tests"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: $RUST_TEST_CMD"
    echo "Would run: cd $TYPESCRIPT_BUILD_DIR && $TYPESCRIPT_TEST_CMD"
else
    if ! run_all_tests; then
        log_error "Tests failed"
        exit 1
    fi
fi

# Step 5: Clean and build TypeScript SDK
log_step "Step 5: Cleaning and building TypeScript SDK"
if [ "$DRY_RUN" = true ]; then
    echo "Would clean TypeScript build directory"
    echo "Would run: cd $TYPESCRIPT_BUILD_DIR && $TYPESCRIPT_BUILD_CMD"
else
    # Clean previous build
    if ! clean_typescript_build; then
        log_error "Failed to clean TypeScript build directory"
        exit 1
    fi
    
    # Build TypeScript SDK
    if ! build_typescript_sdk; then
        log_error "TypeScript SDK build failed"
        exit 1
    fi
fi

# Step 6: Publish TypeScript SDK to NPM
log_step "Step 6: Publishing TypeScript SDK to NPM"
if [ "$DRY_RUN" = true ]; then
    echo "Would publish $NPM_PACKAGE_NAME@$VERSION to NPM"
else
    # Setup NPM authentication
    if ! setup_npm_auth "$NPM_TOKEN"; then
        log_error "Failed to setup NPM authentication"
        exit 1
    fi
    
    # Publish with retry logic
    local retry_count=0
    while [ $retry_count -lt $MAX_RETRIES ]; do
        if publish_npm_package "$TYPESCRIPT_BUILD_DIR" "$NPM_PACKAGE_ACCESS"; then
            break
        else
            retry_count=$((retry_count + 1))
            if [ $retry_count -lt $MAX_RETRIES ]; then
                log_warning "NPM publish failed, retrying in $RETRY_DELAY seconds... (attempt $retry_count/$MAX_RETRIES)"
                sleep $RETRY_DELAY
            else
                log_error "Failed to publish to NPM after $MAX_RETRIES attempts"
                exit 1
            fi
        fi
    done
    
    log_success "TypeScript SDK published to NPM as $NPM_PACKAGE_NAME@$VERSION"
fi

# Step 7: Build and push Docker image
log_step "Step 7: Building and pushing Docker image"
if [ "$DRY_RUN" = true ]; then
    echo "Would build and push Docker image: $DOCKER_USERNAME/$DOCKER_REPO:$VERSION"
else
    # Setup Docker Buildx
    if ! setup_docker_buildx "$DOCKER_BUILDER_NAME"; then
        log_error "Failed to setup Docker Buildx"
        exit 1
    fi

    # Login to Docker Hub
    if ! echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin; then
        log_error "Failed to login to Docker Hub"
        exit 1
    fi

    # Build and push multi-platform image
    IMAGE_NAME="$DOCKER_USERNAME/$DOCKER_REPO:$VERSION"
    log_info "Building and pushing multi-platform image: $IMAGE_NAME"
    
    # Prepare additional tags
    ADDITIONAL_TAGS=(
        "$DOCKER_USERNAME/$DOCKER_REPO:latest"
        "$DOCKER_USERNAME/$DOCKER_REPO:${VERSION}-amd64"
    )
    
    if ! build_docker_image "$IMAGE_NAME" "$DOCKER_PLATFORMS" "true" "${ADDITIONAL_TAGS[*]}"; then
        log_error "Failed to build and push Docker image"
        exit 1
    fi
    
    # Create Railway-compatible AMD64-only tag
    RAILWAY_TAG="$DOCKER_USERNAME/$DOCKER_REPO:${VERSION}-${DOCKER_RAILWAY_TAG_SUFFIX}"
    if ! build_docker_image "$RAILWAY_TAG" "linux/amd64" "true"; then
        log_error "Failed to build Railway-compatible tag"
        exit 1
    fi

    log_success "Docker image published: $IMAGE_NAME"
fi

# Step 8: Create git tag
log_step "Step 8: Creating git tag"
if [ "$DRY_RUN" = true ]; then
    echo "Would create git tag: v$VERSION"
    echo "Would push tag to remote"
else
    # Check if tag already exists
    if git tag -l "v$VERSION" | grep -q "v$VERSION"; then
        log_warning "Git tag v$VERSION already exists"
        read -p "Continue and force update? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Git tagging cancelled"
        else
            git tag -d "v$VERSION" 2>/dev/null || true
        fi
    fi
    
    # Add all changes and commit
    git add .
    if git diff --cached --quiet; then
        log_info "No changes to commit"
    else
        git commit -m "Release version $VERSION"
        log_success "Committed version $VERSION"
    fi
    
    # Create and push tag
    git tag "v$VERSION"
    git push origin master
    git push origin "v$VERSION"
    log_success "Git tag v$VERSION created and pushed"
fi

echo ""
log_success "🎉 Release $VERSION published successfully!"
echo ""
echo "📦 Published artifacts:"
echo "   • Docker Image: $DOCKER_USERNAME/$DOCKER_REPO:$VERSION"
echo "   • Docker Latest: $DOCKER_USERNAME/$DOCKER_REPO:latest"
echo "   • Railway Compatible: $DOCKER_USERNAME/$DOCKER_REPO:${VERSION}-${DOCKER_RAILWAY_TAG_SUFFIX}"
echo "   • TypeScript SDK: $NPM_PACKAGE_NAME@$VERSION"
echo "   • Git Tag: v$VERSION"
echo ""
echo "🔗 Installation commands:"
echo "   # Docker"
echo "   docker pull $DOCKER_USERNAME/$DOCKER_REPO:$VERSION"
echo ""
echo "   # TypeScript SDK"
echo "   npm install $NPM_PACKAGE_NAME@$VERSION"
echo ""
echo "📋 Next steps:"
echo "   1. Verify the release on Docker Hub: https://hub.docker.com/r/$DOCKER_USERNAME/$DOCKER_REPO"
echo "   2. Verify the package on NPM: https://www.npmjs.com/package/$NPM_PACKAGE_NAME"
echo "   3. Create a GitHub release with release notes"
echo "   4. Update documentation if needed"

# Cleanup
if [ "$CLEANUP_TEMP_FILES" = "true" ]; then
    cleanup_temp_files
fi

log_info "✨ Full release process completed successfully!" 