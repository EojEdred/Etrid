//! Cross-Chain Bridge Module for ÉTRID
//!
//! Enables atomic asset transfers between ÉTRID and other blockchains:
//! - Ethereum bridge (token wrapping)
//! - Bitcoin bridge (UTXO coordination)
//! - State proof generation and verification
//! - Validator-backed attestations
//! - Atomic swap settlement

use std::collections::HashMap;
use std::fmt;

/// Supported chains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainId {
    Etrid,
    Ethereum,
    Bitcoin,
    Polygon,
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChainId::Etrid => write!(f, "ÉTRID"),
            ChainId::Ethereum => write!(f, "Ethereum"),
            ChainId::Bitcoin => write!(f, "Bitcoin"),
            ChainId::Polygon => write!(f, "Polygon"),
        }
    }
}

/// Asset identifier (chain-native)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId {
    pub chain: ChainId,
    pub token_address: String,
}

impl AssetId {
    pub fn new(chain: ChainId, token_address: String) -> Self {
        Self { chain, token_address }
    }

    /// Create ÉTRID native asset
    pub fn etrid_native() -> Self {
        Self {
            chain: ChainId::Etrid,
            token_address: "NATIVE".to_string(),
        }
    }

    /// Create wrapped ETH on ÉTRID
    pub fn wrapped_eth() -> Self {
        Self {
            chain: ChainId::Ethereum,
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }
}

/// Cross-chain transfer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferState {
    Initiated,
    Locked,
    Proven,
    Released,
    Reverted,
    Disputed,
}

impl fmt::Display for TransferState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransferState::Initiated => write!(f, "Initiated"),
            TransferState::Locked => write!(f, "Locked"),
            TransferState::Proven => write!(f, "Proven"),
            TransferState::Released => write!(f, "Released"),
            TransferState::Reverted => write!(f, "Reverted"),
            TransferState::Disputed => write!(f, "Disputed"),
        }
    }
}

/// Bridge transfer
#[derive(Debug, Clone, PartialEq)]
pub struct BridgeTransfer {
    pub id: String,
    pub from_chain: ChainId,
    pub to_chain: ChainId,
    pub asset: AssetId,
    pub amount: u128,
    pub sender: String,
    pub recipient: String,
    pub state: TransferState,
    pub created_at: u64,
    pub expires_at: u64,
}

impl BridgeTransfer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        from_chain: ChainId,
        to_chain: ChainId,
        asset: AssetId,
        amount: u128,
        sender: String,
        recipient: String,
        created_at: u64,
        expires_at: u64,
    ) -> Result<Self, BridgeError> {
        if amount == 0 {
            return Err(BridgeError::InvalidAmount);
        }
        if sender.is_empty() || recipient.is_empty() {
            return Err(BridgeError::InvalidAddress);
        }
        if expires_at <= created_at {
            return Err(BridgeError::InvalidExpiration);
        }

        Ok(Self {
            id,
            from_chain,
            to_chain,
            asset,
            amount,
            sender,
            recipient,
            state: TransferState::Initiated,
            created_at,
            expires_at,
        })
    }

    /// Check if transfer is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time > self.expires_at
    }

    /// Transition to next state
    pub fn transition(&mut self, new_state: TransferState) -> Result<(), BridgeError> {
        // Validate state machine transitions
        let valid_next = match self.state {
            TransferState::Initiated => vec![TransferState::Locked, TransferState::Reverted],
            TransferState::Locked => vec![TransferState::Proven, TransferState::Disputed],
            TransferState::Proven => vec![TransferState::Released, TransferState::Disputed],
            TransferState::Released => vec![],
            TransferState::Reverted => vec![],
            TransferState::Disputed => vec![
                TransferState::Released,
                TransferState::Reverted,
            ],
        };

        if valid_next.contains(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(BridgeError::InvalidStateTransition {
                current: self.state,
                requested: new_state,
            })
        }
    }
}

/// State proof for cross-chain finality
#[derive(Debug, Clone, PartialEq)]
pub struct StateProof {
    pub block_hash: String,
    pub block_height: u64,
    pub merkle_root: String,
    pub validator_signatures: Vec<ValidatorSignature>,
    pub threshold: usize,
}

