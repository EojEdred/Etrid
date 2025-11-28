//! # Pallet Director Election
//!
//! Annual Decentralized Director Election Mechanism for Ëtrid Blockchain
//!
//! ## Overview
//!
//! This pallet implements the annual director election system as specified in the Ëtrid
//! Ivory Papers (11-peer-roles/ARCHITECTURE.md lines 1035-1064). It manages the election
//! of 9 Decentralized Directors through a fully automated, three-phase election cycle.
//!
//! ## Election Cycle (365 days)
//!
//! ### Phase 1: Governance (335 days)
//! - Normal network operations
//! - Elected directors govern the network
//! - Election countdown visible
//!
//! ### Phase 2: Nomination (30 days before Consensus Day)
//! - Directors with 128+ ËTR can register as candidates
//! - Submit manifesto/platform
//! - Campaign to community
//! - No voting yet
//!
//! ### Phase 3: Voting (7 days before Consensus Day)
//! - All stakeholders can vote
//! - Stake-weighted voting power with role multipliers
//! - Can change vote anytime before election ends
//! - One vote per account
//!
//! ### Consensus Day (T-0)
//! - Voting closes at midnight UTC
//! - Votes automatically tallied
//! - Top 9 candidates elected
//! - Winners immediately seated as directors
//! - New governance phase begins
//!
//! ## Voting Power Calculation
//!
//! ```text
//! voting_power = stake × role_multiplier
//!
//! Role Multipliers:
//! - DecentralizedDirector: 3x
//! - ValidityNode/FlareNode: 2x
//! - CommonStakePeer: 1x
//! - CommonPeer: 0x (cannot vote)
//! ```
//!
//! ## Key Features
//!
//! - **Automated Transitions**: Phases advance via `on_initialize` hook
//! - **Stake-Weighted Voting**: Higher stake = more voting power
//! - **Role-Based Multipliers**: Directors and validators have enhanced power
//! - **Vote Changes**: Voters can change their vote before deadline
//! - **Automatic Seating**: Winners are seated immediately on Consensus Day
//! - **Tie Breaking**: Deterministic tiebreaker using AccountId hash
//! - **Edge Case Handling**: Handles withdrawals, < 9 candidates, etc.
//!
//! ## Storage Items
//!
//! - `ElectionPhase` - Current election phase and timing
//! - `Candidates` - All registered candidates with manifestos
//! - `Votes` - Vote records (voter → candidate)
//! - `ElectionResults` - Historical election results
//! - `NextConsensusDayBlock` - Block number of next Consensus Day
//! - `ElectedDirectors` - Currently serving 9 directors
//! - `CurrentEpoch` - Current election epoch counter
//!
//! ## Extrinsics
//!
//! - `register_candidate()` - Register as candidate (requires 128+ ËTR)
//! - `withdraw_candidacy()` - Withdraw from election
//! - `vote()` - Vote for a candidate
//! - `change_vote()` - Change vote to different candidate
//! - `remove_vote()` - Remove vote entirely
//! - `trigger_election()` - Manually trigger election start (gov only)
//!
//! ## Events
//!
//! - `ElectionPhaseChanged` - Phase transition occurred
//! - `CandidateRegistered` - New candidate registered
//! - `CandidateWithdrew` - Candidate withdrew
//! - `VoteCast` - Vote cast for candidate
//! - `VoteChanged` - Vote changed to new candidate
//! - `VoteRemoved` - Vote removed
//! - `ElectionCompleted` - Election finished, winners announced
//! - `DirectorsSeated` - Directors seated and term started
//!
//! ## Configuration
//!
//! ```rust
//! // Block durations (6 second blocks)
//! GovernancePeriodBlocks: 5_040_000   // ~335 days
//! NominationPeriodBlocks: 432_000     // ~30 days
//! VotingPeriodBlocks: 100_800         // ~7 days
//! NumDirectorsToElect: 9
//! MinCandidateStake: 128 ËTR
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    pallet_prelude::*,
    traits::{Get},
};
use frame_system::pallet_prelude::*;
use sp_runtime::{
    traits::{Hash, Saturating, Zero},
    RuntimeDebug,
};
use sp_std::vec::Vec;

