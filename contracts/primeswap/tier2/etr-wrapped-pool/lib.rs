#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # ÉTR/Wrapped Token Trading Pool (Tier 2)
///
/// This contract implements a dual-sided AMM pool that:
/// - Provides trading between ÉTR and wrapped tokens (wBTC, wETH, etc.)
/// - Uses VirtualReserveAMM mechanism
/// - Implements constant product formula (x × y = k)
/// - Charges 0.3% swap fee
/// - Supports all 11 wrapped currencies
///
/// Part of the PrimeSwap Two-Tier Architecture for ĒTRID.

#[ink::contract]
mod etr_wrapped_pool {
    use ink::storage::Mapping;

    /// Pool configuration for specific wrapped token
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PoolConfig {
        /// Wrapped currency name (e.g., "wBTC")
        pub currency: String,
        /// ÉTR allocation for this pool
        pub etr_allocation: Balance,
        /// Virtual reserve amount for wrapped token
        pub virtual_reserve: Balance,
    }

    /// The contract's storage
    #[ink(storage)]
    pub struct EtrWrappedPool {
        /// Pool configuration
        config: PoolConfig,
        /// Address of ÉTR token contract
        etr_token: AccountId,
        /// Address of wrapped token contract
        wrapped_token: AccountId,
        /// Real ÉTR reserve in pool
        etr_reserve: Balance,
        /// Real wrapped token reserve in pool
        wrapped_reserve_real: Balance,
        /// Virtual wrapped token reserve (for AMM calculation)
        wrapped_reserve_virtual: Balance,
        /// AMM constant k (x × y = k)
        k: u128,
        /// Contract owner
        owner: AccountId,
        /// Emergency pause state
        paused: bool,
        /// Swap fee (in basis points, e.g., 30 = 0.3%)
        swap_fee_bps: u16,
        /// Total fees collected in ÉTR
        total_fees_etr: Balance,
        /// Total fees collected in wrapped token
        total_fees_wrapped: Balance,
        /// Total swaps executed
        total_swaps: u64,
        /// Slippage tolerance (in basis points, e.g., 50 = 0.5%)
        max_slippage_bps: u16,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct PoolInitialized {
        #[ink(topic)]
        currency: String,
        etr_reserve: Balance,
        wrapped_virtual: Balance,
        k: u128,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct SwapExecuted {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        token_in: SwapDirection,
        amount_in: Balance,
        amount_out: Balance,
        fee: Balance,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct ReserveUpdate {
        etr_reserve: Balance,
        wrapped_reserve_real: Balance,
        wrapped_reserve_virtual: Balance,
        k: u128,
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
    pub struct FeesCollected {
        #[ink(topic)]
        collector: AccountId,
        etr_amount: Balance,
        wrapped_amount: Balance,
        timestamp: Timestamp,
    }

    /// Swap direction
    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum SwapDirection {
        EtrToWrapped,
        WrappedToEtr,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Contract is paused
        ContractPaused,
        /// Caller is not the owner
        NotOwner,
        /// Amount is zero
        ZeroAmount,
        /// Insufficient reserve
        InsufficientReserve,
        /// Insufficient output amount (slippage)
        InsufficientOutputAmount,
        /// Overflow would occur
        Overflow,
        /// Invalid K constant
        InvalidK,
        /// Pool already initialized
        AlreadyInitialized,
        /// Pool not initialized
        NotInitialized,
        /// Invalid swap direction
        InvalidSwapDirection,
        /// Exceeds max slippage
        ExceedsMaxSlippage,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl EtrWrappedPool {
        /// Constructor: Creates a new ÉTR/Wrapped Token Trading Pool
        ///
        /// # Arguments
        /// * `currency` - Wrapped currency name (e.g., "wBTC")
        /// * `etr_allocation` - ÉTR allocation for this pool
        /// * `virtual_reserve` - Virtual wrapped token reserve
        /// * `etr_token` - Address of ÉTR token contract
        /// * `wrapped_token` - Address of wrapped token contract
        #[ink(constructor)]
        pub fn new(
            currency: String,
            etr_allocation: Balance,
            virtual_reserve: Balance,
            etr_token: AccountId,
            wrapped_token: AccountId,
        ) -> Self {
            let caller = Self::env().caller();

            let config = PoolConfig {
                currency,
                etr_allocation,
                virtual_reserve,
            };

            Self {
                config,
                etr_token,
                wrapped_token,
                etr_reserve: 0,
                wrapped_reserve_real: 0,
                wrapped_reserve_virtual: virtual_reserve,
                k: 0,
                owner: caller,
                paused: false,
                swap_fee_bps: 30, // 0.3%
                total_fees_etr: 0,
                total_fees_wrapped: 0,
                total_swaps: 0,
                max_slippage_bps: 100, // 1% max slippage
            }
        }

        /// Initialize pool with ÉTR allocation
        ///
        /// This deposits the ÉTR allocation and sets up the AMM constant k
        ///
        /// # Arguments
        /// * `etr_amount` - Amount of ÉTR to deposit (should match allocation)
        #[ink(message)]
        pub fn initialize_pool(&mut self, etr_amount: Balance) -> Result<()> {
            if self.k > 0 {
                return Err(Error::AlreadyInitialized);
            }

            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if etr_amount == 0 {
                return Err(Error::ZeroAmount);
            }

            // Set ÉTR reserve
            self.etr_reserve = etr_amount;

            // Calculate k = etr_reserve × (wrapped_real + wrapped_virtual)
            // At initialization, wrapped_real = 0, so k = etr × virtual
            let total_wrapped = (self.wrapped_reserve_real as u128)
                .checked_add(self.wrapped_reserve_virtual as u128)
                .ok_or(Error::Overflow)?;

            self.k = (self.etr_reserve as u128)
                .checked_mul(total_wrapped)
                .ok_or(Error::Overflow)?;

            let now = self.env().block_timestamp();

            self.env().emit_event(PoolInitialized {
                currency: self.config.currency.clone(),
                etr_reserve: self.etr_reserve,
                wrapped_virtual: self.wrapped_reserve_virtual,
                k: self.k,
                timestamp: now,
            });

            Ok(())
        }

        /// Swap ÉTR for wrapped tokens
        ///
        /// # Arguments
        /// * `amount_etr` - Amount of ÉTR to swap
        /// * `min_out` - Minimum amount of wrapped tokens to receive (slippage protection)
        ///
        /// # Returns
        /// Amount of wrapped tokens received
        #[ink(message)]
        pub fn swap_etr_for_wrapped(
            &mut self,
            amount_etr: Balance,
            min_out: Balance,
        ) -> Result<Balance> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            if self.k == 0 {
                return Err(Error::NotInitialized);
            }

            if amount_etr == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            // Calculate fee (0.3% of input)
            let fee = self.calculate_fee(amount_etr);
            let amount_etr_after_fee = amount_etr - fee;

            // Calculate output using AMM formula
            // (x + Δx)(y - Δy) = k
            // Δy = y - k / (x + Δx)
            let total_wrapped = (self.wrapped_reserve_real as u128)
                .checked_add(self.wrapped_reserve_virtual as u128)
                .ok_or(Error::Overflow)?;

            let new_etr_reserve = (self.etr_reserve as u128)
                .checked_add(amount_etr_after_fee as u128)
                .ok_or(Error::Overflow)?;

            let new_total_wrapped = self.k
                .checked_div(new_etr_reserve)
                .ok_or(Error::InvalidK)?;

            let amount_out = total_wrapped
                .checked_sub(new_total_wrapped)
                .ok_or(Error::InsufficientReserve)? as Balance;

            // Check slippage protection
            if amount_out < min_out {
                return Err(Error::InsufficientOutputAmount);
            }

            // Check we have enough real wrapped tokens
            if amount_out > self.wrapped_reserve_real {
                return Err(Error::InsufficientReserve);
            }

            // Update reserves
            self.etr_reserve = self.etr_reserve
                .checked_add(amount_etr_after_fee)
                .ok_or(Error::Overflow)?;
            self.wrapped_reserve_real = self.wrapped_reserve_real - amount_out;

            // Track fees
            self.total_fees_etr = self.total_fees_etr
                .checked_add(fee)
                .ok_or(Error::Overflow)?;
            self.total_swaps += 1;

            // Emit events
            self.env().emit_event(SwapExecuted {
                user: caller,
                token_in: SwapDirection::EtrToWrapped,
                amount_in: amount_etr,
                amount_out,
                fee,
                timestamp: now,
            });

            self.env().emit_event(ReserveUpdate {
                etr_reserve: self.etr_reserve,
                wrapped_reserve_real: self.wrapped_reserve_real,
                wrapped_reserve_virtual: self.wrapped_reserve_virtual,
                k: self.k,
                timestamp: now,
            });

            Ok(amount_out)
        }

        /// Swap wrapped tokens for ÉTR
        ///
        /// # Arguments
        /// * `amount_wrapped` - Amount of wrapped tokens to swap
        /// * `min_out` - Minimum amount of ÉTR to receive (slippage protection)
        ///
        /// # Returns
        /// Amount of ÉTR received
        #[ink(message)]
        pub fn swap_wrapped_for_etr(
            &mut self,
            amount_wrapped: Balance,
            min_out: Balance,
        ) -> Result<Balance> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            if self.k == 0 {
                return Err(Error::NotInitialized);
            }

            if amount_wrapped == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller = self.env().caller();
            let now = self.env().block_timestamp();

            // Calculate fee (0.3% of input)
            let fee = self.calculate_fee(amount_wrapped);
            let amount_wrapped_after_fee = amount_wrapped - fee;

            // Calculate output using AMM formula
            let total_wrapped = (self.wrapped_reserve_real as u128)
                .checked_add(self.wrapped_reserve_virtual as u128)
                .ok_or(Error::Overflow)?;

            let new_total_wrapped = total_wrapped
                .checked_add(amount_wrapped_after_fee as u128)
                .ok_or(Error::Overflow)?;

            let new_etr_reserve = self.k
                .checked_div(new_total_wrapped)
                .ok_or(Error::InvalidK)?;

            let amount_out = (self.etr_reserve as u128)
                .checked_sub(new_etr_reserve)
                .ok_or(Error::InsufficientReserve)? as Balance;

            // Check slippage protection
            if amount_out < min_out {
                return Err(Error::InsufficientOutputAmount);
            }

            // Check we have enough ÉTR
            if amount_out > self.etr_reserve {
                return Err(Error::InsufficientReserve);
            }

            // Update reserves
            self.wrapped_reserve_real = self.wrapped_reserve_real
                .checked_add(amount_wrapped_after_fee)
                .ok_or(Error::Overflow)?;
            self.etr_reserve = self.etr_reserve - amount_out;

            // Track fees
            self.total_fees_wrapped = self.total_fees_wrapped
                .checked_add(fee)
                .ok_or(Error::Overflow)?;
            self.total_swaps += 1;

            // Emit events
            self.env().emit_event(SwapExecuted {
                user: caller,
                token_in: SwapDirection::WrappedToEtr,
                amount_in: amount_wrapped,
                amount_out,
                fee,
                timestamp: now,
            });

            self.env().emit_event(ReserveUpdate {
                etr_reserve: self.etr_reserve,
                wrapped_reserve_real: self.wrapped_reserve_real,
                wrapped_reserve_virtual: self.wrapped_reserve_virtual,
                k: self.k,
                timestamp: now,
            });

            Ok(amount_out)
        }

        /// Calculate expected output amount for a given input
        ///
        /// # Arguments
        /// * `amount_in` - Input amount
        /// * `reserve_in` - Reserve of input token
        /// * `reserve_out` - Reserve of output token
        ///
        /// # Returns
        /// Expected output amount (after fee)
        #[ink(message)]
        pub fn get_amount_out(
            &self,
            amount_in: Balance,
            reserve_in: Balance,
            reserve_out: Balance,
        ) -> Balance {
            if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
                return 0;
            }

            // Apply fee
            let fee = self.calculate_fee(amount_in);
            let amount_in_after_fee = amount_in - fee;

            // AMM formula: Δy = y - k / (x + Δx)
            let k = (reserve_in as u128)
                .checked_mul(reserve_out as u128)
                .unwrap_or(0);

            let new_reserve_in = (reserve_in as u128)
                .checked_add(amount_in_after_fee as u128)
                .unwrap_or(u128::MAX);

            let new_reserve_out = k.checked_div(new_reserve_in).unwrap_or(0);

            let amount_out = (reserve_out as u128)
                .checked_sub(new_reserve_out)
                .unwrap_or(0) as Balance;

            amount_out
        }

        /// Get current price (ÉTR per wrapped token)
        #[ink(message)]
        pub fn get_price(&self) -> u128 {
            if self.wrapped_reserve_real == 0 && self.wrapped_reserve_virtual == 0 {
                return 0;
            }

            let total_wrapped = (self.wrapped_reserve_real as u128)
                .checked_add(self.wrapped_reserve_virtual as u128)
                .unwrap_or(0);

            if total_wrapped == 0 {
                return 0;
            }

            // Price = ÉTR reserve / total wrapped reserve
            (self.etr_reserve as u128)
                .checked_mul(1_000_000) // Scale for precision
                .and_then(|v| v.checked_div(total_wrapped))
                .unwrap_or(0)
        }

        /// Get pool configuration
        #[ink(message)]
        pub fn config(&self) -> PoolConfig {
            self.config.clone()
        }

        /// Get ÉTR reserve
        #[ink(message)]
        pub fn etr_reserve(&self) -> Balance {
            self.etr_reserve
        }

        /// Get real wrapped token reserve
        #[ink(message)]
        pub fn wrapped_reserve_real(&self) -> Balance {
            self.wrapped_reserve_real
        }

        /// Get virtual wrapped token reserve
        #[ink(message)]
        pub fn wrapped_reserve_virtual(&self) -> Balance {
            self.wrapped_reserve_virtual
        }

        /// Get AMM constant k
        #[ink(message)]
        pub fn k(&self) -> u128 {
            self.k
        }

        /// Get total swaps executed
        #[ink(message)]
        pub fn total_swaps(&self) -> u64 {
            self.total_swaps
        }

        /// Get total fees collected
        #[ink(message)]
        pub fn total_fees(&self) -> (Balance, Balance) {
            (self.total_fees_etr, self.total_fees_wrapped)
        }

        /// Get swap fee in basis points
        #[ink(message)]
        pub fn swap_fee_bps(&self) -> u16 {
            self.swap_fee_bps
        }

        /// Get owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Get paused state
        #[ink(message)]
        pub fn paused(&self) -> bool {
            self.paused
        }

        /// Emergency pause (owner only)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
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
            if caller != self.owner {
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

        /// Update swap fee (owner only)
        #[ink(message)]
        pub fn update_swap_fee(&mut self, new_fee_bps: u16) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.swap_fee_bps = new_fee_bps;
            Ok(())
        }

        /// Collect accumulated fees (owner only)
        #[ink(message)]
        pub fn collect_fees(&mut self) -> Result<(Balance, Balance)> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let etr_fees = self.total_fees_etr;
            let wrapped_fees = self.total_fees_wrapped;

            // Reset fee counters
            self.total_fees_etr = 0;
            self.total_fees_wrapped = 0;

            self.env().emit_event(FeesCollected {
                collector: caller,
                etr_amount: etr_fees,
                wrapped_amount: wrapped_fees,
                timestamp: self.env().block_timestamp(),
            });

            Ok((etr_fees, wrapped_fees))
        }

        /// Transfer ownership (owner only)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.owner = new_owner;
            Ok(())
        }

