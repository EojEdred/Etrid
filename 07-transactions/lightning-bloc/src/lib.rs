//! Lightning-Bloc: Layer 2 Payment Channel Implementation for ÉTRID
//!
//! Enables fast, off-chain payments with on-chain settlement:
//! - Bidirectional payment channels
//! - Multi-hop routing and pathfinding
//! - State update signatures
//! - Dispute resolution mechanism
//! - Settlement finality
//! - Channel lifecycle management
//! - Watchtower incentive mechanisms
//! - Multi-party state channels
//! - Transaction batching and compression
//! - Optimistic rollup integration
//! - Fraud proof system
//! - Emergency withdrawal procedures

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod routing;
pub mod watchtower;
pub mod fraud_proofs;
pub mod multi_party;
pub mod batching;
pub mod optimistic_rollup;
pub mod emergency;
pub mod auto_discovery;
pub mod invoice;
pub mod multi_path_payments;
pub mod submarine_swaps;
pub mod lsp;
pub mod rebalancing;
pub mod streaming;
pub mod recurring;
pub mod gossip;

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    string::{String, ToString},
    vec,
    vec::Vec,
    format,
};

#[cfg(not(feature = "std"))]
use core::{
    fmt,
    default::Default,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
};

#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    fmt,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self, Some, None},
    default::Default,
};

// Re-export routing types
pub use routing::{
    NetworkGraph, Router, Route, RouteHop, ChannelEdge,
    RoutingError, NodeId, ChannelId,
};

// Re-export watchtower types
pub use watchtower::{
    WatchtowerManager, WatchtowerInfo, WatchtowerError,
    ChannelSubscription, FraudEvidence, FraudReport,
    FraudResolution, WatchtowerStatistics,
    MIN_WATCHTOWER_STAKE, WATCHTOWER_BASE_REWARD,
    FRAUD_REWARD_PERCENTAGE,
};

// Re-export fraud proof types
pub use fraud_proofs::{
    FraudProofSystem, FraudProof, ChallengeResponse,
    Dispute as FraudDispute, StateTransition, DisputeResolution,
    FraudProofError, FraudProofStatistics,
    CHALLENGE_PERIOD, RESPONSE_PERIOD,
};

// Re-export multi-party channel types
pub use multi_party::{
    MultiPartyChannel, MultiPartyChannelManager, Participant,
    Signature, MultiPartyStatistics, MultiPartyError,
    MAX_PARTIES, DEFAULT_CONSENSUS_THRESHOLD,
};

// Re-export batching types
pub use batching::{
    BatchingManager, TransactionBatch, OffChainTransaction,
    BatchSettlement, CompressionResult, BatchingError,
    BatchingStatistics, MAX_BATCH_SIZE, MAX_BATCH_AGE,
};

// Re-export optimistic rollup types
pub use optimistic_rollup::{
    OptimisticRollup, StateCommitment, L2Block, Sequencer,
    CommitmentStatus, RollupStatistics, RollupError,
    OPTIMISTIC_CHALLENGE_PERIOD,
};

// Re-export emergency withdrawal types
pub use emergency::{
    EmergencySystem, WithdrawalRequest, ForcedClosureRequest,
    TimeoutWatchdog, WithdrawalStatus, ClosureStatus,
    EmergencyStatistics, EmergencyError,
    COUNTERPARTY_TIMEOUT, FORCED_CLOSURE_GRACE_PERIOD,
};

// Re-export auto-discovery types
pub use auto_discovery::{
    PBCAutoDiscovery, PBCInfo, DiscoveryConfig, DiscoveryEvent,
    DiscoveryError, DiscoveryStatistics,
};

// Re-export invoice types
pub use invoice::{
    LightningInvoice, InvoiceBuilder, InvoiceError,
    PaymentRequest, InvoiceStatus,
};

// Re-export multi-path payment types
pub use multi_path_payments::{
    MultiPathPayment, PaymentPart, PartStatus, PaymentResult,
    MPPManager, MPPError, MAX_PAYMENT_PARTS,
};

// Re-export submarine swap types
pub use submarine_swaps::{
    SubmarineSwap, SubmarineSwapManager, SwapDirection, SwapStatus,
    SwapDetails, SwapStatistics, SwapError, SWAP_TIMEOUT_BLOCKS,
};

