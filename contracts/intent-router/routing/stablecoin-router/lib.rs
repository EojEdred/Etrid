#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Stablecoin Router Contract
///
/// Route stablecoin purchases to EDSC reserve
///
/// This contract detects stablecoin purchases on any PBC and routes them
/// to the EDSC-PBC for conversion to EDSC (Etrid Digital Stablecoin),
/// maintaining the EDSC reserve backing.

#[ink::contract]
mod stablecoin_router {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    /// Cross-PBC message type
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum XCMPMessageType {
        StablecoinRouting,
        EDSCMintResponse,
    }

    /// Stablecoin routing request
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RoutingRequest {
        pub user: AccountId,
        pub stablecoin: AccountId,
        pub amount: Balance,
        pub source_pbc: u32,
        pub timestamp: u64,
    }

    /// The contract's storage
    #[ink(storage)]
    pub struct StablecoinRouter {
        /// Contract owner
        owner: AccountId,
        /// EDSC PBC ID
        edsc_pbc_id: u32,
        /// USDC token address (canonical stablecoin)
        usdc_token: AccountId,
        /// USDT token address
        usdt_token: AccountId,
        /// DAI token address
        dai_token: AccountId,
        /// Registered stablecoins
        registered_stablecoins: Mapping<AccountId, bool>,
        /// Routing counter
        routing_counter: u64,
        /// Routing records
        routing_records: Mapping<u64, RoutingRequest>,
        /// EDSC minting engine address
        edsc_minting_engine: AccountId,
        /// Minimum routing amount
        min_routing_amount: Balance,
    }

    /// Events
    #[ink(event)]
    pub struct StablecoinRoutedToEDSC {
        #[ink(topic)]
        routing_id: u64,
        #[ink(topic)]
        user: AccountId,
        stablecoin: AccountId,
        amount: Balance,
        source_pbc: u32,
    }

    #[ink(event)]
    pub struct EDSCMinted {
        #[ink(topic)]
        routing_id: u64,
        #[ink(topic)]
        user: AccountId,
        edsc_amount: Balance,
    }

