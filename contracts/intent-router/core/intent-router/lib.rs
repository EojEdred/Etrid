#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Intent Router Contract
///
/// User-facing abstraction for BTC ↔ ÉTR conversions
///
/// This contract hides the complexity of wrapped tokens and multi-tier pools,
/// providing a simple interface for users to convert between external currencies
/// and ÉTR in a single transaction.

#[ink::contract]
mod intent_router {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    /// Route type enum
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RouteType {
        /// External → wToken → ÉTR
        ExternalToEtr,
        /// ÉTR → wToken → External
        EtrToExternal,
        /// ÉTR → ÉTR (no swap)
        EtrToEtr,
    }

    /// Route configuration
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RouteConfig {
        pub route_type: RouteType,
        pub tier1_pool: AccountId,
        pub tier2_pool: AccountId,
        pub wrapped_token: AccountId,
        pub bridge_tracker: AccountId,
    }

    /// Swap intent structure
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SwapIntent {
        pub user: AccountId,
        pub source_token: AccountId,
        pub dest_token: AccountId,
        pub amount_in: Balance,
        pub min_amount_out: Balance,
        pub route: RouteConfig,
        pub deadline: u64,
    }

    /// Quote result
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct QuoteResult {
        pub estimated_out: Balance,
        pub price_impact_bps: u16,
        pub fee: Balance,
    }

    /// The contract's storage
    #[ink(storage)]
    pub struct IntentRouter {
        /// Contract owner
        owner: AccountId,
        /// ÉTR token address
        etr_token: AccountId,
        /// Auto swap executor contract
        auto_swap_executor: AccountId,
        /// Two tier bridge router contract
        two_tier_bridge_router: AccountId,
        /// External token → Tier 1 pool
        tier1_pools: Mapping<AccountId, AccountId>,
        /// Wrapped token → Tier 2 pool
        tier2_pools: Mapping<AccountId, AccountId>,
        /// External token → Wrapped token
        wrapped_tokens: Mapping<AccountId, AccountId>,
        /// Chain ID → Bridge tracker
        bridge_trackers: Mapping<u32, AccountId>,
        /// Platform fee (basis points, e.g., 30 = 0.3%)
        platform_fee_bps: u16,
        /// Fee collector address
        fee_collector: AccountId,
        /// Default max slippage (basis points)
        default_max_slippage_bps: u16,
        /// Paused status
        paused: bool,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct ConversionExecuted {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        source_currency: AccountId,
        #[ink(topic)]
        dest_currency: AccountId,
        amount_in: Balance,
        amount_out: Balance,
        fee: Balance,
    }

    #[ink(event)]
    pub struct WithdrawalInitiated {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        target_currency: AccountId,
        amount: Balance,
        external_address: [u8; 32],
    }

