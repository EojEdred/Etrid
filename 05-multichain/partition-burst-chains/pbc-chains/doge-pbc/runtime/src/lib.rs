//! DOGE-PBC RUNTIME - Dogecoin Partition Burst Chain
//! Integrates: Dogecoin Bridge + Lightning Channels + ASF Consensus
//!          lightning-channels from 05-multichain/lightning-bloc-networks/

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod asf_config;

use pallet_session::disabling::UpToLimitDisablingStrategy;
use sp_core::crypto::KeyTypeId;
use sp_runtime::traits::OpaqueKeys;

// Import common PBC runtime code from pbc-common
pub use pbc_common::*;

// Re-export Dogecoin bridge pallet
pub use pallet_doge_bridge;
pub use pallet_validator_committee;
pub use pallet_validator_rewards;
pub use pallet_etrid_staking;
pub use pallet_bridge_attestation;
pub use pallet_token_messenger;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

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

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

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
            // ASF manages consensus internally - no session keys needed
        }
    }
}

/// Empty session handler for ASF consensus
pub struct EmptySessionHandler;

impl pallet_session::SessionHandler<AccountId> for EmptySessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[];

    fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AccountId, Ks)]) {
        // No-op: ASF handles validator initialization via ValidatorCommittee
    }

    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AccountId, Ks)],
        _queued_validators: &[(AccountId, Ks)],
    ) {
        // No-op: ASF handles validator rotation via ValidatorCommittee
    }

    fn on_disabled(_validator_index: u32) {
        // No-op: ASF handles validator disabling via ValidatorCommittee
    }
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("btc-pbc"),
    impl_name: create_runtime_str!("btc-pbc"),
    authoring_version: 1,
    spec_version: 101,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000; // 6 seconds per block for PBC

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub RuntimeBlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::with_sensible_defaults(
        Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
        NORMAL_DISPATCH_RATIO,
    );
    pub RuntimeBlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The nonce type for storing how many extrinsics an account has signed.
    type Nonce = Nonce;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain.
    type SS58Prefix = SS58Prefix;
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Block = Block;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type ExtensionsWeightInfo = ();
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}





impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

// ASF Consensus Configuration
parameter_types! {
    pub const MaxValidators: u32 = 100;
    pub const SessionDuration: BlockNumber = 10 * MINUTES;
}

impl pallet_consensus::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RandomnessSource = RandomnessCollectiveFlip;
    type Time = Timestamp;
    type MinValidityStake = ConstU128<64_000_000_000_000_000_000_000>; // 64 ETR
    type ValidatorReward = ConstU128<100_000_000_000_000_000_000>; // 0.1 ETR per block
    type CommitteeSize = ConstU32<21>; // PPFA committee size
    type EpochDuration = ConstU32<2400>; // ~4 hours at 6s/block
    type BaseSlotDuration = ConstU64<6000>; // 6 seconds
}

impl pallet_validator_committee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxCommitteeSize = asf_config::AsfMaxCommitteeSize;
    type MinValidatorStake = asf_config::AsfMinValidatorStake;
}

impl pallet_validator_rewards::Config for Runtime {
    type Currency = Balances;
    type EpochDuration = asf_config::AsfEpochDuration;
    type AnnualRewardPoolBps = ConstU32<1_000>; // 10% annual reward pool
    type ValidatorShareBps = ConstU32<9_000>; // 90% of reward pool goes to validators
}

parameter_types! {
    pub const Period: u32 = 600; // 600 blocks = 1 hour at 6s blocks
    pub const Offset: u32 = 0;
    pub TreasuryAccountForStaking: AccountId = AccountId::new([42u8; 32]);
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = pallet_validator_committee::ValidatorIdOf<Self>;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = ValidatorCommittee;
    type SessionHandler = EmptySessionHandler;
    type Keys = opaque::SessionKeys;
    type WeightInfo = ();
    type DisablingStrategy = UpToLimitDisablingStrategy;
}

impl pallet_etrid_staking::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UnbondPeriod = ConstU32<28800>; // ~2 days at 6 second blocks
    type MaxUnbondingEntries = ConstU32<32>; // Max unbonding entries per account
    type TreasuryAccount = TreasuryAccountForStaking;
    type ValidatorRewards = Runtime;
}
// DogeBridge Configuration
use frame_support::PalletId;
use sp_runtime::Perbill;

parameter_types! {
    pub const DogeBridgeFee: Perbill = Perbill::from_percent(1);
    pub const MinDogeBridgeAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxDogeBridgeAmount: Balance = 1_000_000_000_000; // 1M ETR
    pub const DogeBridgePalletId: PalletId = PalletId(*b"doge/brd");
    pub const DogeConfirmations: u32 = 20;
    pub const DogeConversionRate: u64 = 1_000_000; // 1 DOGE = 0.001 ETR
}

// Lock identifier for ETR locking
const ETR_LOCK_ID: [u8; 8] = *b"etr/lock";

// ETR Lock Configuration (required by bridge pallets)
parameter_types! {
    pub const MinLockAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxLockAmount: Balance = 1_000_000_000_000_000; // 1M ETR
    pub const LockPeriod: BlockNumber = 7 * DAYS;
}

parameter_types! {
    pub const EtrLockId: [u8; 8] = ETR_LOCK_ID;
}



