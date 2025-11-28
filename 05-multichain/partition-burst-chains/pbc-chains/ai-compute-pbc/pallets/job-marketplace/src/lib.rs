//! # Job Marketplace Pallet
//!
//! ## Overview
//! The Job Marketplace pallet manages AI compute job submission, matching, and payment escrow.
//!
//! ## Features
//! - Submit AI inference/training jobs
//! - Automatic GPU matching based on requirements
//! - Escrow payments (funds locked until job completion)
//! - Job result verification
//! - Dispute resolution
//!
//! ## Flow
//! 1. User submits job + payment → funds escrowed
//! 2. System matches job to available GPU
//! 3. GPU processes job → submits result hash
//! 4. User confirms result → payment released
//! 5. Reputation updated for both parties

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub, Saturating, Hash};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// AI model types
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ModelType {
        /// Large Language Model (GPT, Claude, etc.)
        LLM,
        /// Image Generation (Stable Diffusion, DALL-E, etc.)
        ImageGen,
        /// Image Recognition (ResNet, YOLO, etc.)
        ImageRecognition,
        /// Audio Processing (Whisper, etc.)
        AudioProcessing,
        /// Video Processing
        VideoProcessing,
        /// Custom Model
        Custom,
    }

    /// Job priority tier (affects price and assignment speed)
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Priority {
        /// Standard queue (cheapest)
        Economy,
        /// Higher priority
        Standard,
        /// Fastest assignment (most expensive)
        Premium,
    }

    /// Job status
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum JobStatus {
        /// Waiting for GPU assignment
        Pending,
        /// Assigned to GPU, processing
        Processing,
        /// Completed successfully
        Completed,
        /// Failed (GPU error, timeout, etc.)
        Failed,
        /// Disputed (user contests result)
        Disputed,
        /// Cancelled by user
        Cancelled,
    }

    /// AI compute job
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Job<T: Config> {
        /// Job submitter
        pub submitter: T::AccountId,
        /// Model ID (from model-registry pallet)
        pub model_id: u64,
        /// Model type
        pub model_type: ModelType,
        /// Input data hash (actual data stored off-chain)
        pub input_hash: T::Hash,
        /// Expected output format
        pub output_format: BoundedVec<u8, ConstU32<32>>,
        /// Max compute time (in seconds)
        pub max_compute_time: u32,
        /// Payment amount (escrowed)
        pub payment: BalanceOf<T>,
        /// Priority tier
        pub priority: Priority,
        /// Current status
        pub status: JobStatus,
        /// Assigned GPU ID (if any)
        pub assigned_gpu: Option<u64>,
        /// Result hash (filled after completion)
        pub result_hash: Option<T::Hash>,
        /// Submission timestamp
        pub submitted_at: u64,
        /// Completion timestamp
        pub completed_at: Option<u64>,
    }

    /// GPU requirements for job
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct GpuRequirements {
        /// Minimum VRAM in GB
        pub min_vram_gb: u16,
        /// Minimum compute units
        pub min_compute_units: u32,
        /// Minimum reputation score (0-100)
        pub min_reputation: u8,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type (ËDSC)
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Platform fee percentage (in basis points, e.g., 500 = 5%)
        #[pallet::constant]
        type PlatformFeeBps: Get<u16>;

        /// Maximum job duration (in blocks)
        #[pallet::constant]
        type MaxJobDuration: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Jobs by ID
    #[pallet::storage]
    #[pallet::getter(fn jobs)]
    pub type Jobs<T: Config> = StorageMap<_, Blake2_128Concat, u64, Job<T>>;

    /// Job ID counter
    #[pallet::storage]
    #[pallet::getter(fn next_job_id)]
    pub type NextJobId<T> = StorageValue<_, u64, ValueQuery>;

    /// User's job IDs
    #[pallet::storage]
    #[pallet::getter(fn user_jobs)]
    pub type UserJobs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u64, ConstU32<1000>>, // Max 1000 jobs per user
        ValueQuery,
    >;

    /// GPU's job queue
    #[pallet::storage]
    #[pallet::getter(fn gpu_jobs)]
    pub type GpuJobs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64, // GPU ID
        BoundedVec<u64, ConstU32<100>>, // Max 100 queued jobs per GPU
        ValueQuery,
    >;

    /// Pending jobs (waiting for assignment)
    #[pallet::storage]
    #[pallet::getter(fn pending_jobs)]
    pub type PendingJobs<T> = StorageValue<_, BoundedVec<u64, ConstU32<10000>>, ValueQuery>;

    /// Total jobs processed (metrics)
    #[pallet::storage]
    #[pallet::getter(fn total_jobs)]
    pub type TotalJobs<T> = StorageValue<_, u64, ValueQuery>;

    /// Total volume (in ËDSC)
    #[pallet::storage]
    #[pallet::getter(fn total_volume)]
    pub type TotalVolume<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Job submitted [job_id, submitter, payment]
        JobSubmitted { job_id: u64, submitter: T::AccountId, payment: BalanceOf<T> },
        /// Job assigned to GPU [job_id, gpu_id]
        JobAssigned { job_id: u64, gpu_id: u64 },
        /// Job completed [job_id, result_hash]
        JobCompleted { job_id: u64, result_hash: T::Hash },
        /// Job failed [job_id, reason]
        JobFailed { job_id: u64, reason: BoundedVec<u8, ConstU32<128>> },
        /// Payment released [job_id, provider, amount]
        PaymentReleased { job_id: u64, provider: T::AccountId, amount: BalanceOf<T> },
        /// Job disputed [job_id]
        JobDisputed { job_id: u64 },
        /// Job cancelled [job_id]
        JobCancelled { job_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Job not found
        JobNotFound,
        /// Not the job submitter
        NotJobSubmitter,
        /// Job not in correct status
        InvalidJobStatus,
        /// Insufficient balance for payment
        InsufficientBalance,
        /// GPU not found
        GpuNotFound,
        /// Overflow in arithmetic
        Overflow,
        /// Job queue full
        JobQueueFull,
        /// Invalid result
        InvalidResult,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit a new AI compute job
        ///
        /// # Arguments
        /// * `origin` - Job submitter
        /// * `model_id` - AI model to use (from model-registry)
        /// * `model_type` - Type of model
        /// * `input_hash` - Hash of input data (stored off-chain)
        /// * `output_format` - Expected output format (JSON, image, etc.)
        /// * `max_compute_time` - Maximum seconds for job
        /// * `payment` - Payment amount in ËDSC
        /// * `priority` - Priority tier
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn submit_job(
            origin: OriginFor<T>,
            model_id: u64,
            model_type: ModelType,
            input_hash: T::Hash,
            output_format: BoundedVec<u8, ConstU32<32>>,
            max_compute_time: u32,
            payment: BalanceOf<T>,
            priority: Priority,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            // Reserve payment (escrow)
            T::Currency::reserve(&submitter, payment)?;

            // Get next job ID
            let job_id = NextJobId::<T>::get();
            let next_id = job_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextJobId::<T>::put(next_id);

            // Create job
            let job = Job {
                submitter: submitter.clone(),
                model_id,
                model_type,
                input_hash,
                output_format,
                max_compute_time,
                payment,
                priority,
                status: JobStatus::Pending,
                assigned_gpu: None,
                result_hash: None,
                submitted_at: Self::current_timestamp(),
                completed_at: None,
            };

            // Store job
            Jobs::<T>::insert(job_id, job);

            // Add to user's jobs
            UserJobs::<T>::try_mutate(&submitter, |jobs| {
                jobs.try_push(job_id).map_err(|_| Error::<T>::JobQueueFull)?;
                Ok::<(), DispatchError>(())
            })?;

            // Add to pending queue
            PendingJobs::<T>::try_mutate(|pending| {
                pending.try_push(job_id).map_err(|_| Error::<T>::JobQueueFull)?;
                Ok::<(), DispatchError>(())
            })?;

            // Update metrics
            TotalJobs::<T>::mutate(|total| *total = total.saturating_add(1));
            TotalVolume::<T>::mutate(|volume| *volume = volume.saturating_add(payment));

            Self::deposit_event(Event::JobSubmitted { job_id, submitter, payment });
            Ok(())
        }

        /// Assign job to GPU (called by off-chain worker or manually)
        #[pallet::call_index(1)]
        #[pallet::weight(8_000)]
        pub fn assign_job(
            origin: OriginFor<T>,
            job_id: u64,
            gpu_id: u64,
        ) -> DispatchResult {
            ensure_root(origin)?; // Only system can assign

            Jobs::<T>::try_mutate(job_id, |maybe_job| {
                let job = maybe_job.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(job.status == JobStatus::Pending, Error::<T>::InvalidJobStatus);

                job.assigned_gpu = Some(gpu_id);
                job.status = JobStatus::Processing;

                Ok::<(), DispatchError>(())
            })?;

            // Add to GPU's job queue
            GpuJobs::<T>::try_mutate(gpu_id, |jobs| {
                jobs.try_push(job_id).map_err(|_| Error::<T>::JobQueueFull)?;
                Ok::<(), DispatchError>(())
            })?;

            // Remove from pending queue
            PendingJobs::<T>::mutate(|pending| {
                pending.retain(|&id| id != job_id);
            });

            Self::deposit_event(Event::JobAssigned { job_id, gpu_id });
            Ok(())
        }

        /// Submit job result (called by GPU provider)
        #[pallet:call_index(2)]
        #[pallet::weight(10_000)]
        pub fn submit_result(
            origin: OriginFor<T>,
            job_id: u64,
            result_hash: T::Hash,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            Jobs::<T>::try_mutate(job_id, |maybe_job| {
                let job = maybe_job.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(job.status == JobStatus::Processing, Error::<T>::InvalidJobStatus);

                // TODO: Verify provider owns the assigned GPU

                job.result_hash = Some(result_hash);
                job.status = JobStatus::Completed;
                job.completed_at = Some(Self::current_timestamp());

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::JobCompleted { job_id, result_hash });
            Ok(())
        }

        /// Confirm job and release payment (called by job submitter)
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn confirm_job(origin: OriginFor<T>, job_id: u64) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            let job = Jobs::<T>::get(job_id).ok_or(Error::<T>::JobNotFound)?;
            ensure!(job.submitter == submitter, Error::<T>::NotJobSubmitter);
            ensure!(job.status == JobStatus::Completed, Error::<T>::InvalidJobStatus);

            // Calculate platform fee
            let platform_fee = job.payment
                .saturating_mul(T::PlatformFeeBps::get().into())
                .saturating_div(10000_u32.into());

            let provider_payment = job.payment.saturating_sub(platform_fee);

            // Release payment to provider (need to get provider account from GPU)
            // For now, simplified - would integrate with gpu-registry pallet
            T::Currency::unreserve(&submitter, job.payment);

            // TODO: Transfer to GPU provider after getting account from gpu-registry
            // T::Currency::transfer(&submitter, &provider, provider_payment, ExistenceRequirement::KeepAlive)?;

            Self::deposit_event(Event::PaymentReleased {
                job_id,
                provider: submitter.clone(), // Placeholder
                amount: provider_payment,
            });

            Ok(())
        }

        /// Report job failure
        #[pallet::call_index(4)]
        #[pallet::weight(8_000)]
        pub fn report_failure(
            origin: OriginFor<T>,
            job_id: u64,
            reason: BoundedVec<u8, ConstU32<128>>,
        ) -> DispatchResult {
            let _provider = ensure_signed(origin)?;

            Jobs::<T>::try_mutate(job_id, |maybe_job| {
                let job = maybe_job.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(job.status == JobStatus::Processing, Error::<T>::InvalidJobStatus);

                job.status = JobStatus::Failed;
                job.completed_at = Some(Self::current_timestamp());

                // Refund payment to submitter
                T::Currency::unreserve(&job.submitter, job.payment);

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::JobFailed { job_id, reason });
            Ok(())
        }

        /// Dispute job result
        #[pallet::call_index(5)]
        #[pallet::weight(8_000)]
        pub fn dispute_job(origin: OriginFor<T>, job_id: u64) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            Jobs::<T>::try_mutate(job_id, |maybe_job| {
                let job = maybe_job.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(job.submitter == submitter, Error::<T>::NotJobSubmitter);
                ensure!(job.status == JobStatus::Completed, Error::<T>::InvalidJobStatus);

                job.status = JobStatus::Disputed;

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::JobDisputed { job_id });
            Ok(())
        }

        /// Cancel pending job
        #[pallet::call_index(6)]
        #[pallet::weight(5_000)]
        pub fn cancel_job(origin: OriginFor<T>, job_id: u64) -> DispatchResult {
            let submitter = ensure_signed(origin)?;

            Jobs::<T>::try_mutate(job_id, |maybe_job| {
                let job = maybe_job.as_mut().ok_or(Error::<T>::JobNotFound)?;
                ensure!(job.submitter == submitter, Error::<T>::NotJobSubmitter);
                ensure!(job.status == JobStatus::Pending, Error::<T>::InvalidJobStatus);

                job.status = JobStatus::Cancelled;

                // Refund payment
                T::Currency::unreserve(&submitter, job.payment);

                Ok::<(), DispatchError>(())
            })?;

            // Remove from pending queue
            PendingJobs::<T>::mutate(|pending| {
                pending.retain(|&id| id != job_id);
            });

            Self::deposit_event(Event::JobCancelled { job_id });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get current timestamp (in seconds)
        fn current_timestamp() -> u64 {
            <frame_system::Pallet<T>>::block_number()
                .try_into()
                .unwrap_or(0u64)
                .saturating_mul(6) // 6-second blocks
        }

        /// Calculate job price based on model, compute time, and priority
        pub fn estimate_job_price(
            model_type: ModelType,
            compute_time_seconds: u32,
            priority: Priority,
        ) -> u128 {
            // Base prices (in micro-ËDSC per second)
            let base_price_per_sec: u128 = match model_type {
                ModelType::LLM => 1000, // $0.001/sec for LLM inference
                ModelType::ImageGen => 500, // $0.0005/sec
                ModelType::ImageRecognition => 300,
                ModelType::AudioProcessing => 400,
                ModelType::VideoProcessing => 2000,
                ModelType::Custom => 1000,
            };

            // Priority multipliers
            let priority_multiplier = match priority {
                Priority::Economy => 100, // 1.0x
                Priority::Standard => 150, // 1.5x
                Priority::Premium => 300, // 3.0x
            };

            let base_cost = base_price_per_sec
                .saturating_mul(compute_time_seconds as u128)
                .saturating_mul(priority_multiplier)
                .saturating_div(100);

            base_cost
        }
    }
}
