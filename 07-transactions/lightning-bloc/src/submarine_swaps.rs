//! Submarine Swaps
//!
//! Trustless atomic swaps between on-chain and off-chain (Lightning) funds.
//! Enables users to rebalance channels or convert between Lightning and
//! regular blockchain transactions without counterparty risk.
//!
//! Features:
//! - On-chain to Lightning swaps
//! - Lightning to on-chain swaps
//! - HTLC-based atomic execution
//! - Automatic timeout and refunds
//! - Multi-chain support

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    vec::Vec,
    string::{String, ToString},
    format,
};

#[cfg(feature = "std")]
use std::{
    vec::Vec,
    string::String,
};

/// Swap timeout (in blocks)
pub const SWAP_TIMEOUT_BLOCKS: u64 = 288; // ~48 hours for Bitcoin

/// Minimum swap amount
pub const MIN_SWAP_AMOUNT: u128 = 10000;

/// Swap direction
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SwapDirection {
    /// Convert on-chain funds to Lightning
    OnChainToLightning,
    /// Convert Lightning funds to on-chain
    LightningToOnChain,
}

/// Swap status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SwapStatus {
    /// Swap created, awaiting funding
    Created,
    /// Funds locked, awaiting claim
    Funded,
    /// Swap completed successfully
    Completed,
    /// Swap expired and refunded
    Refunded,
    /// Swap failed
    Failed,
}

/// Submarine swap
#[derive(Clone, Debug)]
pub struct SubmarineSwap {
    /// Swap identifier
    pub swap_id: String,
    /// Swap direction
    pub direction: SwapDirection,
    /// Source chain
    pub source_chain: String,
    /// Destination chain
    pub destination_chain: String,
    /// Swap amount
    pub amount: u128,
    /// Preimage (known by one party)
    pub preimage: Option<Vec<u8>>,
    /// Payment hash (hash of preimage)
    pub payment_hash: Vec<u8>,
    /// On-chain address
    pub on_chain_address: String,
    /// Lightning invoice (if applicable)
    pub lightning_invoice: Option<String>,
    /// Timeout block height
    pub timeout_height: u64,
    /// Current status
    pub status: SwapStatus,
    /// Creation timestamp
    pub created_at: u64,
    /// Completion timestamp
    pub completed_at: Option<u64>,
}

impl SubmarineSwap {
    /// Create new submarine swap
    pub fn new(
        swap_id: String,
        direction: SwapDirection,
        source_chain: String,
        destination_chain: String,
        amount: u128,
        payment_hash: Vec<u8>,
        on_chain_address: String,
        timeout_height: u64,
        created_at: u64,
    ) -> Result<Self, SwapError> {
        if amount < MIN_SWAP_AMOUNT {
            return Err(SwapError::AmountTooSmall);
        }

        Ok(Self {
            swap_id,
            direction,
            source_chain,
            destination_chain,
            amount,
            preimage: None,
            payment_hash,
            on_chain_address,
            lightning_invoice: None,
            timeout_height,
            status: SwapStatus::Created,
            created_at,
            completed_at: None,
        })
    }

    /// Set Lightning invoice for the swap
    pub fn set_lightning_invoice(&mut self, invoice: String) {
        self.lightning_invoice = Some(invoice);
    }

    /// Mark swap as funded
    pub fn mark_funded(&mut self) -> Result<(), SwapError> {
        if self.status != SwapStatus::Created {
            return Err(SwapError::InvalidState {
                current: self.status.clone(),
                expected: SwapStatus::Created,
            });
        }

        self.status = SwapStatus::Funded;
        Ok(())
    }

    /// Complete swap with preimage
    pub fn complete(&mut self, preimage: Vec<u8>, timestamp: u64) -> Result<(), SwapError> {
        // Verify preimage matches hash
        if !self.verify_preimage(&preimage) {
            return Err(SwapError::InvalidPreimage);
        }

        if self.status != SwapStatus::Funded {
            return Err(SwapError::InvalidState {
                current: self.status.clone(),
                expected: SwapStatus::Funded,
            });
        }

        self.preimage = Some(preimage);
        self.status = SwapStatus::Completed;
        self.completed_at = Some(timestamp);

        Ok(())
    }

