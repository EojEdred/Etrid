//! Multi-Party State Channels for Lightning-Bloc
//!
//! Extends payment channels to support more than 2 participants, enabling
//! complex multi-party agreements and atomic multi-hop payments.

#[cfg(not(feature = "std"))]
use alloc::{
    collections::{BTreeMap as HashMap, BTreeSet as HashSet},
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
    collections::{HashMap, HashSet},
    fmt,
    vec::Vec,
    string::String,
    result::Result::{self, Ok, Err},
    option::Option::{self},
    default::Default,
};

/// Maximum number of parties in a multi-party channel
pub const MAX_PARTIES: usize = 10;

/// Minimum threshold for multi-party consensus (e.g., 67% = 2/3 majority)
pub const DEFAULT_CONSENSUS_THRESHOLD: u8 = 67; // 67%

/// Multi-party channel state
#[derive(Clone, Debug, PartialEq)]
pub struct MultiPartyChannel {
    pub channel_id: String,
    pub participants: Vec<Participant>,
    pub balances: HashMap<String, u128>,
    pub nonce: u64,
    pub consensus_threshold: u8, // Percentage required for state updates
    pub state: ChannelState,
    pub created_at: u64,
    pub expires_at: u64,
}

impl MultiPartyChannel {
    /// Create new multi-party channel
    pub fn new(
        channel_id: String,
        participants: Vec<Participant>,
        consensus_threshold: u8,
        created_at: u64,
        expires_at: u64,
    ) -> Result<Self, MultiPartyError> {
        // Validate participant count
        if participants.len() < 2 {
            return Err(MultiPartyError::InsufficientParticipants {
                have: participants.len(),
                min: 2,
            });
        }

        if participants.len() > MAX_PARTIES {
            return Err(MultiPartyError::TooManyParticipants {
                have: participants.len(),
                max: MAX_PARTIES,
            });
        }

        // Check for duplicate participants
        let mut seen = HashSet::new();
        for participant in &participants {
            if !seen.insert(&participant.address) {
                return Err(MultiPartyError::DuplicateParticipant(participant.address.clone()));
            }
        }

        // Validate consensus threshold
        if consensus_threshold < 51 || consensus_threshold > 100 {
            return Err(MultiPartyError::InvalidThreshold(consensus_threshold));
        }

        // Validate expiration
        if expires_at <= created_at {
            return Err(MultiPartyError::InvalidExpiration);
        }

        // Initialize balances from participants
        let mut balances = HashMap::new();
        for participant in &participants {
            if participant.initial_balance == 0 {
                return Err(MultiPartyError::ZeroBalance(participant.address.clone()));
            }
            balances.insert(participant.address.clone(), participant.initial_balance);
        }

        Ok(Self {
            channel_id,
            participants,
            balances,
            nonce: 0,
            consensus_threshold,
            state: ChannelState::Open,
            created_at,
            expires_at,
        })
    }

    /// Execute multi-party payment (from one party to another)
    pub fn execute_payment(
        &mut self,
        from: &str,
        to: &str,
        amount: u128,
        signatures: Vec<Signature>,
    ) -> Result<(), MultiPartyError> {
        // Check channel state
        if self.state != ChannelState::Open {
            return Err(MultiPartyError::ChannelNotOpen(self.state.clone()));
        }

        // Validate participants
        if !self.has_participant(from) {
            return Err(MultiPartyError::ParticipantNotFound(from.to_string()));
        }
        if !self.has_participant(to) {
            return Err(MultiPartyError::ParticipantNotFound(to.to_string()));
        }

        // Check balance
        let from_balance = self.balances.get(from)
            .ok_or_else(|| MultiPartyError::ParticipantNotFound(from.to_string()))?;

        if *from_balance < amount {
            return Err(MultiPartyError::InsufficientBalance {
                party: from.to_string(),
                have: *from_balance,
                need: amount,
            });
        }

        // Verify consensus
        self.verify_consensus(&signatures)?;

        // Execute transfer
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.get_mut(to).unwrap() += amount;
        self.nonce += 1;

        Ok(())
    }

