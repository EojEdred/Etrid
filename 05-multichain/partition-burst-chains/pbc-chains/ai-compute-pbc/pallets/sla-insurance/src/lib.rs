//! # SLA Insurance Pallet
//!
//! 99.9% uptime guarantee with insurance pool. SLA miss → 10x refund.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct SLA<T: Config> {
        pub gpu_provider: T::AccountId,
        pub insurance_stake: BalanceOf<T>,
        pub uptime_bps: u16, // 9990 = 99.9%
        pub total_payouts: BalanceOf<T>,
        pub is_active: bool,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        #[pallet::constant]
        type MinInsuranceStake: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type RefundMultiplier: Get<u8>; // 10 = 10x refund
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type SLAs<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, SLA<T>>;

    #[pallet::storage]
    pub type InsurancePool<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SLACreated { provider: T::AccountId, stake: BalanceOf<T> },
        SLAViolation { provider: T::AccountId, refund_amount: BalanceOf<T> },
        InsurancePayout { user: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        SLAAlreadyExists,
        SLANotFound,
        InsufficientStake,
        InsufficientInsurancePool,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_sla(origin: OriginFor<T>, stake: BalanceOf<T>) -> DispatchResult {
            let provider = ensure_signed(origin)?;
            ensure!(!SLAs::<T>::contains_key(&provider), Error::<T>::SLAAlreadyExists);
            ensure!(stake >= T::MinInsuranceStake::get(), Error::<T>::InsufficientStake);

            let sla = SLA {
                gpu_provider: provider.clone(),
                insurance_stake: stake,
                uptime_bps: 9990, // 99.9%
                total_payouts: 0_u32.into(),
                is_active: true,
            };

            SLAs::<T>::insert(&provider, sla);
            InsurancePool::<T>::mutate(|pool| *pool = pool.saturating_add(stake));

            Self::deposit_event(Event::SLACreated { provider, stake });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(15_000)]
        pub fn claim_sla_violation(
            origin: OriginFor<T>,
            provider: T::AccountId,
            job_cost: BalanceOf<T>,
        ) -> DispatchResult {
            let user = ensure_signed(origin)?;

            SLAs::<T>::try_mutate(&provider, |maybe_sla| {
                let sla = maybe_sla.as_mut().ok_or(Error::<T>::SLANotFound)?;

                let refund_amount = job_cost.saturating_mul(T::RefundMultiplier::get().into());

                ensure!(
                    InsurancePool::<T>::get() >= refund_amount,
                    Error::<T>::InsufficientInsurancePool
                );

                InsurancePool::<T>::mutate(|pool| *pool = pool.saturating_sub(refund_amount));
                sla.total_payouts = sla.total_payouts.saturating_add(refund_amount);

                // TODO: Transfer refund to user

                Self::deposit_event(Event::SLAViolation { provider, refund_amount });
                Self::deposit_event(Event::InsurancePayout { user, amount: refund_amount });

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }
}
