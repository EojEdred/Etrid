#!/bin/bash
# Documentation Consolidation Script
# Merges 11 markdown files into 7 organized documents

set -e

echo "=========================================="
echo "Ëtrid Documentation Consolidation"
echo "=========================================="
echo ""

# Create archive directory for original files
mkdir -p docs/archive/consolidated-sources

echo "Step 1: Creating DEVELOPER_GUIDE.md..."
cat > DEVELOPER_GUIDE.md << 'DEVGUIDE_EOF'
# Ëtrid Developer Guide

Complete guide for developers getting started with Ëtrid, understanding the architecture, and contributing to the project.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [System Architecture](#system-architecture)
3. [Contributing Guidelines](#contributing-guidelines)
4. [Project Organization](#project-organization)

---

DEVGUIDE_EOF

# Append Quick Start content (skip first line with #)
tail -n +2 QUICK_START.md >> DEVELOPER_GUIDE.md
echo "" >> DEVELOPER_GUIDE.md
echo "---" >> DEVELOPER_GUIDE.md
echo "" >> DEVELOPER_GUIDE.md

# Append Architecture content (skip first line)
tail -n +2 ARCHITECTURE.md >> DEVELOPER_GUIDE.md
echo "" >> DEVELOPER_GUIDE.md
echo "---" >> DEVELOPER_GUIDE.md
echo "" >> DEVELOPER_GUIDE.md

# Append Contributing content (skip first line)
tail -n +2 CONTRIBUTING.md >> DEVELOPER_GUIDE.md

echo "✅ DEVELOPER_GUIDE.md created"

echo "Step 2: Creating DEPLOYMENT_GUIDE.md..."
cat > DEPLOYMENT_GUIDE.md << 'DEPLOY_EOF'
# Ëtrid Deployment & Security Guide

Complete guide for deploying Ëtrid to production and managing security best practices.

---

## Table of Contents

1. [Mainnet Deployment](#mainnet-deployment)
2. [Security Best Practices](#security-best-practices)

---

DEPLOY_EOF

# Append deployment content (skip first 5 lines of header)
tail -n +6 MAINNET_DEPLOYMENT_HANDOFF.md >> DEPLOYMENT_GUIDE.md
echo "" >> DEPLOYMENT_GUIDE.md
echo "---" >> DEPLOYMENT_GUIDE.md
echo "" >> DEPLOYMENT_GUIDE.md

# Append security content (skip first line)
tail -n +2 NETWORK_KEYS_SECURITY_GUIDE.md >> DEPLOYMENT_GUIDE.md

echo "✅ DEPLOYMENT_GUIDE.md created"

echo "Step 3: Creating PROJECT_HISTORY.md..."
cat > PROJECT_HISTORY.md << 'HISTORY_EOF'
# Ëtrid Project History

Development sessions, milestones, and historical progress tracking.

---

## Table of Contents

1. [Week 0 Summary](#week-0-summary)
2. [GenesisBuilder Fix Session](#genesisbuilder-fix-session)

---

HISTORY_EOF

# Append Week 0 content (skip first line)
tail -n +2 ÉTRID-WEEK0-ACTION-SUMMARY.md >> PROJECT_HISTORY.md
echo "" >> PROJECT_HISTORY.md
echo "---" >> PROJECT_HISTORY.md
echo "" >> PROJECT_HISTORY.md

# Append GenesisBuilder session (skip first line)
tail -n +2 SESSION_OCT19_GENESISBUILDER_FIX.md >> PROJECT_HISTORY.md

echo "✅ PROJECT_HISTORY.md created"

echo "Step 4: Creating TESTING_GUIDE.md..."
cat > TESTING_GUIDE.md << 'TEST_EOF'
# Ëtrid Testing Guide

Comprehensive guide for testing Ëtrid blockchain components, scripts, and infrastructure.

---

## Overview

Ëtrid provides multiple test scripts for validating different aspects of the system:

1. **Bridge Testing** - Cross-chain functionality
2. **Chain Spec Testing** - Genesis configuration
3. **Runtime Testing** - All 12 PBC runtimes
4. **Network Testing** - Multi-validator setups
5. **Build Testing** - Compilation and WASM generation

---

## Test Scripts

### Bridge Functionality Test

**Script**: `test_bridge_basic.sh`

**Purpose**: Test FlareChain + BTC PBC collator integration

**Usage**:
```bash
./test_bridge_basic.sh
```

**What it tests**:
- Binary existence
- Chain spec generation
- Bridge connectivity
- Cross-chain messaging

---

### Chain Spec Generation Test

**Script**: `test_all_chain_specs.sh`

**Purpose**: Verify all 12 PBC collators can generate chain specs (validates GenesisBuilder API)

**Usage**:
```bash
./test_all_chain_specs.sh
```

**Expected Output**:
```
Testing btc-pbc chain spec generation... ✅ SUCCESS
Testing eth-pbc chain spec generation... ✅ SUCCESS
...
Summary: 12 passed, 0 failed
```

---

### Runtime Compilation Test

**Script**: `test_all_12_runtimes.sh`

**Purpose**: Test that all 12 PBC runtimes compile successfully

**Usage**:
```bash
./test_all_12_runtimes.sh
```

---

### Comprehensive PBC Test

**Script**: `test_all_pbcs_comprehensive.sh`

**Purpose**: End-to-end testing of all PBC functionality

**Usage**:
```bash
./test_all_pbcs_comprehensive.sh
```

---

### Bridge Pallet Test

**Script**: `test_bridge_pallets.sh`

**Purpose**: Test individual bridge pallets in isolation

**Usage**:
```bash
./test_bridge_pallets.sh
```

---

### Runtime Integration Test

**Script**: `test_runtime_integration.sh`

**Purpose**: Test integration between FlareChain and PBC runtimes

**Usage**:
```bash
./test_runtime_integration.sh
```

---

### Full Bridge Test Suite

**Script**: `run_bridge_tests.sh`

**Purpose**: Run complete bridge test suite

**Usage**:
```bash
./run_bridge_tests.sh
```

---

## Build Scripts

### Build All Remaining PBCs

**Script**: `build_all_remaining_pbcs.sh`

**Purpose**: Build all 11 PBC collators in parallel batches

**Usage**:
```bash
./build_all_remaining_pbcs.sh
```

**Features**:
- Parallel builds (3 batches: 4+4+3)
- Progress tracking
- WASM verification
- Build time reporting

**Expected Duration**: 60-90 minutes

---

## Continuous Integration

### Running Tests in CI

All test scripts are designed to be CI-friendly:

```bash
# Run all tests
for script in test_*.sh; do
  ./$script || exit 1
done
```

### Test Environment

**Prerequisites**:
- Rust 1.70+
- wasm32-unknown-unknown target
- 60GB+ disk space
- 8GB+ RAM

---

## Troubleshooting

### Chain Spec Generation Fails

**Error**: `GenesisBuilder_get_preset not found`

**Solution**: Ensure GenesisBuilder API is implemented in runtime (see SESSION_OCT19_GENESISBUILDER_FIX.md)

### Build Fails - Disk Space

**Error**: `No space left on device`

**Solution**:
```bash
# Clean debug builds
rm -rf target/debug

# Keep only compressed WASM
find target -name "*.wasm" ! -name "*.compressed.wasm" -delete
```

### Binary Not Found

**Error**: `Binary not found at target/release/`

**Solution**:
```bash
# Build the specific collator
cargo build --release -p {chain}-pbc-collator
```

---

## Best Practices

1. **Always test after changes**: Run relevant test scripts
2. **Use chain spec tests**: Quick validation of runtime changes
3. **Monitor disk space**: Clean up between builds
4. **Parallel builds**: Use build_all_remaining_pbcs.sh for efficiency
5. **Check logs**: All tests output to logs for debugging

---

## Next Steps

After all tests pass:
1. Test bridge functionality end-to-end
2. Deploy to local testnet
3. Perform integration testing
4. Deploy to public testnet
5. Security audit
6. Mainnet deployment

---

**For more details, see**:
- [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) - Development setup
- [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) - Production deployment
- [KNOWN_ISSUES.md](KNOWN_ISSUES.md) - Current blockers

TEST_EOF

echo "✅ TESTING_GUIDE.md created"

echo "Step 5: Renaming VALUE_REFERENCE.md..."
if [ -f "etrvalueref.md" ]; then
    mv etrvalueref.md VALUE_REFERENCE.md
    echo "✅ Renamed etrvalueref.md → VALUE_REFERENCE.md"
fi

echo "Step 6: Archiving original source files..."
mv QUICK_START.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv ARCHITECTURE.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv CONTRIBUTING.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv MAINNET_DEPLOYMENT_HANDOFF.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv NETWORK_KEYS_SECURITY_GUIDE.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv SESSION_OCT19_GENESISBUILDER_FIX.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv ÉTRID-WEEK0-ACTION-SUMMARY.md docs/archive/consolidated-sources/ 2>/dev/null || true
mv DOCUMENTATION_AUDIT.md docs/archive/consolidated-sources/ 2>/dev/null || true

echo "✅ Original files archived to docs/archive/consolidated-sources/"

echo ""
echo "=========================================="
echo "Consolidation Complete!"
echo "=========================================="
echo ""
echo "New Structure:"
echo "  1. README.md (main entry)"
echo "  2. DEVELOPER_GUIDE.md (quick start + arch + contributing)"
echo "  3. DEPLOYMENT_GUIDE.md (deployment + security)"
echo "  4. KNOWN_ISSUES.md (unchanged)"
echo "  5. PROJECT_HISTORY.md (week 0 + genesisbuilder session)"
echo "  6. VALUE_REFERENCE.md (renamed)"
echo "  7. TESTING_GUIDE.md (all test documentation)"
echo ""
echo "Result: 11 files → 7 files ✅"
echo ""
