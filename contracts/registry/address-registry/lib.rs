#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// # Address Registry Contract
///
/// Central registry for all ĒTRID contract addresses.
/// This enables contracts to discover each other dynamically.
///
/// Used by: All contracts that need to call other contracts

#[ink::contract]
mod address_registry {
    use ink::storage::Mapping;

    /// The contract's storage
    #[ink(storage)]
    pub struct AddressRegistry {
        /// Contract owner (can update addresses)
        owner: AccountId,

        /// Emergency pause
        paused: bool,

        // ═══════════════════════════════════════════════════════════
        // WRAPPED TOKENS (11 currencies)
        // ═══════════════════════════════════════════════════════════
        wrapped_tokens: Mapping<String, AccountId>,

        // ═══════════════════════════════════════════════════════════
        // TIER 1 POOLS (11 reserve pools)
        // ═══════════════════════════════════════════════════════════
        tier1_pools: Mapping<String, AccountId>,

        // ═══════════════════════════════════════════════════════════
        // TIER 2 POOLS (11 trading pools)
        // ═══════════════════════════════════════════════════════════
        tier2_pools: Mapping<String, AccountId>,

        // ═══════════════════════════════════════════════════════════
        // INTENT ROUTER SYSTEM
        // ═══════════════════════════════════════════════════════════
        intent_router: Option<AccountId>,
        auto_swap_executor: Option<AccountId>,
        two_tier_bridge_router: Option<AccountId>,
        stablecoin_router: Option<AccountId>,

        // ═══════════════════════════════════════════════════════════
        // EDSC SYSTEM
        // ═══════════════════════════════════════════════════════════
        edsc_token: Option<AccountId>,
        edsc_reserve_vault: Option<AccountId>,
        edsc_minting_engine: Option<AccountId>,
        edsc_peg_stabilizer: Option<AccountId>,
        edsc_external_swap_router: Option<AccountId>,

        // ═══════════════════════════════════════════════════════════
        // UTILITY
        // ═══════════════════════════════════════════════════════════
        /// ETR native token (for reference)
        etr_token: Option<AccountId>,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct AddressRegistered {
        #[ink(topic)]
        category: String,
        #[ink(topic)]
        key: String,
        address: AccountId,
    }

    #[ink(event)]
    pub struct AddressUpdated {
        #[ink(topic)]
        category: String,
        #[ink(topic)]
        key: String,
        old_address: AccountId,
        new_address: AccountId,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Contract is paused
        ContractPaused,
        /// Address not found
        AddressNotFound,
        /// Invalid address (zero address)
        InvalidAddress,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl AddressRegistry {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                paused: false,
                wrapped_tokens: Mapping::default(),
                tier1_pools: Mapping::default(),
                tier2_pools: Mapping::default(),
                intent_router: None,
                auto_swap_executor: None,
                two_tier_bridge_router: None,
                stablecoin_router: None,
                edsc_token: None,
                edsc_reserve_vault: None,
                edsc_minting_engine: None,
                edsc_peg_stabilizer: None,
                edsc_external_swap_router: None,
                etr_token: None,
            }
        }

        // ═══════════════════════════════════════════════════════════════
        // WRAPPED TOKEN REGISTRATION
        // ═══════════════════════════════════════════════════════════════

        /// Register a wrapped token
        #[ink(message)]
        pub fn register_wrapped_token(
            &mut self,
            symbol: String, // "wBTC", "wETH", etc.
            address: AccountId,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;
            self.validate_address(&address)?;

            self.wrapped_tokens.insert(&symbol, &address);

            self.env().emit_event(AddressRegistered {
                category: "wrapped_token".to_string(),
                key: symbol,
                address,
            });

            Ok(())
        }

        /// Get wrapped token address
        #[ink(message)]
        pub fn get_wrapped_token(&self, symbol: String) -> Result<AccountId> {
            self.wrapped_tokens.get(&symbol).ok_or(Error::AddressNotFound)
        }

        // ═══════════════════════════════════════════════════════════════
        // TIER 1 POOL REGISTRATION
        // ═══════════════════════════════════════════════════════════════

        /// Register a Tier 1 pool
        #[ink(message)]
        pub fn register_tier1_pool(
            &mut self,
            currency: String, // "BTC", "ETH", etc.
            address: AccountId,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;
            self.validate_address(&address)?;

            self.tier1_pools.insert(&currency, &address);

            self.env().emit_event(AddressRegistered {
                category: "tier1_pool".to_string(),
                key: currency,
                address,
            });

            Ok(())
        }

        /// Get Tier 1 pool address
        #[ink(message)]
        pub fn get_tier1_pool(&self, currency: String) -> Result<AccountId> {
            self.tier1_pools.get(&currency).ok_or(Error::AddressNotFound)
        }

