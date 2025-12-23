#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # EDSC Peg Stabilizer
///
/// Maintains 1 EDSC = $1 via autonomous mint/burn operations.
///
/// Mechanism:
/// - Oracle price feed integration (placeholder for Chainlink)
/// - ±2% deviation threshold triggers action
/// - Algorithmic buy/burn or mint/sell to restore peg
/// - Circuit breaker at ±10% deviation
///
/// Features:
/// - Automatic peg stabilization
/// - Oracle price feed (placeholder)
/// - Deviation monitoring
/// - Circuit breaker protection
/// - Rate limiting

#[ink::contract]
mod peg_stabilizer {
    use ink::storage::Mapping;

    /// Target price: $1.00 (scaled by 10^18)
    const TARGET_PRICE: Balance = 1_000_000_000_000_000_000;

    /// Deviation threshold: ±2%
    const DEVIATION_THRESHOLD: u8 = 2;

    /// Circuit breaker threshold: ±10%
    const CIRCUIT_BREAKER_THRESHOLD: u8 = 10;

    /// Max action per block: 1M EDSC
    const MAX_ACTION_PER_BLOCK: Balance = 1_000_000_000_000_000_000_000_000;

    /// The contract's storage
    #[ink(storage)]
    pub struct PegStabilizer {
        /// EDSC token contract
        edsc_token: AccountId,
        /// Reserve vault contract
        reserve_vault: AccountId,
        /// Oracle price feed (placeholder)
        oracle: AccountId,
        /// Contract owner
        owner: AccountId,
        /// Current EDSC price (from oracle)
        current_price: Balance,
        /// Last stabilization block
        last_stabilization_block: u32,
        /// Total buy actions
        total_buy_actions: u64,
        /// Total sell actions
        total_sell_actions: u64,
        /// Circuit breaker triggered
        circuit_breaker_active: bool,
        /// Stabilization enabled
        stabilization_enabled: bool,
        /// Paused state
        paused: bool,
    }

    /// Events
    #[ink(event)]
    pub struct PegChecked {
        price: Balance,
        target_price: Balance,
        deviation: i32,
        needs_action: bool,
    }

    #[ink(event)]
    pub struct PegStabilized {
        action: StabilizationAction,
        amount: Balance,
        price_before: Balance,
        price_after: Balance,
    }

    #[ink(event)]
    pub struct CircuitBreakerTriggered {
        price: Balance,
        deviation: i32,
    }

    #[ink(event)]
    pub struct CircuitBreakerReset {
        by: AccountId,
    }

    #[ink(event)]
    pub struct OracleUpdated {
        #[ink(topic)]
        old_oracle: AccountId,
        #[ink(topic)]
        new_oracle: AccountId,
    }

