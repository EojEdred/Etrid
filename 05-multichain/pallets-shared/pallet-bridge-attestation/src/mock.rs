//! Mock runtime for pallet-bridge-attestation tests

use crate as pallet_bridge_attestation;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64, EnsureRoot},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		BridgeAttestation: pallet_bridge_attestation,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
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
	pub const TestChainId: u32 = 1;
	pub const MaxAttesters: u32 = 100;
	pub const MaxAttestersPerMessage: u32 = 20;
	pub const MinSignatureThreshold: u32 = 2;
	pub const AttestationMaxAge: u64 = 1000;
}

impl pallet_bridge_attestation::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ChainId = TestChainId;
	type MaxAttesters = MaxAttesters;
	type MaxAttestersPerMessage = MaxAttestersPerMessage;
	type MinSignatureThreshold = MinSignatureThreshold;
	type AttestationMaxAge = AttestationMaxAge;
	type AdminOrigin = EnsureRoot<u64>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	t.into()
}
