//! Ëtrid Consensus-Day — Minting Logic
//!
//! Handles post-governance fiscal minting of ÉTR / EDSC supply and prepares
//! reward schedules for Distribution.  Operates under strict caps and only
//! executes once the Foundation DAO or Consensus-Day vote authorizes it.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement::KeepAlive},
};
use frame_system::pallet_prelude::*;
use sp_runtime::RuntimeDebug;
use consensus_day_proposal_system::{Proposals, ProposalStatus, ProposalCategory};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Currency used for minting (ÉTR)
        type Currency: Currency<Self::AccountId>;

        /// Combined event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Foundation Treasury account (where minted funds originate)
        #[pallet::constant]
        type TreasuryAccount: Get<Self::AccountId>;

        /// Maximum mintable percentage of total supply per year (e.g., 5%)
        #[pallet::constant]
        type AnnualMintCapPercent: Get<u8>;
    }

    /// Record of each minting event.
    #[pallet::storage]
    #[pallet::getter(fn mint_events)]
    pub type MintEvents<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        MintRecord<T::AccountId, BalanceOf<T>>,
        OptionQuery,
    >;

    /// Incrementing ID for minting operations.
    #[pallet::storage]
    #[pallet::getter(fn next_mint_id)]
    pub type NextMintId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Accumulated minted total this fiscal year.
    #[pallet::storage]
    #[pallet::getter(fn annual_minted)]
    pub type AnnualMinted<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintExecuted(u64, BalanceOf<T>),
        MintScheduled(u64, BalanceOf<T>),
        AnnualCapExceeded,
    }

    #[pallet::error]
    pub enum Error<T> {
        ProposalNotApproved,
        AnnualCapReached,
        InvalidAmount,
        TransferFailed,
    }

    #[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
    pub struct MintRecord<AccountId, Balance> {
        pub id: u64,
        pub proposer: AccountId,
        pub amount: Balance,
        pub executed: bool,
        pub timestamp: u64,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // ----------------- Calls -----------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Schedule a mint operation after a fiscal or economic proposal passes.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn schedule_mint(
            origin: OriginFor<T>,
            proposal_id: u64,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let proposal =
                Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotApproved)?;
            ensure!(
                proposal.status == ProposalStatus::Approved
                    && matches!(proposal.category, ProposalCategory::EconomicAdjustment | ProposalCategory::TreasuryAllocation),
                Error::<T>::ProposalNotApproved
            );
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            let current_total = AnnualMinted::<T>::get();
            let cap_percent = T::AnnualMintCapPercent::get() as u128;
            let cap_limit = (current_total.saturated_into::<u128>() * (100 + cap_percent)) / 100;
            ensure!(
                amount.saturated_into::<u128>() <= cap_limit,
                Error::<T>::AnnualCapReached
            );

            let id = NextMintId::<T>::get();
            let record = MintRecord {
                id,
                proposer: who.clone(),
                amount,
                executed: false,
                timestamp: <frame_system::Pallet<T>>::block_number().saturated_into::<u64>(),
            };
            MintEvents::<T>::insert(id, &record);
            NextMintId::<T>::put(id + 1);

            Self::deposit_event(Event::<T>::MintScheduled(id, amount));
            Ok(())
        }

        /// Execute a scheduled mint, transferring funds to the Treasury account.
        #[pallet::call_index(1)]
        #[pallet::weight(20_000)]
        pub fn execute_mint(origin: OriginFor<T>, mint_id: u64) -> DispatchResult {
            ensure_root(origin)?;
            let mut record =
                MintEvents::<T>::get(mint_id).ok_or(Error::<T>::ProposalNotApproved)?;
            ensure!(!record.executed, Error::<T>::AnnualCapReached);

            let treasury = T::TreasuryAccount::get();
            T::Currency::deposit_creating(&treasury, record.amount);

            record.executed = true;
            MintEvents::<T>::insert(mint_id, &record);
            AnnualMinted::<T>::mutate(|t| *t += record.amount);

            Self::deposit_event(Event::<T>::MintExecuted(mint_id, record.amount));
            Ok(())
        }
    }
}