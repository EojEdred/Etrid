# Validator Payment System Runtime Upgrade - Quick Reference

## 🎯 What Was Created

### 1. Migration File
**Location**: `/Users/macbook/Desktop/etrid/src/pallets/pallet-validator-rewards/src/migrations.rs`

**Purpose**: Handles hot upgrade from current system to new validator payment system

**Key Features**:
- ✅ Reads validators from pallet-validator-committee
- ✅ Initializes default payment accounts (session → session)
- ✅ Migrates validator stakes
- ✅ Sets up initial performance metrics
- ✅ Includes pre/post-upgrade verification (try-runtime)
- ✅ Idempotent and safe to run multiple times

**Migration Logic**:
```rust
pub mod v1 {
    pub struct MigrateToV1<T>;
    
    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            // 1. Get all validators from committee
            // 2. Initialize payment account (default: session account)
            // 3. Initialize validator stakes
            // 4. Initialize performance metrics
            // 5. Set total staked amount
            // Returns weight based on storage operations
        }
    }
}
```

### 2. Runtime Upgrade Guide
**Location**: `/Users/macbook/Desktop/etrid/RUNTIME_UPGRADE_GUIDE.md`

**Sections**:
1. **Overview** - What's changing and why
2. **Pre-Upgrade Preparation** - Backup, verification, payment account setup
3. **Building the New Runtime** - Step-by-step build instructions
4. **Testing the Migration** - try-runtime, testnet deployment
5. **Submitting the Upgrade** - Governance, sudo, or multisig methods
6. **Post-Upgrade Validator Actions** - Registration of payment accounts
7. **Verification Checklist** - What to check and when
8. **Rollback Procedure** - Emergency rollback steps

---

## 🚀 Deployment Steps (TL;DR)

### For Core Developers

1. **Update Runtime Configuration**:
   ```rust
   // In runtime/src/lib.rs
   
   // 1. Increment spec_version
   spec_version: 104,  // was 103
   
   // 2. Add ValidatorRewards pallet
   impl pallet_validator_rewards::Config for Runtime {
       type RuntimeEvent = RuntimeEvent;
       type Currency = Balances;
       type EpochDuration = ConstU32<2400>;
       type AnnualRewardPoolBps = ConstU32<300>;
       type ValidatorShareBps = ConstU32<5000>;
   }
   
   // 3. Add to construct_runtime!
   ValidatorRewards: pallet_validator_rewards,
   
   // 4. Add migration to Executive
   pub type Executive = frame_executive::Executive<
       Runtime, Block, ..., 
       migrations::Migrations<Runtime>
   >;
   ```

2. **Build Runtime**:
   ```bash
   cd 05-multichain/flare-chain/runtime
   cargo build --release
   ```

3. **Test Migration**:
   ```bash
   try-runtime \
     --runtime target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
     on-runtime-upgrade \
     --uri wss://mainnet.etrid.network:9944 \
     live
   ```

4. **Deploy to Testnet First**:
   ```bash
   # Test on Ember testnet
   etrid-cli runtime upgrade \
     --wasm target/release/wbuild/etrid-runtime/etrid_runtime.wasm \
     --ws wss://ember.etrid.network:9944
   ```

5. **Submit to Mainnet** (via governance):
   - Create governance proposal with `system.setCode()`
   - Vote and execute when passed

### For Validators (After Upgrade)

1. **Verify Migration**:
   ```bash
   etrid-cli runtime call \
     --method payment_account_of \
     --params '["<your-session-account>"]'
   ```

2. **Register Payment Account** (within 48 hours):
   ```bash
   etrid-cli extrinsic validator-rewards register-payment-account \
     --payment-account <cold-storage-address> \
     --account <session-account>
   ```

3. **Monitor Rewards**:
   ```bash
   etrid-cli runtime call \
     --method pending_rewards \
     --params '["<payment-account>"]'
   ```

4. **Claim Rewards**:
   ```bash
   etrid-cli extrinsic validator-rewards claim-rewards \
     --account <payment-account>
   ```

---

## 📊 Migration Details

### Storage Migrations

