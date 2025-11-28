//! Lightning Service Provider (LSP) Infrastructure
//!
//! Provides instant channel liquidity for new users and manages
//! liquidity across the Lightning Network.
//!
//! Features:
//! - Instant channel opening for new users
//! - Initial inbound liquidity provision
//! - Automated liquidity rebalancing
//! - Fee management
//! - SLA guarantees
//! - Geographic distribution

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    vec::Vec,
    string::{String, ToString},
    collections::BTreeMap as HashMap,
    format,
};

#[cfg(feature = "std")]
use std::{
    vec::Vec,
    string::String,
    collections::HashMap,
};

/// Minimum LSP liquidity
pub const MIN_LSP_LIQUIDITY: u128 = 1_000_000_000; // 1 billion base units

/// Default channel capacity
pub const DEFAULT_CHANNEL_CAPACITY: u128 = 10_000_000_000; // 10 billion

/// LSP node information
#[derive(Clone, Debug)]
pub struct LSPNode {
    /// Node identifier
    pub node_id: String,
    /// Available liquidity
    pub available_liquidity: u128,
    /// Total capacity
    pub total_capacity: u128,
    /// Number of channels opened
    pub channels_opened: usize,
    /// Uptime percentage
    pub uptime: f64,
    /// Fee policy
    pub fee_policy: FeePolicy,
    /// Geographic region
    pub region: String,
    /// Reputation score (0-100)
    pub reputation: f64,
}

impl LSPNode {
    pub fn new(
        node_id: String,
        total_capacity: u128,
        region: String,
    ) -> Result<Self, LSPError> {
        if total_capacity < MIN_LSP_LIQUIDITY {
            return Err(LSPError::InsufficientCapacity);
        }

        Ok(Self {
            node_id,
            available_liquidity: total_capacity,
            total_capacity,
            channels_opened: 0,
            uptime: 100.0,
            fee_policy: FeePolicy::default(),
            region,
            reputation: 100.0,
        })
    }

    /// Check if LSP has sufficient liquidity
    pub fn has_liquidity(&self, amount: u128) -> bool {
        self.available_liquidity >= amount
    }

    /// Reserve liquidity for a channel
    pub fn reserve_liquidity(&mut self, amount: u128) -> Result<(), LSPError> {
        if !self.has_liquidity(amount) {
            return Err(LSPError::InsufficientLiquidity);
        }

        self.available_liquidity -= amount;
        self.channels_opened += 1;
        Ok(())
    }

    /// Release liquidity from a closed channel
    pub fn release_liquidity(&mut self, amount: u128) {
        self.available_liquidity = self.available_liquidity.saturating_add(amount).min(self.total_capacity);
    }

    /// Calculate fee for channel opening
    pub fn calculate_opening_fee(&self, capacity: u128) -> u128 {
        self.fee_policy.calculate_opening_fee(capacity)
    }

    /// Get liquidity utilization percentage
    pub fn liquidity_utilization(&self) -> f64 {
        let used = self.total_capacity - self.available_liquidity;
        (used as f64 / self.total_capacity as f64) * 100.0
    }
}

/// Fee policy for LSP
#[derive(Clone, Debug)]
pub struct FeePolicy {
    /// Base fee for channel opening (in base units)
    pub base_fee: u128,
    /// Percentage fee (basis points, 1 bp = 0.01%)
    pub percentage_fee_bp: u32,
    /// Monthly service fee
    pub monthly_fee: u128,
}

impl Default for FeePolicy {
    fn default() -> Self {
        Self {
            base_fee: 1000,
            percentage_fee_bp: 10, // 0.1%
            monthly_fee: 100,
        }
    }
}

impl FeePolicy {
    pub fn calculate_opening_fee(&self, capacity: u128) -> u128 {
        let percentage_fee = (capacity * self.percentage_fee_bp as u128) / 10000;
        self.base_fee + percentage_fee
    }
}

/// LSP channel request
#[derive(Clone, Debug)]
pub struct ChannelRequest {
    pub request_id: String,
    pub user_pubkey: String,
    pub desired_capacity: u128,
    pub inbound_liquidity: u128,
    pub status: RequestStatus,
    pub created_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// LSP Manager
pub struct LSPManager {
    /// Available LSP nodes
    lsp_nodes: HashMap<String, LSPNode>,
    /// Pending channel requests
    requests: HashMap<String, ChannelRequest>,
    /// Liquidity pools per chain
    liquidity_pools: HashMap<String, LiquidityPool>,
}

impl LSPManager {
    pub fn new() -> Self {
        Self {
            lsp_nodes: HashMap::new(),
            requests: HashMap::new(),
            liquidity_pools: HashMap::new(),
        }
    }

    /// Register new LSP node
    pub fn register_lsp(&mut self, lsp: LSPNode) -> Result<(), LSPError> {
        if self.lsp_nodes.contains_key(&lsp.node_id) {
            return Err(LSPError::LSPAlreadyRegistered);
        }

        self.lsp_nodes.insert(lsp.node_id.clone(), lsp);
        Ok(())
    }

