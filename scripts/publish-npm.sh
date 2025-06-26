#!/bin/bash

# =============================================================================
# DBX NPM-ONLY PUBLISHING SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Focused TypeScript SDK publishing script for DBX. This script only handles
#   NPM operations including building, testing, and publishing the TypeScript SDK
#   to NPM registry. Ideal for SDK-only releases or when Docker isn't needed.
#
# WHAT IT DOES:
#   1. Updates version in TypeScript package.json (optional)
#   2. Runs TypeScript tests to ensure quality
#   3. Builds TypeScript SDK (compiles TypeScript to JavaScript)
#   4. Publishes package to NPM registry
#   5. Provides installation instructions
#
# WHEN TO USE:
#   - SDK-only releases
#   - When you only need to update the TypeScript package
#   - Quick SDK updates without Docker changes
#   - When Docker publishing isn't required
#
# Usage: ./scripts/publish-npm.sh [options]
#
# Options:
#   --version <version>     Version to publish (e.g., 1.0.0) - optional, uses current if not specified
#   --npm-token <token>     NPM token for publishing
#   --update-version        Update version in package.json before publishing
#   --dry-run              Show what would be done without executing
#   --help                 Show this help message

set -e

# Default values
VERSION=""
NPM_TOKEN=""
UPDATE_VERSION=false
DRY_RUN=false

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
        --npm-token)
            NPM_TOKEN="$2"
            shift 2
            ;;
        --update-version)
            UPDATE_VERSION=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --version <version>     Version to publish (e.g., 1.0.0) - optional"
            echo "  --npm-token <token>     NPM token for publishing"
            echo "  --update-version        Update version in package.json before publishing"
            echo "  --dry-run              Show what would be done without executing"
            echo "  --help                 Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --npm-token \$NPM_TOKEN"
            echo "  $0 --version 1.0.0 --npm-token \$NPM_TOKEN --update-version"
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
if [ "$DRY_RUN" = false ] && [ -z "$NPM_TOKEN" ]; then
    log_error "NPM token is required for publishing. Use --npm-token <token>"
    exit 1
fi

# Get current version from package.json
CURRENT_VERSION=$(grep '"version"' ts/package.json | cut -d'"' -f4)
log_info "Current TypeScript SDK version: $CURRENT_VERSION"

# Determine version to publish
if [ -n "$VERSION" ]; then
    PUBLISH_VERSION="$VERSION"
    log_info "Will publish version: $PUBLISH_VERSION"
else
    PUBLISH_VERSION="$CURRENT_VERSION"
    log_info "Will publish current version: $PUBLISH_VERSION"
fi

# Validate version format if provided
if [ -n "$VERSION" ] && [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

log_info "🚀 Starting DBX TypeScript SDK publishing process"
log_info "📦 NPM Package: dbx-sdk"
log_info "📦 Version: $PUBLISH_VERSION"

if [ "$DRY_RUN" = true ]; then
    log_warning "DRY RUN MODE - No changes will be made"
fi

echo ""

# Step 1: Update version in package.json (if requested)
if [ "$UPDATE_VERSION" = true ] && [ -n "$VERSION" ]; then
    log_info "Step 1: Updating version in TypeScript package.json"
    if [ "$DRY_RUN" = true ]; then
        echo "Would update version to $VERSION in ts/package.json"
    else
        sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" ts/package.json
        rm ts/package.json.bak
        log_success "Updated TypeScript SDK version to $VERSION"
    fi
else
    log_info "Step 1: Skipping version update (not requested or no version provided)"
fi

# Step 2: Run TypeScript tests
log_info "Step 2: Running TypeScript tests"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cd ts && npm run test:run"
else
    cd ts
    npm run test:run
    cd ..
    log_success "TypeScript tests passed"
fi

# Step 3: Build TypeScript SDK
log_info "Step 3: Building TypeScript SDK"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cd ts && npm run build"
else
    cd ts
    npm run build
    cd ..
    log_success "TypeScript SDK built successfully"
fi

# Step 4: Publish TypeScript SDK to NPM
log_info "Step 4: Publishing TypeScript SDK to NPM"
if [ "$DRY_RUN" = true ]; then
    echo "Would publish dbx-sdk@$PUBLISH_VERSION to NPM"
else
    cd ts
    echo "//registry.npmjs.org/:_authToken=$NPM_TOKEN" > ~/.npmrc
    npm publish --access public
    cd ..
    log_success "TypeScript SDK published to NPM as dbx-sdk@$PUBLISH_VERSION"
fi

echo ""
log_success "🎉 TypeScript SDK $PUBLISH_VERSION published successfully!"
echo ""
echo "📦 Published artifact:"
echo "   • TypeScript SDK: dbx-sdk@$PUBLISH_VERSION"
echo ""
echo "🔗 Installation commands:"
echo "   # NPM"
echo "   npm install dbx-sdk@$PUBLISH_VERSION"
echo ""
echo "   # Yarn"
echo "   yarn add dbx-sdk@$PUBLISH_VERSION"
echo ""
echo "   # PNPM"
echo "   pnpm add dbx-sdk@$PUBLISH_VERSION"
echo ""
echo "📋 Next steps:"
echo "   1. Verify the package on NPM: https://www.npmjs.com/package/dbx-sdk"
echo "   2. Test the new version in a project"
echo "   3. Update documentation if needed"
echo "   4. Consider creating a GitHub release if this is a significant update" 