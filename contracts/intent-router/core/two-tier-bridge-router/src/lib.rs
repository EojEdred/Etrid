#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod two_tier_bridge_router {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadLayout;
    use ink_storage::collections::HashMap as StorageHashMap;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAuthorized,
        ChainNotSupported,
        InvalidAmount,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, Copy)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ChainId {
        Ethereum = 1,
        Bsc = 56,
        Polygon = 137,
        Solana = 999, // Custom ID for Solana
        Cardano = 1001,
        XRP = 1002,
        Stellar = 1003,
    }

    #[ink(storage)]
    pub struct TwoTierBridgeRouter {
        owner: AccountId,
        supported_chains: StorageHashMap<ChainId, bool>,
        bridge_pallet_id: u32,
    }

    impl TwoTierBridgeRouter {
        #[ink(constructor)]
        pub fn new(bridge_pallet_id: u32) -> Self {
            let mut instance = Self {
                owner: Self::env().caller(),
                supported_chains: StorageHashMap::new(),
                bridge_pallet_id,
            };
            
            // Enable all supported chains
            instance.supported_chains.insert(ChainId::Ethereum, true);
            instance.supported_chains.insert(ChainId::Bsc, true);
            instance.supported_chains.insert(ChainId::Polygon, true);
            instance.supported_chains.insert(ChainId::Solana, true);
            instance.supported_chains.insert(ChainId::Cardano, true);
            instance.supported_chains.insert(ChainId::XRP, true);
            instance.supported_chains.insert(ChainId::Stellar, true);

            instance
        }

        #[ink(message)]
        pub fn route_message(&self, destination_chain: ChainId, payload: Vec<u8>) -> Result<(), Error> {
            if !self.is_chain_supported(destination_chain) {
                return Err(Error::ChainNotSupported);
            }
            
            // Logic to forward message to the configured bridge pallet
            // This would interact with the chain extension or XCM in a real implementation
            
            Ok(())
        }

        #[ink(message)]
        pub fn is_chain_supported(&self, chain_id: ChainId) -> bool {
            *self.supported_chains.get(&chain_id).unwrap_or(&false)
        }

        #[ink(message)]
        pub fn add_supported_chain(&mut self, chain_id: ChainId) -> Result<(), Error> {
            self.ensure_owner()?;
            self.supported_chains.insert(chain_id, true);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_supported_chain(&mut self, chain_id: ChainId) -> Result<(), Error> {
            self.ensure_owner()?;
            self.supported_chains.insert(chain_id, false);
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::NotAuthorized);
            }
            Ok(())
        }
    }
}
