#![cfg_attr(not(feature = "std"), no_std)]

//! # Channels Pallet (Lightning Bloc)
//!
//! State channel implementation for off-chain transactions.
//!
//! ## Overview
//!
//! The Channels pallet handles:
//! - Opening payment channels
//! - Off-chain transaction signing
//! - Channel settlement on-chain
//! - Dispute resolution

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use etrid_primitives::Balance;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type
        type Balance: Parameter
            + Member
            + CheckedAdd
            + CheckedSub
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Weight information for extrinsics
        type WeightInfo: WeightInfo;
    }

    /// State of a payment channel
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ChannelState {
        /// Channel is open and active
        Open,
        /// Channel is in dispute period
        Disputing,
        /// Channel is closed
        Closed,
    }

    /// Payment channel information
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Channel<T: Config> {
        /// Channel ID
        pub id: u64,
        /// Participant A
        pub participant_a: T::AccountId,
        /// Participant B
        pub participant_b: T::AccountId,
        /// Balance locked by participant A
        pub balance_a: T::Balance,
        /// Balance locked by participant B
        pub balance_b: T::Balance,
        /// Current nonce (transaction counter)
        pub nonce: u64,
        /// Channel state
        pub state: ChannelState,
        /// Block when channel can be closed (for dispute period)
        pub close_block: Option<BlockNumberFor<T>>,
    }

    /// Storage for all channels by ID
    #[pallet::storage]
    #[pallet::getter(fn channels)]
    pub type Channels<T: Config> = StorageMap<_, Blake2_128Concat, u64, Channel<T>, OptionQuery>;

    /// Next available channel ID
    #[pallet::storage]
    #[pallet::getter(fn next_channel_id)]
    pub type NextChannelId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Channel opened
        ChannelOpened {
            channel_id: u64,
            participant_a: T::AccountId,
            participant_b: T::AccountId,
            balance_a: T::Balance,
            balance_b: T::Balance,
        },
        /// Channel closed
        ChannelClosed {
            channel_id: u64,
        },
        /// Dispute initiated
        DisputeInitiated {
            channel_id: u64,
            initiator: T::AccountId,
        },
        /// Channel settled
        ChannelSettled {
            channel_id: u64,
            final_balance_a: T::Balance,
            final_balance_b: T::Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Channel does not exist
        ChannelNotFound,
        /// Channel is not open
        ChannelNotOpen,
        /// Not a channel participant
        NotParticipant,
        /// Insufficient balance
        InsufficientBalance,
        /// Invalid nonce
        InvalidNonce,
        /// Channel already exists
        ChannelExists,
        /// Dispute period not ended
        DisputePeriodActive,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Open a new payment channel
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::open_channel())]
        pub fn open_channel(
            origin: OriginFor<T>,
            participant_b: T::AccountId,
            balance_a: T::Balance,
            balance_b: T::Balance,
        ) -> DispatchResult {
            let participant_a = ensure_signed(origin)?;

            let channel_id = NextChannelId::<T>::get();

            let channel = Channel {
                id: channel_id,
                participant_a: participant_a.clone(),
                participant_b: participant_b.clone(),
                balance_a,
                balance_b,
                nonce: 0,
                state: ChannelState::Open,
                close_block: None,
            };

            Channels::<T>::insert(channel_id, channel);
            NextChannelId::<T>::put(channel_id + 1);

            Self::deposit_event(Event::ChannelOpened {
                channel_id,
                participant_a,
                participant_b,
                balance_a,
                balance_b,
            });

            Ok(())
        }

        /// Close a channel cooperatively
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::close_channel())]
        pub fn close_channel(
            origin: OriginFor<T>,
            channel_id: u64,
            final_balance_a: T::Balance,
            final_balance_b: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id).ok_or(Error::<T>::ChannelNotFound)?;

            // Ensure caller is a participant
            ensure!(
                who == channel.participant_a || who == channel.participant_b,
                Error::<T>::NotParticipant
            );

            // Ensure channel is open
            ensure!(
                channel.state == ChannelState::Open,
                Error::<T>::ChannelNotOpen
            );

            // Update channel state
            channel.state = ChannelState::Closed;
            Channels::<T>::insert(channel_id, channel);

            Self::deposit_event(Event::ChannelSettled {
                channel_id,
                final_balance_a,
                final_balance_b,
            });

            Ok(())
        }

        /// Initiate a dispute for unilateral close
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::initiate_dispute())]
        pub fn initiate_dispute(
            origin: OriginFor<T>,
            channel_id: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id).ok_or(Error::<T>::ChannelNotFound)?;

            // Ensure caller is a participant
            ensure!(
                who == channel.participant_a || who == channel.participant_b,
                Error::<T>::NotParticipant
            );

            // Ensure channel is open
            ensure!(
                channel.state == ChannelState::Open,
                Error::<T>::ChannelNotOpen
            );

            // Set dispute period (e.g., 100 blocks)
            let current_block = <frame_system::Pallet<T>>::block_number();
            let close_block = current_block + 100u32.into();

            channel.state = ChannelState::Disputing;
            channel.close_block = Some(close_block);
            Channels::<T>::insert(channel_id, channel);

            Self::deposit_event(Event::DisputeInitiated {
                channel_id,
                initiator: who,
            });

            Ok(())
        }

        /// Finalize channel after dispute period
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::finalize_channel())]
        pub fn finalize_channel(
            origin: OriginFor<T>,
            channel_id: u64,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id).ok_or(Error::<T>::ChannelNotFound)?;

            // Ensure channel is in dispute
            ensure!(
                channel.state == ChannelState::Disputing,
                Error::<T>::ChannelNotOpen
            );

            // Ensure dispute period has ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            if let Some(close_block) = channel.close_block {
                ensure!(
                    current_block >= close_block,
                    Error::<T>::DisputePeriodActive
                );
            }

            // Close channel
            channel.state = ChannelState::Closed;
            Channels::<T>::insert(channel_id, channel.clone());

            Self::deposit_event(Event::ChannelClosed { channel_id });

            Ok(())
        }
    }

    /// Weight functions (placeholder - should be benchmarked)
    pub trait WeightInfo {
        fn open_channel() -> Weight;
        fn close_channel() -> Weight;
        fn initiate_dispute() -> Weight;
        fn finalize_channel() -> Weight;
    }

    impl WeightInfo for () {
        fn open_channel() -> Weight {
            Weight::from_parts(10_000, 0)
        }
        fn close_channel() -> Weight {
            Weight::from_parts(10_000, 0)
        }
        fn initiate_dispute() -> Weight {
            Weight::from_parts(10_000, 0)
        }
        fn finalize_channel() -> Weight {
            Weight::from_parts(10_000, 0)
        }
    }
}
