#!/bin/bash

# =============================================================================
# DBX PUBLISHING SCRIPTS - SHARED COMMON FUNCTIONS
# =============================================================================
# 
# This file contains shared functions and utilities used by all DBX publishing scripts.
# Source this file in your scripts: source "$(dirname "$0")/common.sh"

set -e

# =============================================================================
# CONFIGURATION
# =============================================================================

# Default values (can be overridden by environment variables)
DEFAULT_DOCKER_USERNAME=${DOCKER_USERNAME:-"effortlesslabs"}
DEFAULT_REPO=${DOCKER_REPO:-"dbx"}
DEFAULT_NPM_PACKAGE=${NPM_PACKAGE:-"dbx-sdk"}
DEFAULT_PLATFORMS=${DOCKER_PLATFORMS:-"linux/amd64,linux/arm64"}

# =============================================================================
# COLOR OUTPUT
# =============================================================================

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# =============================================================================
# LOGGING FUNCTIONS
# =============================================================================

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

log_debug() {
    if [ "${DEBUG:-false}" = "true" ]; then
        echo -e "${PURPLE}🔍 DEBUG: $1${NC}"
    fi
}

log_step() {
    echo -e "${CYAN}📋 $1${NC}"
}

# =============================================================================
# VALIDATION FUNCTIONS
# =============================================================================

validate_version_format() {
    local version="$1"
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        log_error "Invalid version format: $version. Use semantic versioning (e.g., 1.0.0)"
        return 1
    fi
    return 0
}

validate_semver() {
    local version="$1"
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version"
    
    if ! [[ "$major" =~ ^[0-9]+$ ]] || ! [[ "$minor" =~ ^[0-9]+$ ]] || ! [[ "$patch" =~ ^[0-9]+$ ]]; then
        log_error "Invalid semantic version: $version"
        return 1
    fi
    
    if [ "$major" -eq 0 ] && [ "$minor" -eq 0 ] && [ "$patch" -eq 0 ]; then
        log_error "Version cannot be 0.0.0"
        return 1
    fi
    
    return 0
}

check_required_tools() {
    local tools=("$@")
    local missing_tools=()
    
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" > /dev/null 2>&1; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -gt 0 ]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        return 1
    fi
    
    return 0
}

# =============================================================================
# VERSION MANAGEMENT FUNCTIONS
# =============================================================================

get_current_version() {
    local file="$1"
    if [ -f "$file" ]; then
        if [[ "$file" == *"Cargo.toml" ]]; then
            grep '^version = ' "$file" | cut -d'"' -f2
        elif [[ "$file" == *"package.json" ]]; then
            grep '"version"' "$file" | cut -d'"' -f4
        else
            log_error "Unsupported file type for version extraction: $file"
            return 1
        fi
    else
        log_error "File not found: $file"
        return 1
    fi
}

update_version_in_file() {
    local file="$1"
    local new_version="$2"
    local backup_suffix="${3:-.bak}"
    
    if [ ! -f "$file" ]; then
        log_error "File not found: $file"
        return 1
    fi
    
    # Create backup
    cp "$file" "${file}${backup_suffix}"
    
    # Update version based on file type
    if [[ "$file" == *"Cargo.toml" ]]; then
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$file"
    elif [[ "$file" == *"package.json" ]]; then
        sed -i.bak "s/\"version\": \".*\"/\"version\": \"$new_version\"/" "$file"
    elif [[ "$file" == *"Dockerfile" ]]; then
        sed -i.bak "s/LABEL version=\".*\"/LABEL version=\"$new_version\"/" "$file"
    else
        log_error "Unsupported file type for version update: $file"
        return 1
    fi
    
    # Remove temporary backup
    rm "${file}.bak"
    
    log_success "Updated version to $new_version in $file"
    return 0
}

restore_version_files() {
    local backup_suffix="${1:-.bak}"
    local files=("Cargo.toml" "bindings/redis_ts/package.json" "Dockerfile")
    
    for file in "${files[@]}"; do
        if [ -f "${file}${backup_suffix}" ]; then
            mv "${file}${backup_suffix}" "$file"
            log_info "Restored $file from backup"
        fi
    done
}