| Storage Item | Source | Migration Action |
|--------------|--------|------------------|
| `PaymentAccounts` | N/A | Initialize: session → session (default) |
| `ValidatorStakes` | `pallet-validator-committee::Validators` | Copy stake values |
| `ValidatorPerformance` | N/A | Initialize with zeroed metrics |
| `TotalStaked` | Computed | Sum of all validator stakes |
| `CurrentEpoch` | N/A | Initialize to 0 |
| `EpochRewardPool` | N/A | Initialize to 0 |

### Expected Results (21 Validators, 1.344M ETR Total Stake)

```
✅ Pre-upgrade checks passed
📋 Found 21 validators in committee to migrate
🔧 Migrated validator with stake: 64000000000000000000000 (64 ETR)
🔧 Migrated validator with stake: 64000000000000000000000 (64 ETR)
... (21 total) ...
✅ Migration complete: 21 validators migrated, total stake: 1344000000000000000000
📊 Storage operations: 22 reads, 86 writes
✅ Post-upgrade verification passed
💰 Total staked: 1344000000000000000000 (1.344M ETR)
```

### Weight Calculation

```rust
Weight::from_parts(
    (reads * 25_000).saturating_add(writes * 100_000),
    0
)

// Example for 21 validators:
// Reads: 22 (1 committee + 21 validators)
// Writes: 86 (21 payment accounts + 21 stakes + 21 performance + 3 globals)
// Total weight: 22 * 25_000 + 86 * 100_000 = 9,150,000
```

---

## 🔐 Security Considerations

### Migration Safety Features

1. **Idempotent**: Can be run multiple times without breaking state
2. **Read-Only Source**: Doesn't modify pallet-validator-committee
3. **Pre-Upgrade Checks**: Verifies committee has validators
4. **Post-Upgrade Verification**: Confirms all validators migrated
5. **Weight Bounds**: Accurate weight calculation prevents block overflow

### Validator Security

1. **Payment Account Separation**:
   - Session keys remain hot (on validator server)
   - Payment accounts MUST be cold storage (hardware wallet)
   - Never expose payment account private keys

2. **Default Mapping Risk**:
   - Initial mapping is session → session (HOT WALLET)
   - Validators MUST update within 48 hours
   - Rewards accumulating in session account are at risk

3. **Recommended Setup**:
   ```
   Session Account (Hot):  5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
   Controller Account:     5EyQqx... (optional, hardware wallet)
   Payment Account (Cold): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   ```

---

## 📅 Timeline

### Testnet Phase (T-7 to T-0)
- **Day 1**: Deploy to Ember testnet
- **Day 2-3**: Validators test payment account registration
- **Day 4-5**: Monitor epoch rewards distribution
- **Day 6-7**: Final verification and mainnet prep

### Mainnet Phase (T-0 to T+48)
- **T-5 days**: Submit governance proposal
- **T-5 to T-2**: Voting period (3 days)
- **T-0**: Runtime upgrade execution
- **T+0 to T+48**: Validator payment account registration window

---

## 🛠️ Troubleshooting

### Common Issues

**Issue**: Migration shows 0 validators
```bash
# Check committee has validators
etrid-cli runtime call --method committee

# Verify pallet-validator-committee is working
etrid-cli runtime call --method is_validator_active \
  --params '["<validator-id>"]'
```

**Issue**: Payment account registration fails
```bash
# Ensure you're signing with session account
etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account <cold-address> \
  --account <session-account>  # MUST be session account

# Verify account has balance for transaction fees
etrid-cli account balance <session-account>
```

**Issue**: Can't claim rewards
```bash
# Check pending rewards exist
etrid-cli runtime call --method pending_rewards \
  --params '["<payment-account>"]'

# Ensure claiming from payment account (not session)
etrid-cli extrinsic validator-rewards claim-rewards \
  --account <payment-account>  # MUST be payment account
```

---

## 📞 Support

- **Documentation**: https://docs.etrid.network/validator-payment-system
- **Discord**: https://discord.gg/etrid #validator-support
- **Email**: validators@etrid.network
- **Emergency**: security@etrid.network

---

**Created**: November 1, 2025
**Runtime Version**: 103 → 104
**Migration**: v1
**Status**: Ready for Testnet Deployment
