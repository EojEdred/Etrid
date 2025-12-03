# PBC PRODUCTION READINESS AUDIT & DEPLOYMENT PLAN

**Date:** 2025-01-28
**Scope:** All 14 Partition Burst Chains (PBCs)
**Objective:** Audit consensus/finality implementation + Create comprehensive deployment strategy

---

## EXECUTIVE SUMMARY

### Critical Findings

#### ✅ POSITIVE FINDINGS:
1. **Primearc Core (Relay Chain)** - Fully integrated ASF consensus
   - `pallet_consensus` configured and active
   - `ValidatorCommittee` and `ValidatorRewards` pallets present
   - ASF properly integrated into runtime

2. **ETH-PBC** - ASF consensus integrated
   - Has `pallet_consensus` at pallet_index(6)
   - Legacy Aura/Grandpa kept for "compatibility but not used for consensus"
   - ASF implementation exists in `consensus/` directory (979 lines)
   - Comprehensive ASF architecture documented

#### ⚠️ CRITICAL ISSUES IDENTIFIED:
1. **Shared PBC Runtime** - Uses Aura + Grandpa, NOT ASF
   - File: `05-multichain/partition-burst-chains/pbc-runtime/src/lib.rs`
   - Lines 220-221: `Aura: pallet_aura`, `Grandpa: pallet_grandpa`
   - NO ASF consensus integration

2. **13 Other PBCs** - Consensus status UNKNOWN
   - Need to verify each PBC individually
   - Uncertain if they use shared runtime or have individual implementations
   - Only ETH-PBC verified to have ASF

---

## PART 1: PRODUCTION READINESS AUDIT

### Architecture Analysis

#### Primearc Core Chain Structure
```
05-multichain/primearc-core-chain/
├── runtime/src/lib.rs
│   ├── ✅ Consensus: pallet_consensus (line 1257)
│   ├── ✅ ValidatorCommittee: pallet_validator_committee (line 1307)
│   ├── ✅ ValidatorRewards: pallet_validator_rewards (line 1308)
│   ├── ✅ Session: pallet_session (line 1243)
│   └── ✅ EmptySessionHandler for ASF (lines 91-112)
├── node/
└── pallets/
```

**Consensus:** ASF (Ascending Scale of Finality)
**Block Production:** HotStuff 4-phase (Prepare → PreCommit → Commit → Decide)
**Finality:** 5-level scale (0-4) of irreversibility
**Validator Management:** PPFA committee (21 validators)

#### PBC Chain Structure

**ETH-PBC (Verified):**
```
pbc-chains/eth-pbc/
├── eth-pbc-runtime/src/lib.rs
│   ├── ✅ Consensus: pallet_consensus (pallet_index 6)
│   ├── ⚠️  Aura: pallet_aura (pallet_index 4) - Legacy
│   ├── ⚠️  Grandpa: pallet_grandpa (pallet_index 5) - Legacy
│   └── Comment: "Legacy pallets kept for compatibility but not used for consensus"
├── consensus/
│   ├── asf-algorithm/
│   ├── block-production/
│   ├── finality-gadget/
│   ├── pallet/ (979 lines)
│   ├── validator-management/
│   └── ARCHITECTURE.md
└── eth-pbc-collator/
```

**Other 13 PBCs (Need Verification):**
- BTC-PBC
- SOL-PBC
- TRX-PBC
- BNB-PBC
- XRP-PBC
- ADA-PBC
- XLM-PBC
- DOGE-PBC
- LINK-PBC
- MATIC-PBC
- SC-USDT-PBC
- EDSC-PBC
- AI-Compute-PBC

### Audit Tasks Required

