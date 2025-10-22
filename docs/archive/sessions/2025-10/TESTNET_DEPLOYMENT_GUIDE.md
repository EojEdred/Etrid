# FlareChain Testnet Deployment Guide

**Date:** October 21, 2025
**Version:** 1.0.0
**Status:** Production-Ready Testnet

---

## 🎯 Overview

This guide walks through deploying a multi-node FlareChain testnet with:
- ✅ ASF consensus with PPFA block sealing
- ✅ Runtime API integration
- ✅ Committee loading from runtime
- ✅ Epoch transitions and rotation
- ✅ Validator key management

---

## 📋 Prerequisites

### System Requirements

**Minimum:**
- CPU: 4 cores
- RAM: 8 GB
- Disk: 50 GB SSD
- OS: Linux/macOS

**Recommended:**
- CPU: 8 cores
- RAM: 16 GB
- Disk: 100 GB NVMe SSD
- OS: Ubuntu 22.04 LTS

### Software Requirements

```bash
# Rust toolchain
rustc --version  # Should be 1.70+
cargo --version

# Build tools
sudo apt-get install -y \
    build-essential \
    git \
    clang \
    libssl-dev \
    pkg-config
```

---

## 🔨 Building the Node

### Step 1: Build FlareChain Binary

```bash
cd /Users/macbook/Desktop/etrid

# Build in release mode (optimized)
cargo build -p flare-chain --release

# Binary location
ls -lh target/release/flare-chain

# Expected output:
# -rwxr-xr-x  1 user  staff   45M Oct 21 14:00 flare-chain
```

**Build Time:** ~15-30 minutes (depending on hardware)

### Step 2: Verify Build

```bash
# Check version
./target/release/flare-chain --version

# Expected output:
# flare-chain 0.1.0-dev

# Check available commands
./target/release/flare-chain --help
```

---

## 🔐 Generating Validator Keys

### For Each Validator Node

FlareChain uses **SR25519** keys for ASF consensus validators.

**Key Type:** `asfk` (ASF Consensus Key)

### Option A: Generate New Keys

```bash
# Generate a new keypair
./target/release/flare-chain key generate \
    --scheme Sr25519 \
    --output-type Json

# Example output:
{
  "secretPhrase": "bottom drive obey lake curtain smoke basket hold race lonely fit walk",
  "secretSeed": "0x...",
  "publicKey": "0x...",
  "accountId": "0x...",
  "ss58Address": "5..."
}

# IMPORTANT: Save the secretPhrase and secretSeed securely!
```

### Option B: Use Existing Keys

If you have existing validator keys:

```bash
# Insert key into keystore
./target/release/flare-chain key insert \
    --base-path /tmp/node1 \
    --chain local \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "your secret phrase here"

# Expected output:
# Key inserted successfully
```

### Generate Keys for 3-Node Testnet

```bash
# Node 1 (Alice)
./target/release/flare-chain key insert \
    --base-path /tmp/node1 \
    --chain local \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//Alice"

# Node 2 (Bob)
./target/release/flare-chain key insert \
    --base-path /tmp/node2 \
    --chain local \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//Bob"

# Node 3 (Charlie)
./target/release/flare-chain key insert \
    --base-path /tmp/node3 \
    --chain local \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//Charlie"
```

### Verify Keys

```bash
# Check keystore for node1
ls /tmp/node1/chains/local_testnet/keystore/

# Expected: File named 6173666b... (asfk in hex)
```

---

## 🚀 Starting the Testnet

### Single-Node Development Setup

```bash
./target/release/flare-chain \
    --dev \
    --tmp \
    --validator \
    --name DevNode \
    --rpc-port 9933 \
    --ws-port 9944

# This starts a single-node development chain
```

### 3-Node Testnet Setup

#### Terminal 1: Node 1 (Alice - Bootnode)

```bash
./target/release/flare-chain \
    --base-path /tmp/node1 \
    --chain local \
    --validator \
    --name Alice \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --port 30333 \
    --rpc-port 9933 \
    --ws-port 9944 \
    --rpc-cors all \
    --rpc-methods Unsafe \
    --unsafe-rpc-external \
    --unsafe-ws-external
```

**Important:** Copy the bootnode address from the logs:
```
Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

#### Terminal 2: Node 2 (Bob)

```bash
./target/release/flare-chain \
    --base-path /tmp/node2 \
    --chain local \
    --validator \
    --name Bob \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
    --port 30334 \
    --rpc-port 9934 \
    --ws-port 9945
```

#### Terminal 3: Node 3 (Charlie)

```bash
./target/release/flare-chain \
    --base-path /tmp/node3 \
    --chain local \
    --validator \
    --name Charlie \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
    --port 30335 \
    --rpc-port 9935 \
    --ws-port 9946