    /// Execute multi-party split payment (from one to multiple)
    pub fn execute_split_payment(
        &mut self,
        from: &str,
        recipients: Vec<(String, u128)>, // (address, amount) pairs
        signatures: Vec<Signature>,
    ) -> Result<(), MultiPartyError> {
        // Check channel state
        if self.state != ChannelState::Open {
            return Err(MultiPartyError::ChannelNotOpen(self.state.clone()));
        }

        // Validate sender
        if !self.has_participant(from) {
            return Err(MultiPartyError::ParticipantNotFound(from.to_string()));
        }

        // Calculate total amount and validate recipients
        let mut total_amount = 0u128;
        for (recipient, amount) in &recipients {
            if !self.has_participant(recipient) {
                return Err(MultiPartyError::ParticipantNotFound(recipient.clone()));
            }
            total_amount = total_amount.checked_add(*amount)
                .ok_or(MultiPartyError::BalanceOverflow)?;
        }

        // Check sender balance
        let from_balance = self.balances.get(from)
            .ok_or_else(|| MultiPartyError::ParticipantNotFound(from.to_string()))?;

        if *from_balance < total_amount {
            return Err(MultiPartyError::InsufficientBalance {
                party: from.to_string(),
                have: *from_balance,
                need: total_amount,
            });
        }

        // Verify consensus
        self.verify_consensus(&signatures)?;

        // Execute transfers
        *self.balances.get_mut(from).unwrap() -= total_amount;
        for (recipient, amount) in recipients {
            *self.balances.get_mut(&recipient).unwrap() += amount;
        }
        self.nonce += 1;

        Ok(())
    }

    /// Verify balance invariant (total balance unchanged)
    pub fn verify_balances(&self) -> Result<(), MultiPartyError> {
        let current_total: u128 = self.balances.values().sum();
        let initial_total: u128 = self.participants
            .iter()
            .map(|p| p.initial_balance)
            .sum();

        if current_total != initial_total {
            return Err(MultiPartyError::BalanceInvariantViolated {
                expected: initial_total,
                got: current_total,
            });
        }

        Ok(())
    }

    /// Check if address is a participant
    pub fn has_participant(&self, address: &str) -> bool {
        self.participants.iter().any(|p| p.address == address)
    }

    /// Get participant by address
    pub fn get_participant(&self, address: &str) -> Option<&Participant> {
        self.participants.iter().find(|p| p.address == address)
    }

    /// Verify consensus threshold is met
    pub fn verify_consensus(&self, signatures: &[Signature]) -> Result<(), MultiPartyError> {
        // Count unique valid signatures
        let mut signed_addresses = HashSet::new();
        for sig in signatures {
            if self.has_participant(&sig.signer) {
                signed_addresses.insert(&sig.signer);
            }
        }

        let signed_count = signed_addresses.len();
        let required_count = self.required_signatures_count();

        if signed_count < required_count {
            return Err(MultiPartyError::InsufficientSignatures {
                have: signed_count,
                required: required_count,
            });
        }

        Ok(())
    }

    /// Calculate required number of signatures based on threshold
    pub fn required_signatures_count(&self) -> usize {
        let total = self.participants.len();
        // Calculate required signatures: (total * threshold + 50) / 100 for rounding
        // For 3 participants with 67% threshold: (3 * 67 + 50) / 100 = (201 + 50) / 100 = 2
        let required = (total * self.consensus_threshold as usize + 50) / 100;
        required.min(total).max(1) // At least 1, at most total
    }

    /// Transition channel state
    pub fn transition(&mut self, new_state: ChannelState) -> Result<(), MultiPartyError> {
        let valid_transitions = match &self.state {
            ChannelState::Open => vec![ChannelState::Closing, ChannelState::Disputed],
            ChannelState::Closing => vec![ChannelState::Closed, ChannelState::Disputed],
            ChannelState::Closed => vec![ChannelState::Settled],
            ChannelState::Disputed => vec![ChannelState::Closed, ChannelState::Settled],
            ChannelState::Settled => vec![],
        };

        if valid_transitions.contains(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(MultiPartyError::InvalidStateTransition {
                current: self.state.clone(),
                requested: new_state,
            })
        }
    }

