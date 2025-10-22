#!/bin/bash
# EDSC-PBC Setup Script
# Creates complete directory structure and scaffolding for EDSC stablecoin PBC

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EDSC_DIR="05-multichain/partition-burst-chains/pbc-chains/edsc-pbc"

echo "==========================================="
echo "EDSC-PBC Setup (Phase 0)"
echo "==========================================="
echo ""

# Step 1: Create directory structure
echo "Step 1: Creating directory structure..."
mkdir -p "$EDSC_DIR"/{runtime/{src,presets},node/src}

# Step 2: Create runtime README
echo "Step 2: Creating documentation..."
cat > "$EDSC_DIR/README.md" << 'README_EOF'
# EDSC-PBC Runtime

**Purpose:** Dedicated Partition Burst Chain for Ëtrid Dollar Stablecoin (EDSC)

## Overview

EDSC-PBC is the 13th PBC in the Ëtrid multichain, responsible for:
- EDSC token management (mint/burn/transfer)
- Receipt-based redemption system (SBT tokens)
- Multi-path redemption engine
- TWAP oracle aggregation
- Circuit breakers and safety controls
- Checkpoint synchronization with FlareChain

## Architecture

```
PBC-EDSC (This Chain)
├─ pallet-edsc-token          # ERC20-like EDSC token
├─ pallet-edsc-receipts        # SBT receipt registry
├─ pallet-edsc-redemption      # 3-path redemption engine
├─ pallet-edsc-oracle          # TWAP price aggregation
├─ pallet-edsc-checkpoint      # State commits to main chain
└─ pallet-circuit-breaker      # Safety controls

FlareChain (Main Chain)
├─ pallet-reserve-vault        # On-chain collateral storage
├─ pallet-custodian-registry   # Off-chain reserve agents
├─ pallet-reserve-oracle       # Aggregate reserve reporting
└─ pallet-pbc-bridge           # Checkpoint verification
```

## Key Features

### 1. Three Redemption Paths

**Path 1: SBT Receipt (No Fee)**
- User provides on-chain receipt from verified purchase
- Redeems at recorded purchase price
- Instant, no dynamic fees

**Path 2: Signed Attestation (Dynamic Fee)**
- Exchange/merchant provides signed proof
- TWAP calculated at purchase time
- Dynamic fee prevents arbitrage

**Path 3: Fallback TWAP (Highest Fee)**
- No proof required
- Uses current 24h TWAP
- Strictest per-wallet caps

### 2. Peg Defense Mechanisms

- **Dynamic Fees:** Remove arbitrage profit during depegs
- **Circuit Breakers:** Pause redemptions if volume exceeds caps
- **Reserve Ratio Enforcement:** Maintain 110-130% collateralization
- **Automated Buybacks:** Protocol buys EDSC when price < $1

### 3. Oracle System

- Multi-source TWAP (Binance, Coinbase, Uniswap, PancakeSwap, Curve)
- Outlier removal (> 2% from median)
- 24h primary window, 7-day fallback
- Off-chain worker for price fetching

## Building

```bash
# Build runtime WASM
cargo build --release -p edsc-pbc-runtime

# Build collator
cargo build --release -p edsc-pbc-collator

# Generate chain spec
./target/release/edsc-pbc-collator build-spec --disable-default-bootnode --chain dev > edsc-pbc-spec.json
```

## Running

```bash
# Development mode
./target/release/edsc-pbc-collator \
    --dev \
    --rpc-port 9955 \
    --port 30343 \
    --relay-chain-rpc-url ws://127.0.0.1:9944

# With FlareChain relay
./target/release/edsc-pbc-collator \
    --collator \
    --base-path /tmp/edsc-pbc \
    --rpc-port 9955 \
    --relay-chain-rpc-url ws://127.0.0.1:9944
```

## Testing

```bash
# Unit tests
cargo test -p pallet-edsc-token
cargo test -p pallet-edsc-redemption
cargo test -p pallet-edsc-oracle

# Integration tests
./test_edsc_integration.sh
```

## Parameters (Production)

| Parameter | Value | Purpose |
|---|---|---|
| `MIN_FEE` | 0.25% | Minimum redemption fee |
| `SAFETY_MULTIPLIER` | 1.2 | Fee calculation multiplier |
| `TWAP_WINDOW` | 24 hours | Primary TWAP window |
| `RESERVE_RATIO_TARGET` | 120% | Optimal collateralization |
| `PER_TX_CAP` | 50,000 EDSC | Max per transaction |
| `DAILY_CAP` | 0.5% of supply | Max daily redemptions |

## Documentation