# =============================================================================
# ENVIRONMENT & CONFIGURATION FUNCTIONS
# =============================================================================

load_environment() {
    # Load from .env file if it exists
    if [ -f ".env" ]; then
        log_debug "Loading environment from .env file"
        export $(grep -v '^#' .env | xargs)
    fi
    
    # Set defaults from environment variables or use defaults
    DOCKER_USERNAME=${DOCKER_USERNAME:-$DEFAULT_DOCKER_USERNAME}
    DOCKER_REPO=${DOCKER_REPO:-$DEFAULT_REPO}
    NPM_PACKAGE=${NPM_PACKAGE:-$DEFAULT_NPM_PACKAGE}
    DOCKER_PLATFORMS=${DOCKER_PLATFORMS:-$DEFAULT_PLATFORMS}
    
    log_debug "Environment loaded - Docker: $DOCKER_USERNAME/$DOCKER_REPO, NPM: $NPM_PACKAGE"
}

# =============================================================================
# TESTING FUNCTIONS
# =============================================================================

run_adapter_tests() {
    log_step "Running adapter tests..."
    if ! (cd "crates/adapter" && cargo test); then
        log_error "Adapter tests failed"
        return 1
    fi
    log_success "Adapter tests passed"
    return 0
}

run_api_tests() {
    log_step "Running API tests..."
    if ! (cd "crates/redis_api" && cargo test); then
        log_error "API tests failed"
        return 1
    fi
    log_success "API tests passed"
    return 0
}

run_client_tests() {
    log_step "Running client tests..."
    if ! (cd "crates/redis_client" && cargo test); then
        log_error "Client tests failed"
        return 1
    fi
    log_success "Client tests passed"
    return 0
}

run_rust_tests() {
    log_step "Running Rust tests sequentially (adapter → api → client)..."
    
    # Run tests in the specified order
    run_adapter_tests || return 1
    run_api_tests || return 1
    run_client_tests || return 1
    
    log_success "All Rust tests passed"
    return 0
}

run_typescript_tests() {
    log_step "Running TypeScript tests..."
    if ! (cd "$TYPESCRIPT_BUILD_DIR" && npm run test:run); then
        log_error "TypeScript tests failed"
        return 1
    fi
    log_success "TypeScript tests passed"
    return 0
}

run_all_tests() {
    log_info "Running comprehensive test suite..."
    
    # Run Rust tests first (sequentially)
    run_rust_tests || return 1
    
    # Then run TypeScript tests
    run_typescript_tests || return 1
    
    log_success "All tests passed"
    return 0
}

# =============================================================================
# BUILD FUNCTIONS
# =============================================================================

clean_typescript_build() {
    local build_dir="${1:-$TYPESCRIPT_BUILD_DIR}"
    
    log_step "Cleaning TypeScript build directory"
    if [ -d "$build_dir/dist" ]; then
        rm -rf "$build_dir/dist"
        log_success "Cleaned TypeScript build directory: $build_dir/dist"
        return 0
    else
        log_info "No TypeScript build directory to clean: $build_dir/dist"
        return 0
    fi
}

# Build TypeScript SDK
build_typescript_sdk() {
    local build_dir="${1:-$TYPESCRIPT_BUILD_DIR}"
    
    log_info "Building TypeScript SDK in $build_dir..."
    
    if [ ! -d "$build_dir" ]; then
        log_error "TypeScript build directory $build_dir not found"
        return 1
    fi
    
    if [ ! -f "$build_dir/package.json" ]; then
        log_error "package.json not found in $build_dir"
        return 1
    fi
    
    # Build the TypeScript SDK
    if ! (cd "$TYPESCRIPT_BUILD_DIR" && npm run build); then
        log_error "TypeScript SDK build failed"
        return 1
    fi
    
    log_success "TypeScript SDK built successfully"
    return 0
}

