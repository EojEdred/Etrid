//! EDSC-PBC Runtime
//!
//! Dedicated Partition Burst Chain for Ëtrid Dollar Stablecoin (EDSC)
//! Uses ASF Consensus + Grandpa finality

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// Import common PBC runtime code from pbc-common
pub use pbc_common::*;

// Additional imports specific to EDSC
use sp_runtime::FixedU128;
use sp_arithmetic::Permill;

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

/// Opaque types.
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
            pub grandpa: Grandpa,
        }
    }
}

// Constant values used within the runtime.
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("edsc-pbc"),
    impl_name: create_runtime_str!("edsc-pbc"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// This determines the average expected block time that we are targeting.
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The existential deposit.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLICENTS;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    pub RuntimeBlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::with_sensible_defaults(
        Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
        NORMAL_DISPATCH_RATIO,
    );
    pub RuntimeBlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type AccountId = AccountId;
    type RuntimeCall = RuntimeCall;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
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

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type MaxNominators = ConstU32<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
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
    type ValidatorReward = ConstU128<1_000_000_000_000_000_000_000>; // 1 ETR per block
    type CommitteeSize = ConstU32<21>; // PPFA committee size
    type EpochDuration = ConstU32<2400>; // ~4 hours at 6s/block
    type BaseSlotDuration = ConstU64<6000>; // 6 seconds
}

// ================================
// EDSC Pallet Configurations
// ================================

// EDSC Token Configuration
impl pallet_edsc_token::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxSupply = ConstU128<50_000_000_000_000_000_000_000_000_000>;  // 50 billion EDSC (with 18 decimals)
    type MinBalance = ConstU128<1_000_000_000_000>;  // 0.000001 EDSC
    type WeightInfo = ();
}

// EDSC Receipts Configuration (SBT for purchase tracking)
impl pallet_edsc_receipts::Config for Runtime {
    type MaxReceiptsPerWallet = ConstU32<1000>;
    type ReceiptExpiryPeriod = ConstU32<5_256_000>;  // ~1 year in blocks
}

// EDSC Redemption Configuration (3-path redemption engine)
parameter_types! {
    pub const MinRedemptionFee: Permill = Permill::from_parts(2_500);  // 0.25%
    pub SafetyMultiplier: FixedU128 = FixedU128::from_rational(12u128, 10u128);  // 1.2x
    pub const Path1DailyLimit: u128 = 100_000_00;  // $100,000 in cents
    pub const Path2DailyLimit: u128 = 50_000_00;   // $50,000 in cents
    pub const Path3DailyLimit: u128 = 10_000_00;   // $10,000 in cents
    pub const HourlyRedemptionCap: Permill = Permill::from_parts(5_000);  // 0.5% of supply per hour
    pub const DailyRedemptionCap: Permill = Permill::from_parts(20_000);  // 2% of supply per day
    pub ThrottleReserveRatio: FixedU128 = FixedU128::from_rational(105u128, 100u128);  // 1.05 (105%)
    pub EmergencyReserveRatio: FixedU128 = FixedU128::from_rational(100u128, 100u128);  // 1.00 (100%)
    pub const MaxQueueSize: u32 = 10_000;
}

impl pallet_edsc_redemption::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinRedemptionFee = MinRedemptionFee;
    type SafetyMultiplier = SafetyMultiplier;
    type Path1DailyLimit = Path1DailyLimit;
    type Path2DailyLimit = Path2DailyLimit;
    type Path3DailyLimit = Path3DailyLimit;
    type HourlyRedemptionCap = HourlyRedemptionCap;
    type DailyRedemptionCap = DailyRedemptionCap;
    type ThrottleReserveRatio = ThrottleReserveRatio;
    type EmergencyReserveRatio = EmergencyReserveRatio;
    type MaxQueueSize = MaxQueueSize;
}



// EDSC Oracle No-Op Callback
pub struct NoOpPriceCallback;
impl pallet_edsc_oracle::PriceUpdateCallback for NoOpPriceCallback {
    fn on_price_updated(_price: u128) -> frame_support::dispatch::DispatchResult {
        Ok(())
    }
}


// EDSC Oracle Configuration (TWAP price oracle)
parameter_types! {
    pub const PrimaryTwapWindow: u32 = 14_400;  // 24 hours in blocks (6s blocks)
    pub const FallbackTwapWindow: u32 = 100_800;  // 7 days in blocks
    pub const MinPriceSources: u32 = 5;  // Binance, Coinbase, Kraken, Bitstamp, Gemini
    pub const OutlierThreshold: Permill = Permill::from_parts(20_000);  // 2% deviation
    pub const StalenessTimeout: u32 = 100;  // blocks
    pub const MaxPriceHistory: u32 = 10_000;  // historical price records
}

impl pallet_edsc_oracle::Config for Runtime {
    type PrimaryTwapWindow = PrimaryTwapWindow;
    type FallbackTwapWindow = FallbackTwapWindow;
    type MinPriceSources = MinPriceSources;
    type OutlierThreshold = OutlierThreshold;
    type StalenessTimeout = StalenessTimeout;
    type MaxPriceHistory = MaxPriceHistory;
    type PriceCallback = NoOpPriceCallback;
}

// EDSC Checkpoint Provider Implementations
impl pallet_edsc_checkpoint::TotalSupplyProvider for Runtime {
    fn get_total_supply() -> u128 {
        pallet_edsc_token::Pallet::<Runtime>::total_supply()
    }
}

impl pallet_edsc_checkpoint::ReserveRatioProvider for Runtime {
    fn get_reserve_ratio() -> u16 {
        // Default to 100% (10000 basis points) - can be connected to vault pallet later
        10000
    }
}


