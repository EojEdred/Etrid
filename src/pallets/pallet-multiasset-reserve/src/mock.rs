//! Mock runtime for testing pallet-multiasset-reserve

use crate as pallet_multiasset_reserve;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64},
	PalletId,
};
use sp_arithmetic::Permill;
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
		MultiassetReserve: pallet_multiasset_reserve,
		ReserveOracle: pallet_reserve_oracle,
		ReserveVault: pallet_reserve_vault,
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
}

// Reserve Oracle mock config
parameter_types! {
	pub const SnapshotInterval: u64 = 10;
	pub const MaxSnapshots: u32 = 100;
	pub const ReserveOptimalMin: u16 = 11000; // 110%
	pub const ReserveOptimalMax: u16 = 13000; // 130%
	pub const ReserveThrottleThreshold: u16 = 10500; // 105%
	pub const ReserveCriticalThreshold: u16 = 10000; // 100%
	pub const MaxPriceStaleness: u64 = 100;
	pub const MaxPriceAge: u64 = 200;
}

impl pallet_reserve_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type SnapshotInterval = SnapshotInterval;
	type MaxSnapshots = MaxSnapshots;
	type ReserveOptimalMin = ReserveOptimalMin;
	type ReserveOptimalMax = ReserveOptimalMax;
	type ReserveThrottleThreshold = ReserveThrottleThreshold;
	type ReserveCriticalThreshold = ReserveCriticalThreshold;
	type MaxPriceStaleness = MaxPriceStaleness;
	type MaxPriceAge = MaxPriceAge;
}

// Reserve Vault mock config
parameter_types! {
	pub const MinDeposit: u128 = 100;
}

impl pallet_reserve_vault::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MinDeposit = MinDeposit;
}

// Multiasset Reserve config
parameter_types! {
	pub const MaxAssets: u32 = 50;
	pub const RebalanceInterval: u64 = 100; // blocks
	pub const RebalanceThreshold: Permill = Permill::from_percent(5); // 5%
	pub const MultiassetReservePalletId: PalletId = PalletId(*b"py/marve");
}

impl pallet_multiasset_reserve::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxAssets = MaxAssets;
	type RebalanceInterval = RebalanceInterval;
	type RebalanceThreshold = RebalanceThreshold;
	type PalletId = MultiassetReservePalletId;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	t.into()
}