impl StateProof {
    pub fn new(
        block_hash: String,
        block_height: u64,
        merkle_root: String,
        validator_signatures: Vec<ValidatorSignature>,
        threshold: usize,
    ) -> Result<Self, BridgeError> {
        if validator_signatures.len() < threshold {
            return Err(BridgeError::InsufficientSignatures {
                have: validator_signatures.len(),
                need: threshold,
            });
        }

        Ok(Self {
            block_hash,
            block_height,
            merkle_root,
            validator_signatures,
            threshold,
        })
    }

    /// Verify proof has sufficient signatures
    pub fn is_valid(&self) -> bool {
        self.validator_signatures.len() >= self.threshold
    }

    /// Get unique signers
    pub fn unique_signers(&self) -> usize {
        let mut signers = std::collections::HashSet::new();
        for sig in &self.validator_signatures {
            signers.insert(sig.validator_id.clone());
        }
        signers.len()
    }
}

/// Validator signature attestation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatorSignature {
    pub validator_id: String,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

impl ValidatorSignature {
    pub fn new(validator_id: String, signature: Vec<u8>, timestamp: u64) -> Self {
        Self {
            validator_id,
            signature,
            timestamp,
        }
    }
}

/// Atomic swap order
#[derive(Debug, Clone, PartialEq)]
pub struct AtomicSwap {
    pub id: String,
    pub chain_a: ChainId,
    pub chain_b: ChainId,
    pub asset_a: AssetId,
    pub asset_b: AssetId,
    pub amount_a: u128,
    pub amount_b: u128,
    pub party_a: String,
    pub party_b: String,
    pub state: SwapState,
    pub lock_time: u64,
}

/// Atomic swap state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapState {
    Initiated,
    LockedA,
    LockedB,
    Complete,
    Refunded,
}

impl fmt::Display for SwapState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapState::Initiated => write!(f, "Initiated"),
            SwapState::LockedA => write!(f, "LockedA"),
            SwapState::LockedB => write!(f, "LockedB"),
            SwapState::Complete => write!(f, "Complete"),
            SwapState::Refunded => write!(f, "Refunded"),
        }
    }
}

impl AtomicSwap {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        chain_a: ChainId,
        chain_b: ChainId,
        asset_a: AssetId,
        asset_b: AssetId,
        amount_a: u128,
        amount_b: u128,
        party_a: String,
        party_b: String,
        lock_time: u64,
    ) -> Result<Self, BridgeError> {
        if amount_a == 0 || amount_b == 0 {
            return Err(BridgeError::InvalidAmount);
        }

        Ok(Self {
            id,
            chain_a,
            chain_b,
            asset_a,
            asset_b,
            amount_a,
            amount_b,
            party_a,
            party_b,
            state: SwapState::Initiated,
            lock_time,
        })
    }

    /// Progress swap to next state
    pub fn advance_state(&mut self, new_state: SwapState) -> Result<(), BridgeError> {
        let valid_next = match self.state {
            SwapState::Initiated => vec![SwapState::LockedA, SwapState::Refunded],
            SwapState::LockedA => vec![SwapState::LockedB, SwapState::Refunded],
            SwapState::LockedB => vec![SwapState::Complete, SwapState::Refunded],
            SwapState::Complete => vec![],
            SwapState::Refunded => vec![],
        };

        if valid_next.contains(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(BridgeError::InvalidSwapTransition {
                current: self.state,
                requested: new_state,
            })
        }
    }
}

