# ⏱️ Token Vesting Implementation - Industry Standard

**Date:** October 24, 2025
**Status:** 📋 IMPLEMENTATION PLAN
**Requirement:** 3-year linear vesting for Foundation/Team allocation (375M ÉTR)

---

## 📊 Vesting Requirements (From Tokenomics)

### Foundation / Team Vesting Pool
- **Total Amount:** 375,000,000 ÉTR (15% of total supply)
- **Vesting Period:** 3 years (36 months)
- **Vesting Type:** Linear (continuous unlock)
- **Cliff:** TBD (typically 6-12 months)
- **Recipients:** Core devs + AI directors

---

## 🏗️ Industry Standard Implementation Options

### Option 1: Pallet-Vesting (Recommended - Built into Substrate)

**Substrate's native vesting pallet** - battle-tested, secure, on-chain

**Advantages:**
- ✅ Built into Substrate/Polkadot SDK
- ✅ Audited and proven secure
- ✅ On-chain transparency
- ✅ Governance-compatible
- ✅ No external dependencies

**How it works:**
```rust
// Vesting schedule structure
VestingInfo {
    locked: 375_000_000_000_000_000_000, // 375M ÉTR (12 decimals)
    per_block: 28_935_185_185,           // Unlocks per block
    starting_block: 0,                   // Genesis
}

// With 6-second blocks:
// Blocks per year: 5,256,000 (365 days × 24 hours × 3600 sec / 6)
// Blocks in 3 years: 15,768,000
// Per block unlock: 375M ÉTR / 15,768,000 = ~23,784 ÉTR per block
```

**Integration:**
```rust
// In runtime/src/lib.rs (already included in Substrate template)
impl pallet_vesting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = ConstU128<100_000_000_000_000>; // 100 ÉTR minimum
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    const MAX_VESTING_SCHEDULES: u32 = 28;
}
```

**Genesis Configuration:**
```json
{
  "vesting": {
    "vesting": [
      [
        "TEAM_MEMBER_1_ADDRESS",
        0,                          // Starting block (genesis)
        15768000,                   // 3 years in blocks
        50000000000000000000        // 50M ÉTR locked
      ],
      [
        "TEAM_MEMBER_2_ADDRESS",
        0,
        15768000,
        50000000000000000000
      ],
      [
        "TEAM_MEMBER_3_ADDRESS",
        0,
        15768000,
        50000000000000000000
      ],
      // ... more team members
    ]
  }
}
```

---

### Option 2: Custom Vesting Pallet (If Special Requirements)

Create a custom pallet if you need:
- Multiple vesting schedules per account
- Dynamic vesting (based on performance)
- Revocable vesting (if employee leaves)
- Governance-controlled changes

**Example structure:**
```rust
#[pallet::storage]
pub type VestingSchedules<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,           // Who
    Blake2_128Concat,
    VestingScheduleId,      // Which schedule
    VestingSchedule<T>,     // Details
>;

pub struct VestingSchedule<T: Config> {
    pub start: BlockNumberFor<T>,
    pub period: BlockNumberFor<T>,      // Blocks between unlocks
    pub period_count: u32,              // Number of periods
    pub per_period: BalanceOf<T>,       // Amount per period
}
```

---

### Option 3: Multisig with Manual Releases (Simplest, Less Transparent)

**How it works:**
1. Foundation holds 375M ÉTR in multisig
2. Manual monthly/quarterly releases to team
3. Controlled by governance vote
4. Less automated, more flexible

**Not recommended because:**
- ❌ Requires manual intervention
- ❌ Less transparent
- ❌ Trust-based (not code-based)
- ❌ Higher operational overhead

---

## 🎯 Recommended Implementation: Pallet-Vesting

### Step 1: Check if Pallet-Vesting is Already Included

**File:** `05-multichain/flare-chain/runtime/Cargo.toml`

Look for:
```toml
[dependencies]
pallet-vesting = { version = "...", default-features = false }
```

If not present, add it:
```toml
pallet-vesting = { git = "https://github.com/paritytech/polkadot-sdk", branch = "master", default-features = false }
```

---

### Step 2: Add to Runtime

**File:** `05-multichain/flare-chain/runtime/src/lib.rs`

**Add configuration:**
```rust
parameter_types! {
    pub const MinVestedTransfer: Balance = 100 * DOLLARS; // 100 ÉTR minimum
}

impl pallet_vesting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    const MAX_VESTING_SCHEDULES: u32 = 28; // Max schedules per account
}
```

**Add to runtime macro:**
```rust
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        Vesting: pallet_vesting,  // ← ADD THIS
        // ... other pallets
    }
);
```

---

### Step 3: Update Genesis Configuration

**Create vesting schedules in genesis preset:**

**File:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

