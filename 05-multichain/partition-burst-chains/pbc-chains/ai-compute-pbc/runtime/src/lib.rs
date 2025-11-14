//! # AI Compute PBC Runtime
//!
//! The runtime for Ëtrid's AI Compute Network (15th Partition Burst Chain).
//!
//! ## Core Pallets
//! - GPU Registry: GPU provider registration + staking
//! - Job Marketplace: AI job submission + matching
//! - Model Registry: AI model catalog with AIDID
//! - Confidential Compute: TEE attestation
//! - Lightning Payment: Streaming micropayments
//! - AI Reputation: ML-based scoring
//!
//! ## Advanced Features
//! - Prompt Marketplace: NFT-based prompt trading
//! - Dispute Arbitration: Staked judge system
//! - Federated Learning: Privacy-preserving training
//! - Tokenomics: ËTRD staking tiers (8% APY)
//! - GPU NFT: Tradeable GPU certificates
//! - Compliance: HIPAA/GDPR/SOC2 templates
//! - SLA Insurance: 99.9% uptime guarantee
//!
//! Plus standard Substrate pallets (System, Balances, etc.)

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU8},
    weights::{IdentityFee, Weight},
};
use frame_system as system;

pub use frame_support::{
    traits::KeyOwnerProofSystem,
    weights::constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;

/// Opaque types for block + opaque_keys
pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

// Type aliases
pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type AccountIndex = u32;
pub type Balance = u128;
pub type Nonce = u32;
pub type Hash = sp_core::H256;

/// Runtime version
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("ai-compute-pbc"),
    impl_name: create_runtime_str!("ai-compute-pbc"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// Native version
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const BlockHashCount: BlockNumber = 2400;
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
        ::with_sensible_defaults(
            Weight::from_parts(2_000_000_000_000, u64::MAX),
            NORMAL_DISPATCH_RATIO,
        );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure frame_system
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type RuntimeCall = RuntimeCall;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Block = opaque::Block;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const ExistentialDeposit: u128 = 500;
}

// Configure pallet_balances
impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeFreezeReason = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 3_000; // 3 seconds (half of 6-second block time)
}

// Configure pallet_timestamp
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

// Configure pallet_aura (consensus)
impl pallet_aura::Config for Runtime {
    type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

// Configure pallet_grandpa (finality)
impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

// Configure pallet_sudo
impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

// Configure pallet_transaction_payment
impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = ConstU8<5>;
}

// Configure pallet_accounts (Ëtrid custom)
impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// ========================================
// AI COMPUTE PBC PALLETS
// ========================================

// Configure pallet_gpu_registry
impl pallet_gpu_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinimumStake = ConstU128<100_000_000_000_000_000_000>; // 100 ËDSC
    type SlashPercentage = ConstU16<1000>; // 10% slash
    type MaxOfflineBlocks = ConstU32<1000>; // ~100 minutes offline
}

// Configure pallet_job_marketplace
impl pallet_job_marketplace::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type PlatformFeeBps = ConstU16<500>; // 5% platform fee
    type MaxJobDuration = ConstU32<10000>; // Max job duration in blocks
}

// Configure pallet_model_registry
impl pallet_model_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RegistrationFee = ConstU128<10_000_000_000_000_000_000>; // 10 ËDSC
    type MaxRoyaltyBps = ConstU16<5000>; // Max 50% royalty
}

// Configure pallet_confidential_compute
impl pallet_confidential_compute::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// Configure pallet_lightning_payment
impl pallet_lightning_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

// Configure pallet_ai_reputation
impl pallet_ai_reputation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// ========================================
// ADVANCED FEATURE PALLETS
// ========================================

// Configure pallet_prompt_marketplace
impl pallet_prompt_marketplace::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinPromptPrice = ConstU128<1_000_000_000_000_000_000>; // 1 ËDSC minimum
}

// Configure pallet_dispute_arbitration
impl pallet_dispute_arbitration::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ArbitratorStake = ConstU128<1000_000_000_000_000_000_000>; // 1,000 ËTRD
    type DisputeFee = ConstU128<5_000_000_000_000_000_000>; // 5 ËDSC
    type SlashPercentage = ConstU16<1000>; // 10% slash for dishonest arbitrators
}

// Configure pallet_federated_learning
impl pallet_federated_learning::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

// Configure pallet_tokenomics
impl pallet_tokenomics::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type StakingAPYBps = ConstU16<800>; // 8% APY
    type MinStakeDuration = ConstU32<100800>; // ~7 days in blocks
}

// Configure pallet_gpu_nft
impl pallet_gpu_nft::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

// Configure pallet_compliance
impl pallet_compliance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// Configure pallet_sla_insurance
impl pallet_sla_insurance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinInsuranceStake = ConstU128<50_000_000_000_000_000_000>; // 50 ËDSC
    type RefundMultiplier = ConstU8<10>; // 10x refund on SLA violation
}

// ========================================
// CONSTRUCT RUNTIME
// ========================================

construct_runtime!(
    pub struct Runtime {
        // Substrate Standard Pallets
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,

        // Ëtrid Custom Pallets
        Accounts: pallet_accounts,

        // AI Compute PBC Core Pallets
        GpuRegistry: pallet_gpu_registry,
        JobMarketplace: pallet_job_marketplace,
        ModelRegistry: pallet_model_registry,
        ConfidentialCompute: pallet_confidential_compute,
        LightningPayment: pallet_lightning_payment,
        AiReputation: pallet_ai_reputation,

        // AI Compute PBC Advanced Features
        PromptMarketplace: pallet_prompt_marketplace,
        DisputeArbitration: pallet_dispute_arbitration,
        FederatedLearning: pallet_federated_learning,
        Tokenomics: pallet_tokenomics,
        GpuNft: pallet_gpu_nft,
        Compliance: pallet_compliance,
        SlaInsurance: pallet_sla_insurance,
    }
);

// ========================================
// RUNTIME APIs
// ========================================

/// Executive: handles dispatch to pallets
pub type Executive = frame_executive::Executive<
    Runtime,
    opaque::Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

impl_runtime_apis! {
    impl sp_api::Core<opaque::Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: opaque::Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<opaque::Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<opaque::Block> for Runtime {
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

    impl sp_block_builder::BlockBuilder<opaque::Block> for Runtime {
        fn apply_extrinsic(extrinsic: <opaque::Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <opaque::Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<opaque::Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: opaque::Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<opaque::Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <opaque::Block as BlockT>::Extrinsic,
            block_hash: <opaque::Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<opaque::Block> for Runtime {
        fn offchain_worker(header: &<opaque::Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<opaque::Block, sp_consensus_aura::sr25519::AuthorityId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<sp_consensus_aura::sr25519::AuthorityId> {
            pallet_aura::Authorities::<Runtime>::get().into_inner()
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<opaque::Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <opaque::Block as BlockT>::Hash,
                sp_runtime::traits::NumberFor<opaque::Block>,
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

    impl sp_session::SessionKeys<opaque::Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<opaque::Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<opaque::Block, Balance> for Runtime {
        fn query_info(
            uxt: <opaque::Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }

        fn query_fee_details(
            uxt: <opaque::Block as BlockT>::Extrinsic,
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
    impl frame_benchmarking::Benchmark<opaque::Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);
            let storage_info = AllPalletsWithSystem::storage_info();
            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use frame_support::traits::TrackedStorageKey;

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

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<opaque::Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, BlockWeights::get().max_block)
        }

        fn execute_block(
            block: opaque::Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }
}

// Missing types that need to be added
use sp_runtime::traits::ConstBool;
use sp_runtime::traits::ConstU16;
