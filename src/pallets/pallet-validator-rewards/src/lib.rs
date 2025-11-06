//! # Pallet Validator Rewards
//!
//! Manages validator payment accounts, performance tracking, and reward distribution.
//!
//! ## Overview
//!
//! This pallet implements the payment system described in the Ëtrid Ivory Papers:
//! - Separates session keys from payment accounts (stash)
//! - Tracks validator performance (uptime, finality, block production)
//! - Calculates rewards with performance multipliers
//! - Distributes rewards to payment accounts (NOT session accounts)
//! - Handles delegator rewards with validator commission
//! - Integrates with slashing events
//!
//! ## Key Architecture
//!
//! Each validator has:
//! 1. **Session Account**: Hot keys for consensus (AURA, GRANDPA, ASF)
//! 2. **Payment Account**: Cold storage that receives rewards
//! 3. **Controller Account**: Manages validator operations (optional)
//!
//! ## Reward Formula (per Ivory Papers Vol III)
//!
//! ```text
//! base_reward = epoch_pool * (validator.stake / total_stake)
//! performance_multiplier = uptime_score * finality_score * participation_score
//! final_reward = base_reward * performance_multiplier
//!
//! validator_share = final_reward * 0.5
//! delegator_share = final_reward * 0.5
//! ```
//!
//! ## Extrinsics
//!
//! - `register_payment_account` - Map session account to payment account
//! - `distribute_epoch_rewards` - Calculate and distribute rewards to all validators
//! - `claim_rewards` - Validator claims accumulated rewards
//! - `update_payment_account` - Change payment account destination
//!
//! ## Storage
//!
//! - `PaymentAccounts` - Maps session account → payment account
//! - `ControllerAccounts` - Maps session account → controller account
//! - `ValidatorPerformance` - Tracks performance metrics per validator
//! - `PendingRewards` - Accumulated unclaimed rewards per payment account
//! - `EpochRewardPool` - Total rewards available for current epoch

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, DecodeWithMemTracking, Encode};
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;

/// Balance type for this pallet
pub type BalanceOf<T> =
    <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Performance metrics for a validator
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug, Default)]
pub struct PerformanceMetrics {
    /// Blocks authored this epoch
    pub blocks_authored: u32,
    /// Expected blocks for this epoch
    pub blocks_expected: u32,
    /// Finality votes signed
    pub finality_votes: u32,
    /// Expected finality votes
    pub finality_expected: u32,
    /// Uptime percentage (0-10000 = 0-100%)
    pub uptime_bps: u16,
    /// Participated in last Consensus Day
    pub consensus_day_participation: bool,
}

impl PerformanceMetrics {
    /// Calculate performance multiplier (0.0 - 1.2)
    /// Returns basis points (0-12000 = 0-120%)
    pub fn calculate_multiplier(&self) -> u32 {
        // Uptime score (0.9 - 1.1)
        let uptime_score = if self.uptime_bps >= 9500 {
            9000 + ((self.uptime_bps - 9500) * 4) // 0.9 to 1.1
        } else {
            (self.uptime_bps * 9) / 10 // Below 95% scales linearly
        }.min(11000);

        // Finality score (0.0 - 1.0)
        let finality_score = if self.finality_expected > 0 {
            (self.finality_votes as u64 * 10000 / self.finality_expected as u64) as u32
        } else {
            10000 // No expected votes = 100%
        };

        // Block production score (0.0 - 1.0)
        let block_score = if self.blocks_expected > 0 {
            (self.blocks_authored as u64 * 10000 / self.blocks_expected as u64) as u32
        } else {
            10000
        };

        // Consensus Day bonus (1.0 or 1.1)
        let participation_score = if self.consensus_day_participation {
            11000
        } else {
            10000
        };

        // Combined multiplier (multiply and divide by 10000)
        let combined = (uptime_score as u64)
            .saturating_mul(finality_score as u64)
            .saturating_mul(block_score as u64)
            .saturating_mul(participation_score as u64)
            / 1_000_000_000_000u64; // Divide by 10000^3

        combined.min(12000) as u32 // Cap at 120%
    }
}