    /// Check if channel is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    /// Get channel summary
    pub fn summary(&self) -> MultiPartyChannelSummary {
        MultiPartyChannelSummary {
            channel_id: self.channel_id.clone(),
            participant_count: self.participants.len(),
            state: self.state.clone(),
            nonce: self.nonce,
            total_locked: self.balances.values().sum(),
        }
    }
}

/// Channel participant
#[derive(Clone, Debug, PartialEq)]
pub struct Participant {
    pub address: String,
    pub initial_balance: u128,
    pub public_key: Vec<u8>,
}

impl Participant {
    pub fn new(address: String, initial_balance: u128, public_key: Vec<u8>) -> Self {
        Self {
            address,
            initial_balance,
            public_key,
        }
    }
}

/// Signature for state updates
#[derive(Clone, Debug, PartialEq)]
pub struct Signature {
    pub signer: String,
    pub signature_data: Vec<u8>,
    pub timestamp: u64,
}

impl Signature {
    pub fn new(signer: String, signature_data: Vec<u8>, timestamp: u64) -> Self {
        Self {
            signer,
            signature_data,
            timestamp,
        }
    }
}

/// Channel state
#[derive(Clone, Debug, PartialEq)]
pub enum ChannelState {
    Open,
    Closing,
    Closed,
    Disputed,
    Settled,
}

impl fmt::Display for ChannelState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelState::Open => write!(f, "Open"),
            ChannelState::Closing => write!(f, "Closing"),
            ChannelState::Closed => write!(f, "Closed"),
            ChannelState::Disputed => write!(f, "Disputed"),
            ChannelState::Settled => write!(f, "Settled"),
        }
    }
}

/// Channel summary
#[derive(Clone, Debug, PartialEq)]
pub struct MultiPartyChannelSummary {
    pub channel_id: String,
    pub participant_count: usize,
    pub state: ChannelState,
    pub nonce: u64,
    pub total_locked: u128,
}

/// Multi-party channel manager
pub struct MultiPartyChannelManager {
    channels: HashMap<String, MultiPartyChannel>,
}

impl MultiPartyChannelManager {
    /// Create new manager
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    /// Open new multi-party channel
    pub fn open_channel(
        &mut self,
        channel: MultiPartyChannel,
    ) -> Result<String, MultiPartyError> {
        let channel_id = channel.channel_id.clone();

        if self.channels.contains_key(&channel_id) {
            return Err(MultiPartyError::ChannelAlreadyExists(channel_id));
        }

        self.channels.insert(channel_id.clone(), channel);
        Ok(channel_id)
    }

    /// Get channel
    pub fn get_channel(&self, channel_id: &str) -> Result<&MultiPartyChannel, MultiPartyError> {
        self.channels
            .get(channel_id)
            .ok_or_else(|| MultiPartyError::ChannelNotFound(channel_id.to_string()))
    }

    /// Get mutable channel
    pub fn get_channel_mut(&mut self, channel_id: &str) -> Result<&mut MultiPartyChannel, MultiPartyError> {
        self.channels
            .get_mut(channel_id)
            .ok_or_else(|| MultiPartyError::ChannelNotFound(channel_id.to_string()))
    }

    /// Execute payment in channel
    pub fn execute_payment(
        &mut self,
        channel_id: &str,
        from: &str,
        to: &str,
        amount: u128,
        signatures: Vec<Signature>,
    ) -> Result<(), MultiPartyError> {
        let channel = self.get_channel_mut(channel_id)?;
        channel.execute_payment(from, to, amount, signatures)
    }

    /// Execute split payment in channel
    pub fn execute_split_payment(
        &mut self,
        channel_id: &str,
        from: &str,
        recipients: Vec<(String, u128)>,
        signatures: Vec<Signature>,
    ) -> Result<(), MultiPartyError> {
        let channel = self.get_channel_mut(channel_id)?;
        channel.execute_split_payment(from, recipients, signatures)
    }

