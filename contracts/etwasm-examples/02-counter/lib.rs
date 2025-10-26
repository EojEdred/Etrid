#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Counter Contract
///
/// A more advanced example demonstrating:
/// - Mapping storage (per-user counters)
/// - Access control (owner-only functions)
/// - Multiple counters (global + per-user)
/// - Batch operations
/// - Events with indexed topics
/// - Comprehensive error handling
///
/// This builds on the Hello World example with real-world patterns.

#[ink::contract]
mod counter {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct Counter {
        /// Global counter (all users combined)
        global_count: u64,
        /// Per-user counters
        user_counts: Mapping<AccountId, u64>,
        /// Contract owner (can reset global counter)
        owner: AccountId,
        /// Total number of unique users
        total_users: u32,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct Incremented {
        #[ink(topic)]
        by: AccountId,
        amount: u64,
        new_value: u64,
    }

    #[ink(event)]
    pub struct Decremented {
        #[ink(topic)]
        by: AccountId,
        amount: u64,
        new_value: u64,
    }

    #[ink(event)]
    pub struct GlobalReset {
        #[ink(topic)]
        by: AccountId,
        old_value: u64,
    }

    #[ink(event)]
    pub struct NewUser {
        #[ink(topic)]
        user: AccountId,
        user_number: u32,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Underflow: cannot decrement below zero
        Underflow,
        /// Overflow: counter would exceed u64::MAX
        Overflow,
        /// Invalid amount (e.g., zero)
        InvalidAmount,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl Counter {
        /// Constructor: Creates a new Counter contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                global_count: 0,
                user_counts: Mapping::default(),
                owner: Self::env().caller(),
                total_users: 0,
            }
        }

        /// Constructor: Creates counter with initial value
        #[ink(constructor)]
        pub fn new_with_value(init_value: u64) -> Self {
            Self {
                global_count: init_value,
                user_counts: Mapping::default(),
                owner: Self::env().caller(),
                total_users: 0,
            }
        }

        /// Returns the global counter value
        #[ink(message)]
        pub fn get_global(&self) -> u64 {
            self.global_count
        }

        /// Returns the caller's counter value
        #[ink(message)]
        pub fn get_mine(&self) -> u64 {
            let caller = self.env().caller();
            self.user_counts.get(&caller).unwrap_or(0)
        }

        /// Returns a specific user's counter value
        #[ink(message)]
        pub fn get_user_count(&self, user: AccountId) -> u64 {
            self.user_counts.get(&user).unwrap_or(0)
        }

        /// Returns the total number of unique users
        #[ink(message)]
        pub fn get_total_users(&self) -> u32 {
            self.total_users
        }

        /// Returns the contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Increments the caller's counter by 1
        #[ink(message)]
        pub fn increment(&mut self) -> Result<()> {
            self.increment_by(1)
        }

        /// Increments the caller's counter by a specific amount
        #[ink(message)]
        pub fn increment_by(&mut self, amount: u64) -> Result<()> {
            if amount == 0 {
                return Err(Error::InvalidAmount);
            }

            let caller = self.env().caller();
            let current = self.user_counts.get(&caller).unwrap_or(0);

            // Check for overflow
            let new_value = current.checked_add(amount)
                .ok_or(Error::Overflow)?;

            // Track new users
            if current == 0 {
                self.total_users += 1;
                self.env().emit_event(NewUser {
                    user: caller,
                    user_number: self.total_users,
                });
            }

            // Update storage
            self.user_counts.insert(&caller, &new_value);
            self.global_count = self.global_count.checked_add(amount)
                .ok_or(Error::Overflow)?;

            // Emit event
            self.env().emit_event(Incremented {
                by: caller,
                amount,
                new_value,
            });

            Ok(())
        }

        /// Decrements the caller's counter by 1
        #[ink(message)]
        pub fn decrement(&mut self) -> Result<()> {
            self.decrement_by(1)
        }

        /// Decrements the caller's counter by a specific amount
        #[ink(message)]
        pub fn decrement_by(&mut self, amount: u64) -> Result<()> {
            if amount == 0 {
                return Err(Error::InvalidAmount);
            }

            let caller = self.env().caller();
            let current = self.user_counts.get(&caller).unwrap_or(0);

            // Check for underflow
            let new_value = current.checked_sub(amount)
                .ok_or(Error::Underflow)?;

            // Update storage
            self.user_counts.insert(&caller, &new_value);
            self.global_count = self.global_count.checked_sub(amount)
                .ok_or(Error::Underflow)?;

            // Emit event
            self.env().emit_event(Decremented {
                by: caller,
                amount,
                new_value,
            });

            Ok(())
        }

        /// Resets the caller's counter to zero
        #[ink(message)]
        pub fn reset_mine(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let current = self.user_counts.get(&caller).unwrap_or(0);

            if current > 0 {
                self.user_counts.insert(&caller, &0);
                self.global_count = self.global_count.checked_sub(current)
                    .ok_or(Error::Underflow)?;

                self.env().emit_event(Decremented {
                    by: caller,
                    amount: current,
                    new_value: 0,
                });
            }

            Ok(())
        }

        /// Resets the global counter to zero (owner only)
        #[ink(message)]
        pub fn reset_global(&mut self) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            let old_value = self.global_count;
            self.global_count = 0;