#### Phase 1: Consensus Verification (URGENT)
1. ✅ **Primearc Core** - Verified ASF integrated
2. ✅ **ETH-PBC** - Verified ASF integrated
3. ❌ **BTC-PBC** - NOT VERIFIED
4. ❌ **SOL-PBC** - NOT VERIFIED
5. ❌ **TRX-PBC** - NOT VERIFIED
6. ❌ **BNB-PBC** - NOT VERIFIED
7. ❌ **XRP-PBC** - NOT VERIFIED
8. ❌ **ADA-PBC** - NOT VERIFIED
9. ❌ **XLM-PBC** - NOT VERIFIED
10. ❌ **DOGE-PBC** - NOT VERIFIED
11. ❌ **LINK-PBC** - NOT VERIFIED
12. ❌ **MATIC-PBC** - NOT VERIFIED
13. ❌ **SC-USDT-PBC** - NOT VERIFIED
14. ❌ **EDSC-PBC** - NOT VERIFIED
15. ❌ **AI-Compute-PBC** - NOT VERIFIED

#### Phase 2: For Each PBC, Verify:
- [ ] ASF consensus pallet integrated in runtime
- [ ] Block production mechanism (ASF vs Aura)
- [ ] Finality gadget (ASF vs Grandpa)
- [ ] Validator committee management
- [ ] Session key management
- [ ] Reward distribution
- [ ] Network/P2P configuration
- [ ] RPC endpoints configured
- [ ] Telemetry integration

#### Phase 3: Integration Verification
- [ ] All PBCs connect to Primearc Core relay chain
- [ ] Cross-chain messaging works (via pallet_pbc_router)
- [ ] Bridge pallets functional
- [ ] State synchronization verified
- [ ] Fork handling tested
- [ ] Validator rotation tested

---

## PART 2: DEPLOYMENT STRATEGY

### Infrastructure Overview

**Total VMs:** 22 validators
- **Contabo:** VMs 1-20
- **Oracle Cloud:** VMs 21-22 (Gizzi, Auditdev)

**Network Architecture:**
- Public IPs: External access
- Tailscale IPs: Private mesh network
- Each VM runs multiple collators (one per PBC)

### Deployment Phases

#### Phase 0: PRE-DEPLOYMENT CHECKS
```bash
# On each VM, verify:
1. Rust toolchain installed
2. System dependencies
3. Storage capacity (>100GB per PBC)
4. Memory (>4GB per PBC)
5. Network connectivity
6. Tailscale mesh configured
```

#### Phase 1: BINARY DEPLOYMENT

**Option A: Build on Each VM (Slow but Safe)**
```bash
# Per VM, per PBC:
cd ~/etrid/05-multichain/partition-burst-chains/pbc-chains/eth-pbc
cargo build --release -p eth-pbc-collator
# Repeat for all 14 PBCs
```

**Option B: Build Once, Distribute (Fast, Recommended)**
```bash
# On build server (Linux x86_64):
./scripts/build-all-pbcs.sh

# Creates:
# - eth-pbc-collator
# - btc-pbc-collator
# - sol-pbc-collator
# ... (14 total)

# Distribute to all VMs:
for vm in vm-{01..20} gizzi auditdev; do
    rsync -avz target/release/*-pbc-collator $vm:/usr/local/bin/
done
```

**Proposed Build Script:** `scripts/build-all-pbcs.sh`
```bash
#!/bin/bash
set -e

PBCS=(
    "eth" "btc" "sol" "trx" "bnb" "xrp"
    "ada" "xlm" "doge" "link" "matic"
    "sc-usdt" "edsc" "ai-compute"
)

for pbc in "${PBCS[@]}"; do
    echo "=== Building ${pbc}-pbc-collator ==="
    cd "pbc-chains/${pbc}-pbc"
    cargo build --release -p "${pbc}-pbc-collator"
    cd ../..
done
```

#### Phase 2: CHAINSPEC GENERATION

Each PBC needs a chainspec with its 22 validators:

```bash
# For each PBC:
./eth-pbc-collator build-spec \
    --chain dev \
    --disable-default-bootnode \
    > eth-pbc-chainspec-raw.json

# Customize chainspec:
# - Add genesis authorities (from VALIDATOR_KEYS_MAPPING.md)
# - Set network ID
# - Configure ports
# - Set bootnode addresses

# Convert to raw:
./eth-pbc-collator build-spec \
    --chain eth-pbc-chainspec.json \
    --raw \
    > eth-pbc-chainspec-raw.json
```

