use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

// ═══════════════════════════════════════════════════════════════════════════════
// RPC CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

pub struct RpcClient {
    endpoint: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_validator_stake_info(&self, validator: &str) -> Result<ValidatorStakeInfo> {
        // In production, this would call: asf_validatorStakeInfo(validator)

        Ok(ValidatorStakeInfo {
            address: validator.to_string(),
            total_stake: 150_000_000_000_000,       // 150,000 ETR
            self_stake: 100_000_000_000_000,        // 100,000 ETR
            delegated_stake: 50_000_000_000_000,    // 50,000 ETR
            is_active: true,
            in_committee: true,
            commission: 10,
            delegator_count: 25,
            pending_rewards: 1_250_000_000_000,     // 1,250 ETR
            last_claim_epoch: 42,
            reputation: 95,
        })
    }

    pub async fn get_validator_rewards(&self, validator: &str, epochs: u32) -> Result<Vec<RewardInfo>> {
        // In production, this would call: asf_validatorRewards(validator, epochs)

        let mut rewards = Vec::new();

        for i in 0..epochs {
            let epoch = 50 - i;
            let amount = 125_000_000_000; // ~125 ETR per epoch
            let claimed = i > 5;

            rewards.push(RewardInfo {
                epoch,
                amount,
                blocks_produced: 100,
                claimed,
            });
        }

        Ok(rewards)
    }

    pub async fn get_all_validators(&self) -> Result<Vec<ValidatorInfo>> {
        // In production, this would call: asf_allValidators()

        let mut validators = Vec::new();

        for i in 0..21 {
            validators.push(ValidatorInfo {
                address: format!("5{:0>63}", i), // Mock address
                total_stake: (200_000 - i as u128 * 5_000) * 1_000_000_000_000,
                reputation: 95 - (i as u8 % 10),
                in_committee: i < 21,
                pending_rewards: (1_000 + i as u128 * 100) * 1_000_000_000,
            });
        }

        Ok(validators)
    }

    pub async fn submit_transaction(&self, extrinsic: &str) -> Result<String> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "author_submitExtrinsic",
            "params": [extrinsic],
            "id": 1,
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        // Parse transaction hash from response
        // For now, return mock hash
        Ok(format!("0x{:0>64}", "123abc"))
    }

    pub async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus> {
        // In production, this would poll for transaction status

        Ok(TransactionStatus {
            hash: tx_hash.to_string(),
            status: TxStatus::Finalized,
            block_number: Some(12345),
            success: true,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RPC TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorStakeInfo {
    pub address: String,
    pub total_stake: u128,
    pub self_stake: u128,
    pub delegated_stake: u128,
    pub is_active: bool,
    pub in_committee: bool,
    pub commission: u8,
    pub delegator_count: u32,
    pub pending_rewards: u128,
    pub last_claim_epoch: u32,
    pub reputation: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardInfo {
    pub epoch: u32,
    pub amount: u128,
    pub blocks_produced: u32,
    pub claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub address: String,
    pub total_stake: u128,
    pub reputation: u8,
    pub in_committee: bool,
    pub pending_rewards: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub hash: String,
    pub status: TxStatus,
    pub block_number: Option<u64>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    InBlock,
    Finalized,
    Failed,
}
