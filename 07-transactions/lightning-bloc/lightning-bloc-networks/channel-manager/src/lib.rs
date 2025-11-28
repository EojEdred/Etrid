#![cfg_attr(not(feature = "std"), no_std)]

//! # Lightning Channels Pallet (Stub)
//!
//! This is a stub implementation of the Lightning Network payment channels pallet.
//!
//! ## Overview
//!
//! This pallet will eventually provide:
//! - Lightning Network payment channel management
//! - Channel opening, closing, and force-closing
//! - Payment routing through channels
//! - HTLCs (Hash Time-Locked Contracts)
//! - Channel state management
//!
//! ## Status
//!
//! **STUB IMPLEMENTATION** - Not yet functional
//! Created to satisfy workspace dependencies while proper implementation is developed.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The balance type
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
	}

	/// Placeholder for channel data
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Channel<T: Config> {
		/// Channel ID
		pub id: u64,
		/// Participant A
		pub party_a: T::AccountId,
		/// Participant B
		pub party_b: T::AccountId,
		/// Channel balance
		pub balance: T::Balance,
		/// Is channel open?
		pub is_open: bool,
	}

	/// Placeholder storage for channels
	#[pallet::storage]
	#[pallet::getter(fn channels)]
	pub type Channels<T: Config> = StorageMap<_, Blake2_128Concat, u64, Channel<T>, OptionQuery>;

	/// Next channel ID
	#[pallet::storage]
	#[pallet::getter(fn next_channel_id)]
	pub type NextChannelId<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Channel opened [channel_id, party_a, party_b]
		ChannelOpened { channel_id: u64, party_a: T::AccountId, party_b: T::AccountId },
		/// Channel closed [channel_id]
		ChannelClosed { channel_id: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Feature not yet implemented
		NotImplemented,
		/// Channel not found
		ChannelNotFound,
		/// Channel already exists
		ChannelAlreadyExists,
		/// Not authorized
		NotAuthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stub function for opening a channel
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn open_channel(
			_origin: OriginFor<T>,
			_counterparty: T::AccountId,
			_amount: T::Balance,
		) -> DispatchResult {
			// Not implemented yet
			Err(Error::<T>::NotImplemented.into())
		}

		/// Stub function for closing a channel
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn close_channel(
			_origin: OriginFor<T>,
			_channel_id: u64,
		) -> DispatchResult {
			// Not implemented yet
			Err(Error::<T>::NotImplemented.into())
		}
	}
}
