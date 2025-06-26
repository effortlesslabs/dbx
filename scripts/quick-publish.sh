#!/bin/bash

# Quick publishing script for DBX
# This script prompts for credentials and publishes both Docker image and TypeScript SDK

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Get current version
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
log_info "Current version: $CURRENT_VERSION"

# Prompt for new version
echo ""
read -p "Enter new version (e.g., 1.0.0): " NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    log_error "Version is required"
    exit 1
fi

# Validate version format
if [[ ! $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

# Prompt for Docker credentials
echo ""
read -p "Docker Hub username (default: fnlog0): " DOCKER_USERNAME
DOCKER_USERNAME=${DOCKER_USERNAME:-fnlog0}

read -s -p "Docker Hub password/token: " DOCKER_PASSWORD
echo ""

if [ -z "$DOCKER_PASSWORD" ]; then
    log_error "Docker password/token is required"
    exit 1
fi

# Prompt for NPM token
echo ""
read -s -p "NPM token: " NPM_TOKEN
echo ""

if [ -z "$NPM_TOKEN" ]; then
    log_error "NPM token is required"
    exit 1
fi

# Confirm before proceeding
echo ""
log_warning "About to publish version $NEW_VERSION"
echo "Docker Hub: $DOCKER_USERNAME/dbx"
echo "NPM Package: dbx-sdk"
echo ""
read -p "Continue? (y/N): " CONFIRM

if [[ ! $CONFIRM =~ ^[Yy]$ ]]; then
    log_info "Publishing cancelled"
    exit 0
fi

# Run the full publishing script
echo ""
log_info "Starting publishing process..."
./scripts/publish-release.sh \
    --version "$NEW_VERSION" \
    --docker-username "$DOCKER_USERNAME" \
    --docker-password "$DOCKER_PASSWORD" \
    --npm-token "$NPM_TOKEN" 