```

---

## 📊 Monitoring PPFA Block Sealing

### Expected Log Messages

#### On Block Production (Every ~6 seconds)

```
2025-10-21 14:30:15 📦 We are proposer for slot #150 (PPFA index: 5)
2025-10-21 14:30:15    Creating block on parent: #149 (0x1234...)
2025-10-21 14:30:16 🔨 Authored block #150 (0xabcd...) with 0 extrinsics
2025-10-21 14:30:16 🔏 Added PPFA seal to block #150: index=5, proposer=0x12345678...
2025-10-21 14:30:16 ✅ Block #150 imported successfully: ImportedKnown
```

#### On Block Import

```
2025-10-21 14:30:16 🔍 Extracted PPFA seal: index=5, proposer=0x1234...
2025-10-21 14:30:16 🔐 Validating PPFA authorization for block #150: proposer=0x1234..., ppfa_index=5
2025-10-21 14:30:16 ✅ PPFA authorization validated for block #150 (proposer in committee)
2025-10-21 14:30:16 ✅ ASF block #150 validated successfully
```

#### On Committee Loading (Startup)

```
2025-10-21 14:25:00 ✅ Loaded 3 committee members from runtime at block 0x0000...
2025-10-21 14:25:00 ✅ Added our validator to committee: 12345678
2025-10-21 14:25:00 🔗 PPFA committee initialized (size: 3/21, mode: production)
2025-10-21 14:25:00 ✅ PPFA proposer initialized
2025-10-21 14:25:00    - Committee size: 3
2025-10-21 14:25:00    - Slot duration: 6000ms
```

#### On Epoch Transition (Every 2400 blocks)

```
2025-10-21 16:00:00 🔄 Epoch transition detected at slot #2400 (slot epoch: #1)
2025-10-21 16:00:00 ✅ Loaded 3 new committee members for epoch #1
2025-10-21 16:00:00 🔄 Committee rotated successfully (size: 3, epoch: 1)
```

### Monitoring Commands

```bash
# Watch logs in real-time
tail -f /tmp/node1/chains/local_testnet/network/polkadot.log

# Filter for PPFA-related logs
tail -f /tmp/node1/chains/local_testnet/network/polkadot.log | grep -E "PPFA|seal|committee"

# Count blocks produced
tail -f /tmp/node1/chains/local_testnet/network/polkadot.log | grep -c "Authored block"
```

---

## 🔍 Validation Checklist

### ✅ Node Startup

- [ ] Node starts without errors
- [ ] Keystore loaded successfully
- [ ] Committee loaded from runtime
- [ ] Validator added to committee
- [ ] PPFA proposer initialized

### ✅ Block Production

- [ ] Blocks produced every ~6 seconds
- [ ] PPFA seal added to blocks
- [ ] Correct proposer rotation (PPFA index increments)
- [ ] All extrinsics included

### ✅ Block Import

- [ ] PPFA seal extracted successfully
- [ ] Proposer authorization validated
- [ ] Blocks imported without errors
- [ ] Chain finalizing (GRANDPA)

### ✅ Committee Management

- [ ] Committee size correct (3 validators)
- [ ] Committee members match genesis
- [ ] Epoch transitions at block 2400
- [ ] New committee loaded on rotation

### ✅ Network

- [ ] Nodes discover each other
- [ ] Peer count: 2 (for 3-node setup)
- [ ] Blocks syncing across nodes
- [ ] No network errors

---

## 🐛 Troubleshooting

### Issue: "No validator keys in keystore"

**Symptom:**
```
⚠️  No ASF validator key found in keystore (key_type: asfk)
```

**Solution:**
```bash
# Insert validator key
./target/release/flare-chain key insert \
    --base-path /tmp/node1 \
    --chain local \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//Alice"
```

### Issue: "Committee is empty"

**Symptom:**
```
⚠️  No validator keys in keystore. Committee will be empty.
```

**Solution:**
1. Ensure genesis config includes validators
2. Check runtime configuration
3. Verify keystore has correct keys

### Issue: "Failed to load committee from runtime"

**Symptom:**
```
❌ Failed to load committee from runtime: RuntimeApiError
```

**Solution:**
1. Verify runtime includes `pallet-validator-committee`
2. Check Runtime API implementation
3. Ensure genesis validators are configured

### Issue: Nodes not connecting

**Symptom:**
```
Peer count: 0
```

**Solution:**
1. Check bootnode address is correct
2. Verify ports are not blocked
3. Ensure all nodes use same `--chain` spec
4. Check firewall settings

### Issue: PPFA seal not appearing

**Symptom:**
```
No "Added PPFA seal" logs
```

**Solution:**
1. Verify you're a validator (check keystore)
2. Check you're the current proposer
3. Ensure PPFA implementation is correct
4. Review block production logs

---

## 📈 Performance Monitoring

### Key Metrics

```bash
# Block production rate
# Expected: 1 block every ~6 seconds (10 blocks/minute)

# Block finalization
# Expected: Blocks finalized within 2-3 blocks

# Peer count
# Expected: (N-1) peers for N-node network

# Memory usage
# Expected: ~500 MB - 2 GB per node

# CPU usage
# Expected: 5-20% per core (idle), 50-100% (under load)
```

### Health Check Script

```bash
#!/bin/bash
# health_check.sh

echo "FlareChain Testnet Health Check"
echo "================================"