    /// Stabilization action types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StabilizationAction {
        /// Buy EDSC and burn (price too low)
        BuyAndBurn,
        /// Mint EDSC and sell (price too high)
        MintAndSell,
        /// No action needed
        None,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Contract is paused
        ContractPaused,
        /// Circuit breaker is active
        CircuitBreakerActive,
        /// Stabilization is disabled
        StabilizationDisabled,
        /// No action needed
        NoActionNeeded,
        /// Oracle price invalid
        InvalidOraclePrice,
        /// Stabilization failed
        StabilizationFailed,
        /// Too soon since last action
        TooSoon,
        /// Overflow
        Overflow,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl PegStabilizer {
        /// Constructor
        #[ink(constructor)]
        pub fn new(
            edsc_token: AccountId,
            reserve_vault: AccountId,
            oracle: AccountId,
        ) -> Self {
            let caller = Self::env().caller();

            Self {
                edsc_token,
                reserve_vault,
                oracle,
                owner: caller,
                current_price: TARGET_PRICE,
                last_stabilization_block: 0,
                total_buy_actions: 0,
                total_sell_actions: 0,
                circuit_breaker_active: false,
                stabilization_enabled: true,
                paused: false,
            }
        }

        /// Checks the peg status
        ///
        /// Returns: (current_price, needs_action)
        #[ink(message)]
        pub fn check_peg(&mut self) -> (Balance, bool) {
            // Fetch price from oracle
            let price = self.fetch_oracle_price();
            self.current_price = price;

            let deviation = self.calculate_deviation(price);
            let needs_action = self.needs_stabilization(deviation);

            self.env().emit_event(PegChecked {
                price,
                target_price: TARGET_PRICE,
                deviation,
                needs_action,
            });

            (price, needs_action)
        }

        /// Performs stabilization action
        #[ink(message)]
        pub fn stabilize(&mut self) -> Result<()> {
            self.ensure_can_stabilize()?;

            // Fetch current price
            let price = self.fetch_oracle_price();
            self.current_price = price;

            let deviation = self.calculate_deviation(price);

            // Check if circuit breaker should trigger
            if self.should_trigger_circuit_breaker(deviation) {
                self.trigger_circuit_breaker(price, deviation);
                return Err(Error::CircuitBreakerActive);
            }

            // Determine action
            let action = self.determine_action(deviation)?;

            // Execute stabilization
            let amount = self.calculate_stabilization_amount(price, deviation);
            let amount = if amount > MAX_ACTION_PER_BLOCK {
                MAX_ACTION_PER_BLOCK
            } else {
                amount
            };

            self.execute_stabilization(action, amount)?;

            // Update price after action (would be fetched from oracle in production)
            let price_after = self.fetch_oracle_price();
            self.current_price = price_after;

            self.last_stabilization_block = self.env().block_number();

            self.env().emit_event(PegStabilized {
                action,
                amount,
                price_before: price,
                price_after,
            });

            Ok(())
        }

        /// Returns the current deviation from target price
        #[ink(message)]
        pub fn get_deviation(&self) -> i32 {
            self.calculate_deviation(self.current_price)
        }

        /// Returns current price
        #[ink(message)]
        pub fn get_current_price(&self) -> Balance {
            self.current_price
        }

        /// Returns target price
        #[ink(message)]
        pub fn get_target_price(&self) -> Balance {
            TARGET_PRICE
        }

        /// Returns stabilization stats
        #[ink(message)]
        pub fn get_stats(&self) -> (u64, u64, Balance, bool) {
            (
                self.total_buy_actions,
                self.total_sell_actions,
                self.current_price,
                self.circuit_breaker_active,
            )
        }

        /// Updates oracle address (owner only)
        #[ink(message)]
        pub fn update_oracle(&mut self, new_oracle: AccountId) -> Result<()> {
            self.ensure_owner()?;

            let old_oracle = self.oracle;
            self.oracle = new_oracle;

            self.env().emit_event(OracleUpdated {
                old_oracle,
                new_oracle,
            });

            Ok(())
        }

        /// Resets circuit breaker (owner only)
        #[ink(message)]
        pub fn reset_circuit_breaker(&mut self) -> Result<()> {
            self.ensure_owner()?;

            self.circuit_breaker_active = false;

            let caller = self.env().caller();
            self.env().emit_event(CircuitBreakerReset { by: caller });

            Ok(())
        }

        /// Toggles stabilization (owner only)
        #[ink(message)]
        pub fn toggle_stabilization(&mut self, enabled: bool) -> Result<()> {
            self.ensure_owner()?;
            self.stabilization_enabled = enabled;
            Ok(())
        }

        /// Pauses the contract (owner only)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            self.ensure_owner()?;
            self.paused = true;
            Ok(())
        }

