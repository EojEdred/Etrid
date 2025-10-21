//! Mock runtime for pallet-xcm-bridge tests

#![cfg(test)]

use frame_support::{
	derive_impl, parameter_types,
};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		XcmBridge: crate,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

parameter_types! {
	pub const MaxPayloadSize: u32 = 1024;
	pub const MessageTimeout: u64 = 100;
	pub const MaxPendingMessages: u32 = 100;
	pub const ChainIdentifier: crate::ChainId = crate::ChainId::FlareChain;
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxPayloadSize = MaxPayloadSize;
	type MessageTimeout = MessageTimeout;
	type MaxPendingMessages = MaxPendingMessages;
	type ChainIdentifier = ChainIdentifier;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	t.into()
}