**Chainspec Structure (per PBC):**
```json
{
  "name": "ETH-PBC Mainnet",
  "id": "eth_pbc_mainnet",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/38.242.229.77/tcp/30533/p2p/<PEER_ID_1>",
    "/ip4/109.123.234.22/tcp/30533/p2p/<PEER_ID_2>"
  ],
  "telemetryEndpoints": null,
  "protocolId": "eth_pbc",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 18
  },
  "relay_chain": "primearc_core",
  "para_id": 2001,
  "genesis": {
    "runtime": {...},
    "asf": {
      "authorities": [
        "0x<VALIDATOR_1_PUBLIC_KEY>",
        "0x<VALIDATOR_2_PUBLIC_KEY>",
        ...
      ]
    }
  }
}
```

#### Phase 3: KEY INSERTION

For each VM, for each PBC, insert validator keys:

```bash
# On VM-01 (Validator 1):
# ETH-PBC
./eth-pbc-collator key insert \
    --chain eth-pbc-chainspec-raw.json \
    --base-path /var/lib/eth-pbc \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//ETH/Validator1"

# BTC-PBC
./btc-pbc-collator key insert \
    --chain btc-pbc-chainspec-raw.json \
    --base-path /var/lib/btc-pbc \
    --key-type asfk \
    --scheme Sr25519 \
    --suri "//BTC/Validator1"

# Repeat for all 14 PBCs
```

**Automated Key Insertion Script:** `scripts/insert-all-keys.sh`
```bash
#!/bin/bash
# Usage: ./insert-all-keys.sh <validator_number>

VALIDATOR_NUM=$1
PBCS=(eth btc sol trx bnb xrp ada xlm doge link matic sc-usdt edsc ai-compute)

for pbc in "${PBCS[@]}"; do
    echo "=== Inserting ${pbc}-pbc Validator${VALIDATOR_NUM} key ==="

    "./${pbc}-pbc-collator" key insert \
        --chain "${pbc}-pbc-chainspec-raw.json" \
        --base-path "/var/lib/${pbc}-pbc" \
        --key-type asfk \
        --scheme Sr25519 \
        --suri "//${pbc^^}/Validator${VALIDATOR_NUM}"
done
```

#### Phase 4: PEER ID COLLECTION

Collect peer IDs for bootnode configuration:

```bash
# On each VM, for each PBC:
./eth-pbc-collator key inspect-node-key \
    --file /var/lib/eth-pbc/chains/eth_pbc_mainnet/network/secret_ed25519

# Output: 12D3KooW... (peer ID)
```

**Automated Collection:** `scripts/collect-peer-ids.sh`
```bash
#!/bin/bash

PBCS=(eth btc sol trx bnb xrp ada xlm doge link matic sc-usdt edsc ai-compute)
VMS=(vm-{01..20} gizzi auditdev)

for pbc in "${PBCS[@]}"; do
    echo "=== ${pbc}-pbc Peer IDs ==="
    for vm in "${VMS[@]}"; do
        peer_id=$(ssh $vm "./${pbc}-pbc-collator key inspect-node-key \
            --file /var/lib/${pbc}-pbc/chains/${pbc}_pbc_mainnet/network/secret_ed25519" 2>/dev/null)
        echo "$vm: $peer_id"
    done
done
```

#### Phase 5: SERVICE SETUP

Create systemd services for each PBC:

