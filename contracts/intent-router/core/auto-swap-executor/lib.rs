#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Auto Swap Executor Contract
///
/// Execute atomic multi-step swaps across tiers
///
/// This contract orchestrates the complex flow of:
/// External Currency → Bridge → Tier 1 → Tier 2 → ÉTR
/// and the reverse flow for withdrawals.

#[ink::contract]
mod auto_swap_executor {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    /// Route type
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RouteType {
        ExternalToEtr,
        EtrToExternal,
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

    /// Swap intent
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

    /// The contract's storage
    #[ink(storage)]
    pub struct AutoSwapExecutor {
        /// Contract owner
        owner: AccountId,
        /// Intent router (only authorized caller)
        intent_router: AccountId,
        /// Reentrancy lock
        locked: bool,
        /// Swap counter for IDs
        swap_counter: u64,
        /// Swap execution records
        swap_records: Mapping<u64, SwapRecord>,
    }

    /// Swap execution record
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SwapRecord {
        pub user: AccountId,
        pub source_token: AccountId,
        pub dest_token: AccountId,
        pub amount_in: Balance,
        pub amount_out: Balance,
        pub timestamp: u64,
        pub deposit_id: Option<[u8; 32]>,
        pub withdrawal_id: Option<[u8; 32]>,
    }

    /// Events
    #[ink(event)]
    pub struct SwapExecuted {
        #[ink(topic)]
        swap_id: u64,
        #[ink(topic)]
        user: AccountId,
        source_token: AccountId,
        dest_token: AccountId,
        amount_in: Balance,
        amount_out: Balance,
        deposit_id: Option<[u8; 32]>,
    }

    #[ink(event)]
    pub struct ReverseSwapExecuted {
        #[ink(topic)]
        swap_id: u64,
        #[ink(topic)]
        user: AccountId,
        source_token: AccountId,
        dest_token: AccountId,
        amount_in: Balance,
        amount_out: Balance,
        withdrawal_id: [u8; 32],
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotAuthorized,
        ReentrantCall,
        InvalidRoute,
        DepositNotVerified,
        SlippageExceeded,
        DeadlinePassed,
        InvalidAmount,
        TransferFailed,
        MintFailed,
        BurnFailed,
        SwapFailed,
        Overflow,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl AutoSwapExecutor {
        /// Constructor
        #[ink(constructor)]
        pub fn new(intent_router: AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                intent_router,
                locked: false,
                swap_counter: 0,
                swap_records: Mapping::default(),
            }
        }

        /// Execute swap: External Currency → ÉTR
        ///
        /// Flow:
        /// 1. Verify bridge deposit
        /// 2. Tier 1: Lock external, mint wrapped
        /// 3. Tier 2: Swap wrapped → ÉTR
        /// 4. Return ÉTR to user
        ///
        /// # Arguments
        /// * `intent` - Swap intent with all parameters
        ///
        /// # Returns
        /// Amount of ÉTR received
        #[ink(message)]
        pub fn execute_swap(&mut self, intent: SwapIntent) -> Result<Balance> {
            self.ensure_authorized()?;
            self.ensure_not_locked()?;
            self.locked = true;

            // Validate intent
            ensure!(
                intent.route.route_type == RouteType::ExternalToEtr,
                Error::InvalidRoute
            );
            ensure!(
                self.env().block_timestamp() <= intent.deadline,
                Error::DeadlinePassed
            );
            ensure!(intent.amount_in > 0, Error::InvalidAmount);

            // Step 1: Verify bridge deposit
            // NOTE: In production, call BridgeTracker to verify deposit
            let deposit_id = self.generate_deposit_id();
            let _verified = true; // Placeholder

            // Step 2: Tier 1 - Lock and mint wrapped tokens
            // NOTE: In production, call ITier1Pool.lockAndMint()
            let wrapped_minted = intent.amount_in; // 1:1 for simplicity

            // Step 3: Tier 2 - Swap wrapped → ÉTR
            // NOTE: In production, call ITier2Pool.swapWrappedForETR()
            let etr_received = wrapped_minted; // Simplified

            // Verify slippage
            ensure!(etr_received >= intent.min_amount_out, Error::SlippageExceeded);

            // Record swap
            let swap_id = self.swap_counter;
            self.swap_counter += 1;

            let record = SwapRecord {
                user: intent.user,
                source_token: intent.source_token,
                dest_token: intent.dest_token,
                amount_in: intent.amount_in,
                amount_out: etr_received,
                timestamp: self.env().block_timestamp(),
                deposit_id: Some(deposit_id),
                withdrawal_id: None,
            };

            self.swap_records.insert(swap_id, &record);

            // Emit event
            self.env().emit_event(SwapExecuted {
                swap_id,
                user: intent.user,
                source_token: intent.source_token,
                dest_token: intent.dest_token,
                amount_in: intent.amount_in,
                amount_out: etr_received,
                deposit_id: Some(deposit_id),
            });

            self.locked = false;
            Ok(etr_received)
        }

        /// Execute reverse swap: ÉTR → External Currency
        ///
        /// Flow:
        /// 1. Tier 2: Swap ÉTR → wrapped
        /// 2. Tier 1: Burn wrapped, prepare release
        /// 3. Bridge: Request withdrawal to external chain
        ///
        /// # Arguments
        /// * `intent` - Swap intent
        /// * `external_address` - Address on external chain
        ///
        /// # Returns
        /// Amount of external currency to be withdrawn
        #[ink(message)]
        pub fn execute_reverse_swap(
            &mut self,
            intent: SwapIntent,
            external_address: [u8; 32],
        ) -> Result<Balance> {
            self.ensure_authorized()?;
            self.ensure_not_locked()?;
            self.locked = true;

            // Validate
            ensure!(
                intent.route.route_type == RouteType::EtrToExternal,
                Error::InvalidRoute
            );
            ensure!(
                self.env().block_timestamp() <= intent.deadline,
                Error::DeadlinePassed
            );
            ensure!(intent.amount_in > 0, Error::InvalidAmount);
            ensure!(external_address != [0u8; 32], Error::InvalidAmount);

            // Step 1: Tier 2 - Swap ÉTR → wrapped
            // NOTE: In production, call ITier2Pool.swapETRForWrapped()
            let wrapped_received = intent.amount_in; // Simplified

            // Step 2: Tier 1 - Burn wrapped
            // NOTE: In production, call ITier1Pool.burnAndRelease()
            let amount_to_release = wrapped_received;

            // Step 3: Bridge - Record withdrawal
            // NOTE: In production, call IBridgeTracker.recordWithdrawal()
            let withdrawal_id = self.generate_withdrawal_id();

            // Verify slippage
            ensure!(
                amount_to_release >= intent.min_amount_out,
                Error::SlippageExceeded
            );

            // Record swap
            let swap_id = self.swap_counter;
            self.swap_counter += 1;

            let record = SwapRecord {
                user: intent.user,
                source_token: intent.source_token,
                dest_token: intent.dest_token,
                amount_in: intent.amount_in,
                amount_out: amount_to_release,
                timestamp: self.env().block_timestamp(),
                deposit_id: None,
                withdrawal_id: Some(withdrawal_id),
            };

            self.swap_records.insert(swap_id, &record);

            // Emit event
            self.env().emit_event(ReverseSwapExecuted {
                swap_id,
                user: intent.user,
                source_token: intent.source_token,
                dest_token: intent.dest_token,
                amount_in: intent.amount_in,
                amount_out: amount_to_release,
                withdrawal_id,
            });

            self.locked = false;
            Ok(amount_to_release)
        }

        /// Get swap record
        #[ink(message)]
        pub fn get_swap_record(&self, swap_id: u64) -> Option<SwapRecord> {
            self.swap_records.get(swap_id)
        }

        /// Get swap counter
        #[ink(message)]
        pub fn swap_counter(&self) -> u64 {
            self.swap_counter
        }

        /// Update intent router
        #[ink(message)]
        pub fn set_intent_router(&mut self, intent_router: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.intent_router = intent_router;
            Ok(())
        }

        /// Get owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Get intent router
        #[ink(message)]
        pub fn intent_router(&self) -> AccountId {
            self.intent_router
        }

        // === Internal functions ===

        fn ensure_owner(&self) -> Result<()> {
            ensure!(self.env().caller() == self.owner, Error::NotOwner);
            Ok(())
        }

        fn ensure_authorized(&self) -> Result<()> {
            ensure!(
                self.env().caller() == self.intent_router,
                Error::NotAuthorized
            );
            Ok(())
        }

        fn ensure_not_locked(&self) -> Result<()> {
            ensure!(!self.locked, Error::ReentrantCall);
            Ok(())
        }

        fn generate_deposit_id(&self) -> [u8; 32] {
            // In production, use proper hash
            let mut id = [0u8; 32];
            let counter_bytes = self.swap_counter.to_le_bytes();
            id[..8].copy_from_slice(&counter_bytes);
            id
        }

        fn generate_withdrawal_id(&self) -> [u8; 32] {
            // In production, use proper hash
            let mut id = [0u8; 32];
            let counter_bytes = self.swap_counter.to_le_bytes();
            id[8..16].copy_from_slice(&counter_bytes);
            id
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

        fn create_contract() -> AutoSwapExecutor {
            let accounts = default_accounts();
            AutoSwapExecutor::new(accounts.bob)
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            let accounts = default_accounts();
            assert_eq!(contract.owner(), accounts.alice);
            assert_eq!(contract.intent_router(), accounts.bob);
            assert_eq!(contract.swap_counter(), 0);
        }

        #[ink::test]
        fn set_intent_router_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();
            assert!(contract.set_intent_router(accounts.charlie).is_ok());
            assert_eq!(contract.intent_router(), accounts.charlie);
        }

        #[ink::test]
        fn only_authorized_can_execute() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let route = RouteConfig {
                route_type: RouteType::ExternalToEtr,
                tier1_pool: accounts.charlie,
                tier2_pool: accounts.django,
                wrapped_token: accounts.eve,
                bridge_tracker: accounts.frank,
            };

            let intent = SwapIntent {
                user: accounts.alice,
                source_token: accounts.bob,
                dest_token: accounts.charlie,
                amount_in: 1000,
                min_amount_out: 990,
                route,
                deadline: u64::MAX,
            };

            // Alice tries to call (not authorized)
            let result = contract.execute_swap(intent);
            assert_eq!(result, Err(Error::NotAuthorized));
        }
    }
}