    /// Get all active channels
    pub fn active_channels(&self) -> Vec<&MultiPartyChannel> {
        self.channels
            .values()
            .filter(|c| c.state == ChannelState::Open)
            .collect()
    }

    /// Get total locked value across all channels
    pub fn total_locked_value(&self) -> u128 {
        self.channels
            .values()
            .map(|c| c.balances.values().sum::<u128>())
            .sum()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> MultiPartyStatistics {
        let total_channels = self.channels.len();
        let active_channels = self.active_channels().len();
        let total_locked = self.total_locked_value();
        let total_participants: usize = self.channels
            .values()
            .map(|c| c.participants.len())
            .sum();

        MultiPartyStatistics {
            total_channels,
            active_channels,
            total_locked,
            total_participants,
        }
    }
}

impl Default for MultiPartyChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-party channel statistics
#[derive(Clone, Debug, PartialEq)]
pub struct MultiPartyStatistics {
    pub total_channels: usize,
    pub active_channels: usize,
    pub total_locked: u128,
    pub total_participants: usize,
}

/// Multi-party channel errors
#[derive(Clone, Debug, PartialEq)]
pub enum MultiPartyError {
    InsufficientParticipants { have: usize, min: usize },
    TooManyParticipants { have: usize, max: usize },
    DuplicateParticipant(String),
    InvalidThreshold(u8),
    InvalidExpiration,
    ZeroBalance(String),
    ChannelNotFound(String),
    ChannelAlreadyExists(String),
    ChannelNotOpen(ChannelState),
    ParticipantNotFound(String),
    InsufficientBalance { party: String, have: u128, need: u128 },
    BalanceOverflow,
    BalanceInvariantViolated { expected: u128, got: u128 },
    InsufficientSignatures { have: usize, required: usize },
    InvalidStateTransition { current: ChannelState, requested: ChannelState },
}

impl fmt::Display for MultiPartyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MultiPartyError::InsufficientParticipants { have, min } => {
                write!(f, "Insufficient participants: {} < {}", have, min)
            }
            MultiPartyError::TooManyParticipants { have, max } => {
                write!(f, "Too many participants: {} > {}", have, max)
            }
            MultiPartyError::DuplicateParticipant(addr) => {
                write!(f, "Duplicate participant: {}", addr)
            }
            MultiPartyError::InvalidThreshold(t) => {
                write!(f, "Invalid threshold: {}% (must be 51-100)", t)
            }
            MultiPartyError::InvalidExpiration => write!(f, "Invalid expiration time"),
            MultiPartyError::ZeroBalance(addr) => {
                write!(f, "Zero balance not allowed: {}", addr)
            }
            MultiPartyError::ChannelNotFound(id) => write!(f, "Channel not found: {}", id),
            MultiPartyError::ChannelAlreadyExists(id) => {
                write!(f, "Channel already exists: {}", id)
            }
            MultiPartyError::ChannelNotOpen(state) => {
                write!(f, "Channel not open: {}", state)
            }
            MultiPartyError::ParticipantNotFound(addr) => {
                write!(f, "Participant not found: {}", addr)
            }
            MultiPartyError::InsufficientBalance { party, have, need } => {
                write!(f, "Insufficient balance for {}: {} < {}", party, have, need)
            }
            MultiPartyError::BalanceOverflow => write!(f, "Balance overflow"),
            MultiPartyError::BalanceInvariantViolated { expected, got } => {
                write!(f, "Balance invariant violated: {} != {}", expected, got)
            }
            MultiPartyError::InsufficientSignatures { have, required } => {
                write!(f, "Insufficient signatures: {} < {}", have, required)
            }
            MultiPartyError::InvalidStateTransition { current, requested } => {
                write!(f, "Invalid state transition: {} -> {}", current, requested)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_participants() -> Vec<Participant> {
        vec![
            Participant::new("alice".to_string(), 1000, vec![1, 2, 3]),
            Participant::new("bob".to_string(), 1000, vec![4, 5, 6]),
            Participant::new("charlie".to_string(), 1000, vec![7, 8, 9]),
        ]
    }

    fn create_test_signatures() -> Vec<Signature> {
        vec![
            Signature::new("alice".to_string(), vec![1, 2, 3], 100),
            Signature::new("bob".to_string(), vec![4, 5, 6], 100),
        ]
    }

    #[test]
    fn test_multi_party_channel_creation() {
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        );
        assert!(channel.is_ok());
    }

