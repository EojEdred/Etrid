# Validator v109 Deployment Guide - 21-Member Committee

## Current Status

**Validators**: 22 validators running, all crash-looping
**Issue**: Binary compiled from old source without ValidatorCommittee pallet
**Error**: `Error parsing spec file: no variant of enum RuntimeGenesisConfigJson found in flattened data`

## Root Cause Analysis

1. ✅ **Runtime source code** includes ValidatorCommittee pallet
   - Location: `05-multichain/flare-chain/runtime/src/lib.rs:1242`
   - Properly integrated in `construct_runtime!` macro

2. ✅ **Chainspec** correctly formatted with 21-member committee
   - File: `/tmp/manual_chainspec_v109.json`
   - Uses snake_case fields: `validator_committee`, `committee_size`
   - Deployed to 20/22 validators

3. ❌ **Binary** compiled from old source (before ValidatorCommittee was added)
   - Current binary: 80MB, modified Nov 19 18:26
   - Doesn't recognize `validator_committee` field in genesis config

## Solution: Build and Deploy Fresh Binary

### Step 1: Build Binary on Linux Machine with Rust Toolchain

```bash
# On a Linux machine with cargo installed
cd /root/etrid
git pull origin main

# Clean and rebuild
cd 05-multichain/flare-chain
cargo clean -p flare-chain-runtime -p flarechain-node
cargo build --release -p flarechain-node

# Binary will be at: target/release/flarechain-node
```

### Step 2: Deploy Binary to All Validators

```bash
# From your local machine
cd /Users/macbook/Desktop/etrid

# Run the deployment script (it pulls from Validator-6)
./DEPLOY_NEW_BINARY_FROM_VM.sh
```

### Step 3: Verify 21-Member Committee Initialization

```bash
# Check validator logs for committee initialization
ssh -i ~/.ssh/contabo-validators root@100.95.0.72 \
  'journalctl -u flarechain-validator -n 100 --no-pager | \
   grep -E "committee|Committee|ValidatorCommittee|Epoch 0" | tail -20'
```

## Alternative: GitHub Actions Build

If you don't have access to a Linux build machine, use GitHub Actions:

1. Push current code to GitHub
2. Create `.github/workflows/build-binary.yml`:

```yaml
name: Build FlareChain Node

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build Binary
        run: |
          cd 05-multichain/flare-chain
          cargo build --release -p flarechain-node
      - name: Upload Binary
        uses: actions/upload-artifact@v3
        with:
          name: flarechain-node
          path: target/release/flarechain-node
```

3. Download artifact and deploy to validators

## Files Created

- `DEPLOY_PRODUCTION_CHAINSPEC_V109.sh` - Deploy production chainspec (camelCase)
- `DEPLOY_FIXED_CHAINSPEC_V109.sh` - Deploy fixed chainspec (snake_case)
- `DEPLOY_NEW_BINARY_FROM_VM.sh` - Deploy binary from Validator-6
- `PURGE_V109_DB_ONLY.sh` - Purge v109 database
- `/tmp/manual_chainspec_v109.json` - Working v109 chainspec with 21 validators

## Validator Configuration

**Chain ID**: `flarechain_prod_21val_v109`
**Committee Size**: 21 validators
**Network Keys**: Preserved at `chains/flarechain_prod_21val_v109/network/secret_ed25519`

## Next Steps

1. Build fresh binary from current source code
2. Deploy to all validators using `DEPLOY_NEW_BINARY_FROM_VM.sh`
3. Validators will automatically restart with flarechain-validator systemd service
4. Verify 21-member committee initialized successfully
5. Confirm consensus and block production

---

**Note**: The chainspec and deployment scripts are ready. Only the binary needs to be rebuilt from the current source that includes the ValidatorCommittee pallet.
