//! Ëtrid Consensus-Day — Distribution
//!
//! Handles fiscal payout scheduling and execution following approved minting events.
//! Connects with the Minting Logic pallet and disburses rewards to governance
//! participants (Foundation, Directors, Validators, and Voters).

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement::KeepAlive, Get},
};
use frame_system::pallet_prelude::*;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

// NOTE: Dependencies on minting-logic and peer-roles are stubbed out for compilation.
// Full integration will be implemented when runtime is configured.

/// Distribution share constants (percentages of each mint event).
/// Later these can be made configurable by DAO vote.
pub const FOUNDATION_SHARE: u8 = 40;
pub const DIRECTORS_SHARE: u8 = 20;
pub const VALIDATORS_SHARE: u8 = 30;
pub const VOTERS_SHARE: u8 = 10;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The token used for distribution (ÉTR)
        type Currency: Currency<Self::AccountId>;
        /// Global event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Foundation Treasury account
        #[pallet::constant]
        type FoundationAccount: Get<Self::AccountId>;
        /// List of Director accounts (to be fetched from Peer Roles pallet in future)
        #[pallet::constant]
        type Directors: Get<Vec<Self::AccountId>>;
        /// List of active validator accounts
        #[pallet::constant]
        type Validators: Get<Vec<Self::AccountId>>;
        /// List of active voter accounts (from Consensus Day registration)
        #[pallet::constant]
        type Voters: Get<Vec<Self::AccountId>>;
    }

    /// Record of each completed distribution
    #[pallet::storage]
    #[pallet::getter(fn distributions)]
    pub type Distributions<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, DistributionRecord<T::AccountId, BalanceOf<T>>, OptionQuery>;

    #[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct DistributionRecord<AccountId: MaxEncodedLen, Balance: MaxEncodedLen> {
        pub mint_id: u64,
        pub total_amount: Balance,
        pub executed: bool,
        #[codec(skip)]
        pub recipients: Vec<(AccountId, Balance)>,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DistributionExecuted(u64, BalanceOf<T>),
        RecipientPaid(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        MintNotFound,
        AlreadyDistributed,
        TransferFailed,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // ----------------- CALLS -----------------

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Execute a distribution event using funds from a completed mint.
        ///
        /// NOTE: Mint record validation is stubbed out for compilation. Full implementation
        /// will require runtime integration with consensus-day-minting-logic.
        #[pallet::call_index(0)]
        #[pallet::weight(30_000)]
        pub fn execute_distribution(origin: OriginFor<T>, mint_id: u64) -> DispatchResult {
            ensure_root(origin)?;

            // TODO: Integrate with consensus-day-minting-logic once runtime is configured
            // Stub validation - will be implemented when runtime has proper dependencies

            ensure!(
                !Distributions::<T>::contains_key(mint_id),
                Error::<T>::AlreadyDistributed
            );

            // Stub: Use a default amount for compilation
            let total = BalanceOf::<T>::from(1000u32);
            let mut recipients: Vec<(T::AccountId, BalanceOf<T>)> = Vec::new();

            // Calculate shares
            let foundation_amt = total * BalanceOf::<T>::from(FOUNDATION_SHARE as u32) / 100u32.into();
            let directors_amt = total * BalanceOf::<T>::from(DIRECTORS_SHARE as u32) / 100u32.into();
            let validators_amt = total * BalanceOf::<T>::from(VALIDATORS_SHARE as u32) / 100u32.into();
            let voters_amt = total * BalanceOf::<T>::from(VOTERS_SHARE as u32) / 100u32.into();

            let foundation = T::FoundationAccount::get();
            T::Currency::transfer(
                &foundation,
                &foundation,
                foundation_amt,
                KeepAlive,
            )?;
            recipients.push((foundation.clone(), foundation_amt));
            Self::deposit_event(Event::<T>::RecipientPaid(foundation.clone(), foundation_amt));

            // Split and pay Directors
            let directors = T::Directors::get();
            if !directors.is_empty() {
                let per_director = directors_amt / BalanceOf::<T>::from(directors.len() as u32);
                for acct in directors.iter() {
                    T::Currency::transfer(&foundation, acct, per_director, KeepAlive)?;
                    recipients.push((acct.clone(), per_director));
                    Self::deposit_event(Event::<T>::RecipientPaid(acct.clone(), per_director));
                }
            }

            // Split and pay Validators
            let validators = T::Validators::get();
            if !validators.is_empty() {
                let per_validator = validators_amt / BalanceOf::<T>::from(validators.len() as u32);
                for acct in validators.iter() {
                    T::Currency::transfer(&foundation, acct, per_validator, KeepAlive)?;
                    recipients.push((acct.clone(), per_validator));
                    Self::deposit_event(Event::<T>::RecipientPaid(acct.clone(), per_validator));
                }
            }

            // Split and pay Registered Voters
            let voters = T::Voters::get();
            if !voters.is_empty() {
                let per_voter = voters_amt / BalanceOf::<T>::from(voters.len() as u32);
                for acct in voters.iter() {
                    T::Currency::transfer(&foundation, acct, per_voter, KeepAlive)?;
                    recipients.push((acct.clone(), per_voter));
                    Self::deposit_event(Event::<T>::RecipientPaid(acct.clone(), per_voter));
                }
            }

            let record = DistributionRecord {
                mint_id,
                total_amount: total,
                executed: true,
                recipients,
            };
            Distributions::<T>::insert(mint_id, &record);

            Self::deposit_event(Event::<T>::DistributionExecuted(mint_id, total));
            Ok(())
        }
    }
}