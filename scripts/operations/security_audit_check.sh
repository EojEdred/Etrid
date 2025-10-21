#!/bin/bash

# Security Audit Readiness Check
# Ëtrid Protocol - Pre-Audit Validation Script

set -e

echo "======================================"
echo "Ëtrid Security Audit Readiness Check"
echo "======================================"
echo ""
echo "Date: $(date)"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

PROJECT_ROOT="/Users/macbook/Desktop/etrid"
cd "$PROJECT_ROOT"

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

# Function to print test result
print_result() {
    local test_name="$1"
    local result="$2"
    local message="$3"
    
    if [ "$result" = "PASS" ]; then
        echo -e "${GREEN}✓${NC} $test_name"
        ((PASS_COUNT++))
    elif [ "$result" = "FAIL" ]; then
        echo -e "${RED}✗${NC} $test_name: $message"
        ((FAIL_COUNT++))
    elif [ "$result" = "WARN" ]; then
        echo -e "${YELLOW}⚠${NC} $test_name: $message"
        ((WARN_COUNT++))
    fi
}

echo "=== 1. Code Compilation Checks ==="
echo ""

# Check if Rust is installed
if command -v cargo &> /dev/null; then
    print_result "Cargo installed" "PASS"
else
    print_result "Cargo installed" "FAIL" "Cargo not found"
fi

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
print_result "Rust version: $RUST_VERSION" "PASS"

# Check if all PBC runtimes build
echo ""
echo "Checking PBC runtime builds..."
PBC_LIST=(
    "btc-pbc-runtime"
    "eth-pbc-runtime"
    "sol-pbc-runtime"
    "ada-pbc-runtime"
    "xrp-pbc-runtime"
    "trx-pbc-runtime"
    "bnb-pbc-runtime"
    "doge-pbc-runtime"
    "matic-pbc-runtime"
    "link-pbc-runtime"
    "xlm-pbc-runtime"
    "sc-usdt-pbc-runtime"
    "edsc-pbc-runtime"
)

PBC_BUILD_SUCCESS=0
for pbc in "${PBC_LIST[@]}"; do
    if cargo check -p "$pbc" &> /dev/null; then
        ((PBC_BUILD_SUCCESS++))
    fi
done

if [ $PBC_BUILD_SUCCESS -eq 13 ]; then
    print_result "All 13 PBC runtimes compile" "PASS"
else
    print_result "PBC runtime compilation" "FAIL" "$PBC_BUILD_SUCCESS/13 PBCs compile"
fi

echo ""
echo "=== 2. Security Tools ==="
echo ""

# Check cargo-audit
if command -v cargo-audit &> /dev/null; then
    print_result "cargo-audit installed" "PASS"
    echo "  Running cargo audit..."
    if cargo audit 2>&1 | grep -q "Success"; then
        print_result "No known vulnerabilities (cargo audit)" "PASS"
    else
        VULN_COUNT=$(cargo audit 2>&1 | grep -c "vulnerability" || echo "0")
        print_result "Vulnerability scan" "WARN" "$VULN_COUNT vulnerabilities found"
    fi
else
    print_result "cargo-audit installed" "WARN" "Install with: cargo install cargo-audit"
fi

# Check cargo-clippy
if command -v cargo-clippy &> /dev/null; then
    print_result "cargo-clippy installed" "PASS"
else
    print_result "cargo-clippy installed" "WARN" "Install with: rustup component add clippy"
fi

# Check cargo-tarpaulin (code coverage)
if command -v cargo-tarpaulin &> /dev/null; then
    print_result "cargo-tarpaulin installed (coverage)" "PASS"
else
    print_result "cargo-tarpaulin installed" "WARN" "Install with: cargo install cargo-tarpaulin"
fi

echo ""
echo "=== 3. Code Quality Checks ==="
echo ""

# Count TODO/FIXME markers
TODO_COUNT=$(grep -r "TODO\|FIXME\|XXX" --include="*.rs" . 2>/dev/null | grep -v "target/" | wc -l)
if [ "$TODO_COUNT" -lt 50 ]; then
    print_result "TODO/FIXME markers: $TODO_COUNT" "PASS"
