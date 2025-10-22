//! # Ëtrid Governance Pallet (E³20)
//!
//! ## Overview
//!
//! This pallet implements decentralized governance for the Ëtrid blockchain,
//! enabling token holders to propose, vote on, and execute protocol changes
//! through a stake-weighted voting system. It supports the E³20 governance
//! framework with automatic vote unreservation after proposal finalization.
//!
//! ## Features
//!
//! - Stake-weighted proposal voting system
//! - Automatic vote reservation and unreservation
//! - Time-bound voting periods
//! - Proposal creation with minimum stake requirement
//! - Vote tallying with majority rule
//! - Proposal cancellation by proposer
//! - Transparent execution tracking
//!
//! ## Extrinsics
//!
//! - `create_proposal` - Create a new governance proposal (requires minimum stake)
//! - `vote` - Vote on an active proposal with staked tokens
//! - `execute_proposal` - Finalize and execute a proposal after voting period
//! - `cancel_proposal` - Cancel an active proposal (proposer only)
//!
//! ## Usage Example
//!
//! ```ignore
//! // Create a proposal
//! Governance::create_proposal(
//!     Origin::signed(alice),
//!     b"Increase block reward".to_vec(),
//!     b"Proposal to increase validator rewards by 10%".to_vec(),
//! )?;
//!
//! // Vote in favor with 1000 tokens
//! Governance::vote(
//!     Origin::signed(bob),
//!     0, // proposal_id
//!     true, // support
//!     1000,
//! )?;
//!
//! // Wait for voting period to end...
//! // Execute the proposal
//! Governance::execute_proposal(
//!     Origin::signed(charlie),
//!     0, // proposal_id
//! )?;
//! ```
//!
//! ## Storage Items
//!
//! - `NextProposalId` - Counter for unique proposal IDs
//! - `LastConsensusDay` - Timestamp of last consensus day
//! - `Proposals` - Maps proposal ID to proposal details
//! - `Votes` - Maps (proposal_id, voter) to vote info (support, stake)
//!
//! ## Events
//!
//! - `ProposalCreated` - When a new proposal is created
//! - `Voted` - When a vote is cast on a proposal
//! - `ProposalPassed` - When a proposal passes (votes_for > votes_against)
//! - `ProposalRejected` - When a proposal is rejected
//! - `ProposalCancelled` - When a proposal is cancelled by proposer
//! - `VotesUnreserved` - When votes are unreserved after finalization
//!
//! ## Errors
//!
//! - `ProposalNotFound` - Proposal does not exist
//! - `VotingClosed` - Voting period has ended or not yet ended
//! - `AlreadyFinalized` - Proposal already finalized
//! - `NotProposer` - Caller is not the proposal creator
//! - `InsufficientStake` - Insufficient tokens for operation
//!
//! ## Vote Reservation
//!
//! All votes are automatically reserved when cast and unreserved when:
//! - Proposal is executed (passed or rejected)
//! - Proposal is cancelled by proposer
//!
//! This ensures economic commitment during voting while freeing funds after resolution.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Time},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use sp_runtime::traits::Zero;

    pub type ProposalId = u32;
    pub type MomentOf<T> = <<T as Config>::Time as Time>::Moment;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
        Cancelled,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<T: Config> {
        pub id: ProposalId,
        pub title: BoundedVec<u8, ConstU32<256>>,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub proposer: T::AccountId,
        pub created_at: MomentOf<T>,
        pub voting_ends: MomentOf<T>,
        pub votes_for: BalanceOf<T>,
        pub votes_against: BalanceOf<T>,
        pub status: ProposalStatus,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    pub struct VoteInfo<Balance> {
        pub vote: bool,
        pub stake: Balance,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type Time: Time;
        type ProposalDuration: Get<MomentOf<Self>>;
        type MinProposalStake: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::type_value]
    pub fn DefaultForNextProposalId() -> ProposalId { 0 }

    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T> = StorageValue<_, ProposalId, ValueQuery, DefaultForNextProposalId>;

    #[pallet::type_value]
    pub fn DefaultForLastConsensusDay<T: Config>() -> MomentOf<T> {
        MomentOf::<T>::default()
    }

    #[pallet::storage]
    #[pallet::getter(fn consensus_day)]
    pub type LastConsensusDay<T> = StorageValue<_, MomentOf<T>, ValueQuery, DefaultForLastConsensusDay<T>>;

    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, ProposalId, Proposal<T>>;

    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, ProposalId,
        Blake2_128Concat, T::AccountId,
        VoteInfo<BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalCreated(ProposalId, T::AccountId),
        Voted(ProposalId, T::AccountId, bool, BalanceOf<T>),
        ProposalPassed(ProposalId),
        ProposalRejected(ProposalId),
        ProposalCancelled(ProposalId),
        VotesUnreserved(ProposalId, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProposalNotFound,
        VotingClosed,
        AlreadyFinalized,
        NotProposer,
        InsufficientStake,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    impl<T: Config> Pallet<T> {
        /// Unreserve all votes for a finalized proposal
        fn unreserve_votes(proposal_id: ProposalId) -> u32 {
            let mut count = 0u32;

            // Iterate through all votes for this proposal
            let _ = Votes::<T>::drain_prefix(proposal_id)
                .for_each(|(voter, vote_info)| {
                    // Unreserve the staked amount
                    T::Currency::unreserve(&voter, vote_info.stake);
                    count += 1;
                });

            count
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            let now = T::Time::now();
            let end = now + T::ProposalDuration::get();

            T::Currency::reserve(&proposer, T::MinProposalStake::get())?;

            let id = NextProposalId::<T>::get();
            NextProposalId::<T>::put(id + 1);

            let bounded_title = BoundedVec::<u8, ConstU32<256>>::try_from(title)
                .map_err(|_| Error::<T>::ProposalNotFound)?;
            let bounded_desc = BoundedVec::<u8, ConstU32<1024>>::try_from(description)
                .map_err(|_| Error::<T>::ProposalNotFound)?;

            let proposal = Proposal::<T> {
                id,
                title: bounded_title,
                description: bounded_desc,
                proposer: proposer.clone(),
                created_at: now,
                voting_ends: end,
                votes_for: Zero::zero(),
                votes_against: Zero::zero(),
                status: ProposalStatus::Active,
            };

            Proposals::<T>::insert(id, proposal);
            Self::deposit_event(Event::ProposalCreated(id, proposer));
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            support: bool,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            let now = T::Time::now();
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                ensure!(now < p.voting_ends, Error::<T>::VotingClosed);

                T::Currency::reserve(&voter, amount)?;

                // Store vote info for later unreservation
                Votes::<T>::insert(
                    proposal_id,
                    voter.clone(),
                    VoteInfo {
                        vote: support,
                        stake: amount,
                    },
                );

                if support {
                    p.votes_for += amount;
                } else {
                    p.votes_against += amount;
                }

                Self::deposit_event(Event::Voted(proposal_id, voter, support, amount));
                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn execute_proposal(origin: OriginFor<T>, proposal_id: ProposalId) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            let now = T::Time::now();
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                ensure!(now >= p.voting_ends, Error::<T>::VotingClosed);

                if p.votes_for > p.votes_against {
                    p.status = ProposalStatus::Passed;
                    Self::deposit_event(Event::ProposalPassed(proposal_id));
                } else {
                    p.status = ProposalStatus::Rejected;
                    Self::deposit_event(Event::ProposalRejected(proposal_id));
                }

                // Unreserve all votes after proposal finalization
                let unreserved_count = Self::unreserve_votes(proposal_id);
                Self::deposit_event(Event::VotesUnreserved(proposal_id, unreserved_count));

                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn cancel_proposal(origin: OriginFor<T>, proposal_id: ProposalId) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Proposals::<T>::try_mutate(proposal_id, |maybe_p| {
                let p = maybe_p.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(p.proposer == sender, Error::<T>::NotProposer);
                ensure!(p.status == ProposalStatus::Active, Error::<T>::AlreadyFinalized);
                p.status = ProposalStatus::Cancelled;
                Self::deposit_event(Event::ProposalCancelled(proposal_id));

                // Unreserve all votes when proposal is cancelled
                let unreserved_count = Self::unreserve_votes(proposal_id);
                Self::deposit_event(Event::VotesUnreserved(proposal_id, unreserved_count));

                Ok(())
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_ok, assert_noop, parameter_types,
        traits::ConstU32,
    };
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Balances: pallet_balances,
            Governance: crate,
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
        type AccountData = pallet_balances::AccountData<u64>;
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
        pub const ExistentialDeposit: u64 = 1;
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ();
        type MaxReserves = ConstU32<50>;
        type ReserveIdentifier = [u8; 8];
        type Balance = u64;
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
        type DoneSlashHandler = ();
    }

    pub struct TestTime;
    impl frame_support::traits::Time for TestTime {
        type Moment = u64;
        fn now() -> Self::Moment {
            System::block_number()
        }
    }

    parameter_types! {
        pub const ProposalDuration: u64 = 100;
        pub const MinProposalStake: u64 = 100;
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type Time = TestTime;
        type ProposalDuration = ProposalDuration;
        type MinProposalStake = MinProposalStake;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (1, 10000),
                (2, 10000),
                (3, 10000),
                (4, 10000),
            ],
            dev_accounts: None,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn create_proposal_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let title = b"Test Proposal".to_vec();
            let description = b"Test Description".to_vec();

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                title,
                description
            ));

            let proposal = Governance::proposals(0).unwrap();
            assert_eq!(proposal.proposer, 1);
            assert_eq!(proposal.status, ProposalStatus::Active);
        });
    }

    #[test]
    fn vote_reserves_balance() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Create proposal
            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            let balance_before = Balances::free_balance(2);
            let reserved_before = Balances::reserved_balance(2);

            // Vote with 500
            assert_ok!(Governance::vote(
                RuntimeOrigin::signed(2),
                0,
                true,
                500
            ));

            let balance_after = Balances::free_balance(2);
            let reserved_after = Balances::reserved_balance(2);

            assert_eq!(balance_before - balance_after, 500);
            assert_eq!(reserved_after - reserved_before, 500);

            // Check vote is stored
            let vote_info = Governance::votes(0, 2).unwrap();
            assert_eq!(vote_info.vote, true);
            assert_eq!(vote_info.stake, 500);
        });
    }

    #[test]
    fn execute_proposal_unreserves_votes() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Create proposal
            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            // Multiple voters
            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 500));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, true, 300));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(4), 0, false, 200));

            // Check balances before finalization
            let voter2_reserved_before = Balances::reserved_balance(2);
            let voter3_reserved_before = Balances::reserved_balance(3);
            let voter4_reserved_before = Balances::reserved_balance(4);

            assert_eq!(voter2_reserved_before, 500);
            assert_eq!(voter3_reserved_before, 300);
            assert_eq!(voter4_reserved_before, 200);

            // Advance time past voting period
            System::set_block_number(102);

            // Execute proposal
            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 0));

            // Check all votes are unreserved
            assert_eq!(Balances::reserved_balance(2), 0);
            assert_eq!(Balances::reserved_balance(3), 0);
            assert_eq!(Balances::reserved_balance(4), 0);

            // Check votes storage is cleared
            assert!(Governance::votes(0, 2).is_none());
            assert!(Governance::votes(0, 3).is_none());
            assert!(Governance::votes(0, 4).is_none());
        });
    }

    #[test]
    fn proposal_passes_with_majority() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 600));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, false, 400));

            System::set_block_number(102);

            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 0));

            let proposal = Governance::proposals(0).unwrap();
            assert_eq!(proposal.status, ProposalStatus::Passed);

            // Verify unreservation
            assert_eq!(Balances::reserved_balance(2), 0);
            assert_eq!(Balances::reserved_balance(3), 0);
        });
    }

    #[test]
    fn proposal_rejected_with_minority() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 400));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, false, 600));

            System::set_block_number(102);

            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 0));

            let proposal = Governance::proposals(0).unwrap();
            assert_eq!(proposal.status, ProposalStatus::Rejected);

            // Verify unreservation
            assert_eq!(Balances::reserved_balance(2), 0);
            assert_eq!(Balances::reserved_balance(3), 0);
        });
    }

    #[test]
    fn cancel_proposal_unreserves_votes() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 500));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, true, 300));

            assert_eq!(Balances::reserved_balance(2), 500);
            assert_eq!(Balances::reserved_balance(3), 300);

            // Cancel proposal
            assert_ok!(Governance::cancel_proposal(RuntimeOrigin::signed(1), 0));

            let proposal = Governance::proposals(0).unwrap();
            assert_eq!(proposal.status, ProposalStatus::Cancelled);

            // Verify unreservation
            assert_eq!(Balances::reserved_balance(2), 0);
            assert_eq!(Balances::reserved_balance(3), 0);

            // Check votes storage is cleared
            assert!(Governance::votes(0, 2).is_none());
            assert!(Governance::votes(0, 3).is_none());
        });
    }

    #[test]
    fn cannot_vote_after_period_ends() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            System::set_block_number(102);

            assert_noop!(
                Governance::vote(RuntimeOrigin::signed(2), 0, true, 500),
                Error::<Test>::VotingClosed
            );
        });
    }

    #[test]
    fn cannot_execute_before_period_ends() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 500));

            System::set_block_number(50);

            assert_noop!(
                Governance::execute_proposal(RuntimeOrigin::signed(1), 0),
                Error::<Test>::VotingClosed
            );
        });
    }

    #[test]
    fn only_proposer_can_cancel() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_noop!(
                Governance::cancel_proposal(RuntimeOrigin::signed(2), 0),
                Error::<Test>::NotProposer
            );
        });
    }

    #[test]
    fn multiple_votes_tracked_correctly() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Proposal 1".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Proposal 2".to_vec(),
                b"Test".to_vec()
            ));

            // Vote on both proposals
            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 500));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 1, true, 300));

            // Total reserved should be 800
            assert_eq!(Balances::reserved_balance(2), 800);

            // Execute first proposal
            System::set_block_number(102);
            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 0));

            // Should have unreserved 500 from proposal 0
            assert_eq!(Balances::reserved_balance(2), 300);

            // Execute second proposal
            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 1));

            // Should have unreserved all
            assert_eq!(Balances::reserved_balance(2), 0);
        });
    }

    #[test]
    fn events_emitted_correctly() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec()
            ));

            assert_ok!(Governance::vote(RuntimeOrigin::signed(2), 0, true, 500));
            assert_ok!(Governance::vote(RuntimeOrigin::signed(3), 0, true, 300));

            System::set_block_number(102);
            assert_ok!(Governance::execute_proposal(RuntimeOrigin::signed(1), 0));

            // Check that VotesUnreserved event was emitted with count of 2
            let events = System::events();
            let unreserved_event = events.iter().find(|e| {
                matches!(
                    e.event,
                    RuntimeEvent::Governance(Event::VotesUnreserved(0, 2))
                )
            });
            assert!(unreserved_event.is_some());
        });
    }
}