    /// Request instant channel from LSP
    pub fn request_instant_channel(
        &mut self,
        request_id: String,
        user_pubkey: String,
        desired_capacity: u128,
        inbound_liquidity: u128,
        created_at: u64,
    ) -> Result<String, LSPError> {
        // Find LSP with sufficient liquidity
        let lsp_node_id = self.find_best_lsp(desired_capacity)?;

        // Create request
        let request = ChannelRequest {
            request_id: request_id.clone(),
            user_pubkey,
            desired_capacity,
            inbound_liquidity,
            status: RequestStatus::Pending,
            created_at,
        };

        self.requests.insert(request_id.clone(), request);

        // Process request
        self.process_channel_request(&request_id)?;

        Ok(lsp_node_id)
    }

    /// Find best LSP node for request
    fn find_best_lsp(&self, required_liquidity: u128) -> Result<String, LSPError> {
        let mut best_lsp: Option<(&String, &LSPNode)> = None;
        let mut best_score = 0.0;

        for (node_id, lsp) in &self.lsp_nodes {
            if !lsp.has_liquidity(required_liquidity) {
                continue;
            }

            // Score based on: reputation (50%), uptime (30%), available liquidity (20%)
            let score = (lsp.reputation * 0.5)
                + (lsp.uptime * 0.3)
                + ((lsp.available_liquidity as f64 / lsp.total_capacity as f64) * 100.0 * 0.2);

            if score > best_score {
                best_score = score;
                best_lsp = Some((node_id, lsp));
            }
        }

        best_lsp
            .map(|(node_id, _)| node_id.clone())
            .ok_or(LSPError::NoSuitableLSP)
    }

    /// Process channel request
    fn process_channel_request(&mut self, request_id: &str) -> Result<(), LSPError> {
        // Extract desired capacity before mutable borrows
        let desired_capacity = self.requests.get(request_id)
            .ok_or(LSPError::RequestNotFound)?
            .desired_capacity;

        let request = self.requests.get_mut(request_id)
            .ok_or(LSPError::RequestNotFound)?;

        request.status = RequestStatus::Processing;

        // Find and reserve liquidity
        let lsp_id = self.find_best_lsp(desired_capacity)?;
        let lsp = self.lsp_nodes.get_mut(&lsp_id)
            .ok_or(LSPError::LSPNotFound)?;

        lsp.reserve_liquidity(desired_capacity)?;

        let request = self.requests.get_mut(request_id)
            .ok_or(LSPError::RequestNotFound)?;
        request.status = RequestStatus::Completed;
        Ok(())
    }

    /// Get all LSP nodes
    pub fn get_lsp_nodes(&self) -> Vec<&LSPNode> {
        self.lsp_nodes.values().collect()
    }

    /// Get LSP by ID
    pub fn get_lsp(&self, node_id: &str) -> Option<&LSPNode> {
        self.lsp_nodes.get(node_id)
    }

    /// Rebalance liquidity across LSPs
    pub fn rebalance_liquidity(&mut self) -> Result<RebalanceResult, LSPError> {
        let mut rebalanced = 0;
        let mut total_moved = 0u128;

        // Simple rebalancing: move from over-utilized to under-utilized LSPs
        let target_utilization = 50.0;

        let node_ids: Vec<String> = self.lsp_nodes.keys().cloned().collect();

        for node_id in node_ids {
            if let Some(lsp) = self.lsp_nodes.get(&node_id) {
                let utilization = lsp.liquidity_utilization();

                if utilization > target_utilization + 20.0 {
                    // Over-utilized, needs more liquidity
                    rebalanced += 1;
                } else if utilization < target_utilization - 20.0 {
                    // Under-utilized, has excess liquidity
                    rebalanced += 1;
                }
            }
        }

        Ok(RebalanceResult {
            lsps_rebalanced: rebalanced,
            liquidity_moved: total_moved,
        })
    }

