#!/bin/bash

# =============================================================================
# DBX TEST WITH SERVER SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Starts the DBX API server with proper environment configuration
#   and then runs all crate tests against the running server.
#
# WHAT IT DOES:
#   1. Sets up environment variables from .env file
#   2. Starts Redis service (if not already running)
#   3. Starts DBX API server with proper configuration
#   4. Waits for server to be ready
#   5. Runs all crate tests against the running server
#   6. Cleans up server and services
#
# Usage: ./scripts/test-with-server.sh [options]
#
# Options:
#   --env-file <path>       Path to .env file (default: .env)
#   --redis-url <url>       Redis connection URL (overrides .env)
#   --server-port <port>    Server port (default: 3000)
#   --skip-server           Skip starting server (assume it's already running)
#   --skip-cleanup          Don't stop server after tests
#   --verbose               Enable verbose output
#   --help                  Show this help message

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Default values
ENV_FILE=".env"
REDIS_URL=""
SERVER_PORT="3000"
SKIP_SERVER=false
SKIP_REDIS=false
SKIP_CLEANUP=false
VERBOSE=false
SERVER_PID=""
DOCKER_CONTAINER_NAME="dbx-redis-api-test-server"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --env-file)
            ENV_FILE="$2"
            shift 2
            ;;
        --redis-url)
            REDIS_URL="$2"
            shift 2
            ;;
        --server-port)
            SERVER_PORT="$2"
            shift 2
            ;;
        --skip-server)
            SKIP_SERVER=true
            shift
            ;;
        --skip-redis)
            SKIP_REDIS=true
            shift
            ;;
        --skip-cleanup)
            SKIP_CLEANUP=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --env-file <path>       Path to .env file (default: .env)"
            echo "  --redis-url <url>       Redis connection URL (overrides .env)"
            echo "  --server-port <port>    Server port (default: 3000)"
            echo "  --skip-server           Skip starting server (assume it's already running)"
            echo "  --skip-redis            Skip starting Redis"
            echo "  --skip-cleanup          Don't stop server after tests"
            echo "  --verbose               Enable verbose output"
            echo "  --help                  Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0"
            echo "  $0 --env-file .env.test"
            echo "  $0 --redis-url redis://localhost:6379 --server-port 3001"
            echo "  $0 --skip-server --skip-cleanup"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Set verbose mode
if [ "$VERBOSE" = true ]; then
    set -x
fi

echo "🧪 DBX Test with Server"
echo "======================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    log_error "Cargo.toml not found. Are you in the correct directory?"
    exit 1
fi

# Check required tools
log_info "Checking required tools..."
if ! check_required_tools "cargo" "docker"; then
    exit 1
fi

# Load environment variables
load_environment() {
    log_step "Loading environment configuration..."
    
    # Load from .env file if it exists
    if [ -f "$ENV_FILE" ]; then
        log_info "Loading environment from $ENV_FILE"
        export $(grep -v '^#' "$ENV_FILE" | xargs)
    else
        log_warning "Environment file $ENV_FILE not found, using defaults"
    fi
    
    # Override with command line arguments
    if [ -n "$REDIS_URL" ]; then
        export REDIS_URL="$REDIS_URL"
        log_info "Using Redis URL from command line: $REDIS_URL"
    fi
    
    # Set defaults if not provided
    export REDIS_URL="${REDIS_URL:-redis://localhost:6379}"
    export HOST="${HOST:-0.0.0.0}"
    export PORT="${PORT:-$SERVER_PORT}"
    export POOL_SIZE="${POOL_SIZE:-10}"
    export LOG_LEVEL="${LOG_LEVEL:-INFO}"
    
    log_info "Environment configuration:"
    log_info "  Redis URL: $REDIS_URL"
    log_info "  Host: $HOST"
    log_info "  Port: $PORT"
    log_info "  Pool Size: $POOL_SIZE"
    log_info "  Log Level: $LOG_LEVEL"
}

# Start Redis service
start_redis() {
    if [ "$SKIP_REDIS" = true ]; then
        log_info "Skipping Redis start (--skip-redis flag)"
        return 0
    fi
    
    log_step "Starting Redis service..."
    
    # Check if Redis is already running
    if docker ps --format "table {{.Names}}" | grep -q "redis-test"; then
        log_info "Redis container already running"
        return 0
    fi
    
    # Start Redis container
    docker run -d \
        --name redis-test \
        -p 6379:6379 \
        redis:7-alpine \
        redis-server --appendonly yes
    
    # Wait for Redis to be ready
    log_info "Waiting for Redis to be ready..."
    for i in {1..30}; do
        if docker exec redis-test redis-cli ping > /dev/null 2>&1; then
            log_success "Redis is ready"
            return 0
        fi
        sleep 1
    done
    
    log_error "Redis failed to start within 30 seconds"
    return 1
}

