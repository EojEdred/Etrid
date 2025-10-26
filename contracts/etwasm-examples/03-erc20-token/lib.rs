#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # ERC20 Token Contract
///
/// A complete implementation of the ERC20 token standard.
/// This demonstrates:
/// - ERC20 standard interface (transfer, approve, transferFrom)
/// - Balance tracking per account
/// - Allowance mechanism (delegated transfers)
/// - Minting (owner only)
/// - Burning (anyone can burn their own tokens)
/// - Events (Transfer, Approval)
/// - Comprehensive error handling
///
/// Perfect for learning how fungible tokens work on Ëtrid.

#[ink::contract]
mod erc20_token {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct Erc20Token {
        /// Total token supply
        total_supply: Balance,
        /// Mapping from account to token balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping from (owner, spender) to allowance amount
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
        /// Token decimals (usually 18)
        decimals: u8,
        /// Contract owner (can mint tokens)
        owner: AccountId,
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
        value: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
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
        /// Overflow would occur
        Overflow,
        /// Transfer amount is zero
        ZeroAmount,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20Token {
        /// Constructor: Creates a new ERC20 token
        ///
        /// # Arguments
        /// * `initial_supply` - Initial token supply (minted to caller)
        /// * `name` - Token name (e.g., "Ëtrid Token")
        /// * `symbol` - Token symbol (e.g., "ÉTR")
        /// * `decimals` - Decimal places (usually 18)
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(&caller, &initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });

            Self {
                total_supply: initial_supply,
                balances,
                allowances: Mapping::default(),
                name,
                symbol,
                decimals,
                owner: caller,
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

        /// Transfers tokens from caller to recipient
        ///
        /// # Arguments
        /// * `to` - Recipient address
        /// * `value` - Amount to transfer
        ///
        /// # Errors
        /// * `InsufficientBalance` - Caller doesn't have enough tokens
        /// * `TransferToZeroAddress` - Recipient is zero address
        /// * `ZeroAmount` - Transfer amount is zero
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
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
        /// * `ApproveToZeroAddress` - Spender is zero address
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();

            // Prevent approval to zero address
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
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            // Transfer tokens
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

        /// Mints new tokens (owner only)
        ///
        /// # Arguments
        /// * `to` - Recipient address
        /// * `value` - Amount to mint
        ///
        /// # Errors
        /// * `NotOwner` - Caller is not the owner
        /// * `Overflow` - Would exceed max supply
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            // Increase total supply
            self.total_supply = self.total_supply
                .checked_add(value)
                .ok_or(Error::Overflow)?;

            // Increase recipient balance
            let to_balance = self.balance_of(to);
            let new_balance = to_balance.checked_add(value).ok_or(Error::Overflow)?;
            self.balances.insert(&to, &new_balance);

            self.env().emit_event(Mint { to, value });

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
        /// * `InsufficientBalance` - Caller doesn't have enough tokens
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            let caller = self.env().caller();

            if value == 0 {
                return Err(Error::ZeroAmount);
            }

            let caller_balance = self.balance_of(caller);
            if caller_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Decrease total supply
            self.total_supply -= value;

            // Decrease caller balance
            let new_balance = caller_balance - value;
            self.balances.insert(&caller, &new_balance);

            self.env().emit_event(Burn {
                from: caller,
                value,
            });

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: None,
                value,
            });

