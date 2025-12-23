#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Wrapped Token Template
///
/// This contract implements an ERC20-like wrapped token that represents
/// external currencies locked in Tier 1 reserve pools.
///
/// Supported currencies: BTC, ETH, SOL, BNB, TRX, XRP, ADA, DOGE, LINK, XLM, MATIC
///
/// Key features:
/// - ERC20 standard interface (transfer, approve, transferFrom)
/// - Controlled minting (only authorized minters)
/// - Burning capability (for redemptions)
/// - 1:1 backing with external currency
/// - Balance tracking and allowances
///
/// Part of the PrimeSwap Two-Tier Architecture for ĒTRID.

#[ink::contract]
mod wrapped_token {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct WrappedToken {
        /// Total token supply
        total_supply: Balance,
        /// Mapping from account to token balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping from (owner, spender) to allowance amount
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Token name (e.g., "Wrapped Bitcoin")
        name: String,
        /// Token symbol (e.g., "wBTC")
        symbol: String,
        /// Token decimals (usually 8 for BTC, 18 for ETH)
        decimals: u8,
        /// Contract owner
        owner: AccountId,
        /// Authorized minters (Tier 1 pools)
        minters: Mapping<AccountId, bool>,
        /// Emergency pause state
        paused: bool,
        /// Total minted amount (for audit)
        total_minted: Balance,
        /// Total burned amount (for audit)
        total_burned: Balance,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        minter: AccountId,
        value: Balance,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        burner: AccountId,
        value: Balance,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct MinterAdded {
        #[ink(topic)]
        minter: AccountId,
        #[ink(topic)]
        added_by: AccountId,
        timestamp: Timestamp,
    }

    #[ink(event)]
    pub struct MinterRemoved {
        #[ink(topic)]
        minter: AccountId,
        #[ink(topic)]
        removed_by: AccountId,
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

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Insufficient balance for transfer
        InsufficientBalance,
        /// Insufficient allowance for transfer
        InsufficientAllowance,
        /// Transfer to zero address
        TransferToZeroAddress,
        /// Approve to zero address
        ApproveToZeroAddress,
        /// Caller is not the owner
        NotOwner,
        /// Caller is not an authorized minter
        NotMinter,
        /// Overflow would occur
        Overflow,
        /// Transfer amount is zero
        ZeroAmount,
        /// Contract is paused
        ContractPaused,
        /// Invalid address
        InvalidAddress,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl WrappedToken {
        /// Constructor: Creates a new Wrapped Token
        ///
        /// # Arguments
        /// * `name` - Token name (e.g., "Wrapped Bitcoin")
        /// * `symbol` - Token symbol (e.g., "wBTC")
        /// * `decimals` - Decimal places (8 for BTC, 18 for ETH)
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            decimals: u8,
        ) -> Self {
            let caller = Self::env().caller();

            Self {
                total_supply: 0,
                balances: Mapping::default(),
                allowances: Mapping::default(),
                name,
                symbol,
                decimals,
                owner: caller,
                minters: Mapping::default(),
                paused: false,
                total_minted: 0,
                total_burned: 0,
            }
        }

        /// Returns the token name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Returns the token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Returns the number of decimals
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }

        /// Returns the total token supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of an account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or(0)
        }

