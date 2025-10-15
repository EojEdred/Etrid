//! Lightning Bloc Channels Pallet for Ëtrid
//! Location: 05-multichain/lightning-bloc-networks/channel-manager/src/lib.rs
//!
//! Manages Lightning-style payment channels for instant BTC transactions
//! Works with bitcoin-bridge for off-chain payment routing

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        
        /// Minimum channel capacity
        #[pallet::constant]
        type MinChannelCapacity: Get<BalanceOf<Self>>;
        
        /// Maximum channel capacity
        #[pallet::constant]
        type MaxChannelCapacity: Get<BalanceOf<Self>>;
        
        /// Channel timeout (in blocks)
        #[pallet::constant]
        type ChannelTimeout: Get<Self::BlockNumber>;
    }

    /// Lightning channel state
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ChannelState {
        Opening,
        Open,
        Closing,
        Closed,
        Disputed,
    }

    /// Payment channel between two parties
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Channel<AccountId, Balance, BlockNumber> {
        pub channel_id: [u8; 32],
        pub party_a: AccountId,
        pub party_b: AccountId,
        pub capacity: Balance,
        pub balance_a: Balance,
        pub balance_b: Balance,
        pub state: ChannelState,
        pub nonce: u64,
        pub opened_at: BlockNumber,
        pub last_update: BlockNumber,
        pub timeout_at: Option<BlockNumber>,
    }

    /// HTLC (Hash Time Locked Contract) for atomic swaps
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct HTLC<AccountId, Balance, BlockNumber> {
        pub htlc_id: [u8; 32],
        pub channel_id: [u8; 32],
        pub sender: AccountId,
        pub receiver: AccountId,
        pub amount: Balance,
        pub hash_lock: [u8; 32],
        pub time_lock: BlockNumber,
        pub preimage: Option<BoundedVec<u8, ConstU32<64>>>,
        pub claimed: bool,
    }

    /// All payment channels
    #[pallet::storage]
    #[pallet::getter(fn channels)]
    pub type Channels<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32], // channel_id
        Channel<T::AccountId, BalanceOf<T>, T::BlockNumber>,
        OptionQuery,
    >;

    /// Active HTLCs
    #[pallet::storage]
    #[pallet::getter(fn htlcs)]
    pub type HTLCs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 32], // htlc_id
        HTLC<T::AccountId, BalanceOf<T>, T::BlockNumber>,
        OptionQuery,
    >;

    /// Channels by participant (for quick lookup)
    #[pallet::storage]
    #[pallet::getter(fn channels_by_party)]
    pub type ChannelsByParty<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        [u8; 32], // channel_id
        OptionQuery,
    >;

    /// Total channels opened
    #[pallet::storage]
    #[pallet::getter(fn total_channels)]
    pub type TotalChannels<T> = StorageValue<_, u64, ValueQuery>;

    /// Total value locked in channels
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Channel opened [channel_id, party_a, party_b, capacity]
        ChannelOpened([u8; 32], T::AccountId, T::AccountId, BalanceOf<T>),
        /// Channel state updated [channel_id, new_balance_a, new_balance_b, nonce]
        ChannelUpdated([u8; 32], BalanceOf<T>, BalanceOf<T>, u64),
        /// Channel closed [channel_id, final_balance_a, final_balance_b]
        ChannelClosed([u8; 32], BalanceOf<T>, BalanceOf<T>),
        /// HTLC created [htlc_id, channel_id, amount]
        HTLCCreated([u8; 32], [u8; 32], BalanceOf<T>),
        /// HTLC claimed [htlc_id, preimage]
        HTLCClaimed([u8; 32], Vec<u8>),
        /// HTLC expired [htlc_id]
        HTLCExpired([u8; 32]),
        /// Channel dispute initiated [channel_id]
        DisputeInitiated([u8; 32]),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Channel already exists
        ChannelAlreadyExists,
        /// Channel not found
        ChannelNotFound,
        /// HTLC not found
        HTLCNotFound,
        /// Invalid channel state
        InvalidChannelState,
        /// Insufficient balance
        InsufficientBalance,
        /// Capacity below minimum
        CapacityBelowMinimum,
        /// Capacity above maximum
        CapacityAboveMaximum,
        /// Not a channel participant
        NotChannelParticipant,
        /// Invalid signature
        InvalidSignature,
        /// Invalid nonce
        InvalidNonce,
        /// HTLC already claimed
        HTLCAlreadyClaimed,
        /// HTLC not expired
        HTLCNotExpired,
        /// Invalid preimage
        InvalidPreimage,
        /// Arithmetic overflow
        ArithmeticOverflow,
        /// Channel timeout
        ChannelTimeout,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Open a new payment channel
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn open_channel(
            origin: OriginFor<T>,
            counterparty: T::AccountId,
            capacity: BalanceOf<T>,
        ) -> DispatchResult {
            let opener = ensure_signed(origin)?;

            // Validate capacity
            ensure!(capacity >= T::MinChannelCapacity::get(), Error::<T>::CapacityBelowMinimum);
            ensure!(capacity <= T::MaxChannelCapacity::get(), Error::<T>::CapacityAboveMaximum);

            // Check if channel already exists
            ensure!(
                !ChannelsByParty::<T>::contains_key(&opener, &counterparty) &&
                !ChannelsByParty::<T>::contains_key(&counterparty, &opener),
                Error::<T>::ChannelAlreadyExists
            );

            // Lock funds from opener
            let half_capacity = capacity / 2u32.into();
            T::Currency::withdraw(
                &opener,
                half_capacity,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            // Generate channel ID
            let channel_id = Self::generate_channel_id(&opener, &counterparty);

            let current_block = frame_system::Pallet::<T>::block_number();
            let timeout_at = current_block + T::ChannelTimeout::get();

            // Create channel
            let channel = Channel {
                channel_id,
                party_a: opener.clone(),
                party_b: counterparty.clone(),
                capacity,
                balance_a: half_capacity,
                balance_b: half_capacity,
                state: ChannelState::Opening,
                nonce: 0,
                opened_at: current_block,
                last_update: current_block,
                timeout_at: Some(timeout_at),
            };

            Channels::<T>::insert(channel_id, channel);
            ChannelsByParty::<T>::insert(&opener, &counterparty, channel_id);

            TotalChannels::<T>::mutate(|total| *total = total.saturating_add(1));
            TotalLocked::<T>::mutate(|total| *total = total.saturating_add(half_capacity));

            Self::deposit_event(Event::ChannelOpened(channel_id, opener, counterparty, capacity));

            Ok(())
        }

        /// Accept and fully fund a channel
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn accept_channel(
            origin: OriginFor<T>,
            channel_id: [u8; 32],
        ) -> DispatchResult {
            let acceptor = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id)
                .ok_or(Error::<T>::ChannelNotFound)?;

            ensure!(channel.state == ChannelState::Opening, Error::<T>::InvalidChannelState);
            ensure!(channel.party_b == acceptor, Error::<T>::NotChannelParticipant);

            // Lock funds from acceptor
            T::Currency::withdraw(
                &acceptor,
                channel.balance_b,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            channel.state = ChannelState::Open;
            channel.timeout_at = None;
            Channels::<T>::insert(channel_id, channel.clone());

            TotalLocked::<T>::mutate(|total| *total = total.saturating_add(channel.balance_b));

            Self::deposit_event(Event::ChannelOpened(
                channel_id,
                channel.party_a,
                channel.party_b,
                channel.capacity,
            ));

            Ok(())
        }

        /// Update channel state (off-chain agreement)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn update_channel(
            origin: OriginFor<T>,
            channel_id: [u8; 32],
            new_balance_a: BalanceOf<T>,
            new_balance_b: BalanceOf<T>,
            nonce: u64,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id)
                .ok_or(Error::<T>::ChannelNotFound)?;

            ensure!(channel.state == ChannelState::Open, Error::<T>::InvalidChannelState);
            ensure!(
                updater == channel.party_a || updater == channel.party_b,
                Error::<T>::NotChannelParticipant
            );
            ensure!(nonce > channel.nonce, Error::<T>::InvalidNonce);

            // Validate balances
            let total_balance = new_balance_a.saturating_add(new_balance_b);
            ensure!(total_balance <= channel.capacity, Error::<T>::InsufficientBalance);

            channel.balance_a = new_balance_a;
            channel.balance_b = new_balance_b;
            channel.nonce = nonce;
            channel.last_update = frame_system::Pallet::<T>::block_number();

            Channels::<T>::insert(channel_id, channel);

            Self::deposit_event(Event::ChannelUpdated(channel_id, new_balance_a, new_balance_b, nonce));

            Ok(())
        }

        /// Close a channel and settle on-chain
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn close_channel(
            origin: OriginFor<T>,
            channel_id: [u8; 32],
        ) -> DispatchResult {
            let closer = ensure_signed(origin)?;

            let mut channel = Channels::<T>::get(channel_id)
                .ok_or(Error::<T>::ChannelNotFound)?;

            ensure!(
                closer == channel.party_a || closer == channel.party_b,
                Error::<T>::NotChannelParticipant
            );

            // Return funds
            if channel.balance_a > 0u32.into() {
                T::Currency::deposit_creating(&channel.party_a, channel.balance_a);
            }
            if channel.balance_b > 0u32.into() {
                T::Currency::deposit_creating(&channel.party_b, channel.balance_b);
            }

            let locked_amount = channel.balance_a.saturating_add(channel.balance_b);
            TotalLocked::<T>::mutate(|total| *total = total.saturating_sub(locked_amount));

            Self::deposit_event(Event::ChannelClosed(channel_id, channel.balance_a, channel.balance_b));

            channel.state = ChannelState::Closed;
            Channels::<T>::insert(channel_id, channel);

            Ok(())
        }

        /// Create HTLC for atomic swap
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn create_htlc(
            origin: OriginFor<T>,
            channel_id: [u8; 32],
            receiver: T::AccountId,
            amount: BalanceOf<T>,
            hash_lock: [u8; 32],
            time_lock_blocks: T::BlockNumber,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let channel = Channels::<T>::get(channel_id)
                .ok_or(Error::<T>::ChannelNotFound)?;

            ensure!(channel.state == ChannelState::Open, Error::<T>::InvalidChannelState);
            ensure!(
                sender == channel.party_a || sender == channel.party_b,
                Error::<T>::NotChannelParticipant
            );

            // Check sender has sufficient balance in channel
            let sender_balance = if sender == channel.party_a {
                channel.balance_a
            } else {
                channel.balance_b
            };
            ensure!(sender_balance >= amount, Error::<T>::InsufficientBalance);

            let htlc_id = Self::generate_htlc_id(&channel_id, &hash_lock);
            let time_lock = frame_system::Pallet::<T>::block_number() + time_lock_blocks;

            let htlc = HTLC {
                htlc_id,
                channel_id,
                sender: sender.clone(),
                receiver: receiver.clone(),
                amount,
                hash_lock,
                time_lock,
                preimage: None,
                claimed: false,
            };

            HTLCs::<T>::insert(htlc_id, htlc);

            Self::deposit_event(Event::HTLCCreated(htlc_id, channel_id, amount));

            Ok(())
        }

        /// Claim HTLC with preimage
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn claim_htlc(
            origin: OriginFor<T>,
            htlc_id: [u8; 32],
            preimage: Vec<u8>,
        ) -> DispatchResult {
            let claimer = ensure_signed(origin)?;

            let mut htlc = HTLCs::<T>::get(htlc_id)
                .ok_or(Error::<T>::HTLCNotFound)?;

            ensure!(claimer == htlc.receiver, Error::<T>::NotChannelParticipant);
            ensure!(!htlc.claimed, Error::<T>::HTLCAlreadyClaimed);

            // Verify preimage
            let hash = sp_io::hashing::blake2_256(&preimage);
            ensure!(hash == htlc.hash_lock, Error::<T>::InvalidPreimage);

            htlc.claimed = true;
            htlc.preimage = Some(preimage.clone().try_into().map_err(|_| Error::<T>::InvalidPreimage)?);
            HTLCs::<T>::insert(htlc_id, htlc);

            Self::deposit_event(Event::HTLCClaimed(htlc_id, preimage));

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        fn generate_channel_id(party_a: &T::AccountId, party_b: &T::AccountId) -> [u8; 32] {
            let mut data = Vec::new();
            data.extend_from_slice(&party_a.encode());
            data.extend_from_slice(&party_b.encode());
            sp_io::hashing::blake2_256(&data)
        }

        fn generate_htlc_id(channel_id: &[u8; 32], hash_lock: &[u8; 32]) -> [u8; 32] {
            let mut data = Vec::new();
            data.extend_from_slice(channel_id);
            data.extend_from_slice(hash_lock);
            sp_io::hashing::blake2_256(&data)
        }
    }
}
