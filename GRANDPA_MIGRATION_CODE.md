# GRANDPA Migration Code - Ready to Deploy

**Quick Reference for Runtime Upgrade to Spec 105**

---

## File 1: Create migrations.rs

**Location:** `/home/ubuntu/etrid/05-multichain/flare-chain/runtime/src/migrations.rs`

**Action:** Create new file with this exact content:

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
            // DIRECTOR 1: Gizzi (64.181.215.19)
            (
                sp_core::sr25519::Public::from_raw([
                    0x00, 0xee, 0x75, 0xf5, 0xf1, 0xfd, 0xf6, 0x47,
                    0x00, 0x6e, 0x74, 0x08, 0xc5, 0xe9, 0xc7, 0xca,
                    0x98, 0xaf, 0xcb, 0x2f, 0xcd, 0x9a, 0xe6, 0x65,
                    0x03, 0xdc, 0xac, 0xaf, 0x71, 0x42, 0x7a, 0x85,
                ]).into(),
                1u64
            ),
            // DIRECTOR 2: EojEdred (85.239.239.194)
            (
                sp_core::sr25519::Public::from_raw([
                    0x0a, 0x94, 0x42, 0xf6, 0x3c, 0xd6, 0x01, 0x9b,
                    0x8d, 0x6f, 0x0c, 0xd2, 0xdd, 0x6c, 0xc8, 0x4d,
                    0x30, 0x2d, 0x8e, 0xeb, 0x61, 0x6b, 0xb1, 0x2d,
                    0x7f, 0x43, 0x91, 0x72, 0x10, 0x7d, 0xbd, 0x2b,
                ]).into(),
                1u64
            ),
            // DIRECTOR 3: governance-dev01 (80.190.82.186)
            (
                sp_core::sr25519::Public::from_raw([
                    0x8a, 0x9a, 0x9d, 0x8a, 0x95, 0x74, 0xeb, 0x75,
                    0x68, 0x2a, 0x35, 0x01, 0xa2, 0xdf, 0x54, 0x67,
                    0x03, 0x6c, 0x2f, 0xc0, 0x39, 0x03, 0xe9, 0xd4,
                    0x6d, 0xfa, 0xb7, 0x7a, 0xf4, 0x18, 0x9a, 0x51,
                ]).into(),
                1u64
            ),
            // DIRECTOR 4: security-dev01 (85.239.239.190)
            (
                sp_core::sr25519::Public::from_raw([
                    0xf0, 0xb5, 0x69, 0x72, 0x54, 0x33, 0xcd, 0xea,
                    0xc5, 0x01, 0x7d, 0xca, 0xf9, 0x59, 0x1a, 0x3d,
                    0x9f, 0x2f, 0xa7, 0x4e, 0x40, 0x34, 0x7b, 0xc8,
                    0xf0, 0x36, 0x5a, 0x96, 0x58, 0xac, 0xfb, 0x82,
                ]).into(),
                1u64
            ),
            // DIRECTOR 5: audit-dev01 (129.80.122.34)
            (
                sp_core::sr25519::Public::from_raw([
                    0xb0, 0x9a, 0x29, 0x1c, 0x03, 0x03, 0xbd, 0x66,
                    0xb5, 0x23, 0xe4, 0x42, 0x03, 0xd5, 0x82, 0xf6,
                    0x9c, 0xd1, 0x92, 0xe6, 0xcd, 0xbf, 0xf8, 0x32,
                    0x9d, 0x1b, 0x01, 0xd0, 0x49, 0xf4, 0xbb, 0x96,
                ]).into(),
                1u64
            ),
            // DIRECTOR 6: consensus-dev01 (85.239.239.189)
            (
                sp_core::sr25519::Public::from_raw([
                    0xdc, 0x43, 0x57, 0xa4, 0xd9, 0x3f, 0x05, 0x99,
                    0xb6, 0x16, 0x15, 0x92, 0x78, 0xd8, 0xce, 0x28,
                    0x1e, 0x19, 0x68, 0x5c, 0x8d, 0xd0, 0xd4, 0x0d,
                    0x59, 0x60, 0xa5, 0x8d, 0x8e, 0xed, 0xa3, 0xb8,
                ]).into(),
                1u64
            ),
            // DIRECTOR 7: runtime-dev01 (85.239.239.193)
            (
                sp_core::sr25519::Public::from_raw([
                    0x29, 0x75, 0x85, 0x99, 0x73, 0xde, 0xcf, 0x0c,
                    0x53, 0x29, 0x6d, 0x42, 0x5a, 0xde, 0x75, 0xb2,
                    0x8a, 0xb3, 0xab, 0x10, 0xea, 0x3c, 0x2d, 0x1b,
                    0x78, 0x17, 0x0c, 0x88, 0x85, 0x09, 0x0a, 0x0d,
                ]).into(),
                1u64
            ),
            // DIRECTOR 8: oracle-dev01 (85.239.239.189)
            (
                sp_core::sr25519::Public::from_raw([
                    0xee, 0x9d, 0x4f, 0x38, 0xc8, 0xb3, 0x75, 0x7c,
                    0x96, 0x04, 0xdb, 0x67, 0x2e, 0xf1, 0x30, 0x9f,
                    0xa7, 0x40, 0xad, 0xf3, 0xd3, 0xc9, 0x89, 0xa2,
                    0xba, 0xa2, 0x19, 0xfc, 0x70, 0xd4, 0xf1, 0x15,
                ]).into(),
                1u64
            ),
            // DIRECTOR 9: compliance-dev (154.12.250.18)
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

