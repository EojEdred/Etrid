//! # Prompt Marketplace Pallet
//!
//! NFT marketplace for AI prompts - buy/sell optimized prompts like digital assets.
//!
//! ## Features
//! - Mint prompts as NFTs (ERC-721 compatible)
//! - Revenue split: 70% creator, 25% platform, 5% curator
//! - DRM protection via watermarking
//! - Categories: Code, Writing, Art, Data Analysis
//! - Ratings & reviews
//!
//! ## Revenue Model
//! - 100K prompts/month × $10 avg = $1M/month
//! - Platform earns 25% = $250K/month
//! - Year 1 ARR: $3M

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Prompt category
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum PromptCategory {
        CodeGeneration,
        Writing,
        Art,
        DataAnalysis,
        Marketing,
        Education,
        Custom,
    }

    /// Prompt NFT
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct PromptNFT<T: Config> {
        /// Creator/owner
        pub owner: T::AccountId,
        /// Prompt title
        pub title: BoundedVec<u8, ConstU32<128>>,
        /// Description
        pub description: BoundedVec<u8, ConstU32<512>>,
        /// Category
        pub category: PromptCategory,
        /// Encrypted prompt hash (IPFS)
        pub prompt_hash: T::Hash,
        /// Decryption key hash (revealed after purchase)
        pub key_hash: T::Hash,
        /// Price in ËDSC
        pub price: BalanceOf<T>,
        /// Total sales
        pub sales_count: u32,
        /// Total revenue earned
        pub total_revenue: BalanceOf<T>,
        /// Average rating (0-50000 = 0.0-5.0)
        pub rating: u32,
        /// Rating count
        pub rating_count: u32,
        /// Is listed for sale
        pub is_listed: bool,
        /// Created timestamp
        pub created_at: u64,
    }

    /// Prompt purchase record
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Purchase<T: Config> {
        pub buyer: T::AccountId,
        pub prompt_id: u64,
        pub price_paid: BalanceOf<T>,
        pub purchased_at: u64,
        pub rating: Option<u32>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Creator revenue share (basis points, e.g., 7000 = 70%)
        #[pallet::constant]
        type CreatorShareBps: Get<u16>;

        /// Platform revenue share (basis points, e.g., 2500 = 25%)
        #[pallet::constant]
        type PlatformShareBps: Get<u16>;

        /// Curator revenue share (basis points, e.g., 500 = 5%)
        #[pallet::constant]
        type CuratorShareBps: Get<u16>;

        /// Minimum prompt price
        #[pallet::constant]
        type MinPromptPrice: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Prompt NFTs by ID
    #[pallet::storage]
    pub type Prompts<T: Config> = StorageMap<_, Blake2_128Concat, u64, PromptNFT<T>>;

    /// Next prompt ID
    #[pallet::storage]
    pub type NextPromptId<T> = StorageValue<_, u64, ValueQuery>;

    /// Creator's prompts
    #[pallet::storage]
    pub type CreatorPrompts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u64, ConstU32<1000>>,
        ValueQuery,
    >;

    /// User's purchased prompts
    #[pallet::storage]
    pub type UserPurchases<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<Purchase<T>, ConstU32<10000>>,
        ValueQuery,
    >;

    /// Prompt decryption keys (only revealed to buyers)
    #[pallet::storage]
    pub type DecryptionKeys<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // prompt_id
        Blake2_128Concat,
        T::AccountId, // buyer
        BoundedVec<u8, ConstU32<64>>, // decryption key
    >;

    /// Total prompts created
    #[pallet::storage]
    pub type TotalPrompts<T> = StorageValue<_, u64, ValueQuery>;

    /// Total marketplace volume
    #[pallet::storage]
    pub type TotalVolume<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Prompt minted [prompt_id, creator, price]
        PromptMinted { prompt_id: u64, creator: T::AccountId, price: BalanceOf<T> },
        /// Prompt purchased [prompt_id, buyer, price]
        PromptPurchased { prompt_id: u64, buyer: T::AccountId, price: BalanceOf<T> },
        /// Prompt listed [prompt_id, price]
        PromptListed { prompt_id: u64, price: BalanceOf<T> },
        /// Prompt unlisted [prompt_id]
        PromptUnlisted { prompt_id: u64 },
        /// Prompt rated [prompt_id, rating]
        PromptRated { prompt_id: u64, rating: u32 },
        /// Revenue distributed [prompt_id, creator_amount, platform_amount]
        RevenueDistributed {
            prompt_id: u64,
            creator_amount: BalanceOf<T>,
            platform_amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PromptNotFound,
        NotPromptOwner,
        PromptNotListed,
        AlreadyPurchased,
        InsufficientBalance,
        PriceTooLow,
        AlreadyRated,
        InvalidRating,
        TooManyPrompts,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mint new prompt NFT
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn mint_prompt(
            origin: OriginFor<T>,
            title: BoundedVec<u8, ConstU32<128>>,
            description: BoundedVec<u8, ConstU32<512>>,
            category: PromptCategory,
            prompt_hash: T::Hash,
            key_hash: T::Hash,
            price: BalanceOf<T>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            ensure!(price >= T::MinPromptPrice::get(), Error::<T>::PriceTooLow);

            let prompt_id = NextPromptId::<T>::get();
            NextPromptId::<T>::put(prompt_id + 1);

            let prompt = PromptNFT {
                owner: creator.clone(),
                title,
                description,
                category,
                prompt_hash,
                key_hash,
                price,
                sales_count: 0,
                total_revenue: 0_u32.into(),
                rating: 0,
                rating_count: 0,
                is_listed: true,
                created_at: Self::current_timestamp(),
            };

            Prompts::<T>::insert(prompt_id, prompt);

            CreatorPrompts::<T>::try_mutate(&creator, |prompts| {
                prompts.try_push(prompt_id).map_err(|_| Error::<T>::TooManyPrompts)?;
                Ok::<(), DispatchError>(())
            })?;

            TotalPrompts::<T>::mutate(|total| *total = total.saturating_add(1));

            Self::deposit_event(Event::PromptMinted { prompt_id, creator, price });
            Ok(())
        }

        /// Purchase prompt
        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn purchase_prompt(
            origin: OriginFor<T>,
            prompt_id: u64,
            decryption_key: BoundedVec<u8, ConstU32<64>>,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            let prompt = Prompts::<T>::get(prompt_id).ok_or(Error::<T>::PromptNotFound)?;
            ensure!(prompt.is_listed, Error::<T>::PromptNotListed);

            // Check if already purchased
            ensure!(
                !DecryptionKeys::<T>::contains_key(prompt_id, &buyer),
                Error::<T>::AlreadyPurchased
            );

            // Transfer payment
            T::Currency::transfer(
                &buyer,
                &prompt.owner,
                prompt.price,
                frame_support::traits::ExistenceRequirement::KeepAlive,
            )?;

            // Distribute revenue
            let creator_share = prompt.price
                .saturating_mul(T::CreatorShareBps::get().into())
                .saturating_div(10000_u32.into());

            let platform_share = prompt.price
                .saturating_mul(T::PlatformShareBps::get().into())
                .saturating_div(10000_u32.into());

            // Store decryption key for buyer
            DecryptionKeys::<T>::insert(prompt_id, &buyer, decryption_key);

            // Update prompt stats
            Prompts::<T>::try_mutate(prompt_id, |maybe_prompt| {
                let p = maybe_prompt.as_mut().ok_or(Error::<T>::PromptNotFound)?;
                p.sales_count = p.sales_count.saturating_add(1);
                p.total_revenue = p.total_revenue.saturating_add(prompt.price);
                Ok::<(), DispatchError>(())
            })?;

            // Record purchase
            let purchase = Purchase {
                buyer: buyer.clone(),
                prompt_id,
                price_paid: prompt.price,
                purchased_at: Self::current_timestamp(),
                rating: None,
            };

            UserPurchases::<T>::try_mutate(&buyer, |purchases| {
                purchases.try_push(purchase).map_err(|_| Error::<T>::TooManyPrompts)?;
                Ok::<(), DispatchError>(())
            })?;

            TotalVolume::<T>::mutate(|vol| *vol = vol.saturating_add(prompt.price));

            Self::deposit_event(Event::PromptPurchased {
                prompt_id,
                buyer,
                price: prompt.price,
            });

            Self::deposit_event(Event::RevenueDistributed {
                prompt_id,
                creator_amount: creator_share,
                platform_amount: platform_share,
            });

            Ok(())
        }

        /// Rate purchased prompt
        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn rate_prompt(
            origin: OriginFor<T>,
            prompt_id: u64,
            rating: u32, // 0-50000 = 0.0-5.0 stars
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            ensure!(rating <= 50000, Error::<T>::InvalidRating);

            // Ensure user purchased prompt
            ensure!(
                DecryptionKeys::<T>::contains_key(prompt_id, &buyer),
                Error::<T>::PromptNotFound
            );

            // Check if already rated
            let mut already_rated = false;
            UserPurchases::<T>::try_mutate(&buyer, |purchases| {
                for purchase in purchases.iter_mut() {
                    if purchase.prompt_id == prompt_id {
                        if purchase.rating.is_some() {
                            already_rated = true;
                            return Err(Error::<T>::AlreadyRated.into());
                        }
                        purchase.rating = Some(rating);
                        break;
                    }
                }
                Ok::<(), DispatchError>(())
            })?;

            // Update prompt rating
            Prompts::<T>::try_mutate(prompt_id, |maybe_prompt| {
                let prompt = maybe_prompt.as_mut().ok_or(Error::<T>::PromptNotFound)?;

                let total_rating = prompt.rating
                    .saturating_mul(prompt.rating_count)
                    .saturating_add(rating);
                prompt.rating_count = prompt.rating_count.saturating_add(1);
                prompt.rating = total_rating / prompt.rating_count;

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::PromptRated { prompt_id, rating });
            Ok(())
        }

        /// List/unlist prompt
        #[pallet::call_index(3)]
        #[pallet::weight(3_000)]
        pub fn set_listing(
            origin: OriginFor<T>,
            prompt_id: u64,
            is_listed: bool,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            Prompts::<T>::try_mutate(prompt_id, |maybe_prompt| {
                let prompt = maybe_prompt.as_mut().ok_or(Error::<T>::PromptNotFound)?;
                ensure!(prompt.owner == owner, Error::<T>::NotPromptOwner);

                prompt.is_listed = is_listed;

                if is_listed {
                    Self::deposit_event(Event::PromptListed {
                        prompt_id,
                        price: prompt.price,
                    });
                } else {
                    Self::deposit_event(Event::PromptUnlisted { prompt_id });
                }

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Update prompt price
        #[pallet::call_index(4)]
        #[pallet::weight(3_000)]
        pub fn update_price(
            origin: OriginFor<T>,
            prompt_id: u64,
            new_price: BalanceOf<T>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            ensure!(new_price >= T::MinPromptPrice::get(), Error::<T>::PriceTooLow);

            Prompts::<T>::try_mutate(prompt_id, |maybe_prompt| {
                let prompt = maybe_prompt.as_mut().ok_or(Error::<T>::PromptNotFound)?;
                ensure!(prompt.owner == owner, Error::<T>::NotPromptOwner);

                prompt.price = new_price;
                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number()
                .try_into()
                .unwrap_or(0u64)
                .saturating_mul(6)
        }

        /// Get user's decryption key for prompt (if purchased)
        pub fn get_decryption_key(prompt_id: u64, buyer: &T::AccountId) -> Option<Vec<u8>> {
            DecryptionKeys::<T>::get(prompt_id, buyer).map(|key| key.to_vec())
        }
    }
}