        /// Unpauses the contract (owner only)
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            self.ensure_owner()?;
            self.paused = false;
            Ok(())
        }

        /// Fetches price from oracle
        /// NOTE: Placeholder - would call Chainlink oracle in production
        fn fetch_oracle_price(&self) -> Balance {
            // Placeholder: Return $1.00
            // In production: Call oracle.latestRoundData()
            TARGET_PRICE
        }

        /// Calculates deviation percentage
        fn calculate_deviation(&self, price: Balance) -> i32 {
            if price == TARGET_PRICE {
                return 0;
            }

            // Calculate percentage deviation
            let diff = if price > TARGET_PRICE {
                price - TARGET_PRICE
            } else {
                TARGET_PRICE - price
            };

            let deviation_pct = ((diff * 100) / TARGET_PRICE) as i32;

            if price < TARGET_PRICE {
                -deviation_pct
            } else {
                deviation_pct
            }
        }

        /// Checks if stabilization is needed
        fn needs_stabilization(&self, deviation: i32) -> bool {
            let abs_deviation = if deviation < 0 { -deviation } else { deviation };
            abs_deviation >= DEVIATION_THRESHOLD as i32
        }

        /// Determines stabilization action
        fn determine_action(&self, deviation: i32) -> Result<StabilizationAction> {
            if deviation <= -(DEVIATION_THRESHOLD as i32) {
                // Price too low: Buy EDSC and burn
                Ok(StabilizationAction::BuyAndBurn)
            } else if deviation >= DEVIATION_THRESHOLD as i32 {
                // Price too high: Mint EDSC and sell
                Ok(StabilizationAction::MintAndSell)
            } else {
                Err(Error::NoActionNeeded)
            }
        }

        /// Calculates stabilization amount
        fn calculate_stabilization_amount(&self, price: Balance, deviation: i32) -> Balance {
            // Simple formula: 10% of deviation as action amount
            let abs_deviation = if deviation < 0 { -deviation } else { deviation };
            let amount = (TARGET_PRICE * abs_deviation as u128) / 1000;
            amount
        }

        /// Executes stabilization action
        fn execute_stabilization(
            &mut self,
            action: StabilizationAction,
            amount: Balance,
        ) -> Result<()> {
            match action {
                StabilizationAction::BuyAndBurn => {
                    // NOTE: Would call reserve vault to buy EDSC, then burn it
                    self.total_buy_actions = self.total_buy_actions
                        .checked_add(1)
                        .ok_or(Error::Overflow)?;
                    Ok(())
                }
                StabilizationAction::MintAndSell => {
                    // NOTE: Would mint EDSC and sell for USDC to reserve
                    self.total_sell_actions = self.total_sell_actions
                        .checked_add(1)
                        .ok_or(Error::Overflow)?;
                    Ok(())
                }
                StabilizationAction::None => Err(Error::NoActionNeeded),
            }
        }

        /// Checks if circuit breaker should trigger
        fn should_trigger_circuit_breaker(&self, deviation: i32) -> bool {
            let abs_deviation = if deviation < 0 { -deviation } else { deviation };
            abs_deviation >= CIRCUIT_BREAKER_THRESHOLD as i32
        }

        /// Triggers circuit breaker
        fn trigger_circuit_breaker(&mut self, price: Balance, deviation: i32) {
            self.circuit_breaker_active = true;

            self.env().emit_event(CircuitBreakerTriggered { price, deviation });
        }

        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                Err(Error::NotOwner)
            } else {
                Ok(())
            }
        }

        fn ensure_can_stabilize(&self) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            if self.circuit_breaker_active {
                return Err(Error::CircuitBreakerActive);
            }

            if !self.stabilization_enabled {
                return Err(Error::StabilizationDisabled);
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn create_contract() -> PegStabilizer {
            let accounts = get_accounts();
            PegStabilizer::new(accounts.alice, accounts.bob, accounts.charlie)
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            assert_eq!(contract.get_current_price(), TARGET_PRICE);
            assert_eq!(contract.get_deviation(), 0);
        }

        #[ink::test]
        fn check_peg_works() {
            let mut contract = create_contract();
            let (price, needs_action) = contract.check_peg();
            assert_eq!(price, TARGET_PRICE);
            assert_eq!(needs_action, false);
        }

        #[ink::test]
        fn calculate_deviation_works() {
            let contract = create_contract();

            // 2% above target
            let high_price = TARGET_PRICE + (TARGET_PRICE * 2) / 100;
            assert_eq!(contract.calculate_deviation(high_price), 2);

            // 2% below target
            let low_price = TARGET_PRICE - (TARGET_PRICE * 2) / 100;
            assert_eq!(contract.calculate_deviation(low_price), -2);
        }

        #[ink::test]
        fn get_stats_works() {
            let contract = create_contract();
            let (buy_actions, sell_actions, price, breaker_active) = contract.get_stats();
            assert_eq!(buy_actions, 0);
            assert_eq!(sell_actions, 0);
            assert_eq!(price, TARGET_PRICE);
            assert_eq!(breaker_active, false);
        }
    }
}
