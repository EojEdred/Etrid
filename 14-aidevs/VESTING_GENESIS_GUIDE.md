# Vesting Genesis Configuration Guide

**Date:** October 25, 2025
**Status:** ✅ Pallet-vesting integrated into runtime
**File:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet_with_vesting.json`

---

## Overview

This guide explains the vesting configuration for FlareChain mainnet genesis, including how team allocation (375M ÉTR) is distributed with vesting schedules.

---

## Team Distribution (375M ÉTR Total)

| Role | Amount (ÉTR) | Vesting Period | Cliff Period | Notes |
|------|--------------|----------------|--------------|-------|
| CEO/Founder | 75,000,000 | 3 years | 12 months | 20% of team pool |
| CTO | 56,250,000 | 3 years | 12 months | 15% of team pool |
| Core Dev 1 | 37,500,000 | 3 years | 6 months | 10% of team pool |
| Core Dev 2 | 37,500,000 | 3 years | 6 months | 10% of team pool |
| Core Dev 3 | 37,500,000 | 3 years | 6 months | 10% of team pool |
| AI Director | 30,000,000 | 3 years | 6 months | 8% of team pool |
| Advisor 1 | 26,250,000 | 3 years | No cliff | 7% of team pool |
| Advisor 2 | 26,250,000 | 3 years | No cliff | 7% of team pool |
| Advisor 3 | 26,250,000 | 3 years | No cliff | 7% of team pool |
| Marketing Lead | 23,500,000 | 3 years | No cliff | 6.27% of team pool |
| **TOTAL** | **375,000,000** | | | **100%** |

---

## Vesting Schedule Calculations

### Block Time Configuration

- **Block time:** 6 seconds
- **Blocks per day:** 14,400 (86,400 / 6)
- **Blocks per year:** 5,256,000 (14,400 × 365)
- **Blocks in 3 years:** 15,768,000 (5,256,000 × 3)

### Cliff Period Blocks

- **6 months:** 2,628,000 blocks (5,256,000 / 2)
- **12 months:** 5,256,000 blocks (1 year)

### Per-Block Unlock Calculation

**Formula:**
```
per_block = (amount_in_base_units) / (vesting_period_blocks)
```

**With 12 decimals:**
- 1 ÉTR = 1,000,000,000,000 base units (10^12)

---

## Example Calculations

### CEO/Founder (75M ÉTR, 12-month cliff, 3-year vest)

```
Amount: 75,000,000 ÉTR
Base units: 75,000,000 × 10^12 = 75,000,000,000,000,000,000

Cliff: 12 months = 5,256,000 blocks
Vesting period: 24 months (remaining) = 10,512,000 blocks

Per-block unlock: 75,000,000,000,000,000,000 / 10,512,000 = 7,133,203,933 base units

Genesis format:
["CEO_ADDRESS", 5256000, 10512000, 7133203933]
         ^         ^         ^          ^
      Account  Start block  Period  Per-block
```

**Vesting timeline:**
- Blocks 0 - 5,256,000: No unlock (cliff)
- Blocks 5,256,000+: Unlock 7,133,203,933 base units per block
- After 10,512,000 blocks: Fully unlocked (75M ÉTR)

### Core Developer (37.5M ÉTR, 6-month cliff, 3-year vest)

```
Amount: 37,500,000 ÉTR
Base units: 37,500,000 × 10^12 = 37,500,000,000,000,000,000

Cliff: 6 months = 2,628,000 blocks
Vesting period: 30 months (remaining) = 13,140,000 blocks

Per-block unlock: 37,500,000,000,000,000,000 / 13,140,000 = 2,853,881,573 base units

Genesis format:
["DEV_ADDRESS", 2628000, 13140000, 2853881573]
```

### Advisor (26.25M ÉTR, no cliff, 3-year vest)

```
Amount: 26,250,000 ÉTR
Base units: 26,250,000 × 10^12 = 26,250,000,000,000,000,000

Cliff: None = 0 blocks
Vesting period: 3 years = 15,768,000 blocks

Per-block unlock: 26,250,000,000,000,000,000 / 15,768,000 = 1,664,893,549 base units

