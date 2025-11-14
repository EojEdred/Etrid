#!/bin/bash

# This script creates all remaining pallet implementations

# GPU NFT Pallet
cat > pallets/gpu-nft/src/lib.rs << 'EOF'
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
EOF

# Compliance Pallet
cat > pallets/compliance/src/lib.rs << 'EOF'
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
EOF

# SLA Insurance Pallet
cat > pallets/sla-insurance/src/lib.rs << 'EOF'
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
EOF

echo "All pallet lib.rs files created successfully!"
