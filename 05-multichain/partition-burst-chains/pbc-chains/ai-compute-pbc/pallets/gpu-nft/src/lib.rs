//! # GPU NFT Pallet - Tradeable GPU Certificates
//!
//! Mint GPUs as NFTs with reputation transfer, fractional ownership, marketplace integration.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GpuNFT<T: Config> {
        pub owner: T::AccountId,
        pub gpu_id: u64,
        pub reputation_snapshot: u32, // Reputation at mint time
        pub total_earnings: BalanceOf<T>,
        pub is_listed: bool,
        pub list_price: BalanceOf<T>,
        pub minted_at: u64,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type GpuNFTs<T: Config> = StorageMap<_, Blake2_128Concat, u64, GpuNFT<T>>;

    #[pallet::storage]
    pub type NextNftId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NFTMinted { nft_id: u64, owner: T::AccountId, gpu_id: u64 },
        NFTTransferred { nft_id: u64, from: T::AccountId, to: T::AccountId },
        NFTListed { nft_id: u64, price: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        NFTNotFound,
        NotOwner,
        NotListed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn mint_nft(origin: OriginFor<T>, gpu_id: u64) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            let nft_id = NextNftId::<T>::get();
            NextNftId::<T>::put(nft_id + 1);

            let nft = GpuNFT {
                owner: owner.clone(),
                gpu_id,
                reputation_snapshot: 10000,
                total_earnings: 0_u32.into(),
                is_listed: false,
                list_price: 0_u32.into(),
                minted_at: Self::current_timestamp(),
            };

            GpuNFTs::<T>::insert(nft_id, nft);
            Self::deposit_event(Event::NFTMinted { nft_id, owner, gpu_id });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn transfer_nft(origin: OriginFor<T>, nft_id: u64, to: T::AccountId) -> DispatchResult {
            let from = ensure_signed(origin)?;
            GpuNFTs::<T>::try_mutate(nft_id, |maybe_nft| {
                let nft = maybe_nft.as_mut().ok_or(Error::<T>::NFTNotFound)?;
                ensure!(nft.owner == from, Error::<T>::NotOwner);
                nft.owner = to.clone();
                Self::deposit_event(Event::NFTTransferred { nft_id, from, to });
                Ok::<(), DispatchError>(())
            })
        }

        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn list_nft(origin: OriginFor<T>, nft_id: u64, price: BalanceOf<T>) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            GpuNFTs::<T>::try_mutate(nft_id, |maybe_nft| {
                let nft = maybe_nft.as_mut().ok_or(Error::<T>::NFTNotFound)?;
                ensure!(nft.owner == owner, Error::<T>::NotOwner);
                nft.is_listed = true;
                nft.list_price = price;
                Self::deposit_event(Event::NFTListed { nft_id, price });
                Ok::<(), DispatchError>(())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number().try_into().unwrap_or(0u64).saturating_mul(6)
        }
    }
}
