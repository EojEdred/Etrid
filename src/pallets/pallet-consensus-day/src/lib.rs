//! # Pallet Consensus Day
//!
//! Annual governance event implementing Ëtrid's constitutional Consensus Day mechanism.
//!
//! ## Overview
//!
//! This pallet implements the Consensus Day system described in the Ëtrid Ivory Papers Vol III:
//! - Annual governance event occurring December 1st at 12:00 AM PST
//! - 4-phase structure: Registration (6h), Voting (12h), Minting (3h), Distribution (1h)
//! - Proposal submission with 10,000 ËTR bond requirement
//! - Time-weighted voting power (staked ËTR × coinage)
//! - Dual quorum system (33% community + 51% validators)
//! - Inflation rate voting (0-5% cap)
//! - Director elections (9 positions)
//! - Participation rewards for all voters
//!
//! ## Consensus Day Phases
//!
//! **Phase 1: Registration (6 hours)**
//! - Submit proposals with 10,000 ËTR bond
//! - Lock stakes for voting power
//! - Validators signal participation
//! - Delegate assignment
//!
//! **Phase 2: Voting (12 hours)**
//! - Vote on proposals (Yes/No/Abstain)
//! - Voting power = Staked ËTR × Coinage multiplier
//! - Quorum: 33% community + 51% validators
//! - Approval: >50% for budget/params, >66% for upgrades
//!
//! **Phase 3: Minting (3 hours)**
//! - Execute approved budgets
//! - Mint new ËTR (0-5% inflation cap)
//! - Apply voted inflation rate
//! - Fund treasury
//!
//! **Phase 4: Distribution (1 hour)**
//! - Distribute participation rewards (1% of minted)
//! - Validator bonus (0.5%)
//! - Director stipends (0.2%)
//! - Proposer rewards (100 ËTR per approved proposal)
//!
//! ## Proposal System
//!
//! ```rust
//! // Proposal categories
//! enum ProposalCategory {
//!     InflationRate,      // Set annual inflation (0-5%)
//!     ParameterChange,    // Adjust protocol parameters
//!     BudgetAllocation,   // Allocate funds for development/grants
//!     ProtocolUpgrade,    // Runtime upgrades
//!     DirectorElection,   // Elect 9 directors
//!     EmergencyAction,    // Emergency protocol changes
//! }
//! ```
//!
//! ## Voting Power Calculation
//!
//! ```rust
//! // Voting power = base_stake × duration_multiplier × history_multiplier
//! // Duration bonus: max +20% for long-term stakes
//! // History bonus: max +10% for consistent participation
//! ```
//!
//! ## Extrinsics
//!
//! - `start_consensus_day()` - Initiate annual event (governance only)
//! - `advance_phase()` - Move to next phase when time elapsed
//! - `submit_proposal()` - Submit proposal with 10,000 ËTR bond
//! - `vote()` - Cast vote on proposal with voting power
//! - `claim_participation_reward()` - Claim voter reward after distribution
//! - `nominate_director()` - Nominate self for director election
//! - `vote_director()` - Vote for director candidate
//!
//! ## Storage
//!
//! - `ConsensusDayState` - Current phase and timing
//! - `Proposals` - All submitted proposals
//! - `Votes` - Vote records per account per proposal
//! - `VotingPower` - Calculated voting power per account
//! - `InflationRate` - Current and voted inflation rate
//! - `DirectorCandidates` - Candidates for director election
//! - `DirectorVotes` - Director election votes
//! - `ParticipationRewards` - Pending participation rewards

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, DecodeWithMemTracking, Encode};
use frame_support::pallet_prelude::*;

/// Budget allocation categories for treasury management
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum BudgetCategory {
    /// Infrastructure development and tooling
    Infrastructure,
    /// Marketing and adoption initiatives
    Marketing,
    /// Security audits and bug bounties
    Security,
    /// Community grants and programs
    CommunityGrants,
    /// Protocol operations and maintenance
    Operations,
    /// Research and development
    Research,
    /// Legal and compliance
    Legal,
    /// Emergency fund reserves
    EmergencyReserves,
}

/// Consensus Day phases
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum Phase {
    /// Not active
    Inactive,
    /// Registration phase (6 hours)
    Registration,
    /// Voting phase (12 hours)
    Voting,
    /// Minting phase (3 hours)
    Minting,
    /// Distribution phase (1 hour)
    Distribution,
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Inactive
    }
}

/// Proposal categories
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum ProposalCategory {
    /// Set annual inflation rate (0-5%)
    InflationRate,
    /// Change protocol parameters
    ParameterChange,
    /// Allocate budget for development/grants
    BudgetAllocation,
    /// Protocol runtime upgrade
    ProtocolUpgrade,
    /// Director election vote
    DirectorElection,
    /// Emergency action
    EmergencyAction,
}