impl pallet_etr_lock::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxLockAmount = MaxLockAmount;
    type BridgeOrigin = frame_system::EnsureRoot<AccountId>;
    type LockIdentifier = EtrLockId;
}


parameter_types! {
    pub const BridgeAuthorityAccount: AccountId = AccountId::new([0u8; 32]);
}

impl pallet_doge_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeFee = DogeBridgeFee;
    type MinBridgeAmount = MinDogeBridgeAmount;
    type MaxBridgeAmount = MaxDogeBridgeAmount;
    type PalletId = DogeBridgePalletId;
    type DogeConfirmations = DogeConfirmations;
    type DogeConversionRate = DogeConversionRate;
}


// Lightning Channels Configuration
parameter_types! {
    pub const MinChannelCapacity: Balance = 1_000_000; // 0.001 ETR
    pub const MaxChannelCapacity: Balance = 1_000_000_000; // 1000 ETR
    pub const ChannelTimeout: BlockNumber = 24 * HOURS;
}

impl pallet_lightning_channels::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinChannelCapacity = MinChannelCapacity;
    type MaxChannelCapacity = MaxChannelCapacity;
    type ChannelTimeout = ChannelTimeout;
}

// Bridge Attestation Configuration
parameter_types! {
    pub const LocalDomain: u32 = 107; // DOGE-PBC domain ID
    pub const MaxAttesters: u32 = 100;
    pub const MaxAttestersPerMessage: u32 = 10;
    pub const MinSignatureThreshold: u32 = 3;
    pub const AttestationMaxAge: BlockNumber = 1000;
}

impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = LocalDomain;
    type MaxAttesters = MaxAttesters;
    type MaxAttestersPerMessage = MaxAttestersPerMessage;
    type MinSignatureThreshold = MinSignatureThreshold;
    type AttestationMaxAge = AttestationMaxAge;
    type AdminOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

// Token Messenger Configuration
parameter_types! {
    pub const MaxMessageBodySize: u32 = 512;
    pub const MaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000; // 1M tokens
    pub const DailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000; // 10M tokens
    pub const MinBurnAmount: u128 = 1_000_000_000_000; // 0.000001 tokens
    pub const MessageTimeout: BlockNumber = 1000;
    pub const BlocksPerDay: BlockNumber = DAYS;
    pub const BridgeFeeRate: u32 = 30; // 0.3% fee
    pub const MinBridgeFee: u128 = 1_000_000_000_000; // 0.000001 tokens minimum fee
    pub FeeCollector: AccountId = AccountId::new([0u8; 32]);
}

impl pallet_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = pallet_token_messenger::token_ops::PbcTokenOperations<Runtime>;
    type AttestationVerifier = pallet_token_messenger::attestation::BridgeAttestationVerifier<Runtime>;
    type WeightInfo = pallet_token_messenger::weights::SubstrateWeight<Runtime>;
    type Currency = Balances;
    type MaxMessageBodySize = MaxMessageBodySize;
    type MaxBurnAmount = MaxBurnAmount;
    type DailyBurnCap = DailyBurnCap;
    type MinBurnAmount = MinBurnAmount;
    type MessageTimeout = MessageTimeout;
    type BlocksPerDay = BlocksPerDay;
    type LocalDomain = LocalDomain;
    type BridgeFeeRate = BridgeFeeRate;
    type MinBridgeFee = MinBridgeFee;
    type FeeCollector = FeeCollector;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime
    where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
        Timestamp: pallet_timestamp,Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        
        // Ëtrid Core
        Consensus: pallet_consensus,
        EtrLock: pallet_etr_lock,

        // ASF Consensus
        Session: pallet_session,
        ValidatorCommittee: pallet_validator_committee,
        ValidatorRewards: pallet_validator_rewards,
        EtridStaking: pallet_etrid_staking,

        // Bitcoin Bridge & Lightning
        DogeBridge: pallet_doge_bridge,
        LightningChannels: pallet_lightning_channels,

        // Shared Bridge Pallets
        BridgeAttestation: pallet_bridge_attestation,
        TokenMessenger: pallet_token_messenger,
    }
);

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

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
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

    impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime {
        fn committee() -> Vec<AccountId> {
            Consensus::committee()
        }

        fn ppfa_index() -> u32 {
            Consensus::ppfa_index()
        }

        fn slot_duration() -> sp_consensus_asf::SlotDuration {
            sp_consensus_asf::SlotDuration::from_millis(Consensus::slot_duration())
        }

        fn should_propose(validator: AccountId) -> bool {
            Consensus::should_propose(validator)
        }

        fn current_epoch() -> u32 {
            Consensus::current_epoch()
        }

        fn active_validators() -> Vec<AccountId> {
            Consensus::active_validators()
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

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            frame_support::genesis_builder_helper::build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            frame_support::genesis_builder_helper::get_preset::<RuntimeGenesisConfig>(id, |name| {
                match name.as_ref() {
                    sp_genesis_builder::DEV_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/development.json").to_vec())
                    },
                    sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/local_testnet.json").to_vec())
                    },
                    _ => None,
                }
            })
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![
                sp_genesis_builder::DEV_RUNTIME_PRESET.into(),
                sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET.into(),
            ]
        }
    }
}
