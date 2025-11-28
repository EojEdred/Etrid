//! # Pallet ASF Collator
//!
//! FRAME pallet for ASF collator consensus in PBC runtimes.
//!
//! Features:
//! - Collator registration and management
//! - Block proposal validation with ASF
//! - Certificate submission and verification
//! - Relay chain finality sync
//! - Cross-chain attestations

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Get},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use sp_core::H256;
    use sp_runtime::traits::Hash;

    use asf_collator::{
        CollatorId, CollatorCommittee, CollatorVote, CollatorCertificate,
        CollatorProposal, CollatorFinalityLevel, RelayChainFinalityProof,
        CrossChainAttestation, MultiSigAttestation, ParaId,
        RotationConfig, RotationManager, FinalityTracker,
        SessionManager, StakeManager, CommitteeChange,
    };
    use asf_algorithm::{ConsensusPhase, BlockNumber as AsfBlockNumber, Balance as AsfBalance};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type for staking
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Parachain ID
        #[pallet::constant]
        type ParaId: Get<u32>;

        /// Minimum collators required
        #[pallet::constant]
        type MinCollators: Get<u32>;

        /// Maximum collators allowed
        #[pallet::constant]
        type MaxCollators: Get<u32>;

        /// Minimum stake required to be a collator
        #[pallet::constant]
        type MinCollatorStake: Get<BalanceOf<Self>>;

        /// Session length in blocks
        #[pallet::constant]
        type SessionLength: Get<BlockNumberFor<Self>>;

        /// Rotation period (blocks between collator rotations)
        #[pallet::constant]
        type RotationPeriod: Get<u64>;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // STORAGE
    // ═══════════════════════════════════════════════════════════════════════════

    /// Current collator committee
    #[pallet::storage]
    #[pallet::getter(fn committee)]
    pub type Committee<T: Config> = StorageValue<_, CollatorCommittee, OptionQuery>;

    /// Collator stakes
    #[pallet::storage]
    #[pallet::getter(fn collator_stake)]
    pub type CollatorStakes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Pending block proposals
    #[pallet::storage]
    #[pallet::getter(fn pending_proposal)]
    pub type PendingProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // block_hash
        CollatorProposal,
        OptionQuery,
    >;

    /// Block certificates
    #[pallet::storage]
    #[pallet::getter(fn certificate)]
    pub type Certificates<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        H256, // block_hash
        Blake2_128Concat,
        ConsensusPhase,
        CollatorCertificate,
        OptionQuery,
    >;

    /// Finality tracking
    #[pallet::storage]
    #[pallet::getter(fn finality_level)]
    pub type FinalityLevels<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // block_hash
        CollatorFinalityLevel,
        ValueQuery,
    >;

    /// Certificate counts per block
    #[pallet::storage]
    #[pallet::getter(fn certificate_count)]
    pub type CertificateCounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // block_hash
        u32,
        ValueQuery,
    >;

    /// Relay chain finality proofs
    #[pallet::storage]
    #[pallet::getter(fn relay_finality)]
    pub type RelayFinality<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256, // para_block_hash
        RelayChainFinalityProof,
        OptionQuery,
    >;

    /// Cross-chain attestations
    #[pallet::storage]
    #[pallet::getter(fn cross_chain_attestation)]
    pub type CrossChainAttestations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // target_para_id
        Blake2_128Concat,
        H256, // target_block_hash
        MultiSigAttestation,
        OptionQuery,
    >;

    /// Current rotation round
    #[pallet::storage]
    #[pallet::getter(fn rotation_round)]
    pub type RotationRound<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Current session index
    #[pallet::storage]
    #[pallet::getter(fn session_index)]
    pub type SessionIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

    // ═══════════════════════════════════════════════════════════════════════════
    // EVENTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Collator registered
        CollatorRegistered {
            collator: T::AccountId,
            stake: BalanceOf<T>,
        },
        /// Collator deregistered
        CollatorDeregistered {
            collator: T::AccountId,
        },
        /// Block proposal submitted
        ProposalSubmitted {
            block_hash: H256,
            collator: T::AccountId,
            phase: ConsensusPhase,
        },
        /// Vote submitted
        VoteSubmitted {
            block_hash: H256,
            collator: T::AccountId,
            phase: ConsensusPhase,
        },
        /// Certificate created
        CertificateCreated {
            block_hash: H256,
            phase: ConsensusPhase,
            votes: u32,
        },
        /// Finality level updated
        FinalityUpdated {
            block_hash: H256,
            finality_level: CollatorFinalityLevel,
        },
        /// Relay chain finality proof submitted
        RelayFinalityProof {
            para_block: H256,
            relay_block: u64,
        },
        /// Cross-chain attestation submitted
        CrossChainAttestation {
            target_para: u32,
            target_block: H256,
        },
        /// Collator rotated
        CollatorRotated {
            new_collator: T::AccountId,
            rotation_round: u64,
        },
        /// Session rotated
        SessionRotated {
            session_index: u64,
        },
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ERRORS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::error]
    pub enum Error<T> {
        /// Not a registered collator
        NotCollator,
        /// Already registered as collator
        AlreadyCollator,
        /// Insufficient stake
        InsufficientStake,
        /// Committee full
        CommitteeFull,
        /// Committee not initialized
        CommitteeNotInitialized,
        /// Invalid proposal
        InvalidProposal,
        /// Invalid vote
        InvalidVote,
        /// Duplicate vote
        DuplicateVote,
        /// Certificate not found
        CertificateNotFound,
        /// Invalid certificate
        InvalidCertificate,
        /// Not authorized
        NotAuthorized,
        /// Invalid phase transition
        InvalidPhaseTransition,
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HOOKS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Check if should rotate collator
            let rotation_period = T::RotationPeriod::get();
            let block_num = TryInto::<u64>::try_into(n).unwrap_or(0);

            if block_num % rotation_period == 0 {
                let mut round = RotationRound::<T>::get();
                round += 1;
                RotationRound::<T>::put(round);
            }

            // Check if should rotate session
            let session_length = T::SessionLength::get();
            if n % session_length == BlockNumberFor::<T>::from(0u32) {
                let mut index = SessionIndex::<T>::get();
                index += 1;
                SessionIndex::<T>::put(index);

                Self::deposit_event(Event::SessionRotated {
                    session_index: index,
                });
            }

            Weight::from_parts(10_000, 0)
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXTRINSICS
    // ═══════════════════════════════════════════════════════════════════════════

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as collator
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn register_collator(
            origin: OriginFor<T>,
            stake: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check minimum stake
            ensure!(
                stake >= T::MinCollatorStake::get(),
                Error::<T>::InsufficientStake
            );

            // Reserve stake
            T::Currency::reserve(&who, stake)?;

            // Add to committee
            Committee::<T>::try_mutate(|committee_opt| -> DispatchResult {
                let committee = committee_opt.as_mut()
                    .ok_or(Error::<T>::CommitteeNotInitialized)?;

                let collator_id = Self::account_to_collator_id(&who);
                let stake_u128: u128 = TryInto::<u128>::try_into(stake).unwrap_or(0);

                committee.add_collator(collator_id, stake_u128)
                    .map_err(|_| Error::<T>::CommitteeFull)?;

                CollatorStakes::<T>::insert(&who, stake);

                Self::deposit_event(Event::CollatorRegistered {
                    collator: who,
                    stake,
                });

                Ok(())
            })
        }

        /// Deregister as collator
        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn deregister_collator(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let stake = CollatorStakes::<T>::take(&who);
            ensure!(stake > BalanceOf::<T>::from(0u32), Error::<T>::NotCollator);

            // Unreserve stake
            T::Currency::unreserve(&who, stake);

            // Remove from committee
            Committee::<T>::try_mutate(|committee_opt| -> DispatchResult {
                let committee = committee_opt.as_mut()
                    .ok_or(Error::<T>::CommitteeNotInitialized)?;

                let collator_id = Self::account_to_collator_id(&who);
                let stake_u128: u128 = TryInto::<u128>::try_into(stake).unwrap_or(0);

                committee.remove_collator(&collator_id, stake_u128)
                    .map_err(|_| Error::<T>::NotCollator)?;

                Self::deposit_event(Event::CollatorDeregistered { collator: who });

                Ok(())
            })
        }

        /// Submit block proposal
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn submit_proposal(
            origin: OriginFor<T>,
            block_hash: H256,
            block_number: u64,
            phase: ConsensusPhase,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify is collator
            ensure!(
                CollatorStakes::<T>::contains_key(&who),
                Error::<T>::NotCollator
            );

            let collator_id = Self::account_to_collator_id(&who);
            let rotation_round = RotationRound::<T>::get();

            let proposal = CollatorProposal {
                para_id: T::ParaId::get(),
                block_hash,
                block_number,
                collator: collator_id,
                phase,
                rotation_round,
                relay_parent: H256::zero(), // Would be populated from cumulus
            };

            PendingProposals::<T>::insert(block_hash, proposal);

            Self::deposit_event(Event::ProposalSubmitted {
                block_hash,
                collator: who,
                phase,
            });

            Ok(())
        }

        /// Submit vote for block
        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn submit_vote(
            origin: OriginFor<T>,
            block_hash: H256,
            block_number: u64,
            phase: ConsensusPhase,
            signature: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify is collator
            ensure!(
                CollatorStakes::<T>::contains_key(&who),
                Error::<T>::NotCollator
            );

            let collator_id = Self::account_to_collator_id(&who);
            let rotation_round = RotationRound::<T>::get();

            let vote = CollatorVote {
                para_id: T::ParaId::get(),
                block_hash,
                block_number,
                collator: collator_id,
                phase,
                rotation_round,
                signature,
            };

            // Try to add to existing certificate or create new one
            Certificates::<T>::try_mutate(block_hash, phase, |cert_opt| -> DispatchResult {
                let stake = CollatorStakes::<T>::get(&who);
                let stake_u128: u128 = TryInto::<u128>::try_into(stake).unwrap_or(0);

                match cert_opt {
                    Some(cert) => {
                        cert.add_vote(vote, stake_u128)
                            .map_err(|_| Error::<T>::DuplicateVote)?;
                    }
                    None => {
                        let mut cert = CollatorCertificate::new(
                            T::ParaId::get(),
                            block_hash,
                            block_number,
                            phase,
                            rotation_round,
                        );
                        cert.add_vote(vote, stake_u128)
                            .map_err(|_| Error::<T>::InvalidVote)?;
                        *cert_opt = Some(cert);
                    }
                }

                Self::deposit_event(Event::VoteSubmitted {
                    block_hash,
                    collator: who,
                    phase,
                });

                Ok(())
            })?;

            // Update certificate count and finality
            Self::update_finality(block_hash)?;

            Ok(())
        }

        /// Submit relay chain finality proof
        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn submit_relay_finality(
            origin: OriginFor<T>,
            proof: RelayChainFinalityProof,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            RelayFinality::<T>::insert(proof.para_hash, proof.clone());

            // Update finality level
            Self::update_finality(proof.para_hash)?;

            Self::deposit_event(Event::RelayFinalityProof {
                para_block: proof.para_hash,
                relay_block: proof.relay_block,
            });

            Ok(())
        }

        /// Submit cross-chain attestation
        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn submit_cross_chain_attestation(
            origin: OriginFor<T>,
            attestation: CrossChainAttestation,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify is collator
            ensure!(
                CollatorStakes::<T>::contains_key(&who),
                Error::<T>::NotCollator
            );

            let target_para = attestation.target_para;
            let target_block = attestation.target_block_hash;

            // Add to multi-sig attestation
            CrossChainAttestations::<T>::try_mutate(
                target_para,
                target_block,
                |multisig_opt| -> DispatchResult {
                    let stake = CollatorStakes::<T>::get(&who);
                    let stake_u128: u128 = TryInto::<u128>::try_into(stake).unwrap_or(0);

                    match multisig_opt {
                        Some(multisig) => {
                            multisig.add_attestation(attestation, stake_u128)
                                .map_err(|_| Error::<T>::InvalidCertificate)?;
                        }
                        None => {
                            let mut multisig = MultiSigAttestation::new(
                                target_para,
                                target_block,
                                attestation.target_block_number,
                            );
                            multisig.add_attestation(attestation, stake_u128)
                                .map_err(|_| Error::<T>::InvalidCertificate)?;
                            *multisig_opt = Some(multisig);
                        }
                    }

                    Self::deposit_event(Event::CrossChainAttestation {
                        target_para,
                        target_block,
                    });

                    Ok(())
                },
            )
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // INTERNAL FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════════════

    impl<T: Config> Pallet<T> {
        /// Convert AccountId to CollatorId
        fn account_to_collator_id(account: &T::AccountId) -> CollatorId {
            let encoded = account.encode();
            let mut bytes = [0u8; 32];
            let len = encoded.len().min(32);
            bytes[..len].copy_from_slice(&encoded[..len]);
            CollatorId::from(bytes)
        }

        /// Update finality level for block
        fn update_finality(block_hash: H256) -> DispatchResult {
            // Count certificates
            let mut count = 0u32;
            for phase in [
                ConsensusPhase::Prepare,
                ConsensusPhase::PreCommit,
                ConsensusPhase::Commit,
                ConsensusPhase::Decide,
            ] {
                if Certificates::<T>::contains_key(block_hash, phase) {
                    count += 1;
                }
            }

            CertificateCounts::<T>::insert(block_hash, count);

            // Calculate local finality
            let local_finality = CollatorFinalityLevel::from(count);

            // Check for relay chain finality
            let combined_finality = if let Some(relay_proof) = RelayFinality::<T>::get(block_hash) {
                let inherited = relay_proof.inherit_finality();
                if inherited > local_finality {
                    inherited
                } else {
                    local_finality
                }
            } else {
                local_finality
            };

            FinalityLevels::<T>::insert(block_hash, combined_finality);

            Self::deposit_event(Event::FinalityUpdated {
                block_hash,
                finality_level: combined_finality,
            });

            Ok(())
        }

        /// Initialize committee (called from genesis)
        pub fn initialize_committee(
            initial_collators: Vec<(T::AccountId, BalanceOf<T>)>,
        ) -> DispatchResult {
            let mut committee = CollatorCommittee::new(
                T::ParaId::get(),
                T::MinCollators::get(),
                T::MaxCollators::get(),
            );

            for (account, stake) in initial_collators {
                let collator_id = Self::account_to_collator_id(&account);
                let stake_u128: u128 = TryInto::<u128>::try_into(stake).unwrap_or(0);

                committee.add_collator(collator_id, stake_u128)
                    .map_err(|_| Error::<T>::CommitteeFull)?;

                CollatorStakes::<T>::insert(&account, stake);
            }

            Committee::<T>::put(committee);
            Ok(())
        }
    }
}
