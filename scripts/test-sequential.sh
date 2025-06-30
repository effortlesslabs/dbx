#!/bin/bash

# =============================================================================
# DBX SEQUENTIAL TESTING SCRIPT
# =============================================================================
# 
# DESCRIPTION:
#   Runs tests in the specified sequential order: adapter → api → client
#   This ensures that dependencies are tested in the correct order.
#
# WHAT IT DOES:
#   1. Runs adapter tests first (foundation)
#   2. Runs API tests second (depends on adapter)
#   3. Runs client tests third (depends on both adapter and API)
#   4. Runs TypeScript tests last
#
# WHEN TO USE:
#   - Before publishing
#   - During development to ensure all components work
#   - CI/CD pipelines
#   - When you want to test in dependency order
#
# Usage: ./scripts/test-sequential.sh [options]
#
# Options:
#   --skip-typescript    Skip TypeScript tests
#   --verbose           Enable verbose output
#   --help             Show this help message

set -e

# Source shared functions and configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/common.sh"

# Parse command line arguments
SKIP_TYPESCRIPT=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-typescript)
            SKIP_TYPESCRIPT=true
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
            echo "  --skip-typescript    Skip TypeScript tests"
            echo "  --verbose           Enable verbose output"
            echo "  --help             Show this help message"
            echo ""
            echo "Test Order:"
            echo "  1. Adapter tests (foundation)"
            echo "  2. API tests (depends on adapter)"
            echo "  3. Client tests (depends on adapter and API)"
            echo "  4. TypeScript tests (if not skipped)"
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

echo "🧪 DBX Sequential Testing"
echo "========================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    log_error "Cargo.toml not found. Are you in the correct directory?"
    exit 1
fi

# Check required tools
log_info "Checking required tools..."
if ! check_required_tools "cargo" "npm"; then
    exit 1
fi

# Run tests in sequential order
log_info "Starting sequential test run..."

# 1. Adapter tests
log_step "Step 1: Running adapter tests..."
if ! run_adapter_tests; then
    log_error "❌ Adapter tests failed. Stopping."
    exit 1
fi

# 2. API tests
log_step "Step 2: Running API tests..."
if ! run_api_tests; then
    log_error "❌ API tests failed. Stopping."
    exit 1
fi

# 3. Client tests
log_step "Step 3: Running client tests..."
if ! run_client_tests; then
    log_error "❌ Client tests failed. Stopping."
    exit 1
fi

# 4. TypeScript tests (optional)
if [ "$SKIP_TYPESCRIPT" = false ]; then
    log_step "Step 4: Running TypeScript tests..."
    if ! run_typescript_tests; then
        log_error "❌ TypeScript tests failed."
        exit 1
    fi
else
    log_info "⏭️  Skipping TypeScript tests (--skip-typescript flag)"
fi

echo ""
log_success "🎉 All tests passed successfully!"
echo ""
echo "📊 Test Summary:"
echo "   ✅ Adapter tests: PASSED"
echo "   ✅ API tests: PASSED"
echo "   ✅ Client tests: PASSED"
if [ "$SKIP_TYPESCRIPT" = false ]; then
    echo "   ✅ TypeScript tests: PASSED"
else
    echo "   ⏭️  TypeScript tests: SKIPPED"
fi

echo ""
log_info "Ready for publishing! 🚀" 