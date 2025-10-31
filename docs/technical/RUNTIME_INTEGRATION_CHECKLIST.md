# Runtime Integration Checklist - Validator Payment System

This checklist ensures the validator payment system migration is properly integrated into the Ëtrid runtime.

## ✅ Pre-Integration Checklist

### Pallet Files
- [x] `/src/pallets/pallet-validator-rewards/src/lib.rs` - Main pallet implementation
- [x] `/src/pallets/pallet-validator-rewards/src/migrations.rs` - Storage migrations
- [ ] `/src/pallets/pallet-validator-rewards/Cargo.toml` - Dependencies configured

### Runtime Files to Modify
- [ ] `/05-multichain/flare-chain/runtime/Cargo.toml` - Add pallet dependency
- [ ] `/05-multichain/flare-chain/runtime/src/lib.rs` - Configure pallet
- [ ] `/05-multichain/flare-chain/runtime/build.rs` - Build script (if needed)

---

## 📝 Step-by-Step Integration

### Step 1: Add Pallet Dependency to Runtime Cargo.toml

**File**: `/05-multichain/flare-chain/runtime/Cargo.toml`

Add to `[dependencies]`:
```toml
pallet-validator-rewards = { path = "../../../src/pallets/pallet-validator-rewards", default-features = false }
```

Add to `[features]`:
```toml
std = [
    # ... existing entries ...
    "pallet-validator-rewards/std",
]
```

**Verification**:
```bash
cd 05-multichain/flare-chain/runtime
cargo check --features runtime-benchmarks
```

---

### Step 2: Update Runtime Version

**File**: `/05-multichain/flare-chain/runtime/src/lib.rs`

**Line ~75**:
```rust
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 104,  // INCREMENT FROM 103 → 104
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};
```

---

### Step 3: Add Pallet Configuration

**File**: `/05-multichain/flare-chain/runtime/src/lib.rs`

**Insert after line ~315** (after `pallet_consensus::Config`):
```rust
// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATOR REWARDS CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

parameter_types! {
    /// Epoch duration: 2400 blocks (~4 hours at 6s blocks)
    pub const ValidatorEpochDuration: u32 = 2400;
    
    /// Annual validator reward pool: 3% of total supply per year
    pub const ValidatorAnnualRewardPoolBps: u32 = 300; // 300 basis points = 3%
    
    /// Validator/Delegator split: 50/50 (as per Ivory Papers)
    pub const ValidatorShareBps: u32 = 5000; // 5000 basis points = 50%
}

impl pallet_validator_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EpochDuration = ValidatorEpochDuration;
    type AnnualRewardPoolBps = ValidatorAnnualRewardPoolBps;
    type ValidatorShareBps = ValidatorShareBps;
}
```

---

### Step 4: Add Pallet to Runtime

**File**: `/05-multichain/flare-chain/runtime/src/lib.rs`

**Modify `construct_runtime!` macro** (around line ~818):
```rust
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        // ... existing pallets ...
        
        // ASF Consensus pallets
        ValidatorCommittee: pallet_validator_committee,
        ValidatorRewards: pallet_validator_rewards,  // ADD THIS LINE
        
        // ... rest of pallets ...
    }
);
```

---

### Step 5: Add Migration to Executive

**File**: `/05-multichain/flare-chain/runtime/src/lib.rs`

**Find the `Executive` type definition** (around line ~916):
```rust
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    pallet_validator_rewards::migrations::Migrations<Runtime>,  // ADD THIS LINE
>;
```

**If there are already migrations**, use a tuple:
```rust
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    (
        ExistingMigration,
        pallet_validator_rewards::migrations::Migrations<Runtime>,
    ),
>;
```

---

### Step 6: Add Runtime API (Optional, for Query Support)

**File**: `/05-multichain/flare-chain/runtime/src/lib.rs`

**Add to `impl_runtime_apis!` block** (after line ~1130):
```rust
impl_runtime_apis! {
    // ... existing implementations ...
    
    // Validator Rewards Runtime API
    impl pallet_validator_rewards_runtime_api::ValidatorRewardsApi<Block, AccountId, Balance> for Runtime {
        fn payment_account_of(session_account: AccountId) -> Option<AccountId> {
            ValidatorRewards::payment_account_of(&session_account)
        }
        
        fn pending_rewards(payment_account: AccountId) -> Balance {
            ValidatorRewards::pending_rewards(&payment_account)
        }
        
        fn validator_stake(session_account: AccountId) -> Balance {
            ValidatorRewards::validator_stake(&session_account)
        }
        
        fn performance_of(session_account: AccountId) -> pallet_validator_rewards::PerformanceMetrics {
            ValidatorRewards::performance_of(&session_account)
        }
    }
}
```

**Note**: This requires creating a runtime-api crate for the pallet (optional for now).

---

## 🧪 Build and Test

### Build Runtime WASM

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime

# Clean build
cargo clean

# Build with runtime benchmarks
cargo build --release --features runtime-benchmarks

# Verify WASM binary
ls -lh target/release/wbuild/etrid-runtime/etrid_runtime.wasm
```

**Expected output**:
```
-rw-r--r--  1 user  staff   1.2M Nov  1 10:00 etrid_runtime.wasm
```

### Run Migration Tests

```bash
# Test with try-runtime (requires try-runtime CLI)
cargo install --git https://github.com/paritytech/try-runtime-cli

# Dry-run migration against testnet
try-runtime \
  --runtime target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
  on-runtime-upgrade \
  --uri wss://ember.etrid.network:9944 \
  live
```

### Verify Metadata

```bash
# Check runtime metadata includes new pallet
subwasm metadata target/release/wbuild/etrid-runtime/etrid_runtime.wasm | \
  jq '.pallets[] | select(.name == "ValidatorRewards")'
```

---

## 🔍 Pre-Deployment Verification

### Code Review Checklist

- [ ] Runtime version incremented (103 → 104)
- [ ] Pallet properly configured with correct parameters
- [ ] Migration added to Executive type
- [ ] No compilation errors or warnings
- [ ] WASM binary builds successfully
- [ ] try-runtime test passes on testnet
- [ ] Metadata includes ValidatorRewards pallet

### Configuration Verification

**Verify pallet parameters**:
```rust
// Should match Ivory Papers specifications
EpochDuration: 2400 blocks (~4 hours)
AnnualRewardPoolBps: 300 (3% annual inflation)
ValidatorShareBps: 5000 (50% to validators, 50% to delegators)
```

**Verify migration is included**:
```rust
// Check Executive type has migration
pub type Executive = frame_executive::Executive<
    ...,
    pallet_validator_rewards::migrations::Migrations<Runtime>,
>;
```

### Testnet Deployment Checklist

- [ ] Deploy to local dev chain first
- [ ] Deploy to Ember testnet
- [ ] Verify migration completes successfully
- [ ] Test validator payment account registration
- [ ] Test epoch rewards distribution
- [ ] Test reward claiming
- [ ] Monitor for 24+ hours on testnet

---

## 📋 Mainnet Deployment Checklist

### Pre-Deployment (T-7 days)

- [ ] Testnet deployment successful for 7+ days
- [ ] All validators tested payment account registration
- [ ] Epoch rewards distribution working correctly
- [ ] No critical issues reported
- [ ] Runtime WASM hash calculated and verified
- [ ] Governance proposal prepared

### Governance Proposal (T-5 days)

- [ ] Submit `system.setCode()` proposal with new runtime WASM
- [ ] Announce to validators on Discord/Telegram
- [ ] Post upgrade guide to governance forum
- [ ] Set preimage hash and bond

### Voting Period (T-5 to T-2 days)

- [ ] Monitor voting progress
- [ ] Answer validator questions
- [ ] Prepare for deployment day

### Deployment Day (T-0)

- [ ] Governance proposal passed
- [ ] Execute upgrade transaction
- [ ] Monitor chain for runtime upgrade event
- [ ] Verify migration logs in validator nodes
- [ ] Verify all 21 validators migrated successfully
- [ ] Announce upgrade completion to validators

### Post-Deployment (T+0 to T+48 hours)

- [ ] Monitor validator payment account registrations
- [ ] Verify epoch rewards distribution works
- [ ] Track validator claim transactions
- [ ] Respond to validator support requests
- [ ] Document any issues encountered

---

## 🚨 Rollback Triggers

Initiate rollback if:

- [ ] More than 33% of validators cannot produce blocks
- [ ] Critical runtime panic occurs
- [ ] Migration fails validation checks (pre/post upgrade)
- [ ] Chain halts for more than 10 minutes
- [ ] Data corruption detected in storage

---

## 📞 Key Contacts

**Core Team**:
- Lead Developer: @etridcore
- Runtime Engineer: @runtime-team
- Validator Coordinator: @validator-ops

**Support Channels**:
- Discord: #validator-support
- Telegram: @etrid_validators
- Email: validators@etrid.network

---

## ✅ Final Sign-Off

Before mainnet deployment, confirm:

- [ ] Code reviewed by at least 2 core developers
- [ ] Testnet deployment successful for 7+ days
- [ ] try-runtime test passed on mainnet state fork
- [ ] Governance proposal approved by community
- [ ] All validators notified and prepared
- [ ] Rollback procedure documented and tested
- [ ] Support team on standby for deployment

**Signed off by**:

- [ ] Lead Developer: _________________ Date: _______
- [ ] Runtime Engineer: _______________ Date: _______
- [ ] Validator Coordinator: __________ Date: _______

---

**Last Updated**: October 31, 2025
**Runtime Version**: 103 → 104
**Migration**: v1
**Status**: Integration Ready
