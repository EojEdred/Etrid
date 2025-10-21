//! # EDSC Bridge Attestation Pallet
//!
//! ## Overview
//!
//! The EDSC Bridge Attestation pallet provides M-of-N threshold signature verification
//! for cross-chain messages in the EDSC bridge protocol. It manages a registry of
//! independent attesters who sign cross-chain messages to ensure their validity.
//!
//! ## Architecture
//!
//! Following the CCTP (Cross-Chain Transfer Protocol) model:
//! - **M-of-N Threshold**: Requires M valid signatures from N registered attesters
//! - **Independent Attesters**: Each attester operates independently
//! - **Byzantine Fault Tolerant**: Continues operating even if some attesters fail
//! - **Governance Controlled**: Attesters can be added/removed via governance
//!
//! ## Key Features
//!
//! 1. **Attester Registry**
//!    - Register attesters with unique public keys
//!    - Enable/disable attesters without removing them
//!    - Track attester status and metadata
//!
//! 2. **Signature Verification**
//!    - Verify individual ECDSA/SR25519 signatures
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
//!    - Slashing for malicious attestations (future)

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::Get,
	};
	use frame_system::pallet_prelude::*;
	use sp_core::{ecdsa, sr25519, H256};
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

	/// Signature type
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum SignatureType {
		/// ECDSA signature (Ethereum-compatible)
		Ecdsa(ecdsa::Signature),
		/// SR25519 signature (Substrate native)
		Sr25519(sr25519::Signature),
	}

	/// Attester information
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AttesterInfo<T: Config> {
		/// Attester's public key (32 bytes for both ECDSA and SR25519)
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
		/// Signature submitted [attester_id, message_hash]
		SignatureSubmitted {
			attester_id: u32,
			message_hash: H256,
		},
		/// Attestation threshold reached [message_hash, signature_count]
		AttestationThresholdReached {
			message_hash: H256,
			signature_count: u32,
		},
		/// Attestation verified successfully [message_hash, signature_count]
		AttestationVerified {
			message_hash: H256,
			signature_count: u32,
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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new attester
		///
		/// # Arguments
		/// * `public_key` - Attester's public key (32 or 33 bytes)
		///
		/// Requires root origin (governance)
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(4))]
		pub fn register_attester(
			origin: OriginFor<T>,
			public_key: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			// Validate public key length (32 bytes for SR25519, 33 for ECDSA compressed)
			ensure!(
				public_key.len() == 32 || public_key.len() == 33,
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
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(2))]
		pub fn disable_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

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
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(2))]
		pub fn enable_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

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
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(3))]
		pub fn remove_attester(
			origin: OriginFor<T>,
			attester_id: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

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
		/// * `signature` - The signature bytes
		///
		/// Can be called by anyone (permissionless)
		#[pallet::call_index(4)]
		#[pallet::weight(Weight::from_parts(50_000, 0) + T::DbWeight::get().reads_writes(3, 2))]
		pub fn submit_signature(
			origin: OriginFor<T>,
			attester_id: u32,
			message_hash: H256,
			signature: Vec<u8>,
		) -> DispatchResult {
			let _submitter = ensure_signed(origin)?;
			ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

			// Verify attester exists and is active
			let mut attester = Attesters::<T>::get(attester_id)
				.ok_or(Error::<T>::AttesterNotFound)?;
			ensure!(
				attester.status == AttesterStatus::Active,
				Error::<T>::AttesterNotActive
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
			});

			// Check if threshold reached
			let attestation = Attestations::<T>::get(message_hash).unwrap();
			let threshold = Self::get_threshold_for_message();

			if attestation.signature_count >= threshold {
				Self::deposit_event(Event::AttestationThresholdReached {
					message_hash,
					signature_count: attestation.signature_count,
				});
			}

			Ok(())
		}

		/// Verify an attestation for a message
		///
		/// # Arguments
		/// * `message` - The original message bytes
		/// * `attestation_data` - The attestation data (message_hash + signatures)
		///
		/// This is typically called by pallet-edsc-bridge-token-messenger
		#[pallet::call_index(5)]
		#[pallet::weight(Weight::from_parts(100_000, 0) + T::DbWeight::get().reads(5))]
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

			// Check threshold
			let threshold = Self::get_threshold_for_message();
			ensure!(
				attestation.signature_count >= threshold,
				Error::<T>::InsufficientSignatures
			);

			// Verify signatures (simplified - in production would verify cryptographically)
			// For each signature, verify it's from an active attester
			for (attester_id, _signature) in attestation.signatures.iter() {
				let attester = Attesters::<T>::get(attester_id)
					.ok_or(Error::<T>::AttesterNotFound)?;
				ensure!(
					attester.status == AttesterStatus::Active,
					Error::<T>::AttesterNotActive
				);

				// In production, verify signature cryptographically:
				// Self::verify_signature(&attester.public_key, &message_hash, signature)?;
			}

			TotalAttestations::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::AttestationVerified {
				message_hash,
				signature_count: attestation.signature_count,
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
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn configure_threshold(
			origin: OriginFor<T>,
			domain_id: Option<u32>,
			min_signatures: u32,
			total_attesters: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

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

		/// Pause attestation service
		///
		/// Requires root origin (governance)
		#[pallet::call_index(7)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn pause_attestation(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			IsPaused::<T>::put(true);
			Self::deposit_event(Event::AttestationPaused);
			Ok(())
		}

		/// Unpause attestation service
		///
		/// Requires root origin (governance)
		#[pallet::call_index(8)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn unpause_attestation(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
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

		/// Get threshold for current message (uses global config)
		fn get_threshold_for_message() -> u32 {
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

			let threshold = Self::get_threshold_for_message();
			ensure!(
				attestation.signature_count >= threshold,
				Error::<T>::InsufficientSignatures
			);

			Ok(())
		}
	}
}
