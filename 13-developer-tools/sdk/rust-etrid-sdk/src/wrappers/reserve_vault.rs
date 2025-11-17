//! Reserve Vault - Collateralized lending and borrowing
//!
//! This module provides functionality for managing collateralized vaults,
//! borrowing against collateral, and liquidations.

use async_trait::async_trait;
use crate::{Client, Error, Result};
use crate::types::{Address, Vault, CollateralType, TxHash};
use serde::{Deserialize, Serialize};

/// Reserve Vault wrapper for DeFi operations
pub struct ReserveVaultWrapper {
    client: Client,
}

/// Vault creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVaultParams {
    /// Collateral type
    pub collateral_type: CollateralType,
    /// Initial collateral amount
    pub collateral_amount: u128,
}

/// Borrow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowParams {
    /// Vault ID
    pub vault_id: String,
    /// Amount to borrow
    pub amount: u128,
}

/// Liquidation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationParams {
    /// Vault ID to liquidate
    pub vault_id: String,
    /// Maximum debt to cover
    pub max_debt_to_cover: u128,
}

impl ReserveVaultWrapper {
    /// Create a new Reserve Vault wrapper instance
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the Ëtrid RPC client
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::ReserveVaultWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// let vault = ReserveVaultWrapper::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new vault
    ///
    /// # Arguments
    ///
    /// * `params` - Vault creation parameters
    ///
    /// # Returns
    ///
    /// Returns the vault ID and transaction hash
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{ReserveVaultWrapper, CreateVaultParams}};
    /// # use etrid_sdk::types::CollateralType;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let vault = ReserveVaultWrapper::new(client);
    /// let params = CreateVaultParams {
    ///     collateral_type: CollateralType::Eth,
    ///     collateral_amount: 10_000_000_000_000,
    /// };
    /// let (vault_id, tx_hash) = vault.create_vault(params).await?;
    /// println!("Vault created: {}", vault_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_vault(&self, params: CreateVaultParams) -> Result<(String, TxHash)> {
        // Validate collateral amount
        if params.collateral_amount == 0 {
            return Err(Error::Vault(
                "Collateral amount must be greater than zero".to_string(),
            ));
        }

        // Check minimum collateral requirements
        let min_collateral = self.get_min_collateral(&params.collateral_type).await?;
        if params.collateral_amount < min_collateral {
            return Err(Error::Vault(format!(
                "Collateral amount must be at least {}",
                min_collateral
            )));
        }

        // In production, submit create_vault extrinsic
        let vault_id = format!("vault_{}", Self::timestamp());
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((vault_id, tx_hash))
    }

    /// Deposit additional collateral
    ///
    /// # Arguments
    ///
    /// * `vault_id` - Vault ID
    /// * `amount` - Amount to deposit
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    pub async fn deposit_collateral(&self, vault_id: &str, amount: u128) -> Result<TxHash> {
        if amount == 0 {
            return Err(Error::Vault("Amount must be greater than zero".to_string()));
        }

        // Verify vault exists
        let _ = self.get_vault(vault_id).await?;

        // In production, submit deposit extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Borrow against collateral
    ///
    /// # Arguments
    ///
    /// * `params` - Borrow parameters
    ///
    /// # Returns
    ///
    /// Returns the transaction hash and new health factor
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{ReserveVaultWrapper, BorrowParams}};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let vault = ReserveVaultWrapper::new(client);
    /// let params = BorrowParams {
    ///     vault_id: "vault_12345".to_string(),
    ///     amount: 5_000_000_000_000,
    /// };
    /// let (tx_hash, health_factor) = vault.borrow(params).await?;
    /// println!("Borrowed successfully, health factor: {}", health_factor);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn borrow(&self, params: BorrowParams) -> Result<(TxHash, u32)> {
        if params.amount == 0 {
            return Err(Error::Vault("Amount must be greater than zero".to_string()));
        }

        // Get vault info
        let vault_info = self.get_vault(&params.vault_id).await?;

        // Calculate new health factor
        let new_borrowed = vault_info.borrowed + params.amount;
        let health_factor = self
            .calculate_health_factor(vault_info.collateral, new_borrowed)
            .await?;

        // Check if vault would remain healthy
        if health_factor < 10000 {
            // 1.0 in scaled format
            return Err(Error::Vault(
                "Borrow would make vault unhealthy".to_string(),
            ));
        }

        // In production, submit borrow extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((tx_hash, health_factor))
    }

    /// Repay borrowed amount
    ///
    /// # Arguments
    ///
    /// * `vault_id` - Vault ID
    /// * `amount` - Amount to repay
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    pub async fn repay(&self, vault_id: &str, amount: u128) -> Result<TxHash> {
        if amount == 0 {
            return Err(Error::Vault("Amount must be greater than zero".to_string()));
        }

        // Get vault info
        let vault_info = self.get_vault(vault_id).await?;

        if amount > vault_info.borrowed {
            return Err(Error::Vault(
                "Repay amount exceeds borrowed amount".to_string(),
            ));
        }

        // In production, submit repay extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// Get vault information
    ///
    /// # Arguments
    ///
    /// * `vault_id` - Vault ID
    ///
    /// # Returns
    ///
    /// Returns vault information
    pub async fn get_vault(&self, vault_id: &str) -> Result<Vault> {
        // In production, query vault from storage
        let vault = Vault {
            id: vault_id.to_string(),
            owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            collateral: 10_000_000_000_000,
            borrowed: 5_000_000_000_000,
            health_factor: 20000, // 2.0
        };

        Ok(vault)
    }

    /// Get health factor for a vault
    ///
    /// # Arguments
    ///
    /// * `vault_id` - Vault ID
    ///
    /// # Returns
    ///
    /// Returns the health factor (scaled by 10000)
    pub async fn get_health_factor(&self, vault_id: &str) -> Result<u32> {
        let vault = self.get_vault(vault_id).await?;
        Ok(vault.health_factor)
    }

    /// Liquidate an unhealthy vault
    ///
    /// # Arguments
    ///
    /// * `params` - Liquidation parameters
    ///
    /// # Returns
    ///
    /// Returns the transaction hash
    pub async fn liquidate(&self, params: LiquidationParams) -> Result<TxHash> {
        // Get vault info
        let vault = self.get_vault(&params.vault_id).await?;

        // Check if vault is eligible for liquidation
        if vault.health_factor >= 10000 {
            return Err(Error::Vault("Vault is healthy, cannot liquidate".to_string()));
        }

        if params.max_debt_to_cover == 0 {
            return Err(Error::Vault(
                "Max debt to cover must be greater than zero".to_string(),
            ));
        }

        // In production, submit liquidation extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok(tx_hash)
    }

    /// List all vaults for an owner
    ///
    /// # Arguments
    ///
    /// * `owner` - Owner address
    ///
    /// # Returns
    ///
    /// Returns a vector of vaults
    pub async fn list_vaults(&self, owner: &str) -> Result<Vec<Vault>> {
        // In production, query all vaults for owner
        Ok(vec![])
    }

    /// Calculate health factor
    async fn calculate_health_factor(&self, collateral: u128, borrowed: u128) -> Result<u32> {
        if borrowed == 0 {
            return Ok(u32::MAX);
        }

        // Simplified calculation: (collateral / borrowed) * 10000
        // In production, would factor in collateral ratio, liquidation threshold, etc.
        let health_factor = ((collateral * 10000) / borrowed) as u32;

        Ok(health_factor)
    }

    /// Get minimum collateral for a collateral type
    async fn get_min_collateral(&self, collateral_type: &CollateralType) -> Result<u128> {
        // In production, query from storage
        Ok(match collateral_type {
            CollateralType::Etr => 1_000_000_000_000,
            CollateralType::Btc => 100_000_000,
            CollateralType::Eth => 1_000_000_000_000,
            CollateralType::Stable => 10_000_000_000,
        })
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
    fn test_health_factor_calculation() {
        let collateral = 10_000_000_000_000u128;
        let borrowed = 5_000_000_000_000u128;
        let health_factor = ((collateral * 10000) / borrowed) as u32;
        assert_eq!(health_factor, 20000); // 2.0
    }

    #[test]
    fn test_health_factor_safe() {
        let health_factor = 20000u32;
        assert!(health_factor >= 10000); // Above 1.0 is safe
    }
}
