//! # ASF Finality Pallet
//!
//! This pallet stores ASF (Ascending Scale of Finality) state on-chain.
//!
//! ## Overview
//!
//! The ASF Finality pallet provides:
//! - Storage for the last finalized block
//! - Certificate storage for finalized blocks
//! - Equivocation reporting and slashing
//! - Checkpoint storage
//!
//! ## Finality Levels
//!
//! 1. **Implicit**: Block is K confirmations deep
//! 2. **Pre-committed**: Has 1 certificate (2f+1 signatures)
//! 3. **Committed**: Has 2 consecutive certificates
//! 4. **Finalized**: Has 3 consecutive certificates
//! 5. **Checkpointed**: All validators signed (permanent)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;
    use sp_std::vec::Vec;

    /// The pallet's configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum signatures per certificate
        #[pallet::constant]
        type MaxSignaturesPerCertificate: Get<u32>;

        /// Maximum certificates to store per block
        #[pallet::constant]
        type MaxCertificatesPerBlock: Get<u32>;

        /// Equivocation slash amount (in native currency units)
        #[pallet::constant]
        type EquivocationSlashAmount: Get<u128>;

        /// BFT threshold numerator (e.g., 2 for 2/3)
        #[pallet::constant]
        type BftThresholdNumerator: Get<u32>;

        /// BFT threshold denominator (e.g., 3 for 2/3)
        #[pallet::constant]
        type BftThresholdDenominator: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // ═══════════════════════════════════════════════════════════════════════════════
    // STORAGE
    // ═══════════════════════════════════════════════════════════════════════════════

    /// The last finalized block number
    #[pallet::storage]
    #[pallet::getter(fn last_finalized)]
    pub type LastFinalized<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// The last finalized block hash
    #[pallet::storage]
    #[pallet::getter(fn last_finalized_hash)]
    pub type LastFinalizedHash<T: Config> = StorageValue<_, T::Hash, OptionQuery>;

    /// Finality state per block hash
    #[pallet::storage]
    #[pallet::getter(fn finality_state)]
    pub type FinalityStateMap<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        FinalityState,
        ValueQuery,
    >;

    /// Certificates per block hash
    #[pallet::storage]
    #[pallet::getter(fn certificates)]
    pub type Certificates<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BoundedVec<Certificate, T::MaxCertificatesPerBlock>,
        ValueQuery,
    >;

    /// Checkpoints by block number
    #[pallet::storage]
    #[pallet::getter(fn checkpoints)]
    pub type Checkpoints<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BlockNumberFor<T>,
        StoredCheckpoint,
        OptionQuery,
    >;

    /// Latest checkpoint block number
    #[pallet::storage]
    #[pallet::getter(fn latest_checkpoint)]
    pub type LatestCheckpoint<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

    /// Equivocation reports
    #[pallet::storage]
    #[pallet::getter(fn equivocation_reports)]
    pub type EquivocationReports<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32], // validator_id
        EquivocationReport,
        OptionQuery,
    >;

    /// Slashed validators
    #[pallet::storage]
    #[pallet::getter(fn slashed_validators)]
    pub type SlashedValidators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32], // validator_id
        SlashRecord,
        OptionQuery,
    >;

    /// Current view number
    #[pallet::storage]
    #[pallet::getter(fn current_view)]
    pub type CurrentView<T: Config> = StorageValue<_, u64, ValueQuery>;

    // ═══════════════════════════════════════════════════════════════════════════════
    // TYPES
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Finality state for a block
    #[derive(Clone, Debug, Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Default)]
    pub enum FinalityState {
        /// Block imported but no certificates yet
        #[default]
        Pending,
        /// Has 1 certificate (2f+1 signatures)
        PreCommitted,
        /// Has 2 consecutive certificates
        Committed,
        /// Has 3 consecutive certificates - fully finalized
        Finalized,
        /// All validators signed - permanent checkpoint
        Checkpointed,
    }

    /// A finality certificate
    #[derive(Clone, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Certificate {
        /// View number
        pub view: u64,
        /// Block hash (as bytes for MaxEncodedLen compatibility)
        pub block_hash: [u8; 32],
        /// Block number
        pub block_number: u32,
        /// Number of signatures (actual signatures stored off-chain)
        pub signature_count: u32,
        /// Timestamp
        pub created_at: u64,
    }

    /// A stored checkpoint
    #[derive(Clone, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub struct StoredCheckpoint {
        /// Block number
        pub block_number: u32,
        /// Block hash
        pub block_hash: [u8; 32],
        /// State root
        pub state_root: [u8; 32],
        /// Number of signatures (all validators)
        pub signature_count: u32,
        /// Timestamp
        pub created_at: u64,
        /// Parent checkpoint block number
        pub parent_checkpoint: Option<u32>,
    }

    /// Equivocation report
    #[derive(Clone, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub struct EquivocationReport {
        /// Offending validator
        pub validator_id: [u8; 32],
        /// View where equivocation occurred
        pub view: u64,
        /// First block voted for
        pub first_block: [u8; 32],
        /// Second block voted for
        pub second_block: [u8; 32],
        /// When reported
        pub reported_at: u64,
        /// Whether slashing has been applied
        pub slashed: bool,
    }

    /// Slash record
    #[derive(Clone, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub struct SlashRecord {
        /// Validator slashed
        pub validator_id: [u8; 32],
        /// Amount slashed
        pub amount: u128,
        /// View where offense occurred
        pub offense_view: u64,
        /// Block number when slashed
        pub slashed_at_block: u32,
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A block has been finalized
        BlockFinalized {
            hash: T::Hash,
            number: BlockNumberFor<T>,
        },
        /// A certificate has been recorded
        CertificateRecorded {
            hash: T::Hash,
            view: u64,
            signature_count: u32,
        },
        /// A checkpoint has been created
        CheckpointCreated {
            number: BlockNumberFor<T>,
            signature_count: u32,
        },
        /// Equivocation detected and reported
        EquivocationReported {
            validator: [u8; 32],
            view: u64,
        },
        /// Validator slashed for equivocation
        ValidatorSlashed {
            validator: [u8; 32],
            amount: u128,
        },
        /// Finality state changed
        FinalityStateChanged {
            hash: T::Hash,
            new_state: FinalityState,
        },
        /// View number advanced
        ViewAdvanced {
            new_view: u64,
        },
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // ERRORS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[pallet::error]
    pub enum Error<T> {
        /// Block not found
        BlockNotFound,
        /// Certificate already exists for this view
        CertificateExists,
        /// Insufficient signatures for certificate
        InsufficientSignatures,
        /// Invalid signature
        InvalidSignature,
        /// Equivocation already reported
        AlreadyReported,
        /// Not a valid equivocation
        NotEquivocation,
        /// Validator already slashed
        AlreadySlashed,
        /// Maximum certificates reached for this block
        MaxCertificatesReached,
        /// Invalid finality state transition
        InvalidStateTransition,
        /// Checkpoint already exists
        CheckpointExists,
        /// Invalid checkpoint
        InvalidCheckpoint,
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // CALLS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Record a finality certificate for a block
        ///
        /// Called by validators when a certificate is created.
        /// This is typically called via an inherent or unsigned transaction.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn record_certificate(
            origin: OriginFor<T>,
            block_hash: T::Hash,
            block_number: BlockNumberFor<T>,
            view: u64,
            signature_count: u32,
        ) -> DispatchResult {
            ensure_none(origin)?;

            // Verify signature count meets threshold
            let threshold = Self::bft_threshold(21); // TODO: Get from validator set
            ensure!(signature_count >= threshold, Error::<T>::InsufficientSignatures);

            // Create certificate
            let certificate = Certificate {
                view,
                block_hash: Self::hash_to_bytes(&block_hash),
                block_number: Self::block_number_to_u32(block_number),
                signature_count,
                created_at: Self::current_timestamp(),
            };

            // Store certificate
            Certificates::<T>::try_mutate(&block_hash, |certs| -> DispatchResult {
                certs.try_push(certificate.clone())
                    .map_err(|_| Error::<T>::MaxCertificatesReached)?;
                Ok(())
            })?;

            // Update finality state
            Self::update_finality_state(block_hash, block_number)?;

            // Update current view
            if view > CurrentView::<T>::get() {
                CurrentView::<T>::put(view);
                Self::deposit_event(Event::ViewAdvanced { new_view: view });
            }

            Self::deposit_event(Event::CertificateRecorded {
                hash: block_hash,
                view,
                signature_count,
            });

            Ok(())
        }

        /// Report an equivocation
        ///
        /// Anyone can report an equivocation with valid proof.
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn report_equivocation(
            origin: OriginFor<T>,
            validator_id: [u8; 32],
            view: u64,
            first_block: [u8; 32],
            second_block: [u8; 32],
            _first_signature: [u8; 64],
            _second_signature: [u8; 64],
        ) -> DispatchResult {
            // Can be signed or unsigned
            let _ = ensure_signed_or_root(origin)?;

            // Verify this is actually equivocation (different blocks)
            ensure!(first_block != second_block, Error::<T>::NotEquivocation);

            // Check not already reported
            ensure!(
                !EquivocationReports::<T>::contains_key(&validator_id),
                Error::<T>::AlreadyReported
            );

            // TODO: Verify signatures on both votes
            // This requires access to the signing data structure

            // Store the report
            let report = EquivocationReport {
                validator_id,
                view,
                first_block,
                second_block,
                reported_at: Self::current_timestamp(),
                slashed: false,
            };
            EquivocationReports::<T>::insert(&validator_id, report);

            Self::deposit_event(Event::EquivocationReported {
                validator: validator_id,
                view,
            });

            // Apply slashing
            Self::slash_validator(validator_id, view)?;

            Ok(())
        }

        /// Record a checkpoint
        #[pallet::call_index(2)]
        #[pallet::weight(20_000)]
        pub fn record_checkpoint(
            origin: OriginFor<T>,
            block_number: BlockNumberFor<T>,
            block_hash: T::Hash,
            state_root: [u8; 32],
            signature_count: u32,
        ) -> DispatchResult {
            ensure_none(origin)?;

            let number_u32 = Self::block_number_to_u32(block_number);

            // Check checkpoint doesn't exist
            ensure!(
                !Checkpoints::<T>::contains_key(block_number),
                Error::<T>::CheckpointExists
            );

            // Get parent checkpoint
            let parent = LatestCheckpoint::<T>::get()
                .map(|n| Self::block_number_to_u32(n));

            let checkpoint = StoredCheckpoint {
                block_number: number_u32,
                block_hash: Self::hash_to_bytes(&block_hash),
                state_root,
                signature_count,
                created_at: Self::current_timestamp(),
                parent_checkpoint: parent,
            };

            Checkpoints::<T>::insert(block_number, checkpoint);
            LatestCheckpoint::<T>::put(block_number);

            // Update finality state to checkpointed
            FinalityStateMap::<T>::insert(&block_hash, FinalityState::Checkpointed);

            Self::deposit_event(Event::CheckpointCreated {
                number: block_number,
                signature_count,
            });

            Ok(())
        }

        /// Advance the view number
        #[pallet::call_index(3)]
        #[pallet::weight(5_000)]
        pub fn advance_view(
            origin: OriginFor<T>,
            new_view: u64,
        ) -> DispatchResult {
            ensure_none(origin)?;

            let current = CurrentView::<T>::get();
            if new_view > current {
                CurrentView::<T>::put(new_view);
                Self::deposit_event(Event::ViewAdvanced { new_view });
            }

            Ok(())
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // INTERNAL FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════════════════

    impl<T: Config> Pallet<T> {
        /// Calculate BFT threshold (2f+1 for 3f+1 validators)
        pub fn bft_threshold(committee_size: u32) -> u32 {
            let num = T::BftThresholdNumerator::get();
            let den = T::BftThresholdDenominator::get();
            (num * committee_size / den) + 1
        }

        /// Update finality state based on certificates
        fn update_finality_state(
            block_hash: T::Hash,
            block_number: BlockNumberFor<T>,
        ) -> DispatchResult {
            let certs = Certificates::<T>::get(&block_hash);
            let cert_count = certs.len();

            let new_state = match cert_count {
                0 => FinalityState::Pending,
                1 => FinalityState::PreCommitted,
                2 => FinalityState::Committed,
                _ => FinalityState::Finalized,
            };

            let old_state = FinalityStateMap::<T>::get(&block_hash);

            if new_state != old_state {
                FinalityStateMap::<T>::insert(&block_hash, new_state.clone());

                Self::deposit_event(Event::FinalityStateChanged {
                    hash: block_hash,
                    new_state: new_state.clone(),
                });

                // If finalized, update last finalized
                if new_state == FinalityState::Finalized {
                    let current_finalized = LastFinalized::<T>::get();
                    if block_number > current_finalized {
                        LastFinalized::<T>::put(block_number);
                        LastFinalizedHash::<T>::put(block_hash);

                        Self::deposit_event(Event::BlockFinalized {
                            hash: block_hash,
                            number: block_number,
                        });
                    }
                }
            }

            Ok(())
        }

        /// Slash a validator for equivocation
        fn slash_validator(validator_id: [u8; 32], offense_view: u64) -> DispatchResult {
            ensure!(
                !SlashedValidators::<T>::contains_key(&validator_id),
                Error::<T>::AlreadySlashed
            );

            let amount = T::EquivocationSlashAmount::get();
            let current_block = frame_system::Pallet::<T>::block_number();

            let record = SlashRecord {
                validator_id,
                amount,
                offense_view,
                slashed_at_block: Self::block_number_to_u32(current_block),
            };

            SlashedValidators::<T>::insert(&validator_id, record);

            // Mark report as slashed
            EquivocationReports::<T>::mutate(&validator_id, |maybe_report| {
                if let Some(report) = maybe_report {
                    report.slashed = true;
                }
            });

            // TODO: Actually transfer/burn the slashed amount
            // This requires integration with staking pallet

            Self::deposit_event(Event::ValidatorSlashed {
                validator: validator_id,
                amount,
            });

            Ok(())
        }

        /// Get current timestamp
        fn current_timestamp() -> u64 {
            // In production, use pallet_timestamp
            0
        }

        /// Convert hash to bytes
        fn hash_to_bytes(hash: &T::Hash) -> [u8; 32] {
            let encoded = hash.encode();
            let mut bytes = [0u8; 32];
            let len = encoded.len().min(32);
            bytes[..len].copy_from_slice(&encoded[..len]);
            bytes
        }

        /// Convert block number to u32
        fn block_number_to_u32(number: BlockNumberFor<T>) -> u32 {
            // Safe conversion - block numbers fit in u32 for practical purposes
            use sp_runtime::traits::UniqueSaturatedInto;
            number.unique_saturated_into()
        }
    }

    /// Ensure origin is signed or root
    fn ensure_signed_or_root<T: Config>(
        origin: OriginFor<T>,
    ) -> Result<Option<T::AccountId>, DispatchError> {
        match origin.into() {
            Ok(frame_system::RawOrigin::Signed(t)) => Ok(Some(t)),
            Ok(frame_system::RawOrigin::Root) => Ok(None),
            _ => Err(frame_support::error::BadOrigin.into()),
        }
    }
}