---

## File 2: Modify lib.rs

**Location:** `/home/ubuntu/etrid/05-multichain/flare-chain/runtime/src/lib.rs`

### Change 1: Add migrations module (after line 51)

**Find this block:**
```rust
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    // ... rest of opaque module ...
}
```

**Add immediately after the closing brace:**
```rust
pub mod opaque {
    // ... existing code ...
}

// ============= ADD THIS LINE =============
mod migrations;
// ========================================
```

### Change 2: Update spec_version (line 77)

**Find:**
```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 104,  // <-- CHANGE THIS
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};
```

**Change to:**
```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid"),
    impl_name: create_runtime_str!("etrid"),
    authoring_version: 1,
    spec_version: 105,  // <-- CHANGED FROM 104
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};
```

### Change 3: Add migration to Executive (around line 1156)

**Find:**
```rust
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
```

**Change to:**
```rust
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    migrations::InitializeGrandpaAuthorities,  // <-- ADD THIS LINE
>;
```

---

## Build Commands

```bash
# SSH to Gizzi VM
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Navigate to runtime
cd ~/etrid/05-multichain/flare-chain/runtime

# Load cargo environment
source ~/.cargo/env

# Clean previous builds
cargo clean

# Build runtime WASM
cargo build --release --features=std

# Verify WASM was created
ls -lh ../target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm

# Get SHA256 hash for verification
sha256sum ../target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm
```

---

## Deployment via Polkadot.js Apps

**URL:** https://polkadot.js.org/apps/?rpc=ws://64.181.215.19:9944#/extrinsics

**Steps:**

1. **Copy WASM to Desktop:**
   ```bash
   scp -i ~/.ssh/gizzi-validator \
     ubuntu@64.181.215.19:~/etrid/05-multichain/flare-chain/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm \
     /Users/macbook/Desktop/flare_chain_runtime_v105.wasm
   ```

2. **In Polkadot.js Apps:**
   - Go to: Developer → Extrinsics
   - Account: Sudo account
   - Submit the following extrinsic:
     - `sudo.sudoUncheckedWeight`
       - call: `system.setCode`
         - code: [Upload flare_chain_runtime_v105.wasm]
       - weight:
         - refTime: `1000000000000`
         - proofSize: `0`
   - Sign and Submit

3. **Verify:**
   ```bash
   # Check runtime version
   curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' \
     http://64.181.215.19:9944

   # Should return spec_version: 105
   ```

---

## Verification Commands

### Check GRANDPA Authorities After Upgrade

```bash
# Method 1: Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params":["GrandpaApi_grandpa_authorities", "0x"]}' \
  http://64.181.215.19:9944

# Method 2: Check storage directly
# Storage key for Grandpa::Authorities
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params":["0x5f9cc45b7a00c5899361e1c6099678dc8a2d09463effcc78a22d75b9cb87dffc"]}' \
  http://64.181.215.19:9944

# Should NOT return 0x0000000000000000 anymore
# Should return encoded data with 9 authority public keys
```

### Monitor Node Logs

```bash
# SSH to Gizzi
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Watch logs
journalctl -fu flarechain-node

# Look for:
# - "🔧 Migration: Initializing GRANDPA authorities for spec 105"
# - "✅ GRANDPA authorities initialized: 9 Directors"
# - GRANDPA finality messages
```

---

## Summary

**Files to Create/Modify:**
1. ✅ Create: `runtime/src/migrations.rs` (new file, 150 lines)
2. ✅ Modify: `runtime/src/lib.rs` (3 changes)
   - Add `mod migrations;` after line 51
   - Change spec_version from 104 → 105 (line 77)
   - Add migration to Executive (line ~1156)

**Build Output:**
- WASM blob: `flare_chain_runtime.compact.compressed.wasm`
- Size: ~2-3 MB
- Location: `target/release/wbuild/flare-chain-runtime/`

**Deployment:**
- Via sudo.sudoUncheckedWeight → system.setCode
- Upload WASM file
- Migration runs automatically on first block

**Expected Result:**
- GRANDPA authorities: 9 Directors set
- Finality: Will resume when 6/9 Directors are synced
- Currently: 3/9 synced, need 3 more

---

**Ready to Execute - Awaiting Your Authorization**
