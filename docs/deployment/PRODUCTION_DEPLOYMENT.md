# Ëtrid Production Deployment Guide

## Overview

This guide covers deploying FlareChain and ETH-PBC to production (testnet → mainnet).

## Prerequisites

- Version alignment complete (stable2506)
- All components building successfully
- Zombienet local testing passed
- HRMP channels tested locally

## Deployment Phases

### Phase 1: Testnet Deployment (Rococo/Westend)

**Timeline**: 2-3 weeks
**Goal**: Test full XCM integration in live environment

#### Step 1: Prepare Binaries

```bash
# Build FlareChain node
cargo build --release -p flarechain-node

# Build ETH-PBC node
cd 05-multichain/partition-burst-chains/pbc-chains/eth-pbc
cargo build --release

# Verify binaries
./target/release/flarechain-node --version
./target/release/eth-pbc-node --version
```

#### Step 2: Generate Chain Specs

```bash
# FlareChain testnet spec
./target/release/flarechain-node build-spec \
    --chain flarechain-testnet \
    --raw > flarechain-testnet-raw.json

# ETH-PBC testnet spec
./target/release/eth-pbc-node build-spec \
    --chain eth-pbc-testnet \
    --raw > eth-pbc-testnet-raw.json
```

#### Step 3: Register Parachains on Rococo

**Requirements:**
- 1000+ ROC tokens for parachain deposit
- Sudo access or governance approval

**Process:**

1. **Reserve Para IDs**
```bash
# Via polkadot.js apps (https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io)
# Developer > Extrinsics > registrar > reserve()
# FlareChain: Request ID 2000
# ETH-PBC: Request ID 2001
```

2. **Generate Genesis State & Wasm**
```bash
# FlareChain
./target/release/flarechain-node export-genesis-state \
    --chain flarechain-testnet-raw.json > flarechain-genesis-state

./target/release/flarechain-node export-genesis-wasm \
    --chain flarechain-testnet-raw.json > flarechain-genesis-wasm

# ETH-PBC
./target/release/eth-pbc-node export-genesis-state \
    --chain eth-pbc-testnet-raw.json > eth-pbc-genesis-state

./target/release/eth-pbc-node export-genesis-wasm \
    --chain eth-pbc-testnet-raw.json > eth-pbc-genesis-wasm
```

3. **Register Parachains**
```bash
# Via Rococo sudo or governance
# registrar.forceRegister(paraId, genesisHead, validationCode)
```

#### Step 4: Start Collators

**FlareChain Collator:**
```bash
./target/release/flarechain-node \
    --collator \
    --name "FlareChain-Collator-01" \
    --chain flarechain-testnet-raw.json \
    --base-path /data/flarechain \
    --port 30333 \
    --rpc-port 9933 \
    --ws-port 9944 \
    --unsafe-rpc-external \
    --unsafe-ws-external \
    --rpc-cors all \
    -- \
    --execution wasm \
    --chain rococo \
    --port 30343 \
    --rpc-port 9934
```

**ETH-PBC Collator:**
```bash
./target/release/eth-pbc-node \
    --collator \
    --name "ETH-PBC-Collator-01" \
    --chain eth-pbc-testnet-raw.json \
    --base-path /data/eth-pbc \
    --port 30334 \
    --rpc-port 9937 \
    --ws-port 9948 \
    --unsafe-rpc-external \
    --unsafe-ws-external \
    --rpc-cors all \
    -- \
    --execution wasm \
    --chain rococo \
    --port 30344 \
    --rpc-port 9938
```

#### Step 5: Setup HRMP Channels

**Wait for parachains to be fully onboarded (producing blocks), then:**

```bash
# Run HRMP setup script
./scripts/setup-hrmp-channels.sh \
    --relay-chain rococo \
    --flarechain-id 2000 \
    --eth-pbc-id 2001
```

**Manual Steps (if script fails):**

1. **Open channel ETH-PBC → FlareChain**
```javascript
// From ETH-PBC
api.tx.hrmp.hrmpInitOpenChannel(
    2000, // FlareChain
    1000, // max capacity
    10240 // max message size
).signAndSend(collatorAccount)

// From FlareChain (accept)
api.tx.hrmp.hrmpAcceptOpenChannel(
    2001 // ETH-PBC
).signAndSend(collatorAccount)
```

