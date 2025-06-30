#!/bin/bash

# =============================================================================
# DBX PUBLISHING SCRIPTS - CONFIGURATION
# =============================================================================
# 
# This file contains all configuration values for DBX publishing scripts.
# Modify these values to customize the publishing behavior.

# =============================================================================
# DOCKER CONFIGURATION
# =============================================================================

# Docker Hub settings
DOCKER_USERNAME="${DOCKER_USERNAME:-effortlesslabs}"
DOCKER_REPO="${DOCKER_REPO:-0dbx_redis}"
DOCKER_PLATFORMS="${DOCKER_PLATFORMS:-linux/amd64,linux/arm64}"

# Docker build settings
DOCKER_BUILDER_NAME="${DOCKER_BUILDER_NAME:-dbx-multiarch-builder}"
DOCKER_DEFAULT_TAG="${DOCKER_DEFAULT_TAG:-latest}"

# Railway compatibility settings
DOCKER_RAILWAY_TAG_SUFFIX="${DOCKER_RAILWAY_TAG_SUFFIX:-amd64-only}"

# =============================================================================
# NPM CONFIGURATION
# =============================================================================

# NPM package settings
NPM_PACKAGE_NAME="${NPM_PACKAGE_NAME:-@0dbx/redis}"
NPM_PACKAGE_ACCESS="${NPM_PACKAGE_ACCESS:-public}"
NPM_REGISTRY="${NPM_REGISTRY:-https://registry.npmjs.org/}"

# =============================================================================
# VERSION CONFIGURATION
# =============================================================================

# Version files to update
VERSION_FILES=(
    "Cargo.toml"
    "bindings/redis_ts/package.json"
    "Dockerfile"
)

# Version validation
VERSION_REGEX="^[0-9]+\.[0-9]+\.[0-9]+$"

# =============================================================================
# TESTING CONFIGURATION
# =============================================================================

# Test commands (sequential order: adapter → api → client)
RUST_TEST_CMD_ADAPTER="${RUST_TEST_CMD_ADAPTER:-cd crates/adapter && cargo test}"
RUST_TEST_CMD_API="${RUST_TEST_CMD_API:-cd crates/redis_api && cargo test}"
RUST_TEST_CMD_CLIENT="${RUST_TEST_CMD_CLIENT:-cd crates/redis_client && cargo test}"
TYPESCRIPT_TEST_CMD="${TYPESCRIPT_TEST_CMD:-npm run test:run}"
TYPESCRIPT_BUILD_CMD="${TYPESCRIPT_BUILD_CMD:-npm run build}"

# Sequential testing (required for dependency order)
ENABLE_SEQUENTIAL_TESTS="${ENABLE_SEQUENTIAL_TESTS:-true}"
TEST_ORDER=("adapter" "api" "client")

# =============================================================================
# BUILD CONFIGURATION
# =============================================================================

# Build directories
TYPESCRIPT_BUILD_DIR="${TYPESCRIPT_BUILD_DIR:-bindings/redis_ts}"
RUST_BUILD_DIR="${RUST_BUILD_DIR:-.}"

# =============================================================================
# LOGGING CONFIGURATION
# =============================================================================

# Log levels
LOG_LEVEL="${LOG_LEVEL:-info}"  # debug, info, warning, error
ENABLE_COLORED_OUTPUT="${ENABLE_COLORED_OUTPUT:-true}"
ENABLE_PROGRESS_INDICATORS="${ENABLE_PROGRESS_INDICATORS:-true}"

# =============================================================================
# ERROR HANDLING CONFIGURATION
# =============================================================================

# Backup settings
BACKUP_SUFFIX="${BACKUP_SUFFIX:-.bak}"
ENABLE_AUTO_BACKUP="${ENABLE_AUTO_BACKUP:-true}"
ENABLE_AUTO_RESTORE="${ENABLE_AUTO_RESTORE:-true}"

# Cleanup settings
CLEANUP_TEMP_FILES="${CLEANUP_TEMP_FILES:-true}"
TEMP_FILE_PATTERNS=("*.bak" "*.tmp" "*.log")

# =============================================================================
# PERFORMANCE CONFIGURATION
# =============================================================================

# Caching
ENABLE_BUILD_CACHE="${ENABLE_BUILD_CACHE:-true}"
CACHE_DIR="${CACHE_DIR:-.cache}"

# Parallel processing
MAX_PARALLEL_JOBS="${MAX_PARALLEL_JOBS:-2}"

# =============================================================================
# VALIDATION CONFIGURATION
# =============================================================================

# Required tools
REQUIRED_TOOLS=(
    "docker"
    "cargo"
    "npm"
    "git"
)