/// Storage migrations for runtime upgrades
// TODO: Uncomment when pallet-validator-committee is added as dependency
// pub mod migrations;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Encode, Decode};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ReservableCurrency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Zero, Saturating, CheckedDiv, CheckedMul, AccountIdConversion};
    use sp_runtime::Permill;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Currency for rewards (ËTR)
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Epoch duration in blocks
        #[pallet::constant]
        type EpochDuration: Get<u32>;

        /// Annual validator reward pool percentage (basis points: 300 = 3%)
        #[pallet::constant]
        type AnnualRewardPoolBps: Get<u32>;

        /// Validator vs delegator split (basis points: 5000 = 50/50)
        #[pallet::constant]
        type ValidatorShareBps: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Maps session account → payment account (where rewards are sent)
    #[pallet::storage]
    #[pallet::getter(fn payment_account_of)]
    pub type PaymentAccounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Session account
        T::AccountId,  // Payment account
        OptionQuery,
    >;

    /// Maps session account → controller account (for management)
    #[pallet::storage]
    #[pallet::getter(fn controller_account_of)]
    pub type ControllerAccounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Session account
        T::AccountId,  // Controller account
        OptionQuery,
    >;

    /// Performance metrics per validator session account
    #[pallet::storage]
    #[pallet::getter(fn performance_of)]
    pub type ValidatorPerformance<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Session account
        PerformanceMetrics,
        ValueQuery,
    >;

    /// Pending unclaimed rewards per payment account
    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub type PendingRewards<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Payment account
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Total reward pool for current epoch
    #[pallet::storage]
    #[pallet::getter(fn epoch_reward_pool)]
    pub type EpochRewardPool<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current epoch number
    #[pallet::storage]
    #[pallet::getter(fn current_epoch)]
    pub type CurrentEpoch<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Total staked by all validators (for reward calculation)
    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    pub type TotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Validator stakes (session account → stake amount)
    #[pallet::storage]
    #[pallet::getter(fn validator_stake)]
    pub type ValidatorStakes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Session account
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Payment account registered [session_account, payment_account]
        PaymentAccountRegistered(T::AccountId, T::AccountId),
        /// Payment account updated [session_account, old_payment, new_payment]
        PaymentAccountUpdated(T::AccountId, T::AccountId, T::AccountId),
        /// Controller account registered [session_account, controller_account]
        ControllerAccountRegistered(T::AccountId, T::AccountId),
        /// Epoch rewards distributed [epoch, total_distributed]
        EpochRewardsDistributed(u32, BalanceOf<T>),
        /// Rewards claimed [payment_account, amount]
        RewardsClaimed(T::AccountId, BalanceOf<T>),
        /// Performance metrics updated [session_account, blocks_authored, finality_votes]
        PerformanceUpdated(T::AccountId, u32, u32),
        /// Reward calculated [session_account, payment_account, amount, multiplier_bps]
        RewardCalculated(T::AccountId, T::AccountId, BalanceOf<T>, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Payment account already registered
        PaymentAccountAlreadyRegistered,
        /// No payment account registered for this session account
        NoPaymentAccount,
        /// No pending rewards to claim
        NoPendingRewards,
        /// Insufficient reward pool
        InsufficientRewardPool,
        /// Invalid performance metrics
        InvalidPerformanceMetrics,
        /// Not authorized (must be session or controller account)
        NotAuthorized,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register payment account for validator
        ///
        /// Must be called by session account before receiving rewards
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_payment_account(
            origin: OriginFor<T>,
            payment_account: T::AccountId,
        ) -> DispatchResult {
            let session_account = ensure_signed(origin)?;

            ensure!(
                !PaymentAccounts::<T>::contains_key(&session_account),
                Error::<T>::PaymentAccountAlreadyRegistered
            );

            PaymentAccounts::<T>::insert(&session_account, &payment_account);

            Self::deposit_event(Event::PaymentAccountRegistered(
                session_account,
                payment_account,
            ));

            Ok(())
        }

        /// Update payment account destination
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn update_payment_account(
            origin: OriginFor<T>,
            new_payment_account: T::AccountId,
        ) -> DispatchResult {
            let session_account = ensure_signed(origin)?;

            let old_payment = PaymentAccounts::<T>::get(&session_account)
                .ok_or(Error::<T>::NoPaymentAccount)?;

            PaymentAccounts::<T>::insert(&session_account, &new_payment_account);

            Self::deposit_event(Event::PaymentAccountUpdated(
                session_account,
                old_payment,
                new_payment_account,
            ));

            Ok(())
        }

        /// Register controller account (optional, for key hierarchy)
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn register_controller_account(
            origin: OriginFor<T>,
            controller_account: T::AccountId,
        ) -> DispatchResult {
            let session_account = ensure_signed(origin)?;

            ControllerAccounts::<T>::insert(&session_account, &controller_account);

            Self::deposit_event(Event::ControllerAccountRegistered(
                session_account,
                controller_account,
            ));

            Ok(())
        }

        /// Claim accumulated rewards
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let payment_account = ensure_signed(origin)?;

            let pending = PendingRewards::<T>::get(&payment_account);
            ensure!(!pending.is_zero(), Error::<T>::NoPendingRewards);

            // Transfer rewards from pallet account to payment account
            T::Currency::transfer(
                &Self::account_id(),
                &payment_account,
                pending,
                ExistenceRequirement::KeepAlive,
            )?;

            // Clear pending rewards
            PendingRewards::<T>::remove(&payment_account);

            Self::deposit_event(Event::RewardsClaimed(payment_account, pending));

            Ok(())
        }

        /// Distribute epoch rewards to all validators (root/governance only)
        ///
        /// This should be called at the end of each epoch
        #[pallet::call_index(4)]
        #[pallet::weight(1_000_000)]
        pub fn distribute_epoch_rewards(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let epoch = CurrentEpoch::<T>::get();
            let total_pool = EpochRewardPool::<T>::get();
            let total_stake = TotalStaked::<T>::get();

            ensure!(!total_stake.is_zero(), Error::<T>::InsufficientRewardPool);

            let mut total_distributed = BalanceOf::<T>::zero();

            // Iterate all validators and calculate rewards
            for (session_account, stake) in ValidatorStakes::<T>::iter() {
                if let Some(payment_account) = PaymentAccounts::<T>::get(&session_account) {
                    // Base reward proportional to stake
                    let base_reward = total_pool
                        .saturating_mul(stake)
                        .checked_div(&total_stake)
                        .unwrap_or_else(|| BalanceOf::<T>::zero());

                    // Apply performance multiplier
                    let performance = ValidatorPerformance::<T>::get(&session_account);
                    let multiplier_bps = performance.calculate_multiplier();

                    let adjusted_reward = {
                        let multiplier: BalanceOf<T> = multiplier_bps.into();
                        let divisor: BalanceOf<T> = 10000u32.into();
                        base_reward.saturating_mul(multiplier) / divisor
                    };

                    // Validator gets 50% (delegators get other 50%, handled separately)
                    let validator_reward = {
                        let share_bps: BalanceOf<T> = T::ValidatorShareBps::get().into();
                        let divisor: BalanceOf<T> = 10000u32.into();
                        adjusted_reward.saturating_mul(share_bps) / divisor
                    };

                    // Add to pending rewards
                    PendingRewards::<T>::mutate(&payment_account, |pending| {
                        *pending = pending.saturating_add(validator_reward);
                    });

                    total_distributed = total_distributed.saturating_add(validator_reward);

                    Self::deposit_event(Event::RewardCalculated(
                        session_account,
                        payment_account,
                        validator_reward,
                        multiplier_bps,
                    ));
                }
            }

            // Increment epoch
            CurrentEpoch::<T>::mutate(|e| *e = e.saturating_add(1));

            Self::deposit_event(Event::EpochRewardsDistributed(epoch, total_distributed));

            Ok(())
        }

        /// Update validator performance metrics (called by consensus/runtime)
        #[pallet::call_index(5)]
        #[pallet::weight(5_000)]
        pub fn update_performance(
            origin: OriginFor<T>,
            session_account: T::AccountId,
            blocks_authored: u32,
            finality_votes: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ValidatorPerformance::<T>::mutate(&session_account, |metrics| {
                metrics.blocks_authored = blocks_authored;
                metrics.finality_votes = finality_votes;
            });

            Self::deposit_event(Event::PerformanceUpdated(
                session_account,
                blocks_authored,
                finality_votes,
            ));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Pallet account ID (holds reward pool before distribution)
        pub fn account_id() -> T::AccountId {
            frame_support::PalletId(*b"py/vlrwd").into_account_truncating()
        }

        /// Record block authorship (called by block production logic)
        pub fn note_block_authored(session_account: &T::AccountId) {
            ValidatorPerformance::<T>::mutate(session_account, |metrics| {
                metrics.blocks_authored = metrics.blocks_authored.saturating_add(1);
            });
        }

        /// Record finality vote (called by finality gadget)
        pub fn note_finality_vote(session_account: &T::AccountId) {
            ValidatorPerformance::<T>::mutate(session_account, |metrics| {
                metrics.finality_votes = metrics.finality_votes.saturating_add(1);
            });
        }

        /// Set expected blocks for epoch (called at epoch start)
        pub fn set_expected_blocks(session_account: &T::AccountId, expected: u32) {
            ValidatorPerformance::<T>::mutate(session_account, |metrics| {
                metrics.blocks_expected = expected;
            });
        }

        /// Set expected finality votes for epoch
        pub fn set_expected_finality_votes(session_account: &T::AccountId, expected: u32) {
            ValidatorPerformance::<T>::mutate(session_account, |metrics| {
                metrics.finality_expected = expected;
            });
        }
    }
}
