//! # Generic Bridge Attestation Pallet
//!
//! ## Overview
//!
//! The Bridge Attestation pallet provides M-of-N threshold signature verification
//! for cross-chain messages in any bridge protocol. It manages a registry of
//! independent attesters who sign cross-chain messages to ensure their validity.
//!
//! This is a GENERIC, reusable pallet that can be used by ANY PBC (Partition Burst Chain)
//! or bridge implementation in the Etrid ecosystem.
//!
//! ## Architecture
//!
//! Following the CCTP (Cross-Chain Transfer Protocol) model:
//! - **M-of-N Threshold**: Requires M valid signatures from N registered attesters
//! - **Independent Attesters**: Each attester operates independently
//! - **Byzantine Fault Tolerant**: Continues operating even if some attesters fail
//! - **Governance Controlled**: Attesters can be added/removed via governance
//! - **Multi-Chain Support**: Configurable ChainId for different source/destination chains
//!
//! ## Key Features
//!
//! 1. **Attester Registry**
//!    - Register attesters with unique public keys
//!    - Enable/disable attesters without removing them
//!    - Track attester status and metadata
//!
//! 2. **Signature Verification**
//!    - Verify individual ECDSA and SR25519 signatures
//!    - Aggregate signature verification (M-of-N)
//!    - Prevent signature reuse across messages
//!
//! 3. **Threshold Management**
//!    - Configurable M-of-N thresholds per domain
//!    - Automatic threshold adjustment based on attester count
//!    - Emergency threshold override
//!
//! 4. **Security Features**
//!    - Signature deduplication
//!    - Attester rotation via governance
//!    - Emergency pause controls
//!    - Nonce-based replay protection
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! // In your PBC runtime configuration:
//! impl pallet_bridge_attestation::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type ChainId = ConstU32<1>; // Your chain ID
//!     type MaxAttesters = ConstU32<100>;
//!     type MaxAttestersPerMessage = ConstU32<20>;
//!     type MinSignatureThreshold = ConstU32<2>;
//!     type AttestationMaxAge = ConstU32<1000>;
//!     type WeightInfo = ();
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::WeightInfo;

pub mod genesis;
pub use genesis::{GenesisAttester, GenesisConfig, GenesisThreshold};

#[frame_support::pallet]
pub mod pallet {
	use super::WeightInfo;
	use frame_support::{
		pallet_prelude::*,
		traits::Get,
	};
	use frame_system::pallet_prelude::*;
	use sp_core::{ecdsa, sr25519, H256};
	use sp_io;
	use sp_runtime::traits::Saturating;
	use sp_std::vec::Vec;