        // ═══════════════════════════════════════════════════════════════
        // TIER 2 POOL REGISTRATION
        // ═══════════════════════════════════════════════════════════════

        /// Register a Tier 2 pool
        #[ink(message)]
        pub fn register_tier2_pool(
            &mut self,
            currency: String, // "BTC", "ETH", etc.
            address: AccountId,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;
            self.validate_address(&address)?;

            self.tier2_pools.insert(&currency, &address);

            self.env().emit_event(AddressRegistered {
                category: "tier2_pool".to_string(),
                key: currency,
                address,
            });

            Ok(())
        }

        /// Get Tier 2 pool address
        #[ink(message)]
        pub fn get_tier2_pool(&self, currency: String) -> Result<AccountId> {
            self.tier2_pools.get(&currency).ok_or(Error::AddressNotFound)
        }

        // ═══════════════════════════════════════════════════════════════
        // INTENT ROUTER SYSTEM REGISTRATION
        // ═══════════════════════════════════════════════════════════════

        /// Register Intent Router
        #[ink(message)]
        pub fn register_intent_router(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.intent_router = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "intent_router".to_string(),
                key: "intent_router".to_string(),
                address,
            });

            Ok(())
        }

        /// Get Intent Router address
        #[ink(message)]
        pub fn get_intent_router(&self) -> Result<AccountId> {
            self.intent_router.ok_or(Error::AddressNotFound)
        }

        /// Register Auto Swap Executor
        #[ink(message)]
        pub fn register_auto_swap_executor(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.auto_swap_executor = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "intent_router".to_string(),
                key: "auto_swap_executor".to_string(),
                address,
            });

            Ok(())
        }

        /// Get Auto Swap Executor address
        #[ink(message)]
        pub fn get_auto_swap_executor(&self) -> Result<AccountId> {
            self.auto_swap_executor.ok_or(Error::AddressNotFound)
        }

        /// Register Two Tier Bridge Router
        #[ink(message)]
        pub fn register_two_tier_bridge_router(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.two_tier_bridge_router = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "intent_router".to_string(),
                key: "two_tier_bridge_router".to_string(),
                address,
            });

            Ok(())
        }

        /// Get Two Tier Bridge Router address
        #[ink(message)]
        pub fn get_two_tier_bridge_router(&self) -> Result<AccountId> {
            self.two_tier_bridge_router.ok_or(Error::AddressNotFound)
        }

        /// Register Stablecoin Router
        #[ink(message)]
        pub fn register_stablecoin_router(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.stablecoin_router = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "intent_router".to_string(),
                key: "stablecoin_router".to_string(),
                address,
            });

            Ok(())
        }

        /// Get Stablecoin Router address
        #[ink(message)]
        pub fn get_stablecoin_router(&self) -> Result<AccountId> {
            self.stablecoin_router.ok_or(Error::AddressNotFound)
        }

        // ═══════════════════════════════════════════════════════════════
        // EDSC SYSTEM REGISTRATION
        // ═══════════════════════════════════════════════════════════════

        /// Register EDSC Token
        #[ink(message)]
        pub fn register_edsc_token(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.edsc_token = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "edsc".to_string(),
                key: "edsc_token".to_string(),
                address,
            });

            Ok(())
        }

        /// Get EDSC Token address
        #[ink(message)]
        pub fn get_edsc_token(&self) -> Result<AccountId> {
            self.edsc_token.ok_or(Error::AddressNotFound)
        }

        /// Register EDSC Reserve Vault
        #[ink(message)]
        pub fn register_edsc_reserve_vault(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.edsc_reserve_vault = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "edsc".to_string(),
                key: "edsc_reserve_vault".to_string(),
                address,
            });

            Ok(())
        }

        /// Get EDSC Reserve Vault address
        #[ink(message)]
        pub fn get_edsc_reserve_vault(&self) -> Result<AccountId> {
            self.edsc_reserve_vault.ok_or(Error::AddressNotFound)
        }

        /// Register EDSC Minting Engine
        #[ink(message)]
        pub fn register_edsc_minting_engine(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.edsc_minting_engine = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "edsc".to_string(),
                key: "edsc_minting_engine".to_string(),
                address,
            });

            Ok(())
        }

        /// Get EDSC Minting Engine address
        #[ink(message)]
        pub fn get_edsc_minting_engine(&self) -> Result<AccountId> {
            self.edsc_minting_engine.ok_or(Error::AddressNotFound)
        }

        /// Register EDSC Peg Stabilizer
        #[ink(message)]
        pub fn register_edsc_peg_stabilizer(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.edsc_peg_stabilizer = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "edsc".to_string(),
                key: "edsc_peg_stabilizer".to_string(),
                address,
            });

            Ok(())
        }

        /// Get EDSC Peg Stabilizer address
        #[ink(message)]
        pub fn get_edsc_peg_stabilizer(&self) -> Result<AccountId> {
            self.edsc_peg_stabilizer.ok_or(Error::AddressNotFound)
        }

        /// Register EDSC External Swap Router
        #[ink(message)]
        pub fn register_edsc_external_swap_router(&mut self, address: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&address)?;

            self.edsc_external_swap_router = Some(address);

            self.env().emit_event(AddressRegistered {
                category: "edsc".to_string(),
                key: "edsc_external_swap_router".to_string(),
                address,
            });

            Ok(())
        }

        /// Get EDSC External Swap Router address
        #[ink(message)]
        pub fn get_edsc_external_swap_router(&self) -> Result<AccountId> {
            self.edsc_external_swap_router.ok_or(Error::AddressNotFound)
        }

        // ═══════════════════════════════════════════════════════════════
        // BATCH OPERATIONS (For efficient deployment)
        // ═══════════════════════════════════════════════════════════════

        /// Register all wrapped tokens at once
        #[ink(message)]
        pub fn register_all_wrapped_tokens(
            &mut self,
            symbols: ink::prelude::vec::Vec<String>,
            addresses: ink::prelude::vec::Vec<AccountId>,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;

            if symbols.len() != addresses.len() {
                return Err(Error::InvalidAddress);
            }

            for (symbol, address) in symbols.iter().zip(addresses.iter()) {
                self.validate_address(address)?;
                self.wrapped_tokens.insert(symbol, address);
            }

            Ok(())
        }

        /// Register all Tier 1 pools at once
        #[ink(message)]
        pub fn register_all_tier1_pools(
            &mut self,
            currencies: ink::prelude::vec::Vec<String>,
            addresses: ink::prelude::vec::Vec<AccountId>,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;

            if currencies.len() != addresses.len() {
                return Err(Error::InvalidAddress);
            }

            for (currency, address) in currencies.iter().zip(addresses.iter()) {
                self.validate_address(address)?;
                self.tier1_pools.insert(currency, address);
            }

            Ok(())
        }

        /// Register all Tier 2 pools at once
        #[ink(message)]
        pub fn register_all_tier2_pools(
            &mut self,
            currencies: ink::prelude::vec::Vec<String>,
            addresses: ink::prelude::vec::Vec<AccountId>,
        ) -> Result<()> {
            self.ensure_owner()?;
            self.ensure_not_paused()?;

            if currencies.len() != addresses.len() {
                return Err(Error::InvalidAddress);
            }

            for (currency, address) in currencies.iter().zip(addresses.iter()) {
                self.validate_address(address)?;
                self.tier2_pools.insert(currency, address);
            }

            Ok(())
        }

        // ═══════════════════════════════════════════════════════════════
        // ADMIN FUNCTIONS
        // ═══════════════════════════════════════════════════════════════

        /// Transfer ownership
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.ensure_owner()?;
            self.validate_address(&new_owner)?;

            self.owner = new_owner;

            Ok(())
        }

        /// Pause contract
        #[ink(message)]
        pub fn pause(&mut self) -> Result<()> {
            self.ensure_owner()?;

            self.paused = true;

            Ok(())
        }

        /// Unpause contract
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<()> {
            self.ensure_owner()?;

            self.paused = false;

            Ok(())
        }

        // ═══════════════════════════════════════════════════════════════
        // HELPER FUNCTIONS
        // ═══════════════════════════════════════════════════════════════

        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            Ok(())
        }

        fn ensure_not_paused(&self) -> Result<()> {
            if self.paused {
                return Err(Error::ContractPaused);
            }
            Ok(())
        }

        fn validate_address(&self, address: &AccountId) -> Result<()> {
            // Check for zero address (all zeros)
            let zero_address = AccountId::from([0u8; 32]);
            if address == &zero_address {
                return Err(Error::InvalidAddress);
            }
            Ok(())
        }

        /// Get owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Check if paused
        #[ink(message)]
        pub fn is_paused(&self) -> bool {
            self.paused
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let registry = AddressRegistry::new();
            assert_eq!(registry.is_paused(), false);
        }

        #[ink::test]
        fn register_wrapped_token_works() {
            let mut registry = AddressRegistry::new();
            let wbtc_address = AccountId::from([1u8; 32]);

            assert!(registry.register_wrapped_token("wBTC".to_string(), wbtc_address).is_ok());
            assert_eq!(registry.get_wrapped_token("wBTC".to_string()).unwrap(), wbtc_address);
        }

        #[ink::test]
        fn pause_works() {
            let mut registry = AddressRegistry::new();

            assert!(registry.pause().is_ok());
            assert_eq!(registry.is_paused(), true);

            // Should fail when paused
            let wbtc_address = AccountId::from([1u8; 32]);
            assert_eq!(
                registry.register_wrapped_token("wBTC".to_string(), wbtc_address),
                Err(Error::ContractPaused)
            );
        }
    }
}