            Ok(())
        }

        /// Transfers ownership to a new address (owner only)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
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

            // Prevent transfer to zero address
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

        fn create_contract(initial_supply: Balance) -> Erc20Token {
            Erc20Token::new(
                initial_supply,
                "Test Token".to_string(),
                "TST".to_string(),
                18,
            )
        }

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract(1000);
            assert_eq!(contract.total_supply(), 1000);
            assert_eq!(contract.name(), "Test Token");
            assert_eq!(contract.symbol(), "TST");
            assert_eq!(contract.decimals(), 18);
        }

        #[ink::test]
        fn balance_of_works() {
            let accounts = get_accounts();
            let contract = create_contract(1000);
            assert_eq!(contract.balance_of(accounts.alice), 1000);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            assert!(contract.transfer(accounts.bob, 100).is_ok());
            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_fails_insufficient_balance() {
            let accounts = get_accounts();
            let mut contract = create_contract(100);

            let result = contract.transfer(accounts.bob, 200);
            assert_eq!(result, Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn transfer_fails_to_zero_address() {
            let mut contract = create_contract(1000);
            let zero_address = AccountId::from([0x0; 32]);

            let result = contract.transfer(zero_address, 100);
            assert_eq!(result, Err(Error::TransferToZeroAddress));
        }

        #[ink::test]
        fn approve_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            assert!(contract.approve(accounts.bob, 200).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 200);
        }

        #[ink::test]
        fn transfer_from_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            // Alice approves Bob to spend 200
            assert!(contract.approve(accounts.bob, 200).is_ok());

            // Bob transfers 100 from Alice to Charlie
            set_caller(accounts.bob);
            assert!(contract.transfer_from(accounts.alice, accounts.charlie, 100).is_ok());

            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.charlie), 100);
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_from_fails_insufficient_allowance() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            contract.approve(accounts.bob, 50).unwrap();

            set_caller(accounts.bob);
            let result = contract.transfer_from(accounts.alice, accounts.charlie, 100);
            assert_eq!(result, Err(Error::InsufficientAllowance));
        }

        #[ink::test]
        fn increase_allowance_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            contract.approve(accounts.bob, 100).unwrap();
            assert!(contract.increase_allowance(accounts.bob, 50).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 150);
        }

        #[ink::test]
        fn decrease_allowance_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            contract.approve(accounts.bob, 100).unwrap();
            assert!(contract.decrease_allowance(accounts.bob, 30).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 70);
        }

        #[ink::test]
        fn mint_works_for_owner() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            assert!(contract.mint(accounts.bob, 500).is_ok());
            assert_eq!(contract.total_supply(), 1500);
            assert_eq!(contract.balance_of(accounts.bob), 500);
        }

        #[ink::test]
        fn mint_fails_for_non_owner() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            set_caller(accounts.bob);
            let result = contract.mint(accounts.bob, 500);
            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn burn_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            assert!(contract.burn(300).is_ok());
            assert_eq!(contract.total_supply(), 700);
            assert_eq!(contract.balance_of(accounts.alice), 700);
        }

        #[ink::test]
        fn burn_fails_insufficient_balance() {
            let mut contract = create_contract(100);

            let result = contract.burn(200);
            assert_eq!(result, Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            assert!(contract.transfer_ownership(accounts.bob).is_ok());
            assert_eq!(contract.owner(), accounts.bob);
        }

        #[ink::test]
        fn complex_scenario() {
            let accounts = get_accounts();
            let mut contract = create_contract(1000);

            // Alice transfers 200 to Bob
            contract.transfer(accounts.bob, 200).unwrap();

            // Alice approves Charlie to spend 100
            contract.approve(accounts.charlie, 100).unwrap();

            // Charlie transfers 50 from Alice to Bob
            set_caller(accounts.charlie);
            contract.transfer_from(accounts.alice, accounts.bob, 50).unwrap();

            // Check final balances
            assert_eq!(contract.balance_of(accounts.alice), 750);
            assert_eq!(contract.balance_of(accounts.bob), 250);
            assert_eq!(contract.allowance(accounts.alice, accounts.charlie), 50);
        }
    }

    /// Integration tests (E2E)
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = Erc20TokenRef::new(
                1000,
                "Test".to_string(),
                "TST".to_string(),
                18,
            );
            let contract_account_id = client
                .instantiate("erc20_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Transfer
            let transfer = build_message::<Erc20TokenRef>(contract_account_id.clone())
                .call(|token| token.transfer(ink_e2e::account_id(ink_e2e::AccountKeyring::Bob), 100));
            client.call(&ink_e2e::alice(), transfer, 0, None).await?;

            // Check balance
            let balance = build_message::<Erc20TokenRef>(contract_account_id.clone())
                .call(|token| token.balance_of(ink_e2e::account_id(ink_e2e::AccountKeyring::Bob)));
            let result = client.call_dry_run(&ink_e2e::alice(), &balance, 0, None).await;

            assert_eq!(result.return_value(), 100);

            Ok(())
        }
    }
}
