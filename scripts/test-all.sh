#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID TEST ALL - Comprehensive Test Script
# ═══════════════════════════════════════════════════════════════════════════════
# This script runs all tests for the Etrid blockchain project including:
# - Rust unit tests (cargo test for all pallets and modules)
# - JavaScript/TypeScript tests for SDK
# - Frontend tests (if available)
# - Integration tests
# - Property-based tests
# - Test coverage report generation
#
# Usage:
#   ./scripts/test-all.sh [OPTIONS]
#
# Options:
#   --skip-rust        Skip Rust tests
#   --skip-sdk         Skip SDK tests
#   --skip-frontend    Skip frontend tests
#   --skip-integration Skip integration tests
#   --coverage         Generate test coverage report
#   --verbose          Show verbose test output
#   --help             Show this help message
#
# Examples:
#   ./scripts/test-all.sh                 # Run all tests
#   ./scripts/test-all.sh --coverage      # Run all tests with coverage
#   ./scripts/test-all.sh --skip-frontend # Run only Rust and SDK tests
#   ./scripts/test-all.sh --verbose       # Run tests with verbose output
#
# Exit codes:
#   0 - All tests passed
#   1 - One or more tests failed
#
# Requirements:
#   - Rust toolchain with cargo-tarpaulin (for coverage)
#   - Node.js >= 18.0.0 with jest
# ═══════════════════════════════════════════════════════════════════════════════

set +e  # Don't exit on error (we want to run all tests)

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SKIP_RUST=false
SKIP_SDK=false
SKIP_FRONTEND=false
SKIP_INTEGRATION=false
GENERATE_COVERAGE=false
VERBOSE=false
EXIT_CODE=0

# Test results tracking
RUST_TESTS_PASSED=0
RUST_TESTS_FAILED=0
SDK_TESTS_PASSED=0
SDK_TESTS_FAILED=0
FRONTEND_TESTS_PASSED=0
FRONTEND_TESTS_FAILED=0
INTEGRATION_TESTS_PASSED=0
INTEGRATION_TESTS_FAILED=0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_section() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    grep '^#' "$0" | grep -v '#!/usr/bin/env' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

format_duration() {
    local seconds=$1
    local minutes=$((seconds / 60))
    local remaining_seconds=$((seconds % 60))

    if [ $minutes -gt 0 ]; then
        echo "${minutes}m ${remaining_seconds}s"
    else
        echo "${seconds}s"
    fi
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        print_error "Required command '$1' not found"
        return 1
    fi
    return 0
}

# ═══════════════════════════════════════════════════════════════════════════════
# Parse Command Line Arguments
# ═══════════════════════════════════════════════════════════════════════════════

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-rust)
            SKIP_RUST=true
            shift
            ;;
        --skip-sdk)
            SKIP_SDK=true
            shift
            ;;
        --skip-frontend)
            SKIP_FRONTEND=true
            shift
            ;;
        --skip-integration)
            SKIP_INTEGRATION=true
            shift
            ;;
        --coverage)
            GENERATE_COVERAGE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-flight Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID TEST ALL - Starting Test Run"

print_section "Checking Prerequisites"

if [ "$SKIP_RUST" = false ]; then
    check_command "cargo" || exit 1
    print_success "Rust toolchain found: $(rustc --version)"

    if [ "$GENERATE_COVERAGE" = true ]; then
        if ! command -v cargo-tarpaulin &> /dev/null; then
            print_warning "cargo-tarpaulin not found. Install it with: cargo install cargo-tarpaulin"
            print_info "Continuing without coverage..."
            GENERATE_COVERAGE=false
        else
            print_success "cargo-tarpaulin found"
        fi
    fi
fi

if [ "$SKIP_SDK" = false ] || [ "$SKIP_FRONTEND" = false ]; then
    check_command "node" || exit 1
    check_command "npm" || exit 1
    print_success "Node.js found: $(node --version)"
fi

cd "$PROJECT_ROOT"

TOTAL_START_TIME=$(date +%s)

