use crate as pallet_treasury;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64, ConstU16, Hooks},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Treasury: pallet_treasury,
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
	type AccountData = pallet_balances::AccountData<Balance>;
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
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

parameter_types! {
	pub const DirectorCount: u8 = 9;
	pub const ApprovalThreshold: u8 = 6;
	pub const EmergencyThreshold: u8 = 7;
	pub const ProposalExpiration: u64 = 50400; // 7 days in blocks (at 1s/block)
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
}

impl pallet_treasury::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type DirectorCount = DirectorCount;
	type ApprovalThreshold = ApprovalThreshold;
	type EmergencyThreshold = EmergencyThreshold;
	type ProposalExpiration = ProposalExpiration;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	// Initialize with 9 directors
	pallet_treasury::GenesisConfig::<Test> {
		directors: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
		budget_allocations: pallet_treasury::BudgetAllocations::default_allocations(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Fund some test accounts
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 1_000_000_000_000), // 1 trillion (director)
			(2, 1_000_000_000_000),
			(3, 1_000_000_000_000),
			(4, 1_000_000_000_000),
			(5, 1_000_000_000_000),
			(6, 1_000_000_000_000),
			(7, 1_000_000_000_000),
			(8, 1_000_000_000_000),
			(9, 1_000_000_000_000),
			(100, 10_000_000_000_000), // 10 trillion (test recipient)
			(Treasury::account_id(), 500_000_000_000_000), // 500 trillion (treasury)
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		// Initialize treasury balance in storage
		pallet_treasury::TreasuryBalance::<Test>::put(500_000_000_000_000u128);
	});
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
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}

// Helper to approve a disbursement by multiple directors
pub fn approve_by_directors(disbursement_id: u64, directors: Vec<u64>) {
	for director in directors {
		assert!(Treasury::approve_disbursement(
			RuntimeOrigin::signed(director),
			disbursement_id
		)
		.is_ok());
	}
}

// Constants for tests
pub const MILLION: Balance = 1_000_000_000_000_000_000; // 1M ETR (18 decimals)
pub const BILLION: Balance = 1_000_000_000_000_000_000_000; // 1B ETR
pub const ETR: Balance = 1_000_000_000_000_000_000; // 1 ETR
