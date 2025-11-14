# DecentralizedDirector Committee Inclusion Strategy

**Date:** November 12, 2025
**Question:** How can we have 9 DecentralizedDirector validators and 21 committee members without filtering them out?

---

## Current Situation

**Validator Distribution:**
- 9 DecentralizedDirector (peerType=2)
- 12 ValidityNode + FlareNode (peerType=0 and 1)
- **Total:** 21 validators

**Problem:**
Current committee filter at `09-consensus/validator-management/src/lib.rs:88-90`:
```rust
pub fn can_be_in_committee(&self) -> bool {
    self.is_validator_type()  // Only allows ValidityNode and FlareNode
}

pub fn is_validator_type(&self) -> bool {
    matches!(self, PeerType::ValidityNode | PeerType::FlareNode)
}
```

This would **exclude all 9 DecentralizedDirector validators**, leaving only 12 in committee.

---

## Solution Options

### Option 1: Include DecentralizedDirector in Committee Filter ⭐ RECOMMENDED

**What to change:**
Update `can_be_in_committee()` to explicitly include DecentralizedDirector.

**Code change:**
```rust
// File: 09-consensus/validator-management/src/lib.rs (lines 87-90)

pub fn can_be_in_committee(&self) -> bool {
    matches!(
        self,
        PeerType::ValidityNode | PeerType::FlareNode | PeerType::DecentralizedDirector
    )
}
```

**Pros:**
- ✅ Simple, clean solution
- ✅ All 21 validators can participate in committee
- ✅ Maintains stake-weighted selection (highest stake gets selected)
- ✅ No changes to runtime or genesis config needed
- ✅ Preserves validator role distinctions

**Cons:**
- None significant

**Implementation:**
```bash
# Edit the file
vim 09-consensus/validator-management/src/lib.rs

# Line 88-90, change from:
pub fn can_be_in_committee(&self) -> bool {
    self.is_validator_type()
}

# To:
pub fn can_be_in_committee(&self) -> bool {
    matches!(
        self,
        PeerType::ValidityNode | PeerType::FlareNode | PeerType::DecentralizedDirector
    )
}

# Rebuild
cargo build --release --bin flarechain-node
```

---

### Option 2: Make DecentralizedDirector a "Validator Type"

**What to change:**
Update `is_validator_type()` to include DecentralizedDirector.

**Code change:**
```rust
// File: 09-consensus/validator-management/src/lib.rs (lines 83-85)

pub fn is_validator_type(&self) -> bool {
    matches!(
        self,
        PeerType::ValidityNode | PeerType::FlareNode | PeerType::DecentralizedDirector
    )
}
```

**Pros:**
- ✅ All 21 validators included
- ✅ Works with existing `can_be_in_committee()` logic
- ✅ Semantically correct if DDs are meant to be validators

**Cons:**
- ⚠️  Changes the meaning of "validator type" (may affect other code)
- ⚠️  Need to audit all uses of `is_validator_type()` to ensure they work correctly

**When to use:**
If DecentralizedDirector validators should be treated the same as ValidityNode/FlareNode for **all purposes**, not just committee membership.

---

### Option 3: Increase Committee Size Beyond 21 ❌ NOT RECOMMENDED

**What to change:**
Set `CommitteeSize = 30` (or higher) in runtime to ensure all validators fit.

**Code change:**
```rust
// File: 05-multichain/flare-chain/runtime/src/lib.rs

parameter_types! {
    pub const CommitteeSize: u32 = 30;  // Changed from 21
}
```

**Pros:**
- None

**Cons:**
- ❌ Doesn't fix the filtering issue (DDs still excluded)
- ❌ Misleading - committee would still only have 12 members, not 30
- ❌ Creates confusion between expected vs actual committee size
- ❌ Doesn't address root cause

**Verdict:** Don't use this option.

---

### Option 4: Reduce to 12 Committee Members ❌ NOT RECOMMENDED

