use crate as pallet_evm;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;

pub type BlockNumber = u64;

frame_support::construct_runtime!(
    pub enum Test where
        Block = system::mocking::MockBlock<Test>,
        NodeBlock = system::mocking::MockBlock<Test>,
        UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>,
    {
        System: frame_system,
        EVMPallet: pallet_evm,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = frame_support::weights::constants::RocksDbWeight;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = sp_core::crypto::AccountId32;
    type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
    type Header = system::mocking::MockHeader<BlockNumber, sp_runtime::traits::BlakeTwo256>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = sp_version::RuntimeVersion;
    type PalletInfo = PalletInfo;
    type AccountData = (); 
    type OnNewAccount = (); 
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = frame_support::traits::ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_evm::pallet::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WasmBridge = ();
    type WeightInfo = crate::weights::SubstrateWeight;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    sp_io::TestExternalities::new(t)
}
