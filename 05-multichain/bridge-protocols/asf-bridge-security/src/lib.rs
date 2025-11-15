//! # ASF Bridge Security
//!
//! Cryptographically provable bridge security using ASF consensus.
//!
//! Features:
//! - Multi-signature attestations for cross-chain transfers
//! - Economic security deposits with slashing
//! - Byzantine fault tolerance for bridge operations
//! - Merkle proof verification for cross-chain state

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

pub mod merkle_proof;
pub mod slash_conditions;
pub mod economic_security;

pub use merkle_proof::*;
pub use slash_conditions::*;
pub use economic_security::*;

use asf_collator::{
    Hash, BlockNumber, ParaId, CollatorId, MultiSigAttestation,
    CrossChainAttestation, CollatorFinalityLevel, Balance,
    BridgeSecurityConfig, AsfError, AsfResult,
};

/// Bridge transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum BridgeDirection {
    /// Transfer from source chain to target chain
    Outbound,
    /// Transfer from target chain back to source chain
    Inbound,
}

/// Cross-chain bridge transfer request
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BridgeTransfer {
    /// Unique transfer ID
    pub transfer_id: H256,
    /// Source parachain
    pub source_para: ParaId,
    /// Target parachain
    pub target_para: ParaId,
    /// Source block where transfer originated
    pub source_block: Hash,
    /// Source block number
    pub source_block_number: BlockNumber,
    /// Asset being transferred (token address or ID)
    pub asset: H256,
    /// Amount being transferred
    pub amount: Balance,
    /// Sender on source chain
    pub sender: Vec<u8>,
    /// Recipient on target chain
    pub recipient: Vec<u8>,
    /// Direction of transfer
    pub direction: BridgeDirection,
}

impl BridgeTransfer {
    /// Get transfer hash
    pub fn transfer_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(self.transfer_id.as_ref());
        data.extend_from_slice(&self.source_para.to_le_bytes());
        data.extend_from_slice(&self.target_para.to_le_bytes());
        data.extend_from_slice(self.source_block.as_ref());
        data.extend_from_slice(&self.source_block_number.to_le_bytes());
        data.extend_from_slice(self.asset.as_ref());
        data.extend_from_slice(&self.amount.to_le_bytes());
        data.extend_from_slice(&self.sender);
        data.extend_from_slice(&self.recipient);
        BlakeTwo256::hash(&data)
    }
}

/// Bridge security proof (ASF-backed)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BridgeSecurityProof {
    /// The transfer being proven
    pub transfer: BridgeTransfer,
    /// Multi-sig attestation from source chain collators
    pub source_attestation: MultiSigAttestation,
    /// Merkle proof of transfer in source block
    pub merkle_proof: MerkleProof,
    /// Economic deposits backing this transfer
    pub economic_deposits: Vec<EconomicDeposit>,
    /// Total security value
    pub total_security: Balance,
}

impl BridgeSecurityProof {
    /// Verify the bridge security proof
    pub fn verify(
        &self,
        config: &BridgeSecurityConfig,
        min_security: Balance,
    ) -> AsfResult<()> {
        // Verify attestation meets threshold
        if !self.source_attestation.meets_threshold(config.min_attestation_stake) {
            return Err(AsfError::InsufficientStake {
                got: self.source_attestation.total_stake,
                need: config.min_attestation_stake,
            });
        }

        // Verify finality level
        if self.source_attestation.min_finality < config.min_finality_level {
            return Err(AsfError::InvalidCertificate("Insufficient finality"));
        }

        // Verify merkle proof
        if !self.merkle_proof.verify(
            self.transfer.transfer_hash(),
            self.transfer.source_block,
        ) {
            return Err(AsfError::InvalidCertificate("Invalid merkle proof"));
        }

        // Verify economic security
        if self.total_security < min_security {
            return Err(AsfError::InsufficientStake {
                got: self.total_security,
                need: min_security,
            });
        }

        Ok(())
    }

    /// Get proof hash
    pub fn proof_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.transfer.transfer_hash().as_ref());
        data.extend_from_slice(&self.source_attestation.multisig_hash().as_ref());
        data.extend_from_slice(&self.merkle_proof.root.as_ref());
        BlakeTwo256::hash(&data)
    }
}

/// Bridge relay (cross-chain message relay)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BridgeRelay {
    /// Source parachain
    pub source_para: ParaId,
    /// Target parachain
    pub target_para: ParaId,
    /// Pending transfers
    pub pending: Vec<BridgeTransfer>,
    /// Completed transfers
    pub completed: BTreeMap<Hash, BridgeSecurityProof>,
    /// Failed/challenged transfers
    pub failed: BTreeMap<Hash, SlashRecord>,
}

impl BridgeRelay {
    /// Create new bridge relay
    pub fn new(source_para: ParaId, target_para: ParaId) -> Self {
        Self {
            source_para,
            target_para,
            pending: Vec::new(),
            completed: BTreeMap::new(),
            failed: BTreeMap::new(),
        }
    }

    /// Submit transfer for relaying
    pub fn submit_transfer(&mut self, transfer: BridgeTransfer) -> AsfResult<()> {
        // Verify direction
        if transfer.source_para != self.source_para
            || transfer.target_para != self.target_para
        {
            return Err(AsfError::InvalidVote("Invalid chain pair"));
        }

        // Check for duplicate
        let transfer_hash = transfer.transfer_hash();
        if self.completed.contains_key(&transfer_hash)
            || self.failed.contains_key(&transfer_hash)
        {
            return Err(AsfError::DuplicateVote);
        }

        self.pending.push(transfer);
        Ok(())
    }

