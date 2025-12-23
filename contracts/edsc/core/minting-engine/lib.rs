#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # EDSC Minting Engine
///
/// Transaction-driven minting of EDSC backed 1:1 by reserves.
///
/// Features:
/// - Direct stablecoin minting (USDC, USDT, DAI)
/// - Volatile asset minting with auto-swap (BTC, ETH)
/// - Cross-PBC routing for stablecoin purchases
/// - Rate limiting (max mint per transaction)
/// - Reserve-first guarantee (deposit succeeds before minting)

#[ink::contract]
mod minting_engine {
    use ink::storage::Mapping;

    /// Maximum mint per transaction (1M EDSC)
    const MAX_MINT_PER_TX: Balance = 1_000_000_000_000_000_000_000_000;

    /// Minimum mint amount (0.01 EDSC)
    const MIN_MINT_AMOUNT: Balance = 10_000_000_000_000_000;

    /// The contract's storage
    #[ink(storage)]
    pub struct MintingEngine {
        /// EDSC token contract
        edsc_token: AccountId,
        /// Reserve vault contract
        reserve_vault: AccountId,
        /// External swap router contract
        swap_router: AccountId,
        /// USDC token address
        usdc_token: AccountId,
        /// USDT token address
        usdt_token: AccountId,
        /// DAI token address
        dai_token: AccountId,
        /// Wrapped BTC token address
        wbtc_token: AccountId,
        /// Wrapped ETH token address
        weth_token: AccountId,
        /// Contract owner
        owner: AccountId,
        /// Total EDSC minted
        total_minted: Balance,
        /// Minting enabled
        minting_enabled: bool,
        /// Paused state
        paused: bool,
        /// Reentrancy guard
        locked: bool,
        /// Per-user mint count (for tracking)
        user_mint_count: Mapping<AccountId, u64>,
    }

    /// Events
    #[ink(event)]
    pub struct EDSCMinted {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        source_token: AccountId,
        source_amount: Balance,
        edsc_amount: Balance,
        total_minted: Balance,
    }

    #[ink(event)]
    pub struct EDSCMintedFromSwap {
        #[ink(topic)]
        user: AccountId,
        #[ink(topic)]
        volatile_asset: AccountId,
        volatile_amount: Balance,
        usdc_received: Balance,
        edsc_amount: Balance,
    }

    #[ink(event)]
    pub struct CrossPBCMint {
        #[ink(topic)]
        user: AccountId,
        source_pbc: u32,
        stablecoin: AccountId,
        amount: Balance,
        edsc_minted: Balance,
    }

    #[ink(event)]
    pub struct MintingToggled {
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
        /// Minting is disabled
        MintingDisabled,
        /// Reentrancy detected
        ReentrancyDetected,
        /// Zero amount
        ZeroAmount,
        /// Amount too small
        AmountTooSmall,
        /// Amount exceeds max per transaction
        ExceedsMaxPerTx,
        /// Invalid token
        InvalidToken,
        /// Reserve deposit failed
        ReserveDepositFailed,
        /// EDSC minting failed
        MintingFailed,
        /// Swap failed
        SwapFailed,
        /// Insufficient output
        InsufficientOutput,
        /// Overflow
        Overflow,
        /// Cross-call failed
        CrossCallFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl MintingEngine {
        /// Constructor
        #[ink(constructor)]
        pub fn new(
            edsc_token: AccountId,
            reserve_vault: AccountId,
            swap_router: AccountId,
            usdc_token: AccountId,
            usdt_token: AccountId,
            dai_token: AccountId,
            wbtc_token: AccountId,
            weth_token: AccountId,
        ) -> Self {
            let caller = Self::env().caller();

            Self {
                edsc_token,
                reserve_vault,
                swap_router,
                usdc_token,
                usdt_token,
                dai_token,
                wbtc_token,
                weth_token,
                owner: caller,
                total_minted: 0,
                minting_enabled: true,
                paused: false,
                locked: false,
                user_mint_count: Mapping::default(),
            }
        }

        /// Mints EDSC with USDC
        #[ink(message)]
        pub fn mint_with_usdc(&mut self, usdc_amount: Balance) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let user = self.env().caller();
            let result = self.mint_with_stablecoin(user, self.usdc_token, usdc_amount);

            self.locked = false;
            result
        }

