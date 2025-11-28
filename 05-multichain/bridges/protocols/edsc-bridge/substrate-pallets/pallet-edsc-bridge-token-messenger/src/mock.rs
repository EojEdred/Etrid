//! Mock runtime for pallet-edsc-bridge-token-messenger tests

#![cfg(test)]

use frame_support::{
	derive_impl, parameter_types,
};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		TokenMessenger: crate,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

parameter_types! {
	pub const MaxMessageBodySize: u32 = 512;
	pub const MaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC
	pub const DailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000;  // 10M EDSC
	pub const MessageTimeout: u64 = 1000;
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxMessageBodySize = MaxMessageBodySize;
	type MaxBurnAmount = MaxBurnAmount;
	type DailyBurnCap = DailyBurnCap;
	type MessageTimeout = MessageTimeout;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	t.into()
}