2. **Open channel FlareChain → ETH-PBC**
```javascript
// From FlareChain
api.tx.hrmp.hrmpInitOpenChannel(
    2001, // ETH-PBC
    1000, // max capacity
    10240 // max message size
).signAndSend(collatorAccount)

// From ETH-PBC (accept)
api.tx.hrmp.hrmpAcceptOpenChannel(
    2000 // FlareChain
).signAndSend(collatorAccount)
```

3. **Verify Channels**
```bash
# Check via polkadot.js
# Developer > Chain State > hrmp > hrmpChannels(2000, 2001)
# Developer > Chain State > hrmp > hrmpChannels(2001, 2000)
```

#### Step 6: Enable Production XCM Bridge

**Update ETH-PBC Runtime:**

```rust
// In runtime/src/lib.rs
// Change from MockXcmBridge to ProductionXcmBridge
impl pallet_evm::Config for Runtime {
    // ...
    type PrecompilesValue = EtridPrecompiles<Self, ProductionXcmBridge>;
}
```

**Upgrade Runtime:**
```bash
# Build new runtime
cargo build --release -p eth-pbc-runtime

# Submit runtime upgrade via governance or sudo
# System > setCode(new_runtime_wasm)
```

#### Step 7: Deploy & Test Contracts

```bash
# Update hardhat config for testnet
# hardhat.config.js
networks: {
  ethPbcTestnet: {
    url: "https://eth-pbc-testnet-rpc.etrid.io",
    chainId: 2001,
    accounts: [DEPLOYER_PRIVATE_KEY]
  }
}

# Deploy example contracts
npx hardhat run scripts/deploy-all.js --network ethPbcTestnet

# Test precompiles
npx hardhat test --network ethPbcTestnet
```

#### Step 8: Monitor XCM Messages

**Tools:**
- Subscan: https://rococo.subscan.io
- Polkadot.js: https://polkadot.js.org/apps
- Custom dashboard (see MONITORING_GUIDE.md)

**Key Metrics:**
- XCM message delivery rate
- Average message latency
- Failed messages
- Response cache hit rate

---

### Phase 2: Kusama Deployment

**Timeline**: 1-2 months after successful Rococo testing

**Requirements:**
- 20+ KSM for parachain slot auction/deposit
- Governance approval
- Community support

**Process**: Similar to Rococo but with:
- Longer auction periods
- More validators
- Higher stakes
- Production-grade monitoring

---

### Phase 3: Polkadot Mainnet

**Timeline**: 3-6 months after Kusama

**Requirements:**
- 100+ DOT for parachain slot
- Proven track record on Kusama
- Security audit
- Community governance

---

## Production Checklist

### Pre-Deployment

- [ ] Version alignment complete (all components on stable2506)
- [ ] All unit tests passing
- [ ] Integration tests passing
- [ ] Zombienet tests passing
- [ ] Security audit complete
- [ ] Documentation complete
- [ ] Monitoring setup complete

### FlareChain

- [ ] Chain spec generated
- [ ] Genesis state exported
- [ ] Collator binaries built
- [ ] Infrastructure provisioned
- [ ] Monitoring configured
- [ ] Backup strategy implemented

### ETH-PBC

- [ ] Chain spec generated
- [ ] Genesis state exported
- [ ] Collator binaries built
- [ ] EVM RPC endpoints configured
- [ ] Frontier database configured
- [ ] Monitoring configured

### XCM Integration

- [ ] HRMP channels configured
- [ ] Production XCM bridge enabled
- [ ] Message queue configured
- [ ] Response caching enabled
- [ ] Failover strategy implemented
- [ ] Monitoring dashboards ready

### Smart Contracts

- [ ] Example contracts deployed
- [ ] Contract verification on block explorer
- [ ] Integration tests run on testnet
- [ ] Gas optimization complete
- [ ] Emergency pause mechanisms tested

---

## Infrastructure Requirements

### FlareChain Collator