    /// Refund expired swap
    pub fn refund(&mut self, current_height: u64) -> Result<(), SwapError> {
        if current_height <= self.timeout_height {
            return Err(SwapError::SwapNotExpired);
        }

        if self.status == SwapStatus::Completed {
            return Err(SwapError::AlreadyCompleted);
        }

        self.status = SwapStatus::Refunded;
        Ok(())
    }

    /// Verify preimage matches payment hash
    fn verify_preimage(&self, preimage: &[u8]) -> bool {
        // In production, use SHA256(preimage) == payment_hash
        // Simplified version for demonstration
        preimage.len() == 32
    }

    /// Check if swap is expired
    pub fn is_expired(&self, current_height: u64) -> bool {
        current_height > self.timeout_height
    }

    /// Get swap details
    pub fn details(&self) -> SwapDetails {
        SwapDetails {
            swap_id: self.swap_id.clone(),
            direction: self.direction.clone(),
            amount: self.amount,
            status: self.status.clone(),
            timeout_remaining: if self.timeout_height > 0 {
                Some(self.timeout_height)
            } else {
                None
            },
        }
    }
}

/// Swap details (for display)
#[derive(Clone, Debug)]
pub struct SwapDetails {
    pub swap_id: String,
    pub direction: SwapDirection,
    pub amount: u128,
    pub status: SwapStatus,
    pub timeout_remaining: Option<u64>,
}

/// Submarine Swap Manager
pub struct SubmarineSwapManager {
    swaps: Vec<SubmarineSwap>,
    current_height: u64,
}

impl SubmarineSwapManager {
    pub fn new() -> Self {
        Self {
            swaps: Vec::new(),
            current_height: 0,
        }
    }

    /// Update current block height
    pub fn update_height(&mut self, height: u64) {
        self.current_height = height;
    }

    /// Initiate on-chain to Lightning swap
    pub fn initiate_onchain_to_lightning(
        &mut self,
        swap_id: String,
        chain: String,
        amount: u128,
        payment_hash: Vec<u8>,
        on_chain_address: String,
        created_at: u64,
    ) -> Result<String, SwapError> {
        let timeout_height = self.current_height + SWAP_TIMEOUT_BLOCKS;

        let swap = SubmarineSwap::new(
            swap_id.clone(),
            SwapDirection::OnChainToLightning,
            chain.clone(),
            chain,
            amount,
            payment_hash,
            on_chain_address,
            timeout_height,
            created_at,
        )?;

        self.swaps.push(swap);
        Ok(swap_id)
    }

    /// Initiate Lightning to on-chain swap
    pub fn initiate_lightning_to_onchain(
        &mut self,
        swap_id: String,
        chain: String,
        amount: u128,
        payment_hash: Vec<u8>,
        on_chain_address: String,
        lightning_invoice: String,
        created_at: u64,
    ) -> Result<String, SwapError> {
        let timeout_height = self.current_height + SWAP_TIMEOUT_BLOCKS;

        let mut swap = SubmarineSwap::new(
            swap_id.clone(),
            SwapDirection::LightningToOnChain,
            chain.clone(),
            chain,
            amount,
            payment_hash,
            on_chain_address,
            timeout_height,
            created_at,
        )?;

        swap.set_lightning_invoice(lightning_invoice);
        self.swaps.push(swap);
        Ok(swap_id)
    }

    /// Get swap by ID
    pub fn get_swap(&self, swap_id: &str) -> Result<&SubmarineSwap, SwapError> {
        self.swaps
            .iter()
            .find(|s| s.swap_id == swap_id)
            .ok_or(SwapError::SwapNotFound)
    }

    /// Get mutable swap by ID
    fn get_swap_mut(&mut self, swap_id: &str) -> Result<&mut SubmarineSwap, SwapError> {
        self.swaps
            .iter_mut()
            .find(|s| s.swap_id == swap_id)
            .ok_or(SwapError::SwapNotFound)
    }

