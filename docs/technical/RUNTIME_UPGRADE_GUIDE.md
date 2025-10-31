# Ëtrid Runtime Upgrade Guide - Validator Payment System

This guide covers the hot upgrade process for deploying the new validator payment system (pallet-validator-rewards) to the Ëtrid mainnet without downtime.

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Pre-Upgrade Preparation](#pre-upgrade-preparation)
3. [Building the New Runtime](#building-the-new-runtime)
4. [Testing the Migration](#testing-the-migration)
5. [Submitting the Upgrade Transaction](#submitting-the-upgrade-transaction)
6. [Post-Upgrade Validator Actions](#post-upgrade-validator-actions)
7. [Verification Checklist](#verification-checklist)
8. [Rollback Procedure](#rollback-procedure)

---

## 🎯 Overview

### What's Changing

This runtime upgrade introduces **pallet-validator-rewards**, which separates validator session accounts (hot keys) from payment accounts (cold storage):

- **Session Account**: Hot keys used for consensus (AURA, GRANDPA, ASF)
- **Payment Account**: Cold storage that receives rewards
- **Controller Account**: Optional management account

### Migration Process

The migration (v1) will:
1. Read existing validators from pallet-validator-committee
2. Create default payment account mappings (session → session)
3. Initialize validator stakes from existing committee data
4. Set initial performance metrics (zeroed state)

**Important**: Validators can update their payment accounts after the upgrade completes.

---

## 🔧 Pre-Upgrade Preparation

### 1. Backup Current Chain State

```bash
# Stop your validator node (on testnet first!)
systemctl stop etrid-validator

# Backup chain database
tar -czf etrid-chain-backup-$(date +%Y%m%d).tar.gz \
  /var/lib/etrid/chains/flarechain/db

# Backup keystore
tar -czf etrid-keystore-backup-$(date +%Y%m%d).tar.gz \
  /var/lib/etrid/chains/flarechain/keystore

# Restart node
systemctl start etrid-validator
```

### 2. Verify Current Validator Set

Use the Runtime API to verify current validators:

```bash
# Get current committee
etrid-cli runtime call \
  --method validator_committee \
  --ws wss://mainnet.etrid.network:9944

# Verify your validator is in the set
etrid-cli runtime call \
  --method is_validator_active \
  --params '["<your-validator-id>"]' \
  --ws wss://mainnet.etrid.network:9944
```

### 3. Prepare Payment Accounts

**IMPORTANT**: Before the upgrade, prepare cold storage accounts for receiving rewards:

```bash
# Generate a new payment account (cold storage)
etrid-cli account generate --scheme Sr25519

# Example output:
# Secret phrase: word1 word2 ... word12
# Public key: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# SS58 Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

**Security Best Practices**:
- Store the seed phrase in a hardware wallet or secure vault
- NEVER expose this account's private key on a hot validator server
- Use a separate machine to generate the address

---

## 🏗️ Building the New Runtime

### 1. Clone and Update Ëtrid Repository

```bash
# Clone repository
git clone https://github.com/etrid/etrid.git
cd etrid

# Checkout the runtime upgrade branch
git checkout runtime-upgrade-validator-payments

# Or if deploying from main
git pull origin main
```

### 2. Add Migration to Runtime

Edit `/05-multichain/flare-chain/runtime/src/lib.rs`:

```rust
// Add to imports at top of file
use pallet_validator_rewards::migrations;

// Update runtime version (increment spec_version)
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 104,  // INCREMENT THIS (was 103)
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

// Add ValidatorRewards pallet to runtime
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        // ... existing pallets ...
        ValidatorCommittee: pallet_validator_committee,
        ValidatorRewards: pallet_validator_rewards,  // ADD THIS LINE
        // ... rest of pallets ...
    }
);

// Configure the pallet (add before construct_runtime!)
parameter_types! {
    pub const ValidatorEpochDuration: u32 = 2400; // 2400 blocks (~4 hours at 6s)
    pub const ValidatorAnnualRewardPoolBps: u32 = 300; // 3% annual inflation
    pub const ValidatorShareBps: u32 = 5000; // 50% to validators, 50% to delegators
}

impl pallet_validator_rewards::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type EpochDuration = ValidatorEpochDuration;
    type AnnualRewardPoolBps = ValidatorAnnualRewardPoolBps;
    type ValidatorShareBps = ValidatorShareBps;
}

// Add to Executive type (for runtime upgrades)
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::Migrations<Runtime>,  // ADD THIS MIGRATION
>;
```

### 3. Build the Runtime WASM

```bash
# Install dependencies (if not already installed)
rustup default nightly
rustup target add wasm32-unknown-unknown

# Build the runtime
cd 05-multichain/flare-chain/runtime
cargo build --release --features=runtime-benchmarks

# Verify WASM binary was created
ls -lh target/release/wbuild/etrid-runtime/etrid_runtime.wasm
```

Expected output:
```
-rw-r--r-- 1 user user 1.2M Nov 1 12:00 etrid_runtime.wasm
```

### 4. Extract Runtime WASM Hash

```bash
# Get the WASM hash (this will be used in upgrade transaction)
subwasm info target/release/wbuild/etrid-runtime/etrid_runtime.wasm

# Output will show:
# Blake2-256: 0x1234567890abcdef...
# Proposal hash: 0x1234567890abcdef...
```

---

## 🧪 Testing the Migration

### 1. Test with Try-Runtime (Recommended)

```bash
# Install try-runtime CLI
cargo install --git https://github.com/paritytech/try-runtime-cli

# Run migration against mainnet state (dry-run)
try-runtime \
  --runtime target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
  on-runtime-upgrade \
  --uri wss://mainnet.etrid.network:9944 \
  live
```

**Expected output:**
```
✅ Pre-upgrade checks passed
🔄 Running migration...
📊 Current committee size: 21
✅ Migrated 21 validators
💰 Total staked: 1344000000000000000000 (1.344M ETR)
✅ Post-upgrade verification passed
```

### 2. Test on Local Testnet

```bash
# Start a local dev chain with the new runtime
./target/release/etrid --dev --tmp \
  --runtime target/release/wbuild/etrid-runtime/etrid_runtime.wasm

# In another terminal, test the migration
etrid-cli runtime upgrade \
  --wasm target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
  --ws ws://localhost:9944 \
  --sudo
```

### 3. Test on Ember Testnet (Public Testnet)

Before mainnet deployment, test on Ember testnet:

```bash
# Submit to Ember testnet
etrid-cli runtime upgrade \
  --wasm target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
  --ws wss://ember.etrid.network:9944 \
  --sudo \
  --account <sudo-account>
```

Monitor logs for migration output.

---

## 🚀 Submitting the Upgrade Transaction

### Method 1: Via Polkadot.js UI (Recommended for Governance)

1. **Upload Runtime WASM**:
   - Navigate to https://polkadot.js.org/apps/?rpc=wss://mainnet.etrid.network:9944
   - Go to **Developer** → **Extrinsics**
   - Select `system` → `setCode(code)`
   - Upload `etrid_runtime.wasm`

2. **Create Governance Proposal**:
   - Go to **Governance** → **Democracy** → **Submit Proposal**
   - Paste the encoded call data from step 1
   - Set preimage hash and bond (10 ETR minimum)

3. **Vote and Execute**:
   - Once voting period completes and proposal passes
   - Execute via **Democracy** → **Execute**

### Method 2: Via Sudo (If Sudo Still Enabled)

**WARNING**: Only use if sudo is still enabled. Should transition to governance ASAP.

```bash
# Submit via sudo
etrid-cli sudo set-code \
  --wasm target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
  --ws wss://mainnet.etrid.network:9944 \
  --account <sudo-account> \
  --password-file sudo-password.txt
```

### Method 3: Via Multisig (Foundation Treasury)

```bash
# Create multisig proposal
etrid-cli multisig as-multi \
  --threshold 3 \
  --other-signatories <account1,account2,account3,account4> \
  --call "0x00..." \  # Encoded setCode call
  --ws wss://mainnet.etrid.network:9944 \
  --account <signer-account>

# Other signers approve
etrid-cli multisig as-multi \
  --threshold 3 \
  --other-signatories <account1,account2,account3,account4> \
  --call "0x00..." \
  --ws wss://mainnet.etrid.network:9944 \
  --account <signer-account-2>

# Final signer executes
etrid-cli multisig as-multi-final \
  --threshold 3 \
  --other-signatories <account1,account2,account3,account4> \
  --call "0x00..." \
  --ws wss://mainnet.etrid.network:9944 \
  --account <final-signer>
```

---

## 👨‍💼 Post-Upgrade Validator Actions

### Step 1: Verify Migration Completed

```bash
# Check your validator's payment account (should default to session account)
etrid-cli runtime call \
  --method payment_account_of \
  --params '["<your-session-account>"]' \
  --ws wss://mainnet.etrid.network:9944

# Check your validator stake
etrid-cli runtime call \
  --method validator_stake \
  --params '["<your-session-account>"]' \
  --ws wss://mainnet.etrid.network:9944

# Check performance metrics (should be initialized)
etrid-cli runtime call \
  --method performance_of \
  --params '["<your-session-account>"]' \
  --ws wss://mainnet.etrid.network:9944
```

### Step 2: Register Payment Account (CRITICAL)

**Within 48 hours of upgrade**, update your payment account to cold storage:

```bash
# Register your cold storage payment account
etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account <your-cold-storage-address> \
  --account <your-session-account> \
  --ws wss://mainnet.etrid.network:9944

# Verify payment account updated
etrid-cli runtime call \
  --method payment_account_of \
  --params '["<your-session-account>"]' \
  --ws wss://mainnet.etrid.network:9944
```

**Example**:
```bash
# Session account (hot): 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
# Payment account (cold): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --account 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV \
  --ws wss://mainnet.etrid.network:9944
```

### Step 3: (Optional) Register Controller Account

For advanced key management, register a controller account:

```bash
etrid-cli extrinsic validator-rewards register-controller-account \
  --controller-account <your-controller-address> \
  --account <your-session-account> \
  --ws wss://mainnet.etrid.network:9944
```

### Step 4: Monitor Rewards Accumulation

After the first epoch completes (~4 hours), check pending rewards:

```bash
# Check pending rewards for your payment account
etrid-cli runtime call \
  --method pending_rewards \
  --params '["<your-payment-account>"]' \
  --ws wss://mainnet.etrid.network:9944
```

### Step 5: Claim Rewards (When Ready)

```bash
# Claim accumulated rewards (from payment account)
etrid-cli extrinsic validator-rewards claim-rewards \
  --account <your-payment-account> \
  --ws wss://mainnet.etrid.network:9944
```

---

## ✅ Verification Checklist

### Immediately After Upgrade

- [ ] Runtime version incremented to 104
- [ ] No runtime errors in validator logs
- [ ] Block production continues normally
- [ ] All 21 validators still active in committee
- [ ] Migration logs show successful completion

### Within 1 Hour

- [ ] All validators have payment accounts initialized (check via runtime API)
- [ ] All validators have stake initialized correctly
- [ ] Total staked matches expected amount (1.344M ETR for 21 validators)
- [ ] Performance metrics initialized for all validators

### Within 48 Hours

- [ ] All validators registered their cold storage payment accounts
- [ ] No validators using default session → session payment mapping
- [ ] First epoch rewards distribution completed successfully
- [ ] Validators can claim rewards from their payment accounts

### Monitoring Commands

```bash
# Monitor runtime events
etrid-cli events subscribe \
  --ws wss://mainnet.etrid.network:9944

# Check validator committee size
etrid-cli runtime call \
  --method committee_size_limit \
  --ws wss://mainnet.etrid.network:9944

# Check current epoch
etrid-cli runtime call \
  --method current_epoch \
  --ws wss://mainnet.etrid.network:9944

# Monitor total staked
etrid-cli runtime call \
  --method total_staked \
  --ws wss://mainnet.etrid.network:9944
```

---

## 🔄 Rollback Procedure

If critical issues occur, rollback to previous runtime:

### Emergency Rollback Steps

1. **Stop All Validator Nodes**:
   ```bash
   systemctl stop etrid-validator
   ```

2. **Restore Previous Chain State**:
   ```bash
   # Remove current database
   rm -rf /var/lib/etrid/chains/flarechain/db

   # Restore backup
   tar -xzf etrid-chain-backup-<date>.tar.gz -C /var/lib/etrid/chains/flarechain/
   ```

3. **Restart with Previous Runtime**:
   ```bash
   # Use previous runtime WASM
   ./target/release/etrid \
     --validator \
     --runtime /path/to/previous-runtime.wasm \
     --chain mainnet
   ```

4. **Submit Rollback Transaction** (if needed):
   ```bash
   etrid-cli sudo set-code \
     --wasm /path/to/previous-runtime.wasm \
     --ws wss://mainnet.etrid.network:9944 \
     --account <sudo-account>
   ```

### Rollback Decision Criteria

Rollback if:
- More than 33% of validators cannot produce blocks
- Critical runtime panic occurs
- Migration fails validation checks
- Chain halts for more than 10 minutes

---

## 📞 Support and Resources

### Validator Support

- **Discord**: https://discord.gg/etrid (validator-support channel)
- **Telegram**: https://t.me/etrid_validators
- **Email**: validators@etrid.network

### Documentation

- **Ëtrid Docs**: https://docs.etrid.network
- **Substrate Docs**: https://docs.substrate.io/reference/how-to-guides/parachains/runtime-upgrades/
- **Runtime Upgrade Guide**: https://docs.substrate.io/maintain/runtime-upgrades/

### Emergency Contacts

- **Lead Validator Coordinator**: @etridcore on Discord
- **Foundation Treasury**: foundation@etrid.network
- **Security Issues**: security@etrid.network

---

## 📝 Notes

### Timeline

- **Testnet Deployment**: T-7 days
- **Governance Proposal Submission**: T-5 days
- **Voting Period**: T-5 to T-2 days
- **Mainnet Upgrade**: T-0 (after vote passes)
- **Validator Registration Period**: T+0 to T+48 hours

### Key Dates

- **Ember Testnet Upgrade**: November 1, 2025, 00:00 UTC
- **Mainnet Governance Proposal**: November 3, 2025, 00:00 UTC
- **Mainnet Voting Period**: November 3-6, 2025
- **Mainnet Upgrade**: November 7, 2025, 00:00 UTC (if approved)

---

**Last Updated**: November 1, 2025
**Runtime Version**: 103 → 104
**Migration Version**: v1
**Status**: Ready for Testnet Deployment