    #[test]
    fn test_insufficient_participants() {
        let participants = vec![
            Participant::new("alice".to_string(), 1000, vec![1, 2, 3]),
        ];
        let result = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_too_many_participants() {
        let mut participants = Vec::new();
        for i in 0..MAX_PARTIES + 1 {
            participants.push(Participant::new(
                format!("party_{}", i),
                1000,
                vec![i as u8],
            ));
        }
        let result = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_participants() {
        let participants = vec![
            Participant::new("alice".to_string(), 1000, vec![1, 2, 3]),
            Participant::new("alice".to_string(), 1000, vec![4, 5, 6]),
        ];
        let result = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_payment() {
        let participants = create_test_participants();
        let mut channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let signatures = create_test_signatures();
        let result = channel.execute_payment("alice", "bob", 100, signatures);
        assert!(result.is_ok());
        assert_eq!(channel.balances.get("alice"), Some(&900));
        assert_eq!(channel.balances.get("bob"), Some(&1100));
    }

    #[test]
    fn test_execute_payment_insufficient_balance() {
        let participants = create_test_participants();
        let mut channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let signatures = create_test_signatures();
        let result = channel.execute_payment("alice", "bob", 2000, signatures);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_split_payment() {
        let participants = create_test_participants();
        let mut channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let recipients = vec![
            ("bob".to_string(), 100),
            ("charlie".to_string(), 200),
        ];
        let signatures = create_test_signatures();

        let result = channel.execute_split_payment("alice", recipients, signatures);
        assert!(result.is_ok());
        assert_eq!(channel.balances.get("alice"), Some(&700));
        assert_eq!(channel.balances.get("bob"), Some(&1100));
        assert_eq!(channel.balances.get("charlie"), Some(&1200));
    }

    #[test]
    fn test_verify_balances() {
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        assert!(channel.verify_balances().is_ok());
    }

    #[test]
    fn test_required_signatures_count() {
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD, // 67%
            100,
            200,
        )
        .unwrap();

        // With 3 participants and 67% threshold, need 2 signatures
        assert_eq!(channel.required_signatures_count(), 2);
    }

    #[test]
    fn test_verify_consensus() {
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let signatures = create_test_signatures(); // 2 signatures
        assert!(channel.verify_consensus(&signatures).is_ok());
    }

    #[test]
    fn test_verify_consensus_insufficient_signatures() {
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let signatures = vec![
            Signature::new("alice".to_string(), vec![1, 2, 3], 100),
        ]; // Only 1 signature, need 2
        assert!(channel.verify_consensus(&signatures).is_err());
    }

    #[test]
    fn test_multi_party_manager() {
        let mut manager = MultiPartyChannelManager::new();
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        let result = manager.open_channel(channel);
        assert!(result.is_ok());
        assert!(manager.get_channel("ch1").is_ok());
    }

    #[test]
    fn test_manager_execute_payment() {
        let mut manager = MultiPartyChannelManager::new();
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        manager.open_channel(channel).unwrap();

        let signatures = create_test_signatures();
        let result = manager.execute_payment("ch1", "alice", "bob", 100, signatures);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_statistics() {
        let mut manager = MultiPartyChannelManager::new();
        let participants = create_test_participants();
        let channel = MultiPartyChannel::new(
            "ch1".to_string(),
            participants,
            DEFAULT_CONSENSUS_THRESHOLD,
            100,
            200,
        )
        .unwrap();

        manager.open_channel(channel).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_channels, 1);
        assert_eq!(stats.active_channels, 1);
        assert_eq!(stats.total_participants, 3);
    }
}