elif [ "$TODO_COUNT" -lt 100 ]; then
    print_result "TODO/FIXME markers: $TODO_COUNT" "WARN" "Consider reducing before audit"
else
    print_result "TODO/FIXME markers: $TODO_COUNT" "FAIL" "Too many TODOs for audit"
fi

# Count unwrap() calls (potential panics)
UNWRAP_COUNT=$(grep -r "\.unwrap()" --include="*.rs" . 2>/dev/null | grep -v "target/" | grep -v "test" | wc -l)
if [ "$UNWRAP_COUNT" -lt 20 ]; then
    print_result "unwrap() calls in production: $UNWRAP_COUNT" "PASS"
elif [ "$UNWRAP_COUNT" -lt 50 ]; then
    print_result "unwrap() calls: $UNWRAP_COUNT" "WARN" "Consider using proper error handling"
else
    print_result "unwrap() calls: $UNWRAP_COUNT" "FAIL" "Too many potential panics"
fi

# Count unsafe blocks
UNSAFE_COUNT=$(grep -r "unsafe" --include="*.rs" . 2>/dev/null | grep -v "target/" | grep -v "test" | wc -l)
print_result "unsafe blocks: $UNSAFE_COUNT" "WARN" "All unsafe code needs review"

# Check for panic! calls
PANIC_COUNT=$(grep -r "panic!" --include="*.rs" . 2>/dev/null | grep -v "target/" | grep -v "test" | wc -l)
if [ "$PANIC_COUNT" -lt 10 ]; then
    print_result "panic! calls: $PANIC_COUNT" "PASS"
else
    print_result "panic! calls: $PANIC_COUNT" "WARN" "Review panic conditions"
fi

echo ""
echo "=== 4. Documentation ==="
echo ""

# Check key documentation files
DOC_FILES=(
    "README.md"
    "CONTRIBUTING.md"
    "KNOWN_ISSUES.md"
    "docs/specifications/ivory-paper.md"
    "docs/architecture/overview.md"
    "docs/operations/SECURITY_AUDIT_PREPARATION.md"
)

for doc in "${DOC_FILES[@]}"; do
    if [ -f "$doc" ]; then
        print_result "Documentation: $(basename $doc)" "PASS"
    else
        print_result "Documentation: $doc" "FAIL" "File missing"
    fi
done

# Check if docs are up to date (modified in last 7 days)
RECENT_DOCS=$(find docs -name "*.md" -mtime -7 | wc -l)
if [ "$RECENT_DOCS" -gt 0 ]; then
    print_result "Documentation recently updated" "PASS" "$RECENT_DOCS files updated"
else
    print_result "Documentation freshness" "WARN" "No docs updated in 7 days"
fi

echo ""
echo "=== 5. Testing Infrastructure ==="
echo ""

# Count test files
TEST_COUNT=$(find . -name "*test*.rs" -o -name "tests" -type d | grep -v "target" | wc -l)
if [ "$TEST_COUNT" -gt 50 ]; then
    print_result "Test files present: $TEST_COUNT" "PASS"
elif [ "$TEST_COUNT" -gt 20 ]; then
    print_result "Test files: $TEST_COUNT" "WARN" "Increase test coverage"
else
    print_result "Test files: $TEST_COUNT" "FAIL" "Insufficient tests"
fi

# Check for integration test scripts
if [ -f "test_full_multichain.sh" ]; then
    print_result "Integration test script exists" "PASS"
else
    print_result "Integration test script" "FAIL" "Missing test_full_multichain.sh"
fi

# Check for CI/CD configuration
if [ -f ".github/workflows/ci.yml" ] || [ -f ".gitlab-ci.yml" ]; then
    print_result "CI/CD configuration present" "PASS"
else
    print_result "CI/CD configuration" "WARN" "No CI/CD found"
fi

echo ""
echo "=== 6. Security Best Practices ==="
echo ""

