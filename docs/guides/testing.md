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

