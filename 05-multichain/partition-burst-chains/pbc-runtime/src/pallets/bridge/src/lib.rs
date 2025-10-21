#![cfg_attr(not(feature = "std"), no_std)]

//! # Bridge Pallet
//!
//! Cross-chain messaging between PBCs and FlareChain.
//!
//! ## Overview
//!
//! The Bridge pallet handles:
//! - Sending messages from PBC to FlareChain
//! - Receiving messages from FlareChain to PBC
//! - State root aggregation
//! - Cross-chain transaction verification

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use etrid_primitives::{BlockNumber as PrimitiveBlockNumber, ChainId};
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Hash as HashT, Saturating};
    use sp_std::vec::Vec;

    /// Maximum message size (1KB)
    pub const MAX_MESSAGE_SIZE: u32 = 1024;

    /// Maximum messages per block
    pub const MAX_MESSAGES_PER_BLOCK: u32 = 100;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The chain ID of this PBC
        #[pallet::constant]
        type ChainId: Get<u8>;

        /// Weight information for extrinsics
        type WeightInfo: WeightInfo;
    }

    /// Simplified cross-chain message (storage-compatible)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Message<T: Config> {
        /// Source chain ID (0-12)
        pub from: u8,
        /// Destination chain ID (0-12)
        pub to: u8,
        /// Block number when sent
        pub block_number: u32,
        /// Message nonce
        pub nonce: u64,
        /// Message data hash
        pub data_hash: T::Hash,
    }

    /// State root information
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct StateRoot<T: Config> {
        /// Chain ID
        pub chain_id: u8,
        /// Block number
        pub block_number: u32,
        /// State root hash
        pub state_root: T::Hash,
        /// Block hash
        pub block_hash: T::Hash,
    }

    /// Pending outgoing messages to FlareChain
    #[pallet::storage]
    #[pallet::getter(fn outgoing_messages)]
    pub type OutgoingMessages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BlockNumberFor<T>,
        BoundedVec<Message<T>, ConstU32<MAX_MESSAGES_PER_BLOCK>>,
        ValueQuery,
    >;

    /// Received incoming messages from FlareChain
    #[pallet::storage]
    #[pallet::getter(fn incoming_messages)]
    pub type IncomingMessages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BlockNumberFor<T>,
        BoundedVec<Message<T>, ConstU32<MAX_MESSAGES_PER_BLOCK>>,
        ValueQuery,
    >;

    /// Message data by hash (temporary storage for processing)
    #[pallet::storage]
    #[pallet::getter(fn message_data)]
    pub type MessageData<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BoundedVec<u8, ConstU32<MAX_MESSAGE_SIZE>>,
        OptionQuery,
    >;

    /// Latest state root submitted to FlareChain
    #[pallet::storage]
    #[pallet::getter(fn latest_state_root)]
    pub type LatestStateRoot<T: Config> = StorageValue<_, StateRoot<T>, OptionQuery>;

    /// Message nonce for ordering
    #[pallet::storage]
    #[pallet::getter(fn message_nonce)]
    pub type MessageNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Message sent to FlareChain
        MessageSent {
            to: u8,
            nonce: u64,
            data_hash: T::Hash,
        },
        /// Message received from FlareChain
        MessageReceived {
            from: u8,
            nonce: u64,
            data_hash: T::Hash,
        },
        /// State root aggregated to FlareChain
        StateRootAggregated {
            block_number: BlockNumberFor<T>,
            state_root: T::Hash,
        },
        /// Message data stored
        MessageDataStored {
            data_hash: T::Hash,
            size: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Invalid chain ID
        InvalidChainId,
        /// Message too large
        MessageTooLarge,
        /// Invalid message nonce
        InvalidNonce,
        /// Message verification failed
        VerificationFailed,
        /// Too many messages in block
        TooManyMessages,
        /// Message data not found
        MessageDataNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Send a message to FlareChain or another PBC
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::send_message())]
        pub fn send_message(origin: OriginFor<T>, to: u8, data: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Validate chain ID
            ensure!(to <= 12, Error::<T>::InvalidChainId);

            // Validate message size
            ensure!(data.len() <= MAX_MESSAGE_SIZE as usize, Error::<T>::MessageTooLarge);

            let from = T::ChainId::get();
            let block_number = <frame_system::Pallet<T>>::block_number();
            let nonce = MessageNonce::<T>::get();

            // Hash the data
            let data_hash = T::Hashing::hash(&data);

            // Convert block number to u32
            let primitive_block_number: u32 = block_number
                .try_into()
                .unwrap_or_else(|_| u32::MAX);

            // Create message
            let message = Message::<T> {
                from,
                to,
                block_number: primitive_block_number,
                nonce,
                data_hash,
            };

            // Store message data
            let bounded_data: BoundedVec<u8, ConstU32<MAX_MESSAGE_SIZE>> =
                data.try_into().map_err(|_| Error::<T>::MessageTooLarge)?;
            MessageData::<T>::insert(data_hash, bounded_data.clone());

            // Add to outgoing messages
            OutgoingMessages::<T>::try_mutate(block_number, |messages| {
                messages
                    .try_push(message)
                    .map_err(|_| Error::<T>::TooManyMessages)
            })?;

            // Increment nonce
            MessageNonce::<T>::put(nonce.saturating_add(1));

            Self::deposit_event(Event::MessageSent {
                to,
                nonce,
                data_hash,
            });

            Self::deposit_event(Event::MessageDataStored {
                data_hash,
                size: bounded_data.len() as u32,
            });

            Ok(())
        }

        /// Receive a message from FlareChain (authority only)
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::receive_message())]
        pub fn receive_message(origin: OriginFor<T>, from: u8, data: Vec<u8>) -> DispatchResult {
            ensure_root(origin)?;

            // Validate chain ID
            ensure!(from <= 12, Error::<T>::InvalidChainId);

            // Validate message size
            ensure!(data.len() <= MAX_MESSAGE_SIZE as usize, Error::<T>::MessageTooLarge);

            let block_number = <frame_system::Pallet<T>>::block_number();
            let nonce = MessageNonce::<T>::get();

            // Hash the data
            let data_hash = T::Hashing::hash(&data);

            // Convert block number to u32
            let primitive_block_number: u32 = block_number
                .try_into()
                .unwrap_or_else(|_| u32::MAX);

            // Create message
            let message = Message::<T> {
                from,
                to: T::ChainId::get(),
                block_number: primitive_block_number,
                nonce,
                data_hash,
            };

            // Store message data
            let bounded_data: BoundedVec<u8, ConstU32<MAX_MESSAGE_SIZE>> =
                data.try_into().map_err(|_| Error::<T>::MessageTooLarge)?;
            MessageData::<T>::insert(data_hash, bounded_data);

            // Add to incoming messages
            IncomingMessages::<T>::try_mutate(block_number, |messages| {
                messages
                    .try_push(message)
                    .map_err(|_| Error::<T>::TooManyMessages)
            })?;

            Self::deposit_event(Event::MessageReceived {
                from,
                nonce,
                data_hash,
            });

            Ok(())
        }

        /// Submit state root to FlareChain (authority only)
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::submit_state_root())]
        pub fn submit_state_root(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let block_number = <frame_system::Pallet<T>>::block_number();
            let block_hash = <frame_system::Pallet<T>>::block_hash(block_number);
            // Use block hash as state root proxy for now
            // In production, this would be the actual merkle root
            let state_root = block_hash;

            // Convert block number to u32
            let primitive_block_number: u32 = block_number
                .try_into()
                .unwrap_or_else(|_| u32::MAX);

            let state_root_info = StateRoot::<T> {
                chain_id: T::ChainId::get(),
                block_number: primitive_block_number,
                state_root,
                block_hash,
            };

            LatestStateRoot::<T>::put(state_root_info);

            Self::deposit_event(Event::StateRootAggregated {
                block_number,
                state_root,
            });

            Ok(())
        }

        /// Retrieve message data by hash
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::get_message_data())]
        pub fn get_message_data(
            origin: OriginFor<T>,
            data_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // Verify message data exists
            ensure!(
                MessageData::<T>::contains_key(data_hash),
                Error::<T>::MessageDataNotFound
            );

            Ok(().into())
        }

        /// Clear old message data (cleanup)
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::clear_message_data())]
        pub fn clear_message_data(origin: OriginFor<T>, data_hash: T::Hash) -> DispatchResult {
            ensure_root(origin)?;

            MessageData::<T>::remove(data_hash);

            Ok(())
        }
    }

    /// Weight functions (placeholder - should be benchmarked)
    pub trait WeightInfo {
        fn send_message() -> Weight;
        fn receive_message() -> Weight;
        fn submit_state_root() -> Weight;
        fn get_message_data() -> Weight;
        fn clear_message_data() -> Weight;
    }

    impl WeightInfo for () {
        fn send_message() -> Weight {
            Weight::from_parts(50_000, 0)
        }
        fn receive_message() -> Weight {
            Weight::from_parts(50_000, 0)
        }
        fn submit_state_root() -> Weight {
            Weight::from_parts(30_000, 0)
        }
        fn get_message_data() -> Weight {
            Weight::from_parts(10_000, 0)
        }
        fn clear_message_data() -> Weight {
            Weight::from_parts(10_000, 0)
        }
    }
}