# Check for .env or secrets in git
if git ls-files | grep -q "\.env$"; then
    print_result "No .env in git" "FAIL" ".env files should not be committed"
else
    print_result "No secrets in version control" "PASS"
fi

# Check .gitignore exists
if [ -f ".gitignore" ]; then
    print_result ".gitignore present" "PASS"
else
    print_result ".gitignore" "FAIL" "Missing .gitignore"
fi

# Check for hardcoded IPs or keys
HARDCODED=$(grep -r "private_key\|secret_key\|api_key" --include="*.rs" --include="*.toml" . 2>/dev/null | grep -v "target/" | grep -v "test" | wc -l)
if [ "$HARDCODED" -eq 0 ]; then
    print_result "No hardcoded secrets detected" "PASS"
else
    print_result "Hardcoded secrets" "WARN" "$HARDCODED potential secrets found"
fi

# Check dependencies are locked
if [ -f "Cargo.lock" ]; then
    print_result "Cargo.lock present" "PASS"
else
    print_result "Cargo.lock" "FAIL" "Dependencies not locked"
fi

echo ""
echo "=== 7. Critical Components Status ==="
echo ""

# Check ASF consensus
if [ -d "09-consensus/asf-consensus" ]; then
    print_result "ASF Consensus module exists" "PASS"
else
    print_result "ASF Consensus" "WARN" "Check implementation status"
fi

# Check ËDSC bridge
if [ -d "05-multichain/bridge-protocols/edsc-bridge" ]; then
    print_result "ËDSC Bridge module exists" "PASS"
else
    print_result "ËDSC Bridge" "FAIL" "Missing critical component"
fi

# Check ËtwasmVM
if [ -d "08-etwasm-vm" ]; then
    print_result "ËtwasmVM module exists" "PASS"
else
    print_result "ËtwasmVM" "FAIL" "Missing critical component"
fi

# Check Lightning Bloc
if [ -d "07-transactions/lightning-bloc" ]; then
    print_result "Lightning Bloc module exists" "PASS"
else
    print_result "Lightning Bloc" "WARN" "Check implementation status"
fi

echo ""
echo "=== 8. Build Artifacts ==="
echo ""

# Check if release build exists
if [ -f "target/release/flarechain-node" ]; then
    print_result "FlareChain release build exists" "PASS"
else
    print_result "FlareChain release build" "WARN" "Run: cargo build --release"
fi

# Check build size
if [ -d "target" ]; then
    TARGET_SIZE=$(du -sh target | awk '{print $1}')
    print_result "Build artifacts size: $TARGET_SIZE" "PASS"
fi

echo ""
echo "======================================"
echo "Summary"
echo "======================================"
echo ""
echo -e "${GREEN}Passed:${NC} $PASS_COUNT"
echo -e "${YELLOW}Warnings:${NC} $WARN_COUNT"
echo -e "${RED}Failed:${NC} $FAIL_COUNT"
echo ""

# Calculate readiness score
TOTAL=$((PASS_COUNT + WARN_COUNT + FAIL_COUNT))
SCORE=$((PASS_COUNT * 100 / TOTAL))

echo "Audit Readiness Score: $SCORE%"
echo ""

if [ $SCORE -ge 90 ]; then
    echo -e "${GREEN}Status: READY FOR AUDIT${NC}"
    echo "The codebase appears ready for external security audit."
elif [ $SCORE -ge 75 ]; then
    echo -e "${YELLOW}Status: MOSTLY READY${NC}"
    echo "Address warnings before proceeding with audit."
else
    echo -e "${RED}Status: NOT READY${NC}"
    echo "Critical issues must be resolved before audit."
fi

echo ""
echo "Next steps:"
echo "1. Review warnings and failures above"
echo "2. Run: cargo test --all"
echo "3. Run: cargo clippy --all-targets -- -D warnings"
echo "4. Generate coverage report: cargo tarpaulin"
echo "5. Update KNOWN_ISSUES.md with any discovered issues"
echo "6. Review docs/operations/SECURITY_AUDIT_PREPARATION.md"
echo ""

exit 0