/// Vote options
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::{BudgetCategory, Phase, ProposalCategory, VoteType};
    use codec::{Encode, Decode};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency, ExistenceRequirement, Get};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Zero, Saturating, CheckedDiv, CheckedMul, AccountIdConversion, Bounded};
    use sp_runtime::{Permill, Percent};
    use sp_std::vec::Vec;

    /// Treasury interface trait for funding operations
    pub trait TreasuryInterface<AccountId, Balance> {
        /// Fund the treasury with minted tokens and category breakdowns
        fn fund_treasury(
            from: &AccountId,
            amount: Balance,
            categories: Vec<(BudgetCategory, Balance)>,
        ) -> DispatchResult;
    }

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Consensus Day state
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct ConsensusDayInfo<T: Config> {
        /// Current phase
        pub phase: Phase,
        /// Block when current phase started
        pub phase_start_block: BlockNumberFor<T>,
        /// Block when Consensus Day started
        pub event_start_block: BlockNumberFor<T>,
        /// Year/iteration of Consensus Day
        pub year: u32,
    }

    impl<T: Config> Default for ConsensusDayInfo<T> {
        fn default() -> Self {
            Self {
                phase: Phase::Inactive,
                phase_start_block: Zero::zero(),
                event_start_block: Zero::zero(),
                year: 0,
            }
        }
    }

    /// Proposal data
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<AccountId, Balance, BoundedString> {
        /// Proposal ID
        pub id: u64,
        /// Proposer account
        pub proposer: AccountId,
        /// Proposal title (max 100 chars)
        pub title: BoundedString,
        /// Proposal category
        pub category: ProposalCategory,
        /// Budget request (if applicable)
        pub budget_request: Balance,
        /// Budget category (for treasury allocation)
        pub budget_category: Option<BudgetCategory>,
        /// Proposal bond (10,000 ËTR)
        pub bond: Balance,
        /// Total Yes votes (by voting power)
        pub yes_votes: u128,
        /// Total No votes (by voting power)
        pub no_votes: u128,
        /// Total Abstain votes (by voting power)
        pub abstain_votes: u128,
        /// Number of validators who voted
        pub validator_count: u32,
        /// Proposal approved flag
        pub approved: bool,
        /// Proposal executed flag
        pub executed: bool,
    }

    /// Vote record
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    pub struct VoteRecord {
        /// Vote type
        pub vote: VoteType,
        /// Voting power used
        pub voting_power: u128,
        /// Is validator
        pub is_validator: bool,
    }

    /// Voting power info
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct VotingPowerInfo<T: Config> {
        /// Base staked amount
        pub staked_amount: BalanceOf<T>,
        /// Block when stake was locked
        pub stake_locked_at: BlockNumberFor<T>,
        /// Number of previous Consensus Days participated
        pub participation_history: u32,
        /// Calculated voting power
        pub voting_power: u128,
    }

    impl<T: Config> Default for VotingPowerInfo<T> {
        fn default() -> Self {
            Self {
                staked_amount: Zero::zero(),
                stake_locked_at: Zero::zero(),
                participation_history: 0,
                voting_power: 0,
            }
        }
    }

    /// Director candidate
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    pub struct DirectorCandidate<AccountId, Balance> {
        /// Candidate account
        pub account: AccountId,
        /// Minimum stake (128 ËTR)
        pub stake: Balance,
        /// Total votes received
        pub votes: u128,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Currency for bonds and rewards (ËTR)
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Treasury interface for fund distribution
        type Treasury: TreasuryInterface<Self::AccountId, BalanceOf<Self>>;

        /// Registration phase duration (6 hours = 21,600 blocks at 1s/block)
        #[pallet::constant]
        type RegistrationDuration: Get<u32>;

        /// Voting phase duration (12 hours = 43,200 blocks)
        #[pallet::constant]
        type VotingDuration: Get<u32>;

        /// Minting phase duration (3 hours = 10,800 blocks)
        #[pallet::constant]
        type MintingDuration: Get<u32>;

        /// Distribution phase duration (1 hour = 3,600 blocks)
        #[pallet::constant]
        type DistributionDuration: Get<u32>;

        /// Proposal bond amount (10,000 ËTR)
        #[pallet::constant]
        type ProposalBond: Get<BalanceOf<Self>>;

        /// Director minimum stake (128 ËTR)
        #[pallet::constant]
        type DirectorMinStake: Get<BalanceOf<Self>>;

        /// Maximum inflation rate (500 = 5%)
        #[pallet::constant]
        type MaxInflationBps: Get<u32>;

        /// Maximum proposals
        #[pallet::constant]
        type MaxProposals: Get<u32>;

        /// Maximum title length
        #[pallet::constant]
        type MaxTitleLength: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Current Consensus Day state
    #[pallet::storage]
    #[pallet::getter(fn consensus_day_state)]
    pub type ConsensusDayState<T: Config> = StorageValue<
        _,
        ConsensusDayInfo<T>,
        ValueQuery,
    >;

    /// All proposals (proposal_id → proposal data)
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,  // Proposal ID
        Proposal<
            T::AccountId,
            BalanceOf<T>,
            BoundedVec<u8, T::MaxTitleLength>,
        >,
        OptionQuery,
    >;

    /// Next proposal ID
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Vote records (account + proposal_id → vote)
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Voter
        Blake2_128Concat,
        u64,  // Proposal ID
        VoteRecord,
        OptionQuery,
    >;

    /// Voting power per account
    #[pallet::storage]
    #[pallet::getter(fn voting_power)]
    pub type VotingPowerMap<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        VotingPowerInfo<T>,
        ValueQuery,
    >;

    /// Current inflation rate (basis points: 300 = 3%)
    #[pallet::storage]
    #[pallet::getter(fn inflation_rate)]
    pub type InflationRate<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Voted inflation rate (to be applied next year)
    #[pallet::storage]
    #[pallet::getter(fn voted_inflation_rate)]
    pub type VotedInflationRate<T: Config> = StorageValue<_, u32, OptionQuery>;

    /// Director candidates
    #[pallet::storage]
    #[pallet::getter(fn director_candidates)]
    pub type DirectorCandidates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        DirectorCandidate<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// Director votes (voter → candidate)
    #[pallet::storage]
    #[pallet::getter(fn director_votes)]
    pub type DirectorVotes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Voter
        T::AccountId,  // Candidate voted for
        OptionQuery,
    >;

    /// Elected directors (9 positions)
    #[pallet::storage]
    #[pallet::getter(fn elected_directors)]
    pub type ElectedDirectors<T: Config> = StorageValue<
        _,
        BoundedVec<T::AccountId, ConstU32<9>>,
        ValueQuery,
    >;

    /// Participation rewards pending claim
    #[pallet::storage]
    #[pallet::getter(fn participation_rewards)]
    pub type ParticipationRewards<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Total minted during current Consensus Day
    #[pallet::storage]
    #[pallet::getter(fn total_minted)]
    pub type TotalMinted<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Total circulating supply (for quorum calculation)
    #[pallet::storage]
    #[pallet::getter(fn circulating_supply)]
    pub type CirculatingSupply<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Total active validators
    #[pallet::storage]
    #[pallet::getter(fn active_validator_count)]
    pub type ActiveValidatorCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Validator accounts (for quorum checking)
    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,  // Is validator
        ValueQuery,
    >;

    /// Proposals voted on by account (for completeness bonus)
    #[pallet::storage]
    #[pallet::getter(fn proposals_voted)]
    pub type ProposalsVoted<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u32,  // Number of proposals voted on
        ValueQuery,
    >;

    /// Treasury allocations by category for current Consensus Day
    #[pallet::storage]
    #[pallet::getter(fn treasury_allocations)]
    pub type TreasuryAllocations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BudgetCategory,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Total amount transferred to treasury during current Consensus Day
    #[pallet::storage]
    #[pallet::getter(fn total_treasury_funded)]
    pub type TotalTreasuryFunded<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Consensus Day started [year, start_block]
        ConsensusDayStarted(u32, BlockNumberFor<T>),
        /// Phase advanced [old_phase, new_phase, block]
        PhaseAdvanced(Phase, Phase, BlockNumberFor<T>),
        /// Proposal submitted [proposal_id, proposer, category, bond]
        ProposalSubmitted(u64, T::AccountId, ProposalCategory, BalanceOf<T>),
        /// Vote cast [voter, proposal_id, vote_type, voting_power]
        VoteCast(T::AccountId, u64, VoteType, u128),
        /// Proposal approved [proposal_id, yes_votes, no_votes]
        ProposalApproved(u64, u128, u128),
        /// Proposal rejected [proposal_id, yes_votes, no_votes]
        ProposalRejected(u64, u128, u128),
        /// Budget minted [proposal_id, amount]
        BudgetMinted(u64, BalanceOf<T>),
        /// Inflation rate set [old_rate, new_rate]
        InflationRateSet(u32, u32),
        /// Participation reward calculated [account, amount]
        ParticipationRewardCalculated(T::AccountId, BalanceOf<T>),
        /// Participation reward claimed [account, amount]
        ParticipationRewardClaimed(T::AccountId, BalanceOf<T>),
        /// Director nominated [candidate, stake]
        DirectorNominated(T::AccountId, BalanceOf<T>),
        /// Director vote cast [voter, candidate, voting_power]
        DirectorVoteCast(T::AccountId, T::AccountId, u128),
        /// Directors elected [directors]
        DirectorsElected(Vec<T::AccountId>),
        /// Consensus Day completed [year, total_minted]
        ConsensusDayCompleted(u32, BalanceOf<T>),
        /// Treasury funded [category, amount]
        TreasuryFunded(BudgetCategory, BalanceOf<T>),
        /// Treasury transfer completed [total_amount, num_categories]
        TreasuryTransferCompleted(BalanceOf<T>, u32),
        /// Budget allocation approved with category [proposal_id, category, amount]
        BudgetAllocationCategorized(u64, BudgetCategory, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Consensus Day not active
        NotActive,
        /// Wrong phase for this action
        WrongPhase,
        /// Proposal bond insufficient
        InsufficientBond,
        /// Proposal not found
        ProposalNotFound,
        /// Already voted on this proposal
        AlreadyVoted,
        /// No voting power (must lock stake first)
        NoVotingPower,
        /// Quorum not met
        QuorumNotMet,
        /// Proposal not approved
        ProposalNotApproved,
        /// Inflation rate exceeds maximum (5%)
        InflationRateTooHigh,
        /// No participation rewards to claim
        NoParticipationRewards,
        /// Director stake insufficient (need 128 ËTR)
        InsufficientDirectorStake,
        /// Already nominated as director
        AlreadyNominatedDirector,
        /// Not a validator
        NotValidator,
        /// Title too long
        TitleTooLong,
        /// Too many proposals
        TooManyProposals,
        /// Phase duration not elapsed
        PhaseDurationNotElapsed,
        /// Consensus Day already active
        AlreadyActive,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Start Consensus Day (governance/root only)
        ///
        /// Initiates the annual Consensus Day event
        #[pallet::call_index(0)]
        #[pallet::weight(100_000)]
        pub fn start_consensus_day(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
            let state = ConsensusDayState::<T>::get();

            ensure!(state.phase == Phase::Inactive, Error::<T>::AlreadyActive);

            let year = state.year.saturating_add(1);

            let new_state = ConsensusDayInfo {
                phase: Phase::Registration,
                phase_start_block: current_block,
                event_start_block: current_block,
                year,
            };

            ConsensusDayState::<T>::put(new_state);
            TotalMinted::<T>::put(BalanceOf::<T>::zero());

            Self::deposit_event(Event::ConsensusDayStarted(year, current_block));

            Ok(())
        }

        /// Advance to next phase
        ///
        /// Moves Consensus Day to the next phase when duration elapsed
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn advance_phase(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let mut state = ConsensusDayState::<T>::get();
            ensure!(state.phase != Phase::Inactive, Error::<T>::NotActive);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let blocks_elapsed = current_block.saturating_sub(state.phase_start_block);

            let phase_duration = match state.phase {
                Phase::Registration => T::RegistrationDuration::get(),
                Phase::Voting => T::VotingDuration::get(),
                Phase::Minting => T::MintingDuration::get(),
                Phase::Distribution => T::DistributionDuration::get(),
                Phase::Inactive => return Err(Error::<T>::NotActive.into()),
            };

            ensure!(
                blocks_elapsed >= phase_duration.into(),
                Error::<T>::PhaseDurationNotElapsed
            );

            let old_phase = state.phase.clone();
            let new_phase = match state.phase {
                Phase::Registration => Phase::Voting,
                Phase::Voting => {
                    // Execute quorum checks and finalize votes
                    Self::finalize_voting()?;
                    Phase::Minting
                }
                Phase::Minting => {
                    // Execute approved proposals
                    Self::execute_minting()?;
                    Phase::Distribution
                }
                Phase::Distribution => {
                    // Calculate and distribute rewards
                    Self::execute_distribution()?;
                    Phase::Inactive
                }
                Phase::Inactive => Phase::Inactive,
            };

            state.phase = new_phase.clone();
            state.phase_start_block = current_block;
            ConsensusDayState::<T>::put(state.clone());

            Self::deposit_event(Event::PhaseAdvanced(old_phase, new_phase, current_block));

            if state.phase == Phase::Inactive {
                let total_minted = TotalMinted::<T>::get();
                Self::deposit_event(Event::ConsensusDayCompleted(state.year, total_minted));
            }

            Ok(())
        }

        /// Submit proposal with bond
        ///
        /// Submits a proposal during Registration phase (requires 10,000 ËTR bond)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn submit_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            category: ProposalCategory,
            budget_request: BalanceOf<T>,
            budget_category: Option<BudgetCategory>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Registration, Error::<T>::WrongPhase);

            let proposal_count = NextProposalId::<T>::get();
            ensure!(
                proposal_count < T::MaxProposals::get().into(),
                Error::<T>::TooManyProposals
            );

            let bounded_title: BoundedVec<u8, T::MaxTitleLength> =
                title.try_into().map_err(|_| Error::<T>::TitleTooLong)?;

            // Reserve proposal bond
            let bond = T::ProposalBond::get();
            T::Currency::reserve(&proposer, bond)?;

            let proposal_id = proposal_count;
            let proposal = Proposal {
                id: proposal_id,
                proposer: proposer.clone(),
                title: bounded_title,
                category: category.clone(),
                budget_request,
                budget_category: budget_category.clone(),
                bond,
                yes_votes: 0,
                no_votes: 0,
                abstain_votes: 0,
                validator_count: 0,
                approved: false,
                executed: false,
            };

            Proposals::<T>::insert(proposal_id, proposal);
            NextProposalId::<T>::put(proposal_id.saturating_add(1));

            Self::deposit_event(Event::ProposalSubmitted(
                proposal_id,
                proposer,
                category,
                bond,
            ));

            // Emit category event if budget allocation
            if let Some(ref cat) = budget_category {
                Self::deposit_event(Event::BudgetAllocationCategorized(
                    proposal_id,
                    cat.clone(),
                    budget_request,
                ));
            }

            Ok(())
        }

        /// Cast vote on proposal
        ///
        /// Vote Yes/No/Abstain with calculated voting power
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: u64,
            vote: VoteType,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Voting, Error::<T>::WrongPhase);

            ensure!(
                !Votes::<T>::contains_key(&voter, proposal_id),
                Error::<T>::AlreadyVoted
            );

            let mut proposal = Proposals::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;

            let voting_power_info = VotingPowerMap::<T>::get(&voter);
            ensure!(
                voting_power_info.voting_power > 0,
                Error::<T>::NoVotingPower
            );

            let is_validator = Validators::<T>::get(&voter);

            // Update proposal vote counts
            match vote {
                VoteType::Yes => proposal.yes_votes = proposal.yes_votes.saturating_add(voting_power_info.voting_power),
                VoteType::No => proposal.no_votes = proposal.no_votes.saturating_add(voting_power_info.voting_power),
                VoteType::Abstain => proposal.abstain_votes = proposal.abstain_votes.saturating_add(voting_power_info.voting_power),
            }

            if is_validator {
                proposal.validator_count = proposal.validator_count.saturating_add(1);
            }

            // Record vote
            let vote_record = VoteRecord {
                vote: vote.clone(),
                voting_power: voting_power_info.voting_power,
                is_validator,
            };

            Votes::<T>::insert(&voter, proposal_id, vote_record);
            Proposals::<T>::insert(proposal_id, proposal);

            // Track proposals voted on (for completeness bonus)
            ProposalsVoted::<T>::mutate(&voter, |count| *count = count.saturating_add(1));

            Self::deposit_event(Event::VoteCast(
                voter,
                proposal_id,
                vote,
                voting_power_info.voting_power,
            ));

            Ok(())
        }

        /// Lock stake to gain voting power
        ///
        /// Must be called during Registration phase to participate
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn lock_stake_for_voting(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let staker = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Registration, Error::<T>::WrongPhase);

            // Reserve stake
            T::Currency::reserve(&staker, amount)?;

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Get existing voting power info or create new
            let mut vp_info = VotingPowerMap::<T>::get(&staker);

            if vp_info.staked_amount.is_zero() {
                vp_info.stake_locked_at = current_block;
            }

            vp_info.staked_amount = vp_info.staked_amount.saturating_add(amount);

            // Calculate voting power
            let voting_power = Self::calculate_voting_power(&vp_info, current_block);
            vp_info.voting_power = voting_power;

            VotingPowerMap::<T>::insert(&staker, vp_info);

            Ok(())
        }

        /// Claim participation reward
        ///
        /// Claim reward for voting during Consensus Day
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn claim_participation_reward(origin: OriginFor<T>) -> DispatchResult {
            let claimer = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(
                state.phase == Phase::Distribution || state.phase == Phase::Inactive,
                Error::<T>::WrongPhase
            );

            let reward = ParticipationRewards::<T>::get(&claimer);
            ensure!(!reward.is_zero(), Error::<T>::NoParticipationRewards);

            // Transfer reward
            T::Currency::transfer(
                &Self::account_id(),
                &claimer,
                reward,
                ExistenceRequirement::KeepAlive,
            )?;

            ParticipationRewards::<T>::remove(&claimer);

            Self::deposit_event(Event::ParticipationRewardClaimed(claimer, reward));

            Ok(())
        }

        /// Nominate self for director election
        ///
        /// Requires 128 ËTR minimum stake
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn nominate_director(origin: OriginFor<T>) -> DispatchResult {
            let candidate = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Registration, Error::<T>::WrongPhase);

            ensure!(
                !DirectorCandidates::<T>::contains_key(&candidate),
                Error::<T>::AlreadyNominatedDirector
            );

            let min_stake = T::DirectorMinStake::get();
            let balance = T::Currency::free_balance(&candidate);
            ensure!(balance >= min_stake, Error::<T>::InsufficientDirectorStake);

            // Reserve stake
            T::Currency::reserve(&candidate, min_stake)?;

            let director_candidate = DirectorCandidate {
                account: candidate.clone(),
                stake: min_stake,
                votes: 0,
            };

            DirectorCandidates::<T>::insert(&candidate, director_candidate);

            Self::deposit_event(Event::DirectorNominated(candidate, min_stake));

            Ok(())
        }

        /// Vote for director candidate
        ///
        /// Vote for one of the director candidates
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn vote_director(
            origin: OriginFor<T>,
            candidate: T::AccountId,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Voting, Error::<T>::WrongPhase);

            let mut director_candidate = DirectorCandidates::<T>::get(&candidate)
                .ok_or(Error::<T>::ProposalNotFound)?;

            let voting_power_info = VotingPowerMap::<T>::get(&voter);
            ensure!(
                voting_power_info.voting_power > 0,
                Error::<T>::NoVotingPower
            );

            // Update director votes
            director_candidate.votes = director_candidate.votes.saturating_add(voting_power_info.voting_power);
            DirectorCandidates::<T>::insert(&candidate, director_candidate);

            DirectorVotes::<T>::insert(&voter, &candidate);

            Self::deposit_event(Event::DirectorVoteCast(
                voter,
                candidate,
                voting_power_info.voting_power,
            ));

            Ok(())
        }

        /// Register validator (called by validator pallet)
        #[pallet::call_index(8)]
        #[pallet::weight(5_000)]
        pub fn register_validator(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if !Validators::<T>::get(&validator) {
                Validators::<T>::insert(&validator, true);
                ActiveValidatorCount::<T>::mutate(|count| *count = count.saturating_add(1));
            }

            Ok(())
        }

        /// Update circulating supply (called by system)
        #[pallet::call_index(9)]
        #[pallet::weight(5_000)]
        pub fn update_circulating_supply(
            origin: OriginFor<T>,
            supply: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            CirculatingSupply::<T>::put(supply);
            Ok(())
        }

        /// Execute approved budgets and fund treasury
        ///
        /// This is typically called automatically during phase transition to Minting,
        /// but can be called manually if needed
        #[pallet::call_index(10)]
        #[pallet::weight(100_000)]
        pub fn execute_approved_budgets(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let state = ConsensusDayState::<T>::get();
            ensure!(state.phase == Phase::Minting, Error::<T>::WrongPhase);

            Self::execute_minting_and_treasury_funding()?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Pallet account ID (holds reward pool)
        pub fn account_id() -> T::AccountId {
            frame_support::PalletId(*b"py/cnsdy").into_account_truncating()
        }

        /// Calculate voting power
        ///
        /// Formula: base_stake × duration_multiplier × history_multiplier
        /// - Duration bonus: max +20% for long-term stakes
        /// - History bonus: max +10% for consistent participation
        pub fn calculate_voting_power(
            info: &VotingPowerInfo<T>,
            current_block: BlockNumberFor<T>,
        ) -> u128 {
            let base_power = Self::balance_to_u128(info.staked_amount);

            // Calculate stake duration in blocks
            let duration = current_block.saturating_sub(info.stake_locked_at);
            let duration_u32: u32 = duration.try_into().unwrap_or(u32::MAX);

            // Duration multiplier (max +20%)
            // Assumes 365 days = ~31,536,000 blocks at 1s/block
            let blocks_per_year = 31_536_000u32;
            let duration_bonus_bps = (duration_u32 as u64 * 2000 / blocks_per_year as u64).min(2000) as u32;
            let duration_multiplier = 10000u32.saturating_add(duration_bonus_bps);

            // Participation history bonus (max +10%)
            let history_bonus_bps = (info.participation_history.saturating_mul(200)).min(1000);
            let history_multiplier = 10000u32.saturating_add(history_bonus_bps);

            // Combined: base × (1 + duration_bonus) × (1 + history_bonus)
            let adjusted_power = (base_power as u128)
                .saturating_mul(duration_multiplier as u128)
                .saturating_mul(history_multiplier as u128)
                / 100_000_000u128; // Divide by 10000 × 10000

            adjusted_power
        }

        /// Finalize voting (called at end of voting phase)
        fn finalize_voting() -> DispatchResult {
            let circulating = CirculatingSupply::<T>::get();
            let active_validators = ActiveValidatorCount::<T>::get();

            // Check quorum and approval for each proposal
            for (proposal_id, mut proposal) in Proposals::<T>::iter() {
                let total_votes = proposal.yes_votes
                    .saturating_add(proposal.no_votes)
                    .saturating_add(proposal.abstain_votes);

                // Community quorum: 33% of circulating supply
                let community_quorum_met = total_votes >= Self::balance_to_u128(circulating).saturating_mul(33) / 100;

                // Validator quorum: 51% of active validators
                let validator_quorum_met = proposal.validator_count >= active_validators.saturating_mul(51) / 100;

                let quorum_met = community_quorum_met && validator_quorum_met;

                if quorum_met {
                    let total_decisive_votes = proposal.yes_votes.saturating_add(proposal.no_votes);

                    // Determine threshold based on category
                    let threshold_bps = match proposal.category {
                        ProposalCategory::BudgetAllocation => 5000,  // 50%
                        ProposalCategory::ParameterChange => 5000,   // 50%
                        ProposalCategory::InflationRate => 5000,     // 50%
                        ProposalCategory::ProtocolUpgrade => 6600,   // 66%
                        ProposalCategory::EmergencyAction => 6600,   // 66%
                        ProposalCategory::DirectorElection => 5000, // 50%
                    };

                    let approval_ratio = if total_decisive_votes > 0 {
                        proposal.yes_votes.saturating_mul(10000) / total_decisive_votes
                    } else {
                        0
                    };

                    if approval_ratio >= threshold_bps as u128 {
                        proposal.approved = true;
                        Self::deposit_event(Event::ProposalApproved(
                            proposal_id,
                            proposal.yes_votes,
                            proposal.no_votes,
                        ));

                        // Unreserve bond
                        T::Currency::unreserve(&proposal.proposer, proposal.bond);
                    } else {
                        Self::deposit_event(Event::ProposalRejected(
                            proposal_id,
                            proposal.yes_votes,
                            proposal.no_votes,
                        ));

                        // Return bond (reached quorum)
                        T::Currency::unreserve(&proposal.proposer, proposal.bond);
                    }
                } else {
                    // Quorum not met - slash 50% of bond
                    let slash_amount = proposal.bond / 2u32.into();
                    let _ = T::Currency::slash_reserved(&proposal.proposer, slash_amount);
                    let _ = T::Currency::unreserve(&proposal.proposer, proposal.bond.saturating_sub(slash_amount));

                    Self::deposit_event(Event::ProposalRejected(
                        proposal_id,
                        proposal.yes_votes,
                        proposal.no_votes,
                    ));
                }

                Proposals::<T>::insert(proposal_id, proposal);
            }

            Ok(())
        }

        /// Execute minting for approved proposals
        fn execute_minting() -> DispatchResult {
            Self::execute_minting_and_treasury_funding()
        }

        /// Execute minting and treasury funding
        fn execute_minting_and_treasury_funding() -> DispatchResult {
            let circulating = CirculatingSupply::<T>::get();
            let max_inflation_bps = T::MaxInflationBps::get();
            let max_mintable = Self::balance_to_u128(circulating)
                .saturating_mul(max_inflation_bps as u128) / 10000u128;

            let mut total_minted = 0u128;
            let mut category_allocations: Vec<(BudgetCategory, BalanceOf<T>)> = Vec::new();

            // Reset treasury allocations for new Consensus Day
            TotalTreasuryFunded::<T>::put(BalanceOf::<T>::zero());
            let _ = TreasuryAllocations::<T>::clear(u32::MAX, None);

            // Execute approved budget proposals
            for (proposal_id, mut proposal) in Proposals::<T>::iter() {
                if proposal.approved && !proposal.executed {
                    if let ProposalCategory::BudgetAllocation = proposal.category {
                        let budget = Self::balance_to_u128(proposal.budget_request);

                        // Check if within inflation cap
                        if total_minted.saturating_add(budget) <= max_mintable {
                            // Mint tokens to pallet account
                            let amount_to_mint = proposal.budget_request;
                            let pallet_account = Self::account_id();

                            // Mint new tokens to pallet account for treasury funding
                            T::Currency::deposit_creating(&pallet_account, amount_to_mint);

                            total_minted = total_minted.saturating_add(budget);
                            proposal.executed = true;

                            // Track category allocation
                            if let Some(ref category) = proposal.budget_category {
                                TreasuryAllocations::<T>::mutate(category, |balance| {
                                    *balance = balance.saturating_add(amount_to_mint);
                                });
                                category_allocations.push((category.clone(), amount_to_mint));

                                Self::deposit_event(Event::TreasuryFunded(
                                    category.clone(),
                                    amount_to_mint,
                                ));
                            }

                            Self::deposit_event(Event::BudgetMinted(
                                proposal_id,
                                proposal.budget_request,
                            ));

                            Proposals::<T>::insert(proposal_id, proposal);
                        }
                    }
                }
            }

            // Transfer minted funds to treasury if any allocations were made
            if !category_allocations.is_empty() {
                let total_treasury_amount = Self::u128_to_balance(total_minted);
                let pallet_account = Self::account_id();

                // Fund treasury with categorized allocations
                T::Treasury::fund_treasury(
                    &pallet_account,
                    total_treasury_amount,
                    category_allocations.clone(),
                )?;

                TotalTreasuryFunded::<T>::put(total_treasury_amount);

                Self::deposit_event(Event::TreasuryTransferCompleted(
                    total_treasury_amount,
                    category_allocations.len() as u32,
                ));
            }

            // Apply voted inflation rate
            if let Some(voted_rate) = VotedInflationRate::<T>::get() {
                let old_rate = InflationRate::<T>::get();
                InflationRate::<T>::put(voted_rate);
                VotedInflationRate::<T>::kill();

                Self::deposit_event(Event::InflationRateSet(old_rate, voted_rate));
            }

            TotalMinted::<T>::put(Self::u128_to_balance(total_minted));

            Ok(())
        }

        /// Execute distribution of participation rewards
        fn execute_distribution() -> DispatchResult {
            let total_minted = TotalMinted::<T>::get();
            let total_proposals = NextProposalId::<T>::get();

            // Calculate participation pool (1% of total minted)
            let participation_pool = total_minted / 100u32.into();

            // Calculate total voting power used
            let mut total_voting_power = 0u128;
            for (voter, _) in VotingPowerMap::<T>::iter() {
                let proposals_voted = ProposalsVoted::<T>::get(&voter);
                if proposals_voted > 0 {
                    let vp_info = VotingPowerMap::<T>::get(&voter);
                    total_voting_power = total_voting_power.saturating_add(vp_info.voting_power);
                }
            }

            if total_voting_power > 0 {
                // Distribute rewards proportionally
                for (voter, _) in VotingPowerMap::<T>::iter() {
                    let proposals_voted = ProposalsVoted::<T>::get(&voter);

                    if proposals_voted > 0 {
                        let vp_info = VotingPowerMap::<T>::get(&voter);
                        let voter_share = vp_info.voting_power as u128 * Self::balance_to_u128(participation_pool) / total_voting_power;

                        // Completeness bonus (20% if voted on all proposals)
                        let completeness_multiplier = if proposals_voted as u64 >= total_proposals {
                            12000u128 // 1.2x
                        } else {
                            10000u128 // 1.0x
                        };

                        let final_reward = voter_share.saturating_mul(completeness_multiplier) / 10000u128;

                        ParticipationRewards::<T>::insert(
                            &voter,
                            Self::u128_to_balance(final_reward),
                        );

                        Self::deposit_event(Event::ParticipationRewardCalculated(
                            voter.clone(),
                            Self::u128_to_balance(final_reward),
                        ));

                        // Increment participation history
                        VotingPowerMap::<T>::mutate(&voter, |info| {
                            info.participation_history = info.participation_history.saturating_add(1);
                        });
                    }
                }
            }

            // Elect top 9 directors
            let mut candidates: Vec<_> = DirectorCandidates::<T>::iter()
                .map(|(_, candidate)| candidate)
                .collect();

            // Sort by votes (descending)
            candidates.sort_by(|a, b| b.votes.cmp(&a.votes));

            let elected: Vec<T::AccountId> = candidates
                .into_iter()
                .take(9)
                .map(|c| c.account)
                .collect();

            if elected.len() > 0 {
                let bounded_elected: BoundedVec<T::AccountId, ConstU32<9>> =
                    elected.clone().try_into().unwrap_or_default();
                ElectedDirectors::<T>::put(bounded_elected);

                Self::deposit_event(Event::DirectorsElected(elected));
            }

            Ok(())
        }

        /// Helper: Convert Balance to u128
        fn balance_to_u128(balance: BalanceOf<T>) -> u128 {
            balance.try_into().unwrap_or(u128::MAX)
        }

        /// Helper: Convert u128 to Balance
        fn u128_to_balance(value: u128) -> BalanceOf<T> {
            value.try_into().unwrap_or_else(|_| Bounded::max_value())
        }
    }
}