**Template:** `/etc/systemd/system/eth-pbc.service`
```ini
[Unit]
Description=ETH-PBC Collator
After=network.target

[Service]
Type=simple
User=etrid
Group=etrid
WorkingDirectory=/home/etrid
ExecStart=/usr/local/bin/eth-pbc-collator \
    --collator \
    --chain /home/etrid/chainspecs/eth-pbc-chainspec-raw.json \
    --base-path /var/lib/eth-pbc \
    --port 30533 \
    --rpc-port 9946 \
    --ws-port 9947 \
    --prometheus-port 9618 \
    --bootnodes /ip4/38.242.229.77/tcp/30533/p2p/<PEER_ID_1> \
    --bootnodes /ip4/109.123.234.22/tcp/30533/p2p/<PEER_ID_2> \
    --name "primearc-validator-01-eth-pbc" \
    --validator
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

**Port Assignments (per PBC, per VM):**
| PBC | Network Port | RPC Port | WS Port | Prometheus |
|-----|--------------|----------|---------|------------|
| ETH | 30533 | 9946 | 9947 | 9618 |
| BTC | 30633 | 9948 | 9949 | 9619 |
| SOL | 30733 | 9950 | 9951 | 9620 |
| TRX | 30833 | 9952 | 9953 | 9621 |
| BNB | 30933 | 9954 | 9955 | 9622 |
| XRP | 31033 | 9956 | 9957 | 9623 |
| ADA | 31133 | 9958 | 9959 | 9624 |
| XLM | 31233 | 9960 | 9961 | 9625 |
| DOGE | 31333 | 9962 | 9963 | 9626 |
| LINK | 31433 | 9964 | 9965 | 9627 |
| MATIC | 31533 | 9966 | 9967 | 9628 |
| USDT | 31633 | 9968 | 9969 | 9629 |
| EDSC | 31733 | 9970 | 9971 | 9630 |
| AI | 31833 | 9972 | 9973 | 9631 |

#### Phase 6: LAUNCH SEQUENCE

**Coordinated Launch:**
```bash
# 1. Start Primearc Core on all VMs first
systemctl start primearc-core
sleep 60  # Wait for relay chain to sync

# 2. Start all PBCs simultaneously
for pbc in eth btc sol trx bnb xrp ada xlm doge link matic sc-usdt edsc ai-compute; do
    systemctl start ${pbc}-pbc
done

# 3. Monitor startup
journalctl -u eth-pbc -f
```

#### Phase 7: HEALTH CHECKS

**Per-PBC Health Check Script:** `scripts/check-pbc-health.sh`
```bash
#!/bin/bash
# Usage: ./check-pbc-health.sh <pbc-name> <rpc-port>

PBC=$1
RPC_PORT=$2

echo "=== $PBC Health Check ==="

# 1. Check if process running
if pgrep -f "${PBC}-pbc-collator" > /dev/null; then
    echo "✅ Process running"
else
    echo "❌ Process NOT running"
    exit 1
fi

# 2. Check RPC endpoint
if curl -s http://localhost:${RPC_PORT} > /dev/null; then
    echo "✅ RPC endpoint accessible"
else
    echo "❌ RPC endpoint NOT accessible"
    exit 1
fi

# 3. Get block height
HEIGHT=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
    http://localhost:${RPC_PORT} | jq -r '.result.block.header.number')

if [ -n "$HEIGHT" ]; then
    echo "✅ Current block: $HEIGHT"
else
    echo "❌ Cannot get block height"
    exit 1
fi

# 4. Check peer count
PEERS=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
    http://localhost:${RPC_PORT} | jq -r '.result | length')

if [ "$PEERS" -gt 0 ]; then
    echo "✅ Connected peers: $PEERS"
else
    echo "⚠️  No peers connected"
fi

# 5. Check sync status
SYNCING=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
    http://localhost:${RPC_PORT} | jq -r '.result.currentBlock')

echo "✅ Sync status: Block $SYNCING"

# 6. Check validator status
IS_VALIDATOR=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_hasSessionKeys"}' \
    http://localhost:${RPC_PORT} | jq -r '.result')

if [ "$IS_VALIDATOR" = "true" ]; then
    echo "✅ Validator keys loaded"
else
    echo "❌ Validator keys NOT loaded"
fi

echo "=== Health check complete ==="
```

**Full Network Health Check:**
```bash
#!/bin/bash
# Check all PBCs across all VMs

