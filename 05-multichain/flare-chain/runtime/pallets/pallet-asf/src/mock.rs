//! Mock runtime for pallet-asf testing

use crate as pallet_asf;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, Everything},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Asf: pallet_asf,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sp_core::crypto::AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
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
    pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1000;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxValidators: u32 = 100;
    pub const MinValidatorStake: u128 = 1_000_000;
    pub const EpochDuration: u64 = 100;
}

impl pallet_asf::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxValidators = MaxValidators;
    type MinValidatorStake = MinValidatorStake;
    type SlashHandler = ();
    type FinalityNotifier = ();
    type EpochDuration = EpochDuration;
}

// Build genesis storage according to the mock runtime
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Configure initial validators for ASF pallet
    pallet_asf::GenesisConfig::<Test> {
        validators: vec![
            // 21 validators with equal stake
            (validator_id(1), 10_000_000),
            (validator_id(2), 10_000_000),
            (validator_id(3), 10_000_000),
            (validator_id(4), 10_000_000),
            (validator_id(5), 10_000_000),
            (validator_id(6), 10_000_000),
            (validator_id(7), 10_000_000),
            (validator_id(8), 10_000_000),
            (validator_id(9), 10_000_000),
            (validator_id(10), 10_000_000),
            (validator_id(11), 10_000_000),
            (validator_id(12), 10_000_000),
            (validator_id(13), 10_000_000),
            (validator_id(14), 10_000_000),
            (validator_id(15), 10_000_000),
            (validator_id(16), 10_000_000),
            (validator_id(17), 10_000_000),
            (validator_id(18), 10_000_000),
            (validator_id(19), 10_000_000),
            (validator_id(20), 10_000_000),
            (validator_id(21), 10_000_000),
        ],
        _phantom: Default::default(),
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    storage.into()
}

// Helper functions for tests

/// Create a validator ID from a u32
pub fn validator_id(id: u32) -> sp_core::crypto::AccountId32 {
    sp_core::crypto::AccountId32::from([id as u8; 32])
}

/// Create a test block hash
pub fn test_block_hash(id: u8) -> H256 {
    H256::from([id; 32])
}

/// Create a test vote
pub fn create_test_vote(
    validator: sp_core::crypto::AccountId32,
    block_hash: H256,
    phase: pallet_asf::ConsensusPhase,
    stake: u128,
) -> pallet_asf::Vote {
    use pallet_asf::{Signature, Vote};

    // Create a dummy signature for testing
    let signature = Signature::from_sr25519_bytes([0u8; 64]);

    Vote::new(
        block_hash,
        1, // block_number
        phase,
        validator,
        stake,
        0, // epoch
        0, // timestamp
        signature,
    )
}

/// Create a test certificate
pub fn create_test_certificate(
    validator: sp_core::crypto::AccountId32,
    block_hash: H256,
    phase: pallet_asf::ConsensusPhase,
    votes: &[pallet_asf::Vote],
) -> pallet_asf::Certificate {
    use pallet_asf::Certificate;

    Certificate::from_votes(
        votes,
        validator,
        10_000_000,
        0, // epoch
        0, // timestamp
    )
}