            self.env().emit_event(GlobalReset {
                by: caller,
                old_value,
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

        /// Batch increment: increases counter multiple times
        /// Useful for demonstrating gas optimization
        #[ink(message)]
        pub fn batch_increment(&mut self, count: u32) -> Result<()> {
            for _ in 0..count {
                self.increment()?;
            }
            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = Counter::new();
            assert_eq!(contract.get_global(), 0);
            assert_eq!(contract.get_mine(), 0);
            assert_eq!(contract.get_total_users(), 0);
        }

        #[ink::test]
        fn new_with_value_works() {
            let contract = Counter::new_with_value(42);
            assert_eq!(contract.get_global(), 42);
        }

        #[ink::test]
        fn increment_works() {
            let mut contract = Counter::new();
            assert!(contract.increment().is_ok());
            assert_eq!(contract.get_mine(), 1);
            assert_eq!(contract.get_global(), 1);
        }

        #[ink::test]
        fn increment_by_works() {
            let mut contract = Counter::new();
            assert!(contract.increment_by(10).is_ok());
            assert_eq!(contract.get_mine(), 10);
            assert_eq!(contract.get_global(), 10);
        }

        #[ink::test]
        fn increment_tracks_new_users() {
            let mut contract = Counter::new();
            assert!(contract.increment().is_ok());
            assert_eq!(contract.get_total_users(), 1);
        }

        #[ink::test]
        fn decrement_works() {
            let mut contract = Counter::new();
            contract.increment_by(5).unwrap();
            assert!(contract.decrement().is_ok());
            assert_eq!(contract.get_mine(), 4);
            assert_eq!(contract.get_global(), 4);
        }

        #[ink::test]
        fn decrement_by_works() {
            let mut contract = Counter::new();
            contract.increment_by(10).unwrap();
            assert!(contract.decrement_by(3).is_ok());
            assert_eq!(contract.get_mine(), 7);
        }

        #[ink::test]
        fn decrement_fails_on_underflow() {
            let mut contract = Counter::new();
            let result = contract.decrement();
            assert_eq!(result, Err(Error::Underflow));
        }

        #[ink::test]
        fn increment_by_zero_fails() {
            let mut contract = Counter::new();
            let result = contract.increment_by(0);
            assert_eq!(result, Err(Error::InvalidAmount));
        }

        #[ink::test]
        fn reset_mine_works() {
            let mut contract = Counter::new();
            contract.increment_by(10).unwrap();
            assert!(contract.reset_mine().is_ok());
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        fn reset_global_works_for_owner() {
            let mut contract = Counter::new();
            contract.increment_by(50).unwrap();
            assert!(contract.reset_global().is_ok());
            assert_eq!(contract.get_global(), 0);
        }

        #[ink::test]
        fn reset_global_fails_for_non_owner() {
            let mut contract = Counter::new();
            // Change caller to non-owner
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = contract.reset_global();
            assert_eq!(result, Err(Error::NotOwner));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = Counter::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(contract.transfer_ownership(accounts.bob).is_ok());
            assert_eq!(contract.get_owner(), accounts.bob);
        }

        #[ink::test]
        fn batch_increment_works() {
            let mut contract = Counter::new();
            assert!(contract.batch_increment(5).is_ok());
            assert_eq!(contract.get_mine(), 5);
            assert_eq!(contract.get_global(), 5);
        }

        #[ink::test]
        fn multiple_users_tracked_correctly() {
            let mut contract = Counter::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice increments
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            contract.increment_by(10).unwrap();

            // Bob increments
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            contract.increment_by(20).unwrap();

            // Check values
            assert_eq!(contract.get_user_count(accounts.alice), 10);
            assert_eq!(contract.get_user_count(accounts.bob), 20);
            assert_eq!(contract.get_global(), 30);
            assert_eq!(contract.get_total_users(), 2);
        }
    }

    /// Integration tests (E2E)
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_new_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CounterRef::new();
            let contract_account_id = client
                .instantiate("counter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get_global = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.get_global());
            let result = client.call_dry_run(&ink_e2e::alice(), &get_global, 0, None).await;

            assert_eq!(result.return_value(), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_increment_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CounterRef::new();
            let contract_account_id = client
                .instantiate("counter", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Increment
            let increment = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.increment());
            let _result = client
                .call(&ink_e2e::bob(), increment, 0, None)
                .await
                .expect("increment failed");

            // Get value
            let get_mine = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.get_mine());
            let result = client.call_dry_run(&ink_e2e::bob(), &get_mine, 0, None).await;

            assert_eq!(result.return_value(), 1);

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_multiple_users_work(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CounterRef::new();
            let contract_account_id = client
                .instantiate("counter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Alice increments by 10
            let increment_alice = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.increment_by(10));
            client.call(&ink_e2e::alice(), increment_alice, 0, None).await?;

            // Bob increments by 20
            let increment_bob = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.increment_by(20));
            client.call(&ink_e2e::bob(), increment_bob, 0, None).await?;

            // Check global
            let get_global = build_message::<CounterRef>(contract_account_id.clone())
                .call(|counter| counter.get_global());
            let result = client.call_dry_run(&ink_e2e::alice(), &get_global, 0, None).await;

            assert_eq!(result.return_value(), 30);

            Ok(())
        }
    }
}
