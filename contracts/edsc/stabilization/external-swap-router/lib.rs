#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # External Swap Router
///
/// Interface contract for external swap aggregators (1inch, ParaSwap).
/// Actual swaps happen off-chain via aggregator APIs.
///
/// Features:
/// - Swap routing to USDC from any asset
/// - Price quotes from aggregators
/// - Slippage protection (max 0.5%)
/// - Multi-aggregator support (1inch, ParaSwap, THORChain)
///
/// NOTE: This is an interface contract. Actual swap execution
/// happens via off-chain API calls to aggregators.

#[ink::contract]
mod external_swap_router {
    use ink::storage::Mapping;

    /// Maximum slippage: 0.5%
    const MAX_SLIPPAGE_BPS: u16 = 50; // 50 basis points = 0.5%

    /// The contract's storage
    #[ink(storage)]
    pub struct ExternalSwapRouter {
        /// USDC token address
        usdc_token: AccountId,
        /// Contract owner
        owner: AccountId,
        /// Supported aggregators
        aggregators: Mapping<u8, Aggregator>,
        /// Total swaps executed
        total_swaps: u64,
        /// Total volume in USDC
        total_volume_usdc: Balance,
        /// Paused state
        paused: bool,
    }

    /// Aggregator configuration
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Aggregator {
        /// Aggregator ID
        id: u8,
        /// Name (e.g., "1inch", "ParaSwap")
        name: String,
        /// Enabled
        enabled: bool,
        /// Priority (lower = higher priority)
        priority: u8,
    }

    /// Events
    #[ink(event)]
    pub struct SwapExecuted {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        asset_in: AccountId,
        #[ink(topic)]
        asset_out: AccountId,
        amount_in: Balance,
        amount_out: Balance,
        aggregator_id: u8,
    }

    #[ink(event)]
    pub struct QuoteRequested {
        #[ink(topic)]
        asset_in: AccountId,
        #[ink(topic)]
        asset_out: AccountId,
        amount_in: Balance,
        expected_out: Balance,
    }