Genesis format:
["ADVISOR_ADDRESS", 0, 15768000, 1664893549]
```

---

## Genesis Configuration Structure

### Balances Section

```json
"balances": {
  "balances": [
    // Treasury and pools
    ["FOUNDATION_TREASURY_MULTISIG_ADDRESS_HERE", 875000000000000000000],
    ["COMMUNITY_LP_POOL_ADDRESS_HERE", 250000000000000000000],
    ["NETWORK_EXPANSION_POOL_ADDRESS_HERE", 625000000000000000000],
    ["FOUNDERS_POOL_ADDRESS_HERE", 125000000000000000000],
    ["INITIAL_CIRCULATING_SUPPLY_ADDRESS_HERE", 250000000000000000000],

    // Team members (with vesting)
    ["TEAM_MEMBER_1_CEO_ADDRESS_HERE", 75000000000000000000],
    ["TEAM_MEMBER_2_CTO_ADDRESS_HERE", 56250000000000000000],
    // ... etc
  ]
}
```

### Vesting Section

```json
"vesting": {
  "vesting": [
    // Format: [account, start_block, period_count, per_period]
    ["TEAM_MEMBER_1_CEO_ADDRESS_HERE", 5256000, 10512000, 7133203933],
    ["TEAM_MEMBER_2_CTO_ADDRESS_HERE", 5256000, 10512000, 5349902950],
    // ... etc
  ]
}
```

---

## Verification

### Total Supply Check

```
Foundation Treasury:     875,000,000 ÉTR
Community LP Pool:       250,000,000 ÉTR
Network Expansion:       625,000,000 ÉTR
Founders Pool:           125,000,000 ÉTR
Initial Circulating:     250,000,000 ÉTR
Team Vesting (10 members): 375,000,000 ÉTR
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TOTAL:                 2,500,000,000 ÉTR ✓
```

### Team Vesting Total Check

```
CEO:             75,000,000 ÉTR
CTO:             56,250,000 ÉTR
Core Dev 1:      37,500,000 ÉTR
Core Dev 2:      37,500,000 ÉTR
Core Dev 3:      37,500,000 ÉTR
AI Director:     30,000,000 ÉTR
Advisor 1:       26,250,000 ÉTR
Advisor 2:       26,250,000 ÉTR
Advisor 3:       26,250,000 ÉTR
Marketing Lead:  23,500,000 ÉTR
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TOTAL:          375,000,000 ÉTR ✓
```

---

## Runtime Configuration

**Pallet-vesting is now integrated in:**
- File: `05-multichain/flare-chain/runtime/src/lib.rs:216-223`
- Cargo.toml: `05-multichain/flare-chain/runtime/Cargo.toml:92`
- construct_runtime!: Line 708

**Configuration:**
```rust
parameter_types! {
    pub const MinVestedTransfer: Balance = 100_000_000_000_000; // 0.0001 ETR
}

impl pallet_vesting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BlockNumberToBalance = sp_runtime::traits::ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    const MAX_VESTING_SCHEDULES: u32 = 28;
}
```

**Runtime version incremented:**
- spec_version: 100 → 101

---

## Next Steps

1. **Replace Placeholder Addresses**
   - Generate secure addresses for all team members
   - Generate validator stash addresses
   - Generate GRANDPA authority keys
   - Generate foundation multisig address

2. **Build Final Chain Spec**
   ```bash
   cd /Users/macbook/Desktop/etrid
   cargo build --release --locked

   # Generate raw chain spec
   ./target/release/etrid build-spec \
     --chain flarechain_mainnet_with_vesting \
     --raw > flarechain-mainnet-raw.json
   ```

3. **Verify Chain Spec**
   ```bash
   # Check vesting schedules
   cat flarechain-mainnet-raw.json | grep -A 20 "vesting"

   # Verify total supply
   # (Sum all balances should equal 2.5B ÉTR)
   ```

4. **Test on Single Node**
   ```bash
   ./target/release/etrid \
     --chain flarechain-mainnet-raw.json \
     --tmp \
     --alice
   ```

---

## Benefits of Individual Vesting

✅ **Transparency:** On-chain, auditable vesting schedules
✅ **Automatic:** No manual distribution required
✅ **Trustless:** Code enforces vesting, not governance
✅ **Standard:** Industry-proven approach (pallet-vesting)
✅ **Flexible:** Different cliff periods for different roles

---

## Alternative: Pool-Based Vesting

If you prefer a single team vesting pool instead of individual schedules:

```json
{
  "balances": {
    "balances": [
      ["FOUNDATION_TEAM_VESTING_ADDRESS_HERE", 375000000000000000000]
    ]
  },
  "vesting": {
    "vesting": [
      ["FOUNDATION_TEAM_VESTING_ADDRESS_HERE", 0, 15768000, 23784722222]
    ]
  }
}
```

**Pros:**
- Simpler genesis
- More flexible team changes
- Foundation controls distribution

**Cons:**
- Requires manual distribution
- Less transparent
- Requires governance/trust

---

**Status:** ✅ Vesting implementation complete, ready for address replacement