# Start DBX API server
start_server() {
    if [ "$SKIP_SERVER" = true ]; then
        log_info "Skipping server start (--skip-server flag)"
        return 0
    fi
    
    log_step "Starting DBX API server..."
    
    # Stop existing container if it exists
    docker stop "$DOCKER_CONTAINER_NAME" 2>/dev/null || true
    docker rm "$DOCKER_CONTAINER_NAME" 2>/dev/null || true
    
    # Build the image
    log_info "Building DBX Redis API Docker image..."
    docker build -t dbx-redis-api:test .
    
    # Start the server
    docker run -d \
        --name "$DOCKER_CONTAINER_NAME" \
        -p "$PORT:3000" \
        -e DATABASE_URL="$REDIS_URL" \
        -e DATABASE_TYPE=redis \
        -e HOST=0.0.0.0 \
        -e PORT=3000 \
        -e POOL_SIZE="$POOL_SIZE" \
        -e LOG_LEVEL="$LOG_LEVEL" \
        dbx-redis-api:test
    
    # Wait for server to be ready
    log_info "Waiting for server to be ready..."
    for i in {1..60}; do
        if curl -s "http://localhost:$PORT/redis/admin/ping" > /dev/null 2>&1; then
            log_success "Server is ready at http://localhost:$PORT"
            return 0
        fi
        sleep 1
    done
    
    log_error "Server failed to start within 60 seconds"
    docker logs "$DOCKER_CONTAINER_NAME"
    return 1
}

# Wait for server to be ready
wait_for_server() {
    if [ "$SKIP_SERVER" = true ]; then
        log_info "Skipping server wait (--skip-server flag)"
        return 0
    fi
    
    log_step "Waiting for server to be ready..."
    for i in {1..30}; do
        if curl -s "http://localhost:$PORT/redis/admin/ping" > /dev/null 2>&1; then
            log_success "Server is ready"
            return 0
        fi
        sleep 1
    done
    
    log_error "Server is not responding"
    return 1
}

# Run crate tests
run_crate_tests() {
    log_step "Running crate tests against server..."
    
    # Set environment variables for tests
    export REDIS_URL="$REDIS_URL"
    export DBX_BASE_URL="http://localhost:$PORT"
    export DBX_WS_HOST_URL="ws://localhost:$PORT/redis_ws"
    
    log_info "Test environment:"
    log_info "  REDIS_URL: $REDIS_URL"
    log_info "  DBX_BASE_URL: $DBX_BASE_URL"
    log_info "  DBX_WS_HOST_URL: $DBX_WS_HOST_URL"
    
    # Run tests in sequential order
    log_info "Running tests sequentially (adapter → api → client)..."
    
    # 1. Adapter tests
    log_step "Running adapter tests..."
    if ! (cd "crates/adapter" && cargo test); then
        log_error "❌ Adapter tests failed"
        return 1
    fi
    log_success "✅ Adapter tests passed"
    
    # 2. API tests
    log_step "Running API tests..."
    if ! (cd "crates/redis_api" && cargo test); then
        log_error "❌ API tests failed"
        return 1
    fi
    log_success "✅ API tests passed"
    
    # 3. Client tests
    log_step "Running client tests..."
    if ! (cd "crates/redis_client" && cargo test); then
        log_error "❌ Client tests failed"
        return 1
    fi
    log_success "✅ Client tests passed"
    
    log_success "🎉 All crate tests passed!"
    return 0
}

# Cleanup function
cleanup() {
    if [ "$SKIP_CLEANUP" = true ]; then
        log_info "Skipping cleanup (--skip-cleanup flag)"
        return 0
    fi
    
    log_step "Cleaning up..."
    
    # Stop and remove DBX server container
    if docker ps --format "table {{.Names}}" | grep -q "$DOCKER_CONTAINER_NAME"; then
        log_info "Stopping DBX server container..."
        docker stop "$DOCKER_CONTAINER_NAME" 2>/dev/null || true
        docker rm "$DOCKER_CONTAINER_NAME" 2>/dev/null || true
    fi
    
    # Stop and remove Redis container
    if docker ps --format "table {{.Names}}" | grep -q "redis-test"; then
        log_info "Stopping Redis container..."
        docker stop redis-test 2>/dev/null || true
        docker rm redis-test 2>/dev/null || true
    fi
    
    log_success "Cleanup completed"
}

# Set up trap for cleanup
trap cleanup EXIT

# Main execution
main() {
    # Load environment
    load_environment
    
    # Start Redis
    start_redis
    
    # Start server
    start_server
    
    # Wait for server
    wait_for_server
    
    # Run tests
    if run_crate_tests; then
        log_success "🎉 All tests completed successfully!"
        echo ""
        echo "📊 Test Summary:"
        echo "   ✅ Adapter tests: PASSED"
        echo "   ✅ API tests: PASSED"
        echo "   ✅ Client tests: PASSED"
        echo ""
        log_info "Ready for next steps! 🚀"
        return 0
    else
        log_error "❌ Tests failed"
        return 1
    fi
}

# Run main function
main "$@" 