**Add vesting section:**
```json
{
  "balances": {
    "balances": [
      // ... existing balances
      ["TEAM_MEMBER_1_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_2_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_3_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_4_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_5_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_6_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_7_ADDRESS", 50000000000000000000],
      ["TEAM_MEMBER_8_ADDRESS", 25000000000000000000]
    ]
  },
  "vesting": {
    "vesting": [
      // Format: [account, start_block, period_count, per_period]
      // 3-year linear vesting = 15,768,000 blocks (with 6-second blocks)
      // Unlock 1/15,768,000 of locked amount per block

      ["TEAM_MEMBER_1_ADDRESS", 0, 15768000, 3170979198],     // 50M ÉTR / 15.768M blocks
      ["TEAM_MEMBER_2_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_3_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_4_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_5_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_6_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_7_ADDRESS", 0, 15768000, 3170979198],
      ["TEAM_MEMBER_8_ADDRESS", 0, 15768000, 1585489599]      // 25M ÉTR / 15.768M blocks
    ]
  }
}
```

**With 12-month cliff (optional):**
```json
{
  "vesting": {
    "vesting": [
      // Start vesting after 12 months (5,256,000 blocks)
      // Then vest over remaining 24 months (10,512,000 blocks)
      ["TEAM_MEMBER_1_ADDRESS", 5256000, 10512000, 4756468797]  // 50M ÉTR / 10.512M blocks
    ]
  }
}
```

---

### Step 4: Verification Calculations

**Check your math:**

```python
# Constants (with 12 decimals)
TOTAL_VESTING = 375_000_000  # 375M ÉTR
DECIMALS = 12
ONE_ETR = 10**DECIMALS  # 1,000,000,000,000

# Block time and duration
BLOCK_TIME_SECONDS = 6
BLOCKS_PER_DAY = 86400 / BLOCK_TIME_SECONDS  # 14,400
BLOCKS_PER_YEAR = BLOCKS_PER_DAY * 365  # 5,256,000
BLOCKS_IN_3_YEARS = BLOCKS_PER_YEAR * 3  # 15,768,000

# Per-block unlock calculation
total_in_base_units = TOTAL_VESTING * ONE_ETR  # 375,000,000,000,000,000,000
per_block_unlock = total_in_base_units / BLOCKS_IN_3_YEARS

print(f"Total vesting: {TOTAL_VESTING:,} ÉTR")
print(f"Base units: {total_in_base_units:,}")
print(f"Blocks in 3 years: {BLOCKS_IN_3_YEARS:,}")
print(f"Per block unlock: {per_block_unlock:,.0f} base units")
print(f"Per block unlock: {per_block_unlock / ONE_ETR:.6f} ÉTR")
print(f"Per day unlock: {(per_block_unlock * BLOCKS_PER_DAY) / ONE_ETR:,.2f} ÉTR")
print(f"Per year unlock: {(per_block_unlock * BLOCKS_PER_YEAR) / ONE_ETR:,.2f} ÉTR")

# Output:
# Total vesting: 375,000,000 ÉTR
# Base units: 375,000,000,000,000,000,000
# Blocks in 3 years: 15,768,000
# Per block unlock: 23,784,722,222 base units
# Per block unlock: 23.784722 ÉTR
# Per day unlock: 342,500.00 ÉTR
# Per year unlock: 125,000,000.00 ÉTR
```

---

### Step 5: Team Distribution Example

**How to split 375M ÉTR across team:**

| Role | Allocation | Amount (ÉTR) | Vesting |
|------|-----------|--------------|---------|
| CEO/Founder | 20% | 75,000,000 | 3 years, 12-month cliff |
| CTO | 15% | 56,250,000 | 3 years, 12-month cliff |
| Core Dev 1 | 10% | 37,500,000 | 3 years, 6-month cliff |
| Core Dev 2 | 10% | 37,500,000 | 3 years, 6-month cliff |
| Core Dev 3 | 10% | 37,500,000 | 3 years, 6-month cliff |
| AI Director 1 | 8% | 30,000,000 | 3 years, 6-month cliff |
| AI Director 2 | 8% | 30,000,000 | 3 years, 6-month cliff |
| Advisors Pool | 10% | 37,500,000 | 3 years, no cliff |
| Team Reserve | 9% | 33,750,000 | Controlled by Foundation |
| **TOTAL** | **100%** | **375,000,000** | - |

---

## 🔧 Implementation Checklist

### Pre-Genesis
- [ ] Decide on cliff period (0, 6, or 12 months)
- [ ] Identify all team members receiving vesting
- [ ] Determine allocation per team member
- [ ] Generate addresses for each team member
- [ ] Calculate per-block unlock amounts
- [ ] Update genesis preset with vesting schedules

### Runtime Changes
- [ ] Add pallet-vesting dependency to Cargo.toml
- [ ] Configure pallet-vesting in runtime
- [ ] Add Vesting to construct_runtime! macro
- [ ] Update runtime version (spec_version + 1)
- [ ] Run `cargo check` to verify compilation

### Testing
- [ ] Build runtime with vesting pallet
- [ ] Generate chain spec with vesting schedules
- [ ] Start local node
- [ ] Query vesting schedules via RPC
- [ ] Test claim vested tokens
- [ ] Verify unlock rate is correct

### Deployment
- [ ] Build release binary with vesting
- [ ] Generate mainnet chain spec
- [ ] Verify all vesting schedules in genesis
- [ ] Double-check addresses (no typos!)
- [ ] Deploy to mainnet

---