# Check if node is running
if pgrep -f "flare-chain" > /dev/null; then
    echo "✅ Node is running"
else
    echo "❌ Node is not running"
    exit 1
fi

# Check RPC endpoint
BLOCK_NUMBER=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
    http://localhost:9933 | jq -r '.result.number')

if [ -n "$BLOCK_NUMBER" ]; then
    echo "✅ RPC responding (block: $BLOCK_NUMBER)"
else
    echo "❌ RPC not responding"
    exit 1
fi

# Check peer count
PEER_COUNT=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://localhost:9933 | jq -r '.result.peers')

echo "✅ Peer count: $PEER_COUNT"

echo "================================"
echo "Health check complete!"
```

---

## 🔧 Advanced Configuration

### Custom Chain Spec

```bash
# Generate chain spec
./target/release/flare-chain build-spec \
    --chain local \
    --disable-default-bootnode \
    > chain-spec.json

# Convert to raw format
./target/release/flare-chain build-spec \
    --chain chain-spec.json \
    --raw \
    --disable-default-bootnode \
    > chain-spec-raw.json

# Use custom spec
./target/release/flare-chain \
    --chain chain-spec-raw.json \
    --validator \
    --name CustomNode
```

### Genesis Configuration

Edit `chain-spec.json` to customize:

```json
{
  "validatorCommittee": {
    "validators": [
      ["0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", 1000, 0],
      ["0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48", 1000, 0],
      ["0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22", 1000, 0]
    ]
  }
}
```

---

## 📊 24-Hour Stability Test

### Test Objectives

1. ✅ Block production continuity
2. ✅ No memory leaks
3. ✅ No consensus failures
4. ✅ Proper epoch transitions
5. ✅ PPFA sealing operational

### Monitoring Script

```bash
#!/bin/bash
# stability_test.sh

DURATION=86400  # 24 hours in seconds
START_TIME=$(date +%s)

while true; do
    CURRENT_TIME=$(date +%s)
    ELAPSED=$((CURRENT_TIME - START_TIME))

    if [ $ELAPSED -gt $DURATION ]; then
        echo "✅ 24-hour test complete!"
        break
    fi

    # Health check every 5 minutes
    ./health_check.sh

    # Log metrics
    echo "$(date): Uptime: $((ELAPSED/3600))h $((ELAPSED%3600/60))m"

    sleep 300  # 5 minutes
done
```

### Success Criteria

- ✅ 100% uptime for 24 hours
- ✅ ~14,400 blocks produced (1 per 6 seconds)
- ✅ 10 epoch transitions (every 2400 blocks)
- ✅ Memory usage stable (no leaks)
- ✅ No consensus errors
- ✅ All PPFA seals valid

---

## 🎯 Next Steps After Testnet

### Once Testnet is Stable

1. **Performance Optimization**
   - Profile block production
   - Optimize database queries
   - Tune network parameters

2. **EDSC Bridge Security**
   - Implement oracle permissions (2-3 days)
   - Integrate reserve vault (3-4 days)
   - Add custodian signatures (4-5 days)

3. **Additional Testing**
   - Load testing (high transaction volume)
   - Stress testing (network disruptions)
   - Fuzz testing (invalid inputs)

4. **Documentation**
   - Update operator guides
   - Create troubleshooting wiki
   - Write incident response plan

5. **External Security Audit**
   - Submit audit package
   - Schedule audit firm
   - Prepare for findings

---

## 📧 Support & Resources

### Documentation

- **Ivory Paper:** `/docs/ivory-paper.md`
- **Architecture:** `/docs/architecture/`
- **API Reference:** `/docs/api/`

### Community

- **GitHub:** https://github.com/etrid-protocol/etrid
- **Discord:** [Coming soon]
- **Forum:** [Coming soon]

### Contact

- **Technical Lead:** [Contact info]
- **Security:** security@etrid.io
- **General:** hello@etrid.io

---

## ✅ Deployment Checklist

### Pre-Deployment

- [ ] Build FlareChain binary
- [ ] Generate validator keys (3+)
- [ ] Configure genesis validators
- [ ] Prepare monitoring scripts
- [ ] Review system requirements

### Deployment

- [ ] Start bootnode (Node 1)
- [ ] Note bootnode address
- [ ] Start additional nodes (2, 3, ...)
- [ ] Verify peer connectivity
- [ ] Check committee loading

### Post-Deployment

- [ ] Monitor PPFA block sealing
- [ ] Verify epoch transitions
- [ ] Check block finalization
- [ ] Run 24-hour stability test
- [ ] Document any issues

### Production Readiness

- [ ] Testnet stable for 24+ hours
- [ ] All tests passing (88/88)
- [ ] EDSC bridge security implemented
- [ ] External security audit complete
- [ ] Incident response plan ready

---

**Prepared by:** Claude Code
**Date:** October 21, 2025
**Version:** 1.0.0
**Status:** ✅ Production-Ready Testnet Guide

---

*FlareChain testnet deployment guide - Ready for multi-node validator testing* 🚀