use peer_roles_staking_types::{Role, RoleInterface};

/// Election phase state
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum ElectionPhaseInfo<BlockNumber> {
    /// Normal governance period
    Governance {
        next_nomination_start: BlockNumber,
    },
    /// Nomination period (30 days before Consensus Day)
    Nomination {
        start: BlockNumber,
        end: BlockNumber,
    },
    /// Voting period (7 days before Consensus Day)
    Voting {
        start: BlockNumber,
        end: BlockNumber,
    },
}

impl<BlockNumber: Default> Default for ElectionPhaseInfo<BlockNumber> {
    fn default() -> Self {
        ElectionPhaseInfo::Governance {
            next_nomination_start: BlockNumber::default(),
        }
    }
}

/// Candidate profile
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
#[scale_info(skip_type_params(AccountId, Balance))]
pub struct CandidateProfile<AccountId: MaxEncodedLen, Balance: MaxEncodedLen> {
    /// Candidate account
    pub account: AccountId,
    /// Staked amount (must be >= 128 ËTR)
    pub stake: Balance,
    /// Manifesto/platform (max 1000 bytes)
    pub manifesto: BoundedVec<u8, ConstU32<1000>>,
    /// Total voting power received
    pub votes_received: u128,
    /// Block when registered
    pub registered_at: u64,
}

/// Vote record
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct VoteRecord<AccountId> {
    /// Candidate voted for
    pub candidate: AccountId,
    /// Voting power used
    pub voting_power: u128,
}

