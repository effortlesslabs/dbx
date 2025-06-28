#!/bin/bash

# =============================================================================
# DBX INTERACTIVE RELEASE HELPER
# =============================================================================
# 
# DESCRIPTION:
#   User-friendly interactive wrapper script that prompts for all necessary
#   information and then calls the full release script. This makes the release
#   process more accessible for developers who prefer interactive workflows.
#
# WHAT IT DOES:
#   1. Prompts for new version number
#   2. Collects Docker Hub credentials
#   3. Collects NPM token
#   4. Validates all inputs
#   5. Confirms before proceeding
#   6. Calls publish-release.sh with collected parameters
#
# WHEN TO USE:
#   - Interactive releases
#   - When you want to be prompted for credentials
#   - Development workflows
#   - When you prefer guided release process
#
# Quick publishing script for DBX
# This script prompts for credentials and publishes both Docker image and TypeScript SDK

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Pre-flight checks
log_info "🔍 Running pre-flight checks..."

# Check required tools
if ! check_required_tools "git"; then
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "bindings/redis_ts/package.json" ] || [ ! -f "Dockerfile" ]; then
    log_error "Required files not found. Are you in the correct directory?"
    log_error "Expected: Cargo.toml, bindings/redis_ts/package.json, Dockerfile"
    exit 1
fi

# Get current version
CURRENT_VERSION=$(get_current_version "Cargo.toml")
if [ $? -ne 0 ]; then
    log_error "Failed to get current version from Cargo.toml"
    exit 1
fi

log_info "Current version: $CURRENT_VERSION"

# Prompt for new version
echo ""
read -p "Enter new version (e.g., 1.0.0): " NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    log_error "Version is required"
    exit 1
fi

# Validate version format
if ! validate_version_format "$NEW_VERSION"; then
    exit 1
fi

if ! validate_semver "$NEW_VERSION"; then
    exit 1
fi

# Check if version already exists
if git tag -l "v$NEW_VERSION" | grep -q "v$NEW_VERSION"; then
    log_warning "Git tag v$NEW_VERSION already exists"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Publishing cancelled"
        exit 0
    fi
fi

# Prompt for Docker credentials
echo ""
log_info "Docker Hub Configuration"
read -p "Docker Hub username (default: $DOCKER_USERNAME): " DOCKER_USERNAME_INPUT
DOCKER_USERNAME=${DOCKER_USERNAME_INPUT:-$DOCKER_USERNAME}

read -s -p "Docker Hub password/token: " DOCKER_PASSWORD
echo ""

if [ -z "$DOCKER_PASSWORD" ]; then
    log_error "Docker password/token is required"
    exit 1
fi

# Prompt for NPM token
echo ""
log_info "NPM Configuration"
read -s -p "NPM token: " NPM_TOKEN
echo ""

if [ -z "$NPM_TOKEN" ]; then
    log_error "NPM token is required"
    exit 1
fi

# Confirm before proceeding
echo ""
log_warning "About to publish version $NEW_VERSION"
echo "Docker Hub: $DOCKER_USERNAME/$DOCKER_REPO"
echo "NPM Package: $NPM_PACKAGE_NAME"
echo ""
echo "This will:"
echo "  • Update version numbers in all files"
echo "  • Run comprehensive tests"
echo "  • Build and publish TypeScript SDK to NPM"
echo "  • Build and push Docker images to Docker Hub"
echo "  • Create git tag v$NEW_VERSION"
echo ""
read -p "Continue? (y/N): " CONFIRM

if [[ ! $CONFIRM =~ ^[Yy]$ ]]; then
    log_info "Publishing cancelled"
    exit 0
fi

# Run the full publishing script
echo ""
log_info "Starting publishing process..."
log_info "Calling: ./scripts/publish-release.sh --version $NEW_VERSION --docker-username $DOCKER_USERNAME --docker-password [HIDDEN] --npm-token [HIDDEN]"

# Export credentials for the child script
export DOCKER_USERNAME
export DOCKER_PASSWORD
export NPM_TOKEN

# Call the full release script
if ./scripts/publish-release.sh \
    --version "$NEW_VERSION" \
    --docker-username "$DOCKER_USERNAME" \
    --docker-password "$DOCKER_PASSWORD" \
    --npm-token "$NPM_TOKEN"; then
    log_success "🎉 Release $NEW_VERSION completed successfully!"
else
    log_error "❌ Release $NEW_VERSION failed"
    exit 1
fi 