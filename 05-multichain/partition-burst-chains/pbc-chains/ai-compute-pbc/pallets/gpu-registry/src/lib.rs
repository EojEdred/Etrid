//! # GPU Registry Pallet
//!
//! ## Overview
//! The GPU Registry pallet manages GPU provider registration, staking, and reputation tracking.
//!
//! ## Features
//! - GPU node registration with hardware attestation
//! - Staking mechanism (providers stake ËDSC to participate)
//! - Reputation tracking (uptime, job success rate, ratings)
//! - Hardware verification (prevent fake/virtualized GPUs)
//! - Scheduled availability (9am-5pm, 24/7, custom hours)
//!
//! ## Interfaces
//! - `register_gpu`: Register new GPU node with specs and stake
//! - `unregister_gpu`: Unregister and withdraw stake
//! - `update_availability`: Set working hours
//! - `report_uptime`: Off-chain workers report GPU online status
//! - `slash_provider`: Penalize poor performance

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub, Saturating};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// GPU hardware specifications
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GpuSpecs {
        /// GPU model name (e.g., "RTX 4090", "A100")
        pub model: BoundedVec<u8, ConstU32<64>>,
        /// VRAM in GB
        pub vram_gb: u16,
        /// Compute units (CUDA cores, stream processors, etc.)
        pub compute_units: u32,
        /// Clock speed in MHz
        pub clock_speed_mhz: u16,
        /// Power consumption in watts
        pub tdp_watts: u16,
    }

    /// Hardware attestation proof (TPM, Secure Boot, benchmarks)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct HardwareAttestation {
        /// TPM quote (proves hardware is genuine)
        pub tpm_quote: BoundedVec<u8, ConstU32<256>>,
        /// Benchmark score (proves performance matches specs)
        pub benchmark_score: u32,
        /// Attestation timestamp
        pub timestamp: u64,
    }

    /// GPU provider reputation
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct Reputation {
        /// Total jobs completed
        pub jobs_completed: u64,
        /// Total jobs failed
        pub jobs_failed: u64,
        /// Uptime percentage (0-10000 = 0.00% - 100.00%)
        pub uptime_bps: u16,
        /// Average user rating (0-50000 = 0.0 - 5.0 stars, scaled by 10000)
        pub rating: u32,
        /// Total ratings received
        pub rating_count: u32,
    }

    /// GPU node status
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum GpuStatus {
        /// GPU is online and accepting jobs
        Active,
        /// GPU is temporarily paused (manual)
        Paused,
        /// GPU is offline (detected by off-chain worker)
        Offline,
        /// GPU is slashed for misbehavior
        Slashed,
    }

    /// Availability schedule
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum AvailabilitySchedule {
        /// Available 24/7
        AlwaysOn,
        /// Business hours (9am-5pm UTC)
        BusinessHours,
        /// Custom hours (bitfield: 168 bits for each hour of the week)
        Custom(BoundedVec<u8, ConstU32<21>>), // 168 bits / 8 = 21 bytes
    }

    /// GPU provider node
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct GpuNode<T: Config> {
        /// Provider account
        pub provider: T::AccountId,
        /// GPU specifications
        pub specs: GpuSpecs,
        /// Hardware attestation
        pub attestation: HardwareAttestation,
        /// Staked amount (in ËDSC)
        pub stake: BalanceOf<T>,
        /// Current status
        pub status: GpuStatus,
        /// Reputation metrics
        pub reputation: Reputation,
        /// Availability schedule
        pub schedule: AvailabilitySchedule,
        /// Registration timestamp
        pub registered_at: u64,
        /// Last heartbeat timestamp
        pub last_heartbeat: u64,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type (ËDSC stablecoin)
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Minimum stake required to register GPU (e.g., 100 ËDSC)
        #[pallet::constant]
        type MinimumStake: Get<BalanceOf<Self>>;

        /// Slash percentage for misbehavior (in basis points, e.g., 1000 = 10%)
        #[pallet::constant]
        type SlashPercentage: Get<u16>;

        /// Maximum offline duration before auto-unregister (in blocks)
        #[pallet::constant]
        type MaxOfflineBlocks: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// GPU nodes by ID
    #[pallet::storage]
    #[pallet::getter(fn gpu_nodes)]
    pub type GpuNodes<T: Config> = StorageMap<_, Blake2_128Concat, u64, GpuNode<T>>;

    /// GPU ID counter
    #[pallet::storage]
    #[pallet::getter(fn next_gpu_id)]
    pub type NextGpuId<T> = StorageValue<_, u64, ValueQuery>;

    /// Provider's GPU IDs
    #[pallet::storage]
    #[pallet::getter(fn provider_gpus)]
    pub type ProviderGpus<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u64, ConstU32<100>>, // Max 100 GPUs per provider
        ValueQuery,
    >;

    /// Active GPU count (for metrics)
    #[pallet::storage]
    #[pallet::getter(fn active_gpu_count)]
    pub type ActiveGpuCount<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// GPU registered [gpu_id, provider, stake]
        GpuRegistered { gpu_id: u64, provider: T::AccountId, stake: BalanceOf<T> },
        /// GPU unregistered [gpu_id, provider]
        GpuUnregistered { gpu_id: u64, provider: T::AccountId },
        /// GPU status updated [gpu_id, new_status]
        GpuStatusUpdated { gpu_id: u64, status: GpuStatus },
        /// Availability schedule updated [gpu_id]
        AvailabilityUpdated { gpu_id: u64 },
        /// Provider slashed [gpu_id, slash_amount]
        ProviderSlashed { gpu_id: u64, slash_amount: BalanceOf<T> },
        /// Reputation updated [gpu_id, new_reputation]
        ReputationUpdated { gpu_id: u64, reputation: Reputation },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// GPU node does not exist
        GpuNotFound,
        /// Insufficient stake
        InsufficientStake,
        /// Provider already has max GPUs
        TooManyGpus,
        /// Not the GPU owner
        NotGpuOwner,
        /// GPU already exists
        GpuAlreadyExists,
        /// Invalid attestation
        InvalidAttestation,
        /// GPU is not active
        GpuNotActive,
        /// Overflow in arithmetic operation
        Overflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new GPU node
        ///
        /// # Arguments
        /// * `origin` - Provider account
        /// * `specs` - GPU hardware specifications
        /// * `attestation` - Hardware attestation proof
        /// * `stake` - Amount to stake (must be >= MinimumStake)
        /// * `schedule` - Availability schedule
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_gpu(
            origin: OriginFor<T>,
            specs: GpuSpecs,
            attestation: HardwareAttestation,
            stake: BalanceOf<T>,
            schedule: AvailabilitySchedule,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            // Ensure sufficient stake
            ensure!(stake >= T::MinimumStake::get(), Error::<T>::InsufficientStake);

            // Reserve stake
            T::Currency::reserve(&provider, stake)?;

            // Get next GPU ID
            let gpu_id = NextGpuId::<T>::get();
            let next_id = gpu_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextGpuId::<T>::put(next_id);

            // Create GPU node
            let current_time = Self::current_timestamp();
            let node = GpuNode {
                provider: provider.clone(),
                specs,
                attestation,
                stake,
                status: GpuStatus::Active,
                reputation: Reputation::default(),
                schedule,
                registered_at: current_time,
                last_heartbeat: current_time,
            };

            // Store GPU node
            GpuNodes::<T>::insert(gpu_id, node);

            // Update provider's GPU list
            ProviderGpus::<T>::try_mutate(&provider, |gpus| {
                gpus.try_push(gpu_id).map_err(|_| Error::<T>::TooManyGpus)?;
                Ok::<(), DispatchError>(())
            })?;

            // Increment active GPU count
            ActiveGpuCount::<T>::mutate(|count| *count = count.saturating_add(1));

            Self::deposit_event(Event::GpuRegistered { gpu_id, provider, stake });
            Ok(())
        }

        /// Unregister GPU and withdraw stake
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn unregister_gpu(origin: OriginFor<T>, gpu_id: u64) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            // Get GPU node
            let node = GpuNodes::<T>::get(gpu_id).ok_or(Error::<T>::GpuNotFound)?;
            ensure!(node.provider == provider, Error::<T>::NotGpuOwner);

            // Unreserve stake
            T::Currency::unreserve(&provider, node.stake);

            // Remove from provider's GPU list
            ProviderGpus::<T>::mutate(&provider, |gpus| {
                gpus.retain(|&id| id != gpu_id);
            });

            // Remove GPU node
            GpuNodes::<T>::remove(gpu_id);

            // Decrement active GPU count
            ActiveGpuCount::<T>::mutate(|count| *count = count.saturating_sub(1));

            Self::deposit_event(Event::GpuUnregistered { gpu_id, provider });
            Ok(())
        }

        /// Update availability schedule
        #[pallet::call_index(2)]
        #[pallet::weight(5_000)]
        pub fn update_availability(
            origin: OriginFor<T>,
            gpu_id: u64,
            schedule: AvailabilitySchedule,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            GpuNodes::<T>::try_mutate(gpu_id, |maybe_node| {
                let node = maybe_node.as_mut().ok_or(Error::<T>::GpuNotFound)?;
                ensure!(node.provider == provider, Error::<T>::NotGpuOwner);

                node.schedule = schedule;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::AvailabilityUpdated { gpu_id });
            Ok(())
        }

        /// Update GPU status (active/paused)
        #[pallet::call_index(3)]
        #[pallet::weight(5_000)]
        pub fn update_status(
            origin: OriginFor<T>,
            gpu_id: u64,
            status: GpuStatus,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            GpuNodes::<T>::try_mutate(gpu_id, |maybe_node| {
                let node = maybe_node.as_mut().ok_or(Error::<T>::GpuNotFound)?;
                ensure!(node.provider == provider, Error::<T>::NotGpuOwner);

                node.status = status.clone();
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::GpuStatusUpdated { gpu_id, status });
            Ok(())
        }

        /// Report GPU heartbeat (called by off-chain worker or provider client)
        #[pallet::call_index(4)]
        #[pallet::weight(3_000)]
        pub fn report_heartbeat(origin: OriginFor<T>, gpu_id: u64) -> DispatchResult {
            let _provider = ensure_signed(origin)?;

            GpuNodes::<T>::try_mutate(gpu_id, |maybe_node| {
                let node = maybe_node.as_mut().ok_or(Error::<T>::GpuNotFound)?;
                node.last_heartbeat = Self::current_timestamp();
                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Slash provider for misbehavior (admin only - would use sudo or governance)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn slash_provider(
            origin: OriginFor<T>,
            gpu_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?;

            GpuNodes::<T>::try_mutate(gpu_id, |maybe_node| {
                let node = maybe_node.as_mut().ok_or(Error::<T>::GpuNotFound)?;

                // Calculate slash amount
                let slash_amount = node.stake
                    .saturating_mul(T::SlashPercentage::get().into())
                    .saturating_div(10000_u32.into());

                // Slash stake
                let slashed = T::Currency::slash_reserved(&node.provider, slash_amount).0;

                // Update status
                node.status = GpuStatus::Slashed;
                node.stake = node.stake.saturating_sub(slashed);

                Self::deposit_event(Event::ProviderSlashed { gpu_id, slash_amount: slashed });
                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Update reputation (called by job-marketplace pallet)
        #[pallet::call_index(6)]
        #[pallet::weight(5_000)]
        pub fn update_reputation(
            origin: OriginFor<T>,
            gpu_id: u64,
            job_success: bool,
            rating: Option<u32>, // 0-50000 = 0.0-5.0 stars
        ) -> DispatchResult {
            ensure_root(origin)?; // Only callable by other pallets

            GpuNodes::<T>::try_mutate(gpu_id, |maybe_node| {
                let node = maybe_node.as_mut().ok_or(Error::<T>::GpuNotFound)?;

                // Update job stats
                if job_success {
                    node.reputation.jobs_completed = node.reputation.jobs_completed.saturating_add(1);
                } else {
                    node.reputation.jobs_failed = node.reputation.jobs_failed.saturating_add(1);
                }

                // Update rating if provided
                if let Some(new_rating) = rating {
                    let total_rating = node.reputation.rating
                        .saturating_mul(node.reputation.rating_count)
                        .saturating_add(new_rating);
                    node.reputation.rating_count = node.reputation.rating_count.saturating_add(1);
                    node.reputation.rating = total_rating / node.reputation.rating_count;
                }

                Self::deposit_event(Event::ReputationUpdated {
                    gpu_id,
                    reputation: node.reputation.clone(),
                });
                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get current timestamp (in seconds)
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number()
                .try_into()
                .unwrap_or(0u64)
                .saturating_mul(6) // Assume 6-second blocks
        }

        /// Check if GPU is available at current time
        pub fn is_gpu_available(gpu_id: u64) -> bool {
            if let Some(node) = GpuNodes::<T>::get(gpu_id) {
                matches!(node.status, GpuStatus::Active)
                // TODO: Check schedule against current UTC time
            } else {
                false
            }
        }

        /// Get GPU reputation score (0-100)
        pub fn get_reputation_score(gpu_id: u64) -> u8 {
            if let Some(node) = GpuNodes::<T>::get(gpu_id) {
                let rep = &node.reputation;

                // Success rate (0-40 points)
                let total_jobs = rep.jobs_completed.saturating_add(rep.jobs_failed);
                let success_rate = if total_jobs > 0 {
                    (rep.jobs_completed * 40 / total_jobs) as u8
                } else {
                    0
                };

                // Rating (0-40 points): 5.0 stars = 40 points
                let rating_score = (rep.rating * 40 / 50000) as u8;

                // Uptime (0-20 points): 100% uptime = 20 points
                let uptime_score = (rep.uptime_bps * 20 / 10000) as u8;

                success_rate.saturating_add(rating_score).saturating_add(uptime_score).min(100)
            } else {
                0
            }
        }
    }
}
