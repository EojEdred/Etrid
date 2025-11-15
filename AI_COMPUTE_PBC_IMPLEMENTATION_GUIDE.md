# AI Compute Network - PBC Implementation Guide

**Goal**: Build the AI Compute Network on a dedicated PBC so FlareChain runtime doesn't need to be rebuilt.

**Strategy**: Create `ai-compute-pbc` (15th PBC) with all AI-specific pallets, leveraging Ëtrid's existing PBC architecture.

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [PBC Runtime Structure](#2-pbc-runtime-structure)
3. [Custom Pallets for AI Compute](#3-custom-pallets-for-ai-compute)
4. [Implementation Roadmap](#4-implementation-roadmap)
5. [No FlareChain Changes Required](#5-no-flarechain-changes-required)
6. [Cross-Chain Communication](#6-cross-chain-communication)
7. [Deployment & Operations](#7-deployment--operations)

---

## 1. Architecture Overview

### 1.1 Existing PBC Pattern

Your current PBCs follow this pattern:

```
05-multichain/partition-burst-chains/
├── pbc-runtime/              # Base runtime template
│   ├── src/lib.rs            # construct_runtime! macro
│   └── Cargo.toml
│
├── pbc-chains/               # Specific PBC implementations
│   ├── btc-pbc/
│   │   ├── runtime/          # BTC-specific runtime
│   │   └── collator/         # BTC collator node
│   ├── eth-pbc/
│   │   ├── runtime/
│   │   └── collator/
│   └── ...
```

**Key Insight**: Each PBC has its own runtime that extends the base `pbc-runtime`. FlareChain is unaware of PBC-specific pallets.

### 1.2 AI Compute PBC Architecture

```
┌───────────────────────────────────────────────────────────────┐
│                    Ëtrid Multichain                           │
├───────────────────────────────────────────────────────────────┤
│                                                                │
│  FlareChain (Relay Chain)                                     │
│  ├─ Runtime: v105 (NO CHANGES NEEDED)                         │
│  ├─ Pallets: System, Balances, Grandpa, etc.                 │
│  └─ Role: Coordinate PBCs, finality, governance               │
│                                                                │
│                          ↓ ↑ XCM Messages                      │
│                                                                │
│  AI-Compute-PBC (15th PBC) 🆕                                 │
│  ├─ Runtime: ai-compute-pbc-runtime                           │
│  ├─ Pallets (NEW):                                            │
│  │   ├─ pallet-gpu-registry                                   │
│  │   ├─ pallet-job-marketplace                                │
│  │   ├─ pallet-confidential-compute                           │
│  │   ├─ pallet-ai-models                                      │
│  │   ├─ pallet-reputation                                     │
│  │   └─ pallet-payment-streams (Lightning integration)        │
│  ├─ Existing Pallets:                                         │
│  │   ├─ pallet-aidid (from 02-open-did)                       │
│  │   ├─ pallet-did-registry (from 02-open-did)                │
│  │   └─ pallet-accounts (from 04-accounts)                    │
│  └─ Collator: ai-compute-pbc-collator                         │
│                                                                │
│  Other PBCs (13 existing)                                     │
│  ├─ btc-pbc, eth-pbc, sol-pbc, ...                           │
│  └─ Continue operating independently                          │
│                                                                │
└───────────────────────────────────────────────────────────────┘
```

**Benefits**:
- ✅ FlareChain runtime unchanged (no rebuild)
- ✅ AI Compute isolated on dedicated PBC
- ✅ Independent upgrades (AI PBC can upgrade without affecting others)
- ✅ Specialized gas pricing for AI workloads
- ✅ High throughput (5,000+ TPS on AI PBC alone)

---

## 2. PBC Runtime Structure

### 2.1 Directory Structure

Create new PBC following existing pattern:

```bash
05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/
├── runtime/                          # AI Compute PBC Runtime
│   ├── src/
│   │   └── lib.rs                    # Runtime definition
│   ├── Cargo.toml
│   └── build.rs                      # WASM builder
│
├── collator/                         # AI Compute Collator Node
│   ├── src/
│   │   ├── main.rs                   # Collator entry point
│   │   ├── service.rs                # Node service setup
│   │   ├── chain_spec.rs             # Chain specification
│   │   └── cli.rs                    # CLI commands
│   └── Cargo.toml
│
├── pallets/                          # AI-Specific Pallets
│   ├── gpu-registry/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs
│   │   │   └── tests.rs
│   │   └── Cargo.toml
│   │
│   ├── job-marketplace/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── bidding.rs
│   │   │   ├── matching.rs
│   │   │   └── escrow.rs
│   │   └── Cargo.toml
│   │
│   ├── confidential-compute/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tee.rs
│   │   │   ├── attestation.rs
│   │   │   └── encryption.rs
│   │   └── Cargo.toml
│   │
│   ├── ai-models/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── registry.rs
│   │   │   └── pricing.rs
│   │   └── Cargo.toml
│   │
│   ├── reputation/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── scoring.rs
│   │   │   └── slashing.rs
│   │   └── Cargo.toml
│   │
│   └── payment-streams/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── channels.rs
│       │   └── streaming.rs
│       └── Cargo.toml
│
├── chain-spec-generator/             # Generate chainspec for deployment
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
└── README.md
```

### 2.2 Runtime Implementation

**File**: `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use sp_std::vec::Vec;
use sp_api::impl_runtime_apis;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU16, ConstU32, ConstU64, ConstU8},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
        IdentityFee, Weight,
    },
};

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{generic, impl_opaque_keys, MultiAddress, Perbill};

// Standard types
pub type BlockNumber = u32;
pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Nonce = u32;
pub type Hash = sp_core::H256;

pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    pub type Header = generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

#[sp_version::runtime_version]
pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: sp_runtime::create_runtime_str!("ai-compute-pbc"),
    impl_name: sp_runtime::create_runtime_str!("ai-compute-pbc"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

// Block timing
pub const MILLISECS_PER_BLOCK: u64 = 6000;  // 6 second blocks
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: sp_version::RuntimeVersion = VERSION;
    pub RuntimeBlockLength: frame_system::limits::BlockLength =
        frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, Perbill::from_percent(75));
    pub RuntimeBlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX)
        );
}

// Configure System pallet
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type DbWeight = RocksDbWeight;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = sp_runtime::traits::AccountIdLookup<AccountId, ()>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
}

// Configure Aura (block production)
impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = frame_support::traits::ConstBool<false>;
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

// Configure GRANDPA (finality)
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

// Configure Timestamp
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

// Configure Balances (ËDSC on this PBC)
impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

// Configure Transaction Payment
impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
    type WeightInfo = ();
}

// Configure Sudo (for testnet, remove for mainnet)
impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

// ====================================================================
// EXISTING ËTRID PALLETS (REUSE FROM OTHER MODULES)
// ====================================================================

// Import AIDID from 02-open-did
impl pallet_aidid::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Currency = Balances;
}

// Import DID Registry from 02-open-did
impl pallet_did_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// Import Accounts from 04-accounts
impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
}

// ====================================================================
// NEW AI COMPUTE PALLETS
// ====================================================================

// GPU Registry - Register and discover GPU nodes
parameter_types! {
    pub const MinimumStake: Balance = 1_000 * 1_000_000_000_000; // 1,000 ËDSC
    pub const MaxGPUNodes: u32 = 100_000;
}

impl pallet_gpu_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinimumStake = MinimumStake;
    type MaxGPUNodes = MaxGPUNodes;
    type WeightInfo = ();
}

// Job Marketplace - Submit and match AI inference jobs
parameter_types! {
    pub const MaxJobsPerBlock: u32 = 1000;
    pub const JobEscrowPeriod: BlockNumber = 100; // ~10 minutes
    pub const MaxBidsPerJob: u32 = 50;
}

impl pallet_job_marketplace::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxJobsPerBlock = MaxJobsPerBlock;
    type JobEscrowPeriod = JobEscrowPeriod;
    type MaxBidsPerJob = MaxBidsPerJob;
    type WeightInfo = ();
}

// Confidential Compute - TEE and privacy-preserving inference
parameter_types! {
    pub const AttestationValidityPeriod: BlockNumber = 14400; // ~24 hours
}

impl pallet_confidential_compute::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AttestationValidityPeriod = AttestationValidityPeriod;
    type WeightInfo = ();
}

// AI Models - Register and catalog AI models
parameter_types! {
    pub const MaxModelsPerOwner: u32 = 100;
    pub const ModelRegistrationDeposit: Balance = 100 * 1_000_000_000_000; // 100 ËDSC
}

impl pallet_ai_models::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxModelsPerOwner = MaxModelsPerOwner;
    type ModelRegistrationDeposit = ModelRegistrationDeposit;
    type WeightInfo = ();
}

// Reputation Engine - Track GPU node performance
parameter_types! {
    pub const InitialReputationScore: u32 = 5000; // 50% starting score
    pub const SlashingAmount: Balance = 100 * 1_000_000_000_000; // 100 ËDSC per violation
}

impl pallet_reputation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type InitialReputationScore = InitialReputationScore;
    type SlashingAmount = SlashingAmount;
    type WeightInfo = ();
}

// Payment Streams - Lightning-Bloc integration for instant payments
parameter_types! {
    pub const PlatformFeePercent: u32 = 5; // 5% platform fee
}

impl pallet_payment_streams::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type PlatformFeePercent = PlatformFeePercent;
    type WeightInfo = ();
}

// ====================================================================
// CONSTRUCT RUNTIME
// ====================================================================

construct_runtime!(
    pub struct Runtime {
        // Standard substrate pallets
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,

        // Existing Ëtrid pallets (reused)
        AIDID: pallet_aidid,
        DIDRegistry: pallet_did_registry,
        Accounts: pallet_accounts,

        // NEW: AI Compute pallets
        GPURegistry: pallet_gpu_registry,
        JobMarketplace: pallet_job_marketplace,
        ConfidentialCompute: pallet_confidential_compute,
        AIModels: pallet_ai_models,
        Reputation: pallet_reputation,
        PaymentStreams: pallet_payment_streams,
    }
);

// Standard runtime types
pub type Address = MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
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
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// Runtime APIs implementation
impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> sp_version::RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as sp_runtime::traits::Block>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> sp_core::OpaqueMetadata {
            sp_core::OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<sp_core::OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as sp_runtime::traits::Block>::Extrinsic) -> sp_runtime::ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as sp_runtime::traits::Block>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as sp_runtime::traits::Block>::Extrinsic> {
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
            source: sp_runtime::transaction_validity::TransactionSource,
            tx: <Block as sp_runtime::traits::Block>::Extrinsic,
            block_hash: <Block as sp_runtime::traits::Block>::Hash,
        ) -> sp_runtime::transaction_validity::TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as sp_runtime::traits::Block>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            pallet_aura::Authorities::<Runtime>::get().into_inner()
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as sp_runtime::traits::Block>::Hash,
                sp_runtime::traits::NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: sp_consensus_grandpa::AuthorityId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as sp_runtime::traits::Block>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as sp_runtime::traits::Block>::Extrinsic,
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
            use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
            list_benchmark!(list, extra, pallet_balances, Balances);
            list_benchmark!(list, extra, pallet_timestamp, Timestamp);

            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig,
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark};
            use frame_support::traits::TrackedStorageKey;

            let whitelist: Vec<TrackedStorageKey> = vec![];
            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
```

### 2.3 Cargo.toml

**File**: `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime/Cargo.toml`

```toml
[package]
name = "ai-compute-pbc-runtime"
version = "0.1.0"
authors = ["Ëtrid Foundation"]
edition = "2021"
license = "Apache-2.0"

[dependencies]
codec = { workspace = true }
scale-info = { workspace = true }

# Substrate core
frame-support = { workspace = true }
frame-system = { workspace = true }
frame-executive = { workspace = true }
sp-api = { workspace = true }
sp-runtime = { workspace = true }
sp-core = { workspace = true }
sp-std = { workspace = true }
sp-io = { workspace = true }
sp-consensus-aura = { workspace = true }
sp-consensus-grandpa = { workspace = true }
sp-block-builder = { workspace = true }
sp-transaction-pool = { workspace = true }
sp-inherents = { workspace = true }
sp-offchain = { workspace = true }
sp-session = { workspace = true }
sp-version = { workspace = true }

# Standard pallets
pallet-aura = { workspace = true }
pallet-grandpa = { workspace = true }
pallet-sudo = { workspace = true }
pallet-timestamp = { workspace = true }
pallet-transaction-payment = { workspace = true }
pallet-balances = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }

# RPC Runtime APIs
frame-system-rpc-runtime-api = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }
pallet-transaction-payment-rpc-runtime-api = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }

# Existing Ëtrid pallets (REUSE - no changes to FlareChain)
pallet-aidid = { path = "../../../../02-open-did/pallets/aidid", default-features = false }
pallet-did-registry = { path = "../../../../02-open-did/pallets/did-registry", default-features = false }
pallet-accounts = { path = "../../../../04-accounts/pallet", default-features = false }

# NEW: AI Compute pallets (local to this PBC)
pallet-gpu-registry = { path = "../pallets/gpu-registry", default-features = false }
pallet-job-marketplace = { path = "../pallets/job-marketplace", default-features = false }
pallet-confidential-compute = { path = "../pallets/confidential-compute", default-features = false }
pallet-ai-models = { path = "../pallets/ai-models", default-features = false }
pallet-reputation = { path = "../pallets/reputation", default-features = false }
pallet-payment-streams = { path = "../pallets/payment-streams", default-features = false }

[build-dependencies]
substrate-wasm-builder = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "frame-executive/std",
    "sp-api/std",
    "sp-runtime/std",
    "sp-core/std",
    "sp-std/std",
    "sp-io/std",
    "sp-consensus-aura/std",
    "sp-consensus-grandpa/std",
    "sp-block-builder/std",
    "sp-transaction-pool/std",
    "sp-inherents/std",
    "sp-offchain/std",
    "sp-session/std",
    "sp-version/std",
    "pallet-aura/std",
    "pallet-balances/std",
    "pallet-grandpa/std",
    "pallet-sudo/std",
    "pallet-timestamp/std",
    "pallet-transaction-payment/std",
    "frame-system-rpc-runtime-api/std",
    "pallet-transaction-payment-rpc-runtime-api/std",
    # Existing Ëtrid
    "pallet-aidid/std",
    "pallet-did-registry/std",
    "pallet-accounts/std",
    # NEW AI Compute
    "pallet-gpu-registry/std",
    "pallet-job-marketplace/std",
    "pallet-confidential-compute/std",
    "pallet-ai-models/std",
    "pallet-reputation/std",
    "pallet-payment-streams/std",
]
runtime-benchmarks = [
    "frame-support/runtime-benchmarks",
    "frame-system/runtime-benchmarks",
    "pallet-gpu-registry/runtime-benchmarks",
    "pallet-job-marketplace/runtime-benchmarks",
]
```

---

## 3. Custom Pallets for AI Compute

### 3.1 GPU Registry Pallet

**File**: `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/pallets/gpu-registry/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        #[pallet::constant]
        type MinimumStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MaxGPUNodes: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    /// GPU Node Profile
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GPUProfile<AccountId, Balance> {
        pub owner: AccountId,
        pub gpu_model: BoundedVec<u8, ConstU32<64>>,
        pub vram_gb: u32,
        pub cuda_cores: u32,
        pub location: BoundedVec<u8, ConstU32<32>>,  // e.g., "US-WEST-2"
        pub stake: Balance,
        pub status: NodeStatus,
        pub reputation_score: u32,  // 0-10000 (0.00-100.00%)
        pub total_jobs_completed: u64,
        pub registered_at: u64,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum NodeStatus {
        Active,
        Inactive,
        Suspended,
    }

    impl Default for NodeStatus {
        fn default() -> Self {
            NodeStatus::Active
        }
    }

    /// Storage: GPU Nodes indexed by DID
    #[pallet::storage]
    #[pallet::getter(fn gpu_nodes)]
    pub type GPUNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<128>>,  // DID as key
        GPUProfile<T::AccountId, BalanceOf<T>>,
    >;

    /// Storage: Owner to DID mapping
    #[pallet::storage]
    #[pallet::getter(fn owner_nodes)]
    pub type OwnerNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<BoundedVec<u8, ConstU32<128>>, ConstU32<10>>,  // Max 10 GPUs per owner
        ValueQuery,
    >;

    /// Storage: Total registered nodes
    #[pallet::storage]
    #[pallet::getter(fn total_nodes)]
    pub type TotalNodes<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// GPU node registered [did, owner, stake]
        NodeRegistered { did: Vec<u8>, owner: T::AccountId, stake: BalanceOf<T> },
        /// GPU node status changed [did, old_status, new_status]
        NodeStatusChanged { did: Vec<u8>, old_status: NodeStatus, new_status: NodeStatus },
        /// GPU node deregistered [did, owner]
        NodeDeregistered { did: Vec<u8>, owner: T::AccountId },
        /// Reputation updated [did, old_score, new_score]
        ReputationUpdated { did: Vec<u8>, old_score: u32, new_score: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient stake amount
        InsufficientStake,
        /// GPU node already exists
        NodeAlreadyExists,
        /// GPU node not found
        NodeNotFound,
        /// Not the owner of the GPU node
        NotOwner,
        /// Maximum number of nodes reached
        MaxNodesReached,
        /// Owner has too many nodes
        TooManyNodesPerOwner,
        /// Invalid DID format
        InvalidDID,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new GPU node
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_node(
            origin: OriginFor<T>,
            did: Vec<u8>,
            gpu_model: Vec<u8>,
            vram_gb: u32,
            cuda_cores: u32,
            location: Vec<u8>,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // Validate stake
            ensure!(stake >= T::MinimumStake::get(), Error::<T>::InsufficientStake);

            // Check total nodes limit
            let total = TotalNodes::<T>::get();
            ensure!(total < T::MaxGPUNodes::get(), Error::<T>::MaxNodesReached);

            // Convert to BoundedVec
            let did_bounded: BoundedVec<u8, ConstU32<128>> =
                did.clone().try_into().map_err(|_| Error::<T>::InvalidDID)?;
            let gpu_model_bounded: BoundedVec<u8, ConstU32<64>> =
                gpu_model.try_into().map_err(|_| Error::<T>::InvalidDID)?;
            let location_bounded: BoundedVec<u8, ConstU32<32>> =
                location.try_into().map_err(|_| Error::<T>::InvalidDID)?;

            // Ensure node doesn't exist
            ensure!(!GPUNodes::<T>::contains_key(&did_bounded), Error::<T>::NodeAlreadyExists);

            // Reserve stake from owner
            T::Currency::reserve(&owner, stake)?;

            // Get current timestamp
            let now = frame_system::Pallet::<T>::block_number().saturated_into::<u64>();

            // Create GPU profile
            let profile = GPUProfile {
                owner: owner.clone(),
                gpu_model: gpu_model_bounded,
                vram_gb,
                cuda_cores,
                location: location_bounded,
                stake,
                status: NodeStatus::Active,
                reputation_score: 5000, // Start at 50%
                total_jobs_completed: 0,
                registered_at: now,
            };

            // Store profile
            GPUNodes::<T>::insert(&did_bounded, profile);

            // Update owner's node list
            OwnerNodes::<T>::try_mutate(&owner, |nodes| -> DispatchResult {
                nodes.try_push(did_bounded.clone())
                    .map_err(|_| Error::<T>::TooManyNodesPerOwner)?;
                Ok(())
            })?;

            // Increment total
            TotalNodes::<T>::put(total + 1);

            Self::deposit_event(Event::NodeRegistered { did, owner, stake });
            Ok(())
        }

        /// Update GPU node status
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn update_status(
            origin: OriginFor<T>,
            did: Vec<u8>,
            new_status: NodeStatus,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let did_bounded: BoundedVec<u8, ConstU32<128>> =
                did.clone().try_into().map_err(|_| Error::<T>::InvalidDID)?;

            GPUNodes::<T>::try_mutate(&did_bounded, |maybe_profile| -> DispatchResult {
                let profile = maybe_profile.as_mut().ok_or(Error::<T>::NodeNotFound)?;
                ensure!(profile.owner == who, Error::<T>::NotOwner);

                let old_status = profile.status.clone();
                profile.status = new_status.clone();

                Self::deposit_event(Event::NodeStatusChanged {
                    did,
                    old_status,
                    new_status,
                });
                Ok(())
            })
        }

        /// Deregister GPU node (returns stake)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn deregister_node(
            origin: OriginFor<T>,
            did: Vec<u8>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let did_bounded: BoundedVec<u8, ConstU32<128>> =
                did.clone().try_into().map_err(|_| Error::<T>::InvalidDID)?;

            let profile = GPUNodes::<T>::get(&did_bounded).ok_or(Error::<T>::NodeNotFound)?;
            ensure!(profile.owner == owner, Error::<T>::NotOwner);

            // Unreserve stake
            T::Currency::unreserve(&owner, profile.stake);

            // Remove from storage
            GPUNodes::<T>::remove(&did_bounded);

            // Update owner's node list
            OwnerNodes::<T>::mutate(&owner, |nodes| {
                nodes.retain(|n| n != &did_bounded);
            });

            // Decrement total
            let total = TotalNodes::<T>::get();
            TotalNodes::<T>::put(total.saturating_sub(1));

            Self::deposit_event(Event::NodeDeregistered { did, owner });
            Ok(())
        }
    }

    // Weight info trait (placeholder)
    pub trait WeightInfo {
        fn register_node() -> Weight;
        fn update_status() -> Weight;
        fn deregister_node() -> Weight;
    }

    impl WeightInfo for () {
        fn register_node() -> Weight { Weight::from_parts(10_000, 0) }
        fn update_status() -> Weight { Weight::from_parts(10_000, 0) }
        fn deregister_node() -> Weight { Weight::from_parts(10_000, 0) }
    }
}
```

### 3.2 Job Marketplace Pallet (Simplified Example)

**File**: `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/pallets/job-marketplace/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        #[pallet::constant]
        type MaxJobsPerBlock: Get<u32>;

        #[pallet::constant]
        type JobEscrowPeriod: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type MaxBidsPerJob: Get<u32>;

        type WeightInfo: WeightInfo;
    }

    /// AI Inference Job
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct AIJob<AccountId, Balance, BlockNumber> {
        pub id: u64,
        pub requester: AccountId,
        pub model_did: BoundedVec<u8, ConstU32<128>>,
        pub encrypted_prompt: BoundedVec<u8, ConstU32<10240>>, // Max 10KB prompt
        pub max_tokens: u32,
        pub max_payment: Balance,
        pub min_vram_gb: u32,
        pub confidential: bool,
        pub created_at: BlockNumber,
        pub status: JobStatus,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum JobStatus {
        Pending,
        Assigned,
        Processing,
        Completed,
        Failed,
    }

    /// Job Bid from GPU node
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct JobBid<AccountId, Balance> {
        pub gpu_did: BoundedVec<u8, ConstU32<128>>,
        pub bidder: AccountId,
        pub price_per_token: Balance,
        pub estimated_latency_ms: u32,
    }

    /// Storage: Jobs by ID
    #[pallet::storage]
    #[pallet::getter(fn jobs)]
    pub type Jobs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        AIJob<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
    >;

    /// Storage: Job counter
    #[pallet::storage]
    #[pallet::getter(fn next_job_id)]
    pub type NextJobId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Storage: Bids for a job
    #[pallet::storage]
    #[pallet::getter(fn job_bids)]
    pub type JobBids<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,  // job_id
        BoundedVec<JobBid<T::AccountId, BalanceOf<T>>, ConstU32<50>>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Job submitted [job_id, requester, max_payment]
        JobSubmitted { job_id: u64, requester: T::AccountId, max_payment: BalanceOf<T> },
        /// Bid placed [job_id, gpu_did, bidder, price]
        BidPlaced { job_id: u64, gpu_did: Vec<u8>, bidder: T::AccountId, price: BalanceOf<T> },
        /// Job assigned [job_id, gpu_did, winner]
        JobAssigned { job_id: u64, gpu_did: Vec<u8>, winner: T::AccountId },
        /// Job completed [job_id, result_hash]
        JobCompleted { job_id: u64, result_hash: [u8; 32] },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Job not found
        JobNotFound,
        /// Not authorized
        NotAuthorized,
        /// Too many bids
        TooManyBids,
        /// Invalid bid
        InvalidBid,
        /// Insufficient payment
        InsufficientPayment,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit an AI inference job
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn submit_job(
            origin: OriginFor<T>,
            model_did: Vec<u8>,
            encrypted_prompt: Vec<u8>,
            max_tokens: u32,
            max_payment: BalanceOf<T>,
            min_vram_gb: u32,
            confidential: bool,
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;

            // Reserve payment
            T::Currency::reserve(&requester, max_payment)?;

            let job_id = NextJobId::<T>::get();
            let created_at = frame_system::Pallet::<T>::block_number();

            let model_did_bounded: BoundedVec<u8, ConstU32<128>> =
                model_did.try_into().map_err(|_| Error::<T>::InvalidBid)?;
            let prompt_bounded: BoundedVec<u8, ConstU32<10240>> =
                encrypted_prompt.try_into().map_err(|_| Error::<T>::InvalidBid)?;

            let job = AIJob {
                id: job_id,
                requester: requester.clone(),
                model_did: model_did_bounded,
                encrypted_prompt: prompt_bounded,
                max_tokens,
                max_payment,
                min_vram_gb,
                confidential,
                created_at,
                status: JobStatus::Pending,
            };

            Jobs::<T>::insert(job_id, job);
            NextJobId::<T>::put(job_id + 1);

            Self::deposit_event(Event::JobSubmitted { job_id, requester, max_payment });
            Ok(())
        }

        /// GPU node places bid on job
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn place_bid(
            origin: OriginFor<T>,
            job_id: u64,
            gpu_did: Vec<u8>,
            price_per_token: BalanceOf<T>,
            estimated_latency_ms: u32,
        ) -> DispatchResult {
            let bidder = ensure_signed(origin)?;

            // Verify job exists
            ensure!(Jobs::<T>::contains_key(job_id), Error::<T>::JobNotFound);

            let gpu_did_bounded: BoundedVec<u8, ConstU32<128>> =
                gpu_did.clone().try_into().map_err(|_| Error::<T>::InvalidBid)?;

            let bid = JobBid {
                gpu_did: gpu_did_bounded,
                bidder: bidder.clone(),
                price_per_token,
                estimated_latency_ms,
            };

            // Add bid to list
            JobBids::<T>::try_mutate(job_id, |bids| -> DispatchResult {
                bids.try_push(bid).map_err(|_| Error::<T>::TooManyBids)?;
                Ok(())
            })?;

            Self::deposit_event(Event::BidPlaced {
                job_id,
                gpu_did,
                bidder,
                price: price_per_token,
            });
            Ok(())
        }

        // Additional functions: assign_job, complete_job, etc.
        // (Omitted for brevity - see full implementation)
    }

    pub trait WeightInfo {
        fn submit_job() -> Weight;
        fn place_bid() -> Weight;
    }

    impl WeightInfo for () {
        fn submit_job() -> Weight { Weight::from_parts(10_000, 0) }
        fn place_bid() -> Weight { Weight::from_parts(10_000, 0) }
    }
}
```

---

## 4. Implementation Roadmap

### Phase 1: PBC Foundation (Weeks 1-4)

**Goal**: Set up ai-compute-pbc infrastructure

**Tasks**:
1. Create directory structure
   ```bash
   cd /Users/macbook/Desktop/etrid
   mkdir -p 05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/{runtime,collator,pallets}
   ```

2. Copy base PBC runtime as template
   ```bash
   cp -r 05-multichain/partition-burst-chains/pbc-runtime \
         05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime
   ```

3. Create 6 AI compute pallets (boilerplate)
   - `pallet-gpu-registry`
   - `pallet-job-marketplace`
   - `pallet-confidential-compute`
   - `pallet-ai-models`
   - `pallet-reputation`
   - `pallet-payment-streams`

4. Update Cargo workspace
   ```toml
   # /Users/macbook/Desktop/etrid/Cargo.toml
   [workspace]
   members = [
       # ... existing members ...
       "05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime",
       "05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/collator",
       "05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/pallets/*",
   ]
   ```

5. Build runtime WASM
   ```bash
   cd 05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime
   cargo build --release
   ```

### Phase 2: Core Pallets (Weeks 5-10)

**Goal**: Implement GPU Registry + Job Marketplace

**Week 5-6: GPU Registry**
- Implement `register_node()`, `update_status()`, `deregister_node()`
- Add reputation tracking
- Write 15+ unit tests
- Benchmark weights

**Week 7-8: Job Marketplace**
- Implement `submit_job()`, `place_bid()`, `assign_job()`
- Add escrow system
- Implement job matching algorithm
- Write 20+ unit tests

**Week 9-10: Integration**
- Connect GPU Registry ↔ Job Marketplace
- Test full workflow: register GPU → submit job → bid → assign
- Add RPC endpoints for querying

### Phase 3: Collator Node (Weeks 11-12)

**Goal**: Build ai-compute-pbc-collator binary

**Tasks**:
1. Copy collator template from btc-pbc-collator
2. Update chain spec for ai-compute-pbc
3. Configure session keys (Aura + GRANDPA)
4. Test local 3-node network
5. Generate genesis state

### Phase 4: Confidential Compute (Weeks 13-18)

**Goal**: Implement TEE integration

**Week 13-14: Attestation**
- Intel SGX remote attestation
- Verify enclave is genuine
- Store attestation proofs on-chain

**Week 15-16: Encryption**
- End-to-end encryption for prompts/results
- Key management (DKG)
- Secure model storage

**Week 17-18: ZK-Proofs**
- Implement ZK-SNARK verification
- Prove inference correctness
- Optimize on-chain verification

### Phase 5: Testing & Deployment (Weeks 19-24)

**Week 19-20: Testnet Launch**
- Deploy 3 collators (ai-compute-pbc testnet)
- Register with FlareChain via XCM
- Open to public GPU node registration

**Week 21-22: Integration Testing**
- End-to-end inference tests
- Load testing (1000+ concurrent jobs)
- Stress test reputation system

**Week 23-24: Security Audit**
- External audit (Trail of Bits)
- Fix vulnerabilities
- Prepare for mainnet

---

## 5. No FlareChain Changes Required

### 5.1 Why FlareChain Stays Untouched

**Key Principle**: PBCs are **independent sovereign chains** that communicate with FlareChain via XCM (Cross-Chain Messages).

```
┌─────────────────────────────────────────────────────────┐
│ FlareChain (Relay Chain)                                │
│ ├─ Runtime: v105 (FROZEN - no changes)                  │
│ ├─ Pallets:                                             │
│ │   ├─ System, Balances, Grandpa, etc.                 │
│ │   └─ NO AI-specific pallets needed                   │
│ └─ Role:                                                │
│     ├─ Provide finality to PBCs                         │
│     ├─ Route XCM messages between PBCs                  │
│     └─ Governance for network-wide decisions            │
└─────────────────────────────────────────────────────────┘
                         ↓ ↑ XCM
┌─────────────────────────────────────────────────────────┐
│ AI-Compute-PBC (Sovereign Chain)                        │
│ ├─ Runtime: ai-compute-pbc-runtime (independent)        │
│ ├─ Pallets: GPU, Jobs, TEE, Models, Reputation, etc.   │
│ └─ Collators: 8+ nodes validating AI transactions       │
└─────────────────────────────────────────────────────────┘
```

**Interaction Model**:
- AI-Compute-PBC submits **state checkpoints** to FlareChain (every 256 blocks)
- FlareChain provides **economic finality** (via GRANDPA)
- No FlareChain code changes needed

### 5.2 State Checkpoints (How PBCs Report to FlareChain)

**Checkpoint Flow**:
```rust
// On AI-Compute-PBC (every 256 blocks)
impl OnFinalize<BlockNumber> for Pallet<T> {
    fn on_finalize(n: BlockNumber) {
        if n % 256 == 0 {
            // Calculate Merkle root of all AI Compute state
            let state_root = Self::calculate_state_root();

            // Send XCM message to FlareChain
            let message = Xcm(vec![
                Transact {
                    origin_kind: OriginKind::SovereignAccount,
                    call: FlareChainCall::RegisterCheckpoint {
                        pbc_id: 15, // AI-Compute-PBC ID
                        block_number: n,
                        state_root,
                    }.encode().into(),
                },
            ]);

            send_xcm::<XcmRouter>(Parent, message);
        }
    }
}
```

**FlareChain receives checkpoint** (no new code, uses existing checkpoint pallet):
```rust
// FlareChain already has pallet_checkpoint (from BTC-PBC, ETH-PBC, etc.)
// AI-Compute-PBC just reuses it

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Register a PBC checkpoint
    pub fn register_checkpoint(
        origin: OriginFor<T>,
        pbc_id: u8,
        block_number: u64,
        state_root: Hash,
    ) -> DispatchResult {
        // Verify sender is authorized PBC collator
        let sender = ensure_signed(origin)?;
        ensure!(Self::is_pbc_collator(pbc_id, sender), Error::<T>::NotAuthorized);

        // Store checkpoint
        Checkpoints::<T>::insert((pbc_id, block_number), state_root);

        Self::deposit_event(Event::CheckpointRegistered { pbc_id, block_number, state_root });
        Ok(())
    }
}
```

**Result**: FlareChain stores AI-Compute-PBC checkpoints **without knowing about AI pallets**.

### 5.3 Cross-Chain Queries (User Perspective)

**User wants to query AI job status**:
```javascript
// User connects to AI-Compute-PBC (not FlareChain)
const api = await ApiPromise.create({
  provider: new WsProvider('wss://ai-compute-pbc.etrid.io')
});

// Query job directly on AI-Compute-PBC
const job = await api.query.jobMarketplace.jobs(job_id);
console.log(job.status); // "Processing"

// No FlareChain interaction needed for queries
```

**User wants to verify checkpoint on FlareChain**:
```javascript
// Connect to FlareChain
const flareApi = await ApiPromise.create({
  provider: new WsProvider('wss://flarechain.etrid.io')
});

// Verify AI-Compute-PBC checkpoint is finalized
const checkpoint = await flareApi.query.checkpoint.checkpoints(15, block_number);
console.log(checkpoint); // state_root: 0xabc123...

// FlareChain confirms: "AI-Compute-PBC block 25600 is finalized"
```

---

## 6. Cross-Chain Communication

### 6.1 XCM Message Examples

**AI-Compute-PBC → FlareChain (Checkpoint)**
```rust
// Submit checkpoint every 256 blocks
let message = Xcm(vec![
    Transact {
        origin_kind: OriginKind::SovereignAccount,
        call: FlareChainCall::RegisterCheckpoint {
            pbc_id: 15,
            block_number: 25600,
            state_root: [0x12, 0x34, ...],
        }.encode().into(),
    },
]);
send_xcm::<XcmRouter>(Parent, message);
```

**FlareChain → AI-Compute-PBC (Governance Update)**
```rust
// FlareChain sends governance decision (e.g., "increase max jobs per block")
let message = Xcm(vec![
    Transact {
        origin_kind: OriginKind::Superuser,
        call: AIComputeCall::UpdateMaxJobs {
            new_max: 2000,
        }.encode().into(),
    },
]);
send_xcm::<XcmRouter>(Parachain(15), message);
```

**AI-Compute-PBC → EDSC-PBC (Payment)**
```rust
// User pays for AI inference using ËDSC on EDSC-PBC
// AI-Compute-PBC requests payment via XCM
let message = Xcm(vec![
    WithdrawAsset((Here, 100_000_000_000_000).into()),
    BuyExecution { fees: (Here, 1_000_000).into(), weight_limit: Unlimited },
    DepositAsset {
        assets: All.into(),
        beneficiary: gpu_owner_account.into(),
    },
]);
send_xcm::<XcmRouter>(Sibling(13), message); // EDSC-PBC is sibling PBC
```

### 6.2 XCM Configuration

**File**: `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/runtime/src/xcm_config.rs`

```rust
use frame_support::traits::Everything;
use xcm::v4::{prelude::*, Weight as XcmWeight};
use xcm_builder::{
    AccountId32Aliases, CurrencyAdapter, FixedWeightBounds, IsConcrete,
    ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
    SignedAccountId32AsNative, SovereignSignedViaLocation,
};
use xcm_executor::XcmExecutor;

pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainAsNative<RelayChainOrigin, AccountId>,
    AccountId32Aliases<RelayChainNetwork, AccountId>,
);

pub type LocalAssetTransactor = CurrencyAdapter<
    Balances,
    IsConcrete<RelayChainLocation>,
    LocationToAccountId,
    AccountId,
    (),
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = ();
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type Trader = ();
    type ResponseHandler = ();
    type AssetTrap = ();
    type AssetClaims = ();
    type SubscriptionService = ();
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type AssetLocker = ();
    type AssetExchanger = ();
    type FeeManager = ();
    type MessageExporter = ();
    type UniversalAliases = ();
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = ();
    type TransactionalProcessor = ();
}

pub type XcmRouter = (
    // Send to parent (FlareChain)
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem>,
    // Send to sibling PBCs
    cumulus_primitives_utility::XcmpQueue,
);
```

---

## 7. Deployment & Operations

### 7.1 Build Commands

```bash
# Build AI-Compute-PBC runtime
cd /Users/macbook/Desktop/etrid
cargo build --release -p ai-compute-pbc-runtime

# Build AI-Compute-PBC collator
cargo build --release --bin ai-compute-pbc-collator

# Verify WASM is generated
ls -lh target/release/wbuild/ai-compute-pbc-runtime/ai_compute_pbc_runtime.compact.compressed.wasm
```

### 7.2 Generate Chain Spec

```bash
# Generate raw chain spec
./target/release/ai-compute-pbc-collator build-spec \
  --chain ai-compute-dev \
  --raw > ai-compute-pbc-chainspec-raw.json

# Generate genesis WASM
./target/release/ai-compute-pbc-collator export-genesis-wasm \
  --chain ai-compute-pbc-chainspec-raw.json \
  > ai-compute-genesis.wasm

# Generate genesis state
./target/release/ai-compute-pbc-collator export-genesis-state \
  --chain ai-compute-pbc-chainspec-raw.json \
  > ai-compute-genesis-state
```

### 7.3 Run Local Testnet (3 Collators)

**Node 1**:
```bash
./target/release/ai-compute-pbc-collator \
  --alice \
  --collator \
  --force-authoring \
  --chain ai-compute-pbc-chainspec-raw.json \
  --base-path /tmp/ai-compute-collator1 \
  --port 30333 \
  --rpc-port 9945
```

**Node 2**:
```bash
./target/release/ai-compute-pbc-collator \
  --bob \
  --collator \
  --force-authoring \
  --chain ai-compute-pbc-chainspec-raw.json \
  --base-path /tmp/ai-compute-collator2 \
  --port 30334 \
  --rpc-port 9946
```

**Node 3**:
```bash
./target/release/ai-compute-pbc-collator \
  --charlie \
  --collator \
  --force-authoring \
  --chain ai-compute-pbc-chainspec-raw.json \
  --base-path /tmp/ai-compute-collator3 \
  --port 30335 \
  --rpc-port 9947
```

### 7.4 Register with FlareChain (Mainnet)

**Step 1: Reserve Para ID**
```javascript
// On FlareChain
const api = await ApiPromise.create({ /* FlareChain WS */ });
await api.tx.registrar.reserve().signAndSend(sudo_account);
// Assigned Para ID: 2015 (example)
```

**Step 2: Register Genesis**
```javascript
await api.tx.registrar.register(
  2015, // para_id
  genesis_head, // ai-compute-genesis-state
  genesis_wasm  // ai-compute-genesis.wasm
).signAndSend(sudo_account);
```

**Step 3: Start Collators**
```bash
./target/release/ai-compute-pbc-collator \
  --collator \
  --chain ai-compute-pbc-mainnet.json \
  --para-id 2015 \
  -- \
  --chain flarechain-mainnet.json \
  --execution wasm
```

### 7.5 Monitor & Maintain

**Health Checks**:
```bash
# Check if collator is running
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9945

# Query block height
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  http://localhost:9945
```

**Upgrade Runtime**:
```bash
# Build new runtime WASM
cargo build --release -p ai-compute-pbc-runtime

# Submit runtime upgrade via sudo
api.tx.sudo.sudoUncheckedWeight(
  api.tx.system.setCode(new_wasm),
  weight
).signAndSend(sudo_account);
```

---

## 8. Summary

### Key Takeaways

1. **No FlareChain changes**: AI Compute Network runs on dedicated `ai-compute-pbc` (15th PBC)
2. **Reuse existing infrastructure**: AIDID, DID Registry, Accounts pallets already exist
3. **Add 6 new pallets**: GPU Registry, Job Marketplace, Confidential Compute, AI Models, Reputation, Payment Streams
4. **Independent runtime**: ai-compute-pbc-runtime builds separately from FlareChain
5. **XCM communication**: PBCs communicate via Cross-Chain Messages (no FlareChain code changes)
6. **12-week implementation**: Phased rollout from foundation to confidential compute to testnet

### Benefits

✅ **Isolation**: AI Compute doesn't affect FlareChain or other PBCs
✅ **Scalability**: 5,000+ TPS dedicated to AI workloads
✅ **Upgradability**: Upgrade ai-compute-pbc independently
✅ **Flexibility**: Specialized gas pricing, block time, finality for AI jobs
✅ **No risk to FlareChain**: Existing validators unaffected

### Next Steps

**Week 1**:
- Create directory structure
- Copy pbc-runtime template
- Set up Cargo workspace

**Week 2-4**:
- Implement pallet-gpu-registry
- Write 15+ tests
- Build first WASM

**Week 5-10**:
- Implement pallet-job-marketplace
- Integration testing
- Local testnet deployment

**Week 11+**:
- Build collator binary
- Confidential compute implementation
- Public testnet launch

---

**End of Implementation Guide**

Ready to start building? 🚀
