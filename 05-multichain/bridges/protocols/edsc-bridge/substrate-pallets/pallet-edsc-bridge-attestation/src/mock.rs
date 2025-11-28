//! Mock runtime for pallet-edsc-bridge-attestation tests

#![cfg(test)]

use frame_support::{
	derive_impl, parameter_types,
};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Attestation: crate,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

parameter_types! {
	pub const MaxAttesters: u32 = 100;
	pub const MaxAttestersPerMessage: u32 = 10;
	pub const MinSignatureThreshold: u32 = 3;  // 3-of-5 by default
	pub const AttestationMaxAge: u64 = 1000;  // 1000 blocks
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxAttesters = MaxAttesters;
	type MaxAttestersPerMessage = MaxAttestersPerMessage;
	type MinSignatureThreshold = MinSignatureThreshold;
	type AttestationMaxAge = AttestationMaxAge;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	t.into()
}
