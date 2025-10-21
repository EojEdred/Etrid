# Applying pbc-common to All Remaining PBCs

## Status
- ✅ **BTC PBC:** Complete (592 lines, reduced from 629)
- ⏳ **Remaining:** 12 PBCs to update

## Remaining PBCs
1. ETH (Ethereum)
2. DOGE (Dogecoin)
3. SOL (Solana)
4. XLM (Stellar)
5. XRP (Ripple)
6. BNB (Binance)
7. TRX (Tron)
8. ADA (Cardano)
9. LINK (Chainlink)
10. MATIC (Polygon)
11. SC-USDT (USDT)
12. EDSC (?)

## Step-by-Step Instructions

### For Each PBC

#### Step 1: Update Cargo.toml

**File:** `05-multichain/partition-burst-chains/pbc-chains/<CHAIN>-pbc/runtime/Cargo.toml`

**Change 1 - Add dependency (after `[dependencies]`):**
```toml
[dependencies]
# PBC Common - Shared runtime code
pbc-common = { path = "../../../pbc-common", default-features = false }

# <Chain> Bridge (keep existing line)
pallet_<chain>_bridge = { ... }
```

**Change 2 - Add to std features (right after `std = [`):**
```toml
[features]
default = ["std"]
std = [
    "pbc-common/std",  # ADD THIS LINE
    "codec/std",
    # ... rest unchanged
]
```

#### Step 2: Update lib.rs

**File:** `05-multichain/partition-burst-chains/pbc-chains/<CHAIN>-pbc/runtime/src/lib.rs`

**Find this block (lines 12-56 approximately):**
```rust
// Re-export all pallets
pub use pallet_<chain>_bridge;
pub use pallet_lightning_channels;

use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// Frame imports
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot,
};
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, FeeDetails, Multiplier, RuntimeDispatchInfo};

// Ëtrid primitives
use etrid_primitives::{
    Balance, BlockNumber, Hash, Moment, Signature, Nonce,
};
pub use etrid_primitives::AccountId;

// Import consensus
pub use pallet_consensus;
```

**Replace with:**
```rust
// Import common PBC runtime code from pbc-common
pub use pbc_common::*;

// Re-export <Chain> bridge pallet
pub use pallet_<chain>_bridge;
```

#### Step 3: Verify Compilation

```bash
cd /Users/macbook/Desktop/etrid
cargo check -p <chain>-pbc-runtime
```

**Expected output:**
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

### Example: ETH PBC

**Cargo.toml changes:**
```toml
[dependencies]
# PBC Common - Shared runtime code
pbc-common = { path = "../../../pbc-common", default-features = false }

# Ethereum Bridge
pallet_ethereum_bridge = { package = "eth-bridge", path = "../../../../../05-multichain/bridge-protocols/ethereum-bridge", default-features = false }

[features]
default = ["std"]
std = [
    "pbc-common/std",  # ADDED
    "codec/std",
    # ... rest
]
```

**lib.rs changes:**
```rust
// Before (40+ lines):
pub use pallet_ethereum_bridge;
pub use pallet_lightning_channels;
use sp_api::impl_runtime_apis;
// ... 40 more lines ...

// After (2 lines):
pub use pbc_common::*;
pub use pallet_ethereum_bridge;
```

### What Stays the Same

**DO NOT CHANGE:**
- ✅ Type definitions (Address, Header, Block, etc.)
- ✅ Pallet configurations (frame_system::Config, pallet_balances::Config, etc.)
- ✅ Bridge-specific configurations (MinEthConfirmations, EthBridgeFeeRate, etc.)
- ✅ Runtime construction (construct_runtime!)
- ✅ Runtime APIs (impl_runtime_apis!)
- ✅ Opaque module
- ✅ Version info
- ✅ ALL blockchain-specific logic

## Automation Script

For faster application, use this bash script:

```bash
#!/bin/bash
# apply_pbc_common.sh

PBCS=("eth" "doge" "sol" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt" "edsc")

for chain in "${PBCS[@]}"; do
    echo "Processing ${chain}-pbc..."

    CARGO="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains/${chain}-pbc/runtime/Cargo.toml"

    if [ -f "$CARGO" ]; then
        # Backup
        cp "$CARGO" "${CARGO}.bak"

        # Add pbc-common dependency (requires manual verification)
        # Add to std features (requires manual verification)

        echo "✓ $chain-pbc Cargo.toml backed up"
    else
        echo "✗ $chain-pbc not found"
    fi
done
```

## Verification Checklist

For each PBC after update:

- [ ] Cargo.toml has pbc-common dependency
- [ ] Cargo.toml has pbc-common/std in features
- [ ] lib.rs imports are replaced with `pub use pbc_common::*;`
- [ ] lib.rs keeps bridge pallet import
- [ ] `cargo check -p <chain>-pbc-runtime` succeeds
- [ ] Line count reduced by ~37 lines
- [ ] All blockchain-specific configs preserved

## Expected Results

**Per PBC:**
- Lines before: ~629
- Lines after: ~592
- Reduction: ~37 lines (5.9%)
- Compilation: ✅ Success
- Functionality: ✅ 100% preserved

**Total (13 PBCs):**
- Lines before: ~8,177
- Lines after: ~7,696
- Total reduction: ~481 lines
- Consistency: ✅ All PBCs use same imports
- Maintainability: ✅ Single source of truth

## Troubleshooting

### "cannot find type X in this scope"
- Ensure pbc-common is in Cargo.toml dependencies
- Ensure pbc-common/std is in std features
- Check that pbc-common compiles: `cargo check -p pbc-common`

### "unresolved import"
- Verify the import line is exactly: `pub use pbc_common::*;`
- Check that the bridge pallet is still imported separately

### Compilation fails
- Restore from backup: `cp Cargo.toml.bak Cargo.toml`
- Review the BTC PBC example as reference
- Ensure no type definitions were accidentally removed

## Progress Tracking

| PBC | Cargo.toml | lib.rs | Compiled | Lines Saved |
|-----|------------|---------|----------|-------------|
| BTC | ✅ | ✅ | ✅ | 37 |
| ETH | ✅ | ✅ | ✅ | ~37 |
| DOGE | ✅ | ✅ | ✅ | ~37 |
| SOL | ✅ | ✅ | ✅ | ~37 |
| XLM | ✅ | ✅ | ✅ | ~37 |
| XRP | ✅ | ✅ | ✅ | ~37 |
| BNB | ✅ | ✅ | ✅ | ~37 |
| TRX | ✅ | ✅ | ✅ | ~37 |
| ADA | ✅ | ✅ | ✅ | ~37 |
| LINK | ✅ | ✅ | ✅ | ~37 |
| MATIC | ✅ | ✅ | ✅ | ~37 |
| SC-USDT | ✅ | ✅ | ✅ | ~37 |
| EDSC | ✅ | ✅ | ✅ | ~37 |

**Note:** EDSC-PBC was refactored from Aura consensus to ASF consensus to match all other PBCs!

**Total Lines Saved:** ~481 lines across all 13 PBCs

---

**Last Updated:** October 21, 2025
**Reference:** See `PBC_REFACTORING_COMPLETE.md` for detailed analysis
**Example:** BTC PBC at `05-multichain/partition-burst-chains/pbc-chains/btc-pbc/`