        /// Mints EDSC with USDT
        #[ink(message)]
        pub fn mint_with_usdt(&mut self, usdt_amount: Balance) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let user = self.env().caller();
            let result = self.mint_with_stablecoin(user, self.usdt_token, usdt_amount);

            self.locked = false;
            result
        }

        /// Mints EDSC with DAI
        #[ink(message)]
        pub fn mint_with_dai(&mut self, dai_amount: Balance) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let user = self.env().caller();
            let result = self.mint_with_stablecoin(user, self.dai_token, dai_amount);

            self.locked = false;
            result
        }

        /// Mints EDSC with BTC (auto-swaps to USDC first)
        #[ink(message)]
        pub fn mint_with_btc(&mut self, btc_amount: Balance) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let result = self.mint_with_volatile_asset(self.wbtc_token, btc_amount);

            self.locked = false;
            result
        }

        /// Mints EDSC with ETH (auto-swaps to USDC first)
        #[ink(message)]
        pub fn mint_with_eth(&mut self, eth_amount: Balance) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let result = self.mint_with_volatile_asset(self.weth_token, eth_amount);

            self.locked = false;
            result
        }

        /// Routes stablecoin purchase from another PBC
        #[ink(message)]
        pub fn route_stablecoin_purchase(
            &mut self,
            source_pbc: u32,
            user: AccountId,
            stablecoin: AccountId,
            amount: Balance,
        ) -> Result<Balance> {
            self.ensure_can_mint()?;
            self.ensure_not_locked()?;
            self.locked = true;

            // Validate stablecoin
            if stablecoin != self.usdc_token
                && stablecoin != self.usdt_token
                && stablecoin != self.dai_token
            {
                self.locked = false;
                return Err(Error::InvalidToken);
            }

            let result = self.mint_with_stablecoin(user, stablecoin, amount);

            if let Ok(edsc_amount) = result {
                self.env().emit_event(CrossPBCMint {
                    user,
                    source_pbc,
                    stablecoin,
                    amount,
                    edsc_minted: edsc_amount,
                });
            }

            self.locked = false;
            result
        }

        /// Returns total EDSC minted
        #[ink(message)]
        pub fn get_total_minted(&self) -> Balance {
            self.total_minted
        }

        /// Returns user mint count
        #[ink(message)]
        pub fn get_user_mint_count(&self, user: AccountId) -> u64 {
            self.user_mint_count.get(&user).unwrap_or(0)
        }

        /// Returns contract addresses
        #[ink(message)]
        pub fn get_contract_addresses(&self) -> (AccountId, AccountId, AccountId) {
            (self.edsc_token, self.reserve_vault, self.swap_router)
        }

        /// Toggles minting (owner only)
        #[ink(message)]
        pub fn toggle_minting(&mut self, enabled: bool) -> Result<()> {
            self.ensure_owner()?;

            self.minting_enabled = enabled;

            self.env().emit_event(MintingToggled { enabled });

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

        /// Internal: Mints EDSC with stablecoin
        fn mint_with_stablecoin(
            &mut self,
            user: AccountId,
            stablecoin: AccountId,
            amount: Balance,
        ) -> Result<Balance> {
            self.validate_mint_amount(amount)?;

            // Step 1: Deposit stablecoin to reserve vault
            // NOTE: In production, this would be a cross-contract call
            // For now, we simulate the deposit
            let deposit_successful = self.deposit_to_reserve(stablecoin, amount)?;

            if !deposit_successful {
                return Err(Error::ReserveDepositFailed);
            }

            // Step 2: Mint EDSC 1:1 with deposited amount
            let edsc_amount = amount; // 1:1 ratio

            // NOTE: In production, this would call edsc_token.mint(user, edsc_amount)
            // For now, we track it internally
            self.total_minted = self.total_minted
                .checked_add(edsc_amount)
                .ok_or(Error::Overflow)?;

            // Update user mint count
            let current_count = self.user_mint_count.get(&user).unwrap_or(0);
            self.user_mint_count.insert(&user, &(current_count + 1));

            self.env().emit_event(EDSCMinted {
                user,
                source_token: stablecoin,
                source_amount: amount,
                edsc_amount,
                total_minted: self.total_minted,
            });

            Ok(edsc_amount)
        }

        /// Internal: Mints EDSC with volatile asset (auto-swap)
        fn mint_with_volatile_asset(
            &mut self,
            volatile_asset: AccountId,
            amount: Balance,
        ) -> Result<Balance> {
            self.validate_mint_amount(amount)?;

            let user = self.env().caller();

            // Step 1: Swap volatile asset to USDC via external router
            // NOTE: In production, this would call swap_router.swap_to_usdc()
            // For now, we simulate a 99% conversion (1% slippage)
            let usdc_received = (amount * 99) / 100;

            if usdc_received < MIN_MINT_AMOUNT {
                return Err(Error::InsufficientOutput);
            }

            // Step 2: Deposit USDC to reserve vault
            let deposit_successful = self.deposit_to_reserve(self.usdc_token, usdc_received)?;

            if !deposit_successful {
                return Err(Error::ReserveDepositFailed);
            }

            // Step 3: Mint EDSC 1:1 with USDC received
            let edsc_amount = usdc_received;

            self.total_minted = self.total_minted
                .checked_add(edsc_amount)
                .ok_or(Error::Overflow)?;

            // Update user mint count
            let current_count = self.user_mint_count.get(&user).unwrap_or(0);
            self.user_mint_count.insert(&user, &(current_count + 1));

            self.env().emit_event(EDSCMintedFromSwap {
                user,
                volatile_asset,
                volatile_amount: amount,
                usdc_received,
                edsc_amount,
            });

            Ok(edsc_amount)
        }

        /// Simulates deposit to reserve vault
        /// NOTE: In production, this would be a cross-contract call
        fn deposit_to_reserve(&self, token: AccountId, amount: Balance) -> Result<bool> {
            // Placeholder: Would call reserve_vault.deposit_usdc/usdt/dai(amount)
            Ok(true)
        }

        fn validate_mint_amount(&self, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            if amount < MIN_MINT_AMOUNT {
                return Err(Error::AmountTooSmall);
            }

            if amount > MAX_MINT_PER_TX {
                return Err(Error::ExceedsMaxPerTx);
            }

            Ok(())
        }

        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                Err(Error::NotOwner)
            } else {
                Ok(())
            }
        }

        fn ensure_can_mint(&self) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            if !self.minting_enabled {
                return Err(Error::MintingDisabled);
            }

            Ok(())
        }

        fn ensure_not_locked(&self) -> Result<()> {
            if self.locked {
                Err(Error::ReentrancyDetected)
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

        fn create_contract() -> MintingEngine {
            let accounts = get_accounts();
            MintingEngine::new(
                accounts.alice,  // edsc_token
                accounts.bob,    // reserve_vault
                accounts.charlie, // swap_router
                accounts.django, // usdc
                accounts.eve,    // usdt
                accounts.frank,  // dai
                AccountId::from([0x1; 32]), // wbtc
                AccountId::from([0x2; 32]), // weth
            )
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            assert_eq!(contract.get_total_minted(), 0);
        }

        #[ink::test]
        fn mint_with_usdc_works() {
            let mut contract = create_contract();
            let amount = 1000_000_000_000_000_000_000;

            let result = contract.mint_with_usdc(amount);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), amount);
            assert_eq!(contract.get_total_minted(), amount);
        }

        #[ink::test]
        fn mint_fails_zero_amount() {
            let mut contract = create_contract();
            let result = contract.mint_with_usdc(0);
            assert_eq!(result, Err(Error::ZeroAmount));
        }

        #[ink::test]
        fn mint_fails_exceeds_max() {
            let mut contract = create_contract();
            let result = contract.mint_with_usdc(MAX_MINT_PER_TX + 1);
            assert_eq!(result, Err(Error::ExceedsMaxPerTx));
        }

        #[ink::test]
        fn mint_with_btc_works() {
            let mut contract = create_contract();
            let btc_amount = 1_000_000_000_000_000_000;

            let result = contract.mint_with_btc(btc_amount);
            assert!(result.is_ok());

            // Should receive 99% after 1% slippage
            let expected_edsc = (btc_amount * 99) / 100;
            assert_eq!(result.unwrap(), expected_edsc);
        }
    }
}
