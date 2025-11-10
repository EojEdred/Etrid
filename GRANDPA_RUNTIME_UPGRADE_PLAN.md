# FlareChain Runtime Upgrade Plan: GRANDPA Finality Fix

**Date:** November 9, 2025
**Critical Issue:** ZERO GRANDPA authorities in runtime storage
**Current Spec Version:** 104
**Target Spec Version:** 105
**Finality Stuck At:** Block #63,274
**Best Block:** #76,401

---

## Problem Analysis

### Current State
- **GRANDPA Storage:** `0x0000000000000000` (ZERO authorities)
- **Expected:** 9 Director validators as GRANDPA authorities
- **Required for Finality:** 6/9 Directors (67% supermajority)
- **Synced Directors:** 3/9 (Gizzi, Audit Dev, EojEdred)
- **Impact:** Chain cannot finalize blocks beyond #63,274

### Root Cause
The runtime was deployed without properly initializing GRANDPA authorities in storage. While the runtime expects GRANDPA authorities to exist, the storage is empty, preventing finality from progressing.

---

## Runtime File Structure

### Location on Gizzi VM
```
~/etrid/05-multichain/flare-chain/
├── runtime/
│   ├── Cargo.toml          # Runtime package configuration
│   ├── build.rs            # WASM builder configuration
│   ├── presets/            # Genesis presets
│   └── src/
│       └── lib.rs          # Main runtime code (1,443 lines)
└── node/                   # Node binary (separate from runtime)
```

### Key Runtime Configuration

**File:** `~/etrid/05-multichain/flare-chain/runtime/src/lib.rs`

**Current Version (lines 77-78):**
```rust
spec_version: 104,
impl_version: 1,
```

**GRANDPA Config (lines 153-160):**
```rust
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}
```

**Runtime Construction (line 1048):**
```rust
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Grandpa: pallet_grandpa,  // Line 1048
        // ... other pallets ...
    }
);
```

**Session Keys (lines 60-66):**
```rust
impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,
    }
}
```

---

## The 9 GRANDPA Authority Keys

**IMPORTANT:** Only Directors (Type 4) participate in GRANDPA finality.
Validity Nodes (Type 3) only produce blocks via AURA but don't finalize.

### Director GRANDPA Public Keys (in order)

```rust
// DIRECTOR 1: Gizzi (64.181.215.19) - ✅ SYNCED
0x00ee75f5f1fdf647006e7408c5e9c7ca98afcb2fcd9ae66503dcacaf71427a85

// DIRECTOR 2: EojEdred (85.239.239.194) - ✅ SYNCED
0x0a9442f63cd6019b8d6f0cd2dd6cc84d302d8eeb616bb12d7f439172107dbd2b

// DIRECTOR 3: governance-dev01 (80.190.82.186) - 🔄 SYNCING
0x8a9a9d8a9574eb75682a3501a2df5467036c2fc03903e9d46dfab77af4189a51

// DIRECTOR 4: security-dev01 (85.239.239.190) - 🔄 SYNCING
0xf0b569725433cdeac5017dcaf9591a3d9f2fa74e40347bc8f0365a9658acfb82

// DIRECTOR 5: audit-dev01 (129.80.122.34) - ✅ SYNCED
0xb09a291c0303bd66b523e44203d582f69cd192e6cdbff8329d1b01d049f4bb96

// DIRECTOR 6: consensus-dev01 (85.239.239.189) - ✅ OPERATIONAL
0xdc4357a4d93f0599b616159278d8ce281e19685c8dd0d40d5960a58d8eeda3b8

// DIRECTOR 7: runtime-dev01 (85.239.239.193) - 🔄 SYNCING
0x2975859973decf0c53296d425ade75b28ab3ab10ea3c2d1b78170c8885090a0d

// DIRECTOR 8: oracle-dev01 (85.239.239.189) - ✅ OPERATIONAL
0xee9d4f38c8b3757c9604db672ef1309fa740adf3d3c989a2baa219fc70d4f115

// DIRECTOR 9: compliance-dev (154.12.250.18) - 🔄 SYNCING
0x7cfe2e4e8e95406cf6f31fe4c7ed7e3f9ab10d3f52c52f1e4b65f6f0e7e25a43
```

**Format for Runtime:** Each authority needs `(AuthorityId, weight)` tuple where weight = 1

---

## Migration Strategy

### Option 1: Storage Migration Module (RECOMMENDED)

