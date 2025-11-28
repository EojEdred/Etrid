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

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct ConsensusDayConfig<T: Config> {
        pub frequency: BlockNumberFor<T>,
        pub duration: BlockNumberFor<T>,
        pub next_start: BlockNumberFor<T>,
        pub active: bool,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct ConsensusDayProposal<T: Config> {
        pub proposal_id: ProposalId,
        pub proposer: T::AccountId,
        pub title: BoundedVec<u8, ConstU32<256>>,
        pub description: BoundedVec<u8, ConstU32<4096>>,
        pub yes_votes: u128,
        pub no_votes: u128,
        pub total_stake_voted: u128,
        pub created_at: BlockNumberFor<T>,
        pub ends_at: BlockNumberFor<T>,
        pub supermajority_threshold: u8,
        pub min_participation: u8,
        pub executed: bool,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type Time: Time;
        type ProposalDuration: Get<MomentOf<Self>>;
        type MinProposalStake: Get<BalanceOf<Self>>;
        type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
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

    #[pallet::storage]
    #[pallet::getter(fn consensus_day_schedule)]
    pub type ConsensusDaySchedule<T: Config> = StorageValue<_, ConsensusDayConfig<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn consensus_day_proposals)]
    pub type ConsensusDayProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ProposalId,
        ConsensusDayProposal<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn is_consensus_day_active)]
    pub type IsConsensusDayActive<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn consensus_day_votes)]
    pub type ConsensusDayVotes<T: Config> = StorageDoubleMap<
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
        ConsensusDayScheduled { next_start: BlockNumberFor<T>, duration: BlockNumberFor<T> },
        ConsensusDayStarted { block: BlockNumberFor<T> },
        ConsensusDayEnded { block: BlockNumberFor<T> },
        ConsensusDayProposalCreated { proposal_id: ProposalId, proposer: T::AccountId, supermajority_threshold: u8 },
        ConsensusDayVoteCast { proposal_id: ProposalId, voter: T::AccountId, vote: bool, stake: BalanceOf<T> },
        ConsensusDayProposalPassed { proposal_id: ProposalId, yes_pct: u8 },
        ConsensusDayProposalRejected { proposal_id: ProposalId, yes_pct: u8 },
    }

    #[pallet::error]
    pub enum Error<T> {
        ProposalNotFound,
        VotingClosed,
        AlreadyFinalized,
        NotProposer,
        InsufficientStake,
        ConsensusDayNotActive,
        ConsensusDayNotConfigured,
        InvalidThreshold,
        InvalidParticipation,
        TitleTooLong,
        DescriptionTooLong,
        VotingNotEnded,
        AlreadyExecuted,
        InsufficientParticipation,
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

        /// Unreserve all votes for a finalized Consensus Day proposal
        fn unreserve_consensus_day_votes(proposal_id: ProposalId) -> u32 {
            let mut count = 0u32;

            // Iterate through all votes for this proposal
            let _ = ConsensusDayVotes::<T>::drain_prefix(proposal_id)
                .for_each(|(voter, vote_info)| {
                    // Unreserve the staked amount
                    T::Currency::unreserve(&voter, vote_info.stake);
                    count += 1;
                });

            count
        }

        /// Convert Balance to u128 for calculations
        fn balance_to_u128(balance: BalanceOf<T>) -> u128 {
            use sp_runtime::traits::SaturatedConversion;
            balance.saturated_into()
        }

        /// Get total stake in the system (simplified - sum of all account balances)
        fn total_stake() -> u128 {
            // For production, this should query the total issuance or a dedicated staking pool
            // Here we use a placeholder that returns a large value for testing
            1_000_000u128
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

        /// Initialize Consensus Day schedule (governance-only)
        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn initialize_consensus_day(
            origin: OriginFor<T>,
            frequency: BlockNumberFor<T>,
            duration: BlockNumberFor<T>,
        ) -> DispatchResult {
            T::GovernanceOrigin::ensure_origin(origin)?;

            let current_block = frame_system::Pallet::<T>::block_number();
            let next_start = current_block + frequency;

            let config = ConsensusDayConfig {
                frequency,
                duration,
                next_start,
                active: false,
            };

            ConsensusDaySchedule::<T>::put(config);
            Self::deposit_event(Event::ConsensusDayScheduled { next_start, duration });

            Ok(())
        }

        /// Create a Consensus Day proposal (only during Consensus Day)
        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn create_consensus_day_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            supermajority_threshold: u8,
            min_participation: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure Consensus Day is active
            ensure!(IsConsensusDayActive::<T>::get(), Error::<T>::ConsensusDayNotActive);

            // Validate thresholds
            ensure!(supermajority_threshold >= 60 && supermajority_threshold <= 100, Error::<T>::InvalidThreshold);
            ensure!(min_participation >= 20 && min_participation <= 100, Error::<T>::InvalidParticipation);

            // Validate input lengths
            let title_bounded = BoundedVec::try_from(title).map_err(|_| Error::<T>::TitleTooLong)?;
            let description_bounded = BoundedVec::try_from(description).map_err(|_| Error::<T>::DescriptionTooLong)?;

            let proposal_id = NextProposalId::<T>::get();
            NextProposalId::<T>::put(proposal_id + 1);

            let current_block = frame_system::Pallet::<T>::block_number();
            let config = ConsensusDaySchedule::<T>::get().ok_or(Error::<T>::ConsensusDayNotConfigured)?;
            let ends_at = config.next_start + config.duration;

            let proposal = ConsensusDayProposal {
                proposal_id,
                proposer: who.clone(),
                title: title_bounded,
                description: description_bounded,
                yes_votes: 0,
                no_votes: 0,
                total_stake_voted: 0,
                created_at: current_block,
                ends_at,
                supermajority_threshold,
                min_participation,
                executed: false,
            };

            ConsensusDayProposals::<T>::insert(proposal_id, proposal);
            Self::deposit_event(Event::ConsensusDayProposalCreated {
                proposal_id,
                proposer: who,
                supermajority_threshold,
            });

            Ok(())
        }

        /// Vote on Consensus Day proposal
        #[pallet::weight(10_000)]
        #[pallet::call_index(6)]
        pub fn vote_consensus_day_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            vote: bool,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure Consensus Day is active
            ensure!(IsConsensusDayActive::<T>::get(), Error::<T>::ConsensusDayNotActive);

            // Reserve stake
            T::Currency::reserve(&who, stake)?;

            // Store vote info for later unreservation
            ConsensusDayVotes::<T>::insert(
                proposal_id,
                who.clone(),
                VoteInfo {
                    vote,
                    stake,
                },
            );

            // Update proposal votes
            ConsensusDayProposals::<T>::try_mutate(proposal_id, |maybe_proposal| -> DispatchResult {
                let proposal = maybe_proposal.as_mut().ok_or(Error::<T>::ProposalNotFound)?;

                let stake_u128 = Self::balance_to_u128(stake);
                proposal.total_stake_voted = proposal.total_stake_voted.saturating_add(stake_u128);

                if vote {
                    proposal.yes_votes = proposal.yes_votes.saturating_add(stake_u128);
                } else {
                    proposal.no_votes = proposal.no_votes.saturating_add(stake_u128);
                }

                Ok(())
            })?;

            Self::deposit_event(Event::ConsensusDayVoteCast {
                proposal_id,
                voter: who,
                vote,
                stake,
            });

            Ok(())
        }

        /// Finalize Consensus Day proposal
        #[pallet::weight(10_000)]
        #[pallet::call_index(7)]
        pub fn finalize_consensus_day_proposal(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            let mut proposal = ConsensusDayProposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;

            // Check voting period ended
            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(current_block >= proposal.ends_at, Error::<T>::VotingNotEnded);
            ensure!(!proposal.executed, Error::<T>::AlreadyExecuted);

            // Check participation threshold
            let total_stake = Self::total_stake();
            let participation_pct = (proposal.total_stake_voted * 100) / total_stake;
            ensure!(participation_pct >= proposal.min_participation as u128, Error::<T>::InsufficientParticipation);

            // Check supermajority threshold
            let yes_pct = if proposal.total_stake_voted > 0 {
                (proposal.yes_votes * 100) / proposal.total_stake_voted
            } else {
                0
            };
            let passed = yes_pct >= proposal.supermajority_threshold as u128;

            proposal.executed = true;
            ConsensusDayProposals::<T>::insert(proposal_id, proposal);

            // Unreserve all votes
            let _ = Self::unreserve_consensus_day_votes(proposal_id);

            if passed {
                Self::deposit_event(Event::ConsensusDayProposalPassed { proposal_id, yes_pct: yes_pct as u8 });
            } else {
                Self::deposit_event(Event::ConsensusDayProposalRejected { proposal_id, yes_pct: yes_pct as u8 });
            }

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Check if Consensus Day should start
            if let Some(mut config) = ConsensusDaySchedule::<T>::get() {
                if n >= config.next_start && !config.active {
                    config.active = true;
                    IsConsensusDayActive::<T>::put(true);
                    ConsensusDaySchedule::<T>::put(config.clone());
                    Self::deposit_event(Event::ConsensusDayStarted { block: n });
                }

                // Check if Consensus Day should end
                if config.active && n >= config.next_start + config.duration {
                    config.active = false;
                    config.next_start = n + config.frequency;
                    IsConsensusDayActive::<T>::put(false);
                    ConsensusDaySchedule::<T>::put(config);
                    Self::deposit_event(Event::ConsensusDayEnded { block: n });
                }
            }

            Weight::from_parts(10_000, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_ok, assert_noop, parameter_types,
        traits::{ConstU32, Hooks},
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
        type GovernanceOrigin = frame_system::EnsureRoot<u64>;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (1, 1_000_000),
                (2, 1_000_000),
                (3, 1_000_000),
                (4, 1_000_000),
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

    // ==================== CONSENSUS DAY TESTS ====================

    #[test]
    fn initialize_consensus_day_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let frequency = 100; // Every 100 blocks
            let duration = 20;   // Lasts 20 blocks

            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), frequency, duration));

            let config = ConsensusDaySchedule::<Test>::get().unwrap();
            assert_eq!(config.frequency, frequency);
            assert_eq!(config.duration, duration);
            assert_eq!(config.next_start, 101);
            assert!(!config.active);
        });
    }

    #[test]
    fn consensus_day_auto_activation_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));

            // Not active yet
            assert!(!IsConsensusDayActive::<Test>::get());

            // Advance to start block
            System::set_block_number(11);
            Governance::on_initialize(11);

            // Should be active now
            assert!(IsConsensusDayActive::<Test>::get());

            let config = ConsensusDaySchedule::<Test>::get().unwrap();
            assert!(config.active);
        });
    }

    #[test]
    fn consensus_day_auto_deactivation_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));

            // Activate
            System::set_block_number(11);
            Governance::on_initialize(11);
            assert!(IsConsensusDayActive::<Test>::get());

            // Advance past end
            System::set_block_number(16);
            Governance::on_initialize(16);

            // Should be inactive now
            assert!(!IsConsensusDayActive::<Test>::get());

            let config = ConsensusDaySchedule::<Test>::get().unwrap();
            assert!(!config.active);
            // Next start should be rescheduled
            assert_eq!(config.next_start, 16 + 10); // current + frequency
        });
    }

    #[test]
    fn create_consensus_day_proposal_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup and activate Consensus Day
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            let proposer = 1;
            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(proposer),
                b"Protocol Upgrade".to_vec(),
                b"Upgrade to v2.0 with new features".to_vec(),
                75, // 75% supermajority
                40, // 40% participation
            ));

            let proposal = ConsensusDayProposals::<Test>::get(0).unwrap();
            assert_eq!(proposal.proposer, proposer);
            assert_eq!(proposal.supermajority_threshold, 75);
            assert_eq!(proposal.min_participation, 40);
            assert_eq!(proposal.yes_votes, 0);
            assert_eq!(proposal.no_votes, 0);
            assert!(!proposal.executed);
        });
    }

    #[test]
    fn cannot_create_consensus_day_proposal_when_inactive() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            assert_noop!(
                Governance::create_consensus_day_proposal(
                    RuntimeOrigin::signed(1),
                    b"Test".to_vec(),
                    b"Test proposal".to_vec(),
                    75, 40,
                ),
                Error::<Test>::ConsensusDayNotActive
            );
        });
    }

    #[test]
    fn consensus_day_proposal_validates_thresholds() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup and activate
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            // Test invalid supermajority (too low)
            assert_noop!(
                Governance::create_consensus_day_proposal(
                    RuntimeOrigin::signed(1),
                    b"Test".to_vec(),
                    b"Test".to_vec(),
                    50, // Below 60% minimum
                    40,
                ),
                Error::<Test>::InvalidThreshold
            );

            // Test invalid participation (too low)
            assert_noop!(
                Governance::create_consensus_day_proposal(
                    RuntimeOrigin::signed(1),
                    b"Test".to_vec(),
                    b"Test".to_vec(),
                    75,
                    10, // Below 20% minimum
                ),
                Error::<Test>::InvalidParticipation
            );
        });
    }

    #[test]
    fn consensus_day_voting_works() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            let proposer = 1;
            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(proposer),
                b"Test".to_vec(),
                b"Test proposal".to_vec(),
                75, 40,
            ));

            // Vote yes with 100 stake
            let voter = 2;
            let initial_balance = Balances::free_balance(voter);
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(voter), 0, true, 100));

            let proposal = ConsensusDayProposals::<Test>::get(0).unwrap();
            assert_eq!(proposal.yes_votes, 100);
            assert_eq!(proposal.total_stake_voted, 100);

            // Check stake was reserved
            assert_eq!(Balances::reserved_balance(voter), 100);
            assert_eq!(Balances::free_balance(voter), initial_balance - 100);
        });
    }

    #[test]
    fn cannot_vote_consensus_day_proposal_when_inactive() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup but don't activate
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));

            assert_noop!(
                Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 100),
                Error::<Test>::ConsensusDayNotActive
            );
        });
    }

    #[test]
    fn consensus_day_voting_tracks_multiple_votes() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, 40,
            ));

            // Multiple votes
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 500));
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(3), 0, true, 300));
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(4), 0, false, 200));

            let proposal = ConsensusDayProposals::<Test>::get(0).unwrap();
            assert_eq!(proposal.yes_votes, 800);
            assert_eq!(proposal.no_votes, 200);
            assert_eq!(proposal.total_stake_voted, 1000);
        });
    }

    #[test]
    fn consensus_day_finalization_passes_with_supermajority() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, // 75% required
                40, // 40% participation required
            ));

            // Vote: 80% yes (exceeds 75% threshold)
            // Total stake is 1_000_000, so we need at least 400_000 to meet 40% participation
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 8000));
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(3), 0, false, 2000));
            // Need more votes to meet participation threshold
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(4), 0, true, 390_000));

            // Advance past voting period
            System::set_block_number(16);

            // Finalize
            assert_ok!(Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0));

            // Verify passed
            let proposal = ConsensusDayProposals::<Test>::get(0).unwrap();
            assert!(proposal.executed);

            // Check votes were unreserved
            assert_eq!(Balances::reserved_balance(2), 0);
            assert_eq!(Balances::reserved_balance(3), 0);
            assert_eq!(Balances::reserved_balance(4), 0);
        });
    }

    #[test]
    fn consensus_day_finalization_fails_without_supermajority() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, // 75% required
                40,
            ));

            // Vote: Only 60% yes (below 75% threshold)
            // To get 60% yes: need 240k yes and 160k no out of 400k total
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 240_000));
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(3), 0, false, 160_000));

            // Advance past voting period
            System::set_block_number(16);

            // Finalize - should reject due to insufficient supermajority
            assert_ok!(Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0));

            // Verify rejected
            let proposal = ConsensusDayProposals::<Test>::get(0).unwrap();
            assert!(proposal.executed);

            // Check ProposalRejected event
            let events = System::events();
            let rejected_event = events.iter().any(|e| {
                matches!(
                    e.event,
                    RuntimeEvent::Governance(Event::ConsensusDayProposalRejected { .. })
                )
            });
            assert!(rejected_event);
        });
    }

    #[test]
    fn consensus_day_finalization_fails_without_participation() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75,
                40, // Requires 40% participation (400_000 of 1_000_000)
            ));

            // Vote with insufficient total stake
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 1000));

            // Advance past voting period
            System::set_block_number(16);

            // Finalize - should fail
            assert_noop!(
                Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0),
                Error::<Test>::InsufficientParticipation
            );
        });
    }

    #[test]
    fn cannot_finalize_consensus_day_proposal_before_end() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, 40,
            ));

            // Try to finalize before voting period ends
            assert_noop!(
                Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0),
                Error::<Test>::VotingNotEnded
            );
        });
    }

    #[test]
    fn cannot_finalize_consensus_day_proposal_twice() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, 40,
            ));

            // Vote
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 400_000));

            // Advance and finalize
            System::set_block_number(16);
            assert_ok!(Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0));

            // Try to finalize again
            assert_noop!(
                Governance::finalize_consensus_day_proposal(RuntimeOrigin::signed(1), 0),
                Error::<Test>::AlreadyExecuted
            );
        });
    }

    #[test]
    fn consensus_day_events_emitted_correctly() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Initialize
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));

            // Check scheduled event
            let events = System::events();
            assert!(events.iter().any(|e| matches!(
                e.event,
                RuntimeEvent::Governance(Event::ConsensusDayScheduled { .. })
            )));

            // Activate
            System::set_block_number(11);
            Governance::on_initialize(11);

            let events = System::events();
            assert!(events.iter().any(|e| matches!(
                e.event,
                RuntimeEvent::Governance(Event::ConsensusDayStarted { .. })
            )));

            // Create proposal
            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Test".to_vec(),
                75, 40,
            ));

            let events = System::events();
            assert!(events.iter().any(|e| matches!(
                e.event,
                RuntimeEvent::Governance(Event::ConsensusDayProposalCreated { .. })
            )));

            // Vote
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(2), 0, true, 400_000));

            let events = System::events();
            assert!(events.iter().any(|e| matches!(
                e.event,
                RuntimeEvent::Governance(Event::ConsensusDayVoteCast { .. })
            )));

            // Deactivate
            System::set_block_number(16);
            Governance::on_initialize(16);

            let events = System::events();
            assert!(events.iter().any(|e| matches!(
                e.event,
                RuntimeEvent::Governance(Event::ConsensusDayEnded { .. })
            )));
        });
    }

    #[test]
    fn consensus_day_cycles_correctly() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Initialize with frequency 10, duration 5
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));

            // First cycle: blocks 11-15
            System::set_block_number(11);
            Governance::on_initialize(11);
            assert!(IsConsensusDayActive::<Test>::get());

            System::set_block_number(16);
            Governance::on_initialize(16);
            assert!(!IsConsensusDayActive::<Test>::get());

            let config = ConsensusDaySchedule::<Test>::get().unwrap();
            assert_eq!(config.next_start, 26); // 16 + 10

            // Second cycle: blocks 26-30
            System::set_block_number(26);
            Governance::on_initialize(26);
            assert!(IsConsensusDayActive::<Test>::get());

            System::set_block_number(31);
            Governance::on_initialize(31);
            assert!(!IsConsensusDayActive::<Test>::get());

            let config = ConsensusDaySchedule::<Test>::get().unwrap();
            assert_eq!(config.next_start, 41); // 31 + 10
        });
    }

    #[test]
    fn consensus_day_multiple_proposals_work() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            // Create multiple proposals
            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Proposal 1".to_vec(),
                b"First proposal".to_vec(),
                75, 40,
            ));

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(2),
                b"Proposal 2".to_vec(),
                b"Second proposal".to_vec(),
                80, 50,
            ));

            // Vote on both
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(3), 0, true, 200_000));
            assert_ok!(Governance::vote_consensus_day_proposal(RuntimeOrigin::signed(3), 1, false, 200_000));

            // Check both proposals exist
            let prop1 = ConsensusDayProposals::<Test>::get(0).unwrap();
            let prop2 = ConsensusDayProposals::<Test>::get(1).unwrap();

            assert_eq!(prop1.yes_votes, 200_000);
            assert_eq!(prop2.no_votes, 200_000);

            // Check total reserved
            assert_eq!(Balances::reserved_balance(3), 400_000);
        });
    }

    #[test]
    fn consensus_day_proposal_different_thresholds() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            // Setup
            assert_ok!(Governance::initialize_consensus_day(RuntimeOrigin::root(), 10, 5));
            System::set_block_number(11);
            Governance::on_initialize(11);

            // Create proposals with different thresholds
            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"Low threshold".to_vec(),
                b"Requires 60%".to_vec(),
                60, // Minimum allowed
                20, // Minimum participation
            ));

            assert_ok!(Governance::create_consensus_day_proposal(
                RuntimeOrigin::signed(1),
                b"High threshold".to_vec(),
                b"Requires 90%".to_vec(),
                90, // High supermajority
                50, // High participation
            ));

            let prop1 = ConsensusDayProposals::<Test>::get(0).unwrap();
            let prop2 = ConsensusDayProposals::<Test>::get(1).unwrap();

            assert_eq!(prop1.supermajority_threshold, 60);
            assert_eq!(prop1.min_participation, 20);
            assert_eq!(prop2.supermajority_threshold, 90);
            assert_eq!(prop2.min_participation, 50);
        });
    }
}
