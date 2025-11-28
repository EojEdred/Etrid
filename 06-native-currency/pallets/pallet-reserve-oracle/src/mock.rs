use crate as pallet_reserve_oracle;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64, Hooks},
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
		ReserveOracle: pallet_reserve_oracle,
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

parameter_types! {
	pub const SnapshotInterval: u64 = 100; // Every 100 blocks
	pub const MaxSnapshots: u32 = 1000;
	pub const ReserveOptimalMin: u16 = 11000; // 110%
	pub const ReserveOptimalMax: u16 = 13000; // 130%
	pub const ReserveThrottleThreshold: u16 = 10500; // 105%
	pub const ReserveCriticalThreshold: u16 = 10000; // 100%
	pub const MaxPriceStaleness: u64 = 200; // 200 blocks
	pub const MaxPriceAge: u64 = 300; // 300 blocks
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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// Helper function to create account
pub fn account(id: u64) -> u64 {
	id
}

// Helper function to advance blocks
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
			<ReserveOracle as Hooks<u64>>::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}
