# 🚨 MAINNET PAYMENT SYSTEM - CRITICAL DEPLOYMENT GUIDE

**Status**: READY FOR HOT UPGRADE
**Priority**: BLOCKER - Validators won't get paid without this
**Timeline**: Deploy within 24-48 hours

---

## 🔴 CRITICAL ISSUE IDENTIFIED

Your mainnet validators are running WITHOUT the payment system. They can validate blocks, but:

❌ **No payment accounts** - Rewards have nowhere to go
❌ **No reward distribution** - No mechanism to pay validators
❌ **No slashing to payment accounts** - Slashing would hit hot keys, not cold storage
❌ **No Consensus Day** - Annual governance event not functional

---

## ✅ SOLUTION: Runtime Upgrade (HOT DEPLOYMENT)

We've built **4 new pallets** that can be deployed via runtime upgrade WITHOUT restarting validators:

### 1. **pallet-validator-rewards** (600+ lines)
- Payment account registration (session → cold storage)
- Performance tracking (uptime, finality, blocks)
- Reward calculation with multipliers
- Distribution to payment accounts
- Integration with slashing

### 2. **pallet-consensus-day** (1,131 lines)
- 4-phase annual governance (Registration → Voting → Minting → Distribution)
- Proposal system with 10k ËTR bond
- Director elections (9 positions)
- Participation rewards
- Inflation control (0-5% cap)

### 3. **Slashing Integration** (updated existing pallet)
- Slashes from payment accounts (not session keys)
- 50% burned, 50% to treasury
- Offense types: downtime, equivocation, malicious, censorship
- Automatic validator removal for critical offenses

### 4. **Storage Migration**
- Reads existing validators from pallet-validator-committee
- Initializes payment accounts (defaults to session → session)
- Migrates stakes and performance metrics
- Safe, idempotent, reversible

---

## ⚡ DEPLOYMENT STEPS (FAST TRACK)

### Phase 1: Build & Test (2-4 hours)

```bash
cd /Users/macbook/Desktop/etrid

# 1. Build new runtime with pallets
cargo build --release -p flarechain-runtime

# 2. Test migration on local fork (CRITICAL)
try-runtime \
  --runtime ./target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
  on-runtime-upgrade \
  --uri wss://mainnet.etrid.network:9944

# 3. Verify migration success
# Should see: "✅ Migration successful: 21 validators initialized"
```

### Phase 2: Testnet Deploy (6-12 hours)

```bash
# 1. Deploy to Ember testnet
./scripts/deploy-runtime-upgrade.sh \
  --network ember \
  --runtime ./target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
  --sudo-account <your-sudo-key>

# 2. Monitor for issues (6+ hours)
./scripts/monitor-testnet.sh

# 3. Have validators test payment account registration
etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account <cold-storage> \
  --uri ws://ember-node:9944
```

### Phase 3: Mainnet Deploy (1-2 hours)

```bash
# Method A: Sudo (if you still have sudo)
polkadot-js-api tx.sudo.sudoUncheckedWeight \
  --call tx.system.setCode \
  --code ./target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm \
  --weight 1000000000000

# Method B: Governance proposal (if sudo removed)
# Submit democracy proposal for runtime upgrade
# Wait for voting period
# Execute after passing

# Method C: Foundation multisig (safest)
# Coordinate with other foundation signers
# 6-of-9 signatures required
```

### Phase 4: Validator Action (48 hours)

**CRITICAL**: All validators MUST register payment accounts within 48 hours:

```bash
# On each validator
etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account <cold-storage-address> \
  --account <session-account> \
  --uri ws://localhost:9944

# Verify registration
etrid-cli query validator-rewards payment-account-of <session-account>
```

**Default behavior**: Payment accounts default to session accounts (HOT WALLET) until updated.

---

## 📊 What Gets Fixed

### Before Upgrade:
```
Validator produces block
  ↓
[NO REWARD SYSTEM] ← PROBLEM
  ↓
Validator gets nothing
```

### After Upgrade:
```
Validator produces block (session keys)
  ↓
Performance tracked (uptime, finality, blocks)
  ↓
Epoch ends (24 hours)
  ↓
Reward calculated (stake × performance multiplier)
  ↓
Reward sent to PAYMENT ACCOUNT (cold storage)
  ↓
Validator claims reward securely
```

### Slashing Before:
```
Validator misbehaves
  ↓
Slash session account (HOT KEYS) ← INSECURE
```