# ═══════════════════════════════════════════════════════════════════════════════
# Run Rust Tests
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_RUST" = false ]; then
    print_header "Running Rust Tests"

    RUST_START_TIME=$(date +%s)

    if [ "$VERBOSE" = true ]; then
        TEST_FLAGS="-- --nocapture"
    else
        TEST_FLAGS=""
    fi

    # Run all workspace tests
    print_section "Running all workspace tests"

    if [ "$GENERATE_COVERAGE" = true ]; then
        print_info "Running tests with coverage (this may take a while)..."

        if cargo tarpaulin --workspace --timeout 300 --out Html --out Xml --output-dir target/coverage 2>&1 | tee /tmp/etrid-test.log; then
            RUST_TESTS_PASSED=1
            print_success "Rust tests passed with coverage"
            print_info "Coverage report: target/coverage/index.html"
            print_info "Coverage XML: target/coverage/cobertura.xml"
        else
            RUST_TESTS_FAILED=1
            EXIT_CODE=1
            print_error "Rust tests failed"
        fi
    else
        if cargo test --workspace $TEST_FLAGS 2>&1 | tee /tmp/etrid-test.log | grep -E "(test result:|error|FAILED)"; then
            if grep -q "test result: FAILED" /tmp/etrid-test.log; then
                RUST_TESTS_FAILED=1
                EXIT_CODE=1
                print_error "Some Rust tests failed"
            else
                RUST_TESTS_PASSED=1
                print_success "All Rust tests passed"
            fi
        else
            RUST_TESTS_FAILED=1
            EXIT_CODE=1
            print_error "Rust test execution failed"
        fi
    fi

    # Run specific pallet tests
    print_section "Running individual pallet tests"

    PALLET_DIRS=(
        "pallets/pallet-reserve-oracle"
        "pallets/pallet-circuit-breaker"
        "pallets/pallet-custodian-registry"
        "pallets/pallet-reserve-vault"
        "pallets/pallet-validator-committee"
        "pallets/pallet-xcm-bridge"
    )

    for pallet_dir in "${PALLET_DIRS[@]}"; do
        if [ -d "$pallet_dir" ] && [ -f "$pallet_dir/Cargo.toml" ]; then
            pallet_name=$(basename "$pallet_dir")
            print_info "Testing $pallet_name..."

            if cargo test --package "$pallet_name" $TEST_FLAGS 2>&1 | tail -n 10 | grep -E "(test result:|FAILED)"; then
                if cargo test --package "$pallet_name" --quiet 2>&1 | grep -q "FAILED"; then
                    print_error "$pallet_name tests failed"
                    RUST_TESTS_FAILED=$((RUST_TESTS_FAILED + 1))
                    EXIT_CODE=1
                else
                    print_success "$pallet_name tests passed"
                    RUST_TESTS_PASSED=$((RUST_TESTS_PASSED + 1))
                fi
            fi
        fi
    done

    # Run integration tests
    if [ "$SKIP_INTEGRATION" = false ]; then
        print_section "Running integration tests"

        if [ -d "tests/integration" ]; then
            if cargo test --package etrid-integration-tests $TEST_FLAGS 2>&1 | tail -n 10; then
                INTEGRATION_TESTS_PASSED=1
                print_success "Integration tests passed"
            else
                INTEGRATION_TESTS_FAILED=1
                EXIT_CODE=1
                print_error "Integration tests failed"
            fi
        else
            print_warning "Integration tests directory not found"
        fi

        # Run property-based tests
        if [ -d "tests/property-based" ]; then
            print_section "Running property-based tests"

            if cargo test --package etrid-property-tests $TEST_FLAGS 2>&1 | tail -n 10; then
                print_success "Property-based tests passed"
            else
                print_error "Property-based tests failed"
                EXIT_CODE=1
            fi
        fi
    fi

    RUST_END_TIME=$(date +%s)
    RUST_DURATION=$((RUST_END_TIME - RUST_START_TIME))

    print_success "Rust tests completed in $(format_duration $RUST_DURATION)"
else
    print_warning "Skipping Rust tests (--skip-rust flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Run SDK Tests
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_SDK" = false ]; then
    print_header "Running SDK Tests"

    SDK_START_TIME=$(date +%s)
    SDK_DIR="13-clients/sdk/js-etrid-sdk"

    if [ -d "$SDK_DIR" ] && [ -f "$SDK_DIR/package.json" ]; then
        cd "$SDK_DIR"

        print_section "Installing SDK dependencies"
        npm install --silent 2>&1 | tail -n 3

        print_section "Running SDK tests"

        if [ "$GENERATE_COVERAGE" = true ]; then
            if npm test -- --coverage 2>&1 | tail -n 20; then
                SDK_TESTS_PASSED=1
                print_success "SDK tests passed with coverage"
                print_info "Coverage report: $SDK_DIR/coverage/lcov-report/index.html"
            else
                SDK_TESTS_FAILED=1
                EXIT_CODE=1
                print_error "SDK tests failed"
            fi
        else
            if npm test 2>&1 | tail -n 20; then
                SDK_TESTS_PASSED=1
                print_success "SDK tests passed"
            else
                SDK_TESTS_FAILED=1
                EXIT_CODE=1
                print_error "SDK tests failed"
            fi
        fi

        cd "$PROJECT_ROOT"

        SDK_END_TIME=$(date +%s)
        SDK_DURATION=$((SDK_END_TIME - SDK_START_TIME))

        print_success "SDK tests completed in $(format_duration $SDK_DURATION)"
    else
        print_warning "SDK directory not found or missing package.json: $SDK_DIR"
    fi