// Re-export LSP types
pub use lsp::{
    LSPManager, LSPNode, ChannelRequest, FeePolicy, LiquidityPool,
    LSPStatistics, LSPError, MIN_LSP_LIQUIDITY, DEFAULT_CHANNEL_CAPACITY,
};

// Re-export rebalancing types
pub use rebalancing::{
    ChannelRebalancer, ChannelBalance, RebalanceRecommendation,
    RebalanceDirection, Priority, RebalanceResult, RebalanceStatistics,
    RebalanceError, DEFAULT_TARGET_RATIO,
};

// Re-export streaming payment types
pub use streaming::{
    StreamingPayment, StreamManager, StreamStatus, StreamPayment,
    StreamStatistics, StreamError, MIN_RATE_PER_SECOND,
};

// Re-export recurring payment types
pub use recurring::{
    RecurringPayment, RecurringManager, PaymentFrequency, RecurringStatus,
    PaymentExecution, RecurringStatistics, RecurringError,
};

// Re-export gossip protocol types
pub use gossip::{
    GossipManager, GossipMessage, NodeAnnouncement, ChannelAnnouncement,
    ChannelUpdate as GossipChannelUpdate, ChannelDirection, SyncRequest,
    SyncResponse, NodeFeatures, GossipStatistics, GossipError,
};

/// Channel state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelState {
    Open,
    Suspended,
    Closing,
    Closed,
    Disputed,
    Settled,
}

impl fmt::Display for ChannelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelState::Open => write!(f, "Open"),
            ChannelState::Suspended => write!(f, "Suspended"),
            ChannelState::Closing => write!(f, "Closing"),
            ChannelState::Closed => write!(f, "Closed"),
            ChannelState::Disputed => write!(f, "Disputed"),
            ChannelState::Settled => write!(f, "Settled"),
        }
    }
}

/// Payment channel
#[derive(Clone, Debug, PartialEq)]
pub struct PaymentChannel {
    pub id: String,
    pub party_a: String,
    pub party_b: String,
    pub initial_balance_a: u128,
    pub initial_balance_b: u128,
    pub current_balance_a: u128,
    pub current_balance_b: u128,
    pub nonce: u64,
    pub state: ChannelState,
    pub created_at: u64,
    pub expires_at: u64,
}

impl PaymentChannel {
    /// Create new payment channel
    pub fn new(
        id: String,
        party_a: String,
        party_b: String,
        balance_a: u128,
        balance_b: u128,
        created_at: u64,
        expires_at: u64,
    ) -> Result<Self, ChannelError> {
        if party_a == party_b {
            return Err(ChannelError::SameParty);
        }
        if balance_a == 0 || balance_b == 0 {
            return Err(ChannelError::InvalidBalance);
        }
        if expires_at <= created_at {
            return Err(ChannelError::InvalidExpiration);
        }

        Ok(Self {
            id,
            party_a,
            party_b,
            initial_balance_a: balance_a,
            initial_balance_b: balance_b,
            current_balance_a: balance_a,
            current_balance_b: balance_b,
            nonce: 0,
            state: ChannelState::Open,
            created_at,
            expires_at,
        })
    }

    /// Execute payment from A to B
    pub fn pay_a_to_b(&mut self, amount: u128) -> Result<(), ChannelError> {
        if self.state != ChannelState::Open {
            return Err(ChannelError::ChannelNotOpen(self.state));
        }
        if amount == 0 {
            return Err(ChannelError::InvalidAmount);
        }
        if self.current_balance_a < amount {
            return Err(ChannelError::InsufficientBalance {
                have: self.current_balance_a,
                need: amount,
            });
        }

        self.current_balance_a -= amount;
        self.current_balance_b += amount;
        self.nonce += 1;

        Ok(())
    }

    /// Execute payment from B to A
    pub fn pay_b_to_a(&mut self, amount: u128) -> Result<(), ChannelError> {
        if self.state != ChannelState::Open {
            return Err(ChannelError::ChannelNotOpen(self.state));
        }
        if amount == 0 {
            return Err(ChannelError::InvalidAmount);
        }
        if self.current_balance_b < amount {
            return Err(ChannelError::InsufficientBalance {
                have: self.current_balance_b,
                need: amount,
            });
        }

        self.current_balance_b -= amount;
        self.current_balance_a += amount;
        self.nonce += 1;

        Ok(())
    }

