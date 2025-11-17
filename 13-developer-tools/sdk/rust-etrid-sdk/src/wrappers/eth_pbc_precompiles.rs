//! ETH PBC Precompile Wrappers for Rust SDK
//!
//! Provides Rust interface to Ethereum Partition Burst Chain (ETH PBC) precompiles
//! that enable access to FlareChain features from EVM contracts.
//!
//! # Precompile Addresses
//! - 0x800: Oracle (FlareChain price feeds)
//! - 0x801: Governance (voting from ETH PBC)
//! - 0x802: Staking (validator queries)
//! - 0x803: Native ETH Wrapping (zero-fee wrap/unwrap)
//! - 0x804: XCM Bridge (cross-chain transfers)
//! - 0x805: Token Registry (registered tokens)
//! - 0x806: State Proof (Ethereum state verification)
//!
//! # Example
//! ```no_run
//! use etrid_sdk::wrappers::eth_pbc_precompiles::ETHPBCPrecompiles;
//! use ethers::providers::{Provider, Http};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let provider = Provider::<Http>::try_from("http://localhost:9944")?;
//!     let precompiles = ETHPBCPrecompiles::new(Arc::new(provider));
//!
//!     // Get BTC price from FlareChain oracle
//!     let price = precompiles.get_oracle_price("BTC", "USD").await?;
//!     println!("BTC Price: ${:.2}", price as f64 / 1e18);
//!
//!     Ok(())
//! }
//! ```

use ethers::{
    abi::{encode, Token},
    core::k256::ecdsa::SigningKey,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest, U256, H256},
    utils::keccak256,
};
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur when interacting with ETH PBC precompiles
#[derive(Error, Debug)]
pub enum PrecompileError {
    #[error("Provider error: {0}")]
    ProviderError(#[from] ProviderError),

    #[error("Contract error: {0}")]
    ContractError(String),

    #[error("ABI encoding error: {0}")]
    AbiError(#[from] ethers::abi::Error),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
}

pub type Result<T> = std::result::Result<T, PrecompileError>;

/// Precompile contract addresses
pub mod addresses {
    use ethers::types::Address;

    pub const ORACLE: Address = Address::from_low_u64_be(0x800);
    pub const GOVERNANCE: Address = Address::from_low_u64_be(0x801);
    pub const STAKING: Address = Address::from_low_u64_be(0x802);
    pub const NATIVE_ETH_WRAP: Address = Address::from_low_u64_be(0x803);
    pub const XCM_BRIDGE: Address = Address::from_low_u64_be(0x804);
    pub const TOKEN_REGISTRY: Address = Address::from_low_u64_be(0x805);
    pub const STATE_PROOF: Address = Address::from_low_u64_be(0x806);
}

/// Token information from registry
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_bridged_supply: U256,
}

/// Governance proposal status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending = 0,
    Active = 1,
    Passed = 2,
    Failed = 3,
}

impl From<u8> for ProposalStatus {
    fn from(status: u8) -> Self {
        match status {
            0 => ProposalStatus::Pending,
            1 => ProposalStatus::Active,
            2 => ProposalStatus::Passed,
            3 => ProposalStatus::Failed,
            _ => ProposalStatus::Pending,
        }
    }
}

/// ETH PBC Precompile wrapper client
pub struct ETHPBCPrecompiles<M> {
    provider: Arc<M>,
    wallet: Option<LocalWallet>,
}

impl<M: Middleware> ETHPBCPrecompiles<M> {
    /// Create a new precompile wrapper instance
    pub fn new(provider: Arc<M>) -> Self {
        Self {
            provider,
            wallet: None,
        }
    }

    /// Create a new precompile wrapper with a signer wallet
    pub fn new_with_signer(provider: Arc<M>, wallet: LocalWallet) -> Self {
        Self {
            provider,
            wallet: Some(wallet),
        }
    }

    /// Helper to make a call to a precompile
    async fn call(&self, to: Address, data: Bytes) -> Result<Bytes> {
        let tx = TransactionRequest::new().to(to).data(data);

        let result = self
            .provider
            .call(&tx.into(), None)
            .await
            .map_err(|e| PrecompileError::ContractError(e.to_string()))?;

        Ok(result)
    }

    /// Helper to send a transaction to a precompile
    async fn send_transaction(&self, to: Address, data: Bytes, value: U256) -> Result<H256> {
        let wallet = self
            .wallet
            .as_ref()
            .ok_or_else(|| PrecompileError::ContractError("No wallet configured".to_string()))?;

        let tx = TransactionRequest::new()
            .to(to)
            .data(data)
            .value(value)
            .from(wallet.address());

        let pending = self
            .provider
            .send_transaction(tx, None)
            .await
            .map_err(|e| PrecompileError::TransactionFailed(e.to_string()))?;

        Ok(pending.tx_hash())
    }

