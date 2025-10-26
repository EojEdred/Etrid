#!/bin/bash
# Ëtrid Pre-Deployment Testing Suite
# Comprehensive tests before mainnet/testnet deployment
# Usage: ./scripts/pre-deployment-tests.sh [--testnet|--mainnet] [--skip-build]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default configuration
NETWORK="testnet"
SKIP_BUILD=false
FAIL_COUNT=0
PASS_COUNT=0
WARN_COUNT=0

# Parse arguments
for arg in "$@"; do
    case $arg in
        --mainnet)
            NETWORK="mainnet"
            shift
            ;;
        --testnet)
            NETWORK="testnet"
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --help)
            echo "Usage: ./scripts/pre-deployment-tests.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --testnet       Run testnet checks (default)"
            echo "  --mainnet       Run mainnet checks (more strict)"
            echo "  --skip-build    Skip runtime build test"
            echo "  --help          Show this help message"
            exit 0
            ;;
    esac
done

# Header
clear
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Ëtrid Pre-Deployment Testing Suite${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Network: ${NETWORK}"
echo "Start time: $(date)"
echo ""

# Test result tracking
pass_test() {
    echo -e "${GREEN}✓ PASS:${NC} $1"
    ((PASS_COUNT++))
}

fail_test() {
    echo -e "${RED}✗ FAIL:${NC} $1"
    ((FAIL_COUNT++))
}

warn_test() {
    echo -e "${YELLOW}⚠ WARN:${NC} $1"
    ((WARN_COUNT++))
}

info_test() {
    echo -e "${BLUE}ℹ INFO:${NC} $1"
}

# ============================================================================
# TEST CATEGORY 1: Environment & Prerequisites
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[1/10] Environment & Prerequisites${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 1.1: Rust toolchain
echo -n "Checking Rust toolchain... "
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    pass_test "Rust ${RUST_VERSION} installed"
else
    fail_test "Rust not installed"
fi

# Test 1.2: Cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version | awk '{print $2}')
    pass_test "Cargo ${CARGO_VERSION} installed"
else
    fail_test "Cargo not installed"
fi

# Test 1.3: Node.js
echo -n "Checking Node.js... "
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    pass_test "Node.js ${NODE_VERSION} installed"
else
    warn_test "Node.js not installed (needed for DEX deployment)"
fi

# Test 1.4: Docker
echo -n "Checking Docker... "
if command -v docker &> /dev/null; then
    DOCKER_VERSION=$(docker --version | awk '{print $3}' | tr -d ',')
    pass_test "Docker ${DOCKER_VERSION} installed"
else
    warn_test "Docker not installed (needed for monitoring)"
fi

# Test 1.5: Git
echo -n "Checking Git... "
if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version | awk '{print $3}')
    pass_test "Git ${GIT_VERSION} installed"
else
    fail_test "Git not installed"
fi

# Test 1.6: Disk space
echo -n "Checking disk space... "
AVAILABLE_SPACE=$(df -h . | tail -1 | awk '{print $4}')
AVAILABLE_SPACE_GB=$(df -BG . | tail -1 | awk '{print $4}' | tr -d 'G')
if [ "$AVAILABLE_SPACE_GB" -ge 50 ]; then
    pass_test "Sufficient disk space: ${AVAILABLE_SPACE}"
else
    warn_test "Low disk space: ${AVAILABLE_SPACE} (recommend 50GB+)"
fi

# Test 1.7: Memory
echo -n "Checking available memory... "
if [[ "$OSTYPE" == "darwin"* ]]; then
    TOTAL_MEM=$(sysctl -n hw.memsize | awk '{print $1/1024/1024/1024}')
else
    TOTAL_MEM=$(free -g | awk '/^Mem:/{print $2}')
fi
if [ "$TOTAL_MEM" -ge 4 ]; then
    pass_test "Sufficient memory: ${TOTAL_MEM}GB"
else
    warn_test "Low memory: ${TOTAL_MEM}GB (recommend 8GB+)"
fi

echo ""

# ============================================================================
# TEST CATEGORY 2: Repository Structure
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[2/10] Repository Structure${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 2.1: Required files exist
echo "Checking required files..."
REQUIRED_FILES=(
    "Cargo.toml"
    "src/main.rs"
    "05-multichain/flare-chain/runtime/Cargo.toml"
    "05-multichain/flare-chain/runtime/src/lib.rs"
    "05-multichain/flare-chain/node/Cargo.toml"
    "scripts/master-deploy.sh"
    "docker-compose.yml"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        pass_test "Found: $file"
    else
        fail_test "Missing: $file"
    fi
done

echo ""

# ============================================================================
# TEST CATEGORY 3: Runtime Configuration
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[3/10] Runtime Configuration${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 3.1: Runtime version
echo -n "Checking runtime version... "
SPEC_VERSION=$(grep "spec_version:" 05-multichain/flare-chain/runtime/src/lib.rs | head -1 | awk '{print $2}' | tr -d ',')
if [ -n "$SPEC_VERSION" ]; then
    pass_test "Runtime version: ${SPEC_VERSION}"
else
    fail_test "Could not determine runtime version"
fi

# Test 3.2: Pallet-vesting configured
echo -n "Checking pallet-vesting... "
if grep -q "pallet_vesting" 05-multichain/flare-chain/runtime/Cargo.toml; then
    pass_test "pallet-vesting configured"
else
    warn_test "pallet-vesting not found in runtime"
fi

# Test 3.3: Pallet-multisig configured
echo -n "Checking pallet-multisig... "
if grep -q "pallet_multisig" 05-multichain/flare-chain/runtime/Cargo.toml; then
    pass_test "pallet-multisig configured"
else
    warn_test "pallet-multisig not found in runtime"
fi

# Test 3.4: Pallet-treasury configured
echo -n "Checking pallet-treasury... "
if grep -q "pallet_treasury" 05-multichain/flare-chain/runtime/Cargo.toml; then
    pass_test "pallet-treasury configured"
else
    warn_test "pallet-treasury not found in runtime"
fi

# Test 3.5: Token decimals
echo -n "Checking token decimals... "
DECIMALS=$(grep "decimals:" res/flarechain.json | awk '{print $2}' | tr -d ',')
if [ "$DECIMALS" == "12" ]; then
    pass_test "Token decimals: 12 (Polkadot standard)"
elif [ "$DECIMALS" == "18" ]; then
    warn_test "Token decimals: 18 (Ethereum standard, should be 12)"
else
    fail_test "Invalid or missing token decimals"
fi

echo ""

# ============================================================================
# TEST CATEGORY 4: Genesis Configuration
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[4/10] Genesis Configuration${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 4.1: Genesis file exists
if [ "$NETWORK" == "mainnet" ]; then
    GENESIS_FILE="05-multichain/flare-chain/runtime/presets/flarechain_mainnet_with_vesting.json"
else
    GENESIS_FILE="05-multichain/flare-chain/runtime/presets/flarechain_testnet.json"
fi

echo -n "Checking genesis file... "
if [ -f "$GENESIS_FILE" ]; then
    pass_test "Found: $GENESIS_FILE"
else
    fail_test "Missing: $GENESIS_FILE"
fi

# Test 4.2: Validate JSON syntax
echo -n "Validating JSON syntax... "
if command -v jq &> /dev/null; then
    if jq empty "$GENESIS_FILE" 2>/dev/null; then
        pass_test "Valid JSON"
    else
        fail_test "Invalid JSON in genesis file"
    fi
else
    warn_test "jq not installed, skipping JSON validation"
fi

# Test 4.3: Check for placeholder addresses (mainnet only)
if [ "$NETWORK" == "mainnet" ]; then
    echo -n "Checking for placeholder addresses... "
    if grep -q "PLACEHOLDER\|ADDRESS_HERE\|REPLACE_ME" "$GENESIS_FILE"; then
        fail_test "Found placeholder addresses in mainnet genesis!"
    else
        pass_test "No placeholder addresses found"
    fi
fi

# Test 4.4: Verify treasury allocation
echo -n "Checking treasury allocation... "
if grep -q "875000000000000000000" "$GENESIS_FILE"; then
    pass_test "Treasury allocation present (875M ÉTR)"
else
    warn_test "Treasury allocation may be missing or incorrect"
fi

echo ""

# ============================================================================
# TEST CATEGORY 5: Runtime Build
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[5/10] Runtime Build${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

if [ "$SKIP_BUILD" == "true" ]; then
    warn_test "Skipping runtime build (--skip-build flag)"
else
    echo "Building runtime (this may take 10-15 minutes)..."
    echo ""

    BUILD_START=$(date +%s)

    if cargo build --release --locked 2>&1 | tee /tmp/etrid-build.log | tail -20; then
        BUILD_END=$(date +%s)
        BUILD_TIME=$((BUILD_END - BUILD_START))
        pass_test "Runtime build successful (${BUILD_TIME}s)"

        # Check for warnings
        WARNINGS=$(grep "warning:" /tmp/etrid-build.log | wc -l)
        if [ "$WARNINGS" -gt 0 ]; then
            warn_test "Build completed with ${WARNINGS} warnings"
        fi
    else
        fail_test "Runtime build failed"
        echo ""
        echo "Last 50 lines of build log:"
        tail -50 /tmp/etrid-build.log
    fi
fi

# Test 5.1: Binary exists
echo -n "Checking compiled binary... "
if [ -f "target/release/etrid" ]; then
    BINARY_SIZE=$(du -h target/release/etrid | cut -f1)
    pass_test "Binary found (${BINARY_SIZE})"
else
    fail_test "Binary not found at target/release/etrid"
fi

# Test 5.2: WASM runtime
echo -n "Checking WASM runtime... "
WASM_RUNTIME=$(find target/release/wbuild -name "*.compact.compressed.wasm" | head -1)
if [ -f "$WASM_RUNTIME" ]; then
    WASM_SIZE=$(du -h "$WASM_RUNTIME" | cut -f1)
    pass_test "WASM runtime found (${WASM_SIZE})"
else
    warn_test "WASM runtime not found"
fi

echo ""

# ============================================================================
# TEST CATEGORY 6: Unit Tests
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[6/10] Unit Tests${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

echo "Running unit tests (this may take several minutes)..."
echo ""

if cargo test --release --workspace 2>&1 | tee /tmp/etrid-tests.log | grep -E "test result:|running"; then
    PASSED=$(grep "test result:" /tmp/etrid-tests.log | tail -1 | awk '{print $4}')
    FAILED=$(grep "test result:" /tmp/etrid-tests.log | tail -1 | awk '{print $6}')

    if [ "$FAILED" == "0" ]; then
        pass_test "All unit tests passed (${PASSED} tests)"
    else
        fail_test "${FAILED} unit tests failed (${PASSED} passed)"
    fi
else
    warn_test "Could not run unit tests"
fi

echo ""

# ============================================================================
# TEST CATEGORY 7: Network Configuration
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[7/10] Network Configuration${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 7.1: Chain spec generation
echo -n "Testing chain spec generation... "
if ./target/release/etrid build-spec --chain local > /tmp/chain-spec.json 2>&1; then
    pass_test "Chain spec generation works"
else
    fail_test "Chain spec generation failed"
fi

# Test 7.2: Validator keys
if [ "$NETWORK" == "mainnet" ]; then
    echo -n "Checking validator keys... "
    if grep -q "session" "$GENESIS_FILE"; then
        pass_test "Session keys present in genesis"
    else
        warn_test "Session keys may be missing from genesis"
    fi
fi

# Test 7.3: Check ports available
echo -n "Checking default ports... "
PORTS_IN_USE=()
for port in 9933 9944 9615 30333; do
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        PORTS_IN_USE+=($port)
    fi
done

if [ ${#PORTS_IN_USE[@]} -eq 0 ]; then
    pass_test "All default ports available"
else
    warn_test "Ports in use: ${PORTS_IN_USE[*]}"
fi

echo ""

# ============================================================================
# TEST CATEGORY 8: Bridge Configuration
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[8/10] Bridge Configuration${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 8.1: Bridge pallets
echo "Checking bridge pallets..."
BRIDGE_PALLETS=(
    "bitcoin-bridge"
    "edsc-bridge"
    "stablecoin-usdt-bridge"
)

for pallet in "${BRIDGE_PALLETS[@]}"; do
    if [ -d "05-multichain/bridge-protocols/$pallet" ]; then
        pass_test "Found: $pallet"
    else
        warn_test "Missing: $pallet"
    fi
done

# Test 8.2: Oracle pallet
echo -n "Checking oracle pallet... "
if [ -d "pallets/pallet-reserve-oracle" ]; then
    pass_test "Oracle pallet present"
else
    fail_test "Oracle pallet missing"
fi

echo ""

# ============================================================================
# TEST CATEGORY 9: DEX Configuration
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[9/10] DEX Configuration${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 9.1: Ethereum contracts
echo -n "Checking Ethereum contracts... "
if [ -f "contracts/ethereum/scripts/deploy-bsc.js" ]; then
    pass_test "BSC deployment script found"
else
    warn_test "BSC deployment script missing"
fi

# Test 9.2: Package.json for hardhat
echo -n "Checking Hardhat configuration... "
if [ -f "contracts/ethereum/package.json" ]; then
    pass_test "package.json found"
else
    warn_test "package.json missing (needed for DEX deployment)"
fi

# Test 9.3: .env file
echo -n "Checking .env file... "
if [ -f "contracts/ethereum/.env" ]; then
    pass_test ".env file present"
else
    warn_test ".env file missing (create from .env.example)"
fi

echo ""

# ============================================================================
# TEST CATEGORY 10: Security Checks
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}[10/10] Security Checks${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

# Test 10.1: No hardcoded private keys
echo -n "Checking for hardcoded secrets... "
if grep -r "PRIVATE_KEY\|private_key\|SECRET_KEY\|secret_key" --include="*.rs" --include="*.js" --include="*.json" . 2>/dev/null | grep -v "PLACEHOLDER\|example\|test" | grep -q "0x\|[a-f0-9]{64}"; then
    fail_test "Possible hardcoded secrets found!"
else
    pass_test "No obvious hardcoded secrets"
fi

# Test 10.2: Sudo key (mainnet check)
if [ "$NETWORK" == "mainnet" ]; then
    echo -n "Checking sudo configuration... "
    if grep -q "\"sudo\":" "$GENESIS_FILE"; then
        warn_test "Sudo pallet enabled (ensure controlled by multisig)"
    else
        info_test "Sudo pallet not configured"
    fi
fi

# Test 10.3: Check for unsafe functions
echo -n "Checking for unsafe code patterns... "
UNSAFE_COUNT=$(grep -r "unsafe {" --include="*.rs" . 2>/dev/null | wc -l)
if [ "$UNSAFE_COUNT" -gt 0 ]; then
    warn_test "Found ${UNSAFE_COUNT} unsafe blocks (review required)"
else
    pass_test "No unsafe blocks found"
fi

# Test 10.4: Cargo audit (if installed)
echo -n "Checking for security vulnerabilities... "
if command -v cargo-audit &> /dev/null; then
    if cargo audit 2>&1 | grep -q "Crate:"; then
        fail_test "Security vulnerabilities found!"
        cargo audit
    else
        pass_test "No known vulnerabilities"
    fi
else
    warn_test "cargo-audit not installed (install with: cargo install cargo-audit)"
fi

echo ""

# ============================================================================
# FINAL REPORT
# ============================================================================
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}Test Summary${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""
echo -e "${GREEN}Passed: ${PASS_COUNT}${NC}"
echo -e "${YELLOW}Warnings: ${WARN_COUNT}${NC}"
echo -e "${RED}Failed: ${FAIL_COUNT}${NC}"
echo ""

TOTAL_TESTS=$((PASS_COUNT + WARN_COUNT + FAIL_COUNT))
PASS_RATE=$((PASS_COUNT * 100 / TOTAL_TESTS))

echo "Pass rate: ${PASS_RATE}%"
echo "Total tests: ${TOTAL_TESTS}"
echo ""

# Deployment readiness
echo -e "${CYAN}========================================${NC}"
echo -e "${CYAN}Deployment Readiness${NC}"
echo -e "${CYAN}========================================${NC}"
echo ""

if [ "$FAIL_COUNT" -eq 0 ] && [ "$WARN_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo -e "${GREEN}System is ready for ${NETWORK} deployment!${NC}"
    EXIT_CODE=0
elif [ "$FAIL_COUNT" -eq 0 ] && [ "$WARN_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}⚠ PASSED WITH WARNINGS${NC}"
    echo -e "${YELLOW}Review warnings before ${NETWORK} deployment${NC}"
    EXIT_CODE=0
else
    echo -e "${RED}✗ TESTS FAILED${NC}"
    echo -e "${RED}Fix ${FAIL_COUNT} failed test(s) before ${NETWORK} deployment${NC}"
    EXIT_CODE=1
fi

echo ""
echo "Test completed: $(date)"
echo "Log saved to: /tmp/etrid-tests.log"
echo ""

# Recommendations
if [ "$FAIL_COUNT" -gt 0 ] || [ "$WARN_COUNT" -gt 0 ]; then
    echo "Recommendations:"
    if [ "$FAIL_COUNT" -gt 0 ]; then
        echo "  1. Fix all failed tests before deployment"
    fi
    if [ "$WARN_COUNT" -gt 0 ]; then
        echo "  2. Review all warnings and assess risks"
    fi
    if [ "$NETWORK" == "mainnet" ]; then
        echo "  3. Perform additional security audit"
        echo "  4. Test on testnet first"
        echo "  5. Have rollback plan ready"
    fi
    echo ""
fi

exit $EXIT_CODE