- [EDSC Implementation Plan](../../EDSC_IMPLEMENTATION_PLAN.md)
- [EDSC-PBT Design](../../edsc-pbt.md)
- [Development Roadmap](../../DEVELOPMENT_ROADMAP.md)

## Status

**Phase 0:** Directory structure created ✅
**Phase 1:** Core pallets (token, receipts, redemption) - Pending
**Phase 2:** Oracle integration - Pending
**Phase 3:** Reserve system - Pending
**Phase 4:** Custodian integration - Pending
**Phase 5:** Testing & audit - Pending
**Phase 6:** Production deployment - Pending

---

**Created:** October 19, 2025
**Estimated Completion:** 14 weeks
README_EOF

# Step 3: Create runtime build.rs (same as other PBCs)
echo "Step 3: Creating build configuration..."
cat > "$EDSC_DIR/runtime/build.rs" << 'BUILD_EOF'
fn main() {
    #[cfg(feature = "std")]
    {
        substrate_wasm_builder::WasmBuilder::new()
            .with_current_project()
            .export_heap_base()
            .import_memory()
            .build()
    }
}
BUILD_EOF

# Step 4: Create runtime Cargo.toml
echo "Step 4: Creating runtime Cargo.toml..."
cat > "$EDSC_DIR/runtime/Cargo.toml" << 'CARGO_EOF'
[package]
name = "edsc-pbc-runtime"
version = "0.1.0"
authors = ["Ëtrid Team"]
edition = "2021"
license = "MIT"
publish = false

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
codec = { package = "parity-scale-codec", workspace = true }
scale-info = { workspace = true }

# Substrate
frame-support = { workspace = true }
frame-system = { workspace = true }
frame-executive = { workspace = true }
sp-api = { workspace = true }
sp-block-builder = { workspace = true }
sp-consensus-aura = { workspace = true }
sp-core = { workspace = true }
sp-inherents = { workspace = true }
sp-offchain = { workspace = true }
sp-runtime = { workspace = true }
sp-session = { workspace = true }
sp-std = { workspace = true }
sp-transaction-pool = { workspace = true }
sp-version = { workspace = true }
sp-genesis-builder = { workspace = true }

# FRAME pallets
pallet-aura = { workspace = true }
pallet-balances = { workspace = true }
pallet-session = { workspace = true }
pallet-sudo = { workspace = true }
pallet-timestamp = { workspace = true }
pallet-transaction-payment = { workspace = true }

# Ëtrid multichain
etrid-multichain-primitives = { path = "../../../primitives", default-features = false }
pallet-bridge = { path = "../../../pallets/pallet-bridge", default-features = false }

# EDSC pallets (to be created in Phase 1)
# pallet-edsc-token = { path = "../../../pallets/pallet-edsc-token", default-features = false }
# pallet-edsc-receipts = { path = "../../../pallets/pallet-edsc-receipts", default-features = false }
# pallet-edsc-redemption = { path = "../../../pallets/pallet-edsc-redemption", default-features = false }
# pallet-edsc-oracle = { path = "../../../pallets/pallet-edsc-oracle", default-features = false }
# pallet-edsc-checkpoint = { path = "../../../pallets/pallet-edsc-checkpoint", default-features = false }
# pallet-circuit-breaker = { path = "../../../pallets/pallet-circuit-breaker", default-features = false }

[build-dependencies]
substrate-wasm-builder = { workspace = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "frame-executive/std",
    "sp-api/std",
    "sp-block-builder/std",
    "sp-consensus-aura/std",
    "sp-core/std",
    "sp-inherents/std",
    "sp-offchain/std",
    "sp-runtime/std",
    "sp-session/std",
    "sp-std/std",
    "sp-transaction-pool/std",
    "sp-version/std",
    "sp-genesis-builder/std",
    "pallet-aura/std",
    "pallet-balances/std",
    "pallet-session/std",
    "pallet-sudo/std",
    "pallet-timestamp/std",
    "pallet-transaction-payment/std",
    "etrid-multichain-primitives/std",
    "pallet-bridge/std",
]
runtime-benchmarks = [
    "frame-support/runtime-benchmarks",
    "frame-system/runtime-benchmarks",
]
CARGO_EOF

# Step 5: Create genesis presets
echo "Step 5: Creating genesis presets..."
mkdir -p "$EDSC_DIR/runtime/presets"

cat > "$EDSC_DIR/runtime/presets/development.json" << 'DEV_PRESET_EOF'
{
  "name": "EDSC-PBC Development",
  "id": "edsc_pbc_dev",
  "chainType": "Development",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "edsc-pbc",
  "properties": {
    "tokenSymbol": "EDSC",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "consensusEngine": null,
  "codeSubstitutes": {},
  "genesis": {
    "runtime": {
      "system": {
        "code": ""
      },
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000]
        ]
      },
      "sudo": {
        "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      }
    }
  }
}
DEV_PRESET_EOF