# =============================================================================
# DOCKER FUNCTIONS
# =============================================================================

setup_docker_buildx() {
    local builder_name="${1:-dbx-multiarch-builder}"
    
    # Check if Docker Buildx is available
    if ! docker buildx version > /dev/null 2>&1; then
        log_error "Docker Buildx is not available. Please install Docker Buildx."
        log_info "You can install it with: docker buildx install"
        return 1
    fi
    
    # Create or use existing builder
    if ! docker buildx inspect "$builder_name" > /dev/null 2>&1; then
        log_info "Creating new buildx builder: $builder_name"
        docker buildx create --name "$builder_name" --use
    else
        log_info "Using existing buildx builder: $builder_name"
        docker buildx use "$builder_name"
    fi
    
    return 0
}

build_docker_image() {
    local image_name="$1"
    local platforms="$2"
    local push="$3"
    local additional_tags="$4"
    
    local build_args="--platform $platforms --tag $image_name"
    
    # Add additional tags
    if [ -n "$additional_tags" ]; then
        for tag in $additional_tags; do
            build_args="$build_args --tag $tag"
        done
    fi
    
    # Add push or load flag
    if [ "$push" = "true" ]; then
        build_args="$build_args --push"
    else
        build_args="$build_args --load"
    fi
    
    log_step "Building Docker image: $image_name"
    log_debug "Build command: docker buildx build $build_args ."
    
    if ! docker buildx build $build_args .; then
        log_error "Docker build failed"
        return 1
    fi
    
    log_success "Docker image built successfully: $image_name"
    return 0
}

# =============================================================================
# NPM FUNCTIONS
# =============================================================================

setup_npm_auth() {
    local npm_token="$1"
    
    if [ -z "$npm_token" ]; then
        log_error "NPM token is required"
        return 1
    fi
    
    # Create .npmrc with authentication
    echo "//registry.npmjs.org/:_authToken=$npm_token" > ~/.npmrc
    log_debug "NPM authentication configured"
    return 0
}

publish_npm_package() {
    local package_dir="$1"
    local access="${2:-public}"
    
    log_step "Publishing NPM package from $package_dir"
    
    if ! (cd "$package_dir" && npm publish --access "$access"); then
        log_error "NPM publish failed"
        return 1
    fi
    
    log_success "NPM package published successfully"
    return 0
}

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================

show_progress() {
    local message="$1"
    local duration="${2:-2}"
    
    echo -n "$message"
    for i in $(seq 1 $duration); do
        echo -n "."
        sleep 0.5
    done
    echo " done!"
}

measure_time() {
    local start_time=$(date +%s)
    "$@"
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    log_info "Operation completed in ${duration}s"
}

cleanup_temp_files() {
    local pattern="$1"
    if [ -n "$pattern" ]; then
        find . -name "$pattern" -type f -delete 2>/dev/null || true
        log_debug "Cleaned up temporary files: $pattern"
    fi
}

# =============================================================================
# ERROR HANDLING
# =============================================================================

cleanup_on_error() {
    local exit_code=$?
    log_error "Script failed with exit code $exit_code"
    
    # Restore version files if they were modified
    restore_version_files
    
    # Cleanup temporary files
    cleanup_temp_files "*.bak"
    cleanup_temp_files "*.tmp"
    
    exit $exit_code
}

# Set up error handling
trap cleanup_on_error ERR

# =============================================================================
# INITIALIZATION
# =============================================================================

# Load environment when this file is sourced
load_environment 

# Update version in multiple files
update_all_versions() {
    local version="$1"
    local files=("Cargo.toml" "bindings/redis_ts/package.json" "Dockerfile")
    
    log_info "Updating version to $version in all files..."
    
    for file in "${files[@]}"; do
        if [ -f "$file" ]; then
            if update_version_in_file "$file" "$version"; then
                log_success "Updated version in $file"
            else
                log_error "Failed to update version in $file"
                return 1
            fi
        else
            log_warning "File $file not found, skipping"
        fi
    done
    
    return 0
} 