        /// Returns the allowance (owner → spender)
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }

        /// Returns the contract owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Returns paused state
        #[ink(message)]
        pub fn paused(&self) -> bool {
            self.paused
        }

        /// Returns total minted amount
        #[ink(message)]
        pub fn total_minted(&self) -> Balance {
            self.total_minted
        }

        /// Returns total burned amount
        #[ink(message)]
        pub fn total_burned(&self) -> Balance {
            self.total_burned
        }

        /// Check if an address is an authorized minter
        #[ink(message)]
        pub fn is_minter(&self, account: AccountId) -> bool {
            self.minters.get(&account).unwrap_or(false)
        }

        /// Transfers tokens from caller to recipient
        ///
        /// # Arguments
        /// * `to` - Recipient address
        /// * `value` - Amount to transfer
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `InsufficientBalance` - Caller doesn't have enough tokens
        /// * `TransferToZeroAddress` - Recipient is zero address
        /// * `ZeroAmount` - Transfer amount is zero
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Approves spender to spend tokens on behalf of caller
        ///
        /// # Arguments
        /// * `spender` - Address that can spend tokens
        /// * `value` - Maximum amount they can spend
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `ApproveToZeroAddress` - Spender is zero address
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let owner = self.env().caller();

            if spender == AccountId::from([0x0; 32]) {
                return Err(Error::ApproveToZeroAddress);
            }

            self.allowances.insert(&(owner, spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        /// Transfers tokens from one address to another using allowance
        ///
        /// # Arguments
        /// * `from` - Source address
        /// * `to` - Recipient address
        /// * `value` - Amount to transfer
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `InsufficientAllowance` - Caller doesn't have enough allowance
        /// * `InsufficientBalance` - Source doesn't have enough tokens
        /// * `TransferToZeroAddress` - Recipient is zero address
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(&from, &to, value)?;

            // Decrease allowance
            let new_allowance = allowance - value;
            self.allowances.insert(&(from, caller), &new_allowance);

            Ok(())
        }

        /// Increases allowance for a spender
        ///
        /// Safer than approve() for avoiding race conditions
        #[ink(message)]
        pub fn increase_allowance(
            &mut self,
            spender: AccountId,
            added_value: Balance,
        ) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let owner = self.env().caller();
            let current_allowance = self.allowance(owner, spender);

            let new_allowance = current_allowance
                .checked_add(added_value)
                .ok_or(Error::Overflow)?;

            self.allowances.insert(&(owner, spender), &new_allowance);

            self.env().emit_event(Approval {
                owner,
                spender,
                value: new_allowance,
            });

            Ok(())
        }

        /// Decreases allowance for a spender
        ///
        /// Safer than approve() for avoiding race conditions
        #[ink(message)]
        pub fn decrease_allowance(
            &mut self,
            spender: AccountId,
            subtracted_value: Balance,
        ) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let owner = self.env().caller();
            let current_allowance = self.allowance(owner, spender);

            if current_allowance < subtracted_value {
                return Err(Error::InsufficientAllowance);
            }

            let new_allowance = current_allowance - subtracted_value;
            self.allowances.insert(&(owner, spender), &new_allowance);

            self.env().emit_event(Approval {
                owner,
                spender,
                value: new_allowance,
            });

            Ok(())
        }

        /// Mints new tokens (authorized minters only)
        ///
        /// # Arguments
        /// * `to` - Recipient address
        /// * `value` - Amount to mint
        ///
        /// # Errors
        /// * `NotMinter` - Caller is not an authorized minter
        /// * `ContractPaused` - Contract is paused
        /// * `ZeroAmount` - Mint amount is zero
        /// * `Overflow` - Would exceed max supply
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();

            // Check if caller is authorized minter or owner
            if !self.is_minter(caller) && caller != self.owner {
                return Err(Error::NotMinter);
            }

            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            if to == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            // Increase total supply
            self.total_supply = self.total_supply
                .checked_add(value)
                .ok_or(Error::Overflow)?;

            // Increase recipient balance
            let to_balance = self.balance_of(to);
            let new_balance = to_balance.checked_add(value).ok_or(Error::Overflow)?;
            self.balances.insert(&to, &new_balance);

            // Track minting
            self.total_minted = self.total_minted
                .checked_add(value)
                .ok_or(Error::Overflow)?;

            let now = self.env().block_timestamp();

            self.env().emit_event(Mint {
                to,
                minter: caller,
                value,
                timestamp: now,
            });

            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value,
            });

            Ok(())
        }

        /// Burns tokens from caller's balance
        ///
        /// # Arguments
        /// * `value` - Amount to burn
        ///
        /// # Errors
        /// * `ContractPaused` - Contract is paused
        /// * `ZeroAmount` - Burn amount is zero
        /// * `InsufficientBalance` - Caller doesn't have enough tokens
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();

            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller_balance = self.balance_of(caller);
            if caller_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Decrease total supply
            self.total_supply = self.total_supply - value;

            // Decrease caller balance
            let new_balance = caller_balance - value;
            self.balances.insert(&caller, &new_balance);

            // Track burning
            self.total_burned = self.total_burned
                .checked_add(value)
                .ok_or(Error::Overflow)?;

            let now = self.env().block_timestamp();

            self.env().emit_event(Burn {
                from: caller,
                burner: caller,
                value,
                timestamp: now,
            });

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: None,
                value,
            });

            Ok(())
        }

        /// Burns tokens from a specific account (authorized minters only)
        ///
        /// Used by Tier 1 pools for redemptions
        ///
        /// # Arguments
        /// * `from` - Account to burn from
        /// * `value` - Amount to burn
        #[ink(message)]
        pub fn burn_from(&mut self, from: AccountId, value: Balance) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }

            let caller = self.env().caller();

            // Check if caller is authorized minter or owner
            if !self.is_minter(caller) && caller != self.owner {
                return Err(Error::NotMinter);
            }

            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Decrease total supply
            self.total_supply = self.total_supply - value;

            // Decrease account balance
            let new_balance = from_balance - value;
            self.balances.insert(&from, &new_balance);

            // Track burning
            self.total_burned = self.total_burned
                .checked_add(value)
                .ok_or(Error::Overflow)?;

            let now = self.env().block_timestamp();

            self.env().emit_event(Burn {
                from,
                burner: caller,
                value,
                timestamp: now,
            });

            self.env().emit_event(Transfer {
                from: Some(from),
                to: None,
                value,
            });

            Ok(())
        }

        /// Add an authorized minter (owner only)
        #[ink(message)]
        pub fn add_minter(&mut self, minter: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if minter == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            self.minters.insert(&minter, &true);

            self.env().emit_event(MinterAdded {
                minter,
                added_by: caller,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        /// Remove an authorized minter (owner only)
        #[ink(message)]
        pub fn remove_minter(&mut self, minter: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.minters.insert(&minter, &false);

            self.env().emit_event(MinterRemoved {
                minter,
                removed_by: caller,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
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

        /// Internal transfer function
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::TransferToZeroAddress);
            }

            let from_balance = self.balance_of(*from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Update balances
            let new_from_balance = from_balance - value;
            self.balances.insert(from, &new_from_balance);

            let to_balance = self.balance_of(*to);
            let new_to_balance = to_balance.checked_add(value).ok_or(Error::Overflow)?;
            self.balances.insert(to, &new_to_balance);

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
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

        fn create_token() -> WrappedToken {
            WrappedToken::new(
                "Wrapped Bitcoin".to_string(),
                "wBTC".to_string(),
                8,
            )
        }

        #[ink::test]
        fn new_works() {
            let token = create_token();
            assert_eq!(token.name(), "Wrapped Bitcoin");
            assert_eq!(token.symbol(), "wBTC");
            assert_eq!(token.decimals(), 8);
            assert_eq!(token.total_supply(), 0);
            assert!(!token.paused());
        }

        #[ink::test]
        fn mint_works_for_owner() {
            let accounts = create_accounts();
            let mut token = create_token();

            let result = token.mint(accounts.bob, 1_000_000);
            assert!(result.is_ok());

            assert_eq!(token.total_supply(), 1_000_000);
            assert_eq!(token.balance_of(accounts.bob), 1_000_000);
            assert_eq!(token.total_minted(), 1_000_000);
        }

        #[ink::test]
        fn mint_fails_not_minter() {
            let accounts = create_accounts();
            let mut token = create_token();

            set_caller(accounts.bob);
            let result = token.mint(accounts.charlie, 1_000_000);
            assert_eq!(result, Err(Error::NotMinter));
        }

        #[ink::test]
        fn mint_works_for_authorized_minter() {
            let accounts = create_accounts();
            let mut token = create_token();

            // Add Bob as minter
            token.add_minter(accounts.bob).unwrap();

            set_caller(accounts.bob);
            let result = token.mint(accounts.charlie, 500_000);
            assert!(result.is_ok());
            assert_eq!(token.total_supply(), 500_000);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.alice, 1_000_000).unwrap();
            let result = token.burn(300_000);
            assert!(result.is_ok());

            assert_eq!(token.total_supply(), 700_000);
            assert_eq!(token.balance_of(accounts.alice), 700_000);
            assert_eq!(token.total_burned(), 300_000);
        }

        #[ink::test]
        fn burn_fails_insufficient_balance() {
            let mut token = create_token();

            let result = token.burn(100_000);
            assert_eq!(result, Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn burn_from_works_for_minter() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.bob, 1_000_000).unwrap();
            token.add_minter(accounts.charlie).unwrap();

            set_caller(accounts.charlie);
            let result = token.burn_from(accounts.bob, 200_000);
            assert!(result.is_ok());

            assert_eq!(token.balance_of(accounts.bob), 800_000);
            assert_eq!(token.total_supply(), 800_000);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.alice, 1_000_000).unwrap();

            let result = token.transfer(accounts.bob, 300_000);
            assert!(result.is_ok());

            assert_eq!(token.balance_of(accounts.alice), 700_000);
            assert_eq!(token.balance_of(accounts.bob), 300_000);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            let accounts = create_accounts();
            let mut token = create_token();

            let result = token.transfer(accounts.bob, 100_000);
            assert_eq!(result, Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn approve_works() {
            let accounts = create_accounts();
            let mut token = create_token();

            let result = token.approve(accounts.bob, 500_000);
            assert!(result.is_ok());

            assert_eq!(token.allowance(accounts.alice, accounts.bob), 500_000);
        }

        #[ink::test]
        fn transfer_from_works() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.alice, 1_000_000).unwrap();
            token.approve(accounts.bob, 500_000).unwrap();

            set_caller(accounts.bob);
            let result = token.transfer_from(accounts.alice, accounts.charlie, 200_000);
            assert!(result.is_ok());

            assert_eq!(token.balance_of(accounts.alice), 800_000);
            assert_eq!(token.balance_of(accounts.charlie), 200_000);
            assert_eq!(token.allowance(accounts.alice, accounts.bob), 300_000);
        }

        #[ink::test]
        fn transfer_from_fails_insufficient_allowance() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.alice, 1_000_000).unwrap();
            token.approve(accounts.bob, 100_000).unwrap();

            set_caller(accounts.bob);
            let result = token.transfer_from(accounts.alice, accounts.charlie, 200_000);
            assert_eq!(result, Err(Error::InsufficientAllowance));
        }

        #[ink::test]
        fn add_remove_minter_works() {
            let accounts = create_accounts();
            let mut token = create_token();

            assert!(!token.is_minter(accounts.bob));

            token.add_minter(accounts.bob).unwrap();
            assert!(token.is_minter(accounts.bob));

            token.remove_minter(accounts.bob).unwrap();
            assert!(!token.is_minter(accounts.bob));
        }

        #[ink::test]
        fn pause_unpause_works() {
            let mut token = create_token();

            token.pause().unwrap();
            assert!(token.paused());

            token.unpause().unwrap();
            assert!(!token.paused());
        }

        #[ink::test]
        fn transfer_fails_when_paused() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.mint(accounts.alice, 1_000_000).unwrap();
            token.pause().unwrap();

            let result = token.transfer(accounts.bob, 100_000);
            assert_eq!(result, Err(Error::ContractPaused));
        }

        #[ink::test]
        fn mint_fails_when_paused() {
            let accounts = create_accounts();
            let mut token = create_token();

            token.pause().unwrap();

            let result = token.mint(accounts.bob, 100_000);
            assert_eq!(result, Err(Error::ContractPaused));
        }
    }
}