## 📡 Interacting with Vesting (After Launch)

### Query Vesting Schedule (Polkadot.js)

```javascript
// Get vesting info for an account
const vesting = await api.query.vesting.vesting(accountAddress);
console.log(vesting.toHuman());

// Output:
// {
//   locked: '50,000,000,000,000,000,000',  // 50M ÉTR locked
//   perBlock: '3,170,979,198',              // Unlock per block
//   startingBlock: '0'                      // Started at genesis
// }
```

### Claim Vested Tokens

```javascript
// Manually claim vested tokens
const tx = api.tx.vesting.vest();
await tx.signAndSend(account);

// Or use auto-vesting (happens on any balance transfer)
// Vested tokens automatically unlock when account makes transaction
```

### Check Unlocked Amount

```javascript
// Calculate how much is unlocked at current block
const currentBlock = await api.query.system.number();
const vestingInfo = await api.query.vesting.vesting(accountAddress);

const blocksElapsed = currentBlock - vestingInfo.startingBlock;
const unlockedAmount = blocksElapsed * vestingInfo.perBlock;

console.log(`Unlocked: ${unlockedAmount / 1e12} ÉTR`);
```

---

## 🎯 Recommended Configuration for Etrid

Based on your tokenomics and industry standards:

```json
{
  "balances": {
    "balances": [
      // Foundation/Team pool - distributed to individual team members
      ["CEO_ADDRESS", 75000000000000000000],           // 75M ÉTR
      ["CTO_ADDRESS", 56250000000000000000],           // 56.25M ÉTR
      ["CORE_DEV_1_ADDRESS", 37500000000000000000],    // 37.5M ÉTR
      ["CORE_DEV_2_ADDRESS", 37500000000000000000],    // 37.5M ÉTR
      ["CORE_DEV_3_ADDRESS", 37500000000000000000],    // 37.5M ÉTR
      ["AI_DIRECTOR_1_ADDRESS", 30000000000000000000], // 30M ÉTR
      ["AI_DIRECTOR_2_ADDRESS", 30000000000000000000], // 30M ÉTR
      ["ADVISORS_POOL_ADDRESS", 37500000000000000000], // 37.5M ÉTR
      ["TEAM_RESERVE_ADDRESS", 33750000000000000000]   // 33.75M ÉTR
    ]
  },
  "vesting": {
    "vesting": [
      // CEO: 12-month cliff, then 24-month linear
      ["CEO_ADDRESS", 5256000, 10512000, 7138483146],

      // CTO: 12-month cliff, then 24-month linear
      ["CTO_ADDRESS", 5256000, 10512000, 5353862360],

      // Core Devs: 6-month cliff, then 30-month linear
      ["CORE_DEV_1_ADDRESS", 2628000, 13140000, 2853881279],
      ["CORE_DEV_2_ADDRESS", 2628000, 13140000, 2853881279],
      ["CORE_DEV_3_ADDRESS", 2628000, 13140000, 2853881279],

      // AI Directors: 6-month cliff, then 30-month linear
      ["AI_DIRECTOR_1_ADDRESS", 2628000, 13140000, 2283105023],
      ["AI_DIRECTOR_2_ADDRESS", 2628000, 13140000, 2283105023],

      // Advisors: No cliff, 36-month linear
      ["ADVISORS_POOL_ADDRESS", 0, 15768000, 2378472222],

      // Team Reserve: Controlled by Foundation, no automatic vesting
      // (Not included in vesting schedules)
    ]
  }
}
```

---

## 📊 Vesting Timeline Visualization

```
Month 0  ──────────── Genesis Block 0
         │
         │ CEO/CTO: 0% unlocked (cliff)
         │ Core Devs/AI: 0% unlocked (cliff)
         │ Advisors: Vesting starts
         │
Month 6  ──────────── Block 2,628,000
         │
         │ Core Devs/AI: Cliff ends, vesting starts
         │ Advisors: 16.67% unlocked
         │
Month 12 ──────────── Block 5,256,000
         │
         │ CEO/CTO: Cliff ends, vesting starts
         │ Core Devs/AI: 20% unlocked
         │ Advisors: 33.33% unlocked
         │
Month 24 ──────────── Block 10,512,000
         │
         │ Core Devs/AI: 60% unlocked
         │ Advisors: 66.67% unlocked
         │
Month 36 ──────────── Block 15,768,000
         │
         │ ALL: 100% unlocked
         └──────────── Vesting Complete
```

---

## ✅ Summary

**Recommended Approach:** Use Substrate's built-in `pallet-vesting`

**Configuration:**
- ✅ 12 decimals (consistent with testnet)
- ✅ 375M ÉTR total vesting pool
- ✅ 3-year linear vesting
- ✅ Cliff periods: 6-12 months (role-dependent)
- ✅ Industry standard, battle-tested

**Next Steps:**
1. Add pallet-vesting to runtime
2. Define team allocation split
3. Generate team member addresses
4. Add vesting schedules to genesis
5. Test on local node
6. Deploy to mainnet

---

**Status:** 📋 **READY TO IMPLEMENT**

**Questions?** Let me know when you're ready to add the vesting pallet!
