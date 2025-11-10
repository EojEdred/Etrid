# Ëtrid FlareChain Mainnet Controlled Network Reset

**Date:** November 9, 2025
**Type:** Technical Announcement
**Status:** Planned Network Restart
**Impact:** Private Validator Network Only

---

## Executive Summary

Ëtrid FlareChain will undergo a **controlled network reset** to resolve a critical finality issue discovered during infrastructure migration operations on November 7-9, 2025. This reset will preserve all finalized blockchain state through block #63,274 and establish a corrected validator configuration for future network operations.

**Key Points:**
- Network is currently **private** with 25 internal validators only
- No public users or exchange listings affected
- All finalized state through block #63,274 will be **preserved**
- Fresh start with corrected GRANDPA authority configuration
- Opportunity to deploy PBC (Parachain Bridge Collators) upgrade simultaneously
- Significant cost savings from completed Azure to Contabo migration

---

## What Happened

### Timeline of Events

**November 7-8, 2025:**
- Azure subscription payment issue caused 16 validators to go offline
- Network fell below consensus threshold, unable to finalize new blocks
- Last finalized block: **#63,274**
- Block production continued to #76,401 but blocks remained unfinalized

**November 8-9, 2025:**
- Emergency migration from Azure to Contabo initiated
- 16 new validators provisioned and deployed
- During migration, validators temporarily operated on different chain forks
- GRANDPA finality stalled due to equivocation (conflicting votes from different chain states)

**November 9, 2025:**
- Root cause identified: GRANDPA authority configuration mismatch
- Genesis configured for 21 validators but runtime authority set inconsistent
- Chain fork during migration created irreconcilable GRANDPA votes
- Decision made to perform controlled network reset

### Technical Root Cause

The finality issue stems from multiple concurrent problems:

1. **GRANDPA Equivocation:** Validators voting for conflicting blocks during migration created Byzantine fault condition
2. **Authority Set Mismatch:** Genesis configuration specified 21 validators, but GRANDPA authority storage showed 0 authorities
3. **Chain Fork History:** Migration process temporarily created different chain forks, causing conflicting GRANDPA prevotes
4. **Configuration Error:** Original genesis incorrectly included all 25 validators (9 Directors + 16 Validity Nodes) as GRANDPA authorities instead of only the 9 Directors

---

## Why Network Reset is Necessary

### Current State Analysis

**Finalized State:**
- Last finalized block: **#63,274**
- Finalized state is consistent and verified
- All critical data through this block is preserved

**Unfinalized State:**
- Blocks #63,275 through #76,401 are produced but **not finalized**
- These blocks contain conflicting GRANDPA votes from migration period
- Cannot achieve 2/3+1 finality threshold due to equivocation
- GRANDPA rounds stuck and unable to progress

### Why Recovery Without Reset is Not Feasible

**Option Considered: Wait for GRANDPA Recovery**
- Likelihood: Very low
- Equivocation typically requires manual intervention
- No guarantee of resolution even with extended time

**Option Considered: Force Authority Set Update**
- Requires finality to execute governance extrinsics
- Creates circular dependency (need finality to fix finality)

**Option Considered: Deploy Original Validators**
- Would restore to broken state with incorrect GRANDPA configuration
- Does not fix root cause

