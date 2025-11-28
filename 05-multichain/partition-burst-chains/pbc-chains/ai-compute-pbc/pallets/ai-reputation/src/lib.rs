//! # AI Reputation Pallet
//!
//! ML-based reputation scoring that predicts GPU reliability.
//! Uses historical data to calculate trust scores.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct ReputationScore {
        pub trust_score: u16, // 0-10000 (0.00% - 100.00%)
        pub reliability: u16,
        pub speed: u16,
        pub quality: u16,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Scores<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ReputationScore, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ScoreUpdated { account: T::AccountId, score: ReputationScore },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(5_000)]
        pub fn update_score(
            origin: OriginFor<T>,
            account: T::AccountId,
            score: ReputationScore,
        ) -> DispatchResult {
            ensure_root(origin)?;
            Scores::<T>::insert(&account, score.clone());
            Self::deposit_event(Event::ScoreUpdated { account, score });
            Ok(())
        }
    }
}
