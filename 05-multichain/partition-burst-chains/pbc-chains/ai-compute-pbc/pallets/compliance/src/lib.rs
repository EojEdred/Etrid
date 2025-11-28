//! # Compliance Pallet - Regulatory Templates
//!
//! One-click HIPAA, GDPR, SOC2, CCPA, FedRAMP compliance modes.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ComplianceTemplate {
        HIPAA,
        GDPR,
        SOC2,
        CCPA,
        FedRAMP,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct ComplianceConfig<T: Config> {
        pub account: T::AccountId,
        pub template: ComplianceTemplate,
        pub data_residency: BoundedVec<u8, ConstU32<32>>, // e.g., "US", "EU"
        pub encryption_required: bool,
        pub audit_logs_required: bool,
        pub enabled_at: u64,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type ComplianceConfigs<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ComplianceConfig<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ComplianceEnabled { account: T::AccountId, template: ComplianceTemplate },
        ComplianceDisabled { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyEnabled,
        NotEnabled,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn enable_compliance(
            origin: OriginFor<T>,
            template: ComplianceTemplate,
            data_residency: BoundedVec<u8, ConstU32<32>>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            ensure!(!ComplianceConfigs::<T>::contains_key(&account), Error::<T>::AlreadyEnabled);

            let config = ComplianceConfig {
                account: account.clone(),
                template: template.clone(),
                data_residency,
                encryption_required: true,
                audit_logs_required: true,
                enabled_at: Self::current_timestamp(),
            };

            ComplianceConfigs::<T>::insert(&account, config);
            Self::deposit_event(Event::ComplianceEnabled { account, template });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn disable_compliance(origin: OriginFor<T>) -> DispatchResult {
            let account = ensure_signed(origin)?;
            ensure!(ComplianceConfigs::<T>::contains_key(&account), Error::<T>::NotEnabled);
            ComplianceConfigs::<T>::remove(&account);
            Self::deposit_event(Event::ComplianceDisabled { account });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number().try_into().unwrap_or(0u64).saturating_mul(6)
        }

        pub fn is_compliant(account: &T::AccountId, template: &ComplianceTemplate) -> bool {
            ComplianceConfigs::<T>::get(account).map_or(false, |config| config.template == *template)
        }
    }
}
