#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Hello World Contract
///
/// This is the simplest possible Ëtrid smart contract.
/// It demonstrates:
/// - Basic contract structure
/// - Constructor
/// - Storage (single String)
/// - Getter function
/// - Setter function (mutable)
/// - Events
///
/// Perfect starting point for learning Ëtrid smart contract development.

#[ink::contract]
mod hello_world {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct HelloWorld {
        /// The greeting message
        message: String,
        /// Track how many times the message was changed
        change_count: u32,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct MessageChanged {
        #[ink(topic)]
        from: Option<AccountId>,
        old_message: String,
        new_message: String,
    }

    /// Errors that can occur
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Message is empty
        EmptyMessage,
        /// Message too long (max 256 chars)
        MessageTooLong,
    }

    /// Result type for contract calls
    pub type Result<T> = core::result::Result<T, Error>;

    impl HelloWorld {
        /// Constructor: Creates a new Hello World contract
        ///
        /// # Arguments
        /// * `init_message` - The initial greeting message
        ///
        /// # Example
        /// ```
        /// let contract = HelloWorld::new("Hello, Ëtrid!".to_string());
        /// ```
        #[ink(constructor)]
        pub fn new(init_message: String) -> Self {
            Self {
                message: init_message,
                change_count: 0,
            }
        }

        /// Constructor: Creates contract with default message
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new("Hello, World!".to_string())
        }

        /// Returns the current message
        ///
        /// This is a read-only function (doesn't modify state)
        #[ink(message)]
        pub fn get_message(&self) -> String {
            self.message.clone()
        }

        /// Returns how many times the message was changed
        #[ink(message)]
        pub fn get_change_count(&self) -> u32 {
            self.change_count
        }

        /// Sets a new message
        ///
        /// This modifies state and emits an event
        ///
        /// # Arguments
        /// * `new_message` - The new greeting message
        ///
        /// # Errors
        /// * `EmptyMessage` - If the new message is empty
        /// * `MessageTooLong` - If message exceeds 256 characters
        #[ink(message)]
        pub fn set_message(&mut self, new_message: String) -> Result<()> {
            // Validate input
            if new_message.is_empty() {
                return Err(Error::EmptyMessage);
            }
            if new_message.len() > 256 {
                return Err(Error::MessageTooLong);
            }

            // Store old message for event
            let old_message = self.message.clone();

            // Update state
            self.message = new_message.clone();
            self.change_count += 1;

            // Emit event
            self.env().emit_event(MessageChanged {
                from: Some(self.env().caller()),
                old_message,
                new_message,
            });

            Ok(())
        }

        /// Resets the message to default
        #[ink(message)]
        pub fn reset(&mut self) {
            let old_message = self.message.clone();
            self.message = "Hello, World!".to_string();
            self.change_count += 1;

            self.env().emit_event(MessageChanged {
                from: Some(self.env().caller()),
                old_message,
                new_message: self.message.clone(),
            });
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = HelloWorld::new("Test message".to_string());
            assert_eq!(contract.get_message(), "Test message");
            assert_eq!(contract.get_change_count(), 0);
        }

        #[ink::test]
        fn default_works() {
            let contract = HelloWorld::default();
            assert_eq!(contract.get_message(), "Hello, World!");
        }

        #[ink::test]
        fn set_message_works() {
            let mut contract = HelloWorld::default();
            assert!(contract.set_message("New message".to_string()).is_ok());
            assert_eq!(contract.get_message(), "New message");
            assert_eq!(contract.get_change_count(), 1);
        }

        #[ink::test]
        fn set_message_rejects_empty() {
            let mut contract = HelloWorld::default();
            let result = contract.set_message("".to_string());
            assert_eq!(result, Err(Error::EmptyMessage));
        }

        #[ink::test]
        fn set_message_rejects_too_long() {
            let mut contract = HelloWorld::default();
            let long_message = "a".repeat(300);
            let result = contract.set_message(long_message);
            assert_eq!(result, Err(Error::MessageTooLong));
        }

        #[ink::test]
        fn reset_works() {
            let mut contract = HelloWorld::new("Custom".to_string());
            contract.reset();
            assert_eq!(contract.get_message(), "Hello, World!");
            assert_eq!(contract.get_change_count(), 1);
        }

        #[ink::test]
        fn multiple_changes_increment_count() {
            let mut contract = HelloWorld::default();
            contract.set_message("First".to_string()).unwrap();
            contract.set_message("Second".to_string()).unwrap();
            contract.set_message("Third".to_string()).unwrap();
            assert_eq!(contract.get_change_count(), 3);
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
            // Deploy contract
            let constructor = HelloWorldRef::new("Test".to_string());
            let contract_account_id = client
                .instantiate("hello_world", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Call get_message
            let get_message = build_message::<HelloWorldRef>(contract_account_id.clone())
                .call(|contract| contract.get_message());
            let result = client
                .call_dry_run(&ink_e2e::alice(), &get_message, 0, None)
                .await;

            assert_eq!(result.return_value(), "Test");

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_set_message_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Deploy
            let constructor = HelloWorldRef::default();
            let contract_account_id = client
                .instantiate("hello_world", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Set message
            let set_message = build_message::<HelloWorldRef>(contract_account_id.clone())
                .call(|contract| contract.set_message("New message".to_string()));
            let _result = client
                .call(&ink_e2e::bob(), set_message, 0, None)
                .await
                .expect("set_message failed");

            // Get message
            let get_message = build_message::<HelloWorldRef>(contract_account_id.clone())
                .call(|contract| contract.get_message());
            let result = client
                .call_dry_run(&ink_e2e::bob(), &get_message, 0, None)
                .await;

            assert_eq!(result.return_value(), "New message");

            Ok(())
        }
    }
}
