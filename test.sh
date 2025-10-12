#!/bin/bash
# test.sh - Runs all termiPet tests correctly
#
# This script handles the race condition in train command tests by running
# tests with --test-threads=1 to ensure sequential execution.
#
# Usage:
#   ./test.sh              # Run all tests
#   ./test.sh --fast       # Run non-train tests in parallel, then train tests
#   ./test.sh --verbose    # Run all tests with output
#   ./test.sh --help       # Show this help

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Function to show help
show_help() {
    echo "termiPet Test Runner"
    echo ""
    echo "Usage: ./test.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --fast      Run non-train tests in parallel, then train tests sequentially"
    echo "              (faster but more complex)"
    echo "  --verbose   Run all tests with output (shows println! statements)"
    echo "  --help      Show this help message"
    echo ""
    echo "Default (no options): Runs all tests sequentially with --test-threads=1"
    echo ""
    echo "Why --test-threads=1?"
    echo "  The train command tests modify the global HOME environment variable,"
    echo "  which causes race conditions when tests run in parallel. Running"
    echo "  sequentially ensures each test has exclusive environment access."
    echo ""
    echo "Examples:"
    echo "  ./test.sh              # Safe, comprehensive (all tests sequential)"
    echo "  ./test.sh --fast       # Faster (parallel + sequential)"
    echo "  ./test.sh --verbose    # Debug test output"
    exit 0
}

# Parse arguments
FAST_MODE=false
VERBOSE_MODE=false

for arg in "$@"; do
    case $arg in
        --fast)
            FAST_MODE=true
            shift
            ;;
        --verbose)
            VERBOSE_MODE=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $arg"
            echo "Run './test.sh --help' for usage information"
            exit 1
            ;;
    esac
done

# Main test execution
echo ""
print_info "Running termiPet tests..."
echo ""

if [ "$FAST_MODE" = true ]; then
    print_info "Fast mode: Running non-train tests in parallel..."

    if cargo test --lib -- --skip train; then
        print_success "Non-train tests passed"
    else
        print_error "Non-train tests failed"
        exit 1
    fi

    echo ""
    print_info "Running train tests sequentially (--test-threads=1)..."

    if cargo test train -- --test-threads=1; then
        print_success "Train tests passed"
    else
        print_error "Train tests failed"
        exit 1
    fi

elif [ "$VERBOSE_MODE" = true ]; then
    print_info "Verbose mode: Running all tests sequentially with output..."

    if cargo test --lib -- --test-threads=1 --nocapture; then
        print_success "All tests passed"
    else
        print_error "Tests failed"
        exit 1
    fi

else
    print_info "Running all tests sequentially (--test-threads=1)..."

    if cargo test --lib -- --test-threads=1; then
        print_success "All tests passed"
    else
        print_error "Tests failed"
        exit 1
    fi
fi

echo ""
print_success "All tests completed successfully! 🎉"
echo ""
