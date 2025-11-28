//! Mock runtime for pallet-edsc-oracle tests

use crate as pallet_edsc_oracle;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64},
};
use sp_arithmetic::Permill;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, DispatchResult,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		EdscOracle: pallet_edsc_oracle,
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
	type ExtensionsWeightInfo = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

// No-op price callback for testing (does not require redemption pallet)
pub struct NoOpPriceCallback;
impl pallet_edsc_oracle::PriceUpdateCallback for NoOpPriceCallback {
	fn on_price_updated(_price: u128) -> DispatchResult {
		// In tests, we don't need to do anything with price updates
		Ok(())
	}
}

// pallet-edsc-oracle config
parameter_types! {
	// 24h window = 14400 blocks @ 6s per block
	pub const PrimaryTwapWindow: u64 = 14_400;
	// 7d window = 100800 blocks @ 6s per block
	pub const FallbackTwapWindow: u64 = 100_800;
	// Minimum 5 sources
	pub const MinPriceSources: u32 = 5;
	// 2% outlier threshold
	pub const OutlierThreshold: Permill = Permill::from_percent(2);
	// 10 min staleness = 100 blocks @ 6s
	pub const StalenessTimeout: u64 = 100;
	// Max 1000 price points in history
	pub const MaxPriceHistory: u32 = 1000;
}

impl pallet_edsc_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PriceCallback = NoOpPriceCallback;
	type PrimaryTwapWindow = PrimaryTwapWindow;
	type FallbackTwapWindow = FallbackTwapWindow;
	type MinPriceSources = MinPriceSources;
	type OutlierThreshold = OutlierThreshold;
	type StalenessTimeout = StalenessTimeout;
	type MaxPriceHistory = MaxPriceHistory;
}

// Helper function to build genesis config
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}
