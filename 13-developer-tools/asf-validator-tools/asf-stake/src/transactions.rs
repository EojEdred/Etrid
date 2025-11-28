use anyhow::Result;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

use crate::rpc::{RpcClient, TxStatus};

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTION SUBMISSION
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn submit_bond_transaction(
    client: &RpcClient,
    amount: u128,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<String> {
    // In production, this would:
    // 1. Load keypair from keyfile using password
    // 2. Construct the bond extrinsic
    // 3. Sign the extrinsic
    // 4. Submit to the chain

    // For now, return mock transaction hash
    let extrinsic = format!("0x{:0>128}", "mock_bond_extrinsic");
    client.submit_transaction(&extrinsic).await
}

pub async fn submit_unbond_transaction(
    client: &RpcClient,
    amount: u128,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<String> {
    // Similar to bond, but for unbonding
    let extrinsic = format!("0x{:0>128}", "mock_unbond_extrinsic");
    client.submit_transaction(&extrinsic).await
}

pub async fn submit_claim_transaction(
    client: &RpcClient,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<String> {
    // Similar to bond, but for claiming rewards
    let extrinsic = format!("0x{:0>128}", "mock_claim_extrinsic");
    client.submit_transaction(&extrinsic).await
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTION CONFIRMATION
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn wait_for_confirmation(client: &RpcClient, tx_hash: &str) -> Result<()> {
    let max_attempts = 30;
    let mut attempts = 0;

    loop {
        attempts += 1;

        if attempts > max_attempts {
            return Err(anyhow::anyhow!("Transaction confirmation timeout"));
        }

        let status = client.get_transaction_status(tx_hash).await?;

        match status.status {
            TxStatus::Finalized => {
                if status.success {
                    return Ok(());
                } else {
                    return Err(anyhow::anyhow!("Transaction failed"));
                }
            }
            TxStatus::Failed => {
                return Err(anyhow::anyhow!("Transaction failed"));
            }
            _ => {
                // Still pending, wait and retry
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
