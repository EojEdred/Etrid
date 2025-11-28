//! # Lightning Payment Pallet
//!
//! Streaming micropayments for AI compute (pay per second).
//! Inspired by Lightning Network but optimized for compute payments.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Payment channel state
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct PaymentChannel<T: Config> {
        pub sender: T::AccountId,
        pub receiver: T::AccountId,
        pub total_deposited: BalanceOf<T>,
        pub total_withdrawn: BalanceOf<T>,
        pub nonce: u64,
        pub is_open: bool,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Channels<T: Config> = StorageMap<_, Blake2_128Concat, u64, PaymentChannel<T>>;

    #[pallet::storage]
    pub type NextChannelId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ChannelOpened { channel_id: u64, sender: T::AccountId, receiver: T::AccountId },
        PaymentStreamed { channel_id: u64, amount: BalanceOf<T> },
        ChannelClosed { channel_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        ChannelNotFound,
        ChannelClosed,
        InsufficientFunds,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn open_channel(
            origin: OriginFor<T>,
            receiver: T::AccountId,
            deposit: BalanceOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let channel_id = NextChannelId::<T>::get();
            NextChannelId::<T>::put(channel_id + 1);

            let channel = PaymentChannel {
                sender: sender.clone(),
                receiver: receiver.clone(),
                total_deposited: deposit,
                total_withdrawn: 0_u32.into(),
                nonce: 0,
                is_open: true,
            };

            Channels::<T>::insert(channel_id, channel);
            Self::deposit_event(Event::ChannelOpened { channel_id, sender, receiver });
            Ok(())
        }
    }
}
