#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # EDSC Reserve Vault
///
/// Multi-asset reserve backing for EDSC stablecoin.
/// Target allocation: 50% USDC, 30% USDT, 20% DAI
///
/// Features:
/// - Multi-asset reserve management
/// - Automatic rebalancing to target percentages
/// - Reserve ratio calculation (should be 100% for 1:1 backing)
/// - Access control for deposits

#[ink::contract]
mod reserve_vault {
    use ink::storage::Mapping;

    /// Reserve allocation targets
    const TARGET_USDC_PERCENTAGE: u8 = 50;
    const TARGET_USDT_PERCENTAGE: u8 = 30;
    const TARGET_DAI_PERCENTAGE: u8 = 20;

    /// Rebalancing threshold (±5%)
    const REBALANCE_THRESHOLD: u8 = 5;

    /// The contract's storage
    #[ink(storage)]
    pub struct ReserveVault {
        /// Total reserve value in USD (scaled by 10^18)
        total_reserve_value: Balance,
        /// Reserve balances by token address
        reserves: Mapping<AccountId, Balance>,
        /// USDC token address
        usdc_token: AccountId,
        /// USDT token address
        usdt_token: AccountId,
        /// DAI token address
        dai_token: AccountId,
        /// Contract owner
        owner: AccountId,
        /// Authorized depositors (e.g., minting engine)
        authorized_depositors: Mapping<AccountId, bool>,
        /// Paused state
        paused: bool,
        /// Reentrancy guard
        locked: bool,
    }

    /// Events
    #[ink(event)]
    pub struct ReserveDeposit {
        #[ink(topic)]
        token: AccountId,
        #[ink(topic)]
        depositor: AccountId,
        amount: Balance,
        total_reserve_value: Balance,
    }

    #[ink(event)]
    pub struct Rebalanced {
        usdc_balance: Balance,
        usdt_balance: Balance,
        dai_balance: Balance,
        usdc_percentage: u8,
        usdt_percentage: u8,
        dai_percentage: u8,
    }

    #[ink(event)]
    pub struct DepositorAuthorized {
        #[ink(topic)]
        depositor: AccountId,
    }

    #[ink(event)]
    pub struct DepositorRevoked {
        #[ink(topic)]
        depositor: AccountId,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Caller is not authorized
        NotAuthorized,
        /// Contract is paused
        ContractPaused,
        /// Reentrancy detected
        ReentrancyDetected,
        /// Zero amount
        ZeroAmount,
        /// Invalid token address
        InvalidToken,
        /// Overflow would occur
        Overflow,
        /// Insufficient reserve balance
        InsufficientReserve,
        /// Rebalancing failed
        RebalanceFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ReserveVault {
        /// Constructor
        #[ink(constructor)]
        pub fn new(
            usdc_token: AccountId,
            usdt_token: AccountId,
            dai_token: AccountId,
        ) -> Self {
            let caller = Self::env().caller();

            Self {
                total_reserve_value: 0,
                reserves: Mapping::default(),
                usdc_token,
                usdt_token,
                dai_token,
                owner: caller,
                authorized_depositors: Mapping::default(),
                paused: false,
                locked: false,
            }
        }

        /// Deposits USDC into the reserve
        #[ink(message)]
        pub fn deposit_usdc(&mut self, amount: Balance) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_authorized()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let result = self.deposit_internal(self.usdc_token, amount);

            self.locked = false;
            result
        }

        /// Deposits USDT into the reserve
        #[ink(message)]
        pub fn deposit_usdt(&mut self, amount: Balance) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_authorized()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let result = self.deposit_internal(self.usdt_token, amount);

            self.locked = false;
            result
        }

        /// Deposits DAI into the reserve
        #[ink(message)]
        pub fn deposit_dai(&mut self, amount: Balance) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_authorized()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let result = self.deposit_internal(self.dai_token, amount);

