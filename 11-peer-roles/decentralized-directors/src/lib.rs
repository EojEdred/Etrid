//! # Ëtrid Decentralized Directors Pallet
//!
//! ## Overview
//!
//! The Decentralized Directors pallet manages governance validators with 128+ ËTR stake
//! who are elected annually on Consensus Day to oversee the Foundation DAO. Directors
//! have emergency governance powers and strategic oversight responsibilities.
//!
//! ## Features
//!
//! - Director registration and lifecycle management (365-day terms)
//! - Emergency governance actions with 2/3 director approval (6 out of 9)
//! - Fast-track proposals (24h voting instead of 7 days)
//! - Emergency network halt capability
//! - Runtime parameter changes
//! - Treasury spend approval
//! - Validator emergency slashing
//! - Time-locked execution with veto mechanism
//!
//! ## Director Powers (Emergency Actions)
//!
//! Directors can propose and vote on emergency actions:
//! - **FastTrack**: Accelerate proposal voting (7 days → 24 hours)
//! - **EmergencyHalt**: Stop network operations immediately
//! - **ParameterChange**: Modify runtime parameters
//! - **TreasurySpend**: Approve emergency treasury allocations
//! - **ValidatorSlash**: Emergency slash misbehaving validators
//!
//! All emergency actions require **2/3 director approval** (6/9 directors).
//!
//! ## Extrinsics
//!
//! - `register_director()` - Register as director candidate (requires 128+ ËTR stake)
//! - `end_term()` - End director term (automatic after 365 days)
//! - `propose_emergency_action()` - Propose emergency action (directors only)
//! - `vote_emergency_action()` - Vote on emergency proposal (directors only)
//! - `execute_emergency_action()` - Execute approved action (after timelock)
//! - `veto_emergency_action()` - Veto emergency action (governance)
//!
//! ## Storage
//!
//! - `Directors` - Active director profiles
//! - `DirectorTerms` - Directors by term/epoch
//! - `EmergencyActions` - Pending emergency proposals
//! - `DirectorVotes` - Director votes on proposals
//! - `NextProposalId` - Proposal ID counter
//!
//! ## Events
//!
//! - `DirectorRegistered` - Director registered for term
//! - `DirectorTermEnded` - Director term ended
//! - `EmergencyActionProposed` - Emergency action proposed
//! - `EmergencyActionVoted` - Director voted on action
//! - `EmergencyActionExecuted` - Action executed
//! - `EmergencyActionVetoed` - Action vetoed by governance
//!
//! ## Example Usage
//!
//! ```ignore
//! // Register as director (must have DecentralizedDirector role)
//! DecentralizedDirectors::register_director(Origin::signed(alice))?;
//!
//! // Propose emergency action
//! let action = EmergencyAction::FastTrack { proposal_id: 42 };
//! DecentralizedDirectors::propose_emergency_action(
//!     Origin::signed(alice),
//!     action,
//!     b"Critical security patch needed".to_vec(),
//! )?;
//!
//! // Vote on action (13 more directors must vote yes)
//! DecentralizedDirectors::vote_emergency_action(
//!     Origin::signed(bob),
//!     1, // proposal_id
//!     true, // approve
//! )?;
//!
//! // Execute after 2/3 approval + timelock
//! DecentralizedDirectors::execute_emergency_action(
//!     Origin::signed(anyone),
//!     1, // proposal_id
//! )?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Get, UnixTime},
};
use frame_system::pallet_prelude::*;
use peer_roles_staking_types::{Role, RoleInterface};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::Saturating,
    RuntimeDebug,
};
use sp_std::vec::Vec;

/// Maximum number of directors (9 for Ëtrid governance)
pub const MAX_DIRECTORS: u32 = 9;

/// Directors required for quorum (2/3 of 9 = 6)
pub const QUORUM_DIRECTORS: u32 = 6;

/// Director term length in seconds (365 days)
pub const DIRECTOR_TERM_SECONDS: u64 = 365 * 24 * 60 * 60;

/// Emergency action timelock in blocks (24 hours at 6s blocks = 14,400 blocks)
pub const EMERGENCY_TIMELOCK_BLOCKS: u32 = 14_400;