// EDSC Checkpoint Configuration (State synchronization with FlareChain)
parameter_types! {
    pub const CheckpointInterval: u32 = 100;  // Create checkpoint every 100 blocks (~10 minutes)
    pub const MaxCheckpoints: u32 = 10_000;  // Max stored checkpoints
    pub const EmergencyReserveThreshold: u16 = 10000;  // 100% (10000 basis points) - trigger emergency checkpoint if below
}

impl pallet_edsc_checkpoint::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type CheckpointInterval = CheckpointInterval;
    type MaxCheckpoints = MaxCheckpoints;
    type EmergencyReserveThreshold = EmergencyReserveThreshold;
    type TotalSupplyProvider = Runtime;
    type ReserveRatioProvider = Runtime;
}

// Circuit Breaker Configuration (Emergency safety controls)
parameter_types! {
    pub const MaxHourlyVolume: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC (with 18 decimals)
    pub const MaxDailyVolume: u128 = 5_000_000_000_000_000_000_000_000;  // 5M EDSC (with 18 decimals)
    pub const ThrottleThreshold: u16 = 10500;  // 105% (throttle redemptions if reserve ratio < 105%)
    pub const CircuitBreakerEmergencyThreshold: u16 = 10000;  // 100% (pause if reserve ratio < 100%)
    pub const BlocksPerHour: u32 = 600;  // 1 hour = 600 blocks (6s blocks)
    pub const BlocksPerDay: u32 = 14_400;  // 1 day = 14400 blocks
}

impl pallet_circuit_breaker::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxHourlyVolume = MaxHourlyVolume;
    type MaxDailyVolume = MaxDailyVolume;
    type ThrottleThreshold = ThrottleThreshold;
    type EmergencyThreshold = CircuitBreakerEmergencyThreshold;
    type BlocksPerHour = BlocksPerHour;
    type BlocksPerDay = BlocksPerDay;
}

// XCM Bridge Configuration (Cross-chain messaging with FlareChain)
parameter_types! {
    pub const MaxPayloadSize: u32 = 1024;  // Max message size (bytes)
    pub const MessageTimeout: u32 = 1_000;  // Message expiry (blocks)
    pub const MaxPendingMessages: u32 = 1_000;  // Max queue size
    pub const ChainIdentifier: pallet_xcm_bridge::ChainId = pallet_xcm_bridge::ChainId::PbcEdsc;
}

impl pallet_xcm_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxPayloadSize = MaxPayloadSize;
    type MessageTimeout = MessageTimeout;
    type MaxPendingMessages = MaxPendingMessages;
    type ChainIdentifier = ChainIdentifier;
}

// Phase 3: External Bridge Protocol (CCTP-style)
parameter_types! {
    pub const TokenMessengerMaxMessageBodySize: u32 = 512;
    pub const TokenMessengerMaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC per tx
    pub const TokenMessengerDailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000;  // 10M EDSC per day
    pub const TokenMessengerMessageTimeout: u32 = 1000;
}

impl pallet_edsc_bridge_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxMessageBodySize = TokenMessengerMaxMessageBodySize;
    type MaxBurnAmount = TokenMessengerMaxBurnAmount;
    type DailyBurnCap = TokenMessengerDailyBurnCap;
    type MessageTimeout = TokenMessengerMessageTimeout;
}

parameter_types! {
    pub const MaxAttesters: u32 = 100;  // Maximum registered attesters
    pub const MaxAttestersPerMessage: u32 = 10;  // Maximum signatures per message
    pub const MinSignatureThreshold: u32 = 3;  // Default M-of-N (3-of-5)
    pub const AttestationMaxAge: u32 = 1000;  // 1000 blocks (~100 minutes)
}

impl pallet_edsc_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAttesters = MaxAttesters;
    type MaxAttestersPerMessage = MaxAttestersPerMessage;
    type MinSignatureThreshold = MinSignatureThreshold;
    type AttestationMaxAge = AttestationMaxAge;
}

// Lightning Bloc Channels Configuration
parameter_types! {
    // Minimum channel capacity: 100 EDSC (stablecoins = smaller amounts)
    pub const MinChannelCapacity: Balance = 100_000_000_000_000_000_000;
    // Maximum channel capacity: 1M EDSC
    pub const MaxChannelCapacity: Balance = 1_000_000_000_000_000_000_000_000;
    // Channel timeout: 14400 blocks (~24 hours at 6s blocks)
    pub const ChannelTimeout: BlockNumber = 14400;
}

impl pallet_lightning_channels::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinChannelCapacity = MinChannelCapacity;
    type MaxChannelCapacity = MaxChannelCapacity;
    type ChannelTimeout = ChannelTimeout;
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
        Timestamp: pallet_timestamp,

        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,

        // Ëtrid Core
        Consensus: pallet_consensus,

        // EDSC pallets (Ëtrid Dollar Stablecoin system - PBC-specific pallets)
        EdscToken: pallet_edsc_token,
        EdscReceipts: pallet_edsc_receipts,
        EdscRedemption: pallet_edsc_redemption,
        EdscOracle: pallet_edsc_oracle,
        EdscCheckpoint: pallet_edsc_checkpoint,
        CircuitBreaker: pallet_circuit_breaker,
        XcmBridge: pallet_xcm_bridge,

        // Phase 3: External Bridge Protocol (CCTP-style)
        TokenMessenger: pallet_edsc_bridge_token_messenger,
        BridgeAttestation: pallet_edsc_bridge_attestation,

        // Lightning Bloc Channels for instant EDSC transfers
        LightningChannels: pallet_lightning_channels,
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

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
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
            _authority_id: sp_consensus_grandpa::AuthorityId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
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

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            frame_support::genesis_builder_helper::build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            frame_support::genesis_builder_helper::get_preset::<RuntimeGenesisConfig>(id, |_name| {
                None
            })
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }
}