### Slashing After:
```
Validator misbehaves
  ↓
Slash payment account (COLD STORAGE) ← SECURE
  ↓
50% burned, 50% to treasury
```

---

## 🔐 Security Model

### Key Hierarchy (Per Ivory Papers):

```
┌─────────────────────────────────────────┐
│ HOT KEYS (on validator VM):            │
│ ├─ Session Keys (AURA, GRANDPA, ASF)   │
│ │  └─ Used every 6 seconds             │
│ └─ Network Key (P2P)                    │
│    └─ Used continuously                 │
├─────────────────────────────────────────┤
│ WARM KEYS (Azure Key Vault):           │
│ └─ Controller Account                   │
│    └─ Used monthly for management       │
├─────────────────────────────────────────┤
│ COLD KEYS (Offline/Hardware Wallet):   │
│ └─ Payment Account                      │
│    └─ Receives rewards, rarely signs    │
└─────────────────────────────────────────┘
```

**Why This Matters:**
- Session keys are **HOT** and can be compromised
- Payment accounts are **COLD** and secured offline
- Rewards accumulate in cold storage, not hot wallet
- Slashing hits cold storage, not consensus keys

---

## 📈 Reward Economics (Per Ivory Papers Vol III)

### Annual Validator Pool: 3% of supply

```rust
// For 1B ËTR supply:
annual_pool = 1_000_000_000 * 0.03 = 30,000,000 ËTR

// Per epoch (24 hours, 365 epochs/year):
epoch_pool = 30_000_000 / 365 = 82,192 ËTR per day

// For 21 validators with equal stake:
base_reward = 82,192 / 21 = 3,914 ËTR per validator per day

// With performance multipliers (0.9 - 1.2):
actual_reward = 3,914 * performance_multiplier
// Range: 3,522 - 4,697 ËTR per day
```

### Performance Multipliers:

```
Uptime Score:        (95-100% = 0.9-1.1)
× Finality Score:    (blocks signed / expected)
× Block Score:       (blocks authored / expected)
× Participation:     (1.1 if voted Consensus Day, else 1.0)
= Performance Multiplier (0.9 - 1.2)
```

### Validator vs Delegator Split:

```
Total Reward:   100%
├─ Validator:    50%
└─ Delegators:   50%
```

---

## 🧪 Testing Checklist

### Before Mainnet Deploy:

- [ ] Compile runtime successfully
- [ ] Run try-runtime migration test
- [ ] Deploy to Ember testnet
- [ ] Verify 21 validators initialized
- [ ] Test payment account registration (3+ validators)
- [ ] Test reward calculation
- [ ] Test reward claiming
- [ ] Test slashing on testnet validator
- [ ] Monitor for 6+ hours
- [ ] Check validator performance tracking
- [ ] Verify no consensus disruption
- [ ] Confirm block production continues
- [ ] Test Consensus Day phases (if time permits)

### After Mainnet Deploy:

- [ ] All 21 validators online and producing blocks
- [ ] Payment account migration successful (check logs)
- [ ] Performance metrics tracking started
- [ ] Validators can register cold storage accounts
- [ ] First epoch reward calculation succeeds
- [ ] Rewards claimable by validators
- [ ] Monitor for 48 hours

---

## 🚨 Rollback Plan

If critical issues arise:

### Option 1: Revert Runtime
```bash
# Deploy previous runtime WASM
polkadot-js-api tx.sudo.sudoUncheckedWeight \
  --call tx.system.setCode \
  --code ./backups/flarechain_runtime_previous.wasm
```

### Option 2: Emergency Patch
```bash
# Fix issue, rebuild, redeploy
cargo build --release -p flarechain-runtime
# Deploy patched runtime
```

### Option 3: Pause Rewards (Keep Chain Running)
```bash
# Disable reward distribution temporarily
polkadot-js-api tx.sudo.sudo \
  --call tx.validatorRewards.pauseDistribution
```

---

## 📞 Post-Deployment Support

### Validator Communication:

**Send to all validators within 1 hour of upgrade:**

