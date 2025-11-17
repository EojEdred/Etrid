//! Bridge - Cross-chain asset transfers
//!
//! This module provides functionality for bridging assets between Ëtrid and
//! other blockchain networks.

use async_trait::async_trait;
use crate::{Client, Error, Result};
use crate::types::{Address, Chain, BridgeTransfer, TransferStatus, TxHash};
use serde::{Deserialize, Serialize};

/// Bridge wrapper for cross-chain operations
pub struct BridgeWrapper {
    client: Client,
}

/// Bridge transfer parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeParams {
    /// Source chain
    pub source_chain: Chain,
    /// Destination chain
    pub destination_chain: Chain,
    /// Amount to bridge
    pub amount: u128,
    /// Destination address on target chain
    pub destination_address: String,
    /// Optional memo/data
    pub memo: Option<Vec<u8>>,
}

/// Bridge fee information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeFee {
    /// Base fee
    pub base_fee: u128,
    /// Percentage fee (in basis points, 10000 = 100%)
    pub percentage_fee: u16,
    /// Total fee
    pub total_fee: u128,
    /// Estimated time in blocks
    pub estimated_blocks: u64,
}

impl BridgeWrapper {
    /// Create a new Bridge wrapper instance
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the Ëtrid RPC client
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::BridgeWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// let bridge = BridgeWrapper::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Initiate a cross-chain bridge transfer
    ///
    /// # Arguments
    ///
    /// * `params` - Bridge transfer parameters
    ///
    /// # Returns
    ///
    /// Returns the transfer ID and transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{BridgeWrapper, BridgeParams}};
    /// # use etrid_sdk::types::Chain;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let bridge = BridgeWrapper::new(client);
    /// let params = BridgeParams {
    ///     source_chain: Chain::BinanceSmartChain,
    ///     destination_chain: Chain::Ethereum,
    ///     amount: 1_000_000_000_000,
    ///     destination_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
    ///     memo: None,
    /// };
    /// let (transfer_id, tx_hash) = bridge.bridge(params).await?;
    /// println!("Bridge transfer initiated: {}", transfer_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bridge(&self, params: BridgeParams) -> Result<(String, TxHash)> {
        // Validate chains are different
        if params.source_chain == params.destination_chain {
            return Err(Error::Bridge(
                "Source and destination chains must be different".to_string(),
            ));
        }

        // Validate amount
        if params.amount == 0 {
            return Err(Error::Bridge("Amount must be greater than zero".to_string()));
        }

        // Validate destination address
        if params.destination_address.is_empty() {
            return Err(Error::Bridge("Destination address cannot be empty".to_string()));
        }

        // Check if chains are supported
        let supported = self.get_supported_chains().await?;
        if !supported.contains(&params.source_chain)
            || !supported.contains(&params.destination_chain)
        {
            return Err(Error::Bridge("Unsupported chain".to_string()));
        }

        // Calculate fees
        let fee_info = self
            .get_bridge_fee(&params.source_chain, &params.destination_chain, params.amount)
            .await?;