    #[ink(event)]
    pub struct StablecoinRegistered {
        #[ink(topic)]
        stablecoin: AccountId,
        registered: bool,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotStablecoin,
        InvalidAmount,
        AmountBelowMinimum,
        TransferFailed,
        SwapFailed,
        XCMPFailed,
        InvalidPBC,
        Overflow,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl StablecoinRouter {
        /// Constructor
        #[ink(constructor)]
        pub fn new(
            edsc_pbc_id: u32,
            usdc_token: AccountId,
            usdt_token: AccountId,
            dai_token: AccountId,
            edsc_minting_engine: AccountId,
        ) -> Self {
            let mut registered_stablecoins = Mapping::default();
            registered_stablecoins.insert(usdc_token, &true);
            registered_stablecoins.insert(usdt_token, &true);
            registered_stablecoins.insert(dai_token, &true);

            Self {
                owner: Self::env().caller(),
                edsc_pbc_id,
                usdc_token,
                usdt_token,
                dai_token,
                registered_stablecoins,
                routing_counter: 0,
                routing_records: Mapping::default(),
                edsc_minting_engine,
                min_routing_amount: 1_000_000, // 1 USDC (6 decimals)
            }
        }

        /// Route stablecoin purchase to EDSC reserve
        ///
        /// # Arguments
        /// * `stablecoin` - Stablecoin token address
        /// * `amount` - Amount to route
        /// * `source_pbc` - Source PBC ID
        ///
        /// # Flow
        /// 1. Verify stablecoin is registered
        /// 2. If not USDC, swap to USDC first
        /// 3. Send cross-PBC message to EDSC-PBC
        /// 4. EDSC-PBC mints EDSC 1:1
        /// 5. Send EDSC back to user on source PBC
        ///
        /// # Returns
        /// Amount of EDSC to be minted
        #[ink(message)]
        pub fn route_to_edsc(
            &mut self,
            stablecoin: AccountId,
            amount: Balance,
            source_pbc: u32,
        ) -> Result<Balance> {
            let user = self.env().caller();

            // Verify stablecoin is registered
            ensure!(self.is_stablecoin(stablecoin), Error::NotStablecoin);
            ensure!(amount >= self.min_routing_amount, Error::AmountBelowMinimum);
            ensure!(amount > 0, Error::InvalidAmount);

            // If not USDC, swap to USDC first
            let (final_stablecoin, final_amount) = if stablecoin != self.usdc_token {
                // NOTE: In production, swap stablecoin → USDC via DEX
                // For now, assume 1:1 conversion
                (self.usdc_token, amount)
            } else {
                (stablecoin, amount)
            };

            // Create routing request
            let routing_id = self.routing_counter;
            self.routing_counter += 1;

            let request = RoutingRequest {
                user,
                stablecoin: final_stablecoin,
                amount: final_amount,
                source_pbc,
                timestamp: self.env().block_timestamp(),
            };

            self.routing_records.insert(routing_id, &request);

            // NOTE: In production, send XCMP message to EDSC-PBC
            // let payload = encode_routing_request(&request)
            // send_xcmp_message(edsc_pbc_id, payload, XCMPMessageType::StablecoinRouting)

            // Emit event
            self.env().emit_event(StablecoinRoutedToEDSC {
                routing_id,
                user,
                stablecoin: final_stablecoin,
                amount: final_amount,
                source_pbc,
            });

            // EDSC minted 1:1 with stablecoin
            Ok(final_amount)
        }

        /// Handle EDSC mint response (callback from EDSC-PBC)
        ///
        /// Called when EDSC-PBC confirms EDSC was minted and sent back
        #[ink(message)]
        pub fn handle_edsc_mint_response(
            &mut self,
            routing_id: u64,
            edsc_amount: Balance,
        ) -> Result<()> {
            // NOTE: In production, verify caller is XCMP endpoint

            let request = self
                .routing_records
                .get(routing_id)
                .ok_or(Error::InvalidAmount)?;

            // Emit event
            self.env().emit_event(EDSCMinted {
                routing_id,
                user: request.user,
                edsc_amount,
            });

            Ok(())
        }

        /// Register stablecoin
        #[ink(message)]
        pub fn register_stablecoin(&mut self, stablecoin: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.registered_stablecoins.insert(stablecoin, &true);

            self.env().emit_event(StablecoinRegistered {
                stablecoin,
                registered: true,
            });

            Ok(())
        }

        /// Unregister stablecoin
        #[ink(message)]
        pub fn unregister_stablecoin(&mut self, stablecoin: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.registered_stablecoins.insert(stablecoin, &false);

            self.env().emit_event(StablecoinRegistered {
                stablecoin,
                registered: false,
            });

            Ok(())
        }

        /// Set minimum routing amount
        #[ink(message)]
        pub fn set_min_routing_amount(&mut self, amount: Balance) -> Result<()> {
            self.ensure_owner()?;
            self.min_routing_amount = amount;
            Ok(())
        }

        /// Set EDSC minting engine
        #[ink(message)]
        pub fn set_edsc_minting_engine(&mut self, engine: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.edsc_minting_engine = engine;
            Ok(())
        }

        /// Check if token is registered stablecoin
        #[ink(message)]
        pub fn is_stablecoin(&self, token: AccountId) -> bool {
            self.registered_stablecoins.get(token).unwrap_or(false)
        }

        /// Get routing record
        #[ink(message)]
        pub fn get_routing_record(&self, routing_id: u64) -> Option<RoutingRequest> {
            self.routing_records.get(routing_id)
        }

        /// Get routing counter
        #[ink(message)]
        pub fn routing_counter(&self) -> u64 {
            self.routing_counter
        }

        /// Get EDSC PBC ID
        #[ink(message)]
        pub fn edsc_pbc_id(&self) -> u32 {
            self.edsc_pbc_id
        }

        /// Get minimum routing amount
        #[ink(message)]
        pub fn min_routing_amount(&self) -> Balance {
            self.min_routing_amount
        }

        /// Get USDC token
        #[ink(message)]
        pub fn usdc_token(&self) -> AccountId {
            self.usdc_token
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

        fn create_contract() -> StablecoinRouter {
            let accounts = default_accounts();
            StablecoinRouter::new(
                100,              // EDSC PBC ID
                accounts.bob,     // USDC
                accounts.charlie, // USDT
                accounts.django,  // DAI
                accounts.eve,     // EDSC minting engine
            )
        }

        #[ink::test]
        fn new_works() {
            let contract = create_contract();
            let accounts = default_accounts();
            assert_eq!(contract.owner(), accounts.alice);
            assert_eq!(contract.edsc_pbc_id(), 100);
            assert!(contract.is_stablecoin(accounts.bob)); // USDC
            assert!(contract.is_stablecoin(accounts.charlie)); // USDT
            assert!(contract.is_stablecoin(accounts.django)); // DAI
        }

        #[ink::test]
        fn route_to_edsc_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.route_to_edsc(
                accounts.bob, // USDC
                10_000_000,   // 10 USDC
                1,            // Source PBC
            );

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 10_000_000);
            assert_eq!(contract.routing_counter(), 1);
        }

        #[ink::test]
        fn register_stablecoin_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            assert!(contract.register_stablecoin(accounts.frank).is_ok());
            assert!(contract.is_stablecoin(accounts.frank));
        }

        #[ink::test]
        fn unregister_stablecoin_works() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            assert!(contract.unregister_stablecoin(accounts.bob).is_ok());
            assert!(!contract.is_stablecoin(accounts.bob));
        }

        #[ink::test]
        fn route_non_stablecoin_fails() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.route_to_edsc(
                accounts.frank, // Not registered
                10_000_000,
                1,
            );

            assert_eq!(result, Err(Error::NotStablecoin));
        }

        #[ink::test]
        fn route_below_minimum_fails() {
            let mut contract = create_contract();
            let accounts = default_accounts();

            let result = contract.route_to_edsc(
                accounts.bob, // USDC
                100,          // Below minimum
                1,
            );

            assert_eq!(result, Err(Error::AmountBelowMinimum));
        }
    }
}