PBCS=(eth btc sol trx bnb xrp ada xlm doge link matic sc-usdt edsc ai-compute)
PORTS=(9946 9948 9950 9952 9954 9956 9958 9960 9962 9964 9966 9968 9970 9972)
VMS=(vm-{01..20} gizzi auditdev)

for i in "${!PBCS[@]}"; do
    pbc="${PBCS[$i]}"
    port="${PORTS[$i]}"

    echo "========== $pbc-PBC =========="

    for vm in "${VMS[@]}"; do
        echo "--- $vm ---"
        ssh $vm "./check-pbc-health.sh $pbc $port"
    done
done
```

---

## PART 3: DEPLOYMENT ORCHESTRATION

### Master Deployment Script

**File:** `scripts/deploy-all-pbcs.sh`
```bash
#!/bin/bash
set -e

# Configuration
PBCS=(eth btc sol trx bnb xrp ada xlm doge link matic sc-usdt edsc ai-compute)
VMS=(vm-{01..20} gizzi auditdev)
ETRID_USER="etrid"

# Functions
log() { echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*"; }
error() { echo "[ERROR] $*" >&2; exit 1; }

# Phase 1: Build binaries
log "Phase 1: Building all PBC binaries..."
./scripts/build-all-pbcs.sh || error "Build failed"

# Phase 2: Generate chainspecs
log "Phase 2: Generating chainspecs..."
./scripts/generate-all-chainspecs.sh || error "Chainspec generation failed"

# Phase 3: Distribute binaries
log "Phase 3: Distributing binaries to all VMs..."
for vm in "${VMS[@]}"; do
    log "Deploying to $vm..."
    rsync -avz target/release/*-pbc-collator ${ETRID_USER}@${vm}:/usr/local/bin/
    rsync -avz chainspecs/*.json ${ETRID_USER}@${vm}:/home/${ETRID_USER}/chainspecs/
done

# Phase 4: Insert keys
log "Phase 4: Inserting validator keys..."
for i in "${!VMS[@]}"; do
    vm="${VMS[$i]}"
    validator_num=$((i + 1))
    log "Inserting keys for Validator $validator_num on $vm..."
    ssh ${ETRID_USER}@${vm} "./insert-all-keys.sh $validator_num"
done

# Phase 5: Collect peer IDs
log "Phase 5: Collecting peer IDs for bootnodes..."
./scripts/collect-peer-ids.sh > bootnode-peer-ids.txt

# Phase 6: Create systemd services
log "Phase 6: Creating systemd services..."
for vm in "${VMS[@]}"; do
    log "Setting up services on $vm..."
    ssh ${ETRID_USER}@${vm} "./setup-systemd-services.sh"
done

# Phase 7: Start services
log "Phase 7: Starting all PBC services..."
for vm in "${VMS[@]}"; do
    log "Starting services on $vm..."
    for pbc in "${PBCS[@]}"; do
        ssh ${ETRID_USER}@${vm} "sudo systemctl enable ${pbc}-pbc && sudo systemctl start ${pbc}-pbc"
    done
done

# Phase 8: Health checks
log "Phase 8: Running health checks..."
sleep 30  # Wait for services to start
./scripts/check-all-pbcs.sh

log "Deployment complete!"
```

---

## PART 4: PRODUCTION CHECKLIST

### Pre-Deployment
- [ ] All 14 PBCs audited for consensus implementation
- [ ] ASF consensus verified in all PBC runtimes
- [ ] Binaries built and tested on Linux x86_64
- [ ] Chainspecs generated for all PBCs
- [ ] Validator keys generated for all 22 validators
- [ ] Infrastructure verified (VMs, storage, network)
- [ ] Monitoring/telemetry configured

### Deployment
- [ ] Binaries distributed to all 22 VMs
- [ ] Chainspecs distributed to all VMs
- [ ] Validator keys inserted on all VMs
- [ ] Peer IDs collected for bootnodes
- [ ] Systemd services created and enabled
- [ ] All services started successfully

### Post-Deployment
- [ ] All PBCs syncing blocks
- [ ] Validator participation confirmed
- [ ] Cross-chain messaging verified
- [ ] No fork detection
- [ ] Telemetry data flowing
- [ ] Log aggregation working
- [ ] Alert system active

### Ongoing Monitoring
- [ ] Block production rate normal
- [ ] Finality progressing (5-level scale)
- [ ] Validator rewards distributing
- [ ] No equivocation detected
- [ ] Network health score stable
- [ ] Resource usage within limits

---

## PART 5: RISK MITIGATION

### Identified Risks

1. **Consensus Mismatch**
   - Risk: Some PBCs may not have ASF integrated
   - Mitigation: Complete audit before deployment
   - Rollback: Use Aura/Grandpa temporarily if ASF not ready

2. **Key Management**
   - Risk: Key loss or compromise
   - Mitigation: Secure backups, hardware security
   - Recovery: Key regeneration from derivation paths

3. **Network Partitioning**
   - Risk: VMs lose connectivity
   - Mitigation: Tailscale mesh + public IPs
   - Recovery: Automatic reconnection, bootnode fallback

4. **Synchronization Issues**
   - Risk: PBCs out of sync with relay chain
   - Mitigation: Monitor sync status, alert on lag
   - Recovery: Warp sync, snapshot restoration

5. **Resource Exhaustion**
   - Risk: VMs run out of disk/memory
   - Mitigation: Monitor resources, set limits
   - Recovery: Scale up VMs, prune old blocks

---

## NEXT STEPS

### Immediate Actions Required:

1. **URGENT: Complete Consensus Audit**
   - Verify all 13 remaining PBCs have ASF integrated
   - If not, integrate ASF into each PBC runtime
   - Test ASF consensus on devnet first

2. **Create Missing Scripts**
   - `scripts/build-all-pbcs.sh`
   - `scripts/generate-all-chainspecs.sh`
   - `scripts/insert-all-keys.sh`
   - `scripts/collect-peer-ids.sh`
   - `scripts/check-pbc-health.sh`
   - `scripts/deploy-all-pbcs.sh`

3. **Generate Chainspecs**
   - One for each PBC with 22 validators
   - Include bootnode addresses
   - Set proper para_ids

4. **Test Deployment on 2 VMs First**
   - Deploy to vm-01 and vm-02 only
   - Verify all 14 PBCs run correctly
   - Test inter-PBC communication
   - Monitor for 24 hours

5. **Full Deployment**
   - Roll out to remaining 20 VMs
   - Coordinate launch sequence
   - Monitor continuously

---

## QUESTIONS FOR USER

Before proceeding with implementation, I need clarification on:

1. **Consensus Priority:**
   - Do ALL PBCs MUST use ASF, or can some use Aura/Grandpa temporarily?
   - If ASF not ready for all PBCs, should we delay or deploy with mixed consensus?

2. **Build Strategy:**
   - Build centrally and distribute, OR build on each VM?
   - Do we have a dedicated build server?

3. **Deployment Timing:**
   - Deploy all 14 PBCs simultaneously, or phased rollout?
   - Recommended: Phase 1 = ETH/BTC/SOL, Phase 2 = rest

4. **Monitoring:**
   - Do we have monitoring infrastructure (Prometheus/Grafana)?
   - Should I include monitoring setup in deployment plan?

5. **Chainspec Customization:**
   - Are there specific para_ids assigned for each PBC?
   - Any custom runtime parameters needed per PBC?

---

## ESTIMATED TIMELINE

### If ASF Already Integrated in All PBCs:
- Scripts creation: 4-6 hours
- Chainspec generation: 2-3 hours
- Binary building: 3-5 hours (parallel)
- Test deployment (2 VMs): 4-6 hours
- Full deployment (22 VMs): 6-8 hours
- Health verification: 2-3 hours
- **Total: 21-31 hours (3-4 days)**

### If ASF Needs Integration:
- ASF integration per PBC: 4-8 hours × 13 = 52-104 hours
- Testing: 16-24 hours
- Plus above timeline
- **Total: 89-159 hours (11-20 days)**

---

**END OF PLAN**