**Chosen Solution: Controlled Network Reset**
- Clean slate with corrected GRANDPA authority configuration
- Preserves all finalized state (block #63,274)
- Eliminates conflicting GRANDPA vote history
- Opportunity to implement pending upgrades simultaneously

---

## What is Preserved

### State Export from Block #63,274

All finalized blockchain state will be **fully preserved**, including:

**Accounts & Balances:**
- All account balances as of block #63,274
- Treasury holdings
- Foundation reserves
- Validator staking balances

**Consensus Configuration:**
- Validator session keys
- Staking state
- Era/epoch information

**Runtime State:**
- All pallet storage (Council, Treasury, Identity, etc.)
- Governance proposals active at block #63,274
- Smart contract state (if any deployed)

**Cryptographic Integrity:**
- State root: Verified and exported
- Genesis hash: Will reference block #63,274 state
- All data cryptographically proven

---

## What is Lost

### Unfinalized Blocks #63,275 - #76,401

**Block Data:**
- 13,127 unfinalized blocks will not be included in new chain
- Block production occurred but finality never achieved
- No consensus on canonical state for these blocks

**Transaction Impact Analysis:**

During November 7-9 migration period, network had:
- **No public users** (private validator network)
- **No exchange listings** (pre-launch phase)
- **No dApp deployments** (development phase)
- **No external transactions** (validators only)

**Internal Transactions (Minimal):**
- Validator session key rotations (if any) - can be resubmitted
- Internal testing transactions - no production impact
- System events (block production, era changes) - will resume from block #63,274 state

**Impact Assessment:**
- Zero impact to external users (none exist yet)
- Minimal impact to internal operations
- All critical validator infrastructure maintained

---

## New Genesis Configuration

### Corrected GRANDPA Authority Setup

**Previous Configuration (Incorrect):**
```
GRANDPA Authorities: 25 validators
- 9 Directors (correct)
- 16 Validity Nodes (incorrect - should not be GRANDPA authorities)
Result: Authority mismatch, storage showed 0 authorities
```

**New Configuration (Correct):**
```
GRANDPA Authorities: 9 Directors ONLY
- Director 1 (Gizzi) - Oracle Cloud
- Director 2 (AuditDev) - Oracle Cloud
- Directors 3-8 - Oracle Cloud
- Director 9 - Oracle Cloud

Validity Nodes: 16 validators (NOT in GRANDPA authority set)
- Validator 10-21 (Contabo validators)
- Participate in block production (AURA)
- Participate in ASF committee
- Do NOT participate in GRANDPA finality
```

### Updated Validator Topology (25 Total)

**Directors (9) - GRANDPA Finality Authority:**
- 2 on Oracle Cloud (Gizzi, AuditDev) - Already deployed
- 4 additional Oracle Cloud directors - Planned
- 3 additional directors - Future expansion

**Validity Nodes (16) - Block Production:**
- 16 on Contabo (vmi2896xxx, vmi3xxx, vmi4xxx regions)
- 4 in St. Louis datacenter (vmi4xxx region)
- Geographic distribution: Germany (9), USA (7)

**Total Active Validators:** 25
**GRANDPA Finality Threshold:** 7 of 9 directors (78%)
**Block Production:** All 25 via AURA rotation

---

## Timing and Process

### Preparation Phase (November 9-12)

**Day 1-2: State Export & Genesis Creation**
1. Export complete state from block #63,274
2. Verify state root and cryptographic integrity
3. Create new genesis.json with corrected GRANDPA authorities
4. Update with current 25-validator configuration
5. Build new raw chainspec

**Day 3: Testing & Validation**
1. Test new chainspec with 2-3 validators
2. Verify GRANDPA authority set correct (9 directors only)
3. Confirm block production and finality working
4. Validate state import successful

### Deployment Phase (November 13-14)

**Day 4: Distribution**
1. Distribute new chainspec to all 25 validators
2. Provide updated startup commands
3. Coordinate restart time window

**Day 5: Network Restart**
1. **Restart Sequence:**
   - Directors 1-9 start first (GRANDPA authorities)
   - Wait for initial finality (2-3 blocks finalized)
   - Validity Nodes 10-25 join network
   - Full consensus within 30 minutes

2. **Verification:**
   - Block production active
   - Finality advancing (GRANDPA working)
   - All 25 validators online
   - State matches block #63,274 export

### Post-Restart Monitoring (November 15-17)

**24-Hour Monitoring:**
- Continuous health checks all validators
- GRANDPA finality progression monitoring
- Block production metrics
- Peer connectivity status

**48-Hour Stability:**
- Era/epoch transitions
- Reward distribution verification
- No finality stalls
- Network performance benchmarks

---

## PBC (Parachain Bridge Collators) Upgrade Integration

### Why Deploy PBC with Network Reset

The network reset provides an optimal opportunity to deploy the **PBC upgrade** that was already in development:

**Technical Rationale:**
1. **Single Disruption Window:** Combining restart + PBC deployment minimizes operational disruption
2. **Clean Configuration:** New genesis can include PBC configuration from start
3. **Validator Alignment:** All 25 validators update simultaneously with consistent configuration
4. **Testing Efficiency:** Validate both network reset and PBC functionality in one testing cycle

**Avoids Future Disruption:**
- Without combined deployment: Second network upgrade needed later for PBC
- With combined deployment: PBC capabilities available immediately at restart
- Reduces validator operational burden (one upgrade vs. two)

### What PBC Enables

**Partition Burst Chains (Parachain Bridge Collators):**

PBCs are Ëtrid's implementation of Polkadot-style parachains, enabling:

1. **Cross-Chain Bridges:**
   - Bitcoin (BTC-PBC)
   - Ethereum (ETH-PBC)
   - Solana (SOL-PBC)
   - 11 additional blockchain bridges

2. **ËDSC Stablecoin Operations:**
   - EDSC-PBC dedicated chain for stablecoin minting/redemption
   - Oracle price feed integration
   - Collateral management

3. **Validity Node Activation:**
   - 16 Contabo validators become active as PBC collators
   - Participate in cross-chain consensus via PPFA (Partition Proof of Authority)
   - Earn rewards for PBC block production

**Initial PBC Deployment:**
- **EDSC-PBC:** Validators 10-17 (8 validators)
- **BTC-PBC:** Validators 18-25 (8 validators)
- Additional PBCs deployed as network grows

### PBC Deployment Impact

**No Additional Risk:**
- PBC runtime already tested in development
- Deployment follows same validator restart procedure
- Can be disabled if issues arise (fallback to FlareChain only)

**Significant Benefits:**
- Cross-chain capabilities immediately available
- 16 Validity Nodes earning PBC rewards from day one
- Foundation for future bridge activations
- Competitive feature parity with Polkadot ecosystem

---

## Infrastructure Cost Analysis

### Azure to Contabo Migration Savings

**Previous Azure Infrastructure (Decommissioned):**
```
16 validators across 4 Azure regions:
- West Europe: 5 VMs
- North Europe: 2 VMs
- UK South: 5 VMs
- France Central: 4 VMs

Estimated monthly cost: $400-500/month
Status: OFFLINE (payment issue triggered migration)
```

**New Contabo Infrastructure (Active):**
```
20 validators across 3 Contabo regions:

Region 1 (vmi2896xxx - Germany):
- 5 validators (VPS M tier)
- €10.50/month each = €52.50/month

Region 2 (vmi3xxx - Germany):
- 4 validators (VPS M tier)
- €10.50/month each = €42/month

Region 3 (vmi4xxx - USA, St. Louis):
- 7 validators (VPS M tier)
- €10.50/month each = €73.50/month

Total: 16 validators × €10.50 = €168/month (~$180/month USD)
```

**Oracle Cloud Infrastructure (Maintained):**
```
9 Directors on Oracle Cloud Free Tier:
- 6 VMs currently active (Gizzi, AuditDev, 4 additional directors)
- 3 VMs planned (future director expansion)

Cost: $0/month (Oracle Cloud Always Free tier)
- 4 ARM Ampere A1 cores per account
- 24 GB RAM per account
- 200 GB storage per account
- Using 2 Oracle accounts for 9 VMs
```

### Total Monthly Cost Comparison

| Infrastructure | Validators | Monthly Cost | Annual Cost |
|---------------|-----------|--------------|-------------|
| **Azure (Previous)** | 16 | $400-500 | $4,800-6,000 |
| **Contabo (Current)** | 16 | $180 | $2,160 |
| **Oracle Cloud (Current)** | 9 | $0 | $0 |
| **TOTAL NEW** | **25** | **$180** | **$2,160** |

**Annual Savings:** $2,640-3,840 per year
**Cost Reduction:** 64-70% decrease
**Validator Increase:** +9 validators (25 vs. 16)

### Cost per Validator Metrics

**Azure Configuration:**
- $25-31 per validator per month
- Standard Azure VMs (B2s, B4ms instances)
- Variable pricing, potential overages

**Contabo Configuration:**
- $11.25 per validator per month (€10.50)
- VPS M tier (6 vCPU, 12GB RAM, 200GB NVMe)
- Fixed pricing, no surprise bills
- Better performance than Azure B-series

**Oracle Cloud Configuration:**
- $0 per director (Always Free tier)
- ARM Ampere A1 (4 cores, 24GB per instance)
- Permanent free tier (not trial)
- Enterprise-grade infrastructure

---

## Performance and Geographic Distribution

### Improved Network Topology

**Azure Distribution (Previous):**
```
Western Europe: 100% of validators
- All 16 validators in Europe
- Single geographic region dependency
- Regulatory risk (EU-only)
```

**New Multi-Region Distribution:**
```
Europe (Germany): 56% (9 Contabo + 6 Oracle in EU regions)
North America (USA): 28% (7 Contabo St. Louis + Oracle US regions)
Global Oracle: 16% (3 Oracle VMs with global anycast)

Total: 25 validators across 2+ continents
```

**Benefits:**
- Geographic decentralization
- Regulatory jurisdiction diversity
- Reduced single-region dependency
- Better global latency distribution
- Improved network resilience

### Performance Benchmarks

**Contabo VPS M Specs:**
- 6 vCPU cores (vs. Azure 2-4 cores)
- 12 GB RAM (vs. Azure 4-8 GB)
- 200 GB NVMe SSD (vs. Azure 64-128 GB HDD)
- 1 Gbps network (same as Azure)

**Expected Performance:**
- 50-100% faster block sync times
- Better disk I/O for database operations
- Improved peer connection handling
- Lower CPU throttling during peak loads

---

## Network Status and Launch Timeline

### Current Phase: Private Validator Network

**Network Characteristics:**
- **Status:** Private mainnet (genesis: November 1, 2025)
- **Participants:** 25 internal validators only
- **Public Access:** None (no public RPC, no external users)
- **Exchange Listings:** None
- **Token Distribution:** Foundation treasury only
- **dApp Deployments:** None (development/testing only)

**Why This Timing is Optimal:**

1. **No External Impact:** Zero public users affected by restart
2. **Early Detection:** Issue found before public launch (avoided major incident)
3. **Clean Restart:** Opportunity to fix configuration properly
4. **Testing Window:** Validate corrections before opening network
5. **Feature Integration:** Deploy PBC upgrade without user disruption

### Post-Restart Launch Plan

**Phase 1: Network Stabilization (November 15-30)**
- 2 weeks monitoring for stability
- Verify GRANDPA finality consistent
- Test PBC bridge operations
- Ensure validator reward distribution

**Phase 2: Limited Testnet (December 1-15)**
- Open RPC endpoints for testing
- Invite select developers/auditors
- Deploy test dApps
- Verify cross-chain bridge functionality

**Phase 3: Public Testnet (December 16-31)**
- Public RPC access
- Faucet for test tokens
- Developer documentation
- Community testing incentives

**Phase 4: Mainnet Public Launch (January 2026)**
- Public token distribution
- Exchange listing preparations
- Public validator onboarding
- Full network decentralization

**Impact of Restart on Timeline:**
- Minimal delay (2-3 weeks added for validation)
- Actually improves launch quality (corrects critical issue)
- Demonstrates proper incident response
- Builds confidence in network stability

---

## Transparency and Communication

### Why We're Announcing This

Even though the network is private with no public users, we are documenting this event for:

**Future Audits:**
- Complete historical record
- Technical decision rationale
- State preservation proof
- Configuration change tracking

**Team Transparency:**
- Internal validator operators need clarity
- Foundation governance documentation
- Development team knowledge base
- Future team member onboarding

**Best Practices:**
- Establishing culture of transparency
- Documenting lessons learned
- Building audit trail for future regulators
- Demonstrating responsible network management

### Lessons Learned

**Technical Lessons:**
1. GRANDPA authority configuration must exactly match intended finality set
2. Genesis validators should be validated before mainnet genesis
3. Validator migration requires strict chain-state synchronization
4. Infrastructure redundancy critical during transitions

**Operational Lessons:**
1. Cost management critical for long-term sustainability
2. Multi-cloud strategy reduces vendor lock-in risk
3. Early detection better than late fixes
4. Private launch phase valuable for identifying issues

**Process Improvements:**
1. Implement pre-genesis validator configuration audit
2. Create automated chainspec validation tools
3. Develop validator migration playbooks
4. Enhance monitoring for authority set mismatches

---

## Technical Support and Resources

### For Validator Operators

**New Chainspec Distribution:**
- Distributed via secure channel to all 25 validators
- File: `chainspec-mainnet-v2-raw.json`
- Genesis hash will be different (new chain)
- Session keys remain same (no regeneration needed)

**Updated Startup Commands:**
- Directors (1-9): Use new chainspec, `--validator` flag
- Validity Nodes (10-25): Use new chainspec, `--validator` flag
- PBC Collators (if enabled): Additional collator commands provided

**Support Channels:**
- Internal validator operations channel
- Technical documentation in `/docs/mainnet/`
- Direct support from core development team

### Documentation References

**Technical Details:**
- [Network Restart Technical Plan](NETWORK_RESTART_TECHNICAL_PLAN.md)
- [Infrastructure Cost Analysis](INFRASTRUCTURE_COST_ANALYSIS.md)
- [PBC Upgrade Rationale](PBC_UPGRADE_RATIONALE.md)
- [Finality Root Cause Analysis](migration_for_mainnet/FINALITY_ROOT_CAUSE_ANALYSIS.md)

**Operational Guides:**
- [Current Validator Infrastructure](CURRENT_VALIDATOR_INFRASTRUCTURE.md)
- [Migration to Contabo Plan](migration_for_mainnet/MIGRATION_TO_CONTABO_PLAN.md)
- [Validator Final Config](VALIDATOR_FINAL_CONFIG.md)

---

## Frequently Asked Questions

### Q: Will my ETR balance be affected?
**A:** No. All account balances as of block #63,274 are fully preserved and will be included in the new genesis state.

### Q: Do I need to regenerate my validator session keys?
**A:** No. Existing session keys remain valid and will be included in the new chainspec.

### Q: Will the genesis hash change?
**A:** Yes. This is a new chain starting from block #63,274 state, so it will have a new genesis hash. This is expected and necessary.

### Q: What happens to blocks #63,275 - #76,401?
**A:** These unfinalized blocks are not included in the new chain. Since the network was private with no external transactions, there is no impact.

### Q: How long will validators be offline?
**A:** Estimated 2-4 hours during coordinated restart. Directors start first, then Validity Nodes join within 30 minutes.

### Q: Can I still use my Oracle Cloud / Contabo / Azure VM?
**A:** Yes. The VM infrastructure doesn't change, only the chainspec file and node software (if PBC enabled).

### Q: Will validator rewards be affected?
**A:** Validator rewards resume from new genesis. Any pending rewards from blocks #63,275-#76,401 (unfinalized) are not distributed.

### Q: What if I miss the restart window?
**A:** Your validator can join anytime after restart. Download new chainspec, update node command, restart. You'll sync from other validators.

### Q: Is this a hard fork?
**A:** No. This is a network restart with state migration. A hard fork implies splitting from existing chain; this is a coordinated fresh start with preserved state.

### Q: Will this happen again?
**A:** Extremely unlikely. The root cause (GRANDPA authority misconfiguration) is being corrected. Improved validation processes prevent recurrence.

---

## Conclusion

The Ëtrid FlareChain controlled network reset is a **proactive measure** to correct a critical configuration issue discovered during infrastructure migration. By acting decisively during the private validator phase, we are:

**Preserving Value:**
- All finalized state (block #63,274) fully preserved
- All account balances maintained
- Validator configurations intact
- Zero impact to future operations

**Improving Operations:**
- Corrected GRANDPA authority configuration (9 directors, not 25)
- Reduced infrastructure costs by 64-70%
- Increased validator count from 16 to 25
- Improved geographic distribution

**Enabling Growth:**
- PBC bridge capabilities deployed
- 16 Validity Nodes activated
- Foundation for cross-chain operations
- Competitive feature set

**Demonstrating Best Practices:**
- Transparent communication
- Documented decision process
- Lessons learned captured
- Audit trail established

The network restart will occur **November 13-14, 2025** after thorough testing and validation. All validator operators will receive detailed technical instructions and support throughout the process.

**This is the right time to fix this properly** - during private operations, with minimal impact, and maximum benefit for future network growth.

---

**Document Version:** 1.0
**Last Updated:** November 9, 2025
**Status:** APPROVED for distribution to validator operators
**Next Update:** Post-restart network status (November 15, 2025)

**Contact:** Ëtrid Foundation Technical Operations Team
**Emergency Support:** Available 24/7 during restart window