**What to change:**
Remove DecentralizedDirector validators from genesis, only use 12 ValidityNode/FlareNode validators.

**Pros:**
- Works with existing code

**Cons:**
- ❌ Loses 9 validators (less decentralization)
- ❌ Wasted infrastructure (9 VMs not participating)
- ❌ Lower Byzantine fault tolerance
- ❌ Goes against goal of having 21 validators

**Verdict:** Don't use this option.

---

## Recommended Solution: Option 1

### Implementation Steps

#### 1. Update the Code

**File:** `09-consensus/validator-management/src/lib.rs`

```diff
  /// Check if this peer type can be in PPFA committee
  pub fn can_be_in_committee(&self) -> bool {
-     self.is_validator_type()
+     matches!(
+         self,
+         PeerType::ValidityNode | PeerType::FlareNode | PeerType::DecentralizedDirector
+     )
  }
```

#### 2. Verify the Change

```bash
# Check the change
git diff 09-consensus/validator-management/src/lib.rs

# Compile to verify syntax
cargo check -p validator-management
```

#### 3. Test Committee Selection

```bash
# Run unit tests
cargo test -p validator-management test_committee

# Check for any test failures
```

#### 4. Update Runtime if Needed

If you need to ensure DecentralizedDirector validators have appropriate stake requirements:

**File:** `09-consensus/validator-management/src/lib.rs` (around line 63-78)

```rust
pub fn minimum_stake(&self) -> Balance {
    match self {
        PeerType::ValidityNode => 64_000_000_000_000_000_000_000,      // 64 ËTR
        PeerType::FlareNode => 64_000_000_000_000_000_000_000,         // 64 ËTR
        PeerType::DecentralizedDirector => 128_000_000_000_000_000_000_000, // 128 ËTR (already correct)
    }
}
```

This ensures DecentralizedDirector validators need 128 ETR minimum (2x regular validators), which seems appropriate for their elevated role.

#### 5. Build and Deploy

```bash
# Build release binary
cargo build --release --bin flarechain-node

# Deploy using architecture-aware strategy (see VM_DEPLOYMENT_REFERENCE.md)
```

---

## Understanding Committee Selection Logic

### How It Works Now

**Step 1: Filter Eligible Validators**
```rust
let mut eligible: Vec<_> = self
    .validator_pool
    .values()
    .filter(|v| {
        v.can_participate()                               // Active and online
            && v.reputation >= MIN_REPUTATION_FOR_COMMITTEE  // Good reputation
            && v.peer_type.can_be_in_committee()          // ⬅️ THIS IS THE GATE
    })
    .collect();
```

**Step 2: Sort by Stake (Highest First)**
```rust
eligible.sort_by(|a, b| {
    b.stake.cmp(&a.stake)
        .then_with(|| b.reputation.cmp(&a.reputation))
});
```

**Step 3: Take Top N Validators**
```rust
let selected = eligible
    .into_iter()
    .take(self.target_size as usize)  // Usually 21
    .collect();
```

### After Fix (Option 1)

**With 9 DecentralizedDirector + 12 ValidityNode/FlareNode:**

All 21 validators pass the `can_be_in_committee()` filter:
- ✅ 9 DecentralizedDirector (peerType=2) - now eligible
- ✅ 12 ValidityNode/FlareNode (peerType=0,1) - eligible

Then sorted by stake:
1. If DecentralizedDirector validators have 128 ETR each
2. If ValidityNode/FlareNode have 64 ETR each
3. All 9 DDs will rank higher (sorted first)
4. Then 12 VN/FN validators

Result: **All 21 selected for committee** (assuming target_size=21)

---

## Genesis Configuration

### Current Genesis Structure

**File:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

