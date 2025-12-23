#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Two Tier Bridge Router Contract
///
/// Route messages between Bridge → Tier 1 → Tier 2
///
/// This contract coordinates the flow of deposits and withdrawals
/// across the bridge tracking system and the two-tier pool architecture.

#[ink::contract]
mod two_tier_bridge_router {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct TwoTierBridgeRouter {
        /// Contract owner
        owner: AccountId,
        /// External currency → Tier 1 pool
        tier1_pools: Mapping<AccountId, AccountId>,
        /// External currency → Tier 2 pool
        tier2_pools: Mapping<AccountId, AccountId>,
        /// External currency → Wrapped token
        wrapped_tokens: Mapping<AccountId, AccountId>,
        /// Tier 1 pool → Bridge tracker
        bridge_tracker_for_pool: Mapping<AccountId, AccountId>,
        /// Authorized bridge trackers
        authorized_bridge_trackers: Mapping<AccountId, bool>,
        /// Authorized tier 1 pools
        authorized_tier1_pools: Mapping<AccountId, bool>,
    }

    /// Events
    #[ink(event)]
    pub struct DepositRouted {
        #[ink(topic)]
        deposit_id: [u8; 32],
        #[ink(topic)]
        user: AccountId,
        tier1_pool: AccountId,
        tier2_pool: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct WithdrawalRouted {
        #[ink(topic)]
        withdrawal_id: [u8; 32],
        #[ink(topic)]
        user: AccountId,
        bridge_tracker: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct PoolConfigured {
        #[ink(topic)]
        external_currency: AccountId,
        tier1_pool: AccountId,
        tier2_pool: AccountId,
        wrapped_token: AccountId,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotAuthorizedBridgeTracker,
        NotAuthorizedTier1Pool,
        NoTier1Pool,
        NoTier2Pool,
        NoWrappedToken,
        UnknownPool,
        InvalidAmount,
        InvalidAddress,
        RoutingFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl TwoTierBridgeRouter {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                tier1_pools: Mapping::default(),
                tier2_pools: Mapping::default(),
                wrapped_tokens: Mapping::default(),
                bridge_tracker_for_pool: Mapping::default(),
                authorized_bridge_trackers: Mapping::default(),
                authorized_tier1_pools: Mapping::default(),
            }
        }

        /// Route verified bridge deposit to appropriate pools
        ///
        /// Called by BridgeTracker after attestation verified
        ///
        /// # Arguments
        /// * `deposit_id` - Unique deposit identifier
        /// * `user` - User account to credit
        /// * `external_currency` - External currency deposited
        /// * `amount` - Amount deposited
        ///
        /// # Flow
        /// 1. Verify caller is authorized bridge tracker
        /// 2. Get Tier 1 and Tier 2 pool addresses
        /// 3. Notify Tier 1 pool (triggers wrapped token minting)
        /// 4. Verify wrapped tokens sent to Tier 2 pool
        /// 5. Emit routing event
        #[ink(message)]
        pub fn route_bridge_deposit(
            &mut self,
            deposit_id: [u8; 32],
            user: AccountId,
            external_currency: AccountId,
            amount: Balance,
        ) -> Result<()> {
            // Verify caller is authorized bridge tracker
            let caller = self.env().caller();
            ensure!(
                self.authorized_bridge_trackers.get(caller).unwrap_or(false),
                Error::NotAuthorizedBridgeTracker
            );

            ensure!(amount > 0, Error::InvalidAmount);

            // Get pool addresses
            let tier1_pool = self
                .tier1_pools
                .get(external_currency)
                .ok_or(Error::NoTier1Pool)?;
            let tier2_pool = self
                .tier2_pools
                .get(external_currency)
                .ok_or(Error::NoTier2Pool)?;
            let _wrapped_token = self
                .wrapped_tokens
                .get(external_currency)
                .ok_or(Error::NoWrappedToken)?;

            // NOTE: In production, call:
            // ITier1Pool(tier1_pool).onBridgeDeposit(deposit_id, user, amount)

            // NOTE: In production, verify wrapped tokens minted to tier2_pool
            // let wrapped_balance = IERC20(wrapped_token).balanceOf(tier2_pool)
            // ensure!(wrapped_balance >= amount, Error::RoutingFailed)

            // Emit event
            self.env().emit_event(DepositRouted {
                deposit_id,
                user,
                tier1_pool,
                tier2_pool,
                amount,
            });

            Ok(())
        }

        /// Route withdrawal request from Tier 1 back to bridge
        ///
        /// Called by Tier 1 pool when user wants to withdraw
        ///
        /// # Arguments
        /// * `withdrawal_id` - Unique withdrawal identifier
        /// * `user` - User requesting withdrawal
        /// * `amount` - Amount to withdraw
        /// * `external_address` - Address on external chain
        #[ink(message)]
        pub fn route_withdrawal(
            &mut self,
            withdrawal_id: [u8; 32],
            user: AccountId,
            amount: Balance,
            external_address: [u8; 32],
        ) -> Result<()> {
            // Verify caller is authorized Tier 1 pool
            let caller = self.env().caller();
            ensure!(
                self.authorized_tier1_pools.get(caller).unwrap_or(false),
                Error::NotAuthorizedTier1Pool
            );

            ensure!(amount > 0, Error::InvalidAmount);
            ensure!(external_address != [0u8; 32], Error::InvalidAddress);

            // Get bridge tracker for this pool
            let bridge_tracker = self
                .bridge_tracker_for_pool
                .get(caller)
                .ok_or(Error::UnknownPool)?;

            // NOTE: In production, call:
            // IBridgeTracker(bridge_tracker).confirmWithdrawalRequest(
            //     withdrawal_id, user, amount, external_address
            // )

            // Emit event
            self.env().emit_event(WithdrawalRouted {
                withdrawal_id,
                user,
                bridge_tracker,
                amount,
            });

            Ok(())
        }

        /// Configure routing for an external currency
        ///
        /// # Arguments
        /// * `external_currency` - External currency address
        /// * `tier1_pool` - Tier 1 pool address
        /// * `tier2_pool` - Tier 2 pool address
        /// * `wrapped_token` - Wrapped token address
        /// * `bridge_tracker` - Bridge tracker address
        #[ink(message)]
        pub fn configure_pools(
            &mut self,
            external_currency: AccountId,
            tier1_pool: AccountId,
            tier2_pool: AccountId,
            wrapped_token: AccountId,
            bridge_tracker: AccountId,
        ) -> Result<()> {
            self.ensure_owner()?;

            self.tier1_pools.insert(external_currency, &tier1_pool);
            self.tier2_pools.insert(external_currency, &tier2_pool);
            self.wrapped_tokens.insert(external_currency, &wrapped_token);
            self.bridge_tracker_for_pool.insert(tier1_pool, &bridge_tracker);

            self.env().emit_event(PoolConfigured {
                external_currency,
                tier1_pool,
                tier2_pool,
                wrapped_token,
            });

            Ok(())
        }

        /// Authorize bridge tracker
        #[ink(message)]
        pub fn authorize_bridge_tracker(&mut self, bridge_tracker: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.authorized_bridge_trackers
                .insert(bridge_tracker, &true);
            Ok(())
        }

        /// Revoke bridge tracker
        #[ink(message)]
        pub fn revoke_bridge_tracker(&mut self, bridge_tracker: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.authorized_bridge_trackers
                .insert(bridge_tracker, &false);
            Ok(())
        }

        /// Authorize Tier 1 pool
        #[ink(message)]
        pub fn authorize_tier1_pool(&mut self, pool: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.authorized_tier1_pools.insert(pool, &true);
            Ok(())
        }

        /// Revoke Tier 1 pool
        #[ink(message)]
        pub fn revoke_tier1_pool(&mut self, pool: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.authorized_tier1_pools.insert(pool, &false);
            Ok(())
        }

        /// Get Tier 1 pool for external currency
        #[ink(message)]
        pub fn get_tier1_pool(&self, external_currency: AccountId) -> Option<AccountId> {
            self.tier1_pools.get(external_currency)
        }

        /// Get Tier 2 pool for external currency
        #[ink(message)]
        pub fn get_tier2_pool(&self, external_currency: AccountId) -> Option<AccountId> {
            self.tier2_pools.get(external_currency)
        }

        /// Get wrapped token for external currency
        #[ink(message)]
        pub fn get_wrapped_token(&self, external_currency: AccountId) -> Option<AccountId> {
            self.wrapped_tokens.get(external_currency)
        }

        /// Check if bridge tracker is authorized
        #[ink(message)]
        pub fn is_bridge_tracker_authorized(&self, bridge_tracker: AccountId) -> bool {
            self.authorized_bridge_trackers
                .get(bridge_tracker)
                .unwrap_or(false)
        }

        /// Check if Tier 1 pool is authorized
        #[ink(message)]
        pub fn is_tier1_pool_authorized(&self, pool: AccountId) -> bool {
            self.authorized_tier1_pools.get(pool).unwrap_or(false)
        }

        /// Transfer ownership
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.owner = new_owner;
            Ok(())
        }

