#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # EDSC Token (Etrid Dollar Stablecoin)
///
/// ERC20-like stablecoin with restricted minting.
/// Only the authorized minting engine can mint new EDSC.
/// Initial supply: 100M EDSC
///
/// Features:
/// - Standard ERC20 interface (transfer, approve, transferFrom)
/// - Restricted minting (only minting engine)
/// - Public burning (anyone can burn their own tokens)
/// - Access control for minting operations

#[ink::contract]
mod edsc_token {
    use ink::storage::Mapping;

    /// Initial supply: 100M EDSC (with 18 decimals)
    const INITIAL_SUPPLY: Balance = 100_000_000_000_000_000_000_000_000;

    /// The contract's storage
    #[ink(storage)]
    pub struct EdscToken {
        /// Total token supply
        total_supply: Balance,
        /// Mapping from account to token balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping from (owner, spender) to allowance amount
        allowances: Mapping<(AccountId, AccountId), Balance>,
        /// Contract owner (can set minting engine)
        owner: AccountId,
        /// Authorized minting engine contract
        minting_engine: Option<AccountId>,
        /// Paused state for emergency
        paused: bool,
        /// Reentrancy guard
        locked: bool,
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
        total_supply: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
        total_supply: Balance,
    }

    #[ink(event)]
    pub struct MintingEngineSet {
        #[ink(topic)]
        old_engine: Option<AccountId>,
        #[ink(topic)]
        new_engine: AccountId,
    }

    #[ink(event)]
    pub struct Paused {
        by: AccountId,
    }

    #[ink(event)]
    pub struct Unpaused {
        by: AccountId,
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
        /// Caller is not the minting engine
        NotMintingEngine,
        /// Minting engine not set
        MintingEngineNotSet,
        /// Overflow would occur
        Overflow,
        /// Transfer amount is zero
        ZeroAmount,
        /// Contract is paused
        ContractPaused,
        /// Reentrancy detected
        ReentrancyDetected,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl EdscToken {
        /// Constructor: Creates EDSC token with 100M initial supply
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(&caller, &INITIAL_SUPPLY);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: INITIAL_SUPPLY,
            });

            Self {
                total_supply: INITIAL_SUPPLY,
                balances,
                allowances: Mapping::default(),
                owner: caller,
                minting_engine: None,
                paused: false,
                locked: false,
            }
        }

        /// Returns the token name
        #[ink(message)]
        pub fn name(&self) -> String {
            "Etrid Dollar Stablecoin".to_string()
        }

        /// Returns the token symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            "EDSC".to_string()
        }

        /// Returns the number of decimals
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            18
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

        /// Returns the allowance (owner -> spender)
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }

        /// Returns the contract owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Returns the minting engine address
        #[ink(message)]
        pub fn minting_engine(&self) -> Option<AccountId> {
            self.minting_engine
        }

        /// Returns whether the contract is paused
        #[ink(message)]
        pub fn is_paused(&self) -> bool {
            self.paused
        }

        /// Transfers tokens from caller to recipient
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let from = self.env().caller();
            let result = self.transfer_from_to(&from, &to, value);

            self.locked = false;
            result
        }

        /// Approves spender to spend tokens on behalf of caller
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            self.ensure_not_paused()?;
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
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            self.ensure_not_paused()?;
            self.ensure_not_locked()?;
            self.locked = true;

            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                self.locked = false;
                return Err(Error::InsufficientAllowance);
            }

            // Transfer tokens
            self.transfer_from_to(&from, &to, value)?;

            // Decrease allowance
            let new_allowance = allowance - value;
            self.allowances.insert(&(from, caller), &new_allowance);

            self.locked = false;
            Ok(())
        }

        /// Mints new tokens (only minting engine)
        ///
        /// CRITICAL: Only callable by the authorized minting engine
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.ensure_not_paused()?;
            let caller = self.env().caller();

            // Verify caller is the minting engine
            match self.minting_engine {
                Some(engine) if caller == engine => {},
                Some(_) => return Err(Error::NotMintingEngine),
                None => return Err(Error::MintingEngineNotSet),
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

            self.env().emit_event(Mint {
                to,
                value,
                total_supply: self.total_supply,
            });

            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value,
            });

            Ok(())
        }

        /// Burns tokens from caller's balance
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {
            self.ensure_not_paused()?;
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
                total_supply: self.total_supply,
            });

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: None,
                value,
            });

            Ok(())
        }

        /// Sets the authorized minting engine (owner only)
        #[ink(message)]
        pub fn set_minting_engine(&mut self, engine: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let old_engine = self.minting_engine;
            self.minting_engine = Some(engine);

            self.env().emit_event(MintingEngineSet {
                old_engine,
                new_engine: engine,
            });

            Ok(())
        }

        /// Pauses the contract (owner only)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.paused = true;

            self.env().emit_event(Paused { by: caller });

            Ok(())
        }

        /// Unpauses the contract (owner only)
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.paused = false;

            self.env().emit_event(Unpaused { by: caller });

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

        /// Ensures contract is not paused
        fn ensure_not_paused(&self) -> Result<()> {
            if self.paused {
                Err(Error::ContractPaused)
            } else {
                Ok(())
            }
        }

        /// Ensures no reentrancy
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

        #[ink::test]
        fn new_works() {
            let contract = EdscToken::new();
            assert_eq!(contract.total_supply(), INITIAL_SUPPLY);
            assert_eq!(contract.name(), "Etrid Dollar Stablecoin");
            assert_eq!(contract.symbol(), "EDSC");
            assert_eq!(contract.decimals(), 18);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = get_accounts();
            let mut contract = EdscToken::new();

            assert!(contract.transfer(accounts.bob, 1000).is_ok());
            assert_eq!(contract.balance_of(accounts.alice), INITIAL_SUPPLY - 1000);
            assert_eq!(contract.balance_of(accounts.bob), 1000);
        }

        #[ink::test]
        fn mint_fails_without_engine() {
            let accounts = get_accounts();
            let mut contract = EdscToken::new();

            let result = contract.mint(accounts.bob, 1000);
            assert_eq!(result, Err(Error::MintingEngineNotSet));
        }

        #[ink::test]
        fn mint_works_with_engine() {
            let accounts = get_accounts();
            let mut contract = EdscToken::new();

            // Set minting engine
            contract.set_minting_engine(accounts.bob).unwrap();

            // Try to mint from engine
            set_caller(accounts.bob);
            assert!(contract.mint(accounts.charlie, 1000).is_ok());
            assert_eq!(contract.balance_of(accounts.charlie), 1000);
            assert_eq!(contract.total_supply(), INITIAL_SUPPLY + 1000);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = get_accounts();
            let mut contract = EdscToken::new();

            assert!(contract.burn(1000).is_ok());
            assert_eq!(contract.total_supply(), INITIAL_SUPPLY - 1000);
            assert_eq!(contract.balance_of(accounts.alice), INITIAL_SUPPLY - 1000);
        }

        #[ink::test]
        fn pause_works() {
            let accounts = get_accounts();
            let mut contract = EdscToken::new();

            contract.pause().unwrap();
            assert_eq!(contract.is_paused(), true);

            let result = contract.transfer(accounts.bob, 1000);
            assert_eq!(result, Err(Error::ContractPaused));
        }
    }
}