```json
{
  "validatorCommittee": {
    "validators": [
      // 9 DecentralizedDirector validators
      {
        "accountId": "0x...",
        "stake": 128000000000000000000000,  // 128 ETR
        "peerType": 2,  // DecentralizedDirector
        "auraKey": "0x...",
        "grandpaKey": "0x..."
      },
      // 12 ValidityNode/FlareNode validators
      {
        "accountId": "0x...",
        "stake": 64000000000000000000000,  // 64 ETR
        "peerType": 0,  // ValidityNode or 1 for FlareNode
        "auraKey": "0x...",
        "grandpaKey": "0x..."
      },
      // ... total 21 validators
    ]
  }
}
```

**No changes needed to genesis** - just update the code to allow peerType=2 in committee.

---

## Verification After Fix

### 1. Check Code Compiles
```bash
cargo check --workspace
```

### 2. Run Tests
```bash
cargo test --workspace
cargo test -p validator-management
```

### 3. Test Committee Selection Locally

Create a test to verify all 21 validators are selected:

```rust
// Add to 09-consensus/validator-management/src/committee.rs tests

#[test]
fn test_decentralized_director_in_committee() {
    let params = ASFParams::default();
    let mut committee = CommitteeManager::new(21);

    // Add 9 DecentralizedDirector validators (128 ETR each)
    for i in 0..9 {
        let validator_id = ValidatorId::from([i as u8; 32]);
        let validator_info = ValidatorInfo::new(
            validator_id,
            128_000_000_000_000_000_000_000,  // 128 ETR
            PeerType::DecentralizedDirector,
        );
        assert!(committee.add_validator(validator_info).is_ok());
    }

    // Add 12 ValidityNode validators (64 ETR each)
    for i in 9..21 {
        let validator_id = ValidatorId::from([i as u8; 32]);
        let validator_info = ValidatorInfo::new(
            validator_id,
            64_000_000_000_000_000_000_000,  // 64 ETR
            PeerType::ValidityNode,
        );
        assert!(committee.add_validator(validator_info).is_ok());
    }

    // Rotate to activate committee
    assert!(committee.rotate_committee(1).is_ok());

    // Verify all 21 validators are in committee
    assert_eq!(committee.active_committee_size(), 21);

    println!("✅ All 21 validators (9 DD + 12 VN) included in committee");
}
```

Run the test:
```bash
cargo test -p validator-management test_decentralized_director_in_committee -- --nocapture
```

### 4. Verify on Running Node

After deployment, check logs:
```bash
journalctl -u flarechain-validator | grep -i "committee.*21\|loaded.*21"
```

Expected output:
```
✅ Loaded 21 committee members from runtime
Committee size: 21 (9 DecentralizedDirector, 12 ValidityNode/FlareNode)
```

### 5. Check via RPC

```bash
# Query validator count (if pallet exposes this)
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "validatorCommittee_validatorCount"}' | jq

# Expected: 21
```

---

## Summary

### Question
"How can we have 9 DecentralizedDirector and 21 committee members so they won't get filtered out?"

### Answer
**Update `can_be_in_committee()` to include DecentralizedDirector:**

```rust
// File: 09-consensus/validator-management/src/lib.rs (line 88)
pub fn can_be_in_committee(&self) -> bool {
    matches!(
        self,
        PeerType::ValidityNode | PeerType::FlareNode | PeerType::DecentralizedDirector
    )
}
```

This is a **one-line change** (well, 5 lines formatted) that:
- ✅ Allows all 9 DecentralizedDirector validators to participate
- ✅ Allows all 12 ValidityNode/FlareNode validators to participate
- ✅ Results in full 21-member committee
- ✅ Maintains stake-weighted selection
- ✅ No genesis or runtime config changes needed

### Next Steps

1. Apply the code change
2. Build new binary
3. Deploy to all VMs
4. Start chain with all 21 validators
5. Verify committee size = 21 in logs

**Result:** All 9 DecentralizedDirector + 12 other validators = **21 total in committee** ✅

---

*Document created: November 12, 2025*
*Status: Solution identified - ready to implement*