    #[ink(event)]
    pub struct AggregatorConfigured {
        aggregator_id: u8,
        name: String,
        enabled: bool,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Contract is paused
        ContractPaused,
        /// Zero amount
        ZeroAmount,
        /// Invalid token
        InvalidToken,
        /// Slippage too high
        SlippageTooHigh,
        /// No aggregator available
        NoAggregatorAvailable,
        /// Swap failed
        SwapFailed,
        /// Insufficient output
        InsufficientOutput,
        /// Overflow
        Overflow,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ExternalSwapRouter {
        /// Constructor
        #[ink(constructor)]
        pub fn new(usdc_token: AccountId) -> Self {
            let caller = Self::env().caller();
            let mut router = Self {
                usdc_token,
                owner: caller,
                aggregators: Mapping::default(),
                total_swaps: 0,
                total_volume_usdc: 0,
                paused: false,
            };

            // Initialize default aggregators
            router.initialize_aggregators();

            router
        }

        /// Swaps any asset to USDC via best aggregator
        ///
        /// NOTE: In production, this would call external aggregator APIs
        #[ink(message)]
        pub fn swap_to_usdc(
            &mut self,
            asset_in: AccountId,
            amount_in: Balance,
            min_usdc_out: Balance,
        ) -> Result<Balance> {
            self.ensure_not_paused()?;

            if amount_in == 0 {
                return Err(Error::ZeroAmount);
            }

            if asset_in == self.usdc_token {
                return Ok(amount_in); // Already USDC
            }

            let user = self.env().caller();

            // Get quote from aggregators
            let quote = self.get_quote(asset_in, self.usdc_token, amount_in);

            // Validate slippage
            if quote < min_usdc_out {
                return Err(Error::InsufficientOutput);
            }

            // NOTE: In production, execute swap via aggregator API:
            // 1. Call 1inch API: POST /swap/v6.0/{chain}/swap
            // 2. Parse response and execute transaction
            // 3. Verify output amount
            //
            // For now, simulate 99.5% conversion (0.5% slippage)
            let usdc_received = (amount_in * 995) / 1000;

            if usdc_received < min_usdc_out {
                return Err(Error::InsufficientOutput);
            }

            // Update stats
            self.total_swaps = self.total_swaps.checked_add(1).ok_or(Error::Overflow)?;
            self.total_volume_usdc = self.total_volume_usdc
                .checked_add(usdc_received)
                .ok_or(Error::Overflow)?;

            // Emit event (using aggregator ID 1 for 1inch)
            self.env().emit_event(SwapExecuted {
                user,
                asset_in,
                asset_out: self.usdc_token,
                amount_in,
                amount_out: usdc_received,
                aggregator_id: 1,
            });

            Ok(usdc_received)
        }

        /// Gets quote for swap
        ///
        /// NOTE: In production, this would call aggregator quote APIs
        #[ink(message)]
        pub fn get_quote(
            &self,
            asset_in: AccountId,
            asset_out: AccountId,
            amount_in: Balance,
        ) -> Balance {
            if amount_in == 0 {
                return 0;
            }

            if asset_in == asset_out {
                return amount_in;
            }

            // NOTE: In production, call aggregator APIs:
            // 1inch: GET /swap/v6.0/{chain}/quote
            // ParaSwap: GET /prices
            //
            // For now, simulate quote with 0.5% slippage
            let quote = (amount_in * 995) / 1000;

            self.env().emit_event(QuoteRequested {
                asset_in,
                asset_out,
                amount_in,
                expected_out: quote,
            });

            quote
        }

        /// Returns swap statistics
        #[ink(message)]
        pub fn get_stats(&self) -> (u64, Balance) {
            (self.total_swaps, self.total_volume_usdc)
        }

        /// Returns USDC token address
        #[ink(message)]
        pub fn get_usdc_token(&self) -> AccountId {
            self.usdc_token
        }

        /// Returns max slippage in basis points
        #[ink(message)]
        pub fn get_max_slippage_bps(&self) -> u16 {
            MAX_SLIPPAGE_BPS
        }

        /// Configures an aggregator (owner only)
        #[ink(message)]
        pub fn configure_aggregator(
            &mut self,
            id: u8,
            name: String,
            enabled: bool,
            priority: u8,
        ) -> Result<()> {
            self.ensure_owner()?;

            let aggregator = Aggregator {
                id,
                name: name.clone(),
                enabled,
                priority,
            };

            self.aggregators.insert(&id, &aggregator);

            self.env().emit_event(AggregatorConfigured {
                aggregator_id: id,
                name,
                enabled,
            });

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

        /// Initializes default aggregators
        fn initialize_aggregators(&mut self) {
            // 1inch (Primary)
            self.aggregators.insert(
                &1,
                &Aggregator {
                    id: 1,
                    name: "1inch".to_string(),
                    enabled: true,
                    priority: 1,
                },
            );

            // ParaSwap (Secondary)
            self.aggregators.insert(
                &2,
                &Aggregator {
                    id: 2,
                    name: "ParaSwap".to_string(),
                    enabled: true,
                    priority: 2,
                },
            );

            // THORChain (Tertiary)
            self.aggregators.insert(
                &3,
                &Aggregator {
                    id: 3,
                    name: "THORChain".to_string(),
                    enabled: true,
                    priority: 3,
                },
            );
        }

        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                Err(Error::NotOwner)
            } else {
                Ok(())
            }
        }

        fn ensure_not_paused(&self) -> Result<()> {
            if self.paused {
                Err(Error::ContractPaused)
            } else {
                Ok(())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn create_contract() -> ExternalSwapRouter {
            let accounts = get_accounts();
            ExternalSwapRouter::new(accounts.alice)
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            let (total_swaps, total_volume) = contract.get_stats();
            assert_eq!(total_swaps, 0);
            assert_eq!(total_volume, 0);
        }

        #[ink::test]
        fn get_quote_works() {
            let accounts = get_accounts();
            let contract = create_contract();

            let quote = contract.get_quote(accounts.bob, accounts.alice, 1000);
            // Should be 99.5% of input (0.5% slippage)
            assert_eq!(quote, 995);
        }

        #[ink::test]
        fn swap_to_usdc_works() {
            let accounts = get_accounts();
            let mut contract = create_contract();

            let result = contract.swap_to_usdc(accounts.bob, 1000, 990);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 995);

            let (total_swaps, total_volume) = contract.get_stats();
            assert_eq!(total_swaps, 1);
            assert_eq!(total_volume, 995);
        }

        #[ink::test]
        fn swap_fails_insufficient_output() {
            let accounts = get_accounts();
            let mut contract = create_contract();

            // Require more output than possible (min > quote)
            let result = contract.swap_to_usdc(accounts.bob, 1000, 1000);
            assert_eq!(result, Err(Error::InsufficientOutput));
        }

        #[ink::test]
        fn swap_usdc_to_usdc_returns_same() {
            let accounts = get_accounts();
            let mut contract = create_contract();

            let result = contract.swap_to_usdc(accounts.alice, 1000, 1000);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1000);
        }
    }
}