    #[ink(event)]
    pub struct RouteConfigured {
        #[ink(topic)]
        external_token: AccountId,
        tier1_pool: AccountId,
        tier2_pool: AccountId,
        wrapped_token: AccountId,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Contract is paused
        Paused,
        /// Deadline expired
        DeadlineExpired,
        /// Slippage too high
        SlippageTooHigh,
        /// Invalid amount
        InvalidAmount,
        /// Route not configured
        RouteNotConfigured,
        /// Invalid external address
        InvalidExternalAddress,
        /// Transfer failed
        TransferFailed,
        /// Arithmetic overflow
        Overflow,
        /// Zero address
        ZeroAddress,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl IntentRouter {
        /// Constructor
        #[ink(constructor)]
        pub fn new(
            etr_token: AccountId,
            auto_swap_executor: AccountId,
            two_tier_bridge_router: AccountId,
            fee_collector: AccountId,
        ) -> Self {
            Self {
                owner: Self::env().caller(),
                etr_token,
                auto_swap_executor,
                two_tier_bridge_router,
                tier1_pools: Mapping::default(),
                tier2_pools: Mapping::default(),
                wrapped_tokens: Mapping::default(),
                bridge_trackers: Mapping::default(),
                platform_fee_bps: 30, // 0.3%
                fee_collector,
                default_max_slippage_bps: 50, // 0.5%
                paused: false,
            }
        }

        /// Convert external currency to ÉTR
        ///
        /// # Arguments
        /// * `source_currency` - External currency address
        /// * `amount` - Amount to convert
        /// * `min_etr_out` - Minimum ÉTR to receive (slippage protection)
        /// * `deadline` - Transaction deadline (Unix timestamp)
        ///
        /// # Returns
        /// Amount of ÉTR received (after fees)
        #[ink(message)]
        pub fn convert_to_etr(
            &mut self,
            source_currency: AccountId,
            amount: Balance,
            min_etr_out: Balance,
            deadline: u64,
        ) -> Result<Balance> {
            let caller = self.env().caller();
            ensure!(!self.paused, Error::Paused);
            ensure!(amount > 0, Error::InvalidAmount);
            ensure!(self.env().block_timestamp() <= deadline, Error::DeadlineExpired);

            // Get route configuration
            let route = self.get_route(source_currency, self.etr_token)?;

            // Estimate output
            let quote = self.get_quote(source_currency, self.etr_token, amount)?;
            ensure!(quote.estimated_out >= min_etr_out, Error::SlippageTooHigh);

            // NOTE: In production, call AutoSwapExecutor here
            // For now, simulate the swap
            let etr_received = quote.estimated_out;

            // Calculate fee
            let fee = self.calculate_fee(etr_received)?;
            let amount_after_fee = etr_received.checked_sub(fee).ok_or(Error::Overflow)?;

            // Emit event
            self.env().emit_event(ConversionExecuted {
                user: caller,
                source_currency,
                dest_currency: self.etr_token,
                amount_in: amount,
                amount_out: amount_after_fee,
                fee,
            });

            Ok(amount_after_fee)
        }

        /// Convert ÉTR to external currency
        ///
        /// # Arguments
        /// * `target_currency` - Target external currency
        /// * `etr_amount` - Amount of ÉTR to convert
        /// * `min_currency_out` - Minimum external currency to receive
        /// * `external_address` - Address on external chain to receive funds
        /// * `deadline` - Transaction deadline
        ///
        /// # Returns
        /// Amount of external currency withdrawn
        #[ink(message)]
        pub fn convert_from_etr(
            &mut self,
            target_currency: AccountId,
            etr_amount: Balance,
            min_currency_out: Balance,
            external_address: [u8; 32],
            deadline: u64,
        ) -> Result<Balance> {
            let caller = self.env().caller();
            ensure!(!self.paused, Error::Paused);
            ensure!(etr_amount > 0, Error::InvalidAmount);
            ensure!(self.env().block_timestamp() <= deadline, Error::DeadlineExpired);
            ensure!(external_address != [0u8; 32], Error::InvalidExternalAddress);

            // Get route configuration
            let route = self.get_route(self.etr_token, target_currency)?;

            // Estimate output
            let quote = self.get_quote(self.etr_token, target_currency, etr_amount)?;
            ensure!(quote.estimated_out >= min_currency_out, Error::SlippageTooHigh);

            // NOTE: In production, call AutoSwapExecutor.executeReverseSwap here
            // For now, simulate
            let currency_withdrawn = quote.estimated_out;

            // Emit event
            self.env().emit_event(WithdrawalInitiated {
                user: caller,
                target_currency,
                amount: currency_withdrawn,
                external_address,
            });

            Ok(currency_withdrawn)
        }

        /// Get quote for conversion
        ///
        /// # Arguments
        /// * `source_token` - Source token address
        /// * `dest_token` - Destination token address
        /// * `amount_in` - Input amount
        ///
        /// # Returns
        /// Quote with estimated output, price impact, and fee
        #[ink(message)]
        pub fn get_quote(
            &self,
            source_token: AccountId,
            dest_token: AccountId,
            amount_in: Balance,
        ) -> Result<QuoteResult> {
            // Get route
            let _route = self.get_route(source_token, dest_token)?;

            // NOTE: In production, query Tier 2 pool for actual price
            // For now, use simplified calculation (1:1 ratio for demo)
            let estimated_out = amount_in;

            // Calculate fee
            let fee = self.calculate_fee(estimated_out)?;

            // Calculate price impact (simplified)
            let price_impact_bps: u16 = 10; // 0.1% price impact

            Ok(QuoteResult {
                estimated_out,
                price_impact_bps,
                fee,
            })
        }

        /// Configure route for external currency
        ///
        /// # Arguments
        /// * `external_token` - External token address
        /// * `tier1_pool` - Tier 1 pool address
        /// * `tier2_pool` - Tier 2 pool address
        /// * `wrapped_token` - Wrapped token address
        /// * `bridge_tracker` - Bridge tracker address
        #[ink(message)]
        pub fn configure_route(
            &mut self,
            external_token: AccountId,
            tier1_pool: AccountId,
            tier2_pool: AccountId,
            wrapped_token: AccountId,
            bridge_tracker: AccountId,
        ) -> Result<()> {
            self.ensure_owner()?;

            self.tier1_pools.insert(external_token, &tier1_pool);
            self.tier2_pools.insert(wrapped_token, &tier2_pool);
            self.wrapped_tokens.insert(external_token, &wrapped_token);

            self.env().emit_event(RouteConfigured {
                external_token,
                tier1_pool,
                tier2_pool,
                wrapped_token,
            });

            Ok(())
        }

        /// Set platform fee
        #[ink(message)]
        pub fn set_platform_fee(&mut self, fee_bps: u16) -> Result<()> {
            self.ensure_owner()?;
            self.platform_fee_bps = fee_bps;
            Ok(())
        }

        /// Pause contract
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            self.ensure_owner()?;
            self.paused = true;
            Ok(())
        }

        /// Unpause contract
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            self.ensure_owner()?;
            self.paused = false;
            Ok(())
        }