cat > "$EDSC_DIR/runtime/presets/local_testnet.json" << 'LOCAL_PRESET_EOF'
{
  "name": "EDSC-PBC Local Testnet",
  "id": "edsc_pbc_local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "edsc-pbc-local",
  "properties": {
    "tokenSymbol": "EDSC",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "consensusEngine": null,
  "codeSubstitutes": {},
  "genesis": {
    "runtime": {
      "system": {
        "code": ""
      },
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000]
        ]
      },
      "sudo": {
        "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      }
    }
  }
}
LOCAL_PRESET_EOF

# Step 6: Create placeholder for runtime lib.rs
echo "Step 6: Creating runtime lib.rs placeholder..."
cat > "$EDSC_DIR/runtime/src/lib.rs" << 'RUNTIME_EOF'
//! EDSC-PBC Runtime
//!
//! Dedicated Partition Burst Chain for Ëtrid Dollar Stablecoin (EDSC)

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
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

use frame_support::{
    construct_runtime,
    dispatch::DispatchClass,
    parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU8, Everything},
    weights::{
        constants::WEIGHT_REF_TIME_PER_SECOND, IdentityFee, Weight,
    },
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot,
};

pub use frame_support::weights::constants::{
    BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_MILLIS,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
        }
    }
}

// Constant values used within the runtime.
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type DbWeight = RocksDbWeight;
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
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

// Placeholder for EDSC pallets (Phase 1)
// impl pallet_edsc_token::Config for Runtime { ... }
// impl pallet_edsc_receipts::Config for Runtime { ... }
// impl pallet_edsc_redemption::Config for Runtime { ... }

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        // Bridge: pallet_bridge,  // Enable when connected to FlareChain

        // EDSC pallets (Phase 1 - to be added)
        // EdscToken: pallet_edsc_token,
        // EdscReceipts: pallet_edsc_receipts,
        // EdscRedemption: pallet_edsc_redemption,
        // EdscOracle: pallet_edsc_oracle,
        // EdscCheckpoint: pallet_edsc_checkpoint,
        // CircuitBreaker: pallet_circuit_breaker,
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
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

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_benchmarking, BaselineBench::<Runtime>]
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_timestamp, Timestamp]
    );
}

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
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

    impl sp_api::Metadata<Block> for Runtime {
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

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
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

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn create_default_config() -> Vec<u8> {
            create_default_config::<RuntimeGenesisConfig>()
        }

        fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_config::<RuntimeGenesisConfig>(config)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use sp_storage::TrackedStorageKey;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            impl frame_system_benchmarking::Config for Runtime {}
            impl baseline::Config for Runtime {}

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![
                sp_genesis_builder::PresetId::from("development"),
                sp_genesis_builder::PresetId::from("local_testnet"),
            ]
        }
    }
}

// Runtime constants and types
pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Nonce = u32;
pub type Hash = sp_core::H256;

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("edsc-pbc"),
    impl_name: create_runtime_str!("edsc-pbc"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLICENTS;

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
const MAXIMUM_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);

use sp_runtime::Perbill;
use frame_support::traits::ConstBool;

#[cfg(feature = "std")]
use sp_genesis_builder::{build_state, create_default_config, get_preset};
use sp_genesis_builder::PresetId;

pub type RuntimeGenesisConfig = <Runtime as frame_system::Config>::GenesisConfig;

// Missing runtime API trait implementations
use frame_system_rpc_runtime_api;
use pallet_transaction_payment_rpc_runtime_api;
RUNTIME_EOF

echo "✅ EDSC-PBC directory structure created successfully!"
echo ""
echo "Directory structure:"
tree -L 3 "$EDSC_DIR" 2>/dev/null || find "$EDSC_DIR" -type d | sed 's|[^/]*/| |g'
echo ""
echo "Next steps:"
echo "  1. Review EDSC_IMPLEMENTATION_PLAN.md for full implementation details"
echo "  2. Phase 1: Create EDSC pallets (pallet-edsc-token, etc.)"
echo "  3. Phase 2: Implement oracle integration"
echo "  4. See: 05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/README.md"
echo ""
echo "To test chain spec generation (after Phase 1 pallets are created):"
echo "  cargo build --release -p edsc-pbc-runtime"
echo "  ./target/release/edsc-pbc-collator build-spec --dev > edsc-dev-spec.json"
echo ""
RUNTIME_EOF

chmod +x setup_edsc_pbc.sh

echo "✅ Script created successfully!"
echo ""
echo "Run the setup script:"
echo "  ./setup_edsc_pbc.sh"
echo ""
