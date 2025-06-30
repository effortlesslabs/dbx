#!/bin/bash

# =============================================================================
# DBX SIMPLE TEST SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Simple test script that runs crate tests against a running DBX server.
#   Assumes the server is already running and accessible.
#
# WHAT IT DOES:
#   1. Sets up environment variables
#   2. Runs all crate tests against the running server
#   3. Provides clear test results
#
# Usage: ./scripts/test-simple.sh [options]
#
# Options:
#   --redis-url <url>       Redis connection URL (default: redis://localhost:6379)
#   --server-url <url>      Server base URL (default: http://localhost:3000)
#   --verbose               Enable verbose output
#   --help                  Show this help message

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Default values
REDIS_URL="redis://localhost:6379"
SERVER_URL="http://localhost:3000"
VERBOSE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --redis-url)
            REDIS_URL="$2"
            shift 2
            ;;
        --server-url)
            SERVER_URL="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --redis-url <url>       Redis connection URL (default: redis://localhost:6379)"
            echo "  --server-url <url>      Server base URL (default: http://localhost:3000)"
            echo "  --verbose               Enable verbose output"
            echo "  --help                  Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0"
            echo "  $0 --redis-url redis://localhost:6379 --server-url http://localhost:3000"
            echo "  $0 --verbose"
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

echo "🧪 DBX Simple Test Runner"
echo "========================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    log_error "Cargo.toml not found. Are you in the correct directory?"
    exit 1
fi

# Check required tools
log_info "Checking required tools..."
if ! check_required_tools "cargo"; then
    exit 1
fi

# Set up environment variables
setup_environment() {
    log_step "Setting up test environment..."
    
    # Set environment variables for tests
    export REDIS_URL="$REDIS_URL"
    export DBX_BASE_URL="$SERVER_URL"
    export DBX_WS_HOST_URL="${SERVER_URL/http/ws}/redis_ws"
    
    log_info "Test environment:"
    log_info "  REDIS_URL: $REDIS_URL"
    log_info "  DBX_BASE_URL: $DBX_BASE_URL"
    log_info "  DBX_WS_HOST_URL: $DBX_WS_HOST_URL"
}

# Check if server is running
check_server() {
    log_step "Checking if server is running..."
    
    if curl -s "$SERVER_URL/redis/admin/ping" > /dev/null 2>&1; then
        log_success "Server is running and responding"
        return 0
    else
        log_error "Server is not responding at $SERVER_URL/redis/admin/ping"
        log_info "Make sure the DBX server is running before running tests"
        return 1
    fi
}

# Run crate tests
run_crate_tests() {
    log_step "Running crate tests against server..."
    
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

# Main execution
main() {
    # Set up environment
    setup_environment
    
    # Check server
    check_server
    
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