    /// Complete transfer with security proof
    pub fn complete_transfer(
        &mut self,
        proof: BridgeSecurityProof,
        config: &BridgeSecurityConfig,
        min_security: Balance,
    ) -> AsfResult<()> {
        // Verify proof
        proof.verify(config, min_security)?;

        let transfer_hash = proof.transfer.transfer_hash();

        // Remove from pending
        self.pending.retain(|t| t.transfer_hash() != transfer_hash);

        // Add to completed
        self.completed.insert(transfer_hash, proof);

        Ok(())
    }

    /// Challenge transfer (if fraudulent)
    pub fn challenge_transfer(
        &mut self,
        transfer_hash: Hash,
        slash_record: SlashRecord,
    ) -> AsfResult<()> {
        // Remove from completed if exists
        self.completed.remove(&transfer_hash);

        // Remove from pending
        self.pending.retain(|t| t.transfer_hash() != transfer_hash);

        // Add to failed
        self.failed.insert(transfer_hash, slash_record);

        Ok(())
    }

    /// Get transfer status
    pub fn get_status(&self, transfer_hash: &Hash) -> TransferStatus {
        if self.completed.contains_key(transfer_hash) {
            TransferStatus::Completed
        } else if self.failed.contains_key(transfer_hash) {
            TransferStatus::Failed
        } else if self.pending.iter().any(|t| &t.transfer_hash() == transfer_hash) {
            TransferStatus::Pending
        } else {
            TransferStatus::Unknown
        }
    }
}

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    Unknown,
    Pending,
    Completed,
    Failed,
}

/// Bridge manager for all PBC bridges
pub struct BridgeManager {
    /// Bridge relays (source_para -> target_para -> relay)
    relays: BTreeMap<(ParaId, ParaId), BridgeRelay>,
    /// Security config
    config: BridgeSecurityConfig,
}

impl BridgeManager {
    /// Create new bridge manager
    pub fn new(config: BridgeSecurityConfig) -> Self {
        Self {
            relays: BTreeMap::new(),
            config,
        }
    }

    /// Get or create relay
    pub fn get_relay_mut(
        &mut self,
        source_para: ParaId,
        target_para: ParaId,
    ) -> &mut BridgeRelay {
        self.relays
            .entry((source_para, target_para))
            .or_insert_with(|| BridgeRelay::new(source_para, target_para))
    }

    /// Submit bridge transfer
    pub fn submit_transfer(&mut self, transfer: BridgeTransfer) -> AsfResult<()> {
        let relay = self.get_relay_mut(transfer.source_para, transfer.target_para);
        relay.submit_transfer(transfer)
    }

    /// Complete transfer with proof
    pub fn complete_transfer(
        &mut self,
        proof: BridgeSecurityProof,
    ) -> AsfResult<()> {
        let min_security = self.calculate_min_security(&proof.transfer);
        let relay = self.get_relay_mut(
            proof.transfer.source_para,
            proof.transfer.target_para,
        );
        relay.complete_transfer(proof, &self.config, min_security)
    }

    /// Calculate minimum security required for transfer
    fn calculate_min_security(&self, transfer: &BridgeTransfer) -> Balance {
        // Security = 2x transfer value (minimum)
        transfer.amount.saturating_mul(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_transfer_hash() {
        let transfer = BridgeTransfer {
            transfer_id: H256::random(),
            source_para: 1000,
            target_para: 2000,
            source_block: H256::random(),
            source_block_number: 100,
            asset: H256::zero(),
            amount: 1_000_000,
            sender: vec![1, 2, 3],
            recipient: vec![4, 5, 6],
            direction: BridgeDirection::Outbound,
        };

        let hash1 = transfer.transfer_hash();
        let hash2 = transfer.transfer_hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_bridge_relay() {
        let mut relay = BridgeRelay::new(1000, 2000);

        let transfer = BridgeTransfer {
            transfer_id: H256::random(),
            source_para: 1000,
            target_para: 2000,
            source_block: H256::random(),
            source_block_number: 100,
            asset: H256::zero(),
            amount: 1_000_000,
            sender: vec![1, 2, 3],
            recipient: vec![4, 5, 6],
            direction: BridgeDirection::Outbound,
        };

        relay.submit_transfer(transfer.clone()).unwrap();

        let transfer_hash = transfer.transfer_hash();
        assert_eq!(relay.get_status(&transfer_hash), TransferStatus::Pending);
    }

    #[test]
    fn test_bridge_manager() {
        let config = BridgeSecurityConfig::default();
        let mut manager = BridgeManager::new(config);

        let transfer = BridgeTransfer {
            transfer_id: H256::random(),
            source_para: 1000,
            target_para: 2000,
            source_block: H256::random(),
            source_block_number: 100,
            asset: H256::zero(),
            amount: 1_000_000,
            sender: vec![1, 2, 3],
            recipient: vec![4, 5, 6],
            direction: BridgeDirection::Outbound,
        };

        manager.submit_transfer(transfer).unwrap();
    }
}