/// Bridge errors
#[derive(Debug, Clone, PartialEq)]
pub enum BridgeError {
    InvalidAmount,
    InvalidAddress,
    InvalidExpiration,
    InvalidStateTransition { current: TransferState, requested: TransferState },
    InvalidSwapTransition { current: SwapState, requested: SwapState },
    InsufficientSignatures { have: usize, need: usize },
    TransferNotFound(String),
    TransferExpired,
    SwapNotFound(String),
    InvalidProof,
    ProofMissing,
    ValidatorNotFound,
    DuplicateSignature,
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BridgeError::InvalidAmount => write!(f, "Invalid amount"),
            BridgeError::InvalidAddress => write!(f, "Invalid address"),
            BridgeError::InvalidExpiration => write!(f, "Invalid expiration"),
            BridgeError::InvalidStateTransition { current, requested } => {
                write!(f, "Invalid state transition: {} -> {}", current, requested)
            }
            BridgeError::InvalidSwapTransition { current, requested } => {
                write!(f, "Invalid swap transition: {} -> {}", current, requested)
            }
            BridgeError::InsufficientSignatures { have, need } => {
                write!(f, "Insufficient signatures: {} < {}", have, need)
            }
            BridgeError::TransferNotFound(id) => write!(f, "Transfer not found: {}", id),
            BridgeError::TransferExpired => write!(f, "Transfer expired"),
            BridgeError::SwapNotFound(id) => write!(f, "Swap not found: {}", id),
            BridgeError::InvalidProof => write!(f, "Invalid proof"),
            BridgeError::ProofMissing => write!(f, "Proof missing"),
            BridgeError::ValidatorNotFound => write!(f, "Validator not found"),
            BridgeError::DuplicateSignature => write!(f, "Duplicate signature"),
        }
    }
}

/// Cross-chain bridge manager
#[derive(Debug, PartialEq)]
pub struct CrossChainBridge {
    transfers: HashMap<String, BridgeTransfer>,
    swaps: HashMap<String, AtomicSwap>,
    proofs: HashMap<String, StateProof>,
    validators: Vec<String>,
    threshold: usize,
}

impl CrossChainBridge {
    /// Create new bridge
    pub fn new(validators: Vec<String>, threshold: usize) -> Result<Self, BridgeError> {
        if validators.is_empty() {
            return Err(BridgeError::ValidatorNotFound);
        }
        if threshold > validators.len() || threshold == 0 {
            return Err(BridgeError::InsufficientSignatures {
                have: threshold,
                need: validators.len(),
            });
        }

        Ok(Self {
            transfers: HashMap::new(),
            swaps: HashMap::new(),
            proofs: HashMap::new(),
            validators,
            threshold,
        })
    }

    /// Initiate bridge transfer
    pub fn initiate_transfer(
        &mut self,
        transfer: BridgeTransfer,
    ) -> Result<String, BridgeError> {
        let id = transfer.id.clone();
        self.transfers.insert(id.clone(), transfer);
        Ok(id)
    }

    /// Get transfer
    pub fn get_transfer(&self, id: &str) -> Result<BridgeTransfer, BridgeError> {
        self.transfers
            .get(id)
            .cloned()
            .ok_or_else(|| BridgeError::TransferNotFound(id.to_string()))
    }

    /// Update transfer state
    pub fn update_transfer_state(
        &mut self,
        id: &str,
        new_state: TransferState,
    ) -> Result<(), BridgeError> {
        let transfer = self
            .transfers
            .get_mut(id)
            .ok_or_else(|| BridgeError::TransferNotFound(id.to_string()))?;
        transfer.transition(new_state)
    }

    /// Submit state proof
    pub fn submit_proof(
        &mut self,
        transfer_id: String,
        proof: StateProof,
    ) -> Result<(), BridgeError> {
        if !proof.is_valid() {
            return Err(BridgeError::InvalidProof);
        }

        self.proofs.insert(transfer_id, proof);
        Ok(())
    }

    /// Get proof for transfer
    pub fn get_proof(&self, transfer_id: &str) -> Result<StateProof, BridgeError> {
        self.proofs
            .get(transfer_id)
            .cloned()
            .ok_or(BridgeError::ProofMissing)
    }

    /// Initiate atomic swap
    pub fn initiate_swap(&mut self, swap: AtomicSwap) -> Result<String, BridgeError> {
        let id = swap.id.clone();
        self.swaps.insert(id.clone(), swap);
        Ok(id)
    }

    /// Get swap
    pub fn get_swap(&self, id: &str) -> Result<AtomicSwap, BridgeError> {
        self.swaps
            .get(id)
            .cloned()
            .ok_or_else(|| BridgeError::SwapNotFound(id.to_string()))
    }

    /// Update swap state
    pub fn update_swap_state(
        &mut self,
        id: &str,
        new_state: SwapState,
    ) -> Result<(), BridgeError> {
        let swap = self
            .swaps
            .get_mut(id)
            .ok_or_else(|| BridgeError::SwapNotFound(id.to_string()))?;
        swap.advance_state(new_state)
    }