Create a new file: `~/etrid/05-multichain/flare-chain/runtime/src/migrations.rs`

```rust
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_std::vec::Vec;

pub struct InitializeGrandpaAuthorities;

impl OnRuntimeUpgrade for InitializeGrandpaAuthorities {
    fn on_runtime_upgrade() -> Weight {
        log::info!("🔧 Migration: Initializing GRANDPA authorities for spec 105");

        // The 9 Director GRANDPA keys
        let authorities: Vec<(GrandpaId, u64)> = vec![
            // DIRECTOR 1: Gizzi
            (
                sp_core::sr25519::Public::from_raw([
                    0x00, 0xee, 0x75, 0xf5, 0xf1, 0xfd, 0xf6, 0x47,
                    0x00, 0x6e, 0x74, 0x08, 0xc5, 0xe9, 0xc7, 0xca,
                    0x98, 0xaf, 0xcb, 0x2f, 0xcd, 0x9a, 0xe6, 0x65,
                    0x03, 0xdc, 0xac, 0xaf, 0x71, 0x42, 0x7a, 0x85,
                ]).into(),
                1u64
            ),
            // DIRECTOR 2: EojEdred
            (
                sp_core::sr25519::Public::from_raw([
                    0x0a, 0x94, 0x42, 0xf6, 0x3c, 0xd6, 0x01, 0x9b,
                    0x8d, 0x6f, 0x0c, 0xd2, 0xdd, 0x6c, 0xc8, 0x4d,
                    0x30, 0x2d, 0x8e, 0xeb, 0x61, 0x6b, 0xb1, 0x2d,
                    0x7f, 0x43, 0x91, 0x72, 0x10, 0x7d, 0xbd, 0x2b,
                ]).into(),
                1u64
            ),
            // DIRECTOR 3: governance-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0x8a, 0x9a, 0x9d, 0x8a, 0x95, 0x74, 0xeb, 0x75,
                    0x68, 0x2a, 0x35, 0x01, 0xa2, 0xdf, 0x54, 0x67,
                    0x03, 0x6c, 0x2f, 0xc0, 0x39, 0x03, 0xe9, 0xd4,
                    0x6d, 0xfa, 0xb7, 0x7a, 0xf4, 0x18, 0x9a, 0x51,
                ]).into(),
                1u64
            ),
            // DIRECTOR 4: security-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0xf0, 0xb5, 0x69, 0x72, 0x54, 0x33, 0xcd, 0xea,
                    0xc5, 0x01, 0x7d, 0xca, 0xf9, 0x59, 0x1a, 0x3d,
                    0x9f, 0x2f, 0xa7, 0x4e, 0x40, 0x34, 0x7b, 0xc8,
                    0xf0, 0x36, 0x5a, 0x96, 0x58, 0xac, 0xfb, 0x82,
                ]).into(),
                1u64
            ),
            // DIRECTOR 5: audit-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0xb0, 0x9a, 0x29, 0x1c, 0x03, 0x03, 0xbd, 0x66,
                    0xb5, 0x23, 0xe4, 0x42, 0x03, 0xd5, 0x82, 0xf6,
                    0x9c, 0xd1, 0x92, 0xe6, 0xcd, 0xbf, 0xf8, 0x32,
                    0x9d, 0x1b, 0x01, 0xd0, 0x49, 0xf4, 0xbb, 0x96,
                ]).into(),
                1u64
            ),
            // DIRECTOR 6: consensus-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0xdc, 0x43, 0x57, 0xa4, 0xd9, 0x3f, 0x05, 0x99,
                    0xb6, 0x16, 0x15, 0x92, 0x78, 0xd8, 0xce, 0x28,
                    0x1e, 0x19, 0x68, 0x5c, 0x8d, 0xd0, 0xd4, 0x0d,
                    0x59, 0x60, 0xa5, 0x8d, 0x8e, 0xed, 0xa3, 0xb8,
                ]).into(),
                1u64
            ),
            // DIRECTOR 7: runtime-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0x29, 0x75, 0x85, 0x99, 0x73, 0xde, 0xcf, 0x0c,
                    0x53, 0x29, 0x6d, 0x42, 0x5a, 0xde, 0x75, 0xb2,
                    0x8a, 0xb3, 0xab, 0x10, 0xea, 0x3c, 0x2d, 0x1b,
                    0x78, 0x17, 0x0c, 0x88, 0x85, 0x09, 0x0a, 0x0d,
                ]).into(),
                1u64
            ),
            // DIRECTOR 8: oracle-dev01
            (
                sp_core::sr25519::Public::from_raw([
                    0xee, 0x9d, 0x4f, 0x38, 0xc8, 0xb3, 0x75, 0x7c,
                    0x96, 0x04, 0xdb, 0x67, 0x2e, 0xf1, 0x30, 0x9f,
                    0xa7, 0x40, 0xad, 0xf3, 0xd3, 0xc9, 0x89, 0xa2,
                    0xba, 0xa2, 0x19, 0xfc, 0x70, 0xd4, 0xf1, 0x15,
                ]).into(),
                1u64
            ),
            // DIRECTOR 9: compliance-dev
            (
                sp_core::sr25519::Public::from_raw([
                    0x7c, 0xfe, 0x2e, 0x4e, 0x8e, 0x95, 0x40, 0x6c,
                    0xf6, 0xf3, 0x1f, 0xe4, 0xc7, 0xed, 0x7e, 0x3f,
                    0x9a, 0xb1, 0x0d, 0x3f, 0x52, 0xc5, 0x2f, 0x1e,
                    0x4b, 0x65, 0xf6, 0xf0, 0xe7, 0xe2, 0x5a, 0x43,
                ]).into(),
                1u64
            ),
        ];

        // Initialize GRANDPA authorities storage
        pallet_grandpa::Authorities::<crate::Runtime>::put(
            sp_runtime::BoundedVec::try_from(authorities)
                .expect("9 authorities is within MaxAuthorities=32")
        );

        // Set to SetId 0 (first set)
        pallet_grandpa::CurrentSetId::<crate::Runtime>::put(0u64);

        log::info!("✅ GRANDPA authorities initialized: 9 Directors");

        // Weight: 2 storage writes (Authorities + SetId)
        Weight::from_parts(10_000_000, 0)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
        let current = pallet_grandpa::Authorities::<crate::Runtime>::get();
        log::info!("Pre-upgrade: Current authorities count = {}", current.len());
        Ok(Vec::new())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
        let new_authorities = pallet_grandpa::Authorities::<crate::Runtime>::get();
        assert_eq!(new_authorities.len(), 9, "Should have 9 authorities");
        log::info!("✅ Post-upgrade: {} authorities set", new_authorities.len());
        Ok(())
    }
}
```