        /// Get owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        // === Internal functions ===

        fn ensure_owner(&self) -> Result<()> {
            ensure!(self.env().caller() == self.owner, Error::NotOwner);
            Ok(())
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

        fn create_contract() -> TwoTierBridgeRouter {
            TwoTierBridgeRouter::new()
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            let accounts = default_accounts();
            assert_eq!(contract.owner(), accounts.alice);
        }

        #[ink::test]
        fn configure_pools_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.configure_pools(
                accounts.bob,     // external currency
                accounts.charlie, // tier1 pool
                accounts.django,  // tier2 pool
                accounts.eve,     // wrapped token
                accounts.frank,   // bridge tracker
            );

            assert!(result.is_ok());
            assert_eq!(contract.get_tier1_pool(accounts.bob), Some(accounts.charlie));
            assert_eq!(contract.get_tier2_pool(accounts.bob), Some(accounts.django));
            assert_eq!(contract.get_wrapped_token(accounts.bob), Some(accounts.eve));
        }

        #[ink::test]
        fn authorize_bridge_tracker_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            assert!(contract.authorize_bridge_tracker(accounts.bob).is_ok());
            assert!(contract.is_bridge_tracker_authorized(accounts.bob));
        }

        #[ink::test]
        fn authorize_tier1_pool_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            assert!(contract.authorize_tier1_pool(accounts.bob).is_ok());
            assert!(contract.is_tier1_pool_authorized(accounts.bob));
        }

        #[ink::test]
        fn route_deposit_requires_authorization() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.route_bridge_deposit(
                [1u8; 32],
                accounts.alice,
                accounts.bob,
                1000,
            );

            assert_eq!(result, Err(Error::NotAuthorizedBridgeTracker));
        }

        #[ink::test]
        fn route_withdrawal_requires_authorization() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.route_withdrawal(
                [1u8; 32],
                accounts.alice,
                1000,
                [2u8; 32],
            );

            assert_eq!(result, Err(Error::NotAuthorizedTier1Pool));
        }
    }
}