    /// Validate transfer expiration
    pub fn check_expired(&self, transfer_id: &str, current_time: u64) -> Result<bool, BridgeError> {
        let transfer = self.get_transfer(transfer_id)?;
        Ok(transfer.is_expired(current_time))
    }

    /// Get active transfers count
    pub fn active_transfers_count(&self) -> usize {
        self.transfers
            .values()
            .filter(|t| t.state != TransferState::Released && t.state != TransferState::Reverted)
            .count()
    }

    /// Get active swaps count
    pub fn active_swaps_count(&self) -> usize {
        self.swaps
            .values()
            .filter(|s| s.state != SwapState::Complete && s.state != SwapState::Refunded)
            .count()
    }

    /// List transfers by state
    pub fn transfers_by_state(&self, state: TransferState) -> Vec<BridgeTransfer> {
        self.transfers
            .values()
            .filter(|t| t.state == state)
            .cloned()
            .collect()
    }

    /// Generate proof for block
    pub fn generate_proof(
        &self,
        block_hash: String,
        block_height: u64,
        merkle_root: String,
        signatures: Vec<ValidatorSignature>,
    ) -> Result<StateProof, BridgeError> {
        StateProof::new(block_hash, block_height, merkle_root, signatures, self.threshold)
    }

    /// Add validator
    pub fn add_validator(&mut self, validator: String) {
        if !self.validators.contains(&validator) {
            self.validators.push(validator);
        }
    }

    /// Check validator exists
    pub fn has_validator(&self, validator: &str) -> bool {
        self.validators.iter().any(|v| v == validator)
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Get signatures from unique validators
    pub fn verify_unique_signers(
        &self,
        signatures: &[ValidatorSignature],
    ) -> Result<usize, BridgeError> {
        let mut seen = std::collections::HashSet::new();
        for sig in signatures {
            if !self.has_validator(&sig.validator_id) {
                return Err(BridgeError::ValidatorNotFound);
            }
            if !seen.insert(&sig.validator_id) {
                return Err(BridgeError::DuplicateSignature);
            }
        }
        Ok(seen.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_display() {
        assert_eq!(format!("{}", ChainId::Etrid), "ÉTRID");
        assert_eq!(format!("{}", ChainId::Ethereum), "Ethereum");
        assert_eq!(format!("{}", ChainId::Bitcoin), "Bitcoin");
    }

    #[test]
    fn test_asset_id_creation() {
        let asset = AssetId::new(ChainId::Ethereum, "0x123".to_string());
        assert_eq!(asset.chain, ChainId::Ethereum);
        assert_eq!(asset.token_address, "0x123");
    }

    #[test]
    fn test_asset_id_native() {
        let asset = AssetId::etrid_native();
        assert_eq!(asset.chain, ChainId::Etrid);
    }

    #[test]
    fn test_asset_id_wrapped_eth() {
        let asset = AssetId::wrapped_eth();
        assert_eq!(asset.chain, ChainId::Ethereum);
    }

    #[test]
    fn test_bridge_transfer_creation() {
        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        );
        assert!(transfer.is_ok());
    }

    #[test]
    fn test_bridge_transfer_invalid_amount() {
        let result = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            0,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        );
        assert_eq!(result, Err(BridgeError::InvalidAmount));
    }

    #[test]
    fn test_bridge_transfer_invalid_address() {
        let result = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "".to_string(),
            "recipient".to_string(),
            100,
            200,
        );
        assert_eq!(result, Err(BridgeError::InvalidAddress));
    }

    #[test]
    fn test_bridge_transfer_expiration() {
        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        assert!(!transfer.is_expired(150));
        assert!(transfer.is_expired(200));
        assert!(transfer.is_expired(300));
    }

    #[test]
    fn test_transfer_state_transition() {
        let mut transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        assert!(transfer.transition(TransferState::Locked).is_ok());
        assert_eq!(transfer.state, TransferState::Locked);

        assert!(transfer.transition(TransferState::Proven).is_ok());
        assert_eq!(transfer.state, TransferState::Proven);

        assert!(transfer.transition(TransferState::Released).is_ok());
        assert_eq!(transfer.state, TransferState::Released);
    }