**Minimum:**
- CPU: 4 cores
- RAM: 16 GB
- Disk: 500 GB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 8 cores
- RAM: 32 GB
- Disk: 1 TB NVMe SSD
- Network: 1 Gbps

### ETH-PBC Collator

**Minimum:**
- CPU: 8 cores (EVM + Frontier overhead)
- RAM: 32 GB
- Disk: 1 TB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 16 cores
- RAM: 64 GB
- Disk: 2 TB NVMe SSD
- Network: 1 Gbps

### Relay Chain Node (if running own)

**Recommended:**
- CPU: 4 cores
- RAM: 16 GB
- Disk: 500 GB NVMe SSD
- Network: 1 Gbps

---

## Monitoring & Alerting

### Key Metrics

**Chain Health:**
- Block production rate
- Finalization lag
- Peer count
- Sync status

**XCM:**
- Messages sent/received
- Message delivery time
- Failed messages
- Queue size

**EVM (ETH-PBC):**
- Transaction throughput
- Gas usage
- Precompile calls
- Block gas limit utilization

### Alert Thresholds

- Block production stopped > 30s
- Finalization lag > 10 blocks
- XCM message failure rate > 5%
- Disk usage > 80%
- Memory usage > 90%

---

## Upgrade Strategy

### Runtime Upgrades

```bash
# Build new runtime
cargo build --release -p flarechain-runtime

# Test locally
./target/release/flarechain-node --dev --tmp

# Deploy via governance
# 1. Propose upgrade
# 2. Vote period (7 days typical)
# 3. Execute upgrade
# 4. Monitor for issues
```

### Node Upgrades

```bash
# Build new node
cargo build --release -p flarechain-node

# Rolling update:
# 1. Stop old node
# 2. Backup database
# 3. Start new node
# 4. Verify sync
# 5. Repeat for other nodes
```

---

## Disaster Recovery

### Backup Strategy

**Chain State:**
```bash
# Daily snapshots
./scripts/backup-chain-state.sh --chain flarechain --output /backups/

# Retention: 30 days
```

**Configuration:**
```bash
# Backup chain specs, keys, configs
tar -czf config-backup-$(date +%Y%m%d).tar.gz \
    flarechain-testnet-raw.json \
    eth-pbc-testnet-raw.json \
    /etc/systemd/system/flarechain.service \
    /etc/systemd/system/eth-pbc.service
```

### Recovery Procedures

1. **Collator Failure**
   - Start backup collator
   - Sync from network
   - Update session keys

2. **Database Corruption**
   - Restore from backup
   - Or resync from genesis

3. **HRMP Channel Issues**
   - Check channel status
   - Reopen if closed
   - Clear stuck messages

---

## Security Considerations

### Access Control

- Use hardware security modules for collator keys
- Implement multi-sig for governance
- Restrict RPC access (VPN/firewall)
- Regular security audits

### Network Security

- DDoS protection (Cloudflare, etc.)
- Rate limiting on RPC endpoints
- Geo-redundancy
- Regular penetration testing

---

## Support & Resources

**Documentation:**
- Technical docs: `/docs/technical/`
- API reference: `/docs/API_REFERENCE.md`
- Troubleshooting: `/docs/TROUBLESHOOTING.md`

**Community:**
- Discord: (TBD)
- Telegram: (TBD)
- Forum: (TBD)

**Emergency Contacts:**
- Security: security@etrid.io
- Infrastructure: ops@etrid.io
- Support: support@etrid.io

---

## Timeline Summary

| Phase | Duration | Milestone |
|---|---|---|
| Testnet (Rococo) | 2-3 weeks | XCM live testing |
| Kusama | 1-2 months | Canary network |
| Polkadot Mainnet | 3-6 months | Production launch |

**Total to Mainnet**: 6-9 months from testnet start

---

## Next Steps

1. ✅ Complete version alignment (Option 1)
2. ⏳ Build and test unified workspace
3. ⏳ Deploy to Rococo testnet
4. ⏳ Setup HRMP channels
5. ⏳ Enable production XCM
6. ⏳ Deploy and test contracts
7. ⏳ Monitor and optimize
8. ⏳ Prepare for Kusama

**Ready to proceed with version alignment implementation!**