        /// Internal: Calculate swap fee
        fn calculate_fee(&self, amount: Balance) -> Balance {
            // Fee = amount × fee_bps / 10000
            ((amount as u128)
                .checked_mul(self.swap_fee_bps as u128)
                .and_then(|v| v.checked_div(10000))
                .unwrap_or(0)) as Balance
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

        fn create_pool() -> EtrWrappedPool {
            let accounts = create_accounts();
            EtrWrappedPool::new(
                "wBTC".to_string(),
                845_750_000, // ÉTR allocation
                33_830_000,  // Virtual BTC (scaled)
                accounts.bob, // ÉTR token
                accounts.charlie, // wrapped token
            )
        }

        #[ink::test]
        fn new_works() {
            let pool = create_pool();
            assert_eq!(pool.config().currency, "wBTC");
            assert_eq!(pool.etr_reserve(), 0);
            assert_eq!(pool.k(), 0);
            assert!(!pool.paused());
        }

        #[ink::test]
        fn initialize_pool_works() {
            let mut pool = create_pool();
            let etr_allocation = 845_750_000;

            let result = pool.initialize_pool(etr_allocation);
            assert!(result.is_ok());

            assert_eq!(pool.etr_reserve(), etr_allocation);
            assert!(pool.k() > 0);
        }

        #[ink::test]
        fn initialize_pool_fails_not_owner() {
            let accounts = create_accounts();
            let mut pool = create_pool();

            set_caller(accounts.bob);
            let result = pool.initialize_pool(845_750_000);
            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn initialize_pool_fails_already_initialized() {
            let mut pool = create_pool();

            pool.initialize_pool(845_750_000).unwrap();
            let result = pool.initialize_pool(100_000);
            assert_eq!(result, Err(Error::AlreadyInitialized));
        }

        #[ink::test]
        fn swap_etr_for_wrapped_works() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();

            // Add some wrapped tokens to the pool
            pool.wrapped_reserve_real = 10_000_000;

            let amount_etr = 1_000_000;
            let result = pool.swap_etr_for_wrapped(amount_etr, 0);

            assert!(result.is_ok());
            let amount_out = result.unwrap();
            assert!(amount_out > 0);
            assert_eq!(pool.total_swaps(), 1);
        }

        #[ink::test]
        fn swap_wrapped_for_etr_works() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();

            // Add some wrapped tokens to the pool
            pool.wrapped_reserve_real = 10_000_000;

            let amount_wrapped = 100_000;
            let result = pool.swap_wrapped_for_etr(amount_wrapped, 0);

            assert!(result.is_ok());
            let amount_out = result.unwrap();
            assert!(amount_out > 0);
            assert_eq!(pool.total_swaps(), 1);
        }

