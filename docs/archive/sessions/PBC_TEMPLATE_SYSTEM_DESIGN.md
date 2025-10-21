# PBC Template System - Architecture Design

**Date:** October 20, 2025
**Version:** 1.0
**Status:** Design Phase

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Component Design](#component-design)
4. [Configuration Format](#configuration-format)
5. [Code Generation](#code-generation)
6. [Migration Strategy](#migration-strategy)
7. [Implementation Plan](#implementation-plan)

---

## Overview

### Goals

1. **Eliminate 92.6% code duplication** across 13 PBCs
2. **Simplify PBC creation** - new PBC in 5 minutes
3. **Ensure consistency** - shared code guarantees uniformity
4. **Improve maintainability** - changes in one place
5. **Faster builds** - shared compilation

### Principles

- **DRY (Don't Repeat Yourself)** - Share all common code
- **Configuration over Code** - PBC-specific details in TOML
- **Type Safety** - Leverage Rust's type system
- **Backward Compatible** - Preserve functionality
- **Well Documented** - Clear examples and guides

---

## Architecture

### High-Level Structure

```
05-multichain/partition-burst-chains/
│
├── pbc-common/                         # Shared runtime code
│   ├── Cargo.toml
│   ├── build.rs
│   └── src/
│       ├── lib.rs                      # Main exports
│       ├── types.rs                    # Common types
│       ├── config.rs                   # Pallet configurations
│       ├── runtime.rs                  # Runtime construction helpers
│       └── apis.rs                     # Runtime API implementations
│
├── pbc-template/                       # Template system
│   ├── config/                         # PBC configurations
│   │   ├── btc.toml
│   │   ├── eth.toml
│   │   ├── doge.toml
│   │   └── ... (13 total)
│   ├── src/
│   │   ├── generator.rs                # Code generator
│   │   ├── parser.rs                   # TOML parser
│   │   └── validator.rs                # Config validator
│   ├── templates/
│   │   ├── lib.rs.hbs                  # Handlebars template
│   │   └── Cargo.toml.hbs              # Cargo template
│   └── scripts/
│       ├── generate_pbc.sh             # Generator script
│       └── generate_all.sh             # Batch generation
│
└── pbc-chains/                         # Individual PBCs
    ├── btc-pbc/
    │   ├── runtime/
    │   │   ├── Cargo.toml              # Minimal dependencies
    │   │   ├── build.rs                # Standard build
    │   │   └── src/
    │   │       └── lib.rs              # ~50 lines (bridge config)
    │   └── node/                       # Node implementation
    └── ... (13 PBCs)
```

### Data Flow

```
┌─────────────┐
│ btc.toml    │  Configuration file
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  Generator  │  Parse & validate config
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  Template   │  Apply to Handlebars template
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  lib.rs     │  Generated PBC runtime
│  Cargo.toml │
└─────────────┘
```

---

## Component Design

### 1. pbc-common Crate

**Purpose:** Contain all shared runtime code (580 lines of common code)

#### Module Structure

**lib.rs** - Main exports
```rust
//! Common runtime code for all PBCs

pub mod types;
pub mod config;
pub mod runtime;
pub mod apis;

// Re-exports
pub use types::*;
pub use config::*;
pub use runtime::*;

// Common imports all PBCs need
pub use sp_api::impl_runtime_apis;
pub use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
pub use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT},
};
// ... etc
```

**types.rs** - Common type definitions
```rust
//! Common type definitions for PBC runtimes

use sp_runtime::{generic, MultiSignature};
use etrid_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce};

/// Address format
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Block header type
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// Signed block
pub type SignedBlock = generic::SignedBlock<Block>;

/// Block ID
pub type BlockId = generic::BlockId<Block>;

/// Signed extensions
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Signed payload
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

/// Executive type
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
```

**config.rs** - Common pallet configurations
```rust
//! Common pallet configurations

use frame_support::{parameter_types, traits::{ConstU32, ConstU64, ConstU128}};

// Common parameter types
parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const SS58Prefix: u8 = 42;
    pub const ExistentialDeposit: Balance = 1_000_000_000; // 0.001 ÉTR
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

/// Macro to generate common pallet configs
#[macro_export]
macro_rules! impl_common_configs {
    ($runtime:ident) => {
        impl frame_system::Config for $runtime {
            type BaseCallFilter = frame_support::traits::Everything;
            type BlockWeights = BlockWeights;
            type BlockLength = BlockLength;
            type RuntimeOrigin = RuntimeOrigin;
            type RuntimeCall = RuntimeCall;
            type Nonce = Nonce;
            type Hash = Hash;
            type Hashing = BlakeTwo256;
            type AccountId = AccountId;
            type Lookup = AccountIdLookup<AccountId, ()>;
            type Block = Block;
            type RuntimeEvent = RuntimeEvent;
            type BlockHashCount = BlockHashCount;
            type DbWeight = RocksDbWeight;
            type Version = Version;
            type PalletInfo = PalletInfo;
            type AccountData = pallet_balances::AccountData<Balance>;
            type OnNewAccount = ();
            type OnKilledAccount = ();
            type SystemWeightInfo = ();
            type SS58Prefix = SS58Prefix;
            type OnSetCode = ();
            type MaxConsumers = ConstU32<16>;
        }

        impl pallet_timestamp::Config for $runtime {
            type Moment = Moment;
            type OnTimestampSet = Aura;
            type MinimumPeriod = ConstU64<3000>;
            type WeightInfo = ();
        }

        impl pallet_balances::Config for $runtime {
            type MaxLocks = MaxLocks;
            type MaxReserves = MaxReserves;
            type ReserveIdentifier = [u8; 8];
            type Balance = Balance;
            type RuntimeEvent = RuntimeEvent;
            type DustRemoval = ();
            type ExistentialDeposit = ExistentialDeposit;
            type AccountStore = System;
            type WeightInfo = ();
            type RuntimeHoldReason = ();
            type FreezeIdentifier = ();
            type MaxHolds = ConstU32<0>;
            type MaxFreezes = ConstU32<0>;
        }

        impl pallet_transaction_payment::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
            type OperationalFeeMultiplier = ConstU8<5>;
            type WeightToFee = IdentityFee<Balance>;
            type LengthToFee = IdentityFee<Balance>;
            type FeeMultiplierUpdate = ConstFeeMultiplier<Multiplier>;
        }

        impl pallet_sudo::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type WeightInfo = ();
        }

        impl pallet_aura::Config for $runtime {
            type AuthorityId = AuraId;
            type DisabledValidators = ();
            type MaxAuthorities = ConstU32<32>;
            type AllowMultipleBlocksPerSlot = ConstBool<false>;
        }

        impl pallet_grandpa::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = ();
            type MaxAuthorities = ConstU32<32>;
            type MaxNominators = ConstU32<0>;
            type MaxSetIdSessionEntries = ConstU64<0>;
            type KeyOwnerProof = sp_core::Void;
            type EquivocationReportSystem = ();
        }

        impl pallet_consensus::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type MaxValidators = ConstU32<100>;
            type MinStake = ConstU128<64_000_000_000_000>; // 64 ÉTR
            type WeightInfo = ();
        }

        impl pallet_lightning_channels::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type MaxChannelsPerAccount = ConstU32<100>;
            type ChannelDeposit = ConstU128<1_000_000_000_000>; // 0.001 ÉTR
            type WeightInfo = ();
        }
    };
}
```

**runtime.rs** - Runtime construction helpers
```rust
//! Runtime construction helpers

/// Macro to construct runtime with common + bridge pallets
#[macro_export]
macro_rules! construct_pbc_runtime {
    (
        $runtime:ident,
        $bridge_pallet_name:ident: $bridge_pallet:path
    ) => {
        frame_support::construct_runtime!(
            pub enum $runtime {
                // Common pallets (always included)
                System: frame_system,
                Timestamp: pallet_timestamp,
                Aura: pallet_aura,
                Grandpa: pallet_grandpa,
                Balances: pallet_balances,
                TransactionPayment: pallet_transaction_payment,
                Sudo: pallet_sudo,
                Consensus: pallet_consensus,
                LightningChannels: pallet_lightning_channels,

                // Bridge pallet (PBC-specific)
                $bridge_pallet_name: $bridge_pallet,
            }
        );
    };
}
```

**apis.rs** - Runtime API implementations
```rust
//! Common runtime API implementations

/// Macro to implement all standard runtime APIs
#[macro_export]
macro_rules! impl_runtime_apis {
    ($runtime:ident) => {
        impl_runtime_apis! {
            impl sp_api::Core<Block> for $runtime {
                fn version() -> RuntimeVersion {
                    VERSION
                }

                fn execute_block(block: Block) {
                    Executive::execute_block(block);
                }

                fn initialize_block(header: &<Block as BlockT>::Header) {
                    Executive::initialize_block(header)
                }
            }

            impl sp_api::Metadata<Block> for $runtime {
                fn metadata() -> OpaqueMetadata {
                    OpaqueMetadata::new(Runtime::metadata().into())
                }

                fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
                    Runtime::metadata_at_version(version)
                }

                fn metadata_versions() -> sp_std::vec::Vec<u32> {
                    Runtime::metadata_versions()
                }
            }

            impl sp_block_builder::BlockBuilder<Block> for $runtime {
                fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
                    Executive::apply_extrinsic(extrinsic)
                }

                fn finalize_block() -> <Block as BlockT>::Header {
                    Executive::finalize_block()
                }

                fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
                    data.create_extrinsics()
                }

                fn check_inherents(
                    block: Block,
                    data: sp_inherents::InherentData,
                ) -> sp_inherents::CheckInherentsResult {
                    data.check_extrinsics(&block)
                }
            }

            impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for $runtime {
                fn validate_transaction(
                    source: TransactionSource,
                    tx: <Block as BlockT>::Extrinsic,
                    block_hash: <Block as BlockT>::Hash,
                ) -> TransactionValidity {
                    Executive::validate_transaction(source, tx, block_hash)
                }
            }

            impl sp_offchain::OffchainWorkerApi<Block> for $runtime {
                fn offchain_worker(header: &<Block as BlockT>::Header) {
                    Executive::offchain_worker(header)
                }
            }

            impl sp_session::SessionKeys<Block> for $runtime {
                fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
                    opaque::SessionKeys::generate(seed)
                }

                fn decode_session_keys(
                    encoded: Vec<u8>,
                ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
                    opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
                }
            }

            impl sp_consensus_aura::AuraApi<Block, AuraId> for $runtime {
                fn slot_duration() -> sp_consensus_aura::SlotDuration {
                    sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
                }

                fn authorities() -> Vec<AuraId> {
                    Aura::authorities().into_inner()
                }
            }

            impl sp_consensus_grandpa::GrandpaApi<Block> for $runtime {
                fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
                    Grandpa::grandpa_authorities()
                }

                fn current_set_id() -> sp_consensus_grandpa::SetId {
                    Grandpa::current_set_id()
                }

                fn submit_report_equivocation_unsigned_extrinsic(
                    _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                        <Block as BlockT>::Hash,
                        NumberFor<Block>,
                    >,
                    _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                ) -> Option<()> {
                    None
                }

                fn generate_key_ownership_proof(
                    _set_id: sp_consensus_grandpa::SetId,
                    _authority_id: GrandpaId,
                ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
                    None
                }
            }

            impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for $runtime {
                fn account_nonce(account: AccountId) -> Nonce {
                    System::account_nonce(account)
                }
            }

            impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for $runtime {
                fn query_info(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> RuntimeDispatchInfo<Balance> {
                    TransactionPayment::query_info(uxt, len)
                }
                fn query_fee_details(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> FeeDetails<Balance> {
                    TransactionPayment::query_fee_details(uxt, len)
                }
                fn query_weight_to_fee(weight: Weight) -> Balance {
                    TransactionPayment::weight_to_fee(weight)
                }
                fn query_length_to_fee(length: u32) -> Balance {
                    TransactionPayment::length_to_fee(length)
                }
            }
        }
    };
}
```

**Cargo.toml**
```toml
[package]
name = "pbc-common"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
scale-info = { version = "2.10.0", default-features = false, features = ["derive"] }

# Substrate
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
frame-executive = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-api = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-core = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-std = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-version = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-block-builder = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-transaction-pool = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-inherents = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-consensus-grandpa = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-offchain = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
sp-session = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }

# Pallets
pallet-aura = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
pallet-balances = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
pallet-grandpa = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
pallet-sudo = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
pallet-timestamp = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }
pallet-transaction-payment = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506", default-features = false }

# Ëtrid
etrid-primitives = { path = "../../../primitives", default-features = false }
pallet-consensus = { path = "../../../09-consensus/pallet", default-features = false }
pallet-lightning-channels = { path = "../../../07-transactions/lightning-bloc/pallet", default-features = false }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "frame-executive/std",
    "sp-api/std",
    "sp-core/std",
    "sp-runtime/std",
    "sp-std/std",
    # ... all other dependencies
]
```

---

## Configuration Format

### PBC Configuration File (TOML)

**btc.toml**
```toml
[pbc]
# Basic PBC information
name = "BTC"
full_name = "Bitcoin"
chain_type = "UTXO"
spec_name = "btc-pbc"
description = "Bitcoin Partition Burst Chain with Lightning Bloc support"

# Version information
spec_version = 100
impl_version = 1
authoring_version = 1
transaction_version = 1

[bridge]
# Bridge pallet information
pallet_name = "BitcoinBridge"
pallet_import = "pallet_bitcoin_bridge"
pallet_path = "05-multichain/bridge-protocols/bitcoin-bridge"

[bridge.params]
# Bridge-specific parameters
min_confirmations = 6
min_deposit_amount = 10_000  # 0.0001 BTC in satoshis
max_deposit_amount = 100_000_000  # 1 BTC in satoshis

[bridge.config]
# Additional bridge config trait implementations
MinConfirmations = "MinBtcConfirmations"
MinDepositAmount = "MinBtcDepositAmount"
MaxDepositAmount = "MaxBtcDepositAmount"
BridgeAuthority = "BridgeAuthorityAccount"

[custom]
# Any custom code snippets (if needed)
additional_imports = []
additional_types = []
additional_config = ""
```

**eth.toml**
```toml
[pbc]
name = "ETH"
full_name = "Ethereum"
chain_type = "Account"
spec_name = "eth-pbc"
description = "Ethereum Partition Burst Chain with Lightning Bloc support"

spec_version = 100
impl_version = 1
authoring_version = 1
transaction_version = 1

[bridge]
pallet_name = "EthereumBridge"
pallet_import = "pallet_ethereum_bridge"
pallet_path = "05-multichain/bridge-protocols/ethereum-bridge"

[bridge.params]
min_confirmations = 12
bridge_fee_rate = 10  # 0.1%
max_gas_limit = 21_000_000
max_deposits_per_account = 100
max_withdrawals_per_account = 50

[bridge.config]
MinConfirmations = "MinEthConfirmations"
BridgeFeeRate = "EthBridgeFeeRate"
MaxGasLimit = "MaxEthGasLimit"
MaxDepositsPerAccount = "MaxEthDepositsPerAccount"
MaxWithdrawalsPerAccount = "MaxEthWithdrawalsPerAccount"
```

---

## Code Generation

### Template (Handlebars)

**lib.rs.hbs**
```rust
//! {{pbc.full_name}} Partition Burst Chain Runtime
//! Generated from template on {{timestamp}}
//!
//! Chain Type: {{pbc.chain_type}}
//! Description: {{pbc.description}}

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// Import common runtime
use pbc_common::*;

// Import bridge pallet
pub use {{bridge.pallet_import}};

// Runtime version
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("{{pbc.spec_name}}"),
    impl_name: create_runtime_str!("{{pbc.spec_name}}-node"),
    authoring_version: {{pbc.authoring_version}},
    spec_version: {{pbc.spec_version}},
    impl_version: {{pbc.impl_version}},
    apis: RUNTIME_API_VERSIONS,
    transaction_version: {{pbc.transaction_version}},
    state_version: 1,
};

// Bridge-specific parameters
parameter_types! {
    {{#each bridge.params}}
    pub const {{@key}}: {{type}} = {{this}};
    {{/each}}
}

// Bridge configuration
impl {{bridge.pallet_import}}::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    {{#each bridge.config}}
    type {{@key}} = {{this}};
    {{/each}}
}

// Apply common configurations
impl_common_configs!(Runtime);

// Construct runtime
construct_pbc_runtime!(
    Runtime,
    {{bridge.pallet_name}}: {{bridge.pallet_import}}
);

// Implement runtime APIs
impl_runtime_apis!(Runtime);
```

### Generator Script

**generate_pbc.sh**
```bash
#!/bin/bash
# PBC Generator Script
# Usage: ./generate_pbc.sh <pbc_name>

PBC_NAME=$1
CONFIG_FILE="config/${PBC_NAME}.toml"
OUTPUT_DIR="../pbc-chains/${PBC_NAME}-pbc/runtime/src"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "Error: Config file $CONFIG_FILE not found"
    exit 1
fi

# Generate using Rust generator
cargo run --bin pbc-generator -- \
    --config "$CONFIG_FILE" \
    --template templates/lib.rs.hbs \
    --output "$OUTPUT_DIR/lib.rs"

cargo run --bin pbc-generator -- \
    --config "$CONFIG_FILE" \
    --template templates/Cargo.toml.hbs \
    --output "../pbc-chains/${PBC_NAME}-pbc/runtime/Cargo.toml"

echo "Generated PBC: $PBC_NAME"
```

---

## Migration Strategy

### Phase 1: Setup (Week 1)
1. Create `pbc-common` crate
2. Extract shared code from btc-pbc
3. Test pbc-common builds independently
4. Create comprehensive tests

### Phase 2: Template System (Week 1)
1. Create `pbc-template` structure
2. Implement Rust generator
3. Create Handlebars templates
4. Write all 13 PBC configs
5. Test generator with btc.toml

### Phase 3: Refactor BTC PBC (Week 1)
1. Backup btc-pbc to btc-pbc.old
2. Generate new btc-pbc from template
3. Verify build succeeds
4. Run tests, compare functionality
5. Verify WASM output matches

### Phase 4: Refactor Remaining PBCs (Week 2)
1. For each PBC:
   - Backup original
   - Generate from template
   - Build and test
   - Compare with original
2. Run integration tests
3. Benchmark performance

### Phase 5: Cleanup (Week 2)
1. Remove .old backups
2. Update documentation
3. Update build scripts
4. Create TEMPLATE_GUIDE.md
5. Celebrate! 🎉

---

## Implementation Plan

### Week 1: Foundation

**Day 1-2:** pbc-common crate
- [ ] Create crate structure
- [ ] Extract common code
- [ ] Write tests
- [ ] Documentation

**Day 3-4:** Template system
- [ ] Implement generator
- [ ] Create templates
- [ ] Write configs
- [ ] Test generator

**Day 5:** BTC PBC refactor
- [ ] Generate BTC PBC
- [ ] Verify functionality
- [ ] Update tests

### Week 2: Rollout

**Day 6-8:** Refactor 12 PBCs
- [ ] Generate all PBCs
- [ ] Build verification
- [ ] Test all PBCs
- [ ] Integration tests

**Day 9-10:** Cleanup & Documentation
- [ ] Remove old code
- [ ] Update docs
- [ ] Final testing
- [ ] Metrics report

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Code Reduction | ≥80% | Line count before/after |
| Build Time | ≥50% faster | Compilation time |
| Functionality | 100% preserved | Test pass rate |
| Consistency | 0 drift | diff between PBCs |
| New PBC Time | <5 minutes | Config → working PBC |

---

## Risks & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Functionality regression | High | Extensive testing, comparison |
| Build failures | Medium | Incremental approach, rollback |
| Learning curve | Low | Good documentation, examples |
| Config errors | Medium | Validation, clear error messages |

---

## Conclusion

This template system will:
- **Eliminate 92.6% duplication**
- **Simplify PBC creation** to 5 minutes
- **Ensure consistency** across all PBCs
- **Improve maintainability** by 13x
- **Speed up builds** significantly

**Next Step:** Begin implementation Phase 1

---

**Document Status:** Design Complete
**Last Updated:** October 20, 2025
**Ready for Implementation:** Yes