# Optional tools (warn if missing)
OPTIONAL_TOOLS=(
    "parallel"
    "jq"
    "yq"
)

# =============================================================================
# ENVIRONMENT DETECTION
# =============================================================================

# Detect CI environment
IS_CI="${CI:-false}"
IS_GITHUB_ACTIONS="${GITHUB_ACTIONS:-false}"
IS_GITLAB_CI="${GITLAB_CI:-false}"

# Detect operating system
OS="$(uname -s)"
ARCH="$(uname -m)"

# =============================================================================
# PLATFORM-SPECIFIC CONFIGURATION
# =============================================================================

# macOS specific settings
if [[ "$OS" == "Darwin" ]]; then
    # Use GNU sed on macOS if available
    if command -v gsed > /dev/null 2>&1; then
        SED_CMD="gsed"
    else
        SED_CMD="sed"
    fi
else
    SED_CMD="sed"
fi

# =============================================================================
# NETWORK CONFIGURATION
# =============================================================================

# Timeout settings
DOCKER_PULL_TIMEOUT="${DOCKER_PULL_TIMEOUT:-300}"
NPM_PUBLISH_TIMEOUT="${NPM_PUBLISH_TIMEOUT:-120}"

# Retry settings
MAX_RETRIES="${MAX_RETRIES:-3}"
RETRY_DELAY="${RETRY_DELAY:-5}"

# =============================================================================
# SECURITY CONFIGURATION
# =============================================================================

# Credential validation
VALIDATE_CREDENTIALS="${VALIDATE_CREDENTIALS:-true}"
MASK_CREDENTIALS_IN_LOGS="${MASK_CREDENTIALS_IN_LOGS:-true}"

# =============================================================================
# DEPLOYMENT CONFIGURATION
# =============================================================================

# Default deployment platforms
DEPLOYMENT_PLATFORMS=(
    "docker-hub"
    "npm"
    "github"
)

# Platform-specific settings
DOCKER_HUB_SETTINGS=(
    "multi-platform"
    "railway-compatible"
)

NPM_SETTINGS=(
    "public-access"
    "typescript-support"
)

# =============================================================================
# NOTIFICATION CONFIGURATION
# =============================================================================

# Success/failure notifications
ENABLE_NOTIFICATIONS="${ENABLE_NOTIFICATIONS:-false}"
NOTIFICATION_WEBHOOK="${NOTIFICATION_WEBHOOK:-}"

# =============================================================================
# DEBUGGING CONFIGURATION
# =============================================================================

# Debug mode
DEBUG="${DEBUG:-false}"
VERBOSE="${VERBOSE:-false}"

# Debug output
if [ "$DEBUG" = "true" ]; then
    set -x
fi

# =============================================================================
# EXPORT CONFIGURATION
# =============================================================================

# Export all configuration variables
export DOCKER_USERNAME DOCKER_REPO DOCKER_PLATFORMS
export DOCKER_BUILDER_NAME DOCKER_DEFAULT_TAG
export NPM_PACKAGE_NAME NPM_PACKAGE_ACCESS NPM_REGISTRY
export VERSION_FILES VERSION_REGEX
export RUST_TEST_CMD_ADAPTER RUST_TEST_CMD_API RUST_TEST_CMD_CLIENT
export TYPESCRIPT_TEST_CMD TYPESCRIPT_BUILD_CMD
export ENABLE_SEQUENTIAL_TESTS TEST_ORDER
export TYPESCRIPT_BUILD_DIR RUST_BUILD_DIR
export LOG_LEVEL ENABLE_COLORED_OUTPUT ENABLE_PROGRESS_INDICATORS
export BACKUP_SUFFIX ENABLE_AUTO_BACKUP ENABLE_AUTO_RESTORE
export CLEANUP_TEMP_FILES TEMP_FILE_PATTERNS
export ENABLE_BUILD_CACHE CACHE_DIR
export MAX_PARALLEL_JOBS
export REQUIRED_TOOLS OPTIONAL_TOOLS
export IS_CI IS_GITHUB_ACTIONS IS_GITLAB_CI
export OS ARCH SED_CMD
export DOCKER_PULL_TIMEOUT NPM_PUBLISH_TIMEOUT
export MAX_RETRIES RETRY_DELAY
export VALIDATE_CREDENTIALS MASK_CREDENTIALS_IN_LOGS
export DEPLOYMENT_PLATFORMS DOCKER_HUB_SETTINGS NPM_SETTINGS
export ENABLE_NOTIFICATIONS NOTIFICATION_WEBHOOK
export DEBUG VERBOSE 