    /// Verify balances are valid (sum equals initial total)
    pub fn verify_balances(&self) -> Result<(), ChannelError> {
        let current_total = self
            .current_balance_a
            .checked_add(self.current_balance_b)
            .ok_or(ChannelError::BalanceOverflow)?;

        let initial_total = self
            .initial_balance_a
            .checked_add(self.initial_balance_b)
            .ok_or(ChannelError::BalanceOverflow)?;

        if current_total == initial_total {
            Ok(())
        } else {
            Err(ChannelError::BalanceInvariantViolated {
                expected: initial_total,
                got: current_total,
            })
        }
    }

    /// Check if channel is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    /// Transition to next state
    pub fn transition(&mut self, new_state: ChannelState) -> Result<(), ChannelError> {
        let valid_next = match self.state {
            ChannelState::Open => vec![
                ChannelState::Suspended,
                ChannelState::Closing,
                ChannelState::Disputed,
            ],
            ChannelState::Suspended => vec![ChannelState::Open, ChannelState::Closing],
            ChannelState::Closing => vec![ChannelState::Closed, ChannelState::Disputed],
            ChannelState::Closed => vec![ChannelState::Settled],
            ChannelState::Disputed => vec![ChannelState::Settled, ChannelState::Closed],
            ChannelState::Settled => vec![],
        };

        if valid_next.contains(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(ChannelError::InvalidStateTransition {
                current: self.state,
                requested: new_state,
            })
        }
    }

    /// Get channel summary
    pub fn summary(&self) -> ChannelSummary {
        ChannelSummary {
            id: self.id.clone(),
            state: self.state,
            balance_a: self.current_balance_a,
            balance_b: self.current_balance_b,
            nonce: self.nonce,
        }
    }
}

/// Channel summary for compact representation
#[derive(Clone, Debug)]
pub struct ChannelSummary {
    pub id: String,
    pub state: ChannelState,
    pub balance_a: u128,
    pub balance_b: u128,
    pub nonce: u64,
}

/// Channel update signature
#[derive(Clone, Debug)]
pub struct ChannelUpdate {
    pub channel_id: String,
    pub nonce: u64,
    pub balance_a: u128,
    pub balance_b: u128,
    pub signature_a: Vec<u8>,
    pub signature_b: Option<Vec<u8>>,
    pub timestamp: u64,
}

impl ChannelUpdate {
    /// Create new channel update
    pub fn new(
        channel_id: String,
        nonce: u64,
        balance_a: u128,
        balance_b: u128,
        signature_a: Vec<u8>,
        timestamp: u64,
    ) -> Self {
        Self {
            channel_id,
            nonce,
            balance_a,
            balance_b,
            signature_a,
            signature_b: None,
            timestamp,
        }
    }

    /// Sign update by party B
    pub fn sign_by_b(&mut self, signature: Vec<u8>) {
        self.signature_b = Some(signature);
    }

    /// Check if update is fully signed
    pub fn is_fully_signed(&self) -> bool {
        self.signature_b.is_some()
    }
}

/// Settlement record
#[derive(Clone, Debug)]
pub struct Settlement {
    pub channel_id: String,
    pub final_balance_a: u128,
    pub final_balance_b: u128,
    pub nonce: u64,
    pub settlement_time: u64,
}

impl Settlement {
    pub fn new(
        channel_id: String,
        final_balance_a: u128,
        final_balance_b: u128,
        nonce: u64,
        settlement_time: u64,
    ) -> Self {
        Self {
            channel_id,
            final_balance_a,
            final_balance_b,
            nonce,
            settlement_time,
        }
    }
}

/// Dispute record
#[derive(Clone, Debug)]
pub struct Dispute {
    pub channel_id: String,
    pub complained_by: String,
    pub reason: DisputeReason,
    pub evidence: DisputeEvidence,
    pub created_at: u64,
    pub resolved: bool,
}

