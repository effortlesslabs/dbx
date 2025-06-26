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
NPM_TOKEN=""
UPDATE_VERSION=false
DRY_RUN=false

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
            echo "  --version <version>     Version to publish (e.g., 1.0.0) - optional"
            echo "  --npm-token <token>     NPM token for publishing"
            echo "  --update-version        Update version in package.json before publishing"
            echo "  --dry-run              Show what would be done without executing"
            echo "  --verbose              Enable verbose output"
            echo "  --debug                Enable debug mode"
            echo "  --help                 Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  NPM_TOKEN              NPM authentication token"
            echo "  NPM_PACKAGE_NAME       Package name (default: dbx-sdk)"
            echo "  DEBUG                  Enable debug mode"
            echo "  VERBOSE                Enable verbose output"
            echo ""
            echo "Examples:"
            echo "  $0 --npm-token \$NPM_TOKEN"
            echo "  $0 --version 1.0.0 --npm-token \$NPM_TOKEN --update-version"
            echo "  $0 --version 1.1.0 --dry-run --verbose"
            echo "  NPM_TOKEN=\$TOKEN $0 --update-version"
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
if ! check_required_tools "npm" "node" "git"; then
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "ts/package.json" ]; then
    log_error "TypeScript package.json not found. Are you in the correct directory?"
    exit 1
fi

# Validate required arguments
if [ "$DRY_RUN" = false ] && [ -z "$NPM_TOKEN" ]; then
    log_error "NPM token is required for publishing. Use --npm-token <token> or set NPM_TOKEN environment variable"
    exit 1
fi

# Get current version from package.json
CURRENT_VERSION=$(get_current_version "ts/package.json")
if [ $? -ne 0 ]; then
    log_error "Failed to get current version from ts/package.json"
    exit 1
fi

log_info "Current TypeScript SDK version: $CURRENT_VERSION"

# Determine version to publish
if [ -n "$VERSION" ]; then
    PUBLISH_VERSION="$VERSION"
    log_info "Will publish version: $PUBLISH_VERSION"
    
    # Validate version format
    if ! validate_version_format "$VERSION"; then
        exit 1
    fi
    
    if ! validate_semver "$VERSION"; then
        exit 1
    fi
else
    PUBLISH_VERSION="$CURRENT_VERSION"
    log_info "Will publish current version: $PUBLISH_VERSION"
fi

# Check if version already exists on NPM
if [ "$DRY_RUN" = false ]; then
    log_debug "Checking if version $PUBLISH_VERSION already exists on NPM..."
    if npm view "$NPM_PACKAGE_NAME@$PUBLISH_VERSION" version > /dev/null 2>&1; then
        log_warning "Version $PUBLISH_VERSION already exists on NPM"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Publishing cancelled"
            exit 0
        fi
    fi
fi

log_info "🚀 Starting DBX TypeScript SDK publishing process"
log_info "📦 NPM Package: $NPM_PACKAGE_NAME"
log_info "📦 Version: $PUBLISH_VERSION"
log_info "📦 Registry: $NPM_REGISTRY"

if [ "$DRY_RUN" = true ]; then
    log_warning "DRY RUN MODE - No changes will be made"
fi

echo ""

# Step 1: Update version in package.json (if requested)
if [ "$UPDATE_VERSION" = true ] && [ -n "$VERSION" ]; then
    log_step "Step 1: Updating version in TypeScript package.json"
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
else
    log_info "Step 1: Skipping version update (not requested or no version provided)"
fi

# Step 2: Clean previous build
log_step "Step 2: Cleaning previous build"
if [ "$DRY_RUN" = true ]; then
    echo "Would clean TypeScript build directory"
else
    if ! clean_typescript_build; then
        log_error "Failed to clean TypeScript build directory"
        exit 1
    fi
fi

# Step 3: Run TypeScript tests
log_step "Step 3: Running TypeScript tests"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cd $TYPESCRIPT_BUILD_DIR && $TYPESCRIPT_TEST_CMD"
else
    if ! run_typescript_tests; then
        log_error "TypeScript tests failed"
        exit 1
    fi
fi

# Step 4: Build TypeScript SDK
log_step "Step 4: Building TypeScript SDK"
if [ "$DRY_RUN" = true ]; then
    echo "Would run: cd $TYPESCRIPT_BUILD_DIR && $TYPESCRIPT_BUILD_CMD"
else
    if ! build_typescript_sdk; then
        log_error "TypeScript SDK build failed"
        exit 1
    fi
fi

# Step 5: Publish TypeScript SDK to NPM
log_step "Step 5: Publishing TypeScript SDK to NPM"
if [ "$DRY_RUN" = true ]; then
    echo "Would publish $NPM_PACKAGE_NAME@$PUBLISH_VERSION to NPM"
else
    # Setup NPM authentication
    if ! setup_npm_auth "$NPM_TOKEN"; then
        log_error "Failed to setup NPM authentication"
        exit 1
    fi
    
    # Publish with retry logic
    retry_count=0
    while [ $retry_count -lt $MAX_RETRIES ]; do
        if publish_npm_package "$TYPESCRIPT_BUILD_DIR" "$NPM_PACKAGE_ACCESS"; then
            break
        else
            retry_count=$((retry_count + 1))
            if [ $retry_count -lt $MAX_RETRIES ]; then
                log_warning "Publish failed, retrying in $RETRY_DELAY seconds... (attempt $retry_count/$MAX_RETRIES)"
                sleep $RETRY_DELAY
            else
                log_error "Failed to publish after $MAX_RETRIES attempts"
                exit 1
            fi
        fi
    done
fi

# Step 6: Verify publication
if [ "$DRY_RUN" = false ]; then
    log_step "Step 6: Verifying publication"
    show_progress "Verifying package on NPM" 3
    
    if npm view "$NPM_PACKAGE_NAME@$PUBLISH_VERSION" version > /dev/null 2>&1; then
        log_success "Package verified on NPM"
    else
        log_warning "Package verification failed (may take a few minutes to appear)"
    fi
fi

echo ""
log_success "🎉 TypeScript SDK $PUBLISH_VERSION published successfully!"
echo ""
echo "📦 Published artifact:"
echo "   • TypeScript SDK: $NPM_PACKAGE_NAME@$PUBLISH_VERSION"
echo ""
echo "🔗 Installation commands:"
echo "   # NPM"
echo "   npm install $NPM_PACKAGE_NAME@$PUBLISH_VERSION"
echo ""
echo "   # Yarn"
echo "   yarn add $NPM_PACKAGE_NAME@$PUBLISH_VERSION"
echo ""
echo "   # PNPM"
echo "   pnpm add $NPM_PACKAGE_NAME@$PUBLISH_VERSION"
echo ""
echo "📋 Next steps:"
echo "   1. Verify the package on NPM: https://www.npmjs.com/package/$NPM_PACKAGE_NAME"
echo "   2. Test the new version in a project"
echo "   3. Update documentation if needed"
echo "   4. Consider creating a GitHub release if this is a significant update"

# Cleanup
if [ "$CLEANUP_TEMP_FILES" = "true" ]; then
    cleanup_temp_files
fi

log_info "✨ NPM publishing process completed successfully!" 