/// Emergency action types
#[derive(Clone, Encode, Decode, DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum EmergencyAction {
    /// Fast-track a governance proposal (24h voting)
    FastTrack { proposal_id: u32 },

    /// Emergency halt network operations
    EmergencyHalt,

    /// Change runtime parameter
    ParameterChange {
        param_name: BoundedVec<u8, ConstU32<64>>,
        param_value: u64,
    },

    /// Approve emergency treasury spending
    TreasurySpend {
        recipient: BoundedVec<u8, ConstU32<64>>, // AccountId as bytes
        amount: u128,
    },

    /// Emergency slash validator
    ValidatorSlash {
        validator: BoundedVec<u8, ConstU32<64>>, // AccountId as bytes
        amount: u128,
        reason: BoundedVec<u8, ConstU32<256>>,
    },
}

/// Director profile
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(AccountId))]
pub struct DirectorProfile<AccountId: MaxEncodedLen> {
    /// Director account
    pub account: AccountId,
    /// Term start timestamp
    pub term_start: u64,
    /// Term end timestamp
    pub term_end: u64,
    /// Whether term is currently active
    pub term_active: bool,
    /// Epoch/term number
    pub epoch: u32,
}

/// Emergency action proposal
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(AccountId, BlockNumber))]
pub struct EmergencyProposal<AccountId: MaxEncodedLen, BlockNumber: MaxEncodedLen> {
    /// Unique proposal ID
    pub id: u32,
    /// Proposer (must be director)
    pub proposer: AccountId,
    /// Action to execute
    pub action: EmergencyAction,
    /// Proposal rationale
    pub rationale: BoundedVec<u8, ConstU32<512>>,
    /// Number of yes votes
    pub votes_yes: u32,
    /// Number of no votes
    pub votes_no: u32,
    /// Block when proposal was created
    pub created_at: BlockNumber,
    /// Block when action can be executed (after timelock)
    pub executable_at: BlockNumber,
    /// Whether proposal has been executed
    pub executed: bool,
    /// Whether proposal was vetoed
    pub vetoed: bool,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Runtime event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Staking pallet interface for role verification
        type StakingInterface: RoleInterface<Self::AccountId, u128>;

        /// Unix time provider for term management
        type UnixTime: UnixTime;

        /// Maximum number of emergency proposals
        #[pallet::constant]
        type MaxEmergencyProposals: Get<u32>;

        /// Emergency action timelock (blocks)
        #[pallet::constant]
        type EmergencyTimelock: Get<u32>;

