//! Distribution Pay - Automated reward distribution system
//!
//! This module handles the distribution of rewards to validators, directors,
//! developers, and community members according to the Ëtrid tokenomics model.

use async_trait::async_trait;
use crate::{Client, Error, Result};
use crate::types::{Address, Reward, RewardCategory, DistributionSchedule, TxHash};
use serde::{Deserialize, Serialize};

/// Distribution Pay wrapper for reward management
pub struct DistributionPayWrapper {
    client: Client,
}

/// Reward claim parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRewardParams {
    /// Category to claim from
    pub category: RewardCategory,
    /// Optional amount (claims all if None)
    pub amount: Option<u128>,
}

/// Eligibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityInfo {
    /// Is eligible
    pub eligible: bool,
    /// Categories eligible for
    pub categories: Vec<RewardCategory>,
    /// Total pending rewards
    pub pending_rewards: u128,
    /// Reason if not eligible
    pub reason: Option<String>,
}

impl DistributionPayWrapper {
    /// Create a new Distribution Pay wrapper instance
    ///
    /// # Arguments
    ///
    /// * `client` - Reference to the Ëtrid RPC client
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::DistributionPayWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Claim pending rewards
    ///
    /// # Arguments
    ///
    /// * `params` - Claim parameters
    ///
    /// # Returns
    ///
    /// Returns the transaction hash and amount claimed
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::{DistributionPayWrapper, ClaimRewardParams}};
    /// # use etrid_sdk::types::RewardCategory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// let params = ClaimRewardParams {
    ///     category: RewardCategory::Validator,
    ///     amount: None, // Claim all
    /// };
    /// let (tx_hash, amount) = distribution.claim_reward(params).await?;
    /// println!("Claimed {} ETR, tx: {}", amount, tx_hash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn claim_reward(&self, params: ClaimRewardParams) -> Result<(TxHash, u128)> {
        // Check eligibility first
        let eligibility = self.is_eligible(&params.category).await?;

        if !eligibility {
            return Err(Error::Distribution(
                "Not eligible to claim from this category".to_string(),
            ));
        }

        // Get pending rewards
        let pending = self.get_pending_rewards(&params.category).await?;

        if pending.is_empty() {
            return Err(Error::Distribution("No pending rewards".to_string()));
        }

        let total_pending: u128 = pending.iter().map(|r| r.amount).sum();

        let claim_amount = params.amount.unwrap_or(total_pending);

        if claim_amount > total_pending {
            return Err(Error::InsufficientBalance {
                required: claim_amount,
                available: total_pending,
            });
        }

        // In production, submit claim extrinsic
        let tx_hash = format!("0x{:064x}", Self::timestamp());

        Ok((tx_hash, claim_amount))
    }

    /// Get pending rewards for an account
    ///
    /// # Arguments
    ///
    /// * `category` - Reward category
    ///
    /// # Returns
    ///
    /// Returns a vector of pending rewards
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::DistributionPayWrapper};
    /// # use etrid_sdk::types::RewardCategory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// let rewards = distribution.get_pending_rewards(&RewardCategory::Validator).await?;
    /// let total: u128 = rewards.iter().map(|r| r.amount).sum();
    /// println!("Total pending: {} ETR", total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pending_rewards(&self, category: &RewardCategory) -> Result<Vec<Reward>> {
        // In production, query pending rewards from storage
        let rewards = vec![
            Reward {
                amount: 1_000_000_000_000,
                category: category.clone(),
                timestamp: Self::timestamp() as u64,
            },
            Reward {
                amount: 500_000_000_000,
                category: category.clone(),
                timestamp: (Self::timestamp() - 86400) as u64,
            },
        ];

        Ok(rewards)
    }

    /// Check if an account is eligible for rewards
    ///
    /// # Arguments
    ///
    /// * `category` - Reward category to check
    ///
    /// # Returns
    ///
    /// Returns true if eligible
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::DistributionPayWrapper};
    /// # use etrid_sdk::types::RewardCategory;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// let eligible = distribution.is_eligible(&RewardCategory::Developer).await?;
    /// if eligible {
    ///     println!("You are eligible for developer rewards!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_eligible(&self, category: &RewardCategory) -> Result<bool> {
        // In production, check on-chain eligibility criteria
        // For validators: must be active validator
        // For directors: must hold Director role
        // For developers: must have contributed code
        // For community: must meet participation threshold

        match category {
            RewardCategory::Validator => Ok(true),
            RewardCategory::Director => Ok(false),
            RewardCategory::Developer => Ok(true),
            RewardCategory::Community => Ok(true),
        }
    }

    /// Get all categories an account is eligible for
    ///
    /// # Returns
    ///
    /// Returns eligibility information
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::DistributionPayWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// let info = distribution.get_eligible_categories().await?;
    /// println!("Eligible for {} categories", info.categories.len());
    /// println!("Total pending: {} ETR", info.pending_rewards);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_eligible_categories(&self) -> Result<EligibilityInfo> {
        let mut categories = Vec::new();
        let mut total_pending = 0u128;

        // Check each category
        for category in &[
            RewardCategory::Validator,
            RewardCategory::Director,
            RewardCategory::Developer,
            RewardCategory::Community,
        ] {
            if self.is_eligible(category).await? {
                categories.push(category.clone());
                let rewards = self.get_pending_rewards(category).await?;
                total_pending += rewards.iter().map(|r| r.amount).sum::<u128>();
            }
        }

        Ok(EligibilityInfo {
            eligible: !categories.is_empty(),
            categories,
            pending_rewards: total_pending,
            reason: None,
        })
    }

    /// Get the distribution schedule
    ///
    /// # Returns
    ///
    /// Returns the current distribution schedule
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use etrid_sdk::{Client, wrappers::DistributionPayWrapper};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("ws://localhost:9944").await?;
    /// let distribution = DistributionPayWrapper::new(client);
    /// let schedule = distribution.get_distribution_schedule().await?;
    /// println!("Next distribution in {} blocks", schedule.next_block);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_distribution_schedule(&self) -> Result<DistributionSchedule> {
        // In production, query distribution schedule from storage
        let schedule = DistributionSchedule {
            total_amount: 10_000_000_000_000_000,
            period: 14400, // Daily (assuming 6s blocks)
            next_block: 1000000,
        };

        Ok(schedule)
    }

    /// Get historical rewards for an account
    ///
    /// # Arguments
    ///
    /// * `category` - Reward category
    /// * `from_block` - Start block
    /// * `to_block` - End block
    ///
    /// # Returns
    ///
    /// Returns historical rewards
    pub async fn get_reward_history(
        &self,
        category: &RewardCategory,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Reward>> {
        if from_block > to_block {
            return Err(Error::InvalidInput(
                "from_block must be <= to_block".to_string(),
            ));
        }

        // In production, query historical rewards from storage
        Ok(vec![])
    }

    /// Get total rewards claimed by an account
    ///
    /// # Arguments
    ///
    /// * `category` - Optional category filter
    ///
    /// # Returns
    ///
    /// Returns total amount claimed
    pub async fn get_total_claimed(&self, category: Option<&RewardCategory>) -> Result<u128> {
        // In production, sum all claimed rewards
        Ok(5_000_000_000_000)
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
    fn test_reward_category_equality() {
        let cat1 = RewardCategory::Validator;
        let cat2 = RewardCategory::Validator;
        assert_eq!(cat1, cat2);
    }

    #[test]
    fn test_eligibility_info() {
        let info = EligibilityInfo {
            eligible: true,
            categories: vec![RewardCategory::Developer],
            pending_rewards: 1000,
            reason: None,
        };
        assert!(info.eligible);
        assert_eq!(info.categories.len(), 1);
    }
}
