//! # Pallet ASF (Ascending Scale of Finality)
//!
//! This pallet integrates the ASF consensus algorithm into the FlareChain runtime.
//! It provides runtime storage, callable functions, and hooks for managing:
//!
//! - HotStuff 4-phase Byzantine consensus (Prepare → PreCommit → Commit → Decide)
//! - Validity certificate generation and aggregation
//! - Ascending scale of finality (5 levels: 0-4)
//! - Validator slashing and Byzantine detection
//! - Validator rotation and committee management
//!
//! ## Overview
//!
//! The ASF pallet wraps the `asf-algorithm` crate to provide:
//! - Storage for votes, certificates, and finality levels
//! - Extrinsic functions for submitting votes and certificates
//! - Automatic slashing enforcement for Byzantine validators
//! - Event emissions for consensus state changes
//! - Runtime API integration for node services
//!
//! ## Integration
//!
//! This pallet is used by:
//! - FlareChain block production (PPFA proposer selection)
//! - Finality gadget (vote collection and certificate generation)
//! - Validator committee pallet (validator management)
//! - Slashing pallet (penalty enforcement)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use alloc::vec::Vec;
    use codec::{Decode, Encode, MaxEncodedLen};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Get, Currency, ReservableCurrency};
    use frame_support::BoundedVec;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};
    use sp_std::collections::btree_map::BTreeMap;

    // Re-export ASF algorithm types
    pub use asf_algorithm::{
        AsfError, AsfResult, Balance as AsfBalance, BlockNumber as AsfBlockNumber,
        Certificate, ConsensusPhase, FinalityLevel, Hash, ValidatorId, Vote,
        VoteAggregate, bft_threshold, meets_threshold,
        SlashingEvent, SlashingSeverity, SlashingEnforcer,
        Signature, verify_vote_signature,
    };

    /// Type alias for currency balance
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Maximum number of pending certificates per block
    const MAX_PENDING_CERTIFICATES: u32 = 100;

    /// Maximum number of votes per phase
    const MAX_VOTES_PER_PHASE: u32 = 100;

    /// Maximum number of slashed validators to track
    const MAX_SLASHED_VALIDATORS: u32 = 1000;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type for stake and slashing
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Maximum number of validators in the active committee
        #[pallet::constant]
        type MaxValidators: Get<u32>;

        /// Minimum stake required to be a validator
        #[pallet::constant]
        type MinValidatorStake: Get<BalanceOf<Self>>;

        /// Handler for validator slashing (integrates with staking pallet)
        type SlashHandler: SlashValidator<Self::AccountId, BalanceOf<Self>>;

        /// Handler for finality notifications
        type FinalityNotifier: FinalityNotification<Self::BlockNumber>;

        /// Epoch duration in blocks (for validator rotation)
        #[pallet::constant]
        type EpochDuration: Get<Self::BlockNumber>;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // STORAGE ITEMS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Current active validators (ValidatorId => stake weight)
    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ValidatorId,
        AsfBalance,
        OptionQuery,
    >;

    /// Ordered list of current validator IDs (for BFT threshold calculations)
    #[pallet::storage]
    #[pallet::getter(fn validator_set)]
    pub type ValidatorSet<T: Config> = StorageValue<
        _,
        BoundedVec<ValidatorId, T::MaxValidators>,
        ValueQuery,
    >;

    /// Current finality level for the latest block
    #[pallet::storage]
    #[pallet::getter(fn current_finality_level)]
    pub type CurrentFinalityLevel<T: Config> = StorageValue<_, FinalityLevel, ValueQuery>;

    /// Finality level per block hash
    #[pallet::storage]
    #[pallet::getter(fn block_finality)]
    pub type BlockFinality<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Hash,
        FinalityLevel,
        ValueQuery,
    >;

    /// Certificate count per block (for finality level calculation)
    #[pallet::storage]
    #[pallet::getter(fn certificate_count)]
    pub type CertificateCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Hash,
        u32,
        ValueQuery,
    >;

    /// Pending certificates for the current block
    #[pallet::storage]
    #[pallet::getter(fn pending_certificates)]
    pub type PendingCertificates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Hash,
        BoundedVec<Certificate, ConstU32<MAX_PENDING_CERTIFICATES>>,
        ValueQuery,
    >;

    /// Votes collected per phase per block
    /// Key: (block_hash, phase) => Vec<Vote>
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        Hash,
        Blake2_128Concat,
        ConsensusPhase,
        BoundedVec<Vote, ConstU32<MAX_VOTES_PER_PHASE>>,
        ValueQuery,
    >;

    /// Track which validators have voted in a specific phase for a block
    /// Key: (block_hash, phase, validator) => bool
    #[pallet::storage]
    pub type HasVoted<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, Hash>,
            NMapKey<Blake2_128Concat, ConsensusPhase>,
            NMapKey<Blake2_128Concat, ValidatorId>,
        ),
        bool,
        ValueQuery,
    >;

    /// Slashed validators (permanently excluded from consensus)
    #[pallet::storage]
    #[pallet::getter(fn slashed_validators)]
    pub type SlashedValidators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ValidatorId,
        SlashingEvent,
        OptionQuery,
    >;

    /// Current epoch number
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Total stake of all active validators
    #[pallet::storage]
    #[pallet::getter(fn total_stake)]
    pub type TotalStake<T: Config> = StorageValue<_, AsfBalance, ValueQuery>;

    // ═══════════════════════════════════════════════════════════════════════════
    // GENESIS CONFIGURATION
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Initial validators (validator_id, stake)
        pub validators: Vec<(ValidatorId, AsfBalance)>,
        #[serde(skip)]
        pub _phantom: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let mut validator_list: BoundedVec<ValidatorId, T::MaxValidators> = BoundedVec::default();
            let mut total_stake = 0u128;

            for (validator_id, stake) in &self.validators {
                Validators::<T>::insert(validator_id, stake);
                let _ = validator_list.try_push(validator_id.clone());
                total_stake += stake;
            }

            ValidatorSet::<T>::put(validator_list);
            TotalStake::<T>::put(total_stake);
            CurrentEpoch::<T>::put(0);
            CurrentFinalityLevel::<T>::put(FinalityLevel::None);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A vote was submitted by a validator
        VoteSubmitted {
            validator: ValidatorId,
            block_hash: Hash,
            phase: ConsensusPhase,
            epoch: u64,
        },
        /// A validity certificate was generated
        CertificateGenerated {
            block_hash: Hash,
            phase: ConsensusPhase,
            validator: ValidatorId,
            certificate_count: u32,
        },
        /// Finality level changed for a block
        FinalityLevelChanged {
            block_hash: Hash,
            old_level: FinalityLevel,
            new_level: FinalityLevel,
        },
        /// A validator was slashed for Byzantine behavior
        ValidatorSlashed {
            validator: ValidatorId,
            amount: AsfBalance,
            severity: SlashingSeverity,
            reason: alloc::string::String,
        },
        /// A validator was excluded from consensus
        ValidatorExcluded {
            validator: ValidatorId,
            reason: alloc::string::String,
        },
        /// Validator set was rotated for a new epoch
        ValidatorSetRotated {
            epoch: u64,
            validator_count: u32,
        },
        /// Consensus threshold was met for a phase
        ThresholdMet {
            block_hash: Hash,
            phase: ConsensusPhase,
            vote_count: u32,
            total_stake: AsfBalance,
        },
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ERRORS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::error]
    pub enum Error<T> {
        /// Vote is invalid (signature, format, or content)
        InvalidVote,
        /// Certificate is invalid (threshold not met, invalid signatures)
        InvalidCertificate,
        /// Sender is not in the validator set
        NotValidator,
        /// Validator has already voted in this phase for this block
        DuplicateVote,
        /// Validator stake is below minimum threshold
        InsufficientStake,
        /// Invalid phase transition attempted
        InvalidPhaseTransition,
        /// Too many pending certificates for this block
        TooManyCertificates,
        /// Too many votes for this phase
        TooManyVotes,
        /// Validator has been slashed and excluded
        ValidatorSlashed,
        /// Invalid signature on vote
        InvalidSignature,
        /// Consensus phase is invalid
        InvalidPhase,
        /// Block hash not found
        BlockNotFound,
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HOOKS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Called at the beginning of each block
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            // Check if we need to rotate validators (epoch transition)
            let epoch_duration = T::EpochDuration::get();
            let current_block_u64: u64 = block_number.try_into().ok().unwrap_or(0);

            if current_block_u64 > 0 && current_block_u64 % epoch_duration.try_into().ok().unwrap_or(u64::MAX) == 0 {
                Self::rotate_epoch();
            }

            Weight::from_parts(10_000, 0)
        }

        /// Called at the end of each block
        fn on_finalize(_block_number: BlockNumberFor<T>) {
            // Apply any pending slashing events
            Self::apply_pending_slashing();
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CALLABLE FUNCTIONS (EXTRINSICS)
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a vote for a block in a specific consensus phase
        ///
        /// # Parameters
        /// - `origin`: Must be a validator
        /// - `vote`: The vote to submit (includes signature)
        ///
        /// # Errors
        /// - `NotValidator`: If sender is not in the validator set
        /// - `DuplicateVote`: If validator already voted in this phase
        /// - `InvalidVote`: If vote validation fails
        /// - `ValidatorSlashed`: If validator has been slashed
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(50_000, 0))]
        pub fn submit_vote(
            origin: OriginFor<T>,
            vote: Vote,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            // Verify validator is in the active set
            let stake = Validators::<T>::get(&vote.validator)
                .ok_or(Error::<T>::NotValidator)?;

            // Check if validator has been slashed
            ensure!(
                !SlashedValidators::<T>::contains_key(&vote.validator),
                Error::<T>::ValidatorSlashed
            );

            // Check for duplicate votes
            ensure!(
                !HasVoted::<T>::get((&vote.block_hash, &vote.phase, &vote.validator)),
                Error::<T>::DuplicateVote
            );

            // Verify vote signature
            let is_valid = verify_vote_signature(&vote);
            ensure!(is_valid, Error::<T>::InvalidSignature);

            // Verify stake weight matches storage
            ensure!(vote.stake_weight == stake, Error::<T>::InvalidVote);

            // Store the vote
            Votes::<T>::try_mutate(&vote.block_hash, &vote.phase, |votes| {
                votes.try_push(vote.clone())
                    .map_err(|_| Error::<T>::TooManyVotes)
            })?;

            // Mark validator as having voted
            HasVoted::<T>::insert((&vote.block_hash, &vote.phase, &vote.validator), true);

            // Emit event
            Self::deposit_event(Event::VoteSubmitted {
                validator: vote.validator.clone(),
                block_hash: vote.block_hash,
                phase: vote.phase,
                epoch: vote.epoch as u64,
            });

            // Check if we've reached consensus threshold
            Self::check_threshold(&vote.block_hash, &vote.phase)?;

            Ok(())
        }

        /// Submit a validity certificate for a block
        ///
        /// # Parameters
        /// - `origin`: Must be a validator
        /// - `certificate`: The certificate to submit
        ///
        /// # Errors
        /// - `NotValidator`: If sender is not a validator
        /// - `InvalidCertificate`: If certificate validation fails
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(100_000, 0))]
        pub fn submit_certificate(
            origin: OriginFor<T>,
            certificate: Certificate,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            // Verify validator
            ensure!(
                Validators::<T>::contains_key(&certificate.validator),
                Error::<T>::NotValidator
            );

            // Validate certificate
            Self::validate_certificate(&certificate)?;

            // Store certificate
            PendingCertificates::<T>::try_mutate(&certificate.block_hash, |certs| {
                certs.try_push(certificate.clone())
                    .map_err(|_| Error::<T>::TooManyCertificates)
            })?;

            // Increment certificate count
            let new_count = CertificateCount::<T>::mutate(&certificate.block_hash, |count| {
                *count += 1;
                *count
            });

            // Update finality level
            Self::update_finality_level(&certificate.block_hash, new_count)?;

            // Emit event
            Self::deposit_event(Event::CertificateGenerated {
                block_hash: certificate.block_hash,
                phase: certificate.phase,
                validator: certificate.validator.clone(),
                certificate_count: new_count,
            });

            Ok(())
        }

        /// Rotate the validator set (admin/governance only)
        ///
        /// # Parameters
        /// - `origin`: Must be root or governance
        /// - `new_validators`: New validator set with stakes
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(150_000, 0))]
        pub fn rotate_validators(
            origin: OriginFor<T>,
            new_validators: Vec<(ValidatorId, AsfBalance)>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Clear old validators
            let _ = Validators::<T>::clear(u32::MAX, None);

            // Set new validators
            let mut validator_list: BoundedVec<ValidatorId, T::MaxValidators> = BoundedVec::default();
            let mut total = 0u128;

            for (validator_id, stake) in new_validators {
                Validators::<T>::insert(&validator_id, stake);
                let _ = validator_list.try_push(validator_id);
                total += stake;
            }

            ValidatorSet::<T>::put(validator_list.clone());
            TotalStake::<T>::put(total);

            let epoch = CurrentEpoch::<T>::mutate(|e| {
                *e += 1;
                *e
            });

            Self::deposit_event(Event::ValidatorSetRotated {
                epoch,
                validator_count: validator_list.len() as u32,
            });

            Ok(())
        }

        /// Slash a validator for Byzantine behavior (admin/governance only)
        ///
        /// # Parameters
        /// - `origin`: Must be root or automated slashing system
        /// - `validator`: Validator to slash
        /// - `severity`: Severity level of the slash
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(75_000, 0))]
        pub fn slash_validator(
            origin: OriginFor<T>,
            validator: ValidatorId,
            severity: SlashingSeverity,
            reason: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let stake = Validators::<T>::get(&validator)
                .ok_or(Error::<T>::NotValidator)?;

            // Calculate slash amount
            let slash_percentage = severity.slash_percentage();
            let slash_amount = (stake as u128 * slash_percentage as u128) / 10000u128;

            // Create slashing event
            let slashing_event = SlashingEvent {
                validator: validator.clone(),
                severity,
                reason: crate::slashing::SuspicionReason::InvalidSignature, // Default
                amount_slashed: slash_amount,
                epoch: CurrentEpoch::<T>::get() as u32,
                timestamp: Self::get_timestamp(),
                excluded: severity.requires_exclusion(),
            };

            // Store slashing record
            SlashedValidators::<T>::insert(&validator, slashing_event);

            // Remove from active validators if critical
            if severity.requires_exclusion() {
                Validators::<T>::remove(&validator);
                ValidatorSet::<T>::mutate(|set| {
                    set.retain(|v| v != &validator);
                });

                Self::deposit_event(Event::ValidatorExcluded {
                    validator: validator.clone(),
                    reason: alloc::string::String::from_utf8_lossy(&reason).into(),
                });
            }

            // Call slash handler
            T::SlashHandler::slash_validator(&validator, slash_amount)?;

            Self::deposit_event(Event::ValidatorSlashed {
                validator,
                amount: slash_amount,
                severity,
                reason: alloc::string::String::from_utf8_lossy(&reason).into(),
            });

            Ok(())
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // INTERNAL HELPER FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════════════

    impl<T: Config> Pallet<T> {
        /// Check if consensus threshold has been met for a phase
        fn check_threshold(block_hash: &Hash, phase: &ConsensusPhase) -> DispatchResult {
            let votes = Votes::<T>::get(block_hash, phase);
            let total_validators = ValidatorSet::<T>::get().len() as u32;
            let threshold = bft_threshold(total_validators);

            if votes.len() >= threshold as usize {
                let total_stake: AsfBalance = votes.iter().map(|v| v.stake_weight).sum();

                Self::deposit_event(Event::ThresholdMet {
                    block_hash: *block_hash,
                    phase: *phase,
                    vote_count: votes.len() as u32,
                    total_stake,
                });
            }

            Ok(())
        }

        /// Validate a certificate
        fn validate_certificate(cert: &Certificate) -> DispatchResult {
            let total_validators = ValidatorSet::<T>::get().len() as u32;
            let threshold = bft_threshold(total_validators);

            ensure!(
                cert.vote_aggregate.vote_count >= threshold,
                Error::<T>::InvalidCertificate
            );

            Ok(())
        }

        /// Update finality level based on certificate count
        fn update_finality_level(block_hash: &Hash, cert_count: u32) -> DispatchResult {
            let old_level = BlockFinality::<T>::get(block_hash);
            let new_level = FinalityLevel::from(cert_count);

            if new_level != old_level {
                BlockFinality::<T>::insert(block_hash, new_level);
                CurrentFinalityLevel::<T>::put(new_level);

                Self::deposit_event(Event::FinalityLevelChanged {
                    block_hash: *block_hash,
                    old_level,
                    new_level,
                });

                // Notify finality handler
                T::FinalityNotifier::notify_finality(*block_hash, new_level);
            }

            Ok(())
        }

        /// Rotate to new epoch
        fn rotate_epoch() {
            let new_epoch = CurrentEpoch::<T>::mutate(|e| {
                *e += 1;
                *e
            });

            log::info!("ASF: Rotated to epoch {}", new_epoch);
        }

        /// Apply pending slashing
        fn apply_pending_slashing() {
            // Placeholder for automated slashing logic
            // This would integrate with the SlashingEnforcer from asf-algorithm
        }

        /// Get current timestamp in milliseconds
        fn get_timestamp() -> u64 {
            // This would integrate with pallet-timestamp
            0u64
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRAIT DEFINITIONS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Trait for handling validator slashing
    pub trait SlashValidator<AccountId, Balance> {
        fn slash_validator(validator: &ValidatorId, amount: AsfBalance) -> DispatchResult;
    }

    /// Trait for finality notifications
    pub trait FinalityNotification<BlockNumber> {
        fn notify_finality(block_hash: Hash, level: FinalityLevel);
    }

    /// Default implementation for testing
    impl<AccountId, Balance> SlashValidator<AccountId, Balance> for () {
        fn slash_validator(_validator: &ValidatorId, _amount: AsfBalance) -> DispatchResult {
            Ok(())
        }
    }

    /// Default implementation for testing
    impl<BlockNumber> FinalityNotification<BlockNumber> for () {
        fn notify_finality(_block_hash: Hash, _level: FinalityLevel) {
            // No-op for testing
        }
    }
}
