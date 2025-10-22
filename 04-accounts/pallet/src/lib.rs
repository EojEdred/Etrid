#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use scale_info::TypeInfo;
    use sp_runtime::{RuntimeDebug, traits::AtLeast32BitUnsigned};

    // Maximum guardians per account
    const MAX_GUARDIANS: u32 = 10;

    #[derive(
        Encode,
        Decode,
        codec::DecodeWithMemTracking,
        Clone,
        Copy,
        Eq,
        PartialEq,
        RuntimeDebug,
        TypeInfo,
        MaxEncodedLen
    )]
    pub enum TokenType {
        ETR,
        ETD,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
    pub struct AccountData<Balance: MaxEncodedLen> {
        pub etr_balance: Balance,
        pub etd_balance: Balance,
        pub nonce: u32,
        pub is_validator: bool,
        pub reputation: u64,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    pub struct RecoveryConfig<AccountId, BlockNumber> {
        pub guardians: BoundedVec<AccountId, ConstU32<MAX_GUARDIANS>>,
        pub threshold: u32,  // M-of-N threshold
        pub delay_period: BlockNumber,  // Blocks to wait before recovery
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Clone, Eq, PartialEq, RuntimeDebug)]
    pub struct ActiveRecovery<AccountId, BlockNumber> {
        pub new_account: AccountId,
        pub approvals: BoundedVec<AccountId, ConstU32<MAX_GUARDIANS>>,
        pub created_at: BlockNumber,
        pub executable_at: BlockNumber,
    }

    impl<AccountId, BlockNumber> RecoveryConfig<AccountId, BlockNumber> {
        pub fn is_guardian(&self, who: &AccountId) -> bool
        where
            AccountId: PartialEq,
        {
            self.guardians.contains(who)
        }

        pub fn has_threshold(&self, approvals: &BoundedVec<AccountId, ConstU32<MAX_GUARDIANS>>) -> bool {
            approvals.len() >= self.threshold as usize
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Balance: Parameter + From<u64> + Into<u64> + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
        type GovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub(super) type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AccountData<T::Balance>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn recovery_configs)]
    pub type RecoveryConfigs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        RecoveryConfig<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn active_recoveries)]
    pub type ActiveRecoveries<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,  // Lost account
        ActiveRecovery<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transferred(T::AccountId, T::AccountId, TokenType, T::Balance),
        Minted(T::AccountId, TokenType, T::Balance),
        Burned(T::AccountId, TokenType, T::Balance),
        RecoveryCreated { account: T::AccountId, threshold: u32 },
        RecoveryInitiated { lost_account: T::AccountId, new_account: T::AccountId, guardian: T::AccountId },
        RecoveryApproved { lost_account: T::AccountId, guardian: T::AccountId, approvals: u32 },
        RecoveryExecuted { lost_account: T::AccountId, new_account: T::AccountId },
        RecoveryCancelled { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        InvalidTokenType,
        InvalidThreshold,
        ThresholdTooHigh,
        NoGuardians,
        TooManyGuardians,
        NoRecoveryConfig,
        RecoveryAlreadyActive,
        NotGuardian,
        NoActiveRecovery,
        AlreadyApproved,
        ThresholdNotMet,
        DelayNotPassed,
        NotAccountOwner,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            token_type: TokenType,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::do_transfer(&sender, &to, token_type, amount)?;
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn mint_etr(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            T::GovernanceOrigin::ensure_origin(origin)?;
            Accounts::<T>::mutate(&to, |acct| {
                acct.etr_balance += amount;
            });
            Self::deposit_event(Event::Minted(to, TokenType::ETR, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn mint_etd(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Accounts::<T>::mutate(&to, |acct| {
                acct.etd_balance += amount;
            });
            Self::deposit_event(Event::Minted(to, TokenType::ETD, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn burn(
            origin: OriginFor<T>,
            token_type: TokenType,
            amount: T::Balance,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            Accounts::<T>::try_mutate(&sender, |acct| -> DispatchResult {
                match token_type {
                    TokenType::ETR => {
                        ensure!(acct.etr_balance >= amount, Error::<T>::InsufficientBalance);
                        acct.etr_balance -= amount;
                    },
                    TokenType::ETD => {
                        ensure!(acct.etd_balance >= amount, Error::<T>::InsufficientBalance);
                        acct.etd_balance -= amount;
                    },
                }
                Ok(())
            })?;
            Self::deposit_event(Event::Burned(sender, token_type, amount));
            Ok(())
        }

        /// Setup recovery configuration for an account
        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn create_recovery(
            origin: OriginFor<T>,
            guardians: Vec<T::AccountId>,
            threshold: u32,
            delay_period: BlockNumberFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(threshold > 0, Error::<T>::InvalidThreshold);
            ensure!(threshold as usize <= guardians.len(), Error::<T>::ThresholdTooHigh);
            ensure!(!guardians.is_empty(), Error::<T>::NoGuardians);
            ensure!(guardians.len() <= MAX_GUARDIANS as usize, Error::<T>::TooManyGuardians);

            let guardians_bounded = BoundedVec::try_from(guardians)
                .map_err(|_| Error::<T>::TooManyGuardians)?;

            let config = RecoveryConfig {
                guardians: guardians_bounded,
                threshold,
                delay_period,
            };

            RecoveryConfigs::<T>::insert(&who, config);
            Self::deposit_event(Event::RecoveryCreated { account: who, threshold });

            Ok(())
        }

        /// Initiate recovery for a lost account
        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn initiate_recovery(
            origin: OriginFor<T>,
            lost_account: T::AccountId,
            new_account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let config = RecoveryConfigs::<T>::get(&lost_account)
                .ok_or(Error::<T>::NoRecoveryConfig)?;

            ensure!(config.is_guardian(&who), Error::<T>::NotGuardian);
            ensure!(!ActiveRecoveries::<T>::contains_key(&lost_account), Error::<T>::RecoveryAlreadyActive);

            let current_block = frame_system::Pallet::<T>::block_number();
            let executable_at = current_block + config.delay_period;

            let approvals = BoundedVec::try_from(vec![who.clone()])
                .map_err(|_| Error::<T>::TooManyGuardians)?;

            let recovery = ActiveRecovery {
                new_account: new_account.clone(),
                approvals,
                created_at: current_block,
                executable_at,
            };

            ActiveRecoveries::<T>::insert(&lost_account, recovery);
            Self::deposit_event(Event::RecoveryInitiated { lost_account, new_account, guardian: who });

            Ok(())
        }

        /// Approve an active recovery
        #[pallet::weight(10_000)]
        #[pallet::call_index(6)]
        pub fn approve_recovery(
            origin: OriginFor<T>,
            lost_account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let config = RecoveryConfigs::<T>::get(&lost_account)
                .ok_or(Error::<T>::NoRecoveryConfig)?;

            ensure!(config.is_guardian(&who), Error::<T>::NotGuardian);

            ActiveRecoveries::<T>::try_mutate(&lost_account, |maybe_recovery| -> DispatchResult {
                let recovery = maybe_recovery.as_mut().ok_or(Error::<T>::NoActiveRecovery)?;

                ensure!(!recovery.approvals.contains(&who), Error::<T>::AlreadyApproved);

                recovery.approvals.try_push(who.clone())
                    .map_err(|_| Error::<T>::TooManyGuardians)?;

                Self::deposit_event(Event::RecoveryApproved {
                    lost_account: lost_account.clone(),
                    guardian: who,
                    approvals: recovery.approvals.len() as u32,
                });

                Ok(())
            })
        }

        /// Execute recovery after threshold and delay period
        #[pallet::weight(10_000)]
        #[pallet::call_index(7)]
        pub fn execute_recovery(
            origin: OriginFor<T>,
            lost_account: T::AccountId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let config = RecoveryConfigs::<T>::get(&lost_account)
                .ok_or(Error::<T>::NoRecoveryConfig)?;

            let recovery = ActiveRecoveries::<T>::get(&lost_account)
                .ok_or(Error::<T>::NoActiveRecovery)?;

            // Check threshold reached
            ensure!(config.has_threshold(&recovery.approvals), Error::<T>::ThresholdNotMet);

            // Check delay period passed
            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(current_block >= recovery.executable_at, Error::<T>::DelayNotPassed);

            // Execute recovery: transfer account ownership
            Self::recover_account(&lost_account, &recovery.new_account)?;

            // Cleanup
            ActiveRecoveries::<T>::remove(&lost_account);
            RecoveryConfigs::<T>::remove(&lost_account);

            Self::deposit_event(Event::RecoveryExecuted {
                lost_account,
                new_account: recovery.new_account,
            });

            Ok(())
        }

        /// Cancel an active recovery (only by lost account owner)
        #[pallet::weight(10_000)]
        #[pallet::call_index(8)]
        pub fn cancel_recovery(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Only the account owner can cancel
            ensure!(who == account, Error::<T>::NotAccountOwner);

            ActiveRecoveries::<T>::remove(&account);
            Self::deposit_event(Event::RecoveryCancelled { account });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn do_transfer(
            from: &T::AccountId,
            to: &T::AccountId,
            token_type: TokenType,
            amount: T::Balance,
        ) -> Result<(), DispatchError> {
            Accounts::<T>::try_mutate(from, |from_acct| -> DispatchResult {
                match token_type {
                    TokenType::ETR => {
                        ensure!(from_acct.etr_balance >= amount, Error::<T>::InsufficientBalance);
                        from_acct.etr_balance -= amount;
                    },
                    TokenType::ETD => {
                        ensure!(from_acct.etd_balance >= amount, Error::<T>::InsufficientBalance);
                        from_acct.etd_balance -= amount;
                    },
                }
                from_acct.nonce += 1;
                Ok(())
            })?;
            Accounts::<T>::mutate(to, |to_acct| {
                match token_type {
                    TokenType::ETR => to_acct.etr_balance += amount,
                    TokenType::ETD => to_acct.etd_balance += amount,
                }
            });
            Self::deposit_event(Event::Transferred(from.clone(), to.clone(), token_type, amount));
            Ok(())
        }

        /// Helper function to transfer account ownership during recovery
        fn recover_account(
            lost_account: &T::AccountId,
            new_account: &T::AccountId,
        ) -> DispatchResult {
            // Transfer all account data from lost to new account
            let lost_data = Accounts::<T>::get(lost_account);

            // Transfer ETR balance
            if lost_data.etr_balance > T::Balance::from(0u64) {
                Self::do_transfer(lost_account, new_account, TokenType::ETR, lost_data.etr_balance)?;
            }

            // Transfer ETD balance
            if lost_data.etd_balance > T::Balance::from(0u64) {
                Self::do_transfer(lost_account, new_account, TokenType::ETD, lost_data.etd_balance)?;
            }

            // Transfer other account properties
            Accounts::<T>::mutate(new_account, |new_acct| {
                new_acct.is_validator = lost_data.is_validator;
                new_acct.reputation = lost_data.reputation;
            });

            // Clear the lost account
            Accounts::<T>::remove(lost_account);

            Ok(())
        }
    }
}
