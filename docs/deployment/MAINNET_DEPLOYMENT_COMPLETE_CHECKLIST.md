# MAINNET DEPLOYMENT COMPLETE CHECKLIST
## Ëtrid Payment/Treasury/Reserve System

**Version:** 1.0
**Date:** 2025-10-31
**Target Deployment:** Q4 2025
**Status:** COMPREHENSIVE DEPLOYMENT GUIDE
**Classification:** CRITICAL - Foundation Directors & Core Team Only

---

## TABLE OF CONTENTS

1. [Pre-Deployment Checklist](#1-pre-deployment-checklist)
2. [Deployment Day Checklist](#2-deployment-day-checklist)
3. [Post-Deployment Validation (0-2 hours)](#3-post-deployment-validation-0-2-hours)
4. [Validator Actions Required (0-48 hours)](#4-validator-actions-required-0-48-hours)
5. [Monitoring Setup (0-7 days)](#5-monitoring-setup-0-7-days)
6. [30-Day Post-Launch](#6-30-day-post-launch)
7. [Emergency Procedures](#7-emergency-procedures)
8. [Contact Information](#8-contact-information)

---

## EXECUTIVE SUMMARY

This checklist covers the complete deployment of the Ëtrid payment, treasury, and reserve system to mainnet. The system includes:

- **6 Core Pallets**: treasury, consensus-day, validator-rewards, multiasset-reserve, edsc-stability, circuit-breaker
- **Foundation Governance**: 9 Directors with 6-of-9 multisig threshold (7-of-9 for emergencies)
- **Annual Consensus Day**: Constitutional governance event (Dec 1st, 12:00 AM PST)
- **Validator Rewards**: Cold storage payment accounts, performance-based rewards
- **EDSC Stablecoin**: Multi-asset reserve backing, $1 peg, circuit breaker protection
- **Treasury Management**: Multi-source funding, category budgets, disbursement workflow

**Timeline**: 2-4 weeks from start to full deployment
**Critical Path**: 12-16 hours of focused development + 48 hours testnet validation

---

## 1. PRE-DEPLOYMENT CHECKLIST

### 1.1 Runtime Development (Est: 4-6 hours)

#### 1.1.1 Transaction Fee Routing to Treasury
- [ ] Create `DealWithFees` handler in runtime
  ```bash
  # Location: /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs
  # Implement OnUnbalanced trait for 50/50 split (treasury/burn)
  ```
- [ ] Wire to `pallet_transaction_payment::Config`
- [ ] Test on local devnet
  ```bash
  cd /Users/macbook/Desktop/etrid
  cargo build --release -p flarechain-runtime
  ./target/release/flarechain-node --dev --tmp
  ```
- [ ] Verify 50% of fees go to treasury account
- [ ] Verify 50% of fees are burned

**Success Criteria**:
- Transaction fees split correctly (50% treasury, 50% burn)
- Treasury balance increases with each transaction
- No runtime errors in logs

---

#### 1.1.2 Consensus Day Minting Integration
- [ ] Add `TreasuryInterface` trait to consensus-day pallet
  ```bash
  # Location: /Users/macbook/Desktop/etrid/src/pallets/pallet-consensus-day/src/lib.rs
  ```
- [ ] Wire `execute_approved_budgets()` to mint and fund treasury
- [ ] Test proposal approval → minting → treasury funding flow
- [ ] Verify inflation rate cap (0-5% max) enforced
- [ ] Test treasury funding source recording

**Success Criteria**:
- Approved proposals trigger minting correctly
- Minted tokens arrive in treasury account
- Inflation cap enforced (cannot exceed 5% annually)
- Treasury records funding source as "ConsensusDayMinting"

---

#### 1.1.3 Validator Slashing Integration
- [ ] Update `execute_slash()` in staking pallet
  ```bash
  # Location: /Users/macbook/Desktop/etrid/11-peer-roles/staking/pallet/src/lib.rs
  ```
- [ ] Implement 50/50 split (treasury/burn)
- [ ] Wire to payment accounts (not session keys)
- [ ] Test slashing events for each offense type:
  - [ ] Downtime (5% slash)
  - [ ] Equivocation (10% slash)
  - [ ] Censorship (20% slash)
  - [ ] Malicious attack (100% slash + removal)
- [ ] Verify slashed funds split correctly

**Success Criteria**:
- Slashing affects payment accounts, not session keys (security!)
- 50% of slashed funds go to treasury
- 50% of slashed funds are burned
- Malicious attacks trigger validator removal

---

#### 1.1.4 Cross-Chain Fee Collection
- [ ] Add fee collection to BTC bridge pallet
- [ ] Add fee collection to ETH bridge pallet
- [ ] Add fee collection to SOL bridge pallet
- [ ] Add fee collection to XRP bridge pallet
- [ ] Add fee collection to BNB bridge pallet
- [ ] Add fee collection to TRX bridge pallet
- [ ] Add fee collection to XLM bridge pallet
- [ ] Implement 10% treasury, 90% validator split
- [ ] Test cross-chain transfers with fee deduction

**Success Criteria**:
- All 7 bridge pallets collect fees (0.1% of transfer amount)
- 10% of fees go to treasury
- 90% of fees go to validator reward pool
- Fee collection recorded in treasury

---

#### 1.1.5 EDSC Stability Fee Integration
- [ ] Wire `collect_interest()` to treasury in pallet-edsc-stability
  ```bash
  # Location: /Users/macbook/Desktop/etrid/src/pallets/pallet-edsc-stability/src/lib.rs
  ```
- [ ] Test interest accrual on EDSC positions
- [ ] Verify interest sent to treasury
- [ ] Test liquidation penalty routing to treasury

**Success Criteria**:
- Interest accrues on EDSC positions
- Collected interest sent to treasury
- Liquidation penalties (5%) sent to treasury
- Treasury records funding source correctly

---

### 1.2 Runtime Integration (Est: 2-3 hours)

#### 1.2.1 Add Pallets to Runtime Cargo.toml
- [ ] Add `pallet-treasury` dependency
  ```toml
  pallet-treasury = { path = "../../../src/pallets/pallet-treasury", default-features = false }
  ```
- [ ] Add `pallet-consensus-day` dependency
- [ ] Add `pallet-multiasset-reserve` dependency
- [ ] Add `pallet-edsc-stability` dependency
- [ ] Add `pallet-circuit-breaker` dependency
- [ ] Update `std` feature list to include all new pallets
- [ ] Run `cargo check` to verify dependencies

---

#### 1.2.2 Implement Runtime Config Traits
- [ ] Implement `pallet_treasury::Config` for Runtime
  ```rust
  parameter_types! {
      pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
      pub const DirectorCount: u8 = 9;
      pub const ApprovalThreshold: u8 = 6;
      pub const EmergencyThreshold: u8 = 7;
      pub const ProposalExpiration: BlockNumber = 7 * DAYS;
  }

  impl pallet_treasury::Config for Runtime {
      type RuntimeEvent = RuntimeEvent;
      type Currency = Balances;
      type DirectorCount = DirectorCount;
      type ApprovalThreshold = ApprovalThreshold;
      type EmergencyThreshold = EmergencyThreshold;
      type ProposalExpiration = ProposalExpiration;
  }
  ```
- [ ] Implement `pallet_consensus_day::Config` for Runtime
  ```rust
  parameter_types! {
      pub const ConsensusDayPalletId: PalletId = PalletId(*b"py/cnsdy");
      pub const ProposalBond: Balance = 10_000 * UNITS; // 10k ËTR
      pub const MaxInflationBps: u32 = 500; // 5% max
      pub const MinDirectorStake: Balance = 128 * UNITS; // 128 ËTR
  }

  impl pallet_consensus_day::Config for Runtime {
      type RuntimeEvent = RuntimeEvent;
      type Currency = Balances;
      type TreasuryAccount = TreasuryAccount;
      type ProposalBond = ProposalBond;
      type MaxInflationBps = MaxInflationBps;
      type MinDirectorStake = MinDirectorStake;
      type WeightInfo = ();
  }
  ```
- [ ] Implement `pallet_multiasset_reserve::Config`
- [ ] Implement `pallet_edsc_stability::Config`
- [ ] Implement `pallet_circuit_breaker::Config`

---

#### 1.2.3 Add to construct_runtime! Macro
- [ ] Add all 5 new pallets to construct_runtime!
  ```rust
  construct_runtime!(
      pub enum Runtime where
          Block = Block,
          NodeBlock = opaque::Block,
          UncheckedExtrinsic = UncheckedExtrinsic,
      {
          // Existing pallets...
          System: frame_system,
          Timestamp: pallet_timestamp,
          Balances: pallet_balances,

          // NEW PALLETS:
          Treasury: pallet_treasury,
          ConsensusDay: pallet_consensus_day,
          ValidatorRewards: pallet_validator_rewards, // Already added
          MultiassetReserve: pallet_multiasset_reserve,
          EdscStability: pallet_edsc_stability,
          CircuitBreaker: pallet_circuit_breaker,
      }
  );
  ```
- [ ] Verify pallet ordering (system pallets first, then custom pallets)

---

#### 1.2.4 Update Runtime Version
- [ ] Increment `spec_version` in RuntimeVersion
  ```rust
  #[sp_version::runtime_version]
  pub const VERSION: RuntimeVersion = RuntimeVersion {
      spec_name: create_runtime_str!("etrid"),
      impl_name: create_runtime_str!("etrid"),
      authoring_version: 1,
      spec_version: 105,  // INCREMENT FROM 104
      impl_version: 1,
      apis: RUNTIME_API_VERSIONS,
      transaction_version: 1,
      system_version: 1,
  };
  ```

---

#### 1.2.5 Create Storage Migrations
- [ ] Write migration code for hot upgrade
  ```rust
  pub struct Migrations<Runtime>(PhantomData<Runtime>);

  impl<Runtime> OnRuntimeUpgrade for Migrations<Runtime> {
      fn on_runtime_upgrade() -> Weight {
          // Migrate treasury data
          // Initialize consensus day state
          // Migrate multiasset reserve
          // Initialize EDSC stability
          // Setup circuit breakers
      }
  }
  ```
- [ ] Add migration to Executive type
- [ ] Test migration with `try-runtime`

---

### 1.3 Testing (Est: 4-6 hours)

#### 1.3.1 Unit Tests
- [ ] Treasury disbursement tests (propose → approve → execute)
  ```bash
  cd /Users/macbook/Desktop/etrid/src/pallets/pallet-treasury
  cargo test --package pallet-treasury
  ```
- [ ] Emergency withdrawal tests (requires 7/9 signatures)
- [ ] Budget allocation update tests
- [ ] Consensus Day phase progression tests
- [ ] Proposal submission and voting tests
- [ ] Validator reward calculation tests
- [ ] Payment account registration tests
- [ ] Performance tracking tests
- [ ] Multiasset reserve rebalancing tests
- [ ] EDSC minting and stability tests
- [ ] Circuit breaker trigger tests

**Success Criteria**: All unit tests pass with 100% coverage on critical paths

---

#### 1.3.2 Integration Tests
- [ ] Build complete runtime
  ```bash
  cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
  cargo build --release
  ```
- [ ] Run try-runtime migration test
  ```bash
  try-runtime \
    --runtime ./target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
    on-runtime-upgrade \
    --uri wss://mainnet.etrid.network:9944 \
    live
  ```
- [ ] Deploy to local testnet
  ```bash
  ./target/release/flarechain-node --dev --tmp
  ```
- [ ] Execute test transactions:
  - [ ] Send transaction (verify 50% fee to treasury)
  - [ ] Mint EDSC (verify stability fees)
  - [ ] Simulate slashing (verify 50% to treasury)
  - [ ] Perform cross-chain transfer (verify fee split)
- [ ] Test full Consensus Day cycle (Registration → Voting → Minting → Distribution)

**Success Criteria**:
- Runtime compiles without errors
- Migration test passes
- All money flows work correctly
- No consensus disruptions

---

#### 1.3.3 Ember Testnet Deployment (48+ hours monitoring)
- [ ] Deploy runtime to Ember testnet
  ```bash
  cd /Users/macbook/Desktop/etrid
  ./scripts/deploy-runtime-upgrade.sh \
    --network ember \
    --runtime ./target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
    --sudo-account <sudo-key>
  ```
- [ ] Monitor for 48+ hours minimum
- [ ] Have 3+ validators test payment account registration
- [ ] Execute test disbursement from treasury
- [ ] Test emergency withdrawal procedure (dry run)
- [ ] Monitor performance metrics:
  - [ ] Block production continuous
  - [ ] Finality not affected
  - [ ] No runtime errors
  - [ ] Memory usage stable
  - [ ] Transaction throughput normal

**Success Criteria**:
- 48+ hours of stable operation
- No critical issues discovered
- Validators successfully register payment accounts
- Treasury operations work correctly

---

### 1.4 Documentation (Est: 2-3 hours)

#### 1.4.1 Update Ivory Papers Vol III
- [ ] Add treasury system architecture section
- [ ] Add multiasset reserve documentation
- [ ] Add EDSC stability mechanism details
- [ ] Add emergency recovery procedures
- [ ] Update economic model with new fee flows
- [ ] Document Consensus Day phases and timing
- [ ] Add validator reward calculation formulas

---

#### 1.4.2 Update Technical Specifications
- [ ] Complete system architecture diagram
- [ ] Document integration points between pallets
- [ ] API reference for all new pallets
- [ ] Storage layout documentation
- [ ] Event emission documentation

---

#### 1.4.3 Update Foundation Charter
- [ ] Treasury governance section
- [ ] Director responsibilities and election process
- [ ] Emergency procedure authorization
- [ ] Multisig signing requirements
- [ ] Annual Consensus Day constitutional role

---

### 1.5 Security & Audit

#### 1.5.1 Security Audit (HIGHLY RECOMMENDED)
- [ ] Engage external auditor for critical pallets
- [ ] Focus on:
  - [ ] Treasury multisig security
  - [ ] EDSC reserve mechanics
  - [ ] Slashing to payment account flow
  - [ ] Emergency withdrawal logic
  - [ ] Migration safety
- [ ] Address all critical and high severity findings
- [ ] Retest after fixes

---

#### 1.5.2 Economic Model Review
- [ ] Verify inflation rate caps enforced
- [ ] Review validator reward economics (3% annual pool)
- [ ] Validate EDSC collateralization ratios (150% min, 120% liquidation)
- [ ] Confirm treasury funding sources cover expected expenses
- [ ] Model emergency scenarios (treasury depletion, EDSC peg break)

---

### 1.6 Foundation Preparation

#### 1.6.1 Director Key Preparation
- [ ] All 9 directors have hardware wallets configured
- [ ] Director accounts created and funded (minimum 128 ËTR each)
- [ ] Director accounts added to genesis config or multisig
- [ ] Backup seeds secured in multiple locations
- [ ] Key ceremony conducted and documented

**Director Key Security Checklist**:
- [ ] Hardware wallet (Ledger/Trezor recommended)
- [ ] Seed phrase written on metal plate
- [ ] Stored in bank safe deposit box or equivalent
- [ ] Second backup with trusted third party
- [ ] Never exposed to internet-connected device

---

#### 1.6.2 Foundation Multisig Setup
- [ ] Create 6-of-9 multisig account for treasury control
  ```bash
  cd /Users/macbook/Desktop/etrid
  ./create-foundation-multisig.sh
  ```
- [ ] Fund multisig account with operational balance (1,000 ËTR)
- [ ] Test multisig signing with 6 directors
- [ ] Test emergency multisig with 7 directors
- [ ] Document multisig address in all relevant places
- [ ] Update genesis config with multisig as treasury controller

**Multisig Address**: `5GFoundationMultisigAddress...` (replace after creation)

---

#### 1.6.3 Initial Treasury Funding
- [ ] Determine initial treasury allocation from genesis
  - **Recommended**: 5% of total supply (50M ËTR if 1B total)
- [ ] Allocate to budget categories:
  - [ ] Development: 40% (20M ËTR)
  - [ ] Marketing: 20% (10M ËTR)
  - [ ] Operations: 15% (7.5M ËTR)
  - [ ] Grants: 15% (7.5M ËTR)
  - [ ] Emergency Reserve: 10% (5M ËTR)
- [ ] Add to genesis config
  ```json
  {
    "treasury": {
      "initialBalance": "50000000000000000000000000",
      "directors": ["5Dir1...", "5Dir2...", "..."],
      "budgetAllocations": {
        "Development": "20000000000000000000000000",
        "Marketing": "10000000000000000000000000",
        "Operations": "7500000000000000000000000",
        "Grants": "7500000000000000000000000",
        "Emergency": "5000000000000000000000000"
      }
    }
  }
  ```

---

#### 1.6.4 Validator Reward Pool Funding
- [ ] Calculate initial reward pool size
  - **Recommended**: 10M ËTR (enough for ~100 epochs)
  - **Formula**: (3% of supply / 365 days) × 100 days = ~8.2M ËTR minimum
- [ ] Add to genesis config or plan post-launch funding
  ```json
  {
    "validatorRewards": {
      "initialRewardPool": "10000000000000000000000000"
    }
  }
  ```
- [ ] Alternative: Fund via treasury disbursement after launch

---

#### 1.6.5 EDSC Reserve Initialization
- [ ] Prepare initial reserve collateral:
  - [ ] ËTR (native): 40% allocation
  - [ ] Wrapped BTC (sBTC): 30% allocation
  - [ ] Wrapped ETH (sETH): 20% allocation
  - [ ] USDC: 10% allocation
- [ ] Calculate amounts based on target EDSC supply
  - **Example**: For 1M EDSC backed at 200% collateralization = $2M reserve
- [ ] Add to genesis config
  ```json
  {
    "multiassetReserve": {
      "initialReserves": [
        {"asset": "ETR", "amount": "1000000000000000000000000"},
        {"asset": "sBTC", "amount": "100000000000"},
        {"asset": "sETH", "amount": "1000000000000000000000"},
        {"asset": "USDC", "amount": "100000000000"}
      ]
    }
  }
  ```

---

#### 1.6.6 Genesis Configuration Review
- [ ] All pallet configurations present in genesis
- [ ] Director accounts correctly specified
- [ ] Initial balances calculated correctly
- [ ] Treasury PalletId matches runtime (`py/trsry`)
- [ ] Consensus Day date set correctly (Dec 1, 2025, 12:00 AM PST = Unix timestamp 1733043600)
- [ ] Validator session keys included (all 21 validators)
- [ ] Chain specification JSON validated
  ```bash
  cd /Users/macbook/Desktop/etrid
  ./target/release/flarechain-node build-spec --chain mainnet --raw > flarechain-mainnet-raw.json
  # Manually verify JSON structure
  ```

---

### 1.7 Pre-Deployment Final Checks

#### 1.7.1 Code Freeze
- [ ] Create release branch `release/v1.0.0-mainnet`
- [ ] Tag release `v1.0.0`
- [ ] No more code changes allowed after this point
- [ ] Archive release artifacts:
  ```bash
  cd /Users/macbook/Desktop/etrid
  tar -czf etrid-v1.0.0-mainnet.tar.gz \
    target/release/flarechain-node \
    flarechain-mainnet-raw.json \
    genesis-accounts-*/
  ```

---

#### 1.7.2 Communication Preparation
- [ ] Draft mainnet launch announcement
- [ ] Prepare social media posts (Twitter, Discord, Telegram)
- [ ] Write blog post explaining new features
- [ ] Create validator notification email/message
- [ ] Prepare status page for monitoring

---

#### 1.7.3 Infrastructure Preparation
- [ ] All 21 validator VMs online and accessible
- [ ] SSH keys configured for all validators
- [ ] Monitoring stack deployed (Prometheus + Grafana)
- [ ] Alerting configured (PagerDuty/email)
- [ ] Backup systems tested
- [ ] RPC nodes ready for public access
- [ ] Telemetry endpoints configured

---

## 2. DEPLOYMENT DAY CHECKLIST

### 2.1 Pre-Deployment (T-2 hours)

#### 2.1.1 Final Verification
- [ ] All directors online and available
- [ ] All validators notified of deployment window
- [ ] Monitoring systems active
- [ ] Status page set to "Scheduled Maintenance"
- [ ] Community notified via all channels
- [ ] Emergency contacts on standby

---

#### 2.1.2 Backup Current Runtime
- [ ] Export current runtime WASM
  ```bash
  cd /Users/macbook/Desktop/etrid
  curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method":"state_getMetadata"}' \
    https://mainnet.etrid.network:9944 > current-runtime-metadata.json
  ```
- [ ] Save current runtime WASM hash
- [ ] Document current spec_version (should be 104)
- [ ] Store backup in multiple locations

---

### 2.2 Deployment Execution (T-0)

#### 2.2.1 Build New Runtime
- [ ] Final build of runtime with all pallets
  ```bash
  cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
  cargo build --release
  ```
- [ ] Extract WASM binary
  ```bash
  cp target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
     /Users/macbook/Desktop/etrid/release-packages/flarechain_runtime_v105.wasm
  ```
- [ ] Calculate and verify WASM hash
  ```bash
  subwasm info release-packages/flarechain_runtime_v105.wasm
  # Record Blake2-256 hash: 0x...
  ```

---

#### 2.2.2 Verify Runtime Hash
- [ ] Compare with Ember testnet deployment hash
- [ ] Verify hash matches build from tagged release
- [ ] All directors independently verify hash
  ```bash
  # Each director runs:
  sha256sum flarechain_runtime_v105.wasm
  # Compare hashes via secure channel
  ```

---

#### 2.2.3 Create Runtime Upgrade Proposal

**Option A: Via Governance (Recommended for Decentralization)**
```bash
# Submit democracy proposal for runtime upgrade
etrid-cli governance submit-proposal \
  --call "system.setCode" \
  --wasm release-packages/flarechain_runtime_v105.wasm \
  --title "Runtime Upgrade v105: Payment/Treasury/Reserve System" \
  --description "Adds 5 new pallets for complete treasury and payment system" \
  --ws wss://mainnet.etrid.network:9944 \
  --account <proposer-account>
```
- [ ] Submit proposal
- [ ] Wait for voting period (typically 7 days)
- [ ] Monitor vote progress
- [ ] Execute after approval

**Option B: Via Sudo (If Still Enabled, Faster)**
```bash
# Submit via sudo
etrid-cli sudo set-code \
  --wasm release-packages/flarechain_runtime_v105.wasm \
  --ws wss://mainnet.etrid.network:9944 \
  --account <sudo-account>
```
- [ ] Submit sudo transaction
- [ ] Immediately executes

**Option C: Via Foundation Multisig (Recommended Balance)**
```bash
# Create multisig proposal (requires 6-of-9, use 7 for safety)
etrid-cli multisig as-multi \
  --threshold 7 \
  --other-signatories <dir1,dir2,dir3,dir4,dir5,dir6,dir7,dir8> \
  --call-hash $(etrid-cli encode-call system.setCode) \
  --max-weight 2000000000000 \
  --ws wss://mainnet.etrid.network:9944 \
  --account <director1-account>
```
- [ ] Director 1 initiates multisig
- [ ] Directors 2-7 approve and sign
- [ ] Director 7 executes final signature

---

#### 2.2.4 Collect Director Signatures

**For Multisig Option (7 required)**:
- [ ] Director 1 (Initiator): `5Dir1...` ✅
- [ ] Director 2: `5Dir2...` ✅
- [ ] Director 3: `5Dir3...` ✅
- [ ] Director 4: `5Dir4...` ✅
- [ ] Director 5: `5Dir5...` ✅
- [ ] Director 6: `5Dir6...` ✅
- [ ] Director 7 (Executor): `5Dir7...` ✅

**Timeline for Signatures**: Maximum 2 hours from initiation to execution

**Director Signing Commands**:
```bash
# Directors 2-6 run:
etrid-cli multisig as-multi \
  --threshold 7 \
  --other-signatories <all-other-directors> \
  --call-hash <call-hash-from-director-1> \
  --ws wss://mainnet.etrid.network:9944 \
  --account <director-account>

# Director 7 (final) runs:
etrid-cli multisig as-multi-final \
  --threshold 7 \
  --other-signatories <all-other-directors> \
  --call <full-call-data> \
  --ws wss://mainnet.etrid.network:9944 \
  --account <director7-account>
```

---

#### 2.2.5 Submit Runtime Upgrade Transaction
- [ ] Final director executes multisig transaction
- [ ] Monitor transaction inclusion in block
  ```bash
  # Watch events
  etrid-cli events subscribe --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Wait for `system.CodeUpdated` event
- [ ] Transaction should finalize within 2-3 blocks (~12-18 seconds)

**Expected Timeline**:
- Transaction broadcast: T+0s
- Included in block: T+6s (one block)
- Finalized: T+12-18s (2-3 blocks)
- Runtime upgraded: T+18s (next block)

---

### 2.3 Monitoring During Upgrade (T+0 to T+30 minutes)

#### 2.3.1 Monitor Block Production
- [ ] Check all 21 validators still producing blocks
  ```bash
  cd /Users/macbook/Desktop/etrid
  ./scripts/monitor-validators.sh
  ```
- [ ] Block time remains 6 seconds
- [ ] No validators slashed during upgrade
- [ ] Finalization continues

**Alert Thresholds**:
- Block production gap >30 seconds: WARNING
- Block production gap >60 seconds: CRITICAL
- Finalization stopped >2 minutes: CRITICAL - INITIATE ROLLBACK

---

#### 2.3.2 Verify Migration Logs
- [ ] Check validator logs for migration success
  ```bash
  ssh ubuntu@<validator-ip> "journalctl -u flarechain-validator -n 100 | grep -i migration"
  ```
- [ ] Expected log: "✅ Migration successful: 21 validators initialized"
- [ ] No error logs during migration
- [ ] Storage migrations completed successfully

**Expected Migration Logs**:
```
[INFO] Starting on_runtime_upgrade
[INFO] Treasury pallet initializing...
[INFO] Consensus Day pallet initializing...
[INFO] Validator Rewards migrating 21 validators...
[INFO] Multiasset Reserve initializing...
[INFO] EDSC Stability initializing...
[INFO] Circuit Breaker initializing...
[INFO] ✅ Migration completed successfully
[INFO] Post-upgrade weight: 500000000
```

---

#### 2.3.3 Check Storage Migrations
- [ ] Verify validator payment accounts initialized
  ```bash
  etrid-cli query validator-rewards payment-accounts \
    --ws wss://mainnet.etrid.network:9944
  # Should return 21 accounts
  ```
- [ ] Verify treasury directors list
  ```bash
  etrid-cli query treasury directors \
    --ws wss://mainnet.etrid.network:9944
  # Should return 9 director accounts
  ```
- [ ] Verify consensus day state initialized
  ```bash
  etrid-cli query consensus-day state \
    --ws wss://mainnet.etrid.network:9944
  ```

---

## 3. POST-DEPLOYMENT VALIDATION (0-2 hours)

### 3.1 Immediate Validation (T+0 to T+15 minutes)

#### 3.1.1 Verify All Validators Online
- [ ] Query validator set
  ```bash
  etrid-cli query session validators \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Confirm count = 21
- [ ] All validators producing blocks
- [ ] No validators offline

**Success Criteria**: 21/21 validators active

---

#### 3.1.2 Verify Payment Account Migration
- [ ] Check each validator has payment account initialized
  ```bash
  for i in {1..21}; do
    etrid-cli query validator-rewards payment-account-of <session-account-$i> \
      --ws wss://mainnet.etrid.network:9944
  done
  ```
- [ ] Default mapping: session → session (validators will update to cold storage)
- [ ] All stakes migrated correctly

**Success Criteria**: All 21 validators have payment accounts

---

#### 3.1.3 Verify Performance Metrics
- [ ] Performance tracking started for all validators
  ```bash
  etrid-cli query validator-rewards performance-of <session-account> \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Uptime metrics recording
- [ ] Block production counting
- [ ] Finality voting tracking

---

#### 3.1.4 Verify Treasury Account Accessible
- [ ] Query treasury balance
  ```bash
  etrid-cli query system account $(etrid-cli pallet-id py/trsry) \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Verify initial funding present
- [ ] Query treasury directors
  ```bash
  etrid-cli query treasury directors \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Confirm 9 directors listed

---

#### 3.1.5 Verify Consensus Day Operational
- [ ] Query next consensus day date
  ```bash
  etrid-cli query consensus-day next-consensus-day \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Should return: December 1, 2025, 12:00 AM PST (Unix: 1733043600)
- [ ] Query current phase
  ```bash
  etrid-cli query consensus-day current-phase \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Should return: `Registration` (not active until Dec 1)

---

#### 3.1.6 Verify EDSC Minting Available
- [ ] Test EDSC position creation (small amount)
  ```bash
  etrid-cli tx edsc-stability open-position \
    --collateral-asset ETR \
    --collateral-amount 1000000000000000000000 \
    --edsc-amount 100000000 \
    --ws wss://mainnet.etrid.network:9944 \
    --account <test-account>
  ```
- [ ] Verify position created
- [ ] Verify collateralization ratio correct (>150%)
- [ ] Close position to clean up

---

#### 3.1.7 Verify Circuit Breaker Operational
- [ ] Query circuit breaker status
  ```bash
  etrid-cli query circuit-breaker status \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Should return: `Active`
- [ ] Query volume caps
  ```bash
  etrid-cli query circuit-breaker volume-caps \
    --ws wss://mainnet.etrid.network:9944
  ```

---

#### 3.1.8 Verify Emergency Procedures Accessible
- [ ] Test emergency withdrawal simulation (dry run, don't execute)
  ```bash
  etrid-cli tx treasury emergency-withdrawal \
    --amount 1000000000000000000 \
    --recipient <test-account> \
    --reason "Dry run test" \
    --dry-run \
    --ws wss://mainnet.etrid.network:9944 \
    --account <director-account>
  ```
- [ ] Verify requires 7/9 signatures
- [ ] Verify only emergency reserve can be withdrawn

---

### 3.2 Continuous Monitoring (T+15 min to T+2 hours)

#### 3.2.1 Block Production Stability
- [ ] Monitor block production every 5 minutes
  ```bash
  watch -n 300 '/Users/macbook/Desktop/etrid/scripts/check-block-production.sh'
  ```
- [ ] Average block time = 6 seconds ±0.5s
- [ ] No missed blocks
- [ ] Finalization continuous

---

#### 3.2.2 Transaction Fee Flow
- [ ] Execute test transactions
- [ ] Monitor treasury balance increase
  ```bash
  # Before transaction
  BALANCE_BEFORE=$(etrid-cli query system account py/trsry | jq .data.free)

  # Execute transaction
  etrid-cli tx balances transfer \
    --dest <recipient> \
    --amount 1000000000000000000 \
    --ws wss://mainnet.etrid.network:9944

  # After transaction (wait 1 block)
  sleep 6
  BALANCE_AFTER=$(etrid-cli query system account py/trsry | jq .data.free)

  # Calculate difference (should be ~50% of transaction fee)
  echo "Treasury increased by: $(($BALANCE_AFTER - $BALANCE_BEFORE))"
  ```
- [ ] Verify ~50% of fee went to treasury
- [ ] Verify ~50% of fee burned (total issuance decreased)

---

#### 3.2.3 Validator Performance Tracking
- [ ] Check performance metrics updating
  ```bash
  # Get current performance
  etrid-cli query validator-rewards performance-of <session-account> \
    --ws wss://mainnet.etrid.network:9944

  # Wait 10 minutes
  sleep 600

  # Check again (metrics should have changed)
  etrid-cli query validator-rewards performance-of <session-account> \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Blocks authored increasing
- [ ] Uptime percentage updating
- [ ] Finality votes increasing

---

## 4. VALIDATOR ACTIONS REQUIRED (0-48 hours)

### 4.1 Immediate Notification (T+0)

#### 4.1.1 Send Notification to All Validators
**Message Template**:
```
🚨 URGENT: Runtime Upgrade Complete - Action Required 🚨

Ëtrid FlareChain runtime has been upgraded to v105.

✅ What Changed:
- Payment account system is now live
- Treasury management system operational
- Rewards will accumulate starting next epoch
- Slashing now protects your hot keys (targets cold storage)

⚠️ ACTION REQUIRED (within 48 hours):
Register your cold storage payment account to receive validator rewards:

etrid-cli tx validator-rewards register-payment-account \
  --payment-account <YOUR-COLD-STORAGE-ADDRESS> \
  --ws wss://mainnet.etrid.network:9944 \
  --account <YOUR-SESSION-ACCOUNT>

⚠️ CRITICAL WARNING:
If you don't register a payment account within 48 hours, rewards will
go to your session account (hot wallet). This is INSECURE and not recommended.

📖 Documentation:
- /Users/macbook/Desktop/etrid/RUNTIME_UPGRADE_GUIDE.md
- /Users/macbook/Desktop/etrid/MAINNET_PAYMENT_SYSTEM_DEPLOYMENT.md

❓ Support:
- Discord: #validator-support
- Email: validators@etrid.network
- Emergency: foundation@etrid.network
```

Send via:
- [ ] Discord announcement (#validators channel)
- [ ] Telegram validator group
- [ ] Direct email to all validators
- [ ] Twitter announcement

---

### 4.2 Validator Registration Period (T+0 to T+48 hours)

#### 4.2.1 Monitor Payment Account Registrations
- [ ] Track registration count every 6 hours
  ```bash
  # Count registered validators
  etrid-cli query validator-rewards payment-accounts \
    --ws wss://mainnet.etrid.network:9944 | jq 'length'

  # Goal: 20-21 out of 21 within 48 hours
  ```
- [ ] Send reminder at T+24 hours to validators who haven't registered
- [ ] Send urgent reminder at T+36 hours
- [ ] Follow up individually with stragglers at T+42 hours

**Registration Milestones**:
- T+6 hours: 5+ validators registered (GOOD)
- T+12 hours: 10+ validators registered (ON TRACK)
- T+24 hours: 15+ validators registered (GOOD PROGRESS)
- T+36 hours: 18+ validators registered (NEARLY COMPLETE)
- T+48 hours: 20-21 validators registered (SUCCESS)

---

#### 4.2.2 Assist Validators with Registration
**Common Issues & Solutions**:

**Issue 1: Validator doesn't have cold storage account**
- Solution: Guide them through creating one:
  ```bash
  # Generate cold storage account (do this OFFLINE)
  etrid-cli account generate --scheme Sr25519

  # Store seed phrase in hardware wallet or secure vault
  # NEVER expose private key on validator server
  ```

**Issue 2: Registration transaction fails**
- Check: Does session account have balance for transaction fee?
- Solution: Send small amount (1 ËTR) to session account for gas

**Issue 3: Validator confused about which account is which**
- Solution: Provide clear documentation:
  - Session account = Hot keys on validator server (AURA, GRANDPA, ASF)
  - Payment account = Cold storage for receiving rewards (NEW)
  - Controller account = Optional management account (OPTIONAL)

---

#### 4.2.3 Verify Registration Success
For each validator who reports registration:
- [ ] Query their payment account
  ```bash
  etrid-cli query validator-rewards payment-account-of <session-account> \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Verify payment account ≠ session account (cold storage properly set)
- [ ] Mark validator as "registered" in tracking sheet

---

### 4.3 Test Reward Claiming (T+4-6 hours, after first epoch)

#### 4.3.1 Wait for First Epoch Completion
- [ ] Epoch duration = 2400 blocks (~4 hours at 6 sec/block)
- [ ] Monitor epoch end event
  ```bash
  etrid-cli events subscribe --ws wss://mainnet.etrid.network:9944 | grep EpochEnded
  ```

---

#### 4.3.2 Verify Reward Calculation
- [ ] Query pending rewards for each validator
  ```bash
  etrid-cli query validator-rewards pending-rewards <payment-account> \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Calculate expected reward:
  ```
  Daily pool = Total supply × 3% / 365
  Per validator (equal stake) = Daily pool / 21 validators
  With performance multiplier = Base × (0.9 to 1.2)
  ```
- [ ] Verify calculated rewards match expected amounts (±5%)

---

#### 4.3.3 Test Reward Claiming
- [ ] Have 2-3 validators test claiming rewards
  ```bash
  etrid-cli tx validator-rewards claim-rewards \
    --ws wss://mainnet.etrid.network:9944 \
    --account <payment-account>
  ```
- [ ] Verify rewards transferred to payment account
- [ ] Verify pending rewards reset to zero
- [ ] Monitor for any errors

**Success Criteria**:
- Rewards claimed successfully
- Balance in payment account increased correctly
- No transaction failures

---

## 5. MONITORING SETUP (0-7 days)

### 5.1 Treasury Balance Monitoring (T+0)

#### 5.1.1 Configure Prometheus Metrics
```yaml
# /Users/macbook/Desktop/etrid/infrastructure/monitoring/prometheus.yml
scrape_configs:
  - job_name: 'treasury-balance'
    scrape_interval: 60s
    static_configs:
      - targets: ['mainnet.etrid.network:9615']
    metrics_path: '/metrics'
    relabel_configs:
      - source_labels: [__address__]
        target_label: instance
        replacement: 'treasury'
```
- [ ] Deploy Prometheus config update
- [ ] Verify treasury balance metric collecting

---

#### 5.1.2 Create Grafana Dashboard
- [ ] Import treasury dashboard template
  ```bash
  cd /Users/macbook/Desktop/etrid/infrastructure/monitoring/grafana-dashboards
  # Create treasury-dashboard.json
  ```
- [ ] Add panels:
  - [ ] Treasury total balance (line chart)
  - [ ] Funding source breakdown (pie chart)
  - [ ] Budget category allocations (bar chart)
  - [ ] Disbursement history (table)
  - [ ] Daily funding rate (line chart)
  - [ ] Burn vs Treasury split (stacked area chart)

---

#### 5.1.3 Set Treasury Balance Alerts
```yaml
# alerts.yml
groups:
  - name: treasury_alerts
    rules:
      - alert: TreasuryBalanceLow
        expr: treasury_balance_etr < 1000000  # 1M ËTR
        for: 1h
        labels:
          severity: warning
        annotations:
          summary: "Treasury balance below 1M ËTR"

      - alert: TreasuryBalanceCritical
        expr: treasury_balance_etr < 100000  # 100k ËTR
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "Treasury balance critically low"

      - alert: TreasuryNoFunding
        expr: rate(treasury_balance_etr[24h]) <= 0
        for: 24h
        labels:
          severity: warning
        annotations:
          summary: "Treasury hasn't received funding in 24 hours"
```
- [ ] Deploy alert rules
- [ ] Test alerts with dry-run

---

### 5.2 Validator Payment Tracking (T+0)

#### 5.2.1 Create Payment Tracking Dashboard
- [ ] Add Grafana panels:
  - [ ] Validators with registered payment accounts (gauge)
  - [ ] Pending rewards by validator (bar chart)
  - [ ] Claimed rewards history (line chart)
  - [ ] Performance multipliers by validator (heatmap)
  - [ ] Epoch reward distribution (timeline)

---

#### 5.2.2 Set Payment Alerts
```yaml
- alert: ValidatorPaymentAccountsLow
  expr: validator_payment_accounts_count < 18
  for: 48h
  labels:
    severity: warning
  annotations:
    summary: "Less than 18 validators registered payment accounts"

- alert: ValidatorRewardClaimsFailing
  expr: rate(validator_rewards_claim_failed[1h]) > 0
  for: 15m
  labels:
    severity: critical
  annotations:
    summary: "Validators unable to claim rewards"
```

---

### 5.3 EDSC Peg Monitoring (T+0)

#### 5.3.1 Configure Oracle Price Feeds
- [ ] Verify oracle integration for reserve assets:
  - [ ] BTC/USD price feed
  - [ ] ETH/USD price feed
  - [ ] ËTR/USD price feed
  - [ ] USDC/USD price feed
- [ ] Test oracle updates
- [ ] Verify price feed freshness (<5 minutes)

---

#### 5.3.2 Create EDSC Stability Dashboard
- [ ] Add Grafana panels:
  - [ ] EDSC peg deviation from $1 (line chart with ±1% bands)
  - [ ] Reserve collateralization ratio (gauge, target >150%)
  - [ ] EDSC total supply (line chart)
  - [ ] Reserve composition (pie chart)
  - [ ] Liquidation queue (table)
  - [ ] Interest rate adjustments (line chart)

---

#### 5.3.3 Set EDSC Peg Alerts
```yaml
- alert: EDSCPegDeviation5Percent
  expr: abs(edsc_price_usd - 1.0) / 1.0 > 0.05
  for: 15m
  labels:
    severity: warning
  annotations:
    summary: "EDSC peg deviation >5%"

- alert: EDSCPegDeviation10Percent
  expr: abs(edsc_price_usd - 1.0) / 1.0 > 0.10
  for: 5m
  labels:
    severity: critical
  annotations:
    summary: "EDSC peg deviation >10% - Circuit breaker should trigger"

- alert: EDSCCollateralizationLow
  expr: edsc_collateralization_ratio < 1.5
  for: 30m
  labels:
    severity: warning
  annotations:
    summary: "EDSC collateralization below 150%"

- alert: EDSCCollateralizationCritical
  expr: edsc_collateralization_ratio < 1.2
  for: 5m
  labels:
    severity: critical
  annotations:
    summary: "EDSC approaching liquidation threshold"
```

---

### 5.4 Circuit Breaker Monitoring (T+0)

#### 5.4.1 Configure Circuit Breaker Alerts
```yaml
- alert: CircuitBreakerTriggered
  expr: circuit_breaker_active == 1
  for: 1m
  labels:
    severity: critical
  annotations:
    summary: "Circuit breaker activated"
    description: "Emergency pause triggered. Investigate immediately."

- alert: CircuitBreakerVolumeCap
  expr: circuit_breaker_volume_cap_hit == 1
  for: 5m
  labels:
    severity: warning
  annotations:
    summary: "Volume cap reached for {{ $labels.operation }}"
```

---

#### 5.4.2 Create Circuit Breaker Dashboard
- [ ] Add panels:
  - [ ] Circuit breaker status (gauge: Active/Paused)
  - [ ] Volume caps by operation (bar chart)
  - [ ] Current volume vs cap (progress bars)
  - [ ] Circuit breaker trigger history (timeline)

---

### 5.5 Performance Metrics Dashboard (T+0)

#### 5.5.1 Create Comprehensive Metrics Dashboard
- [ ] Add panels:
  - [ ] Total transactions per day
  - [ ] Average transaction fee
  - [ ] Transaction fee → treasury flow
  - [ ] Slashing events (count and amounts)
  - [ ] Cross-chain transfers (volume and fees)
  - [ ] Block production rate
  - [ ] Finalization rate
  - [ ] Validator uptime (heatmap)

---

### 5.6 Alert Configuration (T+0)

#### 5.6.1 Configure Alert Channels
- [ ] PagerDuty integration for critical alerts
- [ ] Email notifications for warnings
- [ ] Discord webhook for all alerts
- [ ] Telegram bot for critical only
- [ ] SMS for emergency (circuit breaker, treasury compromise)

---

#### 5.6.2 Test Alert Delivery
- [ ] Trigger test warning alert
- [ ] Trigger test critical alert
- [ ] Verify all channels receive notifications
- [ ] Verify alert escalation working
- [ ] Document response procedures for each alert type

---

### 5.7 Incident Response Readiness (T+0)

#### 5.7.1 Create Incident Runbooks
- [ ] Treasury Compromise Response Runbook
  - Detection → Freeze → Investigation → Recovery
- [ ] EDSC Peg Break Response Runbook
  - Detection → Circuit Breaker → Rebalancing → Resume
- [ ] Validator Payment Failure Runbook
  - Detection → Root Cause → Manual Distribution → Fix
- [ ] Consensus Day Failure Runbook
  - Detection → Phase Rollback → Manual Execution → Recovery

---

#### 5.7.2 Conduct Incident Response Drill (T+7 days)
- [ ] Schedule drill with all directors
- [ ] Simulate treasury compromise scenario
- [ ] Practice multisig signing process
- [ ] Test emergency withdrawal procedure (on Ember testnet)
- [ ] Measure response time: Target <30 minutes from detection to action
- [ ] Document lessons learned
- [ ] Update runbooks based on findings

---

## 6. 30-DAY POST-LAUNCH

### 6.1 First Epoch Reward Distribution (T+24 hours)

#### 6.1.1 Verify Epoch 1 Rewards
- [ ] Calculate expected total rewards:
  ```
  Annual pool = Total supply × 3%
  Daily pool = Annual pool / 365
  Epoch 1 pool = Daily pool × (24 hours / 24 hours) = Full daily allocation
  ```
- [ ] Query actual distributed rewards
  ```bash
  etrid-cli query validator-rewards epoch-rewards 1 \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Compare actual vs expected (should match within 1%)
- [ ] Verify each validator received correct share based on:
  - Stake amount
  - Performance multiplier
  - Participation bonus

---

#### 6.1.2 Validate Reward Distribution Formula
- [ ] For each validator, verify:
  ```
  Reward = (Validator Stake / Total Staked) × Pool × Performance Multiplier
  Performance = Uptime × Finality × Block Production × Participation
  ```
- [ ] Check performance multipliers in range [0.9, 1.2]
- [ ] Verify participation bonus applied to Consensus Day voters (1.1x)

---

### 6.2 Treasury Funding Verification (T+7 days)

#### 6.2.1 Verify All Funding Sources
- [ ] Transaction fees → Treasury (50%)
  ```bash
  etrid-cli query treasury funding-history \
    --source TransactionFees \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Slashing → Treasury (50%)
  ```bash
  etrid-cli query treasury funding-history \
    --source ValidatorSlashing \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] Cross-chain fees → Treasury (10%)
  ```bash
  etrid-cli query treasury funding-history \
    --source CrossChainFees \
    --ws wss://mainnet.etrid.network:9944
  ```
- [ ] EDSC stability fees → Treasury
  ```bash
  etrid-cli query treasury funding-history \
    --source StabilityFees \
    --ws wss://mainnet.etrid.network:9944
  ```

---

#### 6.2.2 Calculate Weekly Funding Rate
- [ ] Query treasury balance at T+0 and T+7 days
- [ ] Calculate net funding rate
  ```
  Weekly funding = Balance(T+7d) - Balance(T+0) + Disbursements
  ```
- [ ] Compare to projected funding model
- [ ] Verify treasury is accumulating at expected rate

---

### 6.3 EDSC Stability Verification (T+14 days)

#### 6.3.1 Monitor EDSC Peg
- [ ] Record EDSC price deviation from $1 over 14 days
- [ ] Calculate metrics:
  - Average deviation: <1% (GOOD), 1-3% (ACCEPTABLE), >3% (NEEDS ATTENTION)
  - Maximum deviation: <5% (GOOD), 5-10% (CONCERNING), >10% (CRITICAL)
  - Time in range (±1%): >95% (EXCELLENT), 80-95% (GOOD), <80% (NEEDS WORK)

---

#### 6.3.2 Verify Reserve Rebalancing
- [ ] Check if reserve ratios deviated >5% from targets
- [ ] Verify automatic rebalancing triggered
- [ ] Confirm reserve composition returned to target ratios:
  - ËTR: 40% ±5%
  - BTC: 30% ±5%
  - ETH: 20% ±5%
  - USDC: 10% ±5%

---

#### 6.3.3 Test EDSC Liquidations
- [ ] Create test position on Ember testnet
- [ ] Let collateralization fall below 120%
- [ ] Verify liquidation triggered automatically
- [ ] Confirm liquidation penalty (5%) sent to treasury

---

### 6.4 Emergency Procedure Verification (T+21 days)

#### 6.4.1 Conduct Quarterly Emergency Drill
- [ ] Schedule with all 9 directors
- [ ] Scenario: Test treasury emergency withdrawal
- [ ] Execute on Ember testnet:
  1. Director 1-7 sign emergency withdrawal
  2. Measure time from initiation to execution
  3. Verify funds moved correctly
  4. Practice reverting if needed
- [ ] Document response time: Target <2 hours
- [ ] Update emergency contact tree if needed

---

### 6.5 Community Feedback Collection (T+30 days)

#### 6.5.1 Validator Survey
- [ ] Send survey to all 21 validators
- [ ] Questions:
  - How was the payment account registration process?
  - Are rewards arriving correctly?
  - Any issues with payment system?
  - Suggestions for improvement?
- [ ] Collect feedback, address concerns

---

#### 6.5.2 Community Forum Discussion
- [ ] Host AMA (Ask Me Anything) about new treasury system
- [ ] Explain Consensus Day mechanics
- [ ] Answer questions about EDSC stablecoin
- [ ] Gather community sentiment

---

### 6.6 Performance Optimization Review (T+30 days)

#### 6.6.1 Analyze System Performance
- [ ] Review 30-day metrics:
  - Block production stability: Target >99.9%
  - Finalization rate: Target 100%
  - Transaction throughput: Measure average and peak
  - Storage growth rate: Verify sustainable
  - Memory usage: Check for leaks or bloat

---

#### 6.6.2 Identify Optimization Opportunities
- [ ] Review validator performance data
- [ ] Identify any bottlenecks
- [ ] Plan optimizations for next runtime upgrade
- [ ] Document performance baseline for future comparison

---

#### 6.6.3 Economic Model Validation
- [ ] Verify validator incentives working as intended
- [ ] Check if treasury funding adequate for planned expenses
- [ ] Validate EDSC stability mechanisms effective
- [ ] Review inflation rate impact on token economics
- [ ] Make recommendations for Consensus Day voting

---

## 7. EMERGENCY PROCEDURES

### 7.1 Treasury Compromise

**Severity**: CRITICAL (Level 1)
**Response Time**: Immediate (0-15 minutes)
**Required Signatures**: 7/9 Directors

#### 7.1.1 Detection
- Unauthorized transaction from treasury account
- Unexpected balance decrease
- Alert from monitoring system

#### 7.1.2 Immediate Actions (0-5 minutes)
1. [ ] FREEZE all treasury operations via circuit breaker
   ```bash
   etrid-cli tx circuit-breaker pause \
     --target Treasury \
     --ws wss://mainnet.etrid.network:9944 \
     --account <director-account>
   # Requires 7/9 multisig
   ```
2. [ ] Notify all 9 directors via emergency call
3. [ ] Post status update: "Treasury operations temporarily paused for security investigation"

#### 7.1.3 Investigation (5-30 minutes)
1. [ ] Review all recent treasury transactions
2. [ ] Identify compromised keys or accounts
3. [ ] Assess amount at risk
4. [ ] Determine if runtime vulnerability or key compromise

#### 7.1.4 Recovery (30-120 minutes)
**If Key Compromise**:
1. [ ] Execute emergency withdrawal to secure account (7/9 signatures)
2. [ ] Rotate compromised director keys
3. [ ] Restore treasury operations with new multisig

**If Runtime Vulnerability**:
1. [ ] Submit emergency runtime patch
2. [ ] Execute via 7/9 multisig
3. [ ] Restore funds if possible

#### 7.1.5 Post-Incident
- [ ] Full security audit
- [ ] Publish incident report
- [ ] Implement additional safeguards
- [ ] Compensate affected parties if necessary

**Reference**: `/Users/macbook/Desktop/etrid/EMERGENCY_FUND_RECOVERY_GUIDE.md` (Scenario 3)

---

### 7.2 EDSC Peg Break (>10% Deviation)

**Severity**: CRITICAL (Level 1)
**Response Time**: Immediate (0-15 minutes)
**Auto-Action**: Circuit breaker triggers at >10% deviation

#### 7.2.1 Detection
- EDSC price >$1.10 or <$0.90 for >5 minutes
- Circuit breaker auto-triggers
- Alert sent to directors

#### 7.2.2 Immediate Actions (Auto + Manual)
1. [ ] Circuit breaker pauses EDSC minting (AUTOMATIC)
2. [ ] Director on-call assesses situation (5 minutes)
3. [ ] Emergency director call initiated (15 minutes)

#### 7.2.3 Analysis (15-30 minutes)
1. [ ] Determine cause:
   - Oracle malfunction?
   - Reserve depletion?
   - Market manipulation?
   - Black swan event?
2. [ ] Assess reserve health
3. [ ] Calculate required intervention

#### 7.2.4 Intervention Options
**Option A: Reserve Rebalancing** (if reserves healthy)
1. [ ] Inject additional collateral (6/9 approval)
2. [ ] Adjust interest rates to incentivize peg restoration
3. [ ] Resume minting once peg restored

**Option B: Emergency Reserve Injection** (if reserves depleted)
1. [ ] Emergency withdrawal from treasury reserve (7/9 approval)
2. [ ] Inject ËTR or stablecoins into EDSC reserve
3. [ ] Restore collateralization ratio >150%
4. [ ] Resume operations

**Option C: Controlled Shutdown** (if unfixable)
1. [ ] Pause all EDSC operations indefinitely
2. [ ] Allow holders to redeem at collateral value
3. [ ] Plan EDSC 2.0 relaunch with fixes

#### 7.2.5 Recovery
1. [ ] Gradually resume EDSC operations
2. [ ] Monitor peg closely for 48 hours
3. [ ] Publish post-mortem analysis
4. [ ] Implement safeguards to prevent recurrence

**Reference**: `/Users/macbook/Desktop/etrid/EMERGENCY_FUND_RECOVERY_GUIDE.md` (Scenario 2)

---

### 7.3 Validator Payment Failure

**Severity**: HIGH (Level 2)
**Response Time**: <1 hour
**Required Signatures**: 5/9 Directors

#### 7.3.1 Detection
- Validators report missing rewards
- Epoch ends but no distribution events
- Reward pool balance depleted

#### 7.3.2 Immediate Actions (0-15 minutes)
1. [ ] Verify issue: Check reward pool balance
   ```bash
   etrid-cli query validator-rewards reward-pool-balance \
     --ws wss://mainnet.etrid.network:9944
   ```
2. [ ] If balance = 0: **Reward pool depleted**
3. [ ] If balance > 0: **Distribution mechanism failure**

#### 7.3.3 Resolution Path A: Reward Pool Depleted
1. [ ] Calculate required funding for next epoch
   ```
   Required = (Total supply × 3% / 365) × 10 epochs
   ```
2. [ ] Create multisig proposal to fund reward pool from treasury (5/9)
   ```bash
   etrid-cli tx treasury propose-disbursement \
     --category Operations \
     --recipient $(etrid-cli pallet-id validator-rewards) \
     --amount <calculated-amount> \
     --description "Emergency validator reward pool funding" \
     --ws wss://mainnet.etrid.network:9944
   ```
3. [ ] Collect 5/9 signatures
4. [ ] Execute disbursement
5. [ ] Rewards resume automatically next epoch

#### 7.3.4 Resolution Path B: Distribution Failure
1. [ ] Identify root cause (runtime bug, storage corruption, etc.)
2. [ ] Calculate owed rewards manually
3. [ ] Execute manual distribution via treasury (5/9)
   ```bash
   for validator in ${VALIDATORS[@]}; do
     etrid-cli tx treasury propose-disbursement \
       --category Operations \
       --recipient $validator \
       --amount <owed-reward> \
       --description "Manual validator reward distribution"
   done
   ```
4. [ ] Prepare runtime patch to fix distribution bug
5. [ ] Deploy patch via governance or multisig

**Reference**: `/Users/macbook/Desktop/etrid/EMERGENCY_FUND_RECOVERY_GUIDE.md` (Scenario 4)

---

### 7.4 Consensus Day Failure

**Severity**: HIGH (Level 2)
**Response Time**: <2 hours (but Consensus Day is 22 hours)
**Required Signatures**: 6/9 Directors

#### 7.4.1 Possible Failures
- Phase doesn't advance automatically
- Proposal submissions fail
- Voting mechanism broken
- Minting doesn't execute
- Distribution fails

#### 7.4.2 Phase Advancement Failure
1. [ ] Detect: Phase timer expires but phase doesn't advance
2. [ ] Manual phase advancement (requires governance origin):
   ```bash
   etrid-cli tx consensus-day advance-phase \
     --ws wss://mainnet.etrid.network:9944 \
     --account <sudo-or-multisig>
   ```
3. [ ] If fails: Investigate storage state
4. [ ] If corrupted: Restore from backup or reconstruct

#### 7.4.3 Minting Failure
1. [ ] Detect: Minting phase completes but no tokens minted
2. [ ] Manually execute approved budgets:
   ```bash
   for proposal_id in ${APPROVED_PROPOSALS[@]}; do
     etrid-cli tx consensus-day execute-approved-budgets \
       --proposal-id $proposal_id \
       --ws wss://mainnet.etrid.network:9944
   done
   ```
3. [ ] Verify treasury received minted funds
4. [ ] Advance to Distribution phase

#### 7.4.4 Distribution Failure
1. [ ] Detect: Distribution phase completes but rewards not paid
2. [ ] Calculate owed participation rewards:
   ```
   Per voter = (Minted amount × 1%) / Voter count
   Validator bonus = (Minted amount × 0.5%) / 21
   Director stipend = (Minted amount × 0.2%) / 9
   ```
3. [ ] Execute manual distribution via treasury (6/9):
   ```bash
   etrid-cli tx treasury propose-disbursement \
     --category Operations \
     --recipient <voter-account> \
     --amount <owed-reward> \
     --description "Manual Consensus Day participation reward"
   ```

#### 7.4.5 Worst Case: Complete Failure
If Consensus Day completely fails:
1. [ ] Document all votes and proposals from chain state
2. [ ] Execute approved budgets manually via treasury
3. [ ] Distribute participation rewards manually
4. [ ] Post-mortem analysis
5. [ ] Fix issues before next Consensus Day (Dec 1, 2026)

**Reference**: `/Users/macbook/Desktop/etrid/EMERGENCY_FUND_RECOVERY_GUIDE.md` (Scenario 5)

---

### 7.5 Runtime Rollback Procedure

**Severity**: CRITICAL (Level 1)
**Trigger**: Catastrophic runtime failure, chain halt >10 minutes
**Required**: All 21 validators + 7/9 directors

#### 7.5.1 Decision Criteria
Rollback if:
- Chain halted for >10 minutes with no recovery
- >33% of validators cannot produce blocks
- Critical runtime panic discovered
- Data corruption detected

#### 7.5.2 Rollback Execution
1. [ ] **STOP ALL VALIDATORS** (coordinate via emergency call)
   ```bash
   # On each validator
   sudo systemctl stop flarechain-validator
   ```
2. [ ] Restore previous runtime (v104)
   ```bash
   etrid-cli sudo set-code \
     --wasm /Users/macbook/Desktop/etrid/backups/flarechain_runtime_v104.wasm \
     --ws wss://mainnet.etrid.network:9944
   ```
3. [ ] Restart all validators with previous runtime
4. [ ] Monitor chain restart
5. [ ] Verify consensus restored
6. [ ] Announce rollback to community

#### 7.5.3 Post-Rollback
1. [ ] Identify root cause of failure
2. [ ] Fix issue in v105 code
3. [ ] Re-test on Ember testnet (minimum 7 days)
4. [ ] Re-deploy to mainnet when stable

---

## 8. CONTACT INFORMATION

### 8.1 Emergency Contacts

**Foundation Directors** (9 total, 6-of-9 required for treasury operations, 7-of-9 for emergencies):
- Director 1 (Lead): [Name] - [Email] - [Phone] - [Telegram]
- Director 2: [Name] - [Email] - [Phone] - [Telegram]
- Director 3: [Name] - [Email] - [Phone] - [Telegram]
- Director 4: [Name] - [Email] - [Phone] - [Telegram]
- Director 5: [Name] - [Email] - [Phone] - [Telegram]
- Director 6: [Name] - [Email] - [Phone] - [Telegram]
- Director 7: [Name] - [Email] - [Phone] - [Telegram]
- Director 8: [Name] - [Email] - [Phone] - [Telegram]
- Director 9: [Name] - [Email] - [Phone] - [Telegram]

**Technical Team**:
- Core Developer: [Name] - [Email] - [Phone]
- Security Lead: [Name] - [Email] - [Phone]
- DevOps Lead: [Name] - [Email] - [Phone]

**External Support**:
- Security Auditor: [Company] - [Email] - [Phone]
- Legal Counsel: [Name/Firm] - [Email] - [Phone]

### 8.2 Communication Channels

**Internal**:
- Emergency Conference Line: [Phone number]
- Secure Chat: [Signal/Telegram group]
- Email: foundation@etrid.network

**Public**:
- Status Page: https://status.etrid.network
- Twitter: @EtridProtocol
- Discord: #announcements, #validator-support
- Telegram: https://t.me/etrid_official
- Blog: https://blog.etrid.network

### 8.3 Critical Repository Locations

- Main Repo: https://github.com/etrid/etrid
- Local: `/Users/macbook/Desktop/etrid`
- Deployment Package: `/Users/macbook/Desktop/etrid/release-packages/`
- Backups: `/Users/macbook/Desktop/etrid/backups/`
- Documentation: `/Users/macbook/Desktop/etrid/docs/`

### 8.4 Key File Locations

- Genesis Config: `/Users/macbook/Desktop/etrid/flarechain_mainnet_genesis.json`
- Validator Keys: `/Users/macbook/Desktop/etrid/validator-keys-setup/generated-keys/`
- Runtime WASM: `/Users/macbook/Desktop/etrid/target/release/wbuild/flarechain-runtime/`
- Emergency Scripts: `/Users/macbook/Desktop/etrid/scripts/emergency/`
- Monitoring Configs: `/Users/macbook/Desktop/etrid/infrastructure/monitoring/`

---

## APPENDIX A: QUICK REFERENCE COMMANDS

### Treasury Operations
```bash
# Query treasury balance
etrid-cli query system account $(etrid-cli pallet-id py/trsry) --ws wss://mainnet.etrid.network:9944

# Query treasury directors
etrid-cli query treasury directors --ws wss://mainnet.etrid.network:9944

# Propose disbursement
etrid-cli tx treasury propose-disbursement \
  --category Development \
  --recipient <account> \
  --amount <amount> \
  --description "Description" \
  --ws wss://mainnet.etrid.network:9944
```

### Validator Rewards
```bash
# Register payment account
etrid-cli tx validator-rewards register-payment-account \
  --payment-account <cold-storage> \
  --ws wss://mainnet.etrid.network:9944

# Query pending rewards
etrid-cli query validator-rewards pending-rewards <payment-account> \
  --ws wss://mainnet.etrid.network:9944

# Claim rewards
etrid-cli tx validator-rewards claim-rewards \
  --ws wss://mainnet.etrid.network:9944 \
  --account <payment-account>
```

### Consensus Day
```bash
# Query next consensus day
etrid-cli query consensus-day next-consensus-day --ws wss://mainnet.etrid.network:9944

# Submit proposal
etrid-cli tx consensus-day submit-proposal \
  --category BudgetAllocation \
  --description "Proposal details" \
  --bond 10000000000000000000000 \
  --ws wss://mainnet.etrid.network:9944
```

### EDSC Stability
```bash
# Query EDSC price
etrid-cli query edsc-stability current-price --ws wss://mainnet.etrid.network:9944

# Open position
etrid-cli tx edsc-stability open-position \
  --collateral-asset ETR \
  --collateral-amount <amount> \
  --edsc-amount <amount> \
  --ws wss://mainnet.etrid.network:9944
```

### Circuit Breaker
```bash
# Query status
etrid-cli query circuit-breaker status --ws wss://mainnet.etrid.network:9944

# Pause (emergency, requires 7/9)
etrid-cli tx circuit-breaker pause --target <pallet-name> --ws wss://mainnet.etrid.network:9944
```

---

## APPENDIX B: SUCCESS METRICS

### Deployment Success
- [ ] Runtime upgrade completed without chain halt
- [ ] All 21 validators remain online
- [ ] Block production continuous (6 sec average)
- [ ] No finalization disruptions
- [ ] Migration completed successfully
- [ ] All pallets operational

### 48-Hour Success
- [ ] 20-21 validators registered payment accounts
- [ ] First epoch rewards calculated correctly
- [ ] Treasury receiving funds from all sources
- [ ] EDSC peg stable (±1% deviation)
- [ ] No critical incidents

### 30-Day Success
- [ ] Validator rewards distributed for 7+ epochs
- [ ] Treasury balance growing at expected rate
- [ ] EDSC peg maintained (>95% time in ±1% range)
- [ ] No emergency procedures required
- [ ] Positive validator feedback
- [ ] Community confidence high

---

## DOCUMENT HISTORY

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-31 | Eoj & Claude | Initial comprehensive checklist created |

---

**END OF MAINNET DEPLOYMENT COMPLETE CHECKLIST**

This checklist is a living document and should be updated based on:
- Lessons learned from Ember testnet deployment
- Feedback from validators and directors
- Changes to runtime or economic model
- Results from emergency drills
- Post-incident reviews

**Next Review Date**: [Date of Ember testnet deployment + 7 days]

**Status**: READY FOR REVIEW BY FOUNDATION DIRECTORS
