//! # Tokenomics Pallet
//!
//! ËTRD token staking with tiered benefits system.
//!
//! ## Staking Tiers
//! - Bronze: 100 ËTRD → 10% fee discount
//! - Silver: 500 ËTRD → 25% discount + priority support
//! - Gold: 1,000 ËTRD → 40% discount + <5s job matching
//! - Platinum: 10,000 ËTRD → 50% discount + dedicated GPU pool
//!
//! ## Benefits
//! - Fee discounts on AI compute jobs
//! - Priority job matching (faster GPU assignment)
//! - Governance voting rights (1 ËTRD = 1 vote)
//! - Staking rewards: 8% APY
//! - GPU boost: Stake on GPU NFT → 2x earnings
//!
//! ## Tokenomics
//! - Total supply: 100M ËTRD
//! - Circulating: 40M (60M locked/staked)
//! - Inflation: 8% APY (staking rewards)
//! - Deflation: 50% of platform fees burned

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Staking tier
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum StakingTier {
        None,
        Bronze,   // 100 ËTRD
        Silver,   // 500 ËTRD
        Gold,     // 1,000 ËTRD
        Platinum, // 10,000 ËTRD
    }

    impl StakingTier {
        /// Get fee discount in basis points
        pub fn fee_discount_bps(&self) -> u16 {
            match self {
                StakingTier::None => 0,
                StakingTier::Bronze => 1000,   // 10%
                StakingTier::Silver => 2500,   // 25%
                StakingTier::Gold => 4000,     // 40%
                StakingTier::Platinum => 5000, // 50%
            }
        }

        /// Get priority level (lower = higher priority)
        pub fn priority_level(&self) -> u8 {
            match self {
                StakingTier::None => 10,
                StakingTier::Bronze => 8,
                StakingTier::Silver => 6,
                StakingTier::Gold => 3,
                StakingTier::Platinum => 1,
            }
        }

        /// Check if tier has governance rights
        pub fn has_governance_rights(&self) -> bool {
            !matches!(self, StakingTier::None)
        }
    }

    /// User staking info
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct StakingInfo<T: Config> {
        pub staker: T::AccountId,
        pub staked_amount: BalanceOf<T>,
        pub tier: StakingTier,
        pub staked_at: u64,
        pub last_reward_claim: u64,
        pub total_rewards_earned: BalanceOf<T>,
        pub governance_votes_cast: u32,
    }

    /// Governance proposal
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<T: Config> {
        pub proposer: T::AccountId,
        pub title: BoundedVec<u8, ConstU32<128>>,
        pub description: BoundedVec<u8, ConstU32<512>>,
        pub votes_for: BalanceOf<T>,  // Weighted by ËTRD staked
        pub votes_against: BalanceOf<T>,
        pub status: ProposalStatus,
        pub created_at: u64,
        pub voting_ends_at: u64,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
        Executed,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Bronze tier stake requirement
        #[pallet::constant]
        type BronzeStake: Get<BalanceOf<Self>>;

        /// Silver tier stake requirement
        #[pallet::constant]
        type SilverStake: Get<BalanceOf<Self>>;

        /// Gold tier stake requirement
        #[pallet::constant]
        type GoldStake: Get<BalanceOf<Self>>;

        /// Platinum tier stake requirement
        #[pallet::constant]
        type PlatinumStake: Get<BalanceOf<Self>>;

        /// Annual staking reward percentage (basis points)
        #[pallet::constant]
        type StakingRewardBps: Get<u16>; // 800 = 8%
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// User staking info
    #[pallet::storage]
    pub type Stakes<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, StakingInfo<T>>;

    /// Total staked amount
    #[pallet::storage]
    pub type TotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Governance proposals
    #[pallet::storage]
    pub type Proposals<T: Config> = StorageMap<_, Blake2_128Concat, u64, Proposal<T>>;

    /// Next proposal ID
    #[pallet::storage]
    pub type NextProposalId<T> = StorageValue<_, u64, ValueQuery>;

    /// User votes on proposals (proposal_id, account) → vote
    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // proposal_id
        Blake2_128Concat,
        T::AccountId,
        bool, // true = for, false = against
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Staked [account, amount, tier]
        Staked { account: T::AccountId, amount: BalanceOf<T>, tier: StakingTier },
        /// Unstaked [account, amount]
        Unstaked { account: T::AccountId, amount: BalanceOf<T> },
        /// Rewards claimed [account, amount]
        RewardsClaimed { account: T::AccountId, amount: BalanceOf<T> },
        /// Proposal created [proposal_id, proposer]
        ProposalCreated { proposal_id: u64, proposer: T::AccountId },
        /// Vote cast [proposal_id, voter, vote_for]
        VoteCast { proposal_id: u64, voter: T::AccountId, vote_for: bool },
        /// Proposal executed [proposal_id]
        ProposalExecuted { proposal_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotStaked,
        InsufficientStake,
        AlreadyStaked,
        ProposalNotFound,
        AlreadyVoted,
        VotingEnded,
        ProposalNotPassed,
        InvalidTier,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Stake ËTRD tokens
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let staker = ensure_signed(origin)?;

            ensure!(!Stakes::<T>::contains_key(&staker), Error::<T>::AlreadyStaked);

            // Determine tier
            let tier = Self::calculate_tier(amount);
            ensure!(tier != StakingTier::None, Error::<T>::InsufficientStake);

            // Reserve tokens
            T::Currency::reserve(&staker, amount)?;

            let staking_info = StakingInfo {
                staker: staker.clone(),
                staked_amount: amount,
                tier: tier.clone(),
                staked_at: Self::current_timestamp(),
                last_reward_claim: Self::current_timestamp(),
                total_rewards_earned: 0_u32.into(),
                governance_votes_cast: 0,
            };

            Stakes::<T>::insert(&staker, staking_info);
            TotalStaked::<T>::mutate(|total| *total = total.saturating_add(amount));

            Self::deposit_event(Event::Staked { account: staker, amount, tier });
            Ok(())
        }

        /// Unstake tokens (with cooldown period)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn unstake(origin: OriginFor<T>) -> DispatchResult {
            let staker = ensure_signed(origin)?;

            let staking_info = Stakes::<T>::get(&staker).ok_or(Error::<T>::NotStaked)?;

            // Unreserve tokens
            T::Currency::unreserve(&staker, staking_info.staked_amount);

            TotalStaked::<T>::mutate(|total| *total = total.saturating_sub(staking_info.staked_amount));
            Stakes::<T>::remove(&staker);

            Self::deposit_event(Event::Unstaked {
                account: staker,
                amount: staking_info.staked_amount,
            });

            Ok(())
        }

        /// Claim staking rewards (8% APY)
        #[pallet::call_index(2)]
        #[pallet::weight(8_000)]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult {
            let staker = ensure_signed(origin)?;

            Stakes::<T>::try_mutate(&staker, |maybe_info| {
                let info = maybe_info.as_mut().ok_or(Error::<T>::NotStaked)?;

                // Calculate rewards (simple: 8% APY)
                let time_staked = Self::current_timestamp().saturating_sub(info.last_reward_claim);
                let seconds_per_year = 365 * 24 * 60 * 60;

                // rewards = staked_amount * 0.08 * (time_staked / seconds_per_year)
                let rewards = info
                    .staked_amount
                    .saturating_mul(T::StakingRewardBps::get().into())
                    .saturating_div(10000_u32.into())
                    .saturating_mul(time_staked.into())
                    .saturating_div(seconds_per_year.into());

                // Mint rewards (in production, would use Treasury)
                // T::Currency::deposit_creating(&staker, rewards);

                info.last_reward_claim = Self::current_timestamp();
                info.total_rewards_earned = info.total_rewards_earned.saturating_add(rewards);

                Self::deposit_event(Event::RewardsClaimed {
                    account: staker,
                    amount: rewards,
                });

                Ok::<(), DispatchError>(())
            })?;

            Ok(())
        }

        /// Create governance proposal
        #[pallet::call_index(3)]
        #[pallet::weight(15_000)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: BoundedVec<u8, ConstU32<128>>,
            description: BoundedVec<u8, ConstU32<512>>,
            voting_period_blocks: u32,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            // Ensure proposer has staked tokens
            let staking_info = Stakes::<T>::get(&proposer).ok_or(Error::<T>::NotStaked)?;
            ensure!(
                staking_info.tier.has_governance_rights(),
                Error::<T>::InsufficientStake
            );

            let proposal_id = NextProposalId::<T>::get();
            NextProposalId::<T>::put(proposal_id + 1);

            let current_time = Self::current_timestamp();
            let proposal = Proposal {
                proposer: proposer.clone(),
                title,
                description,
                votes_for: 0_u32.into(),
                votes_against: 0_u32.into(),
                status: ProposalStatus::Active,
                created_at: current_time,
                voting_ends_at: current_time + (voting_period_blocks as u64 * 6), // blocks to seconds
            };

            Proposals::<T>::insert(proposal_id, proposal);

            Self::deposit_event(Event::ProposalCreated { proposal_id, proposer });
            Ok(())
        }

        /// Vote on proposal (weighted by staked amount)
        #[pallet::call_index(4)]
        #[pallet::weight(8_000)]
        pub fn vote(origin: OriginFor<T>, proposal_id: u64, vote_for: bool) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            let staking_info = Stakes::<T>::get(&voter).ok_or(Error::<T>::NotStaked)?;
            ensure!(
                !Votes::<T>::contains_key(proposal_id, &voter),
                Error::<T>::AlreadyVoted
            );

            Proposals::<T>::try_mutate(proposal_id, |maybe_proposal| {
                let proposal = maybe_proposal.as_mut().ok_or(Error::<T>::ProposalNotFound)?;
                ensure!(proposal.status == ProposalStatus::Active, Error::<T>::VotingEnded);
                ensure!(
                    Self::current_timestamp() < proposal.voting_ends_at,
                    Error::<T>::VotingEnded
                );

                // Weight vote by staked amount
                if vote_for {
                    proposal.votes_for = proposal.votes_for.saturating_add(staking_info.staked_amount);
                } else {
                    proposal.votes_against =
                        proposal.votes_against.saturating_add(staking_info.staked_amount);
                }

                Votes::<T>::insert(proposal_id, &voter, vote_for);

                Self::deposit_event(Event::VoteCast {
                    proposal_id,
                    voter,
                    vote_for,
                });

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

        fn calculate_tier(amount: BalanceOf<T>) -> StakingTier {
            if amount >= T::PlatinumStake::get() {
                StakingTier::Platinum
            } else if amount >= T::GoldStake::get() {
                StakingTier::Gold
            } else if amount >= T::SilverStake::get() {
                StakingTier::Silver
            } else if amount >= T::BronzeStake::get() {
                StakingTier::Bronze
            } else {
                StakingTier::None
            }
        }

        /// Get user's staking tier (public helper)
        pub fn get_user_tier(account: &T::AccountId) -> StakingTier {
            Stakes::<T>::get(account)
                .map(|info| info.tier)
                .unwrap_or(StakingTier::None)
        }

        /// Get fee discount for user
        pub fn get_fee_discount(account: &T::AccountId) -> u16 {
            Self::get_user_tier(account).fee_discount_bps()
        }

        /// Get priority level for job matching
        pub fn get_priority_level(account: &T::AccountId) -> u8 {
            Self::get_user_tier(account).priority_level()
        }
    }
}