/// Dispute reason
#[derive(Clone, Debug)]
pub enum DisputeReason {
    InvalidStateUpdate,
    UnauthorizedPayment,
    BalanceViolation,
    ExpiredChannel,
    ForgeryDetected,
}

impl fmt::Display for DisputeReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisputeReason::InvalidStateUpdate => write!(f, "Invalid state update"),
            DisputeReason::UnauthorizedPayment => write!(f, "Unauthorized payment"),
            DisputeReason::BalanceViolation => write!(f, "Balance violation"),
            DisputeReason::ExpiredChannel => write!(f, "Expired channel"),
            DisputeReason::ForgeryDetected => write!(f, "Forgery detected"),
        }
    }
}

/// Dispute evidence
#[derive(Clone, Debug)]
pub struct DisputeEvidence {
    pub claimed_nonce: u64,
    pub claimed_balance_a: u128,
    pub claimed_balance_b: u128,
    pub witness: String,
}

impl DisputeEvidence {
    pub fn new(
        claimed_nonce: u64,
        claimed_balance_a: u128,
        claimed_balance_b: u128,
        witness: String,
    ) -> Self {
        Self {
            claimed_nonce,
            claimed_balance_a,
            claimed_balance_b,
            witness,
        }
    }
}

/// Channel errors
#[derive(Clone, Debug, PartialEq)]
pub enum ChannelError {
    SameParty,
    InvalidBalance,
    InvalidExpiration,
    InvalidAmount,
    ChannelNotOpen(ChannelState),
    ChannelNotFound(String),
    InsufficientBalance { have: u128, need: u128 },
    BalanceOverflow,
    BalanceInvariantViolated { expected: u128, got: u128 },
    InvalidStateTransition { current: ChannelState, requested: ChannelState },
    InvalidUpdateNonce { expected: u64, got: u64 },
    DisputeNotFound,
    SettlementNotFound(String),
    UpdateAlreadySigned,
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelError::SameParty => write!(f, "Same party for both sides"),
            ChannelError::InvalidBalance => write!(f, "Invalid balance"),
            ChannelError::InvalidExpiration => write!(f, "Invalid expiration time"),
            ChannelError::InvalidAmount => write!(f, "Invalid amount"),
            ChannelError::ChannelNotOpen(state) => write!(f, "Channel not open: {}", state),
            ChannelError::ChannelNotFound(id) => write!(f, "Channel not found: {}", id),
            ChannelError::InsufficientBalance { have, need } => {
                write!(f, "Insufficient balance: {} < {}", have, need)
            }
            ChannelError::BalanceOverflow => write!(f, "Balance overflow"),
            ChannelError::BalanceInvariantViolated { expected, got } => {
                write!(f, "Balance invariant violated: {} != {}", expected, got)
            }
            ChannelError::InvalidStateTransition { current, requested } => {
                write!(f, "Invalid state transition: {} -> {}", current, requested)
            }
            ChannelError::InvalidUpdateNonce { expected, got } => {
                write!(f, "Invalid update nonce: {} != {}", expected, got)
            }
            ChannelError::DisputeNotFound => write!(f, "Dispute not found"),
            ChannelError::SettlementNotFound(id) => write!(f, "Settlement not found: {}", id),
            ChannelError::UpdateAlreadySigned => write!(f, "Update already signed"),
        }
    }
}

/// Lightning-Bloc channel manager
pub struct LightningBloc {
    channels: HashMap<String, PaymentChannel>,
    updates: HashMap<String, Vec<ChannelUpdate>>,
    settlements: HashMap<String, Settlement>,
    disputes: HashMap<String, Dispute>,
}