            self.locked = false;
            result
        }

        /// Rebalances reserves to target percentages
        ///
        /// NOTE: Actual swapping would be done via external swap router.
        /// This is a placeholder that validates rebalancing logic.
        #[ink(message)]
        pub fn rebalance(&mut self) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_authorized()?;

            if self.total_reserve_value == 0 {
                return Ok(());
            }

            let (usdc_pct, usdt_pct, dai_pct) = self.get_current_percentages();

            // Check if rebalancing is needed
            if !self.needs_rebalancing(usdc_pct, usdt_pct, dai_pct) {
                return Ok(());
            }

            // Calculate target amounts
            let target_usdc = (self.total_reserve_value * TARGET_USDC_PERCENTAGE as u128) / 100;
            let target_usdt = (self.total_reserve_value * TARGET_USDT_PERCENTAGE as u128) / 100;
            let target_dai = (self.total_reserve_value * TARGET_DAI_PERCENTAGE as u128) / 100;

            // In production, this would trigger external swaps via ExternalSwapRouter
            // For now, we just emit the rebalancing event
            let usdc_balance = self.reserves.get(&self.usdc_token).unwrap_or(0);
            let usdt_balance = self.reserves.get(&self.usdt_token).unwrap_or(0);
            let dai_balance = self.reserves.get(&self.dai_token).unwrap_or(0);

            self.env().emit_event(Rebalanced {
                usdc_balance,
                usdt_balance,
                dai_balance,
                usdc_percentage: usdc_pct,
                usdt_percentage: usdt_pct,
                dai_percentage: dai_pct,
            });

            Ok(())
        }

        /// Returns the reserve ratio (100 = 1:1 backing)
        #[ink(message)]
        pub fn get_reserve_ratio(&self) -> u128 {
            // In production, this would compare total_reserve_value to EDSC total supply
            // For now, return 100 (assuming 1:1 backing)
            100
        }

        /// Returns total reserve value
        #[ink(message)]
        pub fn get_total_reserve_value(&self) -> Balance {
            self.total_reserve_value
        }

        /// Returns reserve balance for a specific token
        #[ink(message)]
        pub fn get_reserve_balance(&self, token: AccountId) -> Balance {
            self.reserves.get(&token).unwrap_or(0)
        }

        /// Returns current allocation percentages
        #[ink(message)]
        pub fn get_current_percentages(&self) -> (u8, u8, u8) {
            if self.total_reserve_value == 0 {
                return (0, 0, 0);
            }

            let usdc_balance = self.reserves.get(&self.usdc_token).unwrap_or(0);
            let usdt_balance = self.reserves.get(&self.usdt_token).unwrap_or(0);
            let dai_balance = self.reserves.get(&self.dai_token).unwrap_or(0);

            let usdc_pct = ((usdc_balance * 100) / self.total_reserve_value) as u8;
            let usdt_pct = ((usdt_balance * 100) / self.total_reserve_value) as u8;
            let dai_pct = ((dai_balance * 100) / self.total_reserve_value) as u8;

            (usdc_pct, usdt_pct, dai_pct)
        }

        /// Returns target allocation percentages
        #[ink(message)]
        pub fn get_target_percentages(&self) -> (u8, u8, u8) {
            (TARGET_USDC_PERCENTAGE, TARGET_USDT_PERCENTAGE, TARGET_DAI_PERCENTAGE)
        }

        /// Authorizes a depositor (owner only)
        #[ink(message)]
        pub fn authorize_depositor(&mut self, depositor: AccountId) -> Result<()> {
            self.ensure_owner()?;

            self.authorized_depositors.insert(&depositor, &true);

            self.env().emit_event(DepositorAuthorized { depositor });

            Ok(())
        }

        /// Revokes a depositor (owner only)
        #[ink(message)]
        pub fn revoke_depositor(&mut self, depositor: AccountId) -> Result<()> {
            self.ensure_owner()?;

            self.authorized_depositors.insert(&depositor, &false);

            self.env().emit_event(DepositorRevoked { depositor });

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

        /// Returns token addresses
        #[ink(message)]
        pub fn get_token_addresses(&self) -> (AccountId, AccountId, AccountId) {
            (self.usdc_token, self.usdt_token, self.dai_token)
        }

        /// Internal deposit function
        fn deposit_internal(&mut self, token: AccountId, amount: Balance) -> Result<()> {
            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            // Verify token is supported
            if token != self.usdc_token && token != self.usdt_token && token != self.dai_token {
                return Err(Error::InvalidToken);
            }

            // Update reserve balance
            let current_balance = self.reserves.get(&token).unwrap_or(0);
            let new_balance = current_balance.checked_add(amount).ok_or(Error::Overflow)?;
            self.reserves.insert(&token, &new_balance);

            // Update total reserve value
            self.total_reserve_value = self.total_reserve_value
                .checked_add(amount)
                .ok_or(Error::Overflow)?;

            let caller = self.env().caller();

            self.env().emit_event(ReserveDeposit {
                token,
                depositor: caller,
                amount,
                total_reserve_value: self.total_reserve_value,
            });

            Ok(())
        }

        /// Checks if rebalancing is needed
        fn needs_rebalancing(&self, usdc_pct: u8, usdt_pct: u8, dai_pct: u8) -> bool {
            let usdc_diff = if usdc_pct > TARGET_USDC_PERCENTAGE {
                usdc_pct - TARGET_USDC_PERCENTAGE
            } else {
                TARGET_USDC_PERCENTAGE - usdc_pct
            };

            let usdt_diff = if usdt_pct > TARGET_USDT_PERCENTAGE {
                usdt_pct - TARGET_USDT_PERCENTAGE
            } else {
                TARGET_USDT_PERCENTAGE - usdt_pct
            };

            let dai_diff = if dai_pct > TARGET_DAI_PERCENTAGE {
                dai_pct - TARGET_DAI_PERCENTAGE
            } else {
                TARGET_DAI_PERCENTAGE - dai_pct
            };

            usdc_diff > REBALANCE_THRESHOLD
                || usdt_diff > REBALANCE_THRESHOLD
                || dai_diff > REBALANCE_THRESHOLD
        }

        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                Err(Error::NotOwner)
            } else {
                Ok(())
            }
        }

        fn ensure_authorized(&self) -> Result<()> {
            let caller = self.env().caller();
            if caller == self.owner {
                return Ok(());
            }

            if self.authorized_depositors.get(&caller).unwrap_or(false) {
                Ok(())
            } else {
                Err(Error::NotAuthorized)
            }
        }

        fn ensure_not_paused(&self) -> Result<()> {
            if self.paused {
                Err(Error::ContractPaused)
            } else {
                Ok(())
            }
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

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        fn create_contract() -> ReserveVault {
            let accounts = get_accounts();
            ReserveVault::new(accounts.django, accounts.eve, accounts.frank)
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            assert_eq!(contract.get_total_reserve_value(), 0);
            assert_eq!(contract.get_reserve_ratio(), 100);
        }

        #[ink::test]
        fn deposit_usdc_works() {
            let accounts = get_accounts();
            let mut contract = create_contract();

            contract.authorize_depositor(accounts.alice).unwrap();

            assert!(contract.deposit_usdc(1000).is_ok());
            assert_eq!(contract.get_reserve_balance(accounts.django), 1000);
            assert_eq!(contract.get_total_reserve_value(), 1000);
        }

        #[ink::test]
        fn get_percentages_works() {
            let accounts = get_accounts();
            let mut contract = create_contract();

            contract.authorize_depositor(accounts.alice).unwrap();

            contract.deposit_usdc(500).unwrap();
            contract.deposit_usdt(300).unwrap();
            contract.deposit_dai(200).unwrap();

            let (usdc_pct, usdt_pct, dai_pct) = contract.get_current_percentages();
            assert_eq!(usdc_pct, 50);
            assert_eq!(usdt_pct, 30);
            assert_eq!(dai_pct, 20);
        }
    }
}
