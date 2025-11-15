# Phase 4: GRANDPA Removal - lib.rs Changes

**Status**: PREPARED - DO NOT APPLY YET
**File**: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs`

## Changes to Apply

### 1. Remove GRANDPA Import (Line 11)
```rust
// REMOVE THIS LINE:
use sp_consensus_grandpa::AuthorityId as GrandpaId;
```

### 2. Update SessionKeys to ASF-Only (Lines 69-73)
```rust
// REPLACE:
impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,
    }
}

// WITH:
impl_opaque_keys! {
    pub struct SessionKeys {
        pub asf: AsfConsensus,
    }
}
```

### 3. Update Runtime Version to v108 (Line 83)
```rust
// CHANGE:
spec_version: 106,

// TO:
spec_version: 108,
```

### 4. Remove GRANDPA Migration Code (Lines 100-210)
```rust
// DELETE ENTIRE SECTION:
// ═══════════════════════════════════════════════════════════════════════════════
// RUNTIME UPGRADE MIGRATION v106: Fix GRANDPA Committee Formation
// ═══════════════════════════════════════════════════════════════════════════════
pub mod migrations {
    // ... entire migration module
}
```

### 5. Remove pallet_grandpa::Config (Lines 275-285)
```rust
// DELETE:
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<168>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}
```

### 6. Update construct_runtime! (Line 1232-1236)
```rust
// REMOVE this line from construct_runtime!:
Grandpa: pallet_grandpa,
```

### 7. Update Executive Migration (Line 1351)
```rust
// CHANGE:
migrations::FixGrandpaCommitteeV106,

// TO:
(),  // No migrations needed for v108
```

### 8. Remove GRANDPA Runtime API (Lines 1475-1500)
```rust
// DELETE ENTIRE IMPL BLOCK:
impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
        Grandpa::grandpa_authorities()
    }
    // ... rest of implementation
}
```

## Summary of Deletions
- 1 import statement (GrandpaId)
- 1 SessionKeys field (grandpa)
- 110 lines of migration code
- 11 lines of pallet config
- 1 line in construct_runtime!
- 1 migration reference in Executive
- 26 lines of runtime API implementation

Total: ~150 lines removed

## Files to Review Before Applying
1. `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs`
2. Backup at: `runtime/src/lib.rs.phase3-backup`