impl LightningBloc {
    /// Create new Lightning-Bloc manager
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            updates: HashMap::new(),
            settlements: HashMap::new(),
            disputes: HashMap::new(),
        }
    }

    /// Open new payment channel
    pub fn open_channel(
        &mut self,
        channel: PaymentChannel,
    ) -> Result<String, ChannelError> {
        let id = channel.id.clone();
        self.channels.insert(id.clone(), channel);
        self.updates.insert(id.clone(), Vec::new());
        Ok(id)
    }

    /// Get channel
    pub fn get_channel(&self, id: &str) -> Result<PaymentChannel, ChannelError> {
        self.channels
            .get(id)
            .cloned()
            .ok_or_else(|| ChannelError::ChannelNotFound(id.to_string()))
    }

    /// Execute payment in channel
    pub fn execute_payment(
        &mut self,
        channel_id: &str,
        from_a_to_b: bool,
        amount: u128,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(channel_id.to_string()))?;

        if from_a_to_b {
            channel.pay_a_to_b(amount)?;
        } else {
            channel.pay_b_to_a(amount)?;
        }

        Ok(())
    }

    /// Submit channel update
    pub fn submit_update(
        &mut self,
        update: ChannelUpdate,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get(&update.channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(update.channel_id.clone()))?;

        if update.nonce != channel.nonce + 1 {
            return Err(ChannelError::InvalidUpdateNonce {
                expected: channel.nonce + 1,
                got: update.nonce,
            });
        }

        self.updates
            .entry(update.channel_id.clone())
            .or_default()
            .push(update);

        Ok(())
    }

    /// Sign existing update
    pub fn sign_update(&mut self, channel_id: &str, signature_b: Vec<u8>) -> Result<(), ChannelError> {
        let updates = self
            .updates
            .get_mut(channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(channel_id.to_string()))?;

        let last_update = updates
            .last_mut()
            .ok_or(ChannelError::InvalidAmount)?;

        if last_update.signature_b.is_some() {
            return Err(ChannelError::UpdateAlreadySigned);
        }

        last_update.sign_by_b(signature_b);
        Ok(())
    }

    /// Get channel updates
    pub fn get_updates(&self, channel_id: &str) -> Result<Vec<ChannelUpdate>, ChannelError> {
        self.updates
            .get(channel_id)
            .cloned()
            .ok_or_else(|| ChannelError::ChannelNotFound(channel_id.to_string()))
    }

    /// Settle channel
    pub fn settle_channel(
        &mut self,
        channel_id: &str,
        settlement: Settlement,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(channel_id.to_string()))?;

        channel.transition(ChannelState::Settled)?;
        self.settlements.insert(channel_id.to_string(), settlement);
        Ok(())
    }

    /// Get settlement
    pub fn get_settlement(&self, channel_id: &str) -> Result<Settlement, ChannelError> {
        self.settlements
            .get(channel_id)
            .cloned()
            .ok_or_else(|| ChannelError::SettlementNotFound(channel_id.to_string()))
    }

    /// File dispute
    pub fn file_dispute(
        &mut self,
        dispute: Dispute,
    ) -> Result<String, ChannelError> {
        let channel = self
            .channels
            .get_mut(&dispute.channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(dispute.channel_id.clone()))?;

        channel.transition(ChannelState::Disputed)?;

        let dispute_id = format!("{}_dispute_{}", dispute.channel_id, self.disputes.len());
        self.disputes.insert(dispute_id.clone(), dispute);
        Ok(dispute_id)
    }

    /// Get dispute
    pub fn get_dispute(&self, dispute_id: &str) -> Result<Dispute, ChannelError> {
        self.disputes
            .get(dispute_id)
            .cloned()
            .ok_or(ChannelError::DisputeNotFound)
    }

    /// Resolve dispute
    pub fn resolve_dispute(&mut self, dispute_id: &str) -> Result<(), ChannelError> {
        let dispute = self
            .disputes
            .get_mut(dispute_id)
            .ok_or(ChannelError::DisputeNotFound)?;

        dispute.resolved = true;
        Ok(())
    }

    /// Get active channels count
    pub fn active_channels_count(&self) -> usize {
        self.channels
            .values()
            .filter(|c| c.state == ChannelState::Open)
            .count()
    }

    /// Get total locked value
    pub fn total_locked_value(&self) -> u128 {
        self.channels
            .values()
            .map(|c| {
                c.current_balance_a
                    .checked_add(c.current_balance_b)
                    .unwrap_or(0)
            })
            .sum()
    }

    /// Transition channel state
    pub fn transition_channel_state(
        &mut self,
        channel_id: &str,
        new_state: ChannelState,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(channel_id)
            .ok_or_else(|| ChannelError::ChannelNotFound(channel_id.to_string()))?;

        channel.transition(new_state)
    }

    /// Verify channel balance invariants
    pub fn verify_channel(&self, channel_id: &str) -> Result<bool, ChannelError> {
        let channel = self.get_channel(channel_id)?;
        channel.verify_balances()?;
        Ok(true)
    }

    /// Get channel count by state
    pub fn channels_by_state(&self, state: ChannelState) -> usize {
        self.channels
            .values()
            .filter(|c| c.state == state)
            .count()
    }
}