    #[test]
    fn test_transfer_invalid_state_transition() {
        let mut transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        assert!(transfer.transition(TransferState::Released).is_err());
    }

    #[test]
    fn test_state_proof_creation() {
        let sigs = vec![ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100)];
        let proof = StateProof::new(
            "hash".to_string(),
            1000,
            "root".to_string(),
            sigs,
            1,
        );
        assert!(proof.is_ok());
    }

    #[test]
    fn test_state_proof_insufficient_signatures() {
        let sigs = vec![ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100)];
        let result = StateProof::new(
            "hash".to_string(),
            1000,
            "root".to_string(),
            sigs,
            2,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_state_proof_validity() {
        let sigs = vec![
            ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100),
            ValidatorSignature::new("v2".to_string(), vec![4, 5, 6], 101),
        ];
        let proof = StateProof::new(
            "hash".to_string(),
            1000,
            "root".to_string(),
            sigs,
            2,
        )
        .unwrap();
        assert!(proof.is_valid());
    }

    #[test]
    fn test_atomic_swap_creation() {
        let swap = AtomicSwap::new(
            "swap1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            AssetId::etrid_native(),
            1000,
            2000,
            "party_a".to_string(),
            "party_b".to_string(),
            300,
        );
        assert!(swap.is_ok());
    }

    #[test]
    fn test_atomic_swap_invalid_amount() {
        let result = AtomicSwap::new(
            "swap1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            AssetId::etrid_native(),
            0,
            2000,
            "party_a".to_string(),
            "party_b".to_string(),
            300,
        );
        assert_eq!(result, Err(BridgeError::InvalidAmount));
    }

    #[test]
    fn test_swap_state_progression() {
        let mut swap = AtomicSwap::new(
            "swap1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            AssetId::etrid_native(),
            1000,
            2000,
            "party_a".to_string(),
            "party_b".to_string(),
            300,
        )
        .unwrap();

        assert!(swap.advance_state(SwapState::LockedA).is_ok());
        assert!(swap.advance_state(SwapState::LockedB).is_ok());
        assert!(swap.advance_state(SwapState::Complete).is_ok());
    }

    #[test]
    fn test_bridge_creation() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
            2,
        );
        assert!(bridge.is_ok());
    }

    #[test]
    fn test_bridge_empty_validators() {
        let result = CrossChainBridge::new(vec![], 1);
        assert_eq!(result, Err(BridgeError::ValidatorNotFound));
    }

    #[test]
    fn test_bridge_threshold_exceeds_validators() {
        let result = CrossChainBridge::new(
            vec!["v1".to_string(), "v2".to_string()],
            3,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_bridge_initiate_transfer() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        let result = bridge.initiate_transfer(transfer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bridge_get_transfer() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        bridge.initiate_transfer(transfer).unwrap();
        let retrieved = bridge.get_transfer("tx1");
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().id, "tx1");
    }

    #[test]
    fn test_bridge_transfer_not_found() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        assert_eq!(bridge.get_transfer("missing"), Err(BridgeError::TransferNotFound("missing".to_string())));
    }

    #[test]
    fn test_bridge_update_transfer_state() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        bridge.initiate_transfer(transfer).unwrap();
        assert!(bridge.update_transfer_state("tx1", TransferState::Locked).is_ok());
        
        let updated = bridge.get_transfer("tx1").unwrap();
        assert_eq!(updated.state, TransferState::Locked);
    }

    #[test]
    fn test_bridge_submit_proof() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let sigs = vec![ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100)];
        let proof = StateProof::new(
            "hash".to_string(),
            1000,
            "root".to_string(),
            sigs,
            1,
        )
        .unwrap();

        assert!(bridge.submit_proof("tx1".to_string(), proof).is_ok());
    }

    #[test]
    fn test_bridge_get_proof() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let sigs = vec![ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100)];
        let proof = StateProof::new(
            "hash".to_string(),
            1000,
            "root".to_string(),
            sigs,
            1,
        )
        .unwrap();

        bridge.submit_proof("tx1".to_string(), proof).unwrap();
        assert!(bridge.get_proof("tx1").is_ok());
    }

    #[test]
    fn test_bridge_proof_missing() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        assert_eq!(bridge.get_proof("tx1"), Err(BridgeError::ProofMissing));
    }

    #[test]
    fn test_bridge_initiate_swap() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let swap = AtomicSwap::new(
            "swap1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            AssetId::etrid_native(),
            1000,
            2000,
            "party_a".to_string(),
            "party_b".to_string(),
            300,
        )
        .unwrap();

        let result = bridge.initiate_swap(swap);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bridge_active_transfers_count() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        bridge.initiate_transfer(transfer).unwrap();
        assert_eq!(bridge.active_transfers_count(), 1);

        bridge.update_transfer_state("tx1", TransferState::Released).unwrap();
        assert_eq!(bridge.active_transfers_count(), 0);
    }

    #[test]
    fn test_bridge_add_validator() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        bridge.add_validator("v2".to_string());
        assert!(bridge.has_validator("v2"));
    }

    #[test]
    fn test_transfer_state_display() {
        assert_eq!(format!("{}", TransferState::Initiated), "Initiated");
        assert_eq!(format!("{}", TransferState::Locked), "Locked");
        assert_eq!(format!("{}", TransferState::Released), "Released");
    }

    #[test]
    fn test_swap_state_display() {
        assert_eq!(format!("{}", SwapState::Initiated), "Initiated");
        assert_eq!(format!("{}", SwapState::Complete), "Complete");
    }

    #[test]
    fn test_validator_signature_creation() {
        let sig = ValidatorSignature::new("v1".to_string(), vec![1, 2, 3], 100);
        assert_eq!(sig.validator_id, "v1");
    }

    #[test]
    fn test_proof_unique_signers() {
        let sigs = vec![
            ValidatorSignature::new("v1".to_string(), vec![1], 100),
            ValidatorSignature::new("v2".to_string(), vec![2], 101),
        ];
        let proof = StateProof::new("h".to_string(), 1, "r".to_string(), sigs, 2).unwrap();
        assert_eq!(proof.unique_signers(), 2);
    }

    #[test]
    fn test_bridge_check_expired() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        bridge.initiate_transfer(transfer).unwrap();
        assert!(!bridge.check_expired("tx1", 150).unwrap());
        assert!(bridge.check_expired("tx1", 250).unwrap());
    }

    #[test]
    fn test_bridge_transfers_by_state() {
        let mut bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let transfer = BridgeTransfer::new(
            "tx1".to_string(),
            ChainId::Ethereum,
            ChainId::Etrid,
            AssetId::wrapped_eth(),
            1000,
            "sender".to_string(),
            "recipient".to_string(),
            100,
            200,
        )
        .unwrap();

        bridge.initiate_transfer(transfer).unwrap();
        let initiated = bridge.transfers_by_state(TransferState::Initiated);
        assert_eq!(initiated.len(), 1);
    }

    #[test]
    fn test_bridge_verify_unique_signers() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string(), "v2".to_string()],
            1,
        )
        .unwrap();

        let sigs = vec![
            ValidatorSignature::new("v1".to_string(), vec![1], 100),
            ValidatorSignature::new("v2".to_string(), vec![2], 101),
        ];

        let result = bridge.verify_unique_signers(&sigs);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_bridge_duplicate_signature_error() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let sigs = vec![
            ValidatorSignature::new("v1".to_string(), vec![1], 100),
            ValidatorSignature::new("v1".to_string(), vec![2], 101),
        ];

        assert_eq!(bridge.verify_unique_signers(&sigs), Err(BridgeError::DuplicateSignature));
    }

    #[test]
    fn test_bridge_unknown_validator_error() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string()],
            1,
        )
        .unwrap();

        let sigs = vec![ValidatorSignature::new("v_unknown".to_string(), vec![1], 100)];
        assert_eq!(bridge.verify_unique_signers(&sigs), Err(BridgeError::ValidatorNotFound));
    }

    #[test]
    fn test_bridge_validator_count() {
        let bridge = CrossChainBridge::new(
            vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
            2,
        )
        .unwrap();

        assert_eq!(bridge.validator_count(), 3);
    }
}
