//! # EDSC Bridge Token Messenger Pallet
//!
//! CCTP-style cross-chain token messenger for EDSC transfers
//!
//! ## Overview
//!
//! This pallet implements the main interface for cross-chain EDSC transfers using a
//! burn-and-mint architecture similar to Circle's CCTP (Cross-Chain Transfer Protocol).
//!
//! ### Key Features:
//! - Burn EDSC on source chain, mint on destination chain
//! - Nonce-based message ordering prevents replay attacks
//! - Domain registry for supported external blockchains
//! - Rate limiting and safety controls
//! - Attestation-based security model
//!
//! ## Message Flow
//!
//! ### Outbound (Ëtrid → External Chain):
//! ```
//! User → burn_edsc_for_external_chain()
//!     ├─ Burn EDSC tokens
//!     ├─ Create CrossChainMessage
//!     ├─ Emit BurnMessageSent event
//!     └─ Attesters sign message off-chain
//!         └─ Relayer delivers to external chain
//! ```
//!
//! ### Inbound (External Chain → Ëtrid):
//! ```
//! Relayer → receive_and_mint()
//!     ├─ Verify attestation signatures
//!     ├─ Check nonce not used
//!     ├─ Parse message
//!     └─ Mint EDSC to recipient
//! ```

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
	use sp_runtime::traits::Saturating;
	use sp_core::H256;
	use sp_std::vec::Vec;

	/// Domain identifier for different blockchains
	#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Domain {
		/// Ethereum mainnet
		Ethereum = 0,
		/// Solana mainnet
		Solana = 1,
		/// Ëtrid PBC-EDSC (native)
		Etrid = 2,
		/// Polygon
		Polygon = 3,
		/// BNB Chain
		BnbChain = 4,
		/// Avalanche
		Avalanche = 5,
		/// Arbitrum
		Arbitrum = 6,
		/// Optimism
		Optimism = 7,
	}

	impl Domain {
		/// Convert Domain to u32 for message encoding
		pub fn to_u32(&self) -> u32 {
			*self as u32
		}

		/// Convert u32 to Domain
		pub fn from_u32(value: u32) -> Option<Self> {
			match value {
				0 => Some(Domain::Ethereum),
				1 => Some(Domain::Solana),
				2 => Some(Domain::Etrid),
				3 => Some(Domain::Polygon),
				4 => Some(Domain::BnbChain),
				5 => Some(Domain::Avalanche),
				6 => Some(Domain::Arbitrum),
				7 => Some(Domain::Optimism),
				_ => None,
			}
		}
	}

	/// Cross-chain message format (CCTP-style)
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CrossChainMessage {
		/// Message format version
		pub version: u32,
		/// Source domain
		pub source_domain: u32,
		/// Destination domain
		pub destination_domain: u32,
		/// Unique nonce
		pub nonce: u64,
		/// Sender address (flexible format for different chains)
		pub sender: BoundedVec<u8, ConstU32<64>>,
		/// Recipient address
		pub recipient: BoundedVec<u8, ConstU32<64>>,
		/// Message body (contains burn/mint details)
		pub message_body: BoundedVec<u8, ConstU32<512>>,
	}

	/// Burn message body
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct BurnMessage {
		/// Version
		pub version: u32,
		/// Token being burned (EDSC contract address on source chain)
		pub burn_token: BoundedVec<u8, ConstU32<64>>,
		/// Mint recipient on destination
		pub mint_recipient: BoundedVec<u8, ConstU32<64>>,
		/// Amount (with 18 decimals)
		pub amount: u128,
	}

	/// Message status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum MessageStatus {
		/// Message sent, awaiting attestation
		Sent,
		/// Attestation received, ready to process
		Attested,
		/// Message processed and minted
		Completed,
		/// Message failed
		Failed,
	}

	impl Default for MessageStatus {
		fn default() -> Self {
			MessageStatus::Sent
		}
	}

	/// Domain configuration
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DomainConfig {
		/// Is this domain enabled?
		pub enabled: bool,
		/// Maximum burn amount per transaction
		pub max_burn_amount: u128,
		/// Daily burn limit
		pub daily_burn_limit: u128,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum message body size
		#[pallet::constant]
		type MaxMessageBodySize: Get<u32>;

		/// Maximum burn amount per transaction (default limit)
		#[pallet::constant]
		type MaxBurnAmount: Get<u128>;

		/// Daily burn cap across all domains
		#[pallet::constant]
		type DailyBurnCap: Get<u128>;

		/// Message timeout (blocks)
		#[pallet::constant]
		type MessageTimeout: Get<BlockNumberFor<Self>>;
	}

	/// Outbound messages (burned on Ëtrid, awaiting attestation)
	#[pallet::storage]
	#[pallet::getter(fn outbound_messages)]
	pub type OutboundMessages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Nonce
		CrossChainMessage,
		OptionQuery,
	>;

	/// Message nonce counter (per destination domain)
	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub type Nonce<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain
		u64, // Nonce
		ValueQuery,
	>;

	/// Used nonces for inbound messages (prevents replay)
	#[pallet::storage]
	#[pallet::getter(fn used_nonces)]
	pub type UsedNonces<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, // Source domain
		Blake2_128Concat,
		u64, // Nonce
		bool,
		ValueQuery,
	>;

	/// Domain configurations
	#[pallet::storage]
	#[pallet::getter(fn domain_config)]
	pub type DomainConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain ID
		DomainConfig,
		OptionQuery,
	>;

	/// Daily burn volume tracking (per domain)
	#[pallet::storage]
	#[pallet::getter(fn daily_burn_volume)]
	pub type DailyBurnVolume<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain
		(BlockNumberFor<T>, u128), // (last_reset_block, volume)
		ValueQuery,
	>;

	/// Total outbound messages sent
	#[pallet::storage]
	#[pallet::getter(fn total_sent)]
	pub type TotalSent<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Total inbound messages received
	#[pallet::storage]
	#[pallet::getter(fn total_received)]
	pub type TotalReceived<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Bridge pause status (emergency)
	#[pallet::storage]
	#[pallet::getter(fn is_paused)]
	pub type IsPaused<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Burn message sent [nonce, destination_domain, amount, recipient]
		BurnMessageSent {
			nonce: u64,
			destination_domain: u32,
			amount: u128,
			recipient: Vec<u8>,
		},
		/// Message received and minted [source_domain, nonce, amount, recipient]
		MessageReceived {
			source_domain: u32,
			nonce: u64,
			amount: u128,
			recipient: Vec<u8>,
		},
		/// Domain configuration updated [domain, enabled]
		DomainConfigured {
			domain: u32,
			enabled: bool,
		},
		/// Bridge paused
		BridgePaused,
		/// Bridge unpaused
		BridgeUnpaused,
		/// Daily limit exceeded
		DailyLimitExceeded {
			domain: u32,
			attempted: u128,
			limit: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Bridge is paused
		BridgePaused,
		/// Invalid domain
		InvalidDomain,
		/// Domain not enabled
		DomainNotEnabled,
		/// Amount exceeds maximum
		AmountExceedsMax,
		/// Daily limit exceeded
		DailyLimitExceeded,
		/// Message already processed
		MessageAlreadyProcessed,
		/// Invalid message format
		InvalidMessageFormat,
		/// Invalid recipient address
		InvalidRecipient,
		/// Nonce mismatch
		NonceMismatch,
		/// Message too large
		MessageTooLarge,
		/// Attestation verification failed
		AttestationFailed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Clean up expired messages
			// Reset daily limits if needed
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Burn EDSC for transfer to external chain
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn burn_edsc_for_external_chain(
			origin: OriginFor<T>,
			destination_domain: u32,
			amount: u128,
			recipient: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check bridge not paused
			ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

			// Validate domain
			let domain_config = DomainConfigs::<T>::get(destination_domain)
				.ok_or(Error::<T>::InvalidDomain)?;
			ensure!(domain_config.enabled, Error::<T>::DomainNotEnabled);

			// Check amount limits
			ensure!(
				amount <= domain_config.max_burn_amount,
				Error::<T>::AmountExceedsMax
			);

			// Check daily limit
			Self::check_and_update_daily_limit(destination_domain, amount)?;

			// Get next nonce
			let nonce = Nonce::<T>::get(destination_domain);
			Nonce::<T>::insert(destination_domain, nonce.saturating_add(1));

			// Create burn message
			let burn_msg = BurnMessage {
				version: 1,
				burn_token: b"EDSC".to_vec().try_into().unwrap_or_default(),
				mint_recipient: recipient.clone().try_into().unwrap_or_default(),
				amount,
			};

			// Create cross-chain message
			let message = CrossChainMessage {
				version: 1,
				source_domain: Domain::Etrid.to_u32(),
				destination_domain,
				nonce,
				sender: Self::account_to_bytes(&sender),
				recipient: recipient.clone().try_into().unwrap_or_default(),
				message_body: burn_msg.encode().try_into()
					.map_err(|_| Error::<T>::MessageTooLarge)?,
			};

			// Store outbound message
			OutboundMessages::<T>::insert(nonce, message);
			TotalSent::<T>::mutate(|count| *count = count.saturating_add(1));

			// Emit event
			Self::deposit_event(Event::BurnMessageSent {
				nonce,
				destination_domain,
				amount,
				recipient,
			});

			// In production, this would call pallet_edsc_token::burn()
			// For now, just mark as sent

			Ok(())
		}

		/// Receive message and mint EDSC (called by relayers with attestation)
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn receive_and_mint(
			origin: OriginFor<T>,
			message: Vec<u8>,
			attestation: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			// Check bridge not paused
			ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

			// Decode message
			let cross_chain_msg = CrossChainMessage::decode(&mut &message[..])
				.map_err(|_| Error::<T>::InvalidMessageFormat)?;

			// Verify destination is Ëtrid
			ensure!(
				cross_chain_msg.destination_domain == Domain::Etrid.to_u32(),
				Error::<T>::InvalidDomain
			);

			// Check nonce not already used
			ensure!(
				!UsedNonces::<T>::get(cross_chain_msg.source_domain, cross_chain_msg.nonce),
				Error::<T>::MessageAlreadyProcessed
			);

			// Verify attestation (calls pallet_edsc_bridge_attestation)
			// For now, placeholder
			Self::verify_attestation(&message, &attestation)?;

			// Decode burn message from body
			let burn_msg = BurnMessage::decode(&mut &cross_chain_msg.message_body[..])
				.map_err(|_| Error::<T>::InvalidMessageFormat)?;

			// Mark nonce as used
			UsedNonces::<T>::insert(cross_chain_msg.source_domain, cross_chain_msg.nonce, true);
			TotalReceived::<T>::mutate(|count| *count = count.saturating_add(1));

			// Emit event
			Self::deposit_event(Event::MessageReceived {
				source_domain: cross_chain_msg.source_domain,
				nonce: cross_chain_msg.nonce,
				amount: burn_msg.amount,
				recipient: burn_msg.mint_recipient.to_vec(),
			});

			// In production, this would call pallet_edsc_token::mint()
			// For now, just mark as received

			Ok(())
		}

		/// Configure domain (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn configure_domain(
			origin: OriginFor<T>,
			domain: u32,
			enabled: bool,
			max_burn_amount: u128,
			daily_burn_limit: u128,
		) -> DispatchResult {
			ensure_root(origin)?;

			let config = DomainConfig {
				enabled,
				max_burn_amount,
				daily_burn_limit,
			};

			DomainConfigs::<T>::insert(domain, config);

			Self::deposit_event(Event::DomainConfigured {
				domain,
				enabled,
			});

			Ok(())
		}

		/// Pause bridge (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn pause_bridge(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			IsPaused::<T>::put(true);

			Self::deposit_event(Event::BridgePaused);

			Ok(())
		}

		/// Unpause bridge (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(4)]
		pub fn unpause_bridge(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			IsPaused::<T>::put(false);

			Self::deposit_event(Event::BridgeUnpaused);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Convert account to bytes
		fn account_to_bytes(account: &T::AccountId) -> BoundedVec<u8, ConstU32<64>> {
			account.encode().try_into().unwrap_or_default()
		}

		/// Check and update daily burn limit
		fn check_and_update_daily_limit(domain: u32, amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let (last_reset, current_volume) = DailyBurnVolume::<T>::get(domain);

			// Reset if 24 hours passed (14400 blocks at 6s)
			let blocks_per_day: BlockNumberFor<T> = 14_400u32.into();
			let (new_volume, new_reset) = if current_block.saturating_sub(last_reset) >= blocks_per_day {
				(amount, current_block)
			} else {
				(current_volume.saturating_add(amount), last_reset)
			};

			// Check limit
			let domain_config = DomainConfigs::<T>::get(domain)
				.ok_or(Error::<T>::InvalidDomain)?;

			if new_volume > domain_config.daily_burn_limit {
				Self::deposit_event(Event::DailyLimitExceeded {
					domain,
					attempted: amount,
					limit: domain_config.daily_burn_limit,
				});
				return Err(Error::<T>::DailyLimitExceeded.into());
			}

			// Update volume
			DailyBurnVolume::<T>::insert(domain, (new_reset, new_volume));

			Ok(())
		}

		/// Verify attestation (placeholder - will call pallet_edsc_bridge_attestation)
		fn verify_attestation(_message: &[u8], _attestation: &[u8]) -> DispatchResult {
			// In production, this would:
			// 1. Call pallet_edsc_bridge_attestation::verify_attestation()
			// 2. Check M-of-N threshold signatures
			// 3. Verify signatures are from registered attesters

			// For now, always succeed (placeholder)
			Ok(())
		}

		/// Get message hash
		pub fn get_message_hash(message: &CrossChainMessage) -> H256 {
			H256::from(sp_io::hashing::blake2_256(&message.encode()))
		}

		/// Get outbound message by nonce
		pub fn get_outbound_message(nonce: u64) -> Option<CrossChainMessage> {
			OutboundMessages::<T>::get(nonce)
		}

		/// Check if nonce is used
		pub fn is_nonce_used(source_domain: u32, nonce: u64) -> bool {
			UsedNonces::<T>::get(source_domain, nonce)
		}
	}
}
