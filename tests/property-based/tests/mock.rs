//! Shared mock runtime for property-based tests
//!
//! This module provides a common test runtime configuration that can be used
//! across all property-based tests to ensure consistency.

use frame_support::{derive_impl, parameter_types, traits::ConstU32};
use sp_runtime::BuildStorage;

// Re-export commonly used types
pub use frame_system::mocking::MockBlock;
pub use sp_runtime::testing::H256;

pub type AccountId = u64;
pub type Balance = u128;
pub type Block = MockBlock<Runtime>;

// Test account constants
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type AccountId = AccountId;
}

/// Create new test externalities for property tests
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

/// Helper function to run a test within the test externalities
pub fn run_test<R>(test: impl FnOnce() -> R) -> R {
	new_test_ext().execute_with(test)
}