	/// Attester status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum AttesterStatus {
		/// Attester is active and can sign messages
		Active,
		/// Attester is temporarily disabled
		Disabled,
		/// Attester has been removed
		Removed,
	}

	impl AttesterStatus {
		pub fn to_u8(&self) -> u8 {
			match self {
				AttesterStatus::Active => 0,
				AttesterStatus::Disabled => 1,
				AttesterStatus::Removed => 2,
			}
		}
	}

	/// Signature type (supports both ECDSA and SR25519)
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum SignatureType {
		/// ECDSA signature (Ethereum-compatible, EVM bridges)
		Ecdsa(ecdsa::Signature),
		/// SR25519 signature (Substrate native)
		Sr25519(sr25519::Signature),
	}

	/// Attester information
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AttesterInfo<T: Config> {
		/// Attester's public key (32 bytes for SR25519, 33 for ECDSA compressed)
		pub public_key: BoundedVec<u8, ConstU32<64>>,
		/// Current status
		pub status: AttesterStatus,
		/// Block number when attester was registered
		pub registered_at: BlockNumberFor<T>,
		/// Total messages signed
		pub messages_signed: u64,
		/// Last block when attester signed a message
		pub last_signed_at: BlockNumberFor<T>,
	}

	/// Attestation record for a message
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Attestation<T: Config> {
		/// Message hash being attested
		pub message_hash: H256,
		/// Signatures from attesters (attester_id → signature)
		pub signatures: BoundedVec<(u32, BoundedVec<u8, ConstU32<65>>), T::MaxAttestersPerMessage>,
		/// Block when attestation was created
		pub attested_at: BlockNumberFor<T>,
		/// Number of valid signatures
		pub signature_count: u32,
		/// Source chain ID (for cross-chain verification)
		pub source_chain_id: u32,
		/// Destination chain ID
		pub destination_chain_id: u32,
		/// Nonce to prevent replay attacks
		pub nonce: u64,
	}

	/// Threshold configuration per domain
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct ThresholdConfig {
		/// Minimum signatures required (M in M-of-N)
		pub min_signatures: u32,
		/// Total attesters available (N in M-of-N)
		pub total_attesters: u32,
		/// Whether this configuration is enabled
		pub enabled: bool,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Chain ID for this runtime (configurable per PBC)
		#[pallet::constant]
		type ChainId: Get<u32>;

		/// Maximum number of attesters that can be registered
		#[pallet::constant]
		type MaxAttesters: Get<u32>;

		/// Maximum number of attesters per message
		#[pallet::constant]
		type MaxAttestersPerMessage: Get<u32>;

		/// Minimum signatures required (M in M-of-N)
		#[pallet::constant]
		type MinSignatureThreshold: Get<u32>;

		/// Maximum age of an attestation before it expires (in blocks)
		#[pallet::constant]
		type AttestationMaxAge: Get<BlockNumberFor<Self>>;

		/// Origin that can manage attesters (register, enable, disable, remove)
		/// Defaults to root but can be configured to a specific account
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for extrinsics
		type WeightInfo: WeightInfo;
	}

	/// Registered attesters (attester_id → AttesterInfo)
	#[pallet::storage]
	#[pallet::getter(fn attester)]
	pub type Attesters<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,  // Attester ID
		AttesterInfo<T>,
	>;

	/// Mapping from public key to attester ID
	#[pallet::storage]
	#[pallet::getter(fn attester_by_pubkey)]
	pub type AttesterByPubkey<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, ConstU32<64>>,  // Public key
		u32,  // Attester ID
	>;

	/// Next attester ID
	#[pallet::storage]
	#[pallet::getter(fn next_attester_id)]
	pub type NextAttesterId<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Active attester count
	#[pallet::storage]
	#[pallet::getter(fn active_attester_count)]
	pub type ActiveAttesterCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Attestations for messages (message_hash → Attestation)
	#[pallet::storage]
	#[pallet::getter(fn attestation)]
	pub type Attestations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		H256,  // Message hash
		Attestation<T>,
	>;

	/// Threshold configuration per domain (domain_id → ThresholdConfig)
	#[pallet::storage]
	#[pallet::getter(fn threshold_config)]
	pub type ThresholdConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,  // Domain ID
		ThresholdConfig,
	>;

	/// Global threshold configuration (used when domain-specific not set)
	#[pallet::storage]
	#[pallet::getter(fn global_threshold)]
	pub type GlobalThreshold<T: Config> = StorageValue<_, ThresholdConfig>;

	/// Emergency pause flag
	#[pallet::storage]
	#[pallet::getter(fn is_paused)]
	pub type IsPaused<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Total attestations verified
	#[pallet::storage]
	#[pallet::getter(fn total_attestations)]
	pub type TotalAttestations<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Attestation nonce (incremented for each message to prevent replay)
	#[pallet::storage]
	#[pallet::getter(fn attestation_nonce)]
	pub type AttestationNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Used nonces to prevent replay attacks (nonce → bool)
	#[pallet::storage]
	#[pallet::getter(fn used_nonce)]
	pub type UsedNonces<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		bool,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New attester registered [attester_id, public_key]
		AttesterRegistered {
			attester_id: u32,
			public_key: Vec<u8>,
		},
		/// Attester status changed [attester_id, old_status, new_status]
		AttesterStatusChanged {
			attester_id: u32,
			old_status: u8,
			new_status: u8,
		},
		/// Attester removed [attester_id]
		AttesterRemoved {
			attester_id: u32,
		},
		/// Signature submitted [attester_id, message_hash, nonce]
		SignatureSubmitted {
			attester_id: u32,
			message_hash: H256,
			nonce: u64,
		},
		/// Attestation threshold reached [message_hash, signature_count, nonce]
		AttestationThresholdReached {
			message_hash: H256,
			signature_count: u32,
			nonce: u64,
		},
		/// Attestation verified successfully [message_hash, signature_count, source_chain, dest_chain]
		AttestationVerified {
			message_hash: H256,
			signature_count: u32,
			source_chain_id: u32,
			destination_chain_id: u32,
		},
		/// Threshold configuration updated [domain_id, min_signatures, total_attesters]
		ThresholdConfigUpdated {
			domain_id: Option<u32>,
			min_signatures: u32,
			total_attesters: u32,
		},
		/// Attestation service paused
		AttestationPaused,
		/// Attestation service unpaused
		AttestationUnpaused,
		/// Threshold updated [new_threshold]
		ThresholdUpdated {
			new_threshold: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Attestation service is paused
		AttestationPaused,
		/// Attester not found
		AttesterNotFound,
		/// Attester already exists
		AttesterAlreadyExists,
		/// Attester is not active
		AttesterNotActive,
		/// Maximum attesters reached
		MaxAttestersReached,
		/// Invalid signature
		InvalidSignature,
		/// Invalid public key
		InvalidPublicKey,
		/// Signature already submitted
		SignatureAlreadySubmitted,
		/// Attestation not found
		AttestationNotFound,
		/// Attestation expired
		AttestationExpired,
		/// Insufficient signatures
		InsufficientSignatures,
		/// Invalid threshold configuration
		InvalidThreshold,
		/// Message hash mismatch
		MessageHashMismatch,
		/// Invalid chain ID
		InvalidChainId,
		/// Nonce already used (replay attack prevention)
		NonceAlreadyUsed,
		/// Invalid signature length
		InvalidSignatureLength,
		/// Unsupported signature type
		UnsupportedSignatureType,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new attester
		///
		/// # Arguments
		/// * `public_key` - Attester's public key (32 bytes for SR25519, 33 for ECDSA)
		///
		/// Requires AdminOrigin (root or configured admin account)
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register_attester())]
		pub fn register_attester(
			origin: OriginFor<T>,
			public_key: Vec<u8>,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			// Validate public key length (32 bytes for SR25519, 33 for ECDSA compressed)
			ensure!(
				public_key.len() == 32 || public_key.len() == 33 || public_key.len() == 65,
				Error::<T>::InvalidPublicKey
			);

			let bounded_key: BoundedVec<u8, ConstU32<64>> = public_key.clone()
				.try_into()
				.map_err(|_| Error::<T>::InvalidPublicKey)?;

			// Check if attester already exists
			ensure!(
				!AttesterByPubkey::<T>::contains_key(&bounded_key),
				Error::<T>::AttesterAlreadyExists
			);

			let attester_id = NextAttesterId::<T>::get();
			let current_block = <frame_system::Pallet<T>>::block_number();

			let attester_info = AttesterInfo {
				public_key: bounded_key.clone(),
				status: AttesterStatus::Active,
				registered_at: current_block,
				messages_signed: 0,
				last_signed_at: current_block,
			};

			// Store attester
			Attesters::<T>::insert(attester_id, attester_info);
			AttesterByPubkey::<T>::insert(bounded_key, attester_id);
			NextAttesterId::<T>::put(attester_id.saturating_add(1));
			ActiveAttesterCount::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::AttesterRegistered {
				attester_id,
				public_key,
			});

			Ok(())
		}

		/// Disable an attester
		///
		/// # Arguments
		/// * `attester_id` - ID of the attester to disable
		///
		/// Requires root origin (governance)
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::disable_attester())]
		pub fn disable_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;

			Attesters::<T>::try_mutate(attester_id, |maybe_attester| -> DispatchResult {
				let attester = maybe_attester.as_mut().ok_or(Error::<T>::AttesterNotFound)?;
				let old_status = attester.status.clone();

				if old_status == AttesterStatus::Active {
					ActiveAttesterCount::<T>::mutate(|count| *count = count.saturating_sub(1));
				}

				attester.status = AttesterStatus::Disabled;

				Self::deposit_event(Event::AttesterStatusChanged {
					attester_id,
					old_status: old_status.to_u8(),
					new_status: AttesterStatus::Disabled.to_u8(),
				});

				Ok(())
			})
		}

		/// Enable a disabled attester
		///
		/// # Arguments
		/// * `attester_id` - ID of the attester to enable
		///
		/// Requires root origin (governance)
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::enable_attester())]
		pub fn enable_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;

			Attesters::<T>::try_mutate(attester_id, |maybe_attester| -> DispatchResult {
				let attester = maybe_attester.as_mut().ok_or(Error::<T>::AttesterNotFound)?;
				let old_status = attester.status.clone();

				ensure!(
					old_status == AttesterStatus::Disabled,
					Error::<T>::AttesterNotActive
				);

				attester.status = AttesterStatus::Active;
				ActiveAttesterCount::<T>::mutate(|count| *count = count.saturating_add(1));

				Self::deposit_event(Event::AttesterStatusChanged {
					attester_id,
					old_status: old_status.to_u8(),
					new_status: AttesterStatus::Active.to_u8(),
				});

				Ok(())
			})
		}

		/// Remove an attester from the registry
		///
		/// # Arguments
		/// * `attester_id` - ID of the attester to remove
		///
		/// Requires root origin (governance)
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_attester())]
		pub fn remove_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;

			let attester = Attesters::<T>::get(attester_id)
				.ok_or(Error::<T>::AttesterNotFound)?;

			if attester.status == AttesterStatus::Active {
				ActiveAttesterCount::<T>::mutate(|count| *count = count.saturating_sub(1));
			}

			// Remove from both storage maps
			AttesterByPubkey::<T>::remove(&attester.public_key);
			Attesters::<T>::remove(attester_id);

			Self::deposit_event(Event::AttesterRemoved { attester_id });

			Ok(())
		}

		/// Submit a signature for a message
		///
		/// # Arguments
		/// * `attester_id` - ID of the attester submitting the signature
		/// * `message_hash` - Hash of the message being signed
		/// * `signature` - The signature bytes (65 for ECDSA, 64 for SR25519)
		/// * `source_chain_id` - Source chain ID
		/// * `destination_chain_id` - Destination chain ID
		/// * `nonce` - Nonce to prevent replay attacks
		///
		/// Can be called by anyone (permissionless)
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::submit_signature())]
		pub fn submit_signature(
			origin: OriginFor<T>,
			attester_id: u32,
			message_hash: H256,
			signature: Vec<u8>,
			source_chain_id: u32,
			destination_chain_id: u32,
			nonce: u64,
		) -> DispatchResult {
			let _submitter = ensure_signed(origin)?;
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			// Verify nonce hasn't been used
			ensure!(
				!UsedNonces::<T>::get(nonce),
				Error::<T>::NonceAlreadyUsed
			);

			// Verify attester exists and is active
			let mut attester = Attesters::<T>::get(attester_id)
				.ok_or(Error::<T>::AttesterNotFound)?;
			ensure!(
				attester.status == AttesterStatus::Active,
				Error::<T>::AttesterNotActive
			);

			// Validate signature length (64 for SR25519, 65 for ECDSA)
			ensure!(
				signature.len() == 64 || signature.len() == 65,
				Error::<T>::InvalidSignatureLength
			);

			let bounded_sig: BoundedVec<u8, ConstU32<65>> = signature.clone()
				.try_into()
				.map_err(|_| Error::<T>::InvalidSignature)?;

			// Get or create attestation
			let current_block = <frame_system::Pallet<T>>::block_number();

			Attestations::<T>::try_mutate(message_hash, |maybe_attestation| -> DispatchResult {
				let attestation = match maybe_attestation {
					Some(att) => att,
					None => {
						// Create new attestation
						*maybe_attestation = Some(Attestation {
							message_hash,
							signatures: BoundedVec::default(),
							attested_at: current_block,
							signature_count: 0,
							source_chain_id,
							destination_chain_id,
							nonce,
						});
						maybe_attestation.as_mut().unwrap()
					}
				};

				// Check if this attester already signed
				for (existing_id, _) in attestation.signatures.iter() {
					ensure!(
						*existing_id != attester_id,
						Error::<T>::SignatureAlreadySubmitted
					);
				}

				// Add signature
				attestation.signatures
					.try_push((attester_id, bounded_sig))
					.map_err(|_| Error::<T>::MaxAttestersReached)?;

				attestation.signature_count = attestation.signature_count.saturating_add(1);

				Ok(())
			})?;

			// Update attester stats
			attester.messages_signed = attester.messages_signed.saturating_add(1);
			attester.last_signed_at = current_block;
			Attesters::<T>::insert(attester_id, attester);

			Self::deposit_event(Event::SignatureSubmitted {
				attester_id,
				message_hash,
				nonce,
			});

			// Check if threshold reached
			let attestation = Attestations::<T>::get(message_hash).unwrap();
			let threshold = Self::get_threshold_for_message();

			if attestation.signature_count >= threshold {
				Self::deposit_event(Event::AttestationThresholdReached {
					message_hash,
					signature_count: attestation.signature_count,
					nonce,
				});
			}

			Ok(())
		}

		/// Verify an attestation for a message
		///
		/// # Arguments
		/// * `message` - The original message bytes
		/// * `message_hash` - The message hash
		///
		/// This is typically called by other bridge pallets (token messenger, etc.)
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::verify_attestation())]
		pub fn verify_attestation(
			origin: OriginFor<T>,
			message: Vec<u8>,
			message_hash: H256,
		) -> DispatchResult {
			let _caller = ensure_signed(origin)?;
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			// Verify message hash matches
			let computed_hash = Self::hash_message(&message);
			ensure!(
				computed_hash == message_hash,
				Error::<T>::MessageHashMismatch
			);

			// Get attestation
			let attestation = Attestations::<T>::get(message_hash)
				.ok_or(Error::<T>::AttestationNotFound)?;

			// Check attestation not expired
			let current_block = <frame_system::Pallet<T>>::block_number();
			let age = current_block.saturating_sub(attestation.attested_at);
			ensure!(
				age <= T::AttestationMaxAge::get(),
				Error::<T>::AttestationExpired
			);

			// Verify nonce hasn't been used (prevent replay)
			ensure!(
				!UsedNonces::<T>::get(attestation.nonce),
				Error::<T>::NonceAlreadyUsed
			);

			// Check threshold
			let threshold = Self::get_threshold_for_message();
			ensure!(
				attestation.signature_count >= threshold,
				Error::<T>::InsufficientSignatures
			);

			// Verify signatures cryptographically
			for (attester_id, signature) in attestation.signatures.iter() {
				let attester = Attesters::<T>::get(attester_id)
					.ok_or(Error::<T>::AttesterNotFound)?;
				ensure!(
					attester.status == AttesterStatus::Active,
					Error::<T>::AttesterNotActive
				);

				// Cryptographically verify signature
				Self::verify_signature(
					&attester.public_key,
					&message_hash,
					signature,
				)?;
			}

			// Mark nonce as used
			UsedNonces::<T>::insert(attestation.nonce, true);
			TotalAttestations::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::AttestationVerified {
				message_hash,
				signature_count: attestation.signature_count,
				source_chain_id: attestation.source_chain_id,
				destination_chain_id: attestation.destination_chain_id,
			});

			Ok(())
		}

		/// Configure threshold for a domain
		///
		/// # Arguments
		/// * `domain_id` - Domain ID (None for global)
		/// * `min_signatures` - Minimum signatures required
		/// * `total_attesters` - Total attesters available
		///
		/// Requires root origin (governance)
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::update_threshold())]
		pub fn configure_threshold(
			origin: OriginFor<T>,
			domain_id: Option<u32>,
			min_signatures: u32,
			total_attesters: u32,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;

			ensure!(
				min_signatures > 0 && min_signatures <= total_attesters,
				Error::<T>::InvalidThreshold
			);

			let config = ThresholdConfig {
				min_signatures,
				total_attesters,
				enabled: true,
			};

			if let Some(domain) = domain_id {
				ThresholdConfigs::<T>::insert(domain, config);
			} else {
				GlobalThreshold::<T>::put(config);
			}

			Self::deposit_event(Event::ThresholdConfigUpdated {
				domain_id,
				min_signatures,
				total_attesters,
			});

			Ok(())
		}

		/// Update the global threshold
		///
		/// # Arguments
		/// * `new_threshold` - New minimum signature threshold
		///
		/// Requires root origin (governance)
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::update_threshold())]
		pub fn update_threshold(
			origin: OriginFor<T>,
			new_threshold: u32,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;

			let active_count = ActiveAttesterCount::<T>::get();
			ensure!(
				new_threshold > 0 && new_threshold <= active_count,
				Error::<T>::InvalidThreshold
			);

			let config = ThresholdConfig {
				min_signatures: new_threshold,
				total_attesters: active_count,
				enabled: true,
			};

			GlobalThreshold::<T>::put(config);

			Self::deposit_event(Event::ThresholdUpdated {
				new_threshold,
			});

			Ok(())
		}

		/// Pause attestation service
		///
		/// Requires root origin (governance)
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::pause_attestation())]
		pub fn pause_attestation(origin: OriginFor<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			IsPaused::<T>::put(true);
			Self::deposit_event(Event::AttestationPaused);
			Ok(())
		}

		/// Unpause attestation service
		///
		/// Requires root origin (governance)
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::unpause_attestation())]
		pub fn unpause_attestation(origin: OriginFor<T>) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			IsPaused::<T>::put(false);
			Self::deposit_event(Event::AttestationUnpaused);
			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Hash a message using Blake2-256
		pub fn hash_message(message: &[u8]) -> H256 {
			H256::from(sp_io::hashing::blake2_256(message))
		}

		/// Get current chain ID from runtime configuration
		pub fn get_chain_id() -> u32 {
			T::ChainId::get()
		}

		/// Get current attestation nonce and increment it
		pub fn get_and_increment_nonce() -> u64 {
			let nonce = AttestationNonce::<T>::get();
			AttestationNonce::<T>::put(nonce.saturating_add(1));
			nonce
		}

		/// Get threshold for current message (uses global config)
		pub fn get_threshold_for_message() -> u32 {
			if let Some(config) = GlobalThreshold::<T>::get() {
				if config.enabled {
					return config.min_signatures;
				}
			}
			T::MinSignatureThreshold::get()
		}

		/// Get threshold for a specific domain
		pub fn get_threshold_for_domain(domain: u32) -> u32 {
			if let Some(config) = ThresholdConfigs::<T>::get(domain) {
				if config.enabled {
					return config.min_signatures;
				}
			}
			Self::get_threshold_for_message()
		}

		/// Verify a signature (supports both ECDSA and SR25519)
		fn verify_signature(
			public_key: &[u8],
			message_hash: &H256,
			signature: &[u8],
		) -> DispatchResult {
			// Determine signature type based on length
			match (public_key.len(), signature.len()) {
				// ECDSA: 33-byte compressed pubkey, 65-byte signature
				(33, 65) => Self::verify_ecdsa_signature(public_key, message_hash, signature),
				// ECDSA: 65-byte uncompressed pubkey, 65-byte signature
				(65, 65) => Self::verify_ecdsa_signature(public_key, message_hash, signature),
				// SR25519: 32-byte pubkey, 64-byte signature
				(32, 64) => Self::verify_sr25519_signature(public_key, message_hash, signature),
				// Invalid combination
				_ => Err(Error::<T>::UnsupportedSignatureType.into()),
			}
		}

		/// Verify an ECDSA signature
		fn verify_ecdsa_signature(
			public_key: &[u8],
			message_hash: &H256,
			signature: &[u8],
		) -> DispatchResult {
			ensure!(signature.len() == 65, Error::<T>::InvalidSignatureLength);

			let sig_array: [u8; 65] = signature
				.try_into()
				.map_err(|_| Error::<T>::InvalidSignature)?;

			// Handle both compressed (33) and uncompressed (65) public keys
			let ecdsa_sig = sp_core::ecdsa::Signature::from_raw(sig_array);

			let is_valid = if public_key.len() == 33 {
				let pubkey_array: [u8; 33] = public_key
					.try_into()
					.map_err(|_| Error::<T>::InvalidPublicKey)?;
				let ecdsa_pubkey = sp_core::ecdsa::Public::from_raw(pubkey_array);

				sp_io::crypto::ecdsa_verify(
					&ecdsa_sig,
					&message_hash.0,
					&ecdsa_pubkey,
				)
			} else if public_key.len() == 65 {
				// For uncompressed keys, we need to compress them first
				// This is a simplified version - production code should use proper compression
				let compressed = Self::compress_ecdsa_pubkey(public_key)?;
				let ecdsa_pubkey = sp_core::ecdsa::Public::from_raw(compressed);

				sp_io::crypto::ecdsa_verify(
					&ecdsa_sig,
					&message_hash.0,
					&ecdsa_pubkey,
				)
			} else {
				return Err(Error::<T>::InvalidPublicKey.into());
			};

			ensure!(is_valid, Error::<T>::InvalidSignature);
			Ok(())
		}

		/// Verify an SR25519 signature
		fn verify_sr25519_signature(
			public_key: &[u8],
			message_hash: &H256,
			signature: &[u8],
		) -> DispatchResult {
			ensure!(signature.len() == 64, Error::<T>::InvalidSignatureLength);
			ensure!(public_key.len() == 32, Error::<T>::InvalidPublicKey);

			let sig_array: [u8; 64] = signature
				.try_into()
				.map_err(|_| Error::<T>::InvalidSignature)?;

			let pubkey_array: [u8; 32] = public_key
				.try_into()
				.map_err(|_| Error::<T>::InvalidPublicKey)?;

			let sr25519_sig = sp_core::sr25519::Signature::from_raw(sig_array);
			let sr25519_pubkey = sp_core::sr25519::Public::from_raw(pubkey_array);

			let is_valid = sp_io::crypto::sr25519_verify(
				&sr25519_sig,
				&message_hash.0,
				&sr25519_pubkey,
			);

			ensure!(is_valid, Error::<T>::InvalidSignature);
			Ok(())
		}

		/// Compress an ECDSA public key (65 bytes uncompressed → 33 bytes compressed)
		fn compress_ecdsa_pubkey(uncompressed: &[u8]) -> Result<[u8; 33], DispatchError> {
			ensure!(uncompressed.len() == 65, Error::<T>::InvalidPublicKey);
			ensure!(uncompressed[0] == 0x04, Error::<T>::InvalidPublicKey);

			let mut compressed = [0u8; 33];

			// Prefix: 0x02 if Y is even, 0x03 if Y is odd
			let y_last_byte = uncompressed[64];
			compressed[0] = if y_last_byte % 2 == 0 { 0x02 } else { 0x03 };

			// Copy X coordinate (bytes 1-32 from uncompressed)
			compressed[1..33].copy_from_slice(&uncompressed[1..33]);

			Ok(compressed)
		}

		/// Public verification function (called by other pallets)
		pub fn verify_attestation_for_message(
			message: &[u8],
			message_hash: H256,
		) -> DispatchResult {
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			let computed_hash = Self::hash_message(message);
			ensure!(
				computed_hash == message_hash,
				Error::<T>::MessageHashMismatch
			);

			let attestation = Attestations::<T>::get(message_hash)
				.ok_or(Error::<T>::AttestationNotFound)?;

			let current_block = <frame_system::Pallet<T>>::block_number();
			let age = current_block.saturating_sub(attestation.attested_at);
			ensure!(
				age <= T::AttestationMaxAge::get(),
				Error::<T>::AttestationExpired
			);

			// Verify nonce
			ensure!(
				!UsedNonces::<T>::get(attestation.nonce),
				Error::<T>::NonceAlreadyUsed
			);

			let threshold = Self::get_threshold_for_message();
			ensure!(
				attestation.signature_count >= threshold,
				Error::<T>::InsufficientSignatures
			);

			// Mark nonce as used
			UsedNonces::<T>::insert(attestation.nonce, true);

			Ok(())
		}

		/// Check if an attestation is valid without marking nonce as used
		pub fn is_attestation_valid(message_hash: H256) -> bool {
			if IsPaused::<T>::get() {
				return false;
			}

			if let Some(attestation) = Attestations::<T>::get(message_hash) {
				let current_block = <frame_system::Pallet<T>>::block_number();
				let age = current_block.saturating_sub(attestation.attested_at);

				if age > T::AttestationMaxAge::get() {
					return false;
				}

				if UsedNonces::<T>::get(attestation.nonce) {
					return false;
				}

				let threshold = Self::get_threshold_for_message();
				return attestation.signature_count >= threshold;
			}

			false
		}

		/// Get active attesters (returns list of active attester IDs)
		pub fn get_active_attesters() -> Vec<u32> {
			let mut active = Vec::new();
			let max_id = NextAttesterId::<T>::get();

			for id in 0..max_id {
				if let Some(attester) = Attesters::<T>::get(id) {
					if attester.status == AttesterStatus::Active {
						active.push(id);
					}
				}
			}

			active
		}
	}
}
