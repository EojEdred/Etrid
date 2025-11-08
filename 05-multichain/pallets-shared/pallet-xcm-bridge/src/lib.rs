//! # XCM/DETRP2P Bridge Pallet
//!
//! Cross-chain messaging bridge for the EDSC system using DETRP2P protocol.
//!
//! ## Overview
//!
//! This pallet acts as the bridge between Substrate runtime pallets and the DETRP2P
//! peer-to-peer networking layer. It enables:
//! - Checkpoint synchronization from FlareChain to PBC-EDSC
//! - Reserve data propagation
//! - Cross-chain governance messages
//! - Event notifications between chains
//!
//! ## Message Flow
//!
//! ```
//! FlareChain Reserve Oracle
//!     │
//!     ├─ 1. Create checkpoint
//!     └─ 2. Call xcm_bridge::send_checkpoint()
//!             │
//!             └─ 3. Encode message
//!                     │
//!                     └─ 4. Queue for DETRP2P transmission
//!                             │
//!                             └─ 5. DETRP2P sends over network
//!                                     │
//!                                     └─ 6. PBC-EDSC receives
//!                                             │
//!                                             └─ 7. xcm_bridge::receive_checkpoint()
//!                                                     │
//!                                                     └─ 8. Verify signature
//!                                                             │
//!                                                             └─ 9. Call checkpoint::verify_checkpoint()
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
	use sp_core::H256;
	use sp_runtime::traits::Hash;
	use sp_std::vec::Vec;

	/// Chain identifier
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ChainId {
		/// FlareChain (main chain)
		FlareChain,
		/// PBC-EDSC (dedicated EDSC chain)
		PbcEdsc,
		/// Other PBC chains
		Other(u32),
	}

	impl Default for ChainId {
		fn default() -> Self {
			ChainId::FlareChain
		}
	}

	impl ChainId {
		/// Convert ChainId to u8 for event emission
		pub fn to_u8(&self) -> u8 {
			match self {
				ChainId::FlareChain => 0,
				ChainId::PbcEdsc => 1,
				ChainId::Other(_) => 2,
			}
		}

		/// Get chain ID value (for Other variant)
		pub fn chain_value(&self) -> u32 {
			match self {
				ChainId::FlareChain => 0,
				ChainId::PbcEdsc => 1,
				ChainId::Other(id) => *id,
			}
		}
	}

	/// Message type for cross-chain communication
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum MessageType {
		/// Reserve checkpoint data
		ReserveCheckpoint,
		/// Price update
		PriceUpdate,
		/// Governance action
		Governance,
		/// Emergency pause
		EmergencyPause,
		/// Alert notification
		Alert,
	}

	impl MessageType {
		/// Convert MessageType to u8 for event emission
		pub fn to_u8(&self) -> u8 {
			match self {
				MessageType::ReserveCheckpoint => 0,
				MessageType::PriceUpdate => 1,
				MessageType::Governance => 2,
				MessageType::EmergencyPause => 3,
				MessageType::Alert => 4,
			}
		}
	}

	/// Cross-chain message structure
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct CrossChainMessage<BlockNumber> {
		/// Source chain
		pub source: ChainId,
		/// Destination chain
		pub destination: ChainId,
		/// Message type
		pub message_type: MessageType,
		/// Message payload
		pub payload: BoundedVec<u8, ConstU32<1024>>,
		/// Block number when message was created
		pub block_number: BlockNumber,
		/// Nonce for ordering
		pub nonce: u64,
		/// Timestamp
		pub timestamp: u64,
	}

	/// Message status
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum MessageStatus {
		/// Pending transmission
		Pending,
		/// Sent via DETRP2P
		Sent,
		/// Received on destination
		Received,
		/// Verified and processed
		Processed,
		/// Failed to send/receive
		Failed,
	}

	impl Default for MessageStatus {
		fn default() -> Self {
			MessageStatus::Pending
		}
	}

	/// Checkpoint data payload
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CheckpointPayload {
		/// Reserve ratio (basis points)
		pub reserve_ratio: u16,
		/// Total reserves (USD cents)
		pub total_reserves: u128,
		/// Vault value (USD cents)
		pub vault_value: u128,
		/// Custodian value (USD cents)
		pub custodian_value: u128,
		/// Total EDSC supply
		pub total_supply: u128,
		/// Merkle root of state
		pub state_root: H256,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Runtime event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum message payload size (bytes)
		#[pallet::constant]
		type MaxPayloadSize: Get<u32>;

		/// Message timeout (blocks)
		#[pallet::constant]
		type MessageTimeout: Get<BlockNumberFor<Self>>;

		/// Maximum pending messages
		#[pallet::constant]
		type MaxPendingMessages: Get<u32>;

		/// Chain identifier for this runtime
		#[pallet::constant]
		type ChainIdentifier: Get<ChainId>;
	}

	/// Pending outbound messages
	#[pallet::storage]
	#[pallet::getter(fn pending_messages)]
	pub type PendingMessages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Message nonce
		CrossChainMessage<BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Message status tracking
	#[pallet::storage]
	#[pallet::getter(fn message_status)]
	pub type MessageStatusMap<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Message nonce
		MessageStatus,
		ValueQuery,
	>;

	/// Message nonce counter
	#[pallet::storage]
	#[pallet::getter(fn message_nonce)]
	pub type MessageNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Received message hashes (for deduplication)
	#[pallet::storage]
	#[pallet::getter(fn received_messages)]
	pub type ReceivedMessages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		H256, // Message hash
		BlockNumberFor<T>, // Block received
		OptionQuery,
	>;

	/// Total messages sent
	#[pallet::storage]
	#[pallet::getter(fn total_sent)]
	pub type TotalSent<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Total messages received
	#[pallet::storage]
	#[pallet::getter(fn total_received)]
	pub type TotalReceived<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// DETRP2P connection status
	#[pallet::storage]
	#[pallet::getter(fn detrp2p_connected)]
	pub type Detrp2pConnected<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Message queued for transmission [nonce, destination_chain, message_type]
		/// destination_chain: 0=FlareChain, 1=PbcEdsc, 2=Other
		/// message_type: 0=ReserveCheckpoint, 1=PriceUpdate, 2=Governance, 3=EmergencyPause, 4=Alert
		MessageQueued {
			nonce: u64,
			destination_chain: u8,
			message_type: u8,
		},
		/// Message sent via DETRP2P [nonce]
		MessageSent {
			nonce: u64,
		},
		/// Message received from another chain [source_chain, message_type, nonce]
		/// source_chain: 0=FlareChain, 1=PbcEdsc, 2=Other
		/// message_type: 0=ReserveCheckpoint, 1=PriceUpdate, 2=Governance, 3=EmergencyPause, 4=Alert
		MessageReceived {
			source_chain: u8,
			message_type: u8,
			nonce: u64,
		},
		/// Checkpoint received and verified [block_number, reserve_ratio]
		CheckpointProcessed {
			block_number: BlockNumberFor<T>,
			reserve_ratio: u16,
		},
		/// Message failed [nonce, reason]
		MessageFailed {
			nonce: u64,
			reason: Vec<u8>,
		},
		/// DETRP2P connection status changed
		ConnectionStatusChanged {
			connected: bool,
		},
		/// Message timeout [nonce]
		MessageTimeout {
			nonce: u64,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Message payload too large
		PayloadTooLarge,
		/// Message not found
		MessageNotFound,
		/// Invalid message format
		InvalidMessageFormat,
		/// Message already processed
		MessageAlreadyProcessed,
		/// DETRP2P not connected
		Detrp2pNotConnected,
		/// Maximum pending messages reached
		MaxPendingMessagesReached,
		/// Invalid chain identifier
		InvalidChainId,
		/// Signature verification failed
		SignatureVerificationFailed,
		/// Message timeout
		Timeout,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Clean up expired messages
			// In production, this would iterate and remove timed-out messages
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Send checkpoint to destination chain
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn send_checkpoint(
			origin: OriginFor<T>,
			destination_chain: u8, // 0=FlareChain, 1=PbcEdsc
			reserve_ratio: u16,
			total_reserves: u128,
			vault_value: u128,
			custodian_value: u128,
			total_supply: u128,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Reconstruct checkpoint payload
			let checkpoint = CheckpointPayload {
				reserve_ratio,
				total_reserves,
				vault_value,
				custodian_value,
				total_supply,
				state_root: H256::zero(), // Placeholder
			};

			// Reconstruct destination ChainId
			let destination = if destination_chain == 0 {
				ChainId::FlareChain
			} else if destination_chain == 1 {
				ChainId::PbcEdsc
			} else {
				ChainId::Other(destination_chain as u32)
			};

			// Encode checkpoint payload
			let payload: Vec<u8> = checkpoint.encode();
			let bounded_payload: BoundedVec<u8, ConstU32<1024>> = payload.try_into()
				.map_err(|_| Error::<T>::PayloadTooLarge)?;

			// Create message
			let nonce = MessageNonce::<T>::get();
			let block_number = <frame_system::Pallet<T>>::block_number();
			let timestamp = Self::get_timestamp();

			let message = CrossChainMessage {
				source: T::ChainIdentifier::get(),
				destination: destination.clone(),
				message_type: MessageType::ReserveCheckpoint,
				payload: bounded_payload,
				block_number,
				nonce,
				timestamp,
			};

			// Store message
			PendingMessages::<T>::insert(nonce, message);
			MessageStatusMap::<T>::insert(nonce, MessageStatus::Pending);
			MessageNonce::<T>::put(nonce.saturating_add(1));

			Self::deposit_event(Event::MessageQueued {
				nonce,
				destination_chain: destination.to_u8(),
				message_type: MessageType::ReserveCheckpoint.to_u8(),
			});

			// In production, this would trigger DETRP2P transmission
			// For now, mark as sent (internal call)
			Self::internal_mark_message_sent(nonce)?;

			Ok(())
		}

		/// Receive and process checkpoint from FlareChain
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn receive_checkpoint(
			origin: OriginFor<T>,
			source_chain: u8, // 0=FlareChain, 1=PbcEdsc
			nonce: u64,
			reserve_ratio: u16,
			total_reserves: u128,
			vault_value: u128,
			custodian_value: u128,
			total_supply: u128,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Reconstruct source ChainId
			let source = if source_chain == 0 {
				ChainId::FlareChain
			} else if source_chain == 1 {
				ChainId::PbcEdsc
			} else {
				ChainId::Other(source_chain as u32)
			};

			// Reconstruct checkpoint payload
			let checkpoint = CheckpointPayload {
				reserve_ratio,
				total_reserves,
				vault_value,
				custodian_value,
				total_supply,
				state_root: H256::zero(), // Placeholder
			};

			// Calculate message hash for deduplication
			let message_hash = Self::calculate_message_hash(&checkpoint.encode());

			// Check if already processed
			ensure!(
				!ReceivedMessages::<T>::contains_key(&message_hash),
				Error::<T>::MessageAlreadyProcessed
			);

			// Store received message
			let block_number = <frame_system::Pallet<T>>::block_number();
			ReceivedMessages::<T>::insert(&message_hash, block_number);
			TotalReceived::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::MessageReceived {
				source_chain: source.to_u8(),
				message_type: MessageType::ReserveCheckpoint.to_u8(),
				nonce,
			});

			// Process checkpoint
			// In production, this would call pallet_edsc_checkpoint::verify_checkpoint()
			Self::deposit_event(Event::CheckpointProcessed {
				block_number,
				reserve_ratio: checkpoint.reserve_ratio,
			});

			Ok(())
		}

		/// Mark message as sent (called by DETRP2P layer)
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn mark_message_sent(
			origin: OriginFor<T>,
			nonce: u64,
		) -> DispatchResult {
			ensure_root(origin)?;
			Self::internal_mark_message_sent(nonce)
		}

		/// Update DETRP2P connection status
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn set_connection_status(
			origin: OriginFor<T>,
			connected: bool,
		) -> DispatchResult {
			ensure_root(origin)?;

			Detrp2pConnected::<T>::put(connected);

			Self::deposit_event(Event::ConnectionStatusChanged { connected });

			Ok(())
		}

		/// Clean up processed messages
		#[pallet::weight(10_000)]
		#[pallet::call_index(4)]
		pub fn cleanup_messages(
			origin: OriginFor<T>,
			nonce: u64,
		) -> DispatchResult {
			ensure_root(origin)?;

			PendingMessages::<T>::remove(nonce);
			MessageStatusMap::<T>::remove(nonce);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Calculate message hash
		fn calculate_message_hash(data: &[u8]) -> H256 {
			H256::from(sp_io::hashing::blake2_256(data))
		}

		/// Internal helper to mark message as sent (no origin check)
		fn internal_mark_message_sent(nonce: u64) -> DispatchResult {
			ensure!(
				PendingMessages::<T>::contains_key(&nonce),
				Error::<T>::MessageNotFound
			);

			MessageStatusMap::<T>::insert(nonce, MessageStatus::Sent);
			TotalSent::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::MessageSent { nonce });

			Ok(())
		}

		/// Get current timestamp
		fn get_timestamp() -> u64 {
			// In production, use pallet_timestamp
			// For now, return placeholder
			0u64
		}

		/// Get pending message count
		pub fn get_pending_count() -> u32 {
			// In production, iterate PendingMessages
			0u32
		}

		/// Check if DETRP2P is connected
		pub fn is_connected() -> bool {
			Detrp2pConnected::<T>::get()
		}

		/// Get message by nonce
		pub fn get_message(nonce: u64) -> Option<CrossChainMessage<BlockNumberFor<T>>> {
			PendingMessages::<T>::get(nonce)
		}

		/// Get message status
		pub fn get_status(nonce: u64) -> MessageStatus {
			MessageStatusMap::<T>::get(nonce)
		}
	}
}