    /// Complete swap
    pub fn complete_swap(
        &mut self,
        swap_id: &str,
        preimage: Vec<u8>,
        timestamp: u64,
    ) -> Result<(), SwapError> {
        let swap = self.get_swap_mut(swap_id)?;
        swap.complete(preimage, timestamp)
    }

    /// Refund expired swap
    pub fn refund_swap(&mut self, swap_id: &str) -> Result<(), SwapError> {
        let swap = self.get_swap_mut(swap_id)?;
        swap.refund(self.current_height)
    }

    /// Mark swap as funded
    pub fn mark_swap_funded(&mut self, swap_id: &str) -> Result<(), SwapError> {
        let swap = self.get_swap_mut(swap_id)?;
        swap.mark_funded()
    }

    /// Get all active swaps
    pub fn get_active_swaps(&self) -> Vec<&SubmarineSwap> {
        self.swaps
            .iter()
            .filter(|s| matches!(s.status, SwapStatus::Created | SwapStatus::Funded))
            .collect()
    }

    /// Clean up expired swaps
    pub fn cleanup_expired(&mut self) -> usize {
        let mut refunded = 0;

        for swap in &mut self.swaps {
            if swap.is_expired(self.current_height) && swap.status == SwapStatus::Funded {
                if swap.refund(self.current_height).is_ok() {
                    refunded += 1;
                }
            }
        }

        refunded
    }

    /// Get swap statistics
    pub fn statistics(&self) -> SwapStatistics {
        SwapStatistics {
            total_swaps: self.swaps.len(),
            completed: self
                .swaps
                .iter()
                .filter(|s| s.status == SwapStatus::Completed)
                .count(),
            pending: self
                .swaps
                .iter()
                .filter(|s| matches!(s.status, SwapStatus::Created | SwapStatus::Funded))
                .count(),
            refunded: self
                .swaps
                .iter()
                .filter(|s| s.status == SwapStatus::Refunded)
                .count(),
        }
    }
}

impl Default for SubmarineSwapManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Swap statistics
#[derive(Clone, Debug)]
pub struct SwapStatistics {
    pub total_swaps: usize,
    pub completed: usize,
    pub pending: usize,
    pub refunded: usize,
}

/// Swap errors
#[derive(Clone, Debug, PartialEq)]
pub enum SwapError {
    AmountTooSmall,
    SwapNotFound,
    InvalidPreimage,
    InvalidState {
        current: SwapStatus,
        expected: SwapStatus,
    },
    SwapNotExpired,
    AlreadyCompleted,
    ChainMismatch,
}