### Modifications to lib.rs

**1. Add migrations module (after line 51):**
```rust
pub mod opaque {
    use super::*;
    // ... existing code ...
}

// Add this:
mod migrations;
```

**2. Update spec_version (line 77):**
```rust
spec_version: 105,  // Changed from 104
```

**3. Add migration to Executive (around line 1156):**
```rust
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::InitializeGrandpaAuthorities,  // Add this migration
>;
```

---

## Build Process

### Build Command (on Gizzi VM)

```bash
# SSH to Gizzi
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Navigate to runtime
cd ~/etrid/05-multichain/flare-chain/runtime

# Ensure cargo environment is loaded
source ~/.cargo/env

# Clean previous build
cargo clean

# Build WASM runtime
cargo build --release --features=std

# The WASM blob will be located at:
# ~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm
```

**Expected Output Location:**
```
~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/
├── flare_chain_runtime.compact.compressed.wasm   # USE THIS
├── flare_chain_runtime.compact.wasm
└── flare_chain_runtime.wasm
```

**File to Deploy:** `flare_chain_runtime.compact.compressed.wasm` (~2-3 MB)

### Verification Before Deployment

```bash
# Check WASM was built
ls -lh ~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm

# Get file hash
sha256sum ~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm

# Copy to Desktop for upload
scp -i ~/.ssh/gizzi-validator \
  ubuntu@64.181.215.19:~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm \
  /Users/macbook/Desktop/flare_chain_runtime_v105.wasm
```

---

## Deployment Process

### Using Polkadot.js Apps

1. **Navigate to Extrinsics:**
   - Go to: https://polkadot.js.org/apps/?rpc=ws://64.181.215.19:9944#/extrinsics
   - Or local: Developer → Extrinsics

2. **Submit Runtime Upgrade:**
   - **Account:** Select Sudo account (Foundation multisig or current sudo)
   - **Extrinsic:** `sudo` → `sudoUncheckedWeight`
   - **call:** `system` → `setCode`
   - **code:** Upload `flare_chain_runtime_v105.wasm`
   - **weight:**
     - `refTime:` `1,000,000,000,000`
     - `proofSize:` `0`

