//! Mock runtime for testing pallet-token-messenger

use crate as pallet_token_messenger;
use frame_support::{
	parameter_types,
	traits::ConstU32,
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
		TokenMessenger: pallet_token_messenger,
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
	type ExtensionsWeightInfo = ();
}

// Mock token operations
pub struct MockTokenOperations;
impl pallet_token_messenger::TokenOperations<u64> for MockTokenOperations {
	fn burn_tokens(_account: &u64, _amount: u128) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}

	fn mint_tokens(_account: &u64, _amount: u128) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}

	fn balance_of(_account: &u64) -> u128 {
		100_000_000_000_000_000_000_000_u128 // 100000 tokens with 18 decimals (plenty for tests)
	}
}

// Mock attestation verifier
pub struct MockAttestationVerifier;
impl pallet_token_messenger::AttestationVerifier for MockAttestationVerifier {
	fn verify_message_attestation(
		_message: &[u8],
		_attestation: &[u8],
		_message_hash: sp_core::H256,
	) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}
}

parameter_types! {
	pub const MaxMessageBodySize: u32 = 512;
	pub const MaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000_u128; // 1M tokens
	pub const DailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000_u128; // 10M tokens
	pub const MinBurnAmount: u128 = 1_000_000_000_000_000_u128; // 0.001 tokens
	pub const MessageTimeout: u64 = 14400; // 1 day in blocks
	pub const BlocksPerDay: u64 = 14400; // 6s block time
	pub const LocalDomain: u32 = 100; // EDSC PBC domain for testing
}

impl pallet_token_messenger::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TokenOperations = MockTokenOperations;
	type AttestationVerifier = MockAttestationVerifier;
	type WeightInfo = ();
	type MaxMessageBodySize = MaxMessageBodySize;
	type MaxBurnAmount = MaxBurnAmount;
	type DailyBurnCap = DailyBurnCap;
	type MinBurnAmount = MinBurnAmount;
	type MessageTimeout = MessageTimeout;
	type BlocksPerDay = BlocksPerDay;
	type LocalDomain = LocalDomain;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}