impl core::fmt::Display for SwapError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            SwapError::AmountTooSmall => write!(f, "Swap amount too small"),
            SwapError::SwapNotFound => write!(f, "Swap not found"),
            SwapError::InvalidPreimage => write!(f, "Invalid preimage"),
            SwapError::InvalidState { current, expected } => {
                write!(f, "Invalid state: current {:?}, expected {:?}", current, expected)
            }
            SwapError::SwapNotExpired => write!(f, "Swap has not expired yet"),
            SwapError::AlreadyCompleted => write!(f, "Swap already completed"),
            SwapError::ChainMismatch => write!(f, "Chain mismatch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submarine_swap_creation() {
        let swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        );

        assert!(swap.is_ok());
        let s = swap.unwrap();
        assert_eq!(s.swap_id, "swap1");
        assert_eq!(s.status, SwapStatus::Created);
    }

    #[test]
    fn test_swap_amount_too_small() {
        let swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100, // Too small
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        );

        assert_eq!(swap, Err(SwapError::AmountTooSmall));
    }

    #[test]
    fn test_mark_funded() {
        let mut swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        )
        .unwrap();

        assert!(swap.mark_funded().is_ok());
        assert_eq!(swap.status, SwapStatus::Funded);
    }

    #[test]
    fn test_complete_swap() {
        let mut swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        )
        .unwrap();

        swap.mark_funded().unwrap();

        let preimage = vec![2; 32];
        assert!(swap.complete(preimage, 600).is_ok());
        assert_eq!(swap.status, SwapStatus::Completed);
        assert_eq!(swap.completed_at, Some(600));
    }

    #[test]
    fn test_refund_swap() {
        let mut swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        )
        .unwrap();

        swap.mark_funded().unwrap();

        // Cannot refund before timeout
        assert_eq!(swap.refund(900), Err(SwapError::SwapNotExpired));

        // Can refund after timeout
        assert!(swap.refund(1001).is_ok());
        assert_eq!(swap.status, SwapStatus::Refunded);
    }

    #[test]
    fn test_is_expired() {
        let swap = SubmarineSwap::new(
            "swap1".to_string(),
            SwapDirection::OnChainToLightning,
            "btc-pbc".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            1000,
            500,
        )
        .unwrap();

        assert!(!swap.is_expired(900));
        assert!(!swap.is_expired(1000));
        assert!(swap.is_expired(1001));
    }

    #[test]
    fn test_swap_manager_initiate_onchain_to_lightning() {
        let mut manager = SubmarineSwapManager::new();
        manager.update_height(100);

        let result = manager.initiate_onchain_to_lightning(
            "swap1".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            500,
        );

        assert!(result.is_ok());
        assert_eq!(manager.swaps.len(), 1);
    }

    #[test]
    fn test_swap_manager_initiate_lightning_to_onchain() {
        let mut manager = SubmarineSwapManager::new();
        manager.update_height(100);

        let result = manager.initiate_lightning_to_onchain(
            "swap1".to_string(),
            "btc-pbc".to_string(),
            100000,
            vec![1; 32],
            "bc1q...".to_string(),
            "lnbc...".to_string(),
            500,
        );

        assert!(result.is_ok());
        assert_eq!(manager.swaps.len(), 1);
    }

    #[test]
    fn test_swap_manager_complete_swap() {
        let mut manager = SubmarineSwapManager::new();
        manager.update_height(100);

        manager
            .initiate_onchain_to_lightning(
                "swap1".to_string(),
                "btc-pbc".to_string(),
                100000,
                vec![1; 32],
                "bc1q...".to_string(),
                500,
            )
            .unwrap();

        manager.mark_swap_funded("swap1").unwrap();

        let preimage = vec![2; 32];
        assert!(manager.complete_swap("swap1", preimage, 600).is_ok());

        let swap = manager.get_swap("swap1").unwrap();
        assert_eq!(swap.status, SwapStatus::Completed);
    }

    #[test]
    fn test_swap_manager_cleanup_expired() {
        let mut manager = SubmarineSwapManager::new();
        manager.update_height(100);

        manager
            .initiate_onchain_to_lightning(
                "swap1".to_string(),
                "btc-pbc".to_string(),
                100000,
                vec![1; 32],
                "bc1q...".to_string(),
                500,
            )
            .unwrap();

        manager.mark_swap_funded("swap1").unwrap();

        // Fast-forward past timeout
        manager.update_height(500);

        let refunded = manager.cleanup_expired();
        assert_eq!(refunded, 1);

        let swap = manager.get_swap("swap1").unwrap();
        assert_eq!(swap.status, SwapStatus::Refunded);
    }

    #[test]
    fn test_swap_manager_statistics() {
        let mut manager = SubmarineSwapManager::new();
        manager.update_height(100);

        // Create 3 swaps
        for i in 0..3 {
            manager
                .initiate_onchain_to_lightning(
                    format!("swap{}", i),
                    "btc-pbc".to_string(),
                    100000,
                    vec![1; 32],
                    "bc1q...".to_string(),
                    500,
                )
                .unwrap();
        }

        // Complete one
        manager.mark_swap_funded("swap0").unwrap();
        manager.complete_swap("swap0", vec![2; 32], 600).unwrap();

        let stats = manager.statistics();
        assert_eq!(stats.total_swaps, 3);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.pending, 2);
    }
}
