//! Etrid Consensus Pallet - Adaptive Stake-weighted Finality (ASF)
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Randomness, Time},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;
    use sp_std::vec::Vec;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum PeerType {
        Common,
        Staking,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Validator<AccountId, Balance, BlockNumber> {
        pub address: AccountId,
        pub stake: Balance,
        pub reputation: u64,
        pub peer_type: PeerType,
        pub last_block: BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
        type Time: Time;
        type MinStake: Get<BalanceOf<Self>>;
        type ValidatorReward: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::storage]
    #[pallet::getter(fn validators)]
    pub type Validators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Validator<T::AccountId, BalanceOf<T>, T::BlockNumber>>;

    #[pallet::storage]
    #[pallet::getter(fn active_validator_set)]
    pub type ActiveValidators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ValidatorRegistered(T::AccountId),
        ValidatorSelected(T::AccountId, T::BlockNumber),
        ValidatorSlashed(T::AccountId, &'static str),
        BlockFinalized(T::BlockNumber),
        ValidatorRewarded(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyValidator,
        NotEnoughStake,
        NotAValidator,
        FinalizationFailed,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_validator(origin: OriginFor<T>, peer_type: PeerType) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(!Validators::<T>::contains_key(&who), Error::<T>::AlreadyValidator);

            let stake = T::MinStake::get();
            T::Currency::reserve(&who, stake)?;

            let validator = Validator {
                address: who.clone(),
                stake,
                reputation: 0,
                peer_type,
                last_block: Zero::zero(),
            };

            Validators::<T>::insert(&who, validator);
            ActiveValidators::<T>::mutate(|v| v.push(who.clone()));
            Self::deposit_event(Event::ValidatorRegistered(who));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn select_validator(block_number: T::BlockNumber) -> Option<T::AccountId> {
            let validators = ActiveValidators::<T>::get();
            if validators.is_empty() {
                return None;
            }

            let seed = T::RandomnessSource::random(&block_number.encode());
            let seed_num = T::Hashing::hash_of(&seed).as_ref()[0] as usize;
            let index = seed_num % validators.len();
            let selected = validators.get(index)?.clone();

            Self::deposit_event(Event::ValidatorSelected(selected.clone(), block_number));
            Some(selected)
        }

        pub fn finalize_block(block_number: T::BlockNumber, votes: Vec<T::AccountId>) -> DispatchResult {
            let validators = ActiveValidators::<T>::get();
            let total = validators.len();
            let threshold = (total * 2) / 3;

            if votes.len() >= threshold {
                for v in votes.iter() {
                    Self::reward_validator(v.clone())?;
                }
                Self::deposit_event(Event::BlockFinalized(block_number));
                Ok(())
            } else {
                Err(Error::<T>::FinalizationFailed.into())
            }
        }

        pub fn reward_validator(who: T::AccountId) -> DispatchResult {
            let reward = T::ValidatorReward::get();
            T::Currency::unreserve(&who, reward);
            T::Currency::deposit_creating(&who, reward);
            Self::deposit_event(Event::ValidatorRewarded(who, reward));
            Ok(())
        }

        pub fn slash_validator(who: T::AccountId, reason: &'static str) -> DispatchResult {
            Validators::<T>::mutate_exists(&who, |v| {
                if let Some(mut val) = v.take() {
                    val.reputation = 0;
                    *v = Some(val);
                }
                Ok::<(), DispatchError>(())
            })?;
            Self::deposit_event(Event::ValidatorSlashed(who, reason));
            Ok(())
        }
    }
}