3. **Sign and Submit:**
   - Review transaction
   - Sign with sudo key
   - Wait for inclusion in block

4. **Verify Migration:**
   ```bash
   # Check new spec version
   curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' \
     http://64.181.215.19:9944

   # Should show spec_version: 105

   # Check GRANDPA authorities (storage key: Grandpa::Authorities)
   curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params":["GrandpaApi_grandpa_authorities", "0x"]}' \
     http://64.181.215.19:9944

   # Should return 9 authorities with their keys
   ```

5. **Monitor Finality:**
   - Watch node logs: `journalctl -fu flarechain-node`
   - Check for GRANDPA finalization messages
   - Finality should resume once 6/9 Directors are synced and participating

---

## Alternative: Manual Storage Insertion (NOT RECOMMENDED)

If migration doesn't work, authorities could be manually inserted via RPC:

```bash
# Would require 9 separate author_insertKey calls
# NOT RECOMMENDED - storage migration is cleaner
```

---

## Expected Timeline

### Phase 1: Runtime Build (30-60 minutes)
- Create migrations.rs file
- Modify lib.rs (3 changes)
- Build WASM runtime
- Verify build successful

### Phase 2: Deployment (5-10 minutes)
- Upload WASM to Polkadot.js Apps
- Submit sudo.sudoUncheckedWeight transaction
- Wait for block inclusion
- Verify spec_version = 105

### Phase 3: Migration Execution (Automatic)
- Migration runs on first block of new runtime
- GRANDPA authorities storage initialized
- SetId set to 0

### Phase 4: Finality Resumption (Hours to Days)
- Requires 6/9 Directors synced to block #76,401+
- Currently: 3/9 synced (need 3 more)
- Remaining Directors are syncing from genesis
- Finality will resume automatically when threshold reached

---

## Rollback Plan

If runtime upgrade fails:

1. **Quick Rollback:**
   ```bash
   # Re-deploy spec 104 WASM if available
   # OR build from last known-good commit
   ```

2. **Recovery:**
   - Node will continue producing blocks (AURA unaffected)
   - Only finality is impacted
   - Can re-attempt migration with fixes

---

## Testing Recommendations

### Before Mainnet Deployment

1. **Local Test:**
   - Spin up local 2-validator chain
   - Test migration on local chain
   - Verify GRANDPA authorities set correctly

2. **Testnet Deployment:**
   - Deploy to Ember Testnet first if available
   - Monitor for issues
   - Verify finality works

3. **Mainnet Staging:**
   - Build WASM on Gizzi VM
   - Get hash verification
   - Review with team before deployment

---

## Success Criteria

✅ **Runtime Upgrade Successful:**
- spec_version = 105
- No errors in node logs
- Chain continues producing blocks

✅ **Migration Successful:**
- `Grandpa::Authorities` storage contains 9 entries
- `Grandpa::CurrentSetId` = 0
- No migration errors in logs

✅ **Finality Restored:**
- GRANDPA rounds progressing
- Finalized block number increasing
- 6/9 Directors participating in finality

---

## Next Steps

**DO NOT PROCEED** until explicitly authorized. This plan is for review only.

When ready to execute:

1. ✅ Review this plan
2. ✅ Backup current chain state
3. ✅ Create migrations.rs
4. ✅ Modify lib.rs (3 changes)
5. ✅ Build WASM
6. ✅ Test locally
7. ✅ Deploy to mainnet
8. ✅ Monitor and verify

---

## Files to Modify

### New File: `/home/ubuntu/etrid/05-multichain/flare-chain/runtime/src/migrations.rs`
- 150 lines
- Contains `InitializeGrandpaAuthorities` migration

### Modified File: `/home/ubuntu/etrid/05-multichain/flare-chain/runtime/src/lib.rs`
- Line 52: Add `mod migrations;`
- Line 77: Change `spec_version: 104` → `105`
- Line ~1156: Add migration to Executive

**Total Changes:** 1 new file + 3 line modifications

---

## Contact Information

**Gizzi VM:** ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
**RPC Endpoint:** ws://64.181.215.19:9944
**Polkadot.js:** https://polkadot.js.org/apps/?rpc=ws://64.181.215.19:9944

**Current Status:**
- Finality: Stuck at #63,274
- Best Block: #76,401
- Directors Synced: 3/9
- GRANDPA Authorities: 0 (CRITICAL BUG)

---

**End of Plan - Awaiting Authorization to Proceed**