else
    print_warning "Skipping SDK tests (--skip-sdk flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Run Frontend Tests
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_FRONTEND" = false ]; then
    print_header "Running Frontend Tests"

    FRONTEND_START_TIME=$(date +%s)

    FRONTEND_APPS=(
        "apps/wallet-web/etrid-crypto-website:Wallet Web"
        "apps/validator-dashboard:Validator Dashboard"
        "apps/watchtower-monitor:Watchtower Monitor"
    )

    for app_entry in "${FRONTEND_APPS[@]}"; do
        IFS=: read -r app_dir app_name <<< "$app_entry"

        if [ -d "$app_dir" ] && [ -f "$app_dir/package.json" ]; then
            print_section "Testing $app_name"

            cd "$app_dir"

            # Check if test script exists
            if grep -q '"test"' package.json; then
                npm install --silent 2>&1 | tail -n 3

                if npm test 2>&1 | tail -n 20; then
                    FRONTEND_TESTS_PASSED=$((FRONTEND_TESTS_PASSED + 1))
                    print_success "$app_name tests passed"
                else
                    FRONTEND_TESTS_FAILED=$((FRONTEND_TESTS_FAILED + 1))
                    EXIT_CODE=1
                    print_error "$app_name tests failed"
                fi
            else
                print_warning "$app_name has no test script"
            fi

            cd "$PROJECT_ROOT"
        fi
    done

    FRONTEND_END_TIME=$(date +%s)
    FRONTEND_DURATION=$((FRONTEND_END_TIME - FRONTEND_START_TIME))

    print_success "Frontend tests completed in $(format_duration $FRONTEND_DURATION)"
else
    print_warning "Skipping frontend tests (--skip-frontend flag)"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Test Summary
# ═══════════════════════════════════════════════════════════════════════════════

TOTAL_END_TIME=$(date +%s)
TOTAL_DURATION=$((TOTAL_END_TIME - TOTAL_START_TIME))

print_header "Test Results Summary"

echo -e "${CYAN}Test Execution Time: $(format_duration $TOTAL_DURATION)${NC}\n"

# Calculate totals
TOTAL_PASSED=$((RUST_TESTS_PASSED + SDK_TESTS_PASSED + FRONTEND_TESTS_PASSED + INTEGRATION_TESTS_PASSED))
TOTAL_FAILED=$((RUST_TESTS_FAILED + SDK_TESTS_FAILED + FRONTEND_TESTS_FAILED + INTEGRATION_TESTS_FAILED))

echo -e "${CYAN}Results by Component:${NC}"

if [ "$SKIP_RUST" = false ]; then
    if [ $RUST_TESTS_FAILED -eq 0 ]; then
        echo -e "  ${GREEN}✓${NC} Rust Tests: PASSED"
    else
        echo -e "  ${RED}✗${NC} Rust Tests: FAILED"
    fi
fi

if [ "$SKIP_SDK" = false ]; then
    if [ $SDK_TESTS_FAILED -eq 0 ] && [ $SDK_TESTS_PASSED -gt 0 ]; then
        echo -e "  ${GREEN}✓${NC} SDK Tests: PASSED"
    elif [ $SDK_TESTS_FAILED -gt 0 ]; then
        echo -e "  ${RED}✗${NC} SDK Tests: FAILED"
    fi
fi

if [ "$SKIP_FRONTEND" = false ]; then
    if [ $FRONTEND_TESTS_FAILED -eq 0 ] && [ $FRONTEND_TESTS_PASSED -gt 0 ]; then
        echo -e "  ${GREEN}✓${NC} Frontend Tests: PASSED"
    elif [ $FRONTEND_TESTS_FAILED -gt 0 ]; then
        echo -e "  ${RED}✗${NC} Frontend Tests: FAILED"
    fi
fi

if [ "$SKIP_INTEGRATION" = false ]; then
    if [ $INTEGRATION_TESTS_FAILED -eq 0 ] && [ $INTEGRATION_TESTS_PASSED -gt 0 ]; then
        echo -e "  ${GREEN}✓${NC} Integration Tests: PASSED"
    elif [ $INTEGRATION_TESTS_FAILED -gt 0 ]; then
        echo -e "  ${RED}✗${NC} Integration Tests: FAILED"
    fi
fi

echo ""

# Final result
if [ $EXIT_CODE -eq 0 ]; then
    print_success "ALL TESTS PASSED!"
    echo ""
else
    print_error "SOME TESTS FAILED!"
    print_info "Check logs above for details"
    echo ""
fi

# Show coverage reports if generated
if [ "$GENERATE_COVERAGE" = true ]; then
    print_info "Coverage Reports:"
    [ -f "target/coverage/index.html" ] && echo -e "  Rust: ${CYAN}target/coverage/index.html${NC}"
    [ -f "13-clients/sdk/js-etrid-sdk/coverage/lcov-report/index.html" ] && echo -e "  SDK: ${CYAN}13-clients/sdk/js-etrid-sdk/coverage/lcov-report/index.html${NC}"
    echo ""
fi

exit $EXIT_CODE
