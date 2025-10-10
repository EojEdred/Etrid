//! Ëtrid Runtime - E³20 Core Runtime Configuration
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{construct_runtime, parameter_types, traits::Everything};
use frame_system as system;

use pallet_accounts;
use pallet_governance;
use pallet_consensus;
use pallet_multichain;
Use pallet_staking;
use pallet_vm;

pub use sp_runtime::{traits::IdentityLookup, testing::Header, Perbill};

/// Basic type aliases
pub type BlockNumber = u32;
pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = sp_runtime::AccountId32;
pub type Balance = u128;
pub type Index = u32;

/// Runtime Config
#[frame_support::pallet]
pub mod opaque {}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const MaxCodeSize: u32 = 512 * 1024; // 512 KB max contract size
    pub const MinProposalStake: Balance = 10 * 1_000_000_000_000; // 10 ÉTR (in plancks)
    pub const ProposalDuration: u64 = 3 * 24 * 60 * 60 * 1000; // 3 days in milliseconds
    pub const MinStake: Balance = 100 * 1_000_000_000_000; // 100 ÉTR for validator
    pub const ValidatorReward: Balance = 2 * 1_000_000_000_000; // 2 ÉTR
}

impl system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type RuntimeEvent = Event;
    type BlockHashCount = BlockHashCount;
    type Version = (); 
    type PalletInfo = PalletInfo;
    type AccountData = (); // handled in pallet_accounts
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = (); 
    type SS58Prefix = (); 
    type OnSetCode = (); 
}

impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = Event;
    type Balance = Balance;
    type GovernanceOrigin = frame_system::EnsureRoot<AccountId>;
}

impl pallet_governance::Config for Runtime {
    type RuntimeEvent = Event;
    type Currency = pallet_accounts::Pallet<Runtime>; // use internal balances
    type Time = pallet_timestamp::Pallet<Runtime>;
    type ProposalDuration = ProposalDuration;
    type MinProposalStake = MinProposalStake;
}

impl pallet_consensus::Config for Runtime {
    type RuntimeEvent = Event;
    type Currency = pallet_accounts::Pallet<Runtime>;
    type RandomnessSource = pallet_randomness_collective_flip::Pallet<Runtime>;
    type Time = pallet_timestamp::Pallet<Runtime>;
    type MinStake = MinStake;
    type ValidatorReward = ValidatorReward;
}

impl pallet_multichain::Config for Runtime {
    type RuntimeEvent = Event;
    type Currency = pallet_accounts::Pallet<Runtime>;
}

impl pallet_vm::Config for Runtime {
    type RuntimeEvent = Event;
    type Currency = pallet_accounts::Pallet<Runtime>;
    type MaxCodeSize = MaxCodeSize;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Accounts: pallet_accounts::{Pallet, Call, Storage, Event<T>},
        Governance: pallet_governance::{Pallet, Call, Storage, Event<T>},
        Consensus: pallet_consensus::{Pallet, Call, Storage, Event<T>},
        Multichain: pallet_multichain::{Pallet, Call, Storage, Event<T>},
        Staking: pallet_staking::{Pallet, Call, Storage, Event<T>, Config<T>},
        Vm: pallet_vm::{Pallet, Call, Storage, Event<T>},
    }
impl pallet_staking::Config for Runtime {
    type Currency = pallet_accounts::Pallet<Runtime>; // if you use your own accounts logic
    type UnixTime = pallet_timestamp::Pallet<Runtime>;
    type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
    type ElectionProvider = ();
    type GenesisElectionProvider = ();
    type Event = Event;
    type Slash = ();
    type Reward = ();
    type RewardRemainder = ();
    type SlashDeferDuration = ConstU32<7>;
    type SlashCancelOrigin = frame_system::EnsureRoot<AccountId>;
    type SessionsPerEra = ConstU32<6>;
    type BondingDuration = ConstU32<28>;
    type MaxNominations = ConstU32<16>;
    type MaxUnlockingChunks = ConstU32<32>;
    type WeightInfo = ();
    type NextNewSession = ();
    type OffendingValidatorsThreshold = Perbill::from_percent(17);
    type EraPayout = pallet_staking::ConvertCurve<()>;
}
);