        /// Minimum directors for quorum
        #[pallet::constant]
        type QuorumDirectors: Get<u32>;
    }

    /// Active directors by account
    #[pallet::storage]
    #[pallet::getter(fn directors)]
    pub type Directors<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        DirectorProfile<T::AccountId>,
    >;

    /// Directors by term/epoch
    #[pallet::storage]
    #[pallet::getter(fn director_terms)]
    pub type DirectorTerms<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // epoch
        BoundedVec<T::AccountId, ConstU32<MAX_DIRECTORS>>,
        ValueQuery,
    >;

    /// Emergency action proposals
    #[pallet::storage]
    #[pallet::getter(fn emergency_actions)]
    pub type EmergencyActions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // proposal_id
        EmergencyProposal<T::AccountId, BlockNumberFor<T>>,
    >;

    /// Director votes on emergency actions (proposal_id, account) -> vote
    #[pallet::storage]
    #[pallet::getter(fn director_votes)]
    pub type DirectorVotes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // proposal_id
        Blake2_128Concat,
        T::AccountId,
        bool, // true = yes, false = no
    >;

    /// Next proposal ID
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Current epoch number
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Director registered [account, term_start, term_end, epoch]
        DirectorRegistered {
            account: T::AccountId,
            term_start: u64,
            term_end: u64,
            epoch: u32,
        },
        /// Director term ended [account, epoch]
        DirectorTermEnded {
            account: T::AccountId,
            epoch: u32,
        },
        /// Emergency action proposed [proposal_id, proposer, action]
        EmergencyActionProposed {
            proposal_id: u32,
            proposer: T::AccountId,
            action: EmergencyAction,
        },
        /// Director voted on emergency action [proposal_id, director, vote]
        EmergencyActionVoted {
            proposal_id: u32,
            director: T::AccountId,
            vote: bool,
        },
        /// Emergency action executed [proposal_id, action]
        EmergencyActionExecuted {
            proposal_id: u32,
            action: EmergencyAction,
        },
        /// Emergency action vetoed [proposal_id]
        EmergencyActionVetoed {
            proposal_id: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not a decentralized director
        NotDirector,
        /// Insufficient stake for director role
        InsufficientStake,
        /// Director already registered
        AlreadyRegistered,
        /// Director not found
        DirectorNotFound,
        /// Term already ended
        TermAlreadyEnded,
        /// Term not yet ended
        TermNotEnded,
        /// Emergency action not found
        EmergencyActionNotFound,
        /// Already voted on this proposal
        AlreadyVoted,
        /// Proposal not approved (needs 2/3 quorum)
        NotApproved,
        /// Timelock not yet passed
        TimelockNotPassed,
        /// Proposal already executed
        AlreadyExecuted,
        /// Proposal was vetoed
        ProposalVetoed,
        /// Too many emergency proposals
        TooManyProposals,
        /// Invalid proposal ID
        InvalidProposalId,
        /// Maximum directors reached
        MaxDirectorsReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as director candidate
        ///
        /// Requires:
        /// - Caller has DecentralizedDirector role
        /// - Caller has 128+ ËTR staked
        /// - Not already registered for current term
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_director(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify director role and stake
            Self::ensure_director_eligible(&who)?;

            // Check not already registered
            ensure!(
                !Directors::<T>::contains_key(&who),
                Error::<T>::AlreadyRegistered
            );

            // Get current epoch
            let epoch = CurrentEpoch::<T>::get();

            // Create director profile
            let current_time = T::UnixTime::now().as_secs();
            let profile = DirectorProfile {
                account: who.clone(),
                term_start: current_time,
                term_end: current_time.saturating_add(DIRECTOR_TERM_SECONDS),
                term_active: true,
                epoch,
            };

            // Store director
            Directors::<T>::insert(&who, profile.clone());

            // Add to term list
            DirectorTerms::<T>::try_mutate(epoch, |directors| {
                directors
                    .try_push(who.clone())
                    .map_err(|_| Error::<T>::MaxDirectorsReached)
            })?;

            Self::deposit_event(Event::DirectorRegistered {
                account: who,
                term_start: profile.term_start,
                term_end: profile.term_end,
                epoch,
            });

            Ok(())
        }

        /// End director term
        ///
        /// Can be called by:
        /// - Director themselves (resignation)
        /// - Anyone after term expires
        /// - Governance (forced removal)
        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn end_term(origin: OriginFor<T>, director: T::AccountId) -> DispatchResult {
            // Get director profile first
            let mut profile = Directors::<T>::get(&director)
                .ok_or(Error::<T>::DirectorNotFound)?;

            ensure!(profile.term_active, Error::<T>::TermAlreadyEnded);

            // Check authorization:
            // 1. Director can end their own term
            // 2. Anyone can end if term expired
            // 3. Root can force-end
            let current_time = T::UnixTime::now().as_secs();
            let term_expired = current_time >= profile.term_end;

            // Try root origin first
            if ensure_root(origin.clone()).is_err() {
                // Not root, must be signed
                let who = ensure_signed(origin)?;
                // Must be the director themselves, or term must be expired
                ensure!(
                    who == director || term_expired,
                    DispatchError::BadOrigin
                );
            }

            // End term
            profile.term_active = false;
            Directors::<T>::insert(&director, profile.clone());

            Self::deposit_event(Event::DirectorTermEnded {
                account: director,
                epoch: profile.epoch,
            });

            Ok(())
        }

        /// Propose emergency action
        ///
        /// Only active directors can propose emergency actions.
        #[pallet::call_index(2)]
        #[pallet::weight(20_000)]
        pub fn propose_emergency_action(
            origin: OriginFor<T>,
            action: EmergencyAction,
            rationale: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify proposer is active director
            Self::ensure_active_director(&who)?;

            // Get next proposal ID
            let proposal_id = NextProposalId::<T>::get();
            let next_id = proposal_id.checked_add(1).ok_or(Error::<T>::InvalidProposalId)?;
            NextProposalId::<T>::put(next_id);

            // Create proposal
            let current_block = <frame_system::Pallet<T>>::block_number();
            let executable_at = current_block.saturating_add(T::EmergencyTimelock::get().into());

            let rationale_bounded = BoundedVec::try_from(rationale)
                .map_err(|_| Error::<T>::TooManyProposals)?;

            let proposal = EmergencyProposal {
                id: proposal_id,
                proposer: who.clone(),
                action: action.clone(),
                rationale: rationale_bounded,
                votes_yes: 1, // Proposer automatically votes yes
                votes_no: 0,
                created_at: current_block,
                executable_at,
                executed: false,
                vetoed: false,
            };

            // Store proposal
            EmergencyActions::<T>::insert(proposal_id, proposal);

            // Record proposer's vote
            DirectorVotes::<T>::insert(proposal_id, &who, true);

            Self::deposit_event(Event::EmergencyActionProposed {
                proposal_id,
                proposer: who,
                action,
            });

            Ok(())
        }

        /// Vote on emergency action
        ///
        /// Only active directors can vote.
        /// Each director can vote once per proposal.
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn vote_emergency_action(
            origin: OriginFor<T>,
            proposal_id: u32,
            approve: bool,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify voter is active director
            Self::ensure_active_director(&who)?;

            // Check proposal exists
            let mut proposal = EmergencyActions::<T>::get(proposal_id)
                .ok_or(Error::<T>::EmergencyActionNotFound)?;

            // Check not already voted
            ensure!(
                !DirectorVotes::<T>::contains_key(proposal_id, &who),
                Error::<T>::AlreadyVoted
            );

            // Check not already executed or vetoed
            ensure!(!proposal.executed, Error::<T>::AlreadyExecuted);
            ensure!(!proposal.vetoed, Error::<T>::ProposalVetoed);

            // Record vote
            DirectorVotes::<T>::insert(proposal_id, &who, approve);

            // Update vote counts
            if approve {
                proposal.votes_yes = proposal.votes_yes.saturating_add(1);
            } else {
                proposal.votes_no = proposal.votes_no.saturating_add(1);
            }

            EmergencyActions::<T>::insert(proposal_id, proposal);

            Self::deposit_event(Event::EmergencyActionVoted {
                proposal_id,
                director: who,
                vote: approve,
            });

            Ok(())
        }

        /// Execute emergency action
        ///
        /// Requirements:
        /// - 2/3 director approval (6 out of 9)
        /// - Timelock period passed (24 hours)
        /// - Not yet executed
        /// - Not vetoed
        #[pallet::call_index(4)]
        #[pallet::weight(50_000)]
        pub fn execute_emergency_action(
            origin: OriginFor<T>,
            proposal_id: u32,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Get proposal
            let mut proposal = EmergencyActions::<T>::get(proposal_id)
                .ok_or(Error::<T>::EmergencyActionNotFound)?;

            // Verify not executed or vetoed
            ensure!(!proposal.executed, Error::<T>::AlreadyExecuted);
            ensure!(!proposal.vetoed, Error::<T>::ProposalVetoed);

            // Verify quorum (2/3 approval)
            ensure!(
                proposal.votes_yes >= T::QuorumDirectors::get(),
                Error::<T>::NotApproved
            );

            // Verify timelock passed
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                current_block >= proposal.executable_at,
                Error::<T>::TimelockNotPassed
            );

            // Execute action
            Self::execute_action(&proposal.action)?;

            // Mark as executed
            proposal.executed = true;
            EmergencyActions::<T>::insert(proposal_id, proposal.clone());

            Self::deposit_event(Event::EmergencyActionExecuted {
                proposal_id,
                action: proposal.action,
            });

            Ok(())
        }

        /// Veto emergency action (governance only)
        ///
        /// Allows governance to veto emergency actions during timelock period.
        #[pallet::call_index(5)]
        #[pallet::weight(20_000)]
        pub fn veto_emergency_action(
            origin: OriginFor<T>,
            proposal_id: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Get proposal
            let mut proposal = EmergencyActions::<T>::get(proposal_id)
                .ok_or(Error::<T>::EmergencyActionNotFound)?;

            // Verify not already executed
            ensure!(!proposal.executed, Error::<T>::AlreadyExecuted);

            // Mark as vetoed
            proposal.vetoed = true;
            EmergencyActions::<T>::insert(proposal_id, proposal);

            Self::deposit_event(Event::EmergencyActionVetoed { proposal_id });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Verify account is eligible to be a director
        fn ensure_director_eligible(who: &T::AccountId) -> DispatchResult {
            // Check has DecentralizedDirector role
            let role = T::StakingInterface::get_role(who)
                .ok_or(Error::<T>::NotDirector)?;

            ensure!(
                role == Role::DecentralizedDirector,
                Error::<T>::NotDirector
            );

            // Check has minimum stake (128 ËTR)
            let stake = T::StakingInterface::get_stake(who)
                .ok_or(Error::<T>::InsufficientStake)?;

            let min_stake = 128_000_000_000_000_000_000_000u128; // 128 ËTR with 18 decimals
            ensure!(stake >= min_stake, Error::<T>::InsufficientStake);

            Ok(())
        }

        /// Verify account is an active director
        fn ensure_active_director(who: &T::AccountId) -> DispatchResult {
            let profile = Directors::<T>::get(who)
                .ok_or(Error::<T>::NotDirector)?;

            ensure!(profile.term_active, Error::<T>::TermNotEnded);

            // Check term not expired
            let current_time = T::UnixTime::now().as_secs();
            ensure!(current_time < profile.term_end, Error::<T>::TermNotEnded);

            Ok(())
        }

        /// Execute emergency action
        fn execute_action(action: &EmergencyAction) -> DispatchResult {
            match action {
                EmergencyAction::FastTrack { proposal_id } => {
                    // TODO: Integrate with governance pallet to fast-track proposal
                    log::info!("Fast-tracking proposal {}", proposal_id);
                    // This will be implemented when governance pallet integration is complete
                    Ok(())
                }
                EmergencyAction::EmergencyHalt => {
                    // TODO: Implement network halt mechanism
                    log::warn!("EMERGENCY HALT ACTIVATED");
                    // This could pause block production, freeze assets, etc.
                    Ok(())
                }
                EmergencyAction::ParameterChange { param_name, param_value } => {
                    // TODO: Integrate with runtime parameter storage
                    log::info!(
                        "Parameter change: {:?} = {}",
                        param_name,
                        param_value
                    );
                    Ok(())
                }
                EmergencyAction::TreasurySpend { recipient, amount } => {
                    // TODO: Integrate with treasury pallet
                    log::info!(
                        "Treasury spend: {:?} to receive {}",
                        recipient,
                        amount
                    );
                    Ok(())
                }
                EmergencyAction::ValidatorSlash { validator, amount, reason } => {
                    // TODO: Integrate with staking pallet for slashing
                    log::warn!(
                        "Emergency slash: {:?} for {} reason: {:?}",
                        validator,
                        amount,
                        reason
                    );
                    Ok(())
                }
            }
        }

        /// Get number of active directors
        pub fn active_director_count() -> u32 {
            let epoch = CurrentEpoch::<T>::get();
            DirectorTerms::<T>::get(epoch).len() as u32
        }

        /// Check if account is an active director
        pub fn is_active_director(who: &T::AccountId) -> bool {
            if let Some(profile) = Directors::<T>::get(who) {
                if !profile.term_active {
                    return false;
                }
                let current_time = T::UnixTime::now().as_secs();
                current_time < profile.term_end
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_decentralized_directors;
    use frame_support::{
        assert_noop, assert_ok, parameter_types,
        traits::{ConstU32, UnixTime},
    };
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    // Mock staking interface
    pub struct MockStaking;
    impl RoleInterface<u64, u128> for MockStaking {
        fn get_role(account: &u64) -> Option<Role> {
            // Accounts 1-9 are directors, others are not
            if *account >= 1 && *account <= 9 {
                Some(Role::DecentralizedDirector)
            } else {
                None
            }
        }

        fn get_stake(account: &u64) -> Option<u128> {
            // Directors have 128+ ËTR
            if *account >= 1 && *account <= 9 {
                Some(128_000_000_000_000_000_000_000u128)
            } else {
                None
            }
        }

        fn stake_requirement(_role: &Role) -> peer_roles_staking_types::StakeRequirement {
            peer_roles_staking_types::StakeRequirement::Director
        }
    }

    // Mock time provider
    pub struct MockTime;
    parameter_types! {
        pub static MockCurrentTime: u64 = 0;
    }

    impl UnixTime for MockTime {
        fn now() -> core::time::Duration {
            core::time::Duration::from_secs(MockCurrentTime::get())
        }
    }

    // Configure mock runtime
    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Directors: pallet_decentralized_directors,
        }
    );

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
        type BlockHashCount = frame_support::traits::ConstU64<250>;
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
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
        type ExtensionsWeightInfo = ();
    }

    parameter_types! {
        pub const MaxEmergencyProposals: u32 = 100;
        pub const EmergencyTimelock: u32 = 14_400; // 24h
        pub const QuorumDirectors: u32 = 6; // 2/3 of 9
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type StakingInterface = MockStaking;
        type UnixTime = MockTime;
        type MaxEmergencyProposals = MaxEmergencyProposals;
        type EmergencyTimelock = EmergencyTimelock;
        type QuorumDirectors = QuorumDirectors;
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| {
            System::set_block_number(1);
            MockCurrentTime::set(1000);
        });
        ext
    }

    #[test]
    fn test_register_director() {
        new_test_ext().execute_with(|| {
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));

            let profile = Directors::directors(1).unwrap();
            assert_eq!(profile.account, 1);
            assert_eq!(profile.term_active, true);
            assert_eq!(profile.term_start, 1000);
        });
    }

    #[test]
    fn test_register_director_fails_without_role() {
        new_test_ext().execute_with(|| {
            // Account 100 doesn't have director role
            assert_noop!(
                Directors::register_director(RuntimeOrigin::signed(100)),
                Error::<Test>::NotDirector
            );
        });
    }

    #[test]
    fn test_propose_emergency_action() {
        new_test_ext().execute_with(|| {
            // Register director
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));

            // Propose action
            let action = EmergencyAction::FastTrack { proposal_id: 42 };
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test rationale".to_vec()
            ));

            let proposal = Directors::emergency_actions(0).unwrap();
            assert_eq!(proposal.proposer, 1);
            assert_eq!(proposal.votes_yes, 1); // Proposer auto-votes yes
        });
    }

    #[test]
    fn test_vote_emergency_action() {
        new_test_ext().execute_with(|| {
            // Register directors
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(2)));

            // Propose action
            let action = EmergencyAction::EmergencyHalt;
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test".to_vec()
            ));

            // Vote
            assert_ok!(Directors::vote_emergency_action(
                RuntimeOrigin::signed(2),
                0,
                true
            ));

            let proposal = Directors::emergency_actions(0).unwrap();
            assert_eq!(proposal.votes_yes, 2);
        });
    }

    #[test]
    fn test_cannot_vote_twice() {
        new_test_ext().execute_with(|| {
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));

            let action = EmergencyAction::EmergencyHalt;
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test".to_vec()
            ));

            // Try to vote again
            assert_noop!(
                Directors::vote_emergency_action(RuntimeOrigin::signed(1), 0, true),
                Error::<Test>::AlreadyVoted
            );
        });
    }

    #[test]
    fn test_execute_emergency_action_requires_quorum() {
        new_test_ext().execute_with(|| {
            // Register and propose
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));
            let action = EmergencyAction::EmergencyHalt;
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test".to_vec()
            ));

            // Advance past timelock
            System::set_block_number(14_401);

            // Try to execute with only 1 vote (need 6)
            assert_noop!(
                Directors::execute_emergency_action(RuntimeOrigin::signed(1), 0),
                Error::<Test>::NotApproved
            );
        });
    }

    #[test]
    fn test_execute_emergency_action_requires_timelock() {
        new_test_ext().execute_with(|| {
            // Register 6 directors
            for i in 1..=6 {
                assert_ok!(Directors::register_director(RuntimeOrigin::signed(i)));
            }

            // Propose
            let action = EmergencyAction::EmergencyHalt;
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test".to_vec()
            ));

            // Vote with all 6 directors
            for i in 2..=6 {
                assert_ok!(Directors::vote_emergency_action(
                    RuntimeOrigin::signed(i),
                    0,
                    true
                ));
            }

            // Try to execute before timelock
            assert_noop!(
                Directors::execute_emergency_action(RuntimeOrigin::signed(1), 0),
                Error::<Test>::TimelockNotPassed
            );

            // Advance past timelock
            System::set_block_number(14_401);

            // Now should succeed
            assert_ok!(Directors::execute_emergency_action(RuntimeOrigin::signed(1), 0));
        });
    }

    #[test]
    fn test_veto_emergency_action() {
        new_test_ext().execute_with(|| {
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));

            let action = EmergencyAction::EmergencyHalt;
            assert_ok!(Directors::propose_emergency_action(
                RuntimeOrigin::signed(1),
                action,
                b"Test".to_vec()
            ));

            // Governance vetos
            assert_ok!(Directors::veto_emergency_action(RuntimeOrigin::root(), 0));

            let proposal = Directors::emergency_actions(0).unwrap();
            assert_eq!(proposal.vetoed, true);
        });
    }

    #[test]
    fn test_end_term() {
        new_test_ext().execute_with(|| {
            assert_ok!(Directors::register_director(RuntimeOrigin::signed(1)));

            // Advance time past term end
            MockCurrentTime::set(1000 + DIRECTOR_TERM_SECONDS + 1);

            // End term
            assert_ok!(Directors::end_term(RuntimeOrigin::signed(1), 1));

            let profile = Directors::directors(1).unwrap();
            assert_eq!(profile.term_active, false);
        });
    }
}
