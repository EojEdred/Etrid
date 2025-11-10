# PBC (Parachain Bridge Collators) Upgrade Rationale

**Date:** November 9, 2025
**Document Type:** Technical Justification
**Scope:** Network Restart + PBC Deployment Integration

---

## Executive Summary

The Ëtrid FlareChain network restart presents an **optimal opportunity** to simultaneously deploy the **PBC (Parachain Bridge Collators)** upgrade. By combining these two operations, we achieve:

1. **Single Disruption Window:** One coordinated upgrade instead of two separate events
2. **Technical Synergy:** New genesis can include PBC configuration from inception
3. **Validator Efficiency:** All 25 validators update configuration simultaneously
4. **Feature Completeness:** Cross-chain bridge capabilities available immediately at restart
5. **Cost Avoidance:** Eliminates need for second disruptive upgrade later

**Recommendation:** **Deploy PBC upgrade as part of network restart** (November 13-14, 2025)

---

## What is PBC?

### Definition

**PBC = Partition Burst Chain** (Ëtrid's terminology)
**Also Known As:** Parachain (Polkadot terminology)

PBCs are **specialized blockchain instances** that run in parallel to FlareChain (relay chain) and enable:
- Cross-chain asset bridges
- Specialized consensus mechanisms (PPFA)
- Horizontal scalability
- Independent transaction throughput

### Architecture Overview

```
┌────────────────────────────────────────────────────────────┐
│                 FLARECHAIN (Relay Chain)                   │
│                                                            │
│  Directors 1-9: Coordinate PBC checkpoints, finality      │
│  Finality: GRANDPA (9 authorities)                        │
│  Consensus: AURA (all 25 validators)                      │
└───────┬────────────────────────┬───────────────────────────┘
        │                        │
        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐
│   EDSC-PBC       │    │   BTC-PBC        │
│   (Stablecoin)   │    │   (Bitcoin)      │
├──────────────────┤    ├──────────────────┤
│ Validators 10-17 │    │ Validators 18-25 │
│ (8 collators)    │    │ (8 collators)    │
│                  │    │                  │
│ PPFA Consensus   │    │ PPFA Consensus   │
│ 256-block epochs │    │ 256-block epochs │
│                  │    │                  │
│ ËDSC minting     │    │ BTC deposits     │
│ ËDSC redemption  │    │ BTC withdrawals  │
│ Price oracles    │    │ Light client     │
│ Collateral mgmt  │    │ HTLC contracts   │
└──────────────────┘    └──────────────────┘
```

### Technical Specifications

**PBC Consensus: PPFA (Partition Proof of Authority)**

From Ivory Paper Section 9.1 (Lines 765-771):
> "Rotation: 8 validators per PBC, rotate every 256 blocks"

**How PPFA Works:**
```
Epoch = 256 blocks (~51 minutes at 12s/block)

Block Production Schedule:
- Blocks 1-32:   Validator 10 produces
- Blocks 33-64:  Validator 11 produces
- Blocks 65-96:  Validator 12 produces
- Blocks 97-128: Validator 13 produces
- Blocks 129-160: Validator 14 produces
- Blocks 161-192: Validator 15 produces
- Blocks 193-224: Validator 16 produces
- Blocks 225-256: Validator 17 produces

After block 256: Checkpoint submitted to FlareChain
Then: Validator set can rotate (if needed)
```

**Benefits:**
- Deterministic block production (no competition)
- Fair reward distribution (equal blocks per validator)
- Predictable performance (known throughput)
- Low overhead (no proof-of-work, minimal proof-of-stake)

---

## Why Deploy PBC with Network Restart?

### Rationale 1: Minimize Operational Disruption

**Without Combined Deployment:**
```
Timeline:
Nov 13-14: Network restart (finality fix)
  - All validators restart
  - New chainspec deployed
  - 2-4 hour downtime window

Dec 1-2: PBC upgrade (separate event)
  - All validators restart AGAIN
  - PBC collator software deployed
  - 2-4 hour downtime window AGAIN

Total disruption: 2 events, 4-8 hours total downtime
```

**With Combined Deployment:**
```
Timeline:
Nov 13-14: Network restart + PBC upgrade (single event)
  - All validators restart ONCE
  - New chainspec includes PBC config
  - PBC collator software deployed simultaneously
  - 2-4 hour downtime window ONCE

Total disruption: 1 event, 2-4 hours total downtime
```

**Savings:**
- 50% reduction in downtime events
- 50% reduction in operational coordination effort
- Reduced risk of configuration errors (one deployment vs. two)
- Better validator operator experience (one disruption vs. two)

### Rationale 2: Technical Synergy

**New Genesis Configuration Can Include PBC from Start:**

Without PBC at restart:
```json
// Genesis at restart (without PBC)
{
  "genesis": {
    "runtime": {
      "session": { /* 25 validators */ },
      "grandpa": { /* 9 directors */ },
      "aura": { /* 25 validators */ }
    }
  }
}

// Later: Need ANOTHER genesis update for PBC
// Requires runtime upgrade or second restart
```

With PBC at restart:
```json
// Genesis at restart (with PBC)
{
  "genesis": {
    "runtime": {
      "session": { /* 25 validators */ },
      "grandpa": { /* 9 directors */ },
      "aura": { /* 25 validators */ },
      "parachainSystem": {
        "parachains": [
          { "id": 2000, "name": "EDSC-PBC", "validators": [10-17] },
          { "id": 2001, "name": "BTC-PBC", "validators": [18-25] }
        ]
      }
    }
  }
}

// PBC configuration included from block #0
// No second genesis update needed
```

**Benefits:**
- Clean genesis configuration
- PBC infrastructure present from chain start
- No need for complex runtime upgrade later
- Validators configured for PBC from day one

### Rationale 3: Validator Readiness

**Current State (November 9, 2025):**
- 25 validators deployed and configured
- All validators capable of running PBC collators
- Infrastructure in place (Contabo VPS M specs adequate)
- PBC collator binaries already built (ready to deploy)

**If we deploy PBC now:**
- Validators update systemd services ONCE
- PBC collator binaries deployed during same maintenance window
- Session keys for PBC generated during restart process
- No additional coordination needed later

**If we delay PBC:**
- Validators must coordinate SECOND maintenance window
- Risk of reduced participation (some validators may be unavailable)
- Additional testing and validation cycle required
- Potential for configuration drift between validators

### Rationale 4: Feature Completeness

**FlareChain Value Proposition:**
- Multi-chain blockchain with cross-chain bridge capabilities
- ËDSC stablecoin (requires EDSC-PBC for minting/redemption)
- Bitcoin, Ethereum, Solana bridges (require PBCs)
- Competitive with Polkadot, Cosmos ecosystems

**Without PBC:**
- FlareChain is "just another blockchain"
- No cross-chain capabilities (missing core value prop)
- ËDSC stablecoin can't launch (no PBC to support it)
- Delayed competitive advantage

**With PBC at restart:**
- Full feature set available immediately
- ËDSC stablecoin can launch when ready
- Cross-chain bridges operational (when external integration complete)
- Competitive positioning achieved early

**Market Impact:**
- Developers can build cross-chain dApps from day one
- Early adopters see complete feature set
- No "coming soon" disclaimers for core features
- Stronger market positioning vs. competitors

### Rationale 5: Cost Avoidance

**Costs of Second Upgrade (if PBC delayed):**

**Validator Operator Time:**
- 25 validators × 2 hours each = 50 person-hours
- @ $50/hour average = $2,500 labor cost

**Core Development Time:**
- Testing and validation: 20 hours × $100/hour = $2,000
- Deployment coordination: 10 hours × $100/hour = $1,000
- Documentation updates: 5 hours × $100/hour = $500

**Opportunity Cost:**
- Development time diverted from new features
- Network downtime affects early users (if any by December)
- Risk of issues delaying public launch timeline

**Total Cost of Separate PBC Deployment:** ~$6,000 + opportunity cost

**Cost of Combined Deployment:** ~$0 additional (incremental increase in current restart effort)

**Savings:** $6,000 + improved timeline

---

## PBC Deployment Plan

### Initial PBC Chains (November 2025)

**EDSC-PBC (Priority 1):**
- **Purpose:** ËDSC stablecoin minting and redemption
- **Validators:** 10-17 (8 validators)
- **ParaId:** 2000
- **Consensus:** PPFA (256-block epochs)
- **Features:**
  - Mint ËDSC (overcollateralized by ETR)
  - Redeem ËDSC for ETR
  - Oracle price feed integration
  - Collateral management
  - Stability mechanism (similar to MakerDAO)

**BTC-PBC (Priority 2):**
- **Purpose:** Bitcoin cross-chain bridge
- **Validators:** 18-25 (8 validators)
- **ParaId:** 2001
- **Consensus:** PPFA (256-block epochs)
- **Features:**
  - Bitcoin light client
  - BTC deposit detection
  - Wrapped BTC minting (wBTC equivalent)
  - BTC withdrawal via HTLC
  - Multi-signature custody

### Future PBC Chains (2026+)

**ETH-PBC (Q1 2026):**
- Ethereum bridge
- EVM compatibility
- ERC-20 token bridging

**SOL-PBC (Q2 2026):**
- Solana bridge
- SPL token support
- High-throughput transactions

**Additional Chains (as needed):**
- XRP-PBC (Ripple/XRP)
- ADA-PBC (Cardano)
- MATIC-PBC (Polygon)
- ... (11 total PBCs in roadmap)

### Validator Assignment Strategy

**Option A: Two PBCs (Recommended for Restart)**

Assign all 16 Validity Nodes to 2 PBCs:
```
EDSC-PBC: Validators 10-17 (8 validators)
BTC-PBC:  Validators 18-25 (8 validators)
```

**Benefits:**
- Full 8-validator security for each PBC
- Maximum decentralization per chain
- Clear separation of responsibilities
- Easier to monitor and troubleshoot

**Option B: Four PBCs (Maximum Utilization)**

Assign validators to 4 PBCs:
```
EDSC-PBC: Validators 10-13 (4 validators)
BTC-PBC:  Validators 14-17 (4 validators)
ETH-PBC:  Validators 18-21 (4 validators)
SOL-PBC:  Validators 22-25 (4 validators)
```

**Benefits:**
- More bridge chains operational immediately
- Higher total network throughput
- More use cases enabled

**Drawbacks:**
- Lower security per PBC (4 vs. 8 validators)
- More complex management
- Reduced redundancy

**Recommendation for Restart:** **Option A (2 PBCs, 8 validators each)**

Reasons:
1. Network restart is already complex (minimize additional variables)
2. EDSC and BTC are highest priority (focus resources)
3. Can add ETH-PBC and SOL-PBC in Q1 2026 after stabilization
4. Better security posture for initial launch

---

## Technical Requirements

### Software Components

**1. FlareChain Node (Relay Chain)**
- Already deployed on all 25 validators
- **Version:** Latest stable build
- **Functionality:** Coordinates PBC checkpoints, finalizes cross-chain messages

**2. PBC Collator Software**
- **EDSC-PBC Collator:** Built from `etrid/05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/`
- **BTC-PBC Collator:** Built from `etrid/05-multichain/partition-burst-chains/pbc-chains/btc-pbc/`
- **Binary size:** ~100-200 MB each
- **Build time:** ~30-60 minutes per collator
- **Deployment:** Upload to validators, create systemd services

**3. PBC Chainspecs**
- **EDSC-PBC Chainspec:** `chainspec-edsc-pbc-raw.json`
- **BTC-PBC Chainspec:** `chainspec-btc-pbc-raw.json`
- **Contents:** Genesis state, validator set, ParaId, relay chain connection

### Hardware Requirements (Per Validator)

**Current Contabo VPS M Specs:**
- 6 vCPU cores
- 12 GB RAM
- 200 GB NVMe SSD
- 1 Gbps network

**Resource Usage:**

| Component | CPU | RAM | Storage | Network |
|-----------|-----|-----|---------|---------|
| FlareChain Node | 2 cores | 4 GB | 50 GB | 200 MB/day |
| EDSC-PBC Collator | 1 core | 2 GB | 20 GB | 50 MB/day |
| BTC-PBC Collator | 1 core | 2 GB | 20 GB | 50 MB/day |
| **Total (if running both)** | **4 cores** | **8 GB** | **90 GB** | **300 MB/day** |

**Headroom:** 2 vCPU cores, 4 GB RAM, 110 GB storage available

**Conclusion:** Contabo VPS M is **adequate** for running FlareChain + 1 PBC collator per validator.

**Note:** Each validator runs ONLY ONE PBC collator (either EDSC or BTC, not both).

### Network Ports

**FlareChain (Relay Chain):**
- P2P: 30333
- RPC: 9944
- WS: 9945 (optional)
- Prometheus: 9615

**EDSC-PBC:**
- P2P: 30335 (different from relay chain)
- RPC: 9946
- WS: 9947 (optional)
- Prometheus: 9616

**BTC-PBC:**
- P2P: 30336 (different from relay chain)
- RPC: 9948
- WS: 9949 (optional)
- Prometheus: 9617

**Firewall Configuration:**
```bash
# Allow all PBC P2P ports
sudo ufw allow 30333/tcp  # FlareChain
sudo ufw allow 30335/tcp  # EDSC-PBC
sudo ufw allow 30336/tcp  # BTC-PBC

# RPC ports (restrict to monitoring server if desired)
sudo ufw allow from MONITORING_IP to any port 9944  # FlareChain RPC
sudo ufw allow from MONITORING_IP to any port 9946  # EDSC-PBC RPC
sudo ufw allow from MONITORING_IP to any port 9948  # BTC-PBC RPC
```

---

## Deployment Timeline

### Phase 1: Preparation (November 9-11)

**Build PBC Collators:**
```bash
cd ~/Desktop/etrid

# Build EDSC-PBC collator
cargo build --release -p edsc-pbc-collator
# Output: target/release/edsc-pbc-collator

# Build BTC-PBC collator
cargo build --release -p btc-pbc-collator
# Output: target/release/btc-pbc-collator

# Verify binaries
ls -lh target/release/*-pbc-collator
./target/release/edsc-pbc-collator --version
./target/release/btc-pbc-collator --version
```

**Generate PBC Chainspecs:**
```bash
# EDSC-PBC chainspec
./target/release/edsc-pbc-collator build-spec \
    --chain mainnet \
    --raw \
    --disable-default-bootnode \
    > docs/mainnet/chainspec-edsc-pbc-raw.json

# Edit chainspec to configure validators 10-17

# BTC-PBC chainspec
./target/release/btc-pbc-collator build-spec \
    --chain mainnet \
    --raw \
    --disable-default-bootnode \
    > docs/mainnet/chainspec-btc-pbc-raw.json

# Edit chainspec to configure validators 18-25
```

**Test PBC Locally:**
```bash
# Start local FlareChain relay chain
./target/release/flarechain-node --dev --tmp

# Start local EDSC-PBC collator
./target/release/edsc-pbc-collator \
    --collator \
    --chain docs/mainnet/chainspec-edsc-pbc-raw.json \
    --tmp \
    -- \
    --chain dev

# Verify PBC produces blocks and connects to relay chain
```

### Phase 2: Integration with Network Restart (November 12)

**Update Network Restart Genesis:**

Add PBC configuration to FlareChain genesis:
```json
{
  "genesis": {
    "runtime": {
      // ... existing config (session, grandpa, aura)
      "parachainSystem": {
        "parachains": [
          {
            "id": 2000,
            "name": "EDSC-PBC",
            "genesis": "EDSC_PBC_GENESIS_HASH",
            "validators": [
              "VALIDATOR_10_ACCOUNT",
              "VALIDATOR_11_ACCOUNT",
              // ... validators 12-17
            ]
          },
          {
            "id": 2001,
            "name": "BTC-PBC",
            "genesis": "BTC_PBC_GENESIS_HASH",
            "validators": [
              "VALIDATOR_18_ACCOUNT",
              "VALIDATOR_19_ACCOUNT",
              // ... validators 20-25
            ]
          }
        ]
      }
    }
  }
}
```

**Create Validator Deployment Packages:**

For validators 10-17 (EDSC-PBC):
```bash
# Package contents:
- chainspec-mainnet-v2-raw.json (FlareChain)
- chainspec-edsc-pbc-raw.json (EDSC-PBC)
- edsc-pbc-collator (binary)
- systemd service templates
- startup scripts
```

For validators 18-25 (BTC-PBC):
```bash
# Package contents:
- chainspec-mainnet-v2-raw.json (FlareChain)
- chainspec-btc-pbc-raw.json (BTC-PBC)
- btc-pbc-collator (binary)
- systemd service templates
- startup scripts
```

### Phase 3: Deployment (November 13-14, during restart)

**Deployment sequence:**

1. **Stop all validators** (13:45 UTC)
2. **Upload new software:**
   - FlareChain chainspec (all 25 validators)
   - PBC collator binary (validators 10-25)
   - PBC chainspec (validators 10-25)

3. **Update systemd services:**
   - FlareChain validator service (all 25)
   - PBC collator service (validators 10-25 only)

4. **Start validators:**
   - Directors 1-9 start first (14:00 UTC)
   - Verify FlareChain consensus
   - Validators 10-25 start (14:15 UTC)
   - Verify PBC collators connect to relay chain

5. **Verify PBC operation:**
   - EDSC-PBC producing blocks (validators 10-17)
   - BTC-PBC producing blocks (validators 18-25)
   - Checkpoints submitting to FlareChain every 256 blocks

### Phase 4: Monitoring (November 14-17)

**24-Hour PBC Health Check:**
```bash
# Check EDSC-PBC status
curl -H "Content-Type: application/json" -d '{
    "method":"chain_getHeader"
}' http://VALIDATOR_10_IP:9946 | jq .

# Check BTC-PBC status
curl -H "Content-Type: application/json" -d '{
    "method":"chain_getHeader"
}' http://VALIDATOR_18_IP:9948 | jq .

# Verify PBC registered on FlareChain
curl -H "Content-Type: application/json" -d '{
    "method":"state_call",
    "params":["ParachainHost_parachains", "0x"]
}' http://64.181.215.19:9944 | jq .

# Expected: [2000, 2001] (EDSC-PBC and BTC-PBC ParaIds)
```

**Success Criteria:**
- [ ] EDSC-PBC producing blocks (12s block time)
- [ ] BTC-PBC producing blocks (12s block time)
- [ ] Checkpoints submitting to FlareChain every 256 blocks
- [ ] All 8 EDSC-PBC validators participating
- [ ] All 8 BTC-PBC validators participating
- [ ] No PBC collator errors in logs

---

## Risk Assessment

### Risks of Deploying PBC with Restart

**Risk 1: Increased Complexity**
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Extensive testing in Phase 1, rollback plan if issues
- **Status:** Acceptable with proper preparation

**Risk 2: Longer Downtime Window**
- **Likelihood:** Low
- **Impact:** Low (network still private)
- **Mitigation:** Deploy PBC as optional (can disable if issues), stage deployment
- **Status:** Minimal impact

**Risk 3: PBC Collator Software Bugs**
- **Likelihood:** Low (software already tested)
- **Impact:** Medium (could delay restart)
- **Mitigation:** Test thoroughly before restart, have rollback to FlareChain-only
- **Status:** Manageable

**Risk 4: Validator Configuration Errors**
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Clear documentation, deployment automation, pre-deployment validation
- **Status:** Acceptable with good docs

### Risks of Delaying PBC

**Risk 1: Second Disruptive Upgrade Required**
- **Likelihood:** High (100% if PBC not deployed now)
- **Impact:** High (downtime, coordination, testing)
- **Mitigation:** Deploy PBC now (combined with restart)
- **Status:** Avoidable

**Risk 2: Delayed Feature Availability**
- **Likelihood:** High
- **Impact:** Medium (delays ËDSC stablecoin, bridge launches)
- **Mitigation:** Deploy PBC now
- **Status:** Avoidable

**Risk 3: Competitive Disadvantage**
- **Likelihood:** Medium
- **Impact:** Low-Medium (other projects launch cross-chain features first)
- **Mitigation:** Deploy PBC early, enable features ASAP
- **Status:** Avoidable

**Risk 4: Increased Costs**
- **Likelihood:** High
- **Impact:** Medium (~$6,000 additional labor for second deployment)
- **Mitigation:** Deploy PBC now (single deployment)
- **Status:** Avoidable

### Risk Comparison

| Approach | Total Risk Score | Cost | Timeline |
|----------|-----------------|------|----------|
| **Deploy PBC with Restart** | Medium | $0 additional | Nov 13-14 |
| **Deploy PBC Later** | High | +$6,000 | Nov 13-14 + Dec 1-2 |

**Conclusion:** Deploying PBC with network restart has **lower overall risk** than delaying.

---

## Alternative Approaches Considered

### Alternative 1: Deploy PBC in Q1 2026

**Pros:**
- More time to test PBC software
- Simpler network restart (fewer variables)
- Can gather feedback from initial FlareChain operation

**Cons:**
- Requires second disruptive upgrade (+$6,000 cost)
- Delays feature availability (ËDSC, bridges)
- Two downtimes instead of one (worse validator experience)
- Potential for configuration drift between validators

**Decision:** Rejected - costs outweigh benefits

### Alternative 2: Deploy Only EDSC-PBC (Not BTC-PBC)

**Pros:**
- Lower complexity (1 PBC instead of 2)
- Fewer validators need PBC software (8 instead of 16)
- Can test with single PBC before expanding

**Cons:**
- BTC bridge still delayed (second upgrade needed anyway)
- Validators 18-25 remain unutilized
- Doesn't achieve full feature set goal

**Decision:** Rejected - if deploying PBC, deploy both EDSC and BTC

### Alternative 3: Deploy All 14 PBCs Immediately

**Pros:**
- Complete feature set from day one
- All bridges operational immediately
- Maximum network utilization

**Cons:**
- Extremely high complexity (14 PBC collators to deploy)
- Insufficient validator count (need 8 per PBC, only have 16 total)
- Many PBCs don't have external integrations ready (e.g., Solana bridge not coded)
- Very high risk of deployment issues

**Decision:** Rejected - premature, wait for validator expansion

### Alternative 4: Delay Network Restart Until PBC Ready

**Pros:**
- More time to prepare PBC deployment
- Single genesis with PBC from block #0

**Cons:**
- Network remains broken (finality stalled) for weeks/months
- Delays fixing critical issue (GRANDPA configuration)
- Risk of data loss from prolonged finality stall
- Negative impact on validator morale

**Decision:** Rejected - fixing network takes priority

---

## Recommended Approach

### Final Recommendation: Deploy PBC with Network Restart

**Justification:**

1. **Timing is Optimal:**
   - Network restart already scheduled (November 13-14)
   - All validators coordinating for maintenance window
   - Single disruption better than two

2. **Technical Benefits:**
   - New genesis includes PBC configuration
   - Clean deployment (no complex runtime upgrade needed)
   - Validators configured for PBC from day one

3. **Cost Savings:**
   - Avoids ~$6,000 in second deployment costs
   - Reduces validator operational burden
   - Better use of development team time

4. **Feature Enablement:**
   - ËDSC stablecoin can launch when ready
   - Bitcoin bridge operational for early testing
   - Competitive positioning improved

5. **Risk is Manageable:**
   - PBC software already tested
   - Rollback plan in place (can disable PBC, continue with FlareChain only)
   - Network still private (low impact if issues)

### Deployment Scope

**Deploy in November 2025:**
- EDSC-PBC (ParaId 2000) - Validators 10-17
- BTC-PBC (ParaId 2001) - Validators 18-25

**Deploy in Q1 2026 (after stabilization):**
- ETH-PBC (ParaId 2002) - New validators or reassignment
- SOL-PBC (ParaId 2003) - New validators or reassignment

**Deploy Q2 2026 and beyond:**
- Additional PBCs as external integrations complete
- Requires validator expansion (>25 validators)

---

## Success Criteria

### PBC Deployment Considered Successful When:

**Technical Functionality:**
- [ ] EDSC-PBC producing blocks consistently (12s block time)
- [ ] BTC-PBC producing blocks consistently (12s block time)
- [ ] All 16 PBC validators (10-25) participating correctly
- [ ] Checkpoints submitting to FlareChain every 256 blocks
- [ ] PBC finality working (via FlareChain GRANDPA)

**Network Health:**
- [ ] No PBC-related errors in FlareChain logs
- [ ] No PBC collator crashes or errors
- [ ] Cross-chain message queue functioning
- [ ] PBC RPC endpoints responding correctly

**Performance:**
- [ ] PBC block production latency < 13 seconds
- [ ] Checkpoint submission latency < 5 minutes
- [ ] No impact on FlareChain performance
- [ ] Validator resource usage within limits (<80% CPU/RAM)

**Operational:**
- [ ] All 16 validators successfully running PBC collators
- [ ] Monitoring dashboards showing PBC metrics
- [ ] Documentation complete for PBC operations
- [ ] Validator operators trained on PBC troubleshooting

### Rollback Criteria

**Rollback to FlareChain-Only if:**
- PBC collators fail to start on >50% of validators
- PBC blocks not producing after 30 minutes
- Critical errors in PBC software (crashes, data corruption)
- FlareChain performance degraded due to PBC
- Network restart timeline threatened by PBC issues

**Rollback Procedure:**
1. Stop all PBC collators
2. Continue with FlareChain-only restart
3. Diagnose PBC issues offline
4. Schedule PBC deployment for later date

**Note:** Rollback does NOT affect FlareChain network restart success.

---

## Conclusion

Deploying PBC (Parachain Bridge Collators) as part of the November 13-14 network restart is **the optimal strategy** for Ëtrid FlareChain.

By combining these two operations, we:
- **Minimize disruption** (one event instead of two)
- **Reduce costs** (save ~$6,000 in deployment labor)
- **Enable features faster** (ËDSC and BTC bridges available immediately)
- **Improve technical foundation** (clean genesis with PBC configuration)
- **Position competitively** (full feature set from public launch)

The risks of combined deployment are **manageable** and significantly lower than the costs and delays of deploying PBC separately later.

**Recommendation Status:** **APPROVED for inclusion in network restart plan**

---

**Document Version:** 1.0
**Last Updated:** November 9, 2025
**Next Review:** Post-restart evaluation (November 15, 2025)

**Prepared By:** Ëtrid Technical Architecture Team
**Approved By:** Ëtrid Foundation Operations
**Distribution:** Network restart planning team, validator operators

---

## References

**Related Documentation:**
- [Network Restart Announcement](NETWORK_RESTART_ANNOUNCEMENT.md)
- [Network Restart Technical Plan](NETWORK_RESTART_TECHNICAL_PLAN.md)
- [PBC Deployment Guide](PBC_DEPLOYMENT_GUIDE.md)
- [Infrastructure Cost Analysis](INFRASTRUCTURE_COST_ANALYSIS.md)

**Technical Specifications:**
- Ivory Paper Section 9.1: PPFA Consensus
- Ivory Paper Section 6: Cross-Chain Bridges
- Substrate Parachain Documentation
- Polkadot Parachain Architecture

**Code Locations:**
- PBC Runtimes: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains/`
- PBC Collators: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/`
- Bridge Protocols: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/`