        /// Transfer ownership
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.ensure_owner()?;
            ensure!(new_owner != AccountId::from([0x0; 32]), Error::ZeroAddress);
            self.owner = new_owner;
            Ok(())
        }

        /// Get owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Get ÉTR token address
        #[ink(message)]
        pub fn etr_token(&self) -> AccountId {
            self.etr_token
        }

        /// Is paused
        #[ink(message)]
        pub fn is_paused(&self) -> bool {
            self.paused
        }

        /// Get platform fee
        #[ink(message)]
        pub fn platform_fee_bps(&self) -> u16 {
            self.platform_fee_bps
        }

        // === Internal functions ===

        /// Ensure caller is owner
        fn ensure_owner(&self) -> Result<()> {
            ensure!(self.env().caller() == self.owner, Error::NotOwner);
            Ok(())
        }

        /// Get route configuration
        fn get_route(
            &self,
            source_token: AccountId,
            dest_token: AccountId,
        ) -> Result<RouteConfig> {
            let route_type = if source_token != self.etr_token && dest_token == self.etr_token {
                RouteType::ExternalToEtr
            } else if source_token == self.etr_token && dest_token != self.etr_token {
                RouteType::EtrToExternal
            } else if source_token == self.etr_token && dest_token == self.etr_token {
                RouteType::EtrToEtr
            } else {
                return Err(Error::RouteNotConfigured);
            };

            let (wrapped_token, tier1_pool, tier2_pool) = if route_type == RouteType::ExternalToEtr
            {
                let wrapped = self
                    .wrapped_tokens
                    .get(source_token)
                    .ok_or(Error::RouteNotConfigured)?;
                let tier1 = self
                    .tier1_pools
                    .get(source_token)
                    .ok_or(Error::RouteNotConfigured)?;
                let tier2 = self
                    .tier2_pools
                    .get(wrapped)
                    .ok_or(Error::RouteNotConfigured)?;
                (wrapped, tier1, tier2)
            } else if route_type == RouteType::EtrToExternal {
                let wrapped = self
                    .wrapped_tokens
                    .get(dest_token)
                    .ok_or(Error::RouteNotConfigured)?;
                let tier1 = self
                    .tier1_pools
                    .get(dest_token)
                    .ok_or(Error::RouteNotConfigured)?;
                let tier2 = self
                    .tier2_pools
                    .get(wrapped)
                    .ok_or(Error::RouteNotConfigured)?;
                (wrapped, tier1, tier2)
            } else {
                // EtrToEtr - no route needed
                (AccountId::from([0x0; 32]), AccountId::from([0x0; 32]), AccountId::from([0x0; 32]))
            };

            Ok(RouteConfig {
                route_type,
                tier1_pool,
                tier2_pool,
                wrapped_token,
                bridge_tracker: AccountId::from([0x0; 32]), // Placeholder
            })
        }

        /// Calculate platform fee
        fn calculate_fee(&self, amount: Balance) -> Result<Balance> {
            let fee = (amount as u128)
                .checked_mul(self.platform_fee_bps as u128)
                .ok_or(Error::Overflow)?
                .checked_div(10000)
                .ok_or(Error::Overflow)? as Balance;
            Ok(fee)
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        fn create_contract() -> IntentRouter {
            let accounts = default_accounts();
            IntentRouter::new(
                accounts.bob,  // ETR token
                accounts.charlie, // Auto swap executor
                accounts.django, // Two tier bridge router
                accounts.eve, // Fee collector
            )
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            let accounts = default_accounts();
            assert_eq!(contract.owner(), accounts.alice);
            assert_eq!(contract.platform_fee_bps(), 30);
            assert!(!contract.is_paused());
        }

        #[ink::test]
        fn set_platform_fee_works() {
            let mut contract = create_contract();
            assert!(contract.set_platform_fee(50).is_ok());
            assert_eq!(contract.platform_fee_bps(), 50);
        }

        #[ink::test]
        fn pause_unpause_works() {
            let mut contract = create_contract();
            assert!(contract.pause().is_ok());
            assert!(contract.is_paused());
            assert!(contract.unpause().is_ok());
            assert!(!contract.is_paused());
        }

        #[ink::test]
        fn configure_route_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.configure_route(
                accounts.bob,     // external token
                accounts.charlie, // tier1 pool
                accounts.django,  // tier2 pool
                accounts.eve,     // wrapped token
                accounts.frank,   // bridge tracker
            );

            assert!(result.is_ok());
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            assert!(contract.transfer_ownership(accounts.bob).is_ok());
            assert_eq!(contract.owner(), accounts.bob);
        }

        #[ink::test]
        fn only_owner_can_configure() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            set_caller(accounts.bob);
            let result = contract.configure_route(
                accounts.bob,
                accounts.charlie,
                accounts.django,
                accounts.eve,
                accounts.frank,
            );

            assert_eq!(result, Err(Error::NotOwner));
        }
    }
}
