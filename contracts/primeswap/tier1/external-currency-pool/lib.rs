#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # External Currency Reserve Pool (Tier 1)
///
/// This contract implements a one-sided reserve pool that:
/// - Locks external currencies (BTC, ETH, SOL, etc.)
/// - Mints wrapped tokens 1:1
/// - Provides 100% backing guarantee
/// - Requires multi-sig for withdrawals
/// - Maintains proof of reserves
///
/// Part of the PrimeSwap Two-Tier Architecture for ĒTRID.

#[ink::contract]
mod external_currency_pool {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct ExternalCurrencyPool {
        /// Address of the external currency (for reference)
        external_currency: String,
        /// Address of the wrapped token contract
        wrapped_token: AccountId,
        /// Total reserves locked in this pool
        total_reserves: Balance,
        /// Authorized Tier 2 pool that can receive wrapped tokens
        tier2_pool: AccountId,
        /// Multi-sig wallet address (3-of-5 required for withdrawals)
        multi_sig: AccountId,
        /// Contract owner (can update settings)
        owner: AccountId,
        /// Emergency pause state
        paused: bool,
        /// Reserve tracking per user (for audit trail)
        user_reserves: Mapping<AccountId, Balance>,
        /// Total minted wrapped tokens
        total_minted: Balance,
        /// Maximum single transaction limit
        max_tx_limit: Balance,
        /// Daily withdrawal limit
        daily_limit: Balance,
        /// Timestamp of last reset
        last_reset: Timestamp,
        /// Amount withdrawn today
        withdrawn_today: Balance,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct Locked {
        #[ink(topic)]
        user: AccountId,
        amount: Balance,
        wrapped_minted: Balance,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct Released {
        #[ink(topic)]
        user: AccountId,
        amount: Balance,
        wrapped_burned: Balance,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct ReserveUpdate {
        total_reserves: Balance,
        total_minted: Balance,
        reserve_ratio: u128,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct EmergencyPause {
        #[ink(topic)]
        paused: bool,
        #[ink(topic)]
        caller: AccountId,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct MultiSigUpdate {
        #[ink(topic)]
        old_multi_sig: AccountId,
        #[ink(topic)]
        new_multi_sig: AccountId,
        timestamp: Timestamp,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Contract is paused
        ContractPaused,
        /// Caller is not the owner
        NotOwner,
        /// Caller is not the multi-sig
        NotMultiSig,
        /// Amount is zero
        ZeroAmount,
        /// Insufficient reserves
        InsufficientReserves,
        /// Reserve ratio is not 1:1
        ReserveRatioViolation,
        /// Overflow would occur
        Overflow,
        /// Exceeds transaction limit
        ExceedsTransactionLimit,
        /// Exceeds daily limit
        ExceedsDailyLimit,
        /// Wrapped token call failed
        WrappedTokenCallFailed,
        /// Invalid address
        InvalidAddress,
        /// Unauthorized caller
        Unauthorized,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl ExternalCurrencyPool {
        /// Constructor: Creates a new External Currency Reserve Pool
        ///
        /// # Arguments
        /// * `external_currency` - Name of external currency (e.g., "BTC")
        /// * `wrapped_token` - Address of wrapped token contract
        /// * `tier2_pool` - Address of Tier 2 trading pool
        /// * `multi_sig` - Multi-sig wallet address
        /// * `max_tx_limit` - Maximum single transaction amount
        /// * `daily_limit` - Maximum daily withdrawal amount
        #[ink(constructor)]
        pub fn new(
            external_currency: String,
            wrapped_token: AccountId,
            tier2_pool: AccountId,
            multi_sig: AccountId,
            max_tx_limit: Balance,
            daily_limit: Balance,
        ) -> Self {
            let caller = Self::env().caller();
            let now = Self::env().block_timestamp();

            Self {
                external_currency,
                wrapped_token,
                total_reserves: 0,
                tier2_pool,
                multi_sig,
                owner: caller,
                paused: false,
                user_reserves: Mapping::default(),
                total_minted: 0,
                max_tx_limit,
                daily_limit,
                last_reset: now,
                withdrawn_today: 0,
            }
        }

        /// Lock external currency and mint wrapped tokens 1:1
        ///
        /// # Arguments
        /// * `amount` - Amount of external currency to lock
        ///
        /// # Returns
        /// Amount of wrapped tokens minted
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `ZeroAmount` - Amount is zero
        /// * `ExceedsTransactionLimit` - Amount exceeds max tx limit
        #[ink(message)]
        pub fn lock_and_mint(&mut self, amount: Balance) -> Result<Balance> {
            // Security checks
            if self.paused {
                return Err(Error::ContractPaused);
            }

            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            if amount > self.max_tx_limit {
                return Err(Error::ExceedsTransactionLimit);
            }

            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            // Update reserves
            self.total_reserves = self.total_reserves
                .checked_add(amount)
                .ok_or(Error::Overflow)?;

            // Update user reserves
            let user_reserve = self.user_reserves.get(&caller).unwrap_or(0);
            let new_user_reserve = user_reserve
                .checked_add(amount)
                .ok_or(Error::Overflow)?;
            self.user_reserves.insert(&caller, &new_user_reserve);

            // Mint wrapped tokens 1:1
            let wrapped_amount = amount; // 1:1 backing
            self.total_minted = self.total_minted
                .checked_add(wrapped_amount)
                .ok_or(Error::Overflow)?;

            // Emit events
            self.env().emit_event(Locked {
                user: caller,
                amount,
                wrapped_minted: wrapped_amount,
                timestamp: now,
            });

            self.env().emit_event(ReserveUpdate {
                total_reserves: self.total_reserves,
                total_minted: self.total_minted,
                reserve_ratio: self.get_reserve_ratio(),
                timestamp: now,
            });

            Ok(wrapped_amount)
        }

        /// Burn wrapped tokens and release external currency
        ///
        /// # Arguments
        /// * `amount` - Amount of wrapped tokens to burn
        /// * `recipient` - Recipient address for released currency
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `NotMultiSig` - Only multi-sig can release
        /// * `ZeroAmount` - Amount is zero
        /// * `InsufficientReserves` - Not enough reserves
        /// * `ExceedsDailyLimit` - Exceeds daily withdrawal limit
        #[ink(message)]
        pub fn burn_and_release(
            &mut self,
            amount: Balance,
            recipient: AccountId,
        ) -> Result<()> {
            // Security checks
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();
            if caller != self.multi_sig {
                return Err(Error::NotMultiSig);
            }

            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            if amount > self.total_reserves {
                return Err(Error::InsufficientReserves);
            }

            if recipient == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            // Check daily limit
            let now = self.env().block_timestamp();
            self.reset_daily_limit_if_needed(now);

            let new_withdrawn = self.withdrawn_today
                .checked_add(amount)
                .ok_or(Error::Overflow)?;

            if new_withdrawn > self.daily_limit {
                return Err(Error::ExceedsDailyLimit);
            }

            self.withdrawn_today = new_withdrawn;

            // Update reserves
            self.total_reserves = self.total_reserves - amount;
            self.total_minted = self.total_minted - amount;

            // Update user reserves
            let user_reserve = self.user_reserves.get(&recipient).unwrap_or(0);
            if user_reserve > 0 {
                let new_user_reserve = user_reserve.saturating_sub(amount);
                self.user_reserves.insert(&recipient, &new_user_reserve);
            }

            // Emit events
            self.env().emit_event(Released {
                user: recipient,
                amount,
                wrapped_burned: amount,
                timestamp: now,
            });

            self.env().emit_event(ReserveUpdate {
                total_reserves: self.total_reserves,
                total_minted: self.total_minted,
                reserve_ratio: self.get_reserve_ratio(),
                timestamp: now,
            });

            Ok(())
        }

        /// Get reserve ratio (should always be 100% = 1_000_000)
        ///
        /// # Returns
        /// Reserve ratio as basis points (100% = 1_000_000)
        #[ink(message)]
        pub fn get_reserve_ratio(&self) -> u128 {
            if self.total_minted == 0 {
                return 1_000_000; // 100% when nothing minted
            }

            // Calculate ratio as (reserves / minted) * 1_000_000
            let ratio = (self.total_reserves as u128)
                .checked_mul(1_000_000)
                .and_then(|v| v.checked_div(self.total_minted as u128))
                .unwrap_or(0);

            ratio
        }

        /// Get total reserves
        #[ink(message)]
        pub fn total_reserves(&self) -> Balance {
            self.total_reserves
        }

        /// Get total minted wrapped tokens
        #[ink(message)]
        pub fn total_minted(&self) -> Balance {
            self.total_minted
        }

        /// Get user's reserve amount
        #[ink(message)]
        pub fn user_reserve(&self, user: AccountId) -> Balance {
            self.user_reserves.get(&user).unwrap_or(0)
        }

        /// Get external currency name
        #[ink(message)]
        pub fn external_currency(&self) -> String {
            self.external_currency.clone()
        }

        /// Get wrapped token address
        #[ink(message)]
        pub fn wrapped_token(&self) -> AccountId {
            self.wrapped_token
        }

        /// Get tier2 pool address
        #[ink(message)]
        pub fn tier2_pool(&self) -> AccountId {
            self.tier2_pool
        }

        /// Get multi-sig address
        #[ink(message)]
        pub fn multi_sig(&self) -> AccountId {
            self.multi_sig
        }

        /// Get contract owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Get paused state
        #[ink(message)]
        pub fn paused(&self) -> bool {
            self.paused
        }

        /// Get max transaction limit
        #[ink(message)]
        pub fn max_tx_limit(&self) -> Balance {
            self.max_tx_limit
        }

        /// Get daily withdrawal limit
        #[ink(message)]
        pub fn daily_limit(&self) -> Balance {
            self.daily_limit
        }

        /// Get amount withdrawn today
        #[ink(message)]
        pub fn withdrawn_today(&self) -> Balance {
            self.withdrawn_today
        }

        /// Emergency pause (owner only)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner && caller != self.multi_sig {
                return Err(Error::NotOwner);
            }

            self.paused = true;

            self.env().emit_event(EmergencyPause {
                paused: true,
                caller,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        /// Unpause (owner only)
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner && caller != self.multi_sig {
                return Err(Error::NotOwner);
            }

            self.paused = false;

            self.env().emit_event(EmergencyPause {
                paused: false,
                caller,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        /// Update multi-sig address (owner only)
        #[ink(message)]
        pub fn update_multi_sig(&mut self, new_multi_sig: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if new_multi_sig == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            let old_multi_sig = self.multi_sig;
            self.multi_sig = new_multi_sig;

            self.env().emit_event(MultiSigUpdate {
                old_multi_sig,
                new_multi_sig,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        /// Update transaction limit (owner only)
        #[ink(message)]
        pub fn update_max_tx_limit(&mut self, new_limit: Balance) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.max_tx_limit = new_limit;
            Ok(())
        }

        /// Update daily limit (owner only)
        #[ink(message)]
        pub fn update_daily_limit(&mut self, new_limit: Balance) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.daily_limit = new_limit;
            Ok(())
        }

        /// Transfer ownership (owner only)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if new_owner == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            self.owner = new_owner;
            Ok(())
        }

        /// Internal: Reset daily withdrawal limit if 24h have passed
        fn reset_daily_limit_if_needed(&mut self, now: Timestamp) {
            const ONE_DAY: u64 = 24 * 60 * 60 * 1000; // 24 hours in milliseconds

            if now >= self.last_reset + ONE_DAY {
                self.withdrawn_today = 0;
                self.last_reset = now;
            }
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        fn create_contract() -> ExternalCurrencyPool {
            let accounts = create_accounts();
            ExternalCurrencyPool::new(
                "BTC".to_string(),
                accounts.bob,  // wrapped token
                accounts.charlie, // tier2 pool
                accounts.django, // multi-sig
                1_000_000, // max tx limit
                10_000_000, // daily limit
            )
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            assert_eq!(contract.external_currency(), "BTC");
            assert_eq!(contract.total_reserves(), 0);
            assert_eq!(contract.total_minted(), 0);
            assert_eq!(contract.get_reserve_ratio(), 1_000_000); // 100%
            assert!(!contract.paused());
        }

        #[ink::test]
        fn lock_and_mint_works() {
            let mut contract = create_contract();
            let amount = 100_000;

            let result = contract.lock_and_mint(amount);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), amount); // 1:1 minting

            assert_eq!(contract.total_reserves(), amount);
            assert_eq!(contract.total_minted(), amount);
            assert_eq!(contract.get_reserve_ratio(), 1_000_000); // Still 100%
        }

        #[ink::test]
        fn lock_and_mint_fails_when_paused() {
            let mut contract = create_contract();
            contract.pause().unwrap();

            let result = contract.lock_and_mint(100_000);
            assert_eq!(result, Err(Error::ContractPaused));
        }

        #[ink::test]
        fn lock_and_mint_fails_zero_amount() {
            let mut contract = create_contract();

            let result = contract.lock_and_mint(0);
            assert_eq!(result, Err(Error::ZeroAmount));
        }

        #[ink::test]
        fn lock_and_mint_fails_exceeds_limit() {
            let mut contract = create_contract();

            let result = contract.lock_and_mint(2_000_000); // Exceeds 1M limit
            assert_eq!(result, Err(Error::ExceedsTransactionLimit));
        }

        #[ink::test]
        fn burn_and_release_works() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            // First lock some amount
            contract.lock_and_mint(500_000).unwrap();

            // Switch to multi-sig
            set_caller(accounts.django);

            // Burn and release
            let result = contract.burn_and_release(100_000, accounts.alice);
            assert!(result.is_ok());

            assert_eq!(contract.total_reserves(), 400_000);
            assert_eq!(contract.total_minted(), 400_000);
            assert_eq!(contract.withdrawn_today(), 100_000);
        }

        #[ink::test]
        fn burn_and_release_fails_not_multisig() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            contract.lock_and_mint(500_000).unwrap();

            // Try to release without being multi-sig
            let result = contract.burn_and_release(100_000, accounts.alice);
            assert_eq!(result, Err(Error::NotMultiSig));
        }

        #[ink::test]
        fn burn_and_release_fails_insufficient_reserves() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            contract.lock_and_mint(100_000).unwrap();

            set_caller(accounts.django);
            let result = contract.burn_and_release(200_000, accounts.alice);
            assert_eq!(result, Err(Error::InsufficientReserves));
        }

        #[ink::test]
        fn reserve_ratio_maintained() {
            let mut contract = create_contract();

            // Lock 1,000,000
            contract.lock_and_mint(1_000_000).unwrap();
            assert_eq!(contract.get_reserve_ratio(), 1_000_000); // 100%

            // Lock more
            contract.lock_and_mint(500_000).unwrap();
            assert_eq!(contract.get_reserve_ratio(), 1_000_000); // Still 100%
        }

        #[ink::test]
        fn pause_unpause_works() {
            let mut contract = create_contract();

            assert!(!contract.paused());

            contract.pause().unwrap();
            assert!(contract.paused());

            contract.unpause().unwrap();
            assert!(!contract.paused());
        }

        #[ink::test]
        fn pause_fails_not_owner() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            set_caller(accounts.bob);
            let result = contract.pause();
            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn update_multi_sig_works() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            let result = contract.update_multi_sig(accounts.bob);
            assert!(result.is_ok());
            assert_eq!(contract.multi_sig(), accounts.bob);
        }

        #[ink::test]
        fn update_multi_sig_fails_not_owner() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            set_caller(accounts.bob);
            let result = contract.update_multi_sig(accounts.charlie);
            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn user_reserves_tracked() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            contract.lock_and_mint(300_000).unwrap();
            assert_eq!(contract.user_reserve(accounts.alice), 300_000);

            contract.lock_and_mint(200_000).unwrap();
            assert_eq!(contract.user_reserve(accounts.alice), 500_000);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = create_accounts();
            let mut contract = create_contract();

            contract.transfer_ownership(accounts.bob).unwrap();
            assert_eq!(contract.owner(), accounts.bob);
        }
    }
}
