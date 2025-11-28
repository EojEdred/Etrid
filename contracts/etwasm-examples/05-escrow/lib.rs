#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod escrow {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    /// Escrow state
    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum EscrowState {
        AwaitingPayment,  // Created, waiting for buyer to deposit
        AwaitingDelivery, // Buyer paid, waiting for delivery confirmation
        Complete,         // Buyer confirmed delivery, funds released
        Refunded,         // Funds returned to buyer
        Disputed,         // Dispute raised, arbiter needed
    }

    /// Escrow details
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct EscrowDetails {
        pub id: u32,
        pub buyer: AccountId,
        pub seller: AccountId,
        pub arbiter: AccountId,
        pub amount: Balance,
        pub description: String,
        pub state: EscrowState,
        pub created_at: Timestamp,
        pub timeout_at: Timestamp,
    }

    /// Custom errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotBuyer,
        NotSeller,
        NotArbiter,
        NotAuthorized,
        InvalidState,
        InvalidAmount,
        InsufficientFunds,
        EscrowNotFound,
        AlreadyPaid,
        NotPaid,
        TimeoutNotReached,
        AlreadyComplete,
        EmptyDescription,
        TransferFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Escrow contract storage
    #[ink(storage)]
    pub struct Escrow {
        /// All escrows (id => EscrowDetails)
        escrows: Mapping<u32, EscrowDetails>,
        /// Next escrow ID
        next_escrow_id: u32,
        /// Escrow balances (id => amount held)
        balances: Mapping<u32, Balance>,
    }

    /// Events
    #[ink(event)]
    pub struct EscrowCreated {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        seller: AccountId,
        arbiter: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct PaymentDeposited {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct DeliveryConfirmed {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        buyer: AccountId,
    }

    #[ink(event)]
    pub struct FundsReleased {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        seller: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct FundsRefunded {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        buyer: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct DisputeRaised {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        raised_by: AccountId,
    }

    #[ink(event)]
    pub struct DisputeResolved {
        #[ink(topic)]
        escrow_id: u32,
        #[ink(topic)]
        arbiter: AccountId,
        release_to_seller: bool,
    }

    impl Escrow {
        /// Constructor - creates a new escrow contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                escrows: Mapping::default(),
                next_escrow_id: 0,
                balances: Mapping::default(),
            }
        }

        /// Create a new escrow agreement
        #[ink(message)]
        pub fn create_escrow(
            &mut self,
            seller: AccountId,
            arbiter: AccountId,
            amount: Balance,
            description: String,
            timeout_days: u32,
        ) -> Result<u32> {
            let buyer = self.env().caller();

            if amount == 0 {
                return Err(Error::InvalidAmount);
            }

            if description.is_empty() {
                return Err(Error::EmptyDescription);
            }

            let escrow_id = self.next_escrow_id;
            let now = self.env().block_timestamp();
            let timeout_at = now + (timeout_days as u64 * 24 * 60 * 60 * 1000);

            let escrow = EscrowDetails {
                id: escrow_id,
                buyer,
                seller,
                arbiter,
                amount,
                description,
                state: EscrowState::AwaitingPayment,
                created_at: now,
                timeout_at,
            };

            self.escrows.insert(&escrow_id, &escrow);
            self.next_escrow_id += 1;

            self.env().emit_event(EscrowCreated {
                escrow_id,
                buyer,
                seller,
                arbiter,
                amount,
            });

            Ok(escrow_id)
        }

        /// Get escrow details
        #[ink(message)]
        pub fn get_escrow(&self, escrow_id: u32) -> Option<EscrowDetails> {
            self.escrows.get(&escrow_id)
        }

        /// Get total escrow count
        #[ink(message)]
        pub fn escrow_count(&self) -> u32 {
            self.next_escrow_id
        }

        /// Buyer deposits payment into escrow
        #[ink(message, payable)]
        pub fn deposit(&mut self, escrow_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            // Only buyer can deposit
            if caller != escrow.buyer {
                return Err(Error::NotBuyer);
            }

            // Must be in AwaitingPayment state
            if escrow.state != EscrowState::AwaitingPayment {
                return Err(Error::InvalidState);
            }

            // Check deposited amount matches
            let deposited = self.env().transferred_value();
            if deposited != escrow.amount {
                return Err(Error::InvalidAmount);
            }

            // Update state and balance
            escrow.state = EscrowState::AwaitingDelivery;
            self.escrows.insert(&escrow_id, &escrow);
            self.balances.insert(&escrow_id, &deposited);

            self.env().emit_event(PaymentDeposited {
                escrow_id,
                buyer: caller,
                amount: deposited,
            });

            Ok(())
        }

        /// Buyer confirms delivery and releases funds to seller
        #[ink(message)]
        pub fn confirm_delivery(&mut self, escrow_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            // Only buyer can confirm delivery
            if caller != escrow.buyer {
                return Err(Error::NotBuyer);
            }

            // Must be in AwaitingDelivery state
            if escrow.state != EscrowState::AwaitingDelivery {
                return Err(Error::InvalidState);
            }

            // Get balance
            let balance = self.balances.get(&escrow_id).ok_or(Error::NotPaid)?;

            // Update state
            escrow.state = EscrowState::Complete;
            self.escrows.insert(&escrow_id, &escrow);

            // Release funds to seller
            self.balances.insert(&escrow_id, &0);
            if self.env().transfer(escrow.seller, balance).is_err() {
                return Err(Error::TransferFailed);
            }

            self.env().emit_event(DeliveryConfirmed {
                escrow_id,
                buyer: caller,
            });

            self.env().emit_event(FundsReleased {
                escrow_id,
                seller: escrow.seller,
                amount: balance,
            });

            Ok(())
        }

        /// Buyer requests refund (if timeout reached and no delivery)
        #[ink(message)]
        pub fn request_refund(&mut self, escrow_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            // Only buyer can request refund
            if caller != escrow.buyer {
                return Err(Error::NotBuyer);
            }

            // Must be in AwaitingDelivery state
            if escrow.state != EscrowState::AwaitingDelivery {
                return Err(Error::InvalidState);
            }

            // Check timeout
            let now = self.env().block_timestamp();
            if now <= escrow.timeout_at {
                return Err(Error::TimeoutNotReached);
            }

            // Get balance
            let balance = self.balances.get(&escrow_id).ok_or(Error::NotPaid)?;

            // Update state
            escrow.state = EscrowState::Refunded;
            self.escrows.insert(&escrow_id, &escrow);

            // Refund to buyer
            self.balances.insert(&escrow_id, &0);
            if self.env().transfer(escrow.buyer, balance).is_err() {
                return Err(Error::TransferFailed);
            }

            self.env().emit_event(FundsRefunded {
                escrow_id,
                buyer: caller,
                amount: balance,
            });

            Ok(())
        }

        /// Raise a dispute (buyer or seller)
        #[ink(message)]
        pub fn raise_dispute(&mut self, escrow_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            // Only buyer or seller can raise dispute
            if caller != escrow.buyer && caller != escrow.seller {
                return Err(Error::NotAuthorized);
            }

            // Must be in AwaitingDelivery state
            if escrow.state != EscrowState::AwaitingDelivery {
                return Err(Error::InvalidState);
            }

            // Update state
            escrow.state = EscrowState::Disputed;
            self.escrows.insert(&escrow_id, &escrow);

            self.env().emit_event(DisputeRaised {
                escrow_id,
                raised_by: caller,
            });

            Ok(())
        }

        /// Arbiter resolves dispute
        #[ink(message)]
        pub fn resolve_dispute(
            &mut self,
            escrow_id: u32,
            release_to_seller: bool,
        ) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(&escrow_id).ok_or(Error::EscrowNotFound)?;

            // Only arbiter can resolve
            if caller != escrow.arbiter {
                return Err(Error::NotArbiter);
            }

            // Must be in Disputed state
            if escrow.state != EscrowState::Disputed {
                return Err(Error::InvalidState);
            }

            // Get balance
            let balance = self.balances.get(&escrow_id).ok_or(Error::NotPaid)?;

            // Update state
            escrow.state = if release_to_seller {
                EscrowState::Complete
            } else {
                EscrowState::Refunded
            };
            self.escrows.insert(&escrow_id, &escrow);

            // Transfer funds
            self.balances.insert(&escrow_id, &0);
            let recipient = if release_to_seller {
                escrow.seller
            } else {
                escrow.buyer
            };

            if self.env().transfer(recipient, balance).is_err() {
                return Err(Error::TransferFailed);
            }

            self.env().emit_event(DisputeResolved {
                escrow_id,
                arbiter: caller,
                release_to_seller,
            });

            if release_to_seller {
                self.env().emit_event(FundsReleased {
                    escrow_id,
                    seller: escrow.seller,
                    amount: balance,
                });
            } else {
                self.env().emit_event(FundsRefunded {
                    escrow_id,
                    buyer: escrow.buyer,
                    amount: balance,
                });
            }

            Ok(())
        }

        /// Get escrow balance
        #[ink(message)]
        pub fn get_balance(&self, escrow_id: u32) -> Balance {
            self.balances.get(&escrow_id).unwrap_or(0)
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        fn set_balance(account: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account, balance);
        }

        #[ink::test]
        fn new_works() {
            let escrow = Escrow::new();
            assert_eq!(escrow.escrow_count(), 0);
        }

        #[ink::test]
        fn create_escrow_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(
                    accounts.bob,
                    accounts.charlie,
                    1000,
                    "Test escrow".into(),
                    7,
                )
                .unwrap();

            assert_eq!(escrow_id, 0);
            assert_eq!(escrow.escrow_count(), 1);

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.buyer, accounts.alice);
            assert_eq!(details.seller, accounts.bob);
            assert_eq!(details.arbiter, accounts.charlie);
            assert_eq!(details.amount, 1000);
            assert_eq!(details.state, EscrowState::AwaitingPayment);
        }

        #[ink::test]
        fn create_escrow_empty_description_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let result = escrow.create_escrow(accounts.bob, accounts.charlie, 1000, "".into(), 7);
            assert_eq!(result, Err(Error::EmptyDescription));
        }

        #[ink::test]
        fn create_escrow_zero_amount_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let result =
                escrow.create_escrow(accounts.bob, accounts.charlie, 0, "Test".into(), 7);
            assert_eq!(result, Err(Error::InvalidAmount));
        }

        #[ink::test]
        fn deposit_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create escrow
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            // Set balance and deposit
            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);

            assert!(escrow.deposit(escrow_id).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::AwaitingDelivery);
            assert_eq!(escrow.get_balance(escrow_id), 1000);
        }

        #[ink::test]
        fn deposit_wrong_amount_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(500);

            assert_eq!(escrow.deposit(escrow_id), Err(Error::InvalidAmount));
        }

        #[ink::test]
        fn deposit_not_buyer_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            // Bob tries to deposit (not buyer)
            set_caller(accounts.bob);
            set_balance(accounts.bob, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);

            assert_eq!(escrow.deposit(escrow_id), Err(Error::NotBuyer));
        }

        #[ink::test]
        fn confirm_delivery_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create and fund escrow
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Confirm delivery
            assert!(escrow.confirm_delivery(escrow_id).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::Complete);
            assert_eq!(escrow.get_balance(escrow_id), 0);
        }

        #[ink::test]
        fn confirm_delivery_not_buyer_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Bob tries to confirm (not buyer)
            set_caller(accounts.bob);
            assert_eq!(escrow.confirm_delivery(escrow_id), Err(Error::NotBuyer));
        }

        #[ink::test]
        fn raise_dispute_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create and fund
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Raise dispute
            assert!(escrow.raise_dispute(escrow_id).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::Disputed);
        }

        #[ink::test]
        fn raise_dispute_by_seller_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Seller raises dispute
            set_caller(accounts.bob);
            assert!(escrow.raise_dispute(escrow_id).is_ok());
        }

        #[ink::test]
        fn raise_dispute_not_authorized_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Random person tries to raise dispute
            set_caller(accounts.django);
            assert_eq!(escrow.raise_dispute(escrow_id), Err(Error::NotAuthorized));
        }

        #[ink::test]
        fn resolve_dispute_to_seller_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create, fund, and dispute
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();
            escrow.raise_dispute(escrow_id).unwrap();

            // Arbiter resolves in favor of seller
            set_caller(accounts.charlie);
            assert!(escrow.resolve_dispute(escrow_id, true).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::Complete);
            assert_eq!(escrow.get_balance(escrow_id), 0);
        }

        #[ink::test]
        fn resolve_dispute_to_buyer_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create, fund, and dispute
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();
            escrow.raise_dispute(escrow_id).unwrap();

            // Arbiter resolves in favor of buyer (refund)
            set_caller(accounts.charlie);
            assert!(escrow.resolve_dispute(escrow_id, false).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::Refunded);
            assert_eq!(escrow.get_balance(escrow_id), 0);
        }

        #[ink::test]
        fn resolve_dispute_not_arbiter_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();
            escrow.raise_dispute(escrow_id).unwrap();

            // Alice (buyer) tries to resolve
            assert_eq!(
                escrow.resolve_dispute(escrow_id, true),
                Err(Error::NotArbiter)
            );
        }

        #[ink::test]
        fn request_refund_works() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            // Create and fund
            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Advance time past timeout
            ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(
                8 * 24 * 60 * 60 * 1000,
            );

            // Request refund
            assert!(escrow.request_refund(escrow_id).is_ok());

            let details = escrow.get_escrow(escrow_id).unwrap();
            assert_eq!(details.state, EscrowState::Refunded);
        }

        #[ink::test]
        fn request_refund_before_timeout_fails() {
            let accounts = get_accounts();
            let mut escrow = Escrow::new();

            let escrow_id = escrow
                .create_escrow(accounts.bob, accounts.charlie, 1000, "Test".into(), 7)
                .unwrap();

            set_balance(accounts.alice, 2000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            escrow.deposit(escrow_id).unwrap();

            // Try to refund before timeout
            assert_eq!(
                escrow.request_refund(escrow_id),
                Err(Error::TimeoutNotReached)
            );
        }
    }

    /// E2E tests
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_escrow_lifecycle(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Deploy contract
            let constructor = EscrowRef::new();
            let contract_account_id = client
                .instantiate("escrow", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Create escrow
            let create = build_message::<EscrowRef>(contract_account_id.clone()).call(|escrow| {
                escrow.create_escrow(
                    ink_e2e::account_id(ink_e2e::AccountKeyring::Bob),
                    ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie),
                    1000,
                    "Test escrow".into(),
                    7,
                )
            });

            let create_result = client
                .call(&ink_e2e::alice(), create, 0, None)
                .await
                .expect("create_escrow failed");

            assert!(create_result.return_value().is_ok());

            // Deposit funds
            let deposit = build_message::<EscrowRef>(contract_account_id.clone())
                .call(|escrow| escrow.deposit(0));

            let deposit_result = client
                .call(&ink_e2e::alice(), deposit, 1000, None)
                .await
                .expect("deposit failed");

            assert!(deposit_result.return_value().is_ok());

            Ok(())
        }
    }
}