        #[ink::test]
        fn swap_fails_when_paused() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();
            pool.pause().unwrap();

            let result = pool.swap_etr_for_wrapped(100_000, 0);
            assert_eq!(result, Err(Error::ContractPaused));
        }

        #[ink::test]
        fn swap_fails_not_initialized() {
            let mut pool = create_pool();

            let result = pool.swap_etr_for_wrapped(100_000, 0);
            assert_eq!(result, Err(Error::NotInitialized));
        }

        #[ink::test]
        fn swap_fails_zero_amount() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();

            let result = pool.swap_etr_for_wrapped(0, 0);
            assert_eq!(result, Err(Error::ZeroAmount));
        }

        #[ink::test]
        fn get_amount_out_works() {
            let pool = create_pool();

            let amount_out = pool.get_amount_out(
                1_000_000, // amount in
                100_000_000, // reserve in
                50_000_000,  // reserve out
            );

            assert!(amount_out > 0);
        }

        #[ink::test]
        fn get_price_works() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();

            let price = pool.get_price();
            assert!(price > 0);
        }

        #[ink::test]
        fn fee_calculation_correct() {
            let pool = create_pool();
            let amount = 1_000_000;
            let fee = pool.calculate_fee(amount);

            // 0.3% of 1,000,000 = 3,000
            assert_eq!(fee, 3_000);
        }

        #[ink::test]
        fn pause_unpause_works() {
            let mut pool = create_pool();

            pool.pause().unwrap();
            assert!(pool.paused());

            pool.unpause().unwrap();
            assert!(!pool.paused());
        }

        #[ink::test]
        fn collect_fees_works() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();
            pool.wrapped_reserve_real = 10_000_000;

            // Execute a swap to generate fees
            pool.swap_etr_for_wrapped(1_000_000, 0).unwrap();

            let (etr_fees, _) = pool.total_fees();
            assert!(etr_fees > 0);

            let result = pool.collect_fees();
            assert!(result.is_ok());

            let (new_etr_fees, _) = pool.total_fees();
            assert_eq!(new_etr_fees, 0); // Fees reset
        }

        #[ink::test]
        fn update_swap_fee_works() {
            let mut pool = create_pool();

            pool.update_swap_fee(50).unwrap(); // Change to 0.5%
            assert_eq!(pool.swap_fee_bps(), 50);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = create_accounts();
            let mut pool = create_pool();

            pool.transfer_ownership(accounts.bob).unwrap();
            assert_eq!(pool.owner(), accounts.bob);
        }

        #[ink::test]
        fn slippage_protection_works() {
            let mut pool = create_pool();
            pool.initialize_pool(845_750_000).unwrap();
            pool.wrapped_reserve_real = 10_000_000;

            // Set min_out too high
            let result = pool.swap_etr_for_wrapped(1_000_000, 999_999_999);
            assert_eq!(result, Err(Error::InsufficientOutputAmount));
        }
    }
}
