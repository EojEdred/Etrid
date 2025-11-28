//! Simplified mock runtime for pallet-edsc-oracle tests
//! This version uses trait mocking to avoid complex dependencies

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
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a minimal mock runtime
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
}

// Mock redemption pallet with minimal implementation
#[frame_support::pallet]
pub mod mock_redemption {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type OraclePrice<T: Config> = StorageValue<_, u128, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OraclePriceUpdated { price: u128 },
	}

	impl<T: Config> Pallet<T> {
		pub fn do_update_oracle_price(price: u128) -> DispatchResult {
			OraclePrice::<T>::put(price);
			Self::deposit_event(Event::OraclePriceUpdated { price });
			Ok(())
		}
	}
}

impl mock_redemption::Config for Test {
	type RuntimeEvent = RuntimeEvent;
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

// Temporarily satisfy Config requirements - we'll fix the actual trait bound
impl pallet_edsc_redemption::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PriceOracleSource = EdscOracle;
	type MinRedemptionFee = sp_runtime::traits::ConstU32<2500>; // Placeholder
	type SafetyMultiplier = sp_runtime::traits::ConstU128<120>; // Placeholder
	type Path1DailyLimit = sp_runtime::traits::ConstU128<10000>; // Placeholder
	type Path2DailyLimit = sp_runtime::traits::ConstU128<50000>; // Placeholder
	type Path3DailyLimit = sp_runtime::traits::ConstU128<5000>; // Placeholder
	type HourlyRedemptionCap = sp_runtime::traits::ConstU32<500>; // Placeholder
	type DailyRedemptionCap = sp_runtime::traits::ConstU32<2000>; // Placeholder
	type ThrottleReserveRatio = sp_runtime::traits::ConstU128<105>; // Placeholder
	type EmergencyReserveRatio = sp_runtime::traits::ConstU128<100>; // Placeholder
	type SafeReserveRatio = sp_runtime::traits::ConstU128<120>; // Placeholder
	type CircuitBreakerThreshold = sp_runtime::traits::ConstU128<1000000>; // Placeholder
	type MaxQueueSize = ConstU32<1000>; // Placeholder
}

impl pallet_edsc_oracle::Config for Test {
	type RuntimeEvent = RuntimeEvent;
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
