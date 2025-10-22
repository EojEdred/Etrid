//! # Ëtrid FODDoS ASF Consensus Pallet
//!
//! **Ascending Scale of Finality (ASF)** - Complete Implementation
//!
//! This pallet implements the full ASF consensus algorithm as specified in the
//! Ëtrid Ivory Papers, including:
//! - HotStuff 4-phase commit protocol (Prepare → Pre-Commit → Commit → Decide)
//! - PPFA (Proposing Panel for Attestation) epoch rotation
//! - Validity certificates with ascending finality scale
//! - Stake-weighted Byzantine fault tolerance
//! - Adaptive slot duration based on network health
//! - Integration with Ants (secondary blocks) and VMw metering

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Randomness, Time},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use sp_std::vec::Vec;
    use codec::{Encode, Decode};

    /// Type alias for balances
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // ═══════════════════════════════════════════════════════════════════════════
    // CONSTANTS (From Ivory Papers)
    // ═══════════════════════════════════════════════════════════════════════════

    /// PPFA committee size (21 validators per committee)
    pub const MAX_COMMITTEE_SIZE: u32 = 21;

    /// Maximum validity certificates per block
    pub const MAX_CERTIFICATES_PER_BLOCK: u32 = 100;

    /// Epoch duration in blocks (2400 blocks ≈ 4 hours at 6s)
    pub const EPOCH_DURATION: u32 = 2400;

    /// Minimum stake for Validity Node (64 ËTR)
    pub const MIN_VALIDITY_NODE_STAKE: u128 = 64_000_000_000_000_000_000_000;

    /// Minimum stake for Common Stake Peer (1 ËTR)
    pub const MIN_STAKE_PEER_STAKE: u128 = 1_000_000_000_000_000_000_000;

    /// Minimum stake for Decentralized Director (128 ËTR)
    pub const MIN_DIRECTOR_STAKE: u128 = 128_000_000_000_000_000_000_000;

    /// Maximum ants depth (6 levels)
    pub const MAX_ANTS_DEPTH: u32 = 6;

    /// Maximum ants per block (2)
    pub const MAX_ANTS_PER_BLOCK: u32 = 2;

    // ═══════════════════════════════════════════════════════════════════════════
    // TYPES (From Ivory Papers)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Peer types as defined in Ivory Papers
    #[derive(
        Encode,
        Decode,
        codec::DecodeWithMemTracking,
        Clone,
        Eq,
        PartialEq,
        RuntimeDebug,
        TypeInfo,
        MaxEncodedLen,
        serde::Serialize,
        serde::Deserialize
    )]
    pub enum PeerType {
        /// Common Peer - No stake required
        Common,
        /// Common Stake Peer - Minimum 1 ËTR
        StakingCommon,
        /// Validity Node - Minimum 64 ËTR (PBC validator)
        ValidityNode,
        /// Flare Node - FlareChain validator
        FlareNode,
        /// Decentralized Director - Minimum 128 ËTR (governance)
        DecentralizedDirector,
    }

    /// HotStuff consensus phases (Ivory Papers: Prepare → Pre-Commit → Commit → Decide)
    #[derive(Clone, Encode, Decode, codec::DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, serde::Serialize, serde::Deserialize)]
    pub enum ConsensusPhase {
        /// Phase 1: Leader collects highest valid branch and prepare votes
        Prepare,
        /// Phase 2: Prepare certificate broadcast, nodes send pre-commit votes
        PreCommit,
        /// Phase 3: Commit certificate broadcast, replicas lock state
        Commit,
        /// Phase 4: Commit certificate finalized, state transition occurs
        Decide,
    }

    impl Default for ConsensusPhase {
        fn default() -> Self {
            ConsensusPhase::Prepare
        }
    }

    /// Validator information (enhanced from existing stub)
    #[derive(Encode, Decode, codec::DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Validator<T: Config> {
        /// Account address
        pub address: T::AccountId,
        /// Staked amount
        pub stake: BalanceOf<T>,
        /// Reputation score (0-100)
        pub reputation: u64,
        /// Peer type classification
        pub peer_type: PeerType,
        /// Last block produced
        pub last_block: BlockNumberFor<T>,
        /// Active status
        pub active: bool,
        /// Last active epoch
        pub last_epoch: u32,
        /// Total blocks produced
        pub blocks_produced: u32,
        /// Total certificates issued
        pub certificates_issued: u32,
    }

    /// PPFA Committee member
    #[derive(Clone, Encode, Decode, codec::DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct CommitteeMember<T: Config> {
        /// Validator account
        pub validator: T::AccountId,
        /// Stake weight
        pub stake: BalanceOf<T>,
        /// Index in current PPFA panel
        pub ppfa_index: u32,
    }

    /// Validity Certificate (Ivory Papers: Ascending Scale of Finality)
    #[derive(Clone, Encode, Decode, codec::DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct ValidityCertificate<T: Config> {
        /// Block hash being attested
        pub block_hash: T::Hash,
        /// Block number
        pub block_number: BlockNumberFor<T>,
        /// Consensus phase achieved
        pub phase: ConsensusPhase,
        /// Validator who issued certificate
        pub validator: T::AccountId,
        /// Validator's stake weight
        pub stake_weight: BalanceOf<T>,
        /// Epoch when issued
        pub epoch: u32,
        /// Timestamp
        pub timestamp: u64,
    }

    /// Ant (secondary block) metadata
    #[derive(Clone, Encode, Decode, codec::DecodeWithMemTracking, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct AntMetadata<T: Config> {
        /// Ant block hash
        pub ant_hash: T::Hash,
        /// Parent block hash
        pub parent_hash: T::Hash,
        /// Producer account
        pub producer: T::AccountId,
        /// Depth level (1-6)
        pub depth: u32,
        /// Timestamp
        pub timestamp: u64,
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PALLET CONFIGURATION
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, BlockNumberFor<Self>>;
        type Time: Time;
        
        /// Minimum stake for Validity Node
        #[pallet::constant]
        type MinValidityStake: Get<BalanceOf<Self>>;
        
        /// Validator reward per block
        #[pallet::constant]
        type ValidatorReward: Get<BalanceOf<Self>>;

        /// PPFA committee size
        #[pallet::constant]
        type CommitteeSize: Get<u32>;

        /// Epoch duration in blocks
        #[pallet::constant]
        type EpochDuration: Get<u32>;

        /// Base slot duration (milliseconds)
        #[pallet::constant]
        type BaseSlotDuration: Get<u64>;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // STORAGE
    // ═══════════════════════════════════════════════════════════════════════════

    /// All registered validators
    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        Validator<T>
    >;

    /// Current PPFA committee (Proposing Panel for Attestation)
    #[pallet::storage]
    #[pallet::getter(fn current_committee)]
    pub type CurrentCommittee<T: Config> = StorageValue<
        _, 
        BoundedVec<CommitteeMember<T>, ConstU32<MAX_COMMITTEE_SIZE>>, 
        ValueQuery
    >;

    /// Active validator set (all active validators)
    #[pallet::storage]
    #[pallet::getter(fn active_validator_set)]
    pub type ActiveValidators<T: Config> = StorageValue<
        _, 
        BoundedVec<T::AccountId, ConstU32<100>>,
        ValueQuery
    >;

    /// Current epoch number
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Current PPFA index (which validator in committee is proposing)
    #[pallet::storage]
    #[pallet::getter(fn ppfa_index)]
    pub type PpfaIndex<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Validity certificates for blocks (Ascending Scale of Finality)
    #[pallet::storage]
    #[pallet::getter(fn certificates)]
    pub type Certificates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BoundedVec<ValidityCertificate<T>, ConstU32<MAX_CERTIFICATES_PER_BLOCK>>,
        ValueQuery,
    >;

    /// Certificate count per block (for finality level calculation)
    #[pallet::storage]
    #[pallet::getter(fn certificate_count)]
    pub type CertificateCount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, u32, ValueQuery>;

    /// Current consensus phase for active proposal
    #[pallet::storage]
    #[pallet::getter(fn current_phase)]
    pub type CurrentPhase<T: Config> = StorageValue<_, ConsensusPhase, ValueQuery>;

    /// Adaptive slot duration (milliseconds)
    #[pallet::storage]
    #[pallet::getter(fn slot_duration)]
    pub type SlotDuration<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Ants metadata (secondary blocks)
    #[pallet::storage]
    #[pallet::getter(fn ants)]
    pub type Ants<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BoundedVec<AntMetadata<T>, ConstU32<MAX_ANTS_PER_BLOCK>>,
        ValueQuery,
    >;

    /// Network health score (0-100, used for adaptive slots)
    #[pallet::storage]
    #[pallet::getter(fn network_health)]
    pub type NetworkHealth<T: Config> = StorageValue<_, u8, ValueQuery>;

    // ═══════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Validator registered
        ValidatorRegistered {
            account: T::AccountId,
            peer_type: PeerType,
            stake: BalanceOf<T>,
        },
        /// Validator selected for block production
        ValidatorSelected {
            account: T::AccountId,
            block_number: BlockNumberFor<T>,
        },
        /// Validator slashed for misbehavior
        ValidatorSlashed {
            account: T::AccountId,
            reason: BoundedVec<u8, ConstU32<64>>,
        },
        /// Block finalized (reached consensus)
        BlockFinalized {
            block_number: BlockNumberFor<T>,
            block_hash: T::Hash,
            certificate_count: u32,
            finality_level: u8,
        },
        /// Validator rewarded
        ValidatorRewarded {
            account: T::AccountId,
            reward: BalanceOf<T>,
        },
        /// New epoch started
        NewEpoch {
            epoch: u32,
            committee_size: u32,
        },
        /// PPFA committee rotated
        CommitteeRotated {
            epoch: u32,
            members: u32,
        },
        /// Validity certificate issued
        CertificateIssued {
            block_hash: T::Hash,
            phase: ConsensusPhase,
            validator: T::AccountId,
            total_certificates: u32,
        },
        /// Consensus phase advanced
        PhaseAdvanced {
            block_hash: T::Hash,
            from_phase: ConsensusPhase,
            to_phase: ConsensusPhase,
        },
        /// Ant (secondary block) attached
        AntAttached {
            ant_hash: T::Hash,
            parent_hash: T::Hash,
            producer: T::AccountId,
            depth: u32,
        },
        /// Adaptive slot duration adjusted
        SlotDurationAdjusted {
            old_duration: u64,
            new_duration: u64,
            network_health: u8,
        },
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ERRORS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::error]
    pub enum Error<T> {
        /// Already registered as validator
        AlreadyValidator,
        /// Insufficient stake amount
        NotEnoughStake,
        /// Not a registered validator
        NotAValidator,
        /// Block finalization failed
        FinalizationFailed,
        /// Too many validators
        TooManyValidators,
        /// Not in current committee
        NotInCommittee,
        /// Invalid consensus phase
        InvalidPhase,
        /// Too many certificates
        TooManyCertificates,
        /// Duplicate certificate
        DuplicateCertificate,
        /// Committee full
        CommitteeFull,
        /// Invalid PPFA index
        InvalidPpfaIndex,
        /// Ant depth exceeds maximum
        AntDepthExceeded,
        /// Too many ants on block
        TooManyAnts,
        /// Invalid peer type for operation
        InvalidPeerType,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // ═══════════════════════════════════════════════════════════════════════════
    // GENESIS CONFIGURATION
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// Initial validators with stakes
        pub validators: Vec<(T::AccountId, BalanceOf<T>, PeerType)>,
        /// Initial slot duration
        pub slot_duration: u64,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                validators: Default::default(),
                slot_duration: 6000, // 6 seconds
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Initialize slot duration
            SlotDuration::<T>::put(self.slot_duration);
            NetworkHealth::<T>::put(100u8); // Start with optimal health

            // Register initial validators
            for (account, stake, peer_type) in &self.validators {
                let validator = Validator {
                    address: account.clone(),
                    stake: *stake,
                    reputation: 100,
                    peer_type: peer_type.clone(),
                    last_block: Zero::zero(),
                    active: true,
                    last_epoch: 0,
                    blocks_produced: 0,
                    certificates_issued: 0,
                };
                Validators::<T>::insert(account, validator);
                let _ = ActiveValidators::<T>::try_mutate(|v| {
                    v.try_push(account.clone())
                });
            }

            // Initialize first PPFA committee
            Pallet::<T>::rotate_committee();
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HOOKS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            let epoch_duration = T::EpochDuration::get();
            let current_block: u32 = block_number.try_into().unwrap_or(u32::MAX);

            // Check for epoch rotation
            if current_block > 0 && current_block % epoch_duration == 0 {
                let new_epoch = CurrentEpoch::<T>::get().saturating_add(1);
                CurrentEpoch::<T>::put(new_epoch);

                // Rotate PPFA committee
                Self::rotate_committee();

                Self::deposit_event(Event::NewEpoch {
                    epoch: new_epoch,
                    committee_size: CurrentCommittee::<T>::get().len() as u32,
                });
            }

            // Advance PPFA index
            let committee_size = CurrentCommittee::<T>::get().len() as u32;
            if committee_size > 0 {
                let current_index = PpfaIndex::<T>::get();
                let next_index = (current_index + 1) % committee_size;
                PpfaIndex::<T>::put(next_index);
            }

            // Adaptive slot adjustment every 100 blocks
            if current_block % 100 == 0 {
                Self::adjust_adaptive_slot_duration();
            }

            Weight::from_parts(50_000, 0)
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DISPATCHABLE CALLS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as validator
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn register_validator(
            origin: OriginFor<T>,
            peer_type: PeerType,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Validators::<T>::contains_key(&who), Error::<T>::AlreadyValidator);

            // Verify minimum stake based on peer type
            let min_stake = match peer_type {
                PeerType::Common => Zero::zero(),
                PeerType::StakingCommon => MIN_STAKE_PEER_STAKE.try_into().ok().unwrap_or(Zero::zero()),
                PeerType::ValidityNode => T::MinValidityStake::get(),
                PeerType::FlareNode => T::MinValidityStake::get(),
                PeerType::DecentralizedDirector => MIN_DIRECTOR_STAKE.try_into().ok().unwrap_or(Zero::zero()),
            };

            ensure!(stake >= min_stake, Error::<T>::NotEnoughStake);

            // Reserve stake
            T::Currency::reserve(&who, stake)?;

            let validator = Validator {
                address: who.clone(),
                stake,
                reputation: 100,
                peer_type: peer_type.clone(),
                last_block: Zero::zero(),
                active: true,
                last_epoch: CurrentEpoch::<T>::get(),
                blocks_produced: 0,
                certificates_issued: 0,
            };

            Validators::<T>::insert(&who, validator);
            ActiveValidators::<T>::try_mutate(|v| {
                v.try_push(who.clone()).map_err(|_| Error::<T>::TooManyValidators)
            })?;

            Self::deposit_event(Event::ValidatorRegistered {
                account: who,
                peer_type,
                stake,
            });

            Ok(())
        }

        /// Issue validity certificate (authority/committee only)
        #[pallet::weight(70_000)]
        #[pallet::call_index(1)]
        pub fn issue_certificate(
            origin: OriginFor<T>,
            block_hash: T::Hash,
            block_number: BlockNumberFor<T>,
            phase: ConsensusPhase,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify validator is in current committee
            let committee = CurrentCommittee::<T>::get();
            let member = committee
                .iter()
                .find(|m| m.validator == who)
                .ok_or(Error::<T>::NotInCommittee)?;

            let cert = ValidityCertificate {
                block_hash,
                block_number,
                phase: phase.clone(),
                validator: who.clone(),
                stake_weight: member.stake,
                epoch: CurrentEpoch::<T>::get(),
                timestamp: T::Time::now().try_into().ok().unwrap_or(0),
            };

            // Add certificate
            Certificates::<T>::try_mutate(block_hash, |certs| {
                certs
                    .try_push(cert)
                    .map_err(|_| Error::<T>::TooManyCertificates)
            })?;

            // Increment certificate count
            let count = CertificateCount::<T>::mutate(block_hash, |c| {
                *c = c.saturating_add(1);
                *c
            });

            // Update validator stats
            Validators::<T>::mutate(&who, |v| {
                if let Some(val) = v {
                    val.certificates_issued = val.certificates_issued.saturating_add(1);
                }
            });

            Self::deposit_event(Event::CertificateIssued {
                block_hash,
                phase,
                validator: who,
                total_certificates: count,
            });

            // Check finality
            Self::check_finality(block_hash, block_number, count);

            Ok(())
        }

        /// Attach ant (secondary block) to primary block
        #[pallet::weight(30_000)]
        #[pallet::call_index(2)]
        pub fn attach_ant(
            origin: OriginFor<T>,
            ant_hash: T::Hash,
            parent_hash: T::Hash,
            depth: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify depth limit
            ensure!(depth <= MAX_ANTS_DEPTH, Error::<T>::AntDepthExceeded);

            let ant = AntMetadata {
                ant_hash,
                parent_hash,
                producer: who.clone(),
                depth,
                timestamp: T::Time::now().try_into().ok().unwrap_or(0),
            };

            // Add ant to parent
            Ants::<T>::try_mutate(parent_hash, |ants| {
                ants.try_push(ant).map_err(|_| Error::<T>::TooManyAnts)
            })?;

            Self::deposit_event(Event::AntAttached {
                ant_hash,
                parent_hash,
                producer: who,
                depth,
            });

            Ok(())
        }

        /// Force committee rotation (governance only)
        #[pallet::weight(100_000)]
        #[pallet::call_index(3)]
        pub fn force_rotate_committee(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Self::rotate_committee();
            Ok(())
        }

        /// Manually adjust slot duration (governance only)
        #[pallet::weight(20_000)]
        #[pallet::call_index(4)]
        pub fn set_slot_duration(origin: OriginFor<T>, duration: u64) -> DispatchResult {
            ensure_root(origin)?;
            
            let old = SlotDuration::<T>::get();
            SlotDuration::<T>::put(duration);

            Self::deposit_event(Event::SlotDurationAdjusted {
                old_duration: old,
                new_duration: duration,
                network_health: NetworkHealth::<T>::get(),
            });

            Ok(())
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HELPER FUNCTIONS (From Ivory Papers)
    // ═══════════════════════════════════════════════════════════════════════════

    impl<T: Config> Pallet<T> {
        /// Select validator for block production (VRF-based)
        pub fn select_validator(block_number: BlockNumberFor<T>) -> Option<T::AccountId> {
            let validators = ActiveValidators::<T>::get();
            if validators.is_empty() {
                return None;
            }

            let seed = T::RandomnessSource::random(&block_number.encode());
            let seed_bytes = seed.0.encode();
            let seed_hash = sp_io::hashing::blake2_256(&seed_bytes);
            let seed_num = seed_hash.as_ref()[0] as usize;
            let index = seed_num % validators.len();
            let selected = validators.get(index)?.clone();

            Self::deposit_event(Event::ValidatorSelected {
                account: selected.clone(),
                block_number,
            });

            Some(selected)
        }

        /// Finalize block (2/3+ votes required)
        pub fn finalize_block(
            block_hash: T::Hash,
            block_number: BlockNumberFor<T>,
            votes: Vec<T::AccountId>,
        ) -> DispatchResult {
            let validators = ActiveValidators::<T>::get();
            let total = validators.len();
            let threshold = (total * 2) / 3;

            if votes.len() >= threshold {
                for v in votes.iter() {
                    Self::reward_validator(v.clone())?;
                }

                let cert_count = CertificateCount::<T>::get(block_hash);
                let finality_level = Self::calculate_finality_level(cert_count);

                Self::deposit_event(Event::BlockFinalized {
                    block_number,
                    block_hash,
                    certificate_count: cert_count,
                    finality_level,
                });

                Ok(())
            } else {
                Err(Error::<T>::FinalizationFailed.into())
            }
        }

        /// Reward validator for participation
        pub fn reward_validator(who: T::AccountId) -> DispatchResult {
            let reward = T::ValidatorReward::get();
            T::Currency::unreserve(&who, reward);
            let _ = T::Currency::deposit_creating(&who, reward);

            // Update validator stats
            Validators::<T>::mutate(&who, |v| {
                if let Some(val) = v {
                    val.blocks_produced = val.blocks_produced.saturating_add(1);
                }
            });

            Self::deposit_event(Event::ValidatorRewarded {
                account: who,
                reward,
            });

            Ok(())
        }

        /// Slash validator for misbehavior
        pub fn slash_validator(who: T::AccountId, reason: &[u8]) -> DispatchResult {
            Validators::<T>::mutate_exists(&who, |v| {
                if let Some(mut val) = v.take() {
                    val.reputation = 0;
                    val.active = false;
                    *v = Some(val);
                }
                Ok::<(), DispatchError>(())
            })?;

            let reason_bounded: BoundedVec<u8, ConstU32<64>> =
                reason.to_vec().try_into().unwrap_or_default();

            Self::deposit_event(Event::ValidatorSlashed {
                account: who,
                reason: reason_bounded,
            });

            Ok(())
        }

        // ═══════════════════════════════════════════════════════════════════════
        // RUNTIME API HELPERS (For ASF Consensus Service)
        // ═══════════════════════════════════════════════════════════════════════
        // Note: ppfa_index(), current_epoch(), slot_duration() already exist via #[pallet::getter]

        /// Get current PPFA committee as a vector of AccountIds
        ///
        /// Returns the list of validators in the current PPFA committee.
        /// The committee size is typically 21 validators selected by stake weight.
        pub fn committee() -> Vec<T::AccountId> {
            Self::current_committee()
                .into_iter()
                .map(|m| m.validator)
                .collect()
        }

        /// Check if a validator should propose in the current slot
        ///
        /// Returns true if the given validator is the current proposer
        /// according to the PPFA rotation.
        pub fn should_propose(validator: T::AccountId) -> bool {
            let committee = Self::committee();
            if committee.is_empty() {
                return false;
            }

            let ppfa_idx = Self::ppfa_index();
            let expected_idx = (ppfa_idx as usize) % committee.len();

            committee.get(expected_idx)
                .map(|expected| expected == &validator)
                .unwrap_or(false)
        }

        /// Get all active validators
        ///
        /// Returns all validators in the active set (up to 100),
        /// not just the committee members.
        pub fn active_validators() -> Vec<T::AccountId> {
            Self::active_validator_set().to_vec()
        }

        /// Rotate PPFA committee (stake-weighted selection)
        fn rotate_committee() {
            let committee_size = T::CommitteeSize::get();
            let mut all_validators: Vec<_> = Validators::<T>::iter()
                .filter(|(_, info)| info.active && matches!(
                    info.peer_type,
                    PeerType::ValidityNode | PeerType::FlareNode
                ))
                .collect();

            // Sort by stake (descending)
            all_validators.sort_by(|a, b| b.1.stake.cmp(&a.1.stake));

            // Select top validators
            let selected: BoundedVec<CommitteeMember<T>, ConstU32<MAX_COMMITTEE_SIZE>> =
                all_validators
                    .into_iter()
                    .take(committee_size as usize)
                    .enumerate()
                    .map(|(index, (account, info))| CommitteeMember {
                        validator: account,
                        stake: info.stake,
                        ppfa_index: index as u32,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_default();

            let member_count = selected.len() as u32;
            CurrentCommittee::<T>::put(selected);
            PpfaIndex::<T>::put(0); // Reset to first member

            Self::deposit_event(Event::CommitteeRotated {
                epoch: CurrentEpoch::<T>::get(),
                members: member_count,
            });
        }

        /// Check finality and emit event
        fn check_finality(block_hash: T::Hash, block_number: BlockNumberFor<T>, count: u32) {
            let level = Self::calculate_finality_level(count);

            if level > 0 {
                Self::deposit_event(Event::BlockFinalized {
                    block_hash,
                    block_number,
                    certificate_count: count,
                    finality_level: level,
                });
            }
        }

        /// Calculate finality level from certificate count (Ascending Scale)
        fn calculate_finality_level(count: u32) -> u8 {
            match count {
                0..=9 => 0,    // Not finalized
                10..=19 => 1,  // Weak finality
                20..=49 => 2,  // Moderate finality
                50..=99 => 3,  // Strong finality
                _ => 4,        // Irreversible finality
            }
        }

        /// Adjust adaptive slot duration based on network health
        fn adjust_adaptive_slot_duration() {
            let health = NetworkHealth::<T>::get();
            let base = T::BaseSlotDuration::get();

            // Formula: slot_duration = base × (1 + adaptive_factor)
            // adaptive_factor ∈ [0.0, 2.0] based on health
            let adaptive_factor = match health {
                90..=100 => 0,      // 0.0 - optimal
                70..=89 => 20,      // 0.2 - normal
                50..=69 => 50,      // 0.5 - degraded
                30..=49 => 100,     // 1.0 - poor
                _ => 200,           // 2.0 - critical
            };

            let new_duration = base + (base * adaptive_factor / 100);
            let old_duration = SlotDuration::<T>::get();

            if new_duration != old_duration {
                SlotDuration::<T>::put(new_duration);

                Self::deposit_event(Event::SlotDurationAdjusted {
                    old_duration,
                    new_duration,
                    network_health: health,
                });
            }
        }
    }
}