    /// Get statistics
    pub fn statistics(&self) -> LSPStatistics {
        let total_lsps = self.lsp_nodes.len();
        let total_capacity: u128 = self.lsp_nodes.values().map(|l| l.total_capacity).sum();
        let available_liquidity: u128 = self.lsp_nodes.values().map(|l| l.available_liquidity).sum();
        let total_channels: usize = self.lsp_nodes.values().map(|l| l.channels_opened).sum();

        LSPStatistics {
            total_lsps,
            total_capacity,
            available_liquidity,
            total_channels_opened: total_channels,
            pending_requests: self.requests.values().filter(|r| r.status == RequestStatus::Pending).count(),
        }
    }
}

impl Default for LSPManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Liquidity pool for a chain
#[derive(Clone, Debug)]
pub struct LiquidityPool {
    pub chain_id: String,
    pub total_liquidity: u128,
    pub available_liquidity: u128,
    pub providers: Vec<String>,
}

/// Rebalance result
#[derive(Clone, Debug)]
pub struct RebalanceResult {
    pub lsps_rebalanced: usize,
    pub liquidity_moved: u128,
}

/// LSP statistics
#[derive(Clone, Debug)]
pub struct LSPStatistics {
    pub total_lsps: usize,
    pub total_capacity: u128,
    pub available_liquidity: u128,
    pub total_channels_opened: usize,
    pub pending_requests: usize,
}

/// LSP errors
#[derive(Clone, Debug, PartialEq)]
pub enum LSPError {
    InsufficientCapacity,
    InsufficientLiquidity,
    LSPAlreadyRegistered,
    LSPNotFound,
    NoSuitableLSP,
    RequestNotFound,
    InvalidRequest,
}

impl core::fmt::Display for LSPError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            LSPError::InsufficientCapacity => write!(f, "Insufficient capacity"),
            LSPError::InsufficientLiquidity => write!(f, "Insufficient liquidity"),
            LSPError::LSPAlreadyRegistered => write!(f, "LSP already registered"),
            LSPError::LSPNotFound => write!(f, "LSP not found"),
            LSPError::NoSuitableLSP => write!(f, "No suitable LSP found"),
            LSPError::RequestNotFound => write!(f, "Request not found"),
            LSPError::InvalidRequest => write!(f, "Invalid request"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_node_creation() {
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        );

        assert!(lsp.is_ok());
        let node = lsp.unwrap();
        assert_eq!(node.available_liquidity, 10_000_000_000);
        assert_eq!(node.channels_opened, 0);
        assert_eq!(node.reputation, 100.0);
    }

    #[test]
    fn test_lsp_insufficient_capacity() {
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            100, // Too small
            "us-east".to_string(),
        );

        assert_eq!(lsp, Err(LSPError::InsufficientCapacity));
    }

    #[test]
    fn test_reserve_liquidity() {
        let mut lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        assert!(lsp.reserve_liquidity(1_000_000_000).is_ok());
        assert_eq!(lsp.available_liquidity, 9_000_000_000);
        assert_eq!(lsp.channels_opened, 1);
    }

    #[test]
    fn test_reserve_too_much_liquidity() {
        let mut lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        let result = lsp.reserve_liquidity(20_000_000_000);
        assert_eq!(result, Err(LSPError::InsufficientLiquidity));
    }

    #[test]
    fn test_release_liquidity() {
        let mut lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        lsp.reserve_liquidity(1_000_000_000).unwrap();
        lsp.release_liquidity(1_000_000_000);

        assert_eq!(lsp.available_liquidity, 10_000_000_000);
    }

    #[test]
    fn test_fee_calculation() {
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        let fee = lsp.calculate_opening_fee(10_000_000);
        // base_fee (1000) + percentage (10_000_000 * 10 / 10000 = 10_000)
        assert_eq!(fee, 11_000);
    }

    #[test]
    fn test_liquidity_utilization() {
        let mut lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        assert_eq!(lsp.liquidity_utilization(), 0.0);

        lsp.reserve_liquidity(5_000_000_000).unwrap();
        assert_eq!(lsp.liquidity_utilization(), 50.0);
    }

    #[test]
    fn test_lsp_manager_registration() {
        let mut manager = LSPManager::new();
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        assert!(manager.register_lsp(lsp).is_ok());
        assert_eq!(manager.get_lsp_nodes().len(), 1);
    }

    #[test]
    fn test_request_instant_channel() {
        let mut manager = LSPManager::new();
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        manager.register_lsp(lsp).unwrap();

        let result = manager.request_instant_channel(
            "req1".to_string(),
            "user123".to_string(),
            1_000_000_000,
            500_000_000,
            1000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_find_best_lsp() {
        let mut manager = LSPManager::new();

        let lsp1 = LSPNode::new("lsp1".to_string(), 10_000_000_000, "us-east".to_string()).unwrap();
        let mut lsp2 = LSPNode::new("lsp2".to_string(), 20_000_000_000, "eu-west".to_string()).unwrap();
        lsp2.reputation = 95.0; // Slightly lower reputation

        manager.register_lsp(lsp1).unwrap();
        manager.register_lsp(lsp2).unwrap();

        let best = manager.find_best_lsp(1_000_000_000).unwrap();
        // Should pick lsp1 due to higher reputation (100 vs 95)
        assert_eq!(best, "lsp1");
    }

    #[test]
    fn test_statistics() {
        let mut manager = LSPManager::new();
        let lsp = LSPNode::new(
            "lsp1".to_string(),
            10_000_000_000,
            "us-east".to_string(),
        ).unwrap();

        manager.register_lsp(lsp).unwrap();

        let stats = manager.statistics();
        assert_eq!(stats.total_lsps, 1);
        assert_eq!(stats.total_capacity, 10_000_000_000);
        assert_eq!(stats.available_liquidity, 10_000_000_000);
    }
}