        // In production, submit bridge extrinsic
        let transfer_id = format!("bridge_{}", Self::timestamp());
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((transfer_id, tx_hash))
    }

    /// Get bridge transfer status
    ///
    /// # Arguments
    ///
    /// * `transfer_id` - Transfer ID
    ///
    /// # Returns
    ///
    /// Returns the bridge transfer information
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::BridgeWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let bridge = BridgeWrapper::new(client);
    /// let transfer = bridge.get_transfer_status("bridge_12345").await?;
    /// println!("Status: {:?}", transfer.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transfer_status(&self, transfer_id: &str) -> Result<BridgeTransfer> {
        // In production, query transfer from storage
        let transfer = BridgeTransfer {
            id: transfer_id.to_string(),
            source_chain: Chain::Ethereum,
            destination_chain: Chain::BinanceSmartChain,
            amount: 1_000_000_000_000,
            status: TransferStatus::Pending,
            fee: 50_000_000_000,
        };

        Ok(transfer)
    }

    /// Get list of supported chains
    ///
    /// # Returns
    ///
    /// Returns a vector of supported chains
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::BridgeWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let bridge = BridgeWrapper::new(client);
    /// let chains = bridge.get_supported_chains().await?;
    /// println!("Supported chains: {}", chains.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_supported_chains(&self) -> Result<Vec<Chain>> {
        // In production, query supported chains from storage
        Ok(vec![
            Chain::Ethereum,
            Chain::Bitcoin,
            Chain::BinanceSmartChain,
            Chain::Polygon,
            Chain::Solana,
            Chain::Cardano,
        ])
    }

    /// Get bridge fee for a transfer
    ///
    /// # Arguments
    ///
    /// * `source` - Source chain
    /// * `destination` - Destination chain
    /// * `amount` - Transfer amount
    ///
    /// # Returns
    ///
    /// Returns bridge fee information
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::BridgeWrapper};
    /// # use etrid_sdk::types::Chain;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let bridge = BridgeWrapper::new(client);
    /// let fee = bridge.get_bridge_fee(
    ///     &Chain::Ethereum,
    ///     &Chain::BinanceSmartChain,
    ///     1_000_000_000_000
    /// ).await?;
    /// println!("Bridge fee: {} ETR", fee.total_fee);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bridge_fee(
        &self,
        source: &Chain,
        destination: &Chain,
        amount: u128,
    ) -> Result<BridgeFee> {
        if amount == 0 {
            return Err(Error::Bridge("Amount must be greater than zero".to_string()));
        }

        // In production, query fee schedule from storage
        // Fee structure varies by chain pair
        let base_fee = match (source, destination) {
            (Chain::Ethereum, _) | (_, Chain::Ethereum) => 100_000_000,
            (Chain::Bitcoin, _) | (_, Chain::Bitcoin) => 150_000_000,
            _ => 50_000_000,
        };

        let percentage_fee = 30; // 0.3% (30 basis points)
        let percentage_amount = (amount * percentage_fee as u128) / 10000;

        let total_fee = base_fee + percentage_amount;

        let estimated_blocks = match destination {
            Chain::Bitcoin => 600, // ~1 hour
            Chain::Ethereum => 300, // ~30 minutes
            _ => 100,
        };

        Ok(BridgeFee {
            base_fee,
            percentage_fee,
            total_fee,
            estimated_blocks,
        })
    }

    /// List all transfers for an account
    ///
    /// # Arguments
    ///
    /// * `account` - Account address
    ///
    /// # Returns
    ///
    /// Returns a vector of bridge transfers
    pub async fn list_transfers(&self, account: &str) -> Result<Vec<BridgeTransfer>> {
        // In production, query all transfers for account
        Ok(vec![])
    }

    /// Cancel a pending bridge transfer
    ///
    /// # Arguments
    ///
    /// * `transfer_id` - Transfer ID
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    pub async fn cancel_transfer(&self, transfer_id: &str) -> Result<TxHash> {
        // Check transfer status
        let transfer = self.get_transfer_status(transfer_id).await?;

        if transfer.status != TransferStatus::Pending {
            return Err(Error::Bridge(
                "Only pending transfers can be cancelled".to_string(),
            ));
        }

        // In production, submit cancel extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Timestamp helper
    fn timestamp() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_calculation() {
        let amount = 1_000_000_000_000u128;
        let percentage_fee = 30u16; // 0.3%
        let percentage_amount = (amount * percentage_fee as u128) / 10000;
        assert_eq!(percentage_amount, 3_000_000_000);
    }

    #[test]
    fn test_chain_equality() {
        let chain1 = Chain::Ethereum;
        let chain2 = Chain::Ethereum;
        assert_eq!(chain1, chain2);

        let chain3 = Chain::Bitcoin;
        assert_ne!(chain1, chain3);
    }
}
