// LINK-PBC RUNTIME - Chainlink Partition Burst Chain
// Integrates: Chainlink Bridge + Oracle Services + Lightning Channels + ASF Consensus + VMw
// Priority Oracle Network - Decentralized oracle services, VRF, Proof of Reserve

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Re-export dependencies
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness, StorageInfo},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system as system;
pub use pallet_balances as balances;
pub use pallet_timestamp as timestamp;
pub use sp_api::impl_runtime_apis;
pub use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
pub use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
pub use sp_std::prelude::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// Custom pallets
pub use chainlink_bridge;
// pub use pallet_lightning_channels; // TODO: Implement lightning channels

pub use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Index = u32;
pub type Hash = sp_core::H256;
pub type BlockNumber = u32;
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type SignedBlock = generic::SignedBlock<Block>;
pub type BlockId = generic::BlockId<Block>;

pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
);

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {}
    }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("link-pbc"),
    impl_name: create_runtime_str!("link-pbc"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

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
        ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type Call = Call;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type Origin = Origin;
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
}

parameter_types! {
    pub const MinimumPeriod: u64 = 6000; // 12 second blocks (matching Ethereum block time)
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

// ============================================================
// CHAINLINK BRIDGE CONFIGURATION
// ============================================================

parameter_types! {
    /// Ethereum requires 12 confirmations
    pub const LinkMinConfirmations: u32 = 12;

    /// Bridge fee: 1% (100 basis points)
    pub const LinkBridgeFeeRate: u32 = 10;

    /// Maximum oracle nodes
    pub const MaxOracleNodes: u32 = 100;

    /// Maximum data feeds
    pub const MaxDataFeeds: u32 = 50;

    /// Maximum VRF requests
    pub const MaxVRFRequests: u32 = 100;

    /// Oracle price staleness threshold (300 blocks ~5 min)
    pub const PriceStalenessThreshold: u32 = 300;
}

impl chainlink_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = LinkMinConfirmations;
    type BridgeFeeRate = LinkBridgeFeeRate;
    type MaxOracleNodes = MaxOracleNodes;
    type MaxDataFeeds = MaxDataFeeds;
    type MaxVRFRequests = MaxVRFRequests;
    type PriceStalenessThreshold = PriceStalenessThreshold;
}

// ============================================================
// LIGHTNING CHANNELS CONFIGURATION (TODO: Implement)
// ============================================================

// parameter_types! {
//     /// Minimum channel capacity: 100 LINK
//     pub const MinChannelCapacity: Balance = 100_000_000_000_000_000_000; // 100 LINK (18 decimals)
//
//     /// Maximum channel capacity: 100,000 LINK
//     pub const MaxChannelCapacity: Balance = 100_000_000_000_000_000_000_000; // 100k LINK
//
//     /// Channel timeout: 1 day (7,200 blocks at 12s/block)
//     pub const ChannelTimeout: BlockNumber = 7_200;
// }

// impl pallet_lightning_channels::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type MinChannelCapacity = MinChannelCapacity;
//     type MaxChannelCapacity = MaxChannelCapacity;
//     type ChannelTimeout = ChannelTimeout;
// }

// ============================================================
// CONSTRUCT RUNTIME
// ============================================================

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,

        // LINK-PBC Specific
        ChainlinkBridge: chainlink_bridge,
        // LightningChannels: pallet_lightning_channels, // TODO: Implement
    }
);

// ============================================================
// RUNTIME APIs
// ============================================================

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

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_version() {
        assert_eq!(VERSION.spec_name, "link-pbc");
        assert_eq!(VERSION.spec_version, 100);
    }

    #[test]
    fn test_chainlink_confirmations() {
        // Chainlink runs on Ethereum - requires 12 confirmations
        assert_eq!(LinkMinConfirmations::get(), 12);
    }

    #[test]
    fn test_link_block_time() {
        // 12 second blocks (matching Ethereum block time)
        assert_eq!(MinimumPeriod::get(), 6000);
    }
}