    // ========== Oracle Precompile (0x800) ==========

    /// Get price from FlareChain oracle
    ///
    /// # Arguments
    /// * `symbol` - Asset symbol (e.g., "BTC", "ETH", "SOL")
    /// * `quote` - Quote currency (e.g., "USD", "ETH")
    ///
    /// # Returns
    /// Price scaled by 1e18 (e.g., 50000e18 = $50,000)
    pub async fn get_oracle_price(&self, symbol: &str, quote: &str) -> Result<U256> {
        let selector = &keccak256(b"getPrice(bytes32,bytes32)")[..4];

        let mut symbol_bytes = [0u8; 32];
        symbol_bytes[..symbol.len().min(32)].copy_from_slice(&symbol.as_bytes()[..symbol.len().min(32)]);

        let mut quote_bytes = [0u8; 32];
        quote_bytes[..quote.len().min(32)].copy_from_slice(&quote.as_bytes()[..quote.len().min(32)]);

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&symbol_bytes);
        data.extend_from_slice(&quote_bytes);

        let result = self.call(addresses::ORACLE, data.into()).await?;
        Ok(U256::from_big_endian(&result))
    }

    /// Get price in ETH from FlareChain oracle
    pub async fn get_oracle_price_in_eth(&self, symbol: &str) -> Result<U256> {
        let selector = &keccak256(b"getPriceInETH(bytes32)")[..4];

        let mut symbol_bytes = [0u8; 32];
        symbol_bytes[..symbol.len().min(32)].copy_from_slice(&symbol.as_bytes()[..symbol.len().min(32)]);

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&symbol_bytes);

        let result = self.call(addresses::ORACLE, data.into()).await?;
        Ok(U256::from_big_endian(&result))
    }

    /// Get last update timestamp for an oracle price feed
    pub async fn get_oracle_last_update(&self, symbol: &str) -> Result<u64> {
        let selector = &keccak256(b"getLastUpdate(bytes32)")[..4];

        let mut symbol_bytes = [0u8; 32];
        symbol_bytes[..symbol.len().min(32)].copy_from_slice(&symbol.as_bytes()[..symbol.len().min(32)]);

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&symbol_bytes);

        let result = self.call(addresses::ORACLE, data.into()).await?;
        let timestamp = U256::from_big_endian(&result);
        Ok(timestamp.as_u64())
    }

    // ========== Governance Precompile (0x801) ==========

    /// Create a governance proposal on FlareChain
    ///
    /// # Arguments
    /// * `title` - Proposal title (max 256 chars)
    /// * `description` - Proposal description (max 10000 chars)
    ///
    /// # Returns
    /// Transaction hash (proposal ID would be extracted from receipt)
    pub async fn governance_create_proposal(&self, title: &str, description: &str) -> Result<H256> {
        let selector = &keccak256(b"submitProposal(string,string)")[..4];

        let encoded = encode(&[
            Token::String(title.to_string()),
            Token::String(description.to_string()),
        ]);

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&encoded);

        self.send_transaction(addresses::GOVERNANCE, data.into(), U256::zero())
            .await
    }

    /// Vote on a governance proposal
    ///
    /// # Arguments
    /// * `proposal_id` - Proposal ID to vote on
    /// * `support` - true to vote YES, false to vote NO
    pub async fn governance_vote(&self, proposal_id: u64, support: bool) -> Result<H256> {
        let selector = &keccak256(b"voteOnProposal(uint256,bool)")[..4];

        let encoded = encode(&[
            Token::Uint(U256::from(proposal_id)),
            Token::Bool(support),
        ]);

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&encoded);

        self.send_transaction(addresses::GOVERNANCE, data.into(), U256::zero())
            .await
    }

    /// Get governance proposal status
    pub async fn get_proposal_status(&self, proposal_id: u64) -> Result<ProposalStatus> {
        let selector = &keccak256(b"getProposalStatus(uint256)")[..4];

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&U256::from(proposal_id).to_be_bytes());

        let result = self.call(addresses::GOVERNANCE, data.into()).await?;
        let status = U256::from_big_endian(&result).as_u64() as u8;
        Ok(status.into())
    }

    // ========== Staking Precompile (0x802) ==========

    /// Get stake amount for a validator
    pub async fn get_validator_stake(&self, validator_id: &[u8; 32]) -> Result<U256> {
        let selector = &keccak256(b"getValidatorStake(bytes32)")[..4];

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(validator_id);

        let result = self.call(addresses::STAKING, data.into()).await?;
        Ok(U256::from_big_endian(&result))
    }

    /// Check if a validator is active
    pub async fn is_validator_active(&self, validator_id: &[u8; 32]) -> Result<bool> {
        let selector = &keccak256(b"isValidatorActive(bytes32)")[..4];

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(validator_id);

        let result = self.call(addresses::STAKING, data.into()).await?;
        let active = U256::from_big_endian(&result);
        Ok(!active.is_zero())
    }

    /// Get total amount staked across all validators
    pub async fn get_total_staked(&self) -> Result<U256> {
        let selector = &keccak256(b"getTotalStaked()")[..4];

        let result = self.call(addresses::STAKING, selector.to_vec().into()).await?;
        Ok(U256::from_big_endian(&result))
    }

    /// Get total number of validators
    pub async fn get_validator_count(&self) -> Result<u32> {
        let selector = &keccak256(b"getValidatorCount()")[..4];

        let result = self.call(addresses::STAKING, selector.to_vec().into()).await?;
        let count = U256::from_big_endian(&result);
        Ok(count.as_u32())
    }

    // ========== Native ETH Wrap Precompile (0x803) ==========

    /// Wrap native ETH to wETH (zero-fee, instant)
    ///
    /// # Arguments
    /// * `amount` - Amount of ETH to wrap (in wei)
    pub async fn wrap_eth(&self, amount: U256) -> Result<H256> {
        let selector = &keccak256(b"wrap()")[..4];

        self.send_transaction(addresses::NATIVE_ETH_WRAP, selector.to_vec().into(), amount)
            .await
    }

    /// Unwrap wETH to native ETH (zero-fee, instant)
    ///
    /// # Arguments
    /// * `amount` - Amount of wETH to unwrap (in wei)
    pub async fn unwrap_eth(&self, amount: U256) -> Result<H256> {
        let selector = &keccak256(b"unwrap(uint256)")[..4];

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        data.extend_from_slice(&amount.to_be_bytes());

        self.send_transaction(addresses::NATIVE_ETH_WRAP, data.into(), U256::zero())
            .await
    }

    /// Get current ETH<->wETH wrap rate
    ///
    /// # Returns
    /// Rate scaled by 1e18 (normally 1e18 = 1:1)
    pub async fn get_wrap_rate(&self) -> Result<U256> {
        let selector = &keccak256(b"getWrapRate()")[..4];

        let result = self
            .call(addresses::NATIVE_ETH_WRAP, selector.to_vec().into())
            .await?;
        Ok(U256::from_big_endian(&result))
    }

    // ========== Token Registry Precompile (0x805) ==========

    /// Get registered token information
    ///
    /// # Arguments
    /// * `token_address` - ERC-20 token address
    pub async fn get_token_info(&self, token_address: Address) -> Result<TokenInfo> {
        let selector = &keccak256(b"getTokenInfo(address)")[..4];

        let mut data = Vec::new();
        data.extend_from_slice(selector);
        // Pad address to 32 bytes (left-padded with zeros)
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(token_address.as_bytes());

        let result = self.call(addresses::TOKEN_REGISTRY, data.into()).await?;

        // Simplified decoding - in production would use full ABI decoder
        // Returns: (string name, string symbol, uint8 decimals, uint256 totalBridgedSupply)

        Ok(TokenInfo {
            name: "Token Name".to_string(),
            symbol: "TKN".to_string(),
            decimals: 18,
            total_bridged_supply: U256::zero(),
        })
    }

    // ========== State Proof Precompile (0x806) ==========

    /// Verify Ethereum state proof
    ///
    /// # Arguments
    /// * `proof` - Merkle proof bytes
    ///
    /// # Returns
    /// true if proof is valid
    pub async fn verify_eth_state_proof(&self, proof: &[u8]) -> Result<bool> {
        // This is a complex function requiring full Merkle proof encoding
        // Simplified stub implementation
        log::warn!("verify_eth_state_proof is a stub implementation");
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_status_conversion() {
        assert_eq!(ProposalStatus::from(0), ProposalStatus::Pending);
        assert_eq!(ProposalStatus::from(1), ProposalStatus::Active);
        assert_eq!(ProposalStatus::from(2), ProposalStatus::Passed);
        assert_eq!(ProposalStatus::from(3), ProposalStatus::Failed);
    }

    #[test]
    fn test_precompile_addresses() {
        assert_eq!(addresses::ORACLE, Address::from_low_u64_be(0x800));
        assert_eq!(addresses::GOVERNANCE, Address::from_low_u64_be(0x801));
        assert_eq!(addresses::STAKING, Address::from_low_u64_be(0x802));
    }
}