impl Default for LightningBloc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_state_display() {
        assert_eq!(format!("{}", ChannelState::Open), "Open");
        assert_eq!(format!("{}", ChannelState::Closed), "Closed");
        assert_eq!(format!("{}", ChannelState::Settled), "Settled");
    }

    #[test]
    fn test_payment_channel_creation() {
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        );
        assert!(channel.is_ok());
    }

    #[test]
    fn test_payment_channel_same_party() {
        let result = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "alice".to_string(),
            1000,
            1000,
            100,
            200,
        );
        assert_eq!(result, Err(ChannelError::SameParty));
    }

    #[test]
    fn test_payment_channel_invalid_balance() {
        let result = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            0,
            1000,
            100,
            200,
        );
        assert_eq!(result, Err(ChannelError::InvalidBalance));
    }

    #[test]
    fn test_payment_a_to_b() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(channel.pay_a_to_b(100).is_ok());
        assert_eq!(channel.current_balance_a, 900);
        assert_eq!(channel.current_balance_b, 1100);
    }

    #[test]
    fn test_payment_b_to_a() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(channel.pay_b_to_a(100).is_ok());
        assert_eq!(channel.current_balance_a, 1100);
        assert_eq!(channel.current_balance_b, 900);
    }

    #[test]
    fn test_payment_insufficient_balance() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(channel.pay_a_to_b(200).is_err());
    }

    #[test]
    fn test_channel_nonce_increment() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert_eq!(channel.nonce, 0);
        assert!(channel.pay_a_to_b(100).is_ok());
        assert_eq!(channel.nonce, 1);
    }

    #[test]
    fn test_verify_balances() {
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(channel.verify_balances().is_ok());
    }

    #[test]
    fn test_channel_expiration() {
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(!channel.is_expired(150));
        assert!(!channel.is_expired(200)); // Not expired AT the expiry time
        assert!(channel.is_expired(201)); // Expired AFTER the expiry time
    }

    #[test]
    fn test_channel_state_transition() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        assert!(channel.transition(ChannelState::Closing).is_ok());
        assert_eq!(channel.state, ChannelState::Closing);
    }

    #[test]
    fn test_channel_summary() {
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        let summary = channel.summary();
        assert_eq!(summary.id, "ch1");
        assert_eq!(summary.balance_a, 1000);
        assert_eq!(summary.balance_b, 1000);
    }

    #[test]
    fn test_channel_update_creation() {
        let update = ChannelUpdate::new(
            "ch1".to_string(),
            1,
            900,
            1100,
            vec![1, 2, 3],
            100,
        );
        assert_eq!(update.nonce, 1);
        assert!(!update.is_fully_signed());
    }

    #[test]
    fn test_channel_update_signing() {
        let mut update = ChannelUpdate::new(
            "ch1".to_string(),
            1,
            900,
            1100,
            vec![1, 2, 3],
            100,
        );
        assert!(!update.is_fully_signed());
        update.sign_by_b(vec![4, 5, 6]);
        assert!(update.is_fully_signed());
    }

    #[test]
    fn test_settlement_creation() {
        let settlement = Settlement::new("ch1".to_string(), 900, 1100, 5, 300);
        assert_eq!(settlement.final_balance_a, 900);
        assert_eq!(settlement.final_balance_b, 1100);
    }

    #[test]
    fn test_dispute_reason_display() {
        assert_eq!(format!("{}", DisputeReason::InvalidStateUpdate), "Invalid state update");
        assert_eq!(format!("{}", DisputeReason::BalanceViolation), "Balance violation");
    }

    #[test]
    fn test_dispute_evidence_creation() {
        let evidence = DisputeEvidence::new(5, 900, 1100, "witness".to_string());
        assert_eq!(evidence.claimed_nonce, 5);
    }

    #[test]
    fn test_lightning_bloc_creation() {
        let bloc = LightningBloc::new();
        assert_eq!(bloc.active_channels_count(), 0);
    }

    #[test]
    fn test_lightning_bloc_open_channel() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        let result = bloc.open_channel(channel);
        assert!(result.is_ok());
        assert_eq!(bloc.active_channels_count(), 1);
    }

    #[test]
    fn test_lightning_bloc_get_channel() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();
        let retrieved = bloc.get_channel("ch1");
        assert!(retrieved.is_ok());
    }

    #[test]
    fn test_lightning_bloc_channel_not_found() {
        let bloc = LightningBloc::new();
        assert_eq!(bloc.get_channel("missing"), Err(ChannelError::ChannelNotFound("missing".to_string())));
    }

    #[test]
    fn test_lightning_bloc_execute_payment() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();
        assert!(bloc.execute_payment("ch1", true, 100).is_ok());

        let ch = bloc.get_channel("ch1").unwrap();
        assert_eq!(ch.current_balance_a, 900);
    }

    #[test]
    fn test_lightning_bloc_submit_update() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        let update = ChannelUpdate::new(
            "ch1".to_string(),
            1,
            900,
            1100,
            vec![1, 2, 3],
            100,
        );

        assert!(bloc.submit_update(update).is_ok());
    }

    #[test]
    fn test_lightning_bloc_get_updates() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        let update = ChannelUpdate::new(
            "ch1".to_string(),
            1,
            900,
            1100,
            vec![1, 2, 3],
            100,
        );

        bloc.submit_update(update).unwrap();
        let updates = bloc.get_updates("ch1");
        assert!(updates.is_ok());
        assert_eq!(updates.unwrap().len(), 1);
    }

    #[test]
    fn test_lightning_bloc_settle_channel() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        // Must transition to closing before settling
        bloc.transition_channel_state("ch1", ChannelState::Closing).unwrap();
        bloc.transition_channel_state("ch1", ChannelState::Closed).unwrap();

        let settlement = Settlement::new("ch1".to_string(), 900, 1100, 5, 300);
        assert!(bloc.settle_channel("ch1", settlement).is_ok());
    }

    #[test]
    fn test_lightning_bloc_file_dispute() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        let evidence = DisputeEvidence::new(5, 900, 1100, "witness".to_string());
        let dispute = Dispute {
            channel_id: "ch1".to_string(),
            complained_by: "alice".to_string(),
            reason: DisputeReason::InvalidStateUpdate,
            evidence,
            created_at: 100,
            resolved: false,
        };

        let result = bloc.file_dispute(dispute);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lightning_bloc_sign_update() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        let update = ChannelUpdate::new(
            "ch1".to_string(),
            1,
            900,
            1100,
            vec![1, 2, 3],
            100,
        );

        bloc.submit_update(update).unwrap();
        assert!(bloc.sign_update("ch1", vec![4, 5, 6]).is_ok());
    }

    #[test]
    fn test_lightning_bloc_total_locked_value() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();
        assert_eq!(bloc.total_locked_value(), 2000);
    }

    #[test]
    fn test_lightning_bloc_channels_by_state() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();
        assert_eq!(bloc.channels_by_state(ChannelState::Open), 1);
    }

    #[test]
    fn test_channel_payment_non_open_state() {
        let mut channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        channel.state = ChannelState::Closed;
        assert!(channel.pay_a_to_b(100).is_err());
    }

    #[test]
    fn test_lightning_bloc_resolve_dispute() {
        let mut bloc = LightningBloc::new();
        let channel = PaymentChannel::new(
            "ch1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            1000,
            100,
            200,
        )
        .unwrap();

        bloc.open_channel(channel).unwrap();

        let evidence = DisputeEvidence::new(5, 900, 1100, "witness".to_string());
        let dispute = Dispute {
            channel_id: "ch1".to_string(),
            complained_by: "alice".to_string(),
            reason: DisputeReason::InvalidStateUpdate,
            evidence,
            created_at: 100,
            resolved: false,
        };

        let dispute_id = bloc.file_dispute(dispute).unwrap();
        assert!(bloc.resolve_dispute(&dispute_id).is_ok());
    }
}