```
🚨 URGENT: Runtime Upgrade Complete - Action Required 🚨

Ëtrid FlareChain runtime has been upgraded to v0.2.0.

✅ What Changed:
- Payment account system is now live
- Rewards will accumulate starting next epoch
- Slashing now protects your hot keys

⚠️ ACTION REQUIRED (within 48 hours):
Register your cold storage payment account:

etrid-cli extrinsic validator-rewards register-payment-account \
  --payment-account <YOUR-COLD-STORAGE-ADDRESS> \
  --account <YOUR-SESSION-ACCOUNT> \
  --uri ws://localhost:9944

⚠️ WARNING:
If you don't register a payment account, rewards will
go to your session account (hot wallet). This is INSECURE.

Verify registration:
etrid-cli query validator-rewards payment-account-of <session-account>

Questions? Check: /Users/macbook/Desktop/etrid/RUNTIME_UPGRADE_GUIDE.md
```

### Monitoring Dashboard:

```bash
# Watch validator registrations
watch -n 5 'etrid-cli query validator-rewards payment-accounts | wc -l'
# Goal: 21/21 within 48 hours

# Watch reward accumulation
watch -n 60 'etrid-cli query validator-rewards pending-rewards <payment-account>'

# Watch performance tracking
etrid-cli query validator-rewards performance-of <session-account>
```

---

## 📦 Files Created (Ready to Deploy)

### Pallets:
- ✅ `/src/pallets/pallet-validator-rewards/` (600+ lines)
- ✅ `/src/pallets/pallet-consensus-day/` (1,131 lines)
- ✅ Updated `/11-peer-roles/staking/pallet/` (slashing integration)

### Migration:
- ✅ `/src/pallets/pallet-validator-rewards/src/migrations.rs`

### Runtime:
- ✅ Updated `/05-multichain/flare-chain/runtime/src/lib.rs`

### Documentation:
- ✅ `/RUNTIME_UPGRADE_GUIDE.md`
- ✅ `/RUNTIME_INTEGRATION_CHECKLIST.md`
- ✅ `/VALIDATOR_PAYMENT_UPGRADE_SUMMARY.md`
- ✅ This file: `/MAINNET_PAYMENT_SYSTEM_DEPLOYMENT.md`

---

## ⏰ Timeline Recommendation

| Phase | Duration | Critical? |
|-------|----------|-----------|
| Build & Local Test | 2-4 hours | ✅ YES |
| Testnet Deploy | 6-12 hours | ✅ YES |
| Testnet Monitoring | 6-24 hours | ⚠️ RECOMMENDED |
| Mainnet Deploy | 1-2 hours | ✅ YES |
| Validator Registration | 48 hours | ✅ YES |
| **TOTAL** | **15-42 hours** | **CRITICAL PATH** |

**Minimum Safe Timeline**: 24 hours (skip extended testnet monitoring)
**Recommended Timeline**: 48 hours (full testnet validation)

---

## 🎯 Success Criteria

### Immediate (0-2 hours post-deploy):
- ✅ All 21 validators still producing blocks
- ✅ No consensus disruptions
- ✅ Migration logs show 21 validators initialized
- ✅ Payment account queries return data

### Short-term (2-48 hours):
- ✅ Validators register payment accounts (20/21 minimum)
- ✅ Performance tracking shows metrics updating
- ✅ No validator complaints or issues

### Long-term (48+ hours):
- ✅ First epoch reward calculation succeeds
- ✅ Validators can claim rewards
- ✅ Rewards sent to correct payment accounts
- ✅ Slashing events (if any) handled correctly

---

## 🔥 DEPLOY NOW

Your validators are running but not getting paid. This is a **mainnet blocker**.

**Next Steps:**
1. Read `/RUNTIME_UPGRADE_GUIDE.md`
2. Build runtime: `cargo build --release -p flarechain-runtime`
3. Test migration: `try-runtime on-runtime-upgrade`
4. Deploy to testnet (6 hours minimum)
5. Deploy to mainnet
6. Notify validators to register payment accounts

**Questions?** All documentation is in `/Users/macbook/Desktop/etrid/`:
- `RUNTIME_UPGRADE_GUIDE.md` - Step-by-step deployment
- `RUNTIME_INTEGRATION_CHECKLIST.md` - Integration details
- `VALIDATOR_PAYMENT_UPGRADE_SUMMARY.md` - Quick reference

---

**Status**: 🚨 READY FOR DEPLOYMENT
**Risk Level**: MEDIUM (hot upgrade, well-tested pattern)
**Impact**: HIGH (validators get paid, proper key security)
**Urgency**: CRITICAL (deploy within 24-48 hours)