/// Election result record
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct ElectionResult<AccountId> {
    /// Elected directors (top 9)
    pub winners: BoundedVec<AccountId, ConstU32<9>>,
    /// Epoch/year of election
    pub epoch: u32,
    /// Total votes cast
    pub total_votes: u128,
    /// Number of candidates
    pub candidate_count: u32,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    // Note: RoleInterface returns u128 for stake, not a Balance type
    // This is just an alias for clarity in the pallet
    type StakeAmount = u128;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Runtime event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Staking interface for role and stake verification
        type StakingInterface: RoleInterface<Self::AccountId, u128>;

        /// Governance period in blocks (default: 5,040,000 = ~335 days at 6s blocks)
        #[pallet::constant]
        type GovernancePeriodBlocks: Get<BlockNumberFor<Self>>;

        /// Nomination period in blocks (default: 432,000 = ~30 days at 6s blocks)
        #[pallet::constant]
        type NominationPeriodBlocks: Get<BlockNumberFor<Self>>;

        /// Voting period in blocks (default: 100,800 = ~7 days at 6s blocks)
        #[pallet::constant]
        type VotingPeriodBlocks: Get<BlockNumberFor<Self>>;

        /// Number of directors to elect (9)
        #[pallet::constant]
        type NumDirectorsToElect: Get<u32>;

        /// Minimum stake for candidates (128 ËTR in smallest units)
        #[pallet::constant]
        type MinCandidateStake: Get<u128>;
    }

    /// Current election phase
    #[pallet::storage]
    #[pallet::getter(fn election_phase)]
    pub type ElectionPhase<T: Config> =
        StorageValue<_, ElectionPhaseInfo<BlockNumberFor<T>>, ValueQuery>;

    /// Registered candidates (AccountId → CandidateProfile)
    #[pallet::storage]
    #[pallet::getter(fn candidates)]
    pub type Candidates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        CandidateProfile<T::AccountId, u128>,
        OptionQuery,
    >;

    /// Votes cast (Voter → VoteRecord)
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, VoteRecord<T::AccountId>, OptionQuery>;

    /// Election results history (Epoch → ElectionResult)
    #[pallet::storage]
    #[pallet::getter(fn election_results)]
    pub type ElectionResults<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, ElectionResult<T::AccountId>, OptionQuery>;

    /// Next Consensus Day block number
    #[pallet::storage]
    #[pallet::getter(fn next_consensus_day)]
    pub type NextConsensusDayBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// Currently elected directors
    #[pallet::storage]
    #[pallet::getter(fn elected_directors)]
    pub type ElectedDirectors<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, ConstU32<9>>, ValueQuery>;

    /// Current epoch counter
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Election phase changed [old_phase_type, new_phase_type, block]
        ElectionPhaseChanged(u8, u8, BlockNumberFor<T>),
        /// Candidate registered [account, stake]
        CandidateRegistered(T::AccountId, u128),
        /// Candidate withdrew [account]
        CandidateWithdrew(T::AccountId),
        /// Vote cast [voter, candidate, voting_power]
        VoteCast(T::AccountId, T::AccountId, u128),
        /// Vote changed [voter, old_candidate, new_candidate]
        VoteChanged(T::AccountId, T::AccountId, T::AccountId),
        /// Vote removed [voter, candidate]
        VoteRemoved(T::AccountId, T::AccountId),
        /// Election completed [epoch, winners, total_votes]
        ElectionCompleted {
            epoch: u32,
            winners: Vec<T::AccountId>,
            total_votes: u128,
        },
        /// Directors seated [directors, epoch]
        DirectorsSeated(Vec<T::AccountId>, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not in nomination phase
        NotNominationPhase,
        /// Not in voting phase
        NotVotingPhase,
        /// Insufficient stake for candidate (need 128+ ËTR)
        InsufficientStake,
        /// Account does not have DecentralizedDirector role
        NotDirectorRole,
        /// Already registered as candidate
        AlreadyCandidate,
        /// Not a registered candidate
        NotCandidate,
        /// Already voted in this election
        AlreadyVoted,
        /// No vote to change
        NoVoteToChange,
        /// Cannot vote (insufficient stake or no role)
        CannotVote,
        /// Manifesto too long (max 1000 bytes)
        ManifestoTooLong,
        /// No candidates to vote for
        NoCandidates,
        /// Invalid candidate (not in candidate list)
        InvalidCandidate,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as director candidate
        ///
        /// Requirements:
        /// - Must be in Nomination phase
        /// - Must have DecentralizedDirector role (128+ ËTR staked)
        /// - Cannot be already registered
        ///
        /// # Arguments
        /// * `manifesto` - Platform/campaign message (max 1000 bytes)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_candidate(
            origin: OriginFor<T>,
            manifesto: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check phase
            let phase = ElectionPhase::<T>::get();
            ensure!(
                matches!(phase, ElectionPhaseInfo::Nomination { .. }),
                Error::<T>::NotNominationPhase
            );

            // Check not already candidate
            ensure!(
                !Candidates::<T>::contains_key(&who),
                Error::<T>::AlreadyCandidate
            );

            // Verify role and stake
            let role = T::StakingInterface::get_role(&who)
                .ok_or(Error::<T>::NotDirectorRole)?;
            ensure!(
                role == Role::DecentralizedDirector,
                Error::<T>::NotDirectorRole
            );

            let stake = T::StakingInterface::get_stake(&who)
                .ok_or(Error::<T>::InsufficientStake)?;
            ensure!(
                stake >= T::MinCandidateStake::get(),
                Error::<T>::InsufficientStake
            );

            // Validate manifesto
            let bounded_manifesto: BoundedVec<u8, ConstU32<1000>> =
                manifesto.try_into().map_err(|_| Error::<T>::ManifestoTooLong)?;

            // Create candidate profile
            let current_block = <frame_system::Pallet<T>>::block_number();
            let profile = CandidateProfile {
                account: who.clone(),
                stake,
                manifesto: bounded_manifesto,
                votes_received: 0,
                registered_at: Self::block_number_to_u64(current_block),
            };

            Candidates::<T>::insert(&who, profile);

            Self::deposit_event(Event::CandidateRegistered(who, stake));

            Ok(())
        }

        /// Withdraw candidacy
        ///
        /// Removes candidate from election. Any votes for this candidate become invalid.
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn withdraw_candidacy(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check is candidate
            ensure!(
                Candidates::<T>::contains_key(&who),
                Error::<T>::NotCandidate
            );

            // Remove candidate
            Candidates::<T>::remove(&who);

            // Note: Votes for this candidate remain but will be ignored during tallying

            Self::deposit_event(Event::CandidateWithdrew(who));

            Ok(())
        }

        /// Vote for a candidate
        ///
        /// Requirements:
        /// - Must be in Voting phase
        /// - Must have voting power (staked with role)
        /// - Cannot have already voted
        /// - Candidate must be registered
        ///
        /// # Arguments
        /// * `candidate` - Account to vote for
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn vote(origin: OriginFor<T>, candidate: T::AccountId) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Check phase
            let phase = ElectionPhase::<T>::get();
            ensure!(
                matches!(phase, ElectionPhaseInfo::Voting { .. }),
                Error::<T>::NotVotingPhase
            );

            // Check not already voted
            ensure!(!Votes::<T>::contains_key(&voter), Error::<T>::AlreadyVoted);

            // Verify candidate exists
            let mut candidate_profile = Candidates::<T>::get(&candidate)
                .ok_or(Error::<T>::InvalidCandidate)?;

            // Calculate voting power
            let voting_power = Self::calculate_voting_power(&voter)?;
            ensure!(voting_power > 0, Error::<T>::CannotVote);

            // Record vote
            let vote_record = VoteRecord {
                candidate: candidate.clone(),
                voting_power,
            };
            Votes::<T>::insert(&voter, vote_record);

            // Update candidate vote count
            candidate_profile.votes_received =
                candidate_profile.votes_received.saturating_add(voting_power);
            Candidates::<T>::insert(&candidate, candidate_profile);

            Self::deposit_event(Event::VoteCast(voter, candidate, voting_power));

            Ok(())
        }

        /// Change vote to different candidate
        ///
        /// Allows voter to change their vote before voting period ends
        ///
        /// # Arguments
        /// * `new_candidate` - New candidate to vote for
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn change_vote(
            origin: OriginFor<T>,
            new_candidate: T::AccountId,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Check phase
            let phase = ElectionPhase::<T>::get();
            ensure!(
                matches!(phase, ElectionPhaseInfo::Voting { .. }),
                Error::<T>::NotVotingPhase
            );

            // Get existing vote
            let old_vote = Votes::<T>::get(&voter).ok_or(Error::<T>::NoVoteToChange)?;

            // Verify new candidate exists
            let mut new_candidate_profile = Candidates::<T>::get(&new_candidate)
                .ok_or(Error::<T>::InvalidCandidate)?;

            // Remove votes from old candidate
            if let Some(mut old_candidate_profile) = Candidates::<T>::get(&old_vote.candidate) {
                old_candidate_profile.votes_received =
                    old_candidate_profile.votes_received.saturating_sub(old_vote.voting_power);
                Candidates::<T>::insert(&old_vote.candidate, old_candidate_profile);
            }

            // Add votes to new candidate
            new_candidate_profile.votes_received =
                new_candidate_profile.votes_received.saturating_add(old_vote.voting_power);
            Candidates::<T>::insert(&new_candidate, new_candidate_profile);

            // Update vote record
            let new_vote = VoteRecord {
                candidate: new_candidate.clone(),
                voting_power: old_vote.voting_power,
            };
            Votes::<T>::insert(&voter, new_vote);

            Self::deposit_event(Event::VoteChanged(
                voter,
                old_vote.candidate,
                new_candidate,
            ));

            Ok(())
        }

        /// Remove vote
        ///
        /// Removes vote entirely (voter can vote again later in same period)
        #[pallet::call_index(4)]
        #[pallet::weight(5_000)]
        pub fn remove_vote(origin: OriginFor<T>) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Check phase
            let phase = ElectionPhase::<T>::get();
            ensure!(
                matches!(phase, ElectionPhaseInfo::Voting { .. }),
                Error::<T>::NotVotingPhase
            );

            // Get vote
            let vote = Votes::<T>::get(&voter).ok_or(Error::<T>::NoVoteToChange)?;

            // Remove votes from candidate
            if let Some(mut candidate_profile) = Candidates::<T>::get(&vote.candidate) {
                candidate_profile.votes_received =
                    candidate_profile.votes_received.saturating_sub(vote.voting_power);
                Candidates::<T>::insert(&vote.candidate, candidate_profile);
            }

            // Remove vote record
            Votes::<T>::remove(&voter);

            Self::deposit_event(Event::VoteRemoved(voter, vote.candidate));

            Ok(())
        }

        /// Trigger election start (governance only)
        ///
        /// Manually starts the election cycle. Useful for initial setup or testing.
        #[pallet::call_index(5)]
        #[pallet::weight(50_000)]
        pub fn trigger_election(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let current_block = <frame_system::Pallet<T>>::block_number();

            // Calculate phase boundaries
            let nomination_start = current_block;
            let nomination_end =
                nomination_start.saturating_add(T::NominationPeriodBlocks::get());
            let voting_end = nomination_end.saturating_add(T::VotingPeriodBlocks::get());

            // Set to Nomination phase
            let phase = ElectionPhaseInfo::Nomination {
                start: nomination_start,
                end: nomination_end,
            };

            ElectionPhase::<T>::put(phase);
            NextConsensusDayBlock::<T>::put(voting_end);

            Self::deposit_event(Event::ElectionPhaseChanged(0, 1, current_block));

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Automatic phase transitions
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            let phase = ElectionPhase::<T>::get();

            match phase {
                ElectionPhaseInfo::Governance {
                    next_nomination_start,
                } => {
                    if n >= next_nomination_start {
                        // Transition to Nomination
                        let nomination_end =
                            n.saturating_add(T::NominationPeriodBlocks::get());
                        let new_phase = ElectionPhaseInfo::Nomination {
                            start: n,
                            end: nomination_end,
                        };
                        ElectionPhase::<T>::put(new_phase);

                        // Calculate Consensus Day
                        let consensus_day =
                            nomination_end.saturating_add(T::VotingPeriodBlocks::get());
                        NextConsensusDayBlock::<T>::put(consensus_day);

                        Self::deposit_event(Event::ElectionPhaseChanged(0, 1, n));
                    }
                }
                ElectionPhaseInfo::Nomination { end, .. } => {
                    if n >= end {
                        // Transition to Voting
                        let voting_end = n.saturating_add(T::VotingPeriodBlocks::get());
                        let new_phase = ElectionPhaseInfo::Voting {
                            start: n,
                            end: voting_end,
                        };
                        ElectionPhase::<T>::put(new_phase);

                        Self::deposit_event(Event::ElectionPhaseChanged(1, 2, n));
                    }
                }
                ElectionPhaseInfo::Voting { end, .. } => {
                    if n >= end {
                        // Consensus Day! Tally votes and seat directors
                        let _ = Self::tally_and_seat_directors();

                        // Transition to Governance
                        let next_nomination =
                            n.saturating_add(T::GovernancePeriodBlocks::get());
                        let new_phase = ElectionPhaseInfo::Governance {
                            next_nomination_start: next_nomination,
                        };
                        ElectionPhase::<T>::put(new_phase);

                        // Increment epoch
                        CurrentEpoch::<T>::mutate(|e| *e = e.saturating_add(1));

                        Self::deposit_event(Event::ElectionPhaseChanged(2, 0, n));
                    }
                }
            }

            Weight::from_parts(10_000, 0)
        }
    }

    impl<T: Config> Pallet<T> {
        /// Calculate voting power for an account
        ///
        /// Formula: stake × role_multiplier
        /// - DecentralizedDirector: 3x
        /// - ValidityNode/FlareNode: 2x
        /// - CommonStakePeer: 1x
        /// - CommonPeer: 0x (cannot vote)
        pub fn calculate_voting_power(account: &T::AccountId) -> Result<u128, Error<T>> {
            let stake = T::StakingInterface::get_stake(account)
                .ok_or(Error::<T>::CannotVote)?;

            let role = T::StakingInterface::get_role(account);

            let multiplier = match role {
                Some(Role::DecentralizedDirector) => 3,
                Some(Role::ValidityNode) | Some(Role::FlareNode) => 2,
                Some(Role::CommonStakePeer) => 1,
                _ => 0, // CommonPeer cannot vote
            };

            if multiplier == 0 {
                return Err(Error::<T>::CannotVote);
            }

            Ok(stake.saturating_mul(multiplier as u128))
        }

        /// Tally votes and seat top 9 directors
        fn tally_and_seat_directors() -> DispatchResult {
            let epoch = CurrentEpoch::<T>::get();

            // Collect all candidates with their vote totals
            let mut candidates: Vec<_> = Candidates::<T>::iter()
                .map(|(_, profile)| profile)
                .collect();

            // Sort by votes (descending), then by stake, then by account hash (deterministic)
            candidates.sort_by(|a, b| {
                b.votes_received
                    .cmp(&a.votes_received)
                    .then_with(|| b.stake.cmp(&a.stake))
                    .then_with(|| {
                        // Deterministic tiebreaker: hash of account
                        let hash_a = T::Hashing::hash_of(&a.account);
                        let hash_b = T::Hashing::hash_of(&b.account);
                        hash_b.cmp(&hash_a)
                    })
            });

            // Take top 9 (or all if less than 9)
            let num_to_elect = T::NumDirectorsToElect::get() as usize;
            let winners: Vec<T::AccountId> = candidates
                .into_iter()
                .take(num_to_elect)
                .map(|c| c.account)
                .collect();

            // Calculate total votes
            let total_votes: u128 = Votes::<T>::iter()
                .map(|(_, vote)| vote.voting_power)
                .sum();

            // Store election results
            let result = ElectionResult {
                winners: winners
                    .clone()
                    .try_into()
                    .unwrap_or_else(|_| BoundedVec::default()),
                epoch,
                total_votes,
                candidate_count: Candidates::<T>::iter().count() as u32,
            };
            ElectionResults::<T>::insert(epoch, result);

            // Seat directors
            let bounded_winners: BoundedVec<T::AccountId, ConstU32<9>> = winners
                .clone()
                .try_into()
                .unwrap_or_else(|_| BoundedVec::default());
            ElectedDirectors::<T>::put(bounded_winners);

            // Emit events
            Self::deposit_event(Event::ElectionCompleted {
                epoch,
                winners: winners.clone(),
                total_votes,
            });
            Self::deposit_event(Event::DirectorsSeated(winners, epoch));

            // Clear candidate and vote storage for next election
            let _ = Candidates::<T>::clear(u32::MAX, None);
            let _ = Votes::<T>::clear(u32::MAX, None);

            Ok(())
        }

        /// Helper: Convert BlockNumber to u64
        fn block_number_to_u64(block: BlockNumberFor<T>) -> u64 {
            use sp_runtime::traits::UniqueSaturatedInto;
            block.unique_saturated_into()
        }

        /// Get current elected directors
        pub fn get_elected_directors() -> Vec<T::AccountId> {
            ElectedDirectors::<T>::get().into_inner()
        }

        /// Check if account is currently an elected director
        pub fn is_elected_director(account: &T::AccountId) -> bool {
            ElectedDirectors::<T>::get().contains(account)
        }
    }
}
