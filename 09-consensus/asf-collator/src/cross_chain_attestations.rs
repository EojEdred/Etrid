//! # Cross-Chain Attestations
//!
//! Implements cross-chain finality attestations for bridge security.
//!
//! PBC collators can attest to the finality of blocks on other PBCs,
//! enabling secure cross-chain bridges with ASF-backed security.

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

use crate::{
    Hash, BlockNumber, ParaId, CollatorId, CollatorFinalityLevel,
    AsfError, AsfResult, Balance,
};

/// Cross-chain attestation from one PBC to another
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct CrossChainAttestation {
    /// Source parachain (attesting)
    pub source_para: ParaId,
    /// Target parachain (being attested)
    pub target_para: ParaId,
    /// Target block hash
    pub target_block_hash: Hash,
    /// Target block number
    pub target_block_number: BlockNumber,
    /// Finality level observed
    pub finality_level: CollatorFinalityLevel,
    /// Attesting collator
    pub collator: CollatorId,
    /// Attestation timestamp (relay chain block)
    pub relay_block: BlockNumber,
    /// Collator signature
    pub signature: Vec<u8>,
}

impl CrossChainAttestation {
    /// Create new attestation
    pub fn new(
        source_para: ParaId,
        target_para: ParaId,
        target_block_hash: Hash,
        target_block_number: BlockNumber,
        finality_level: CollatorFinalityLevel,
        collator: CollatorId,
        relay_block: BlockNumber,
    ) -> Self {
        Self {
            source_para,
            target_para,
            target_block_hash,
            target_block_number,
            finality_level,
            collator,
            relay_block,
            signature: Vec::new(),
        }
    }

    /// Get attestation hash for signing
    pub fn attestation_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.source_para.to_le_bytes());
        data.extend_from_slice(&self.target_para.to_le_bytes());
        data.extend_from_slice(self.target_block_hash.as_ref());
        data.extend_from_slice(&self.target_block_number.to_le_bytes());
        data.push(self.finality_level as u8);
        data.extend_from_slice(self.collator.as_ref());
        data.extend_from_slice(&self.relay_block.to_le_bytes());
        BlakeTwo256::hash(&data)
    }
}

/// Multi-signature attestation (aggregated from multiple collators)
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct MultiSigAttestation {
    /// Target parachain
    pub target_para: ParaId,
    /// Target block hash
    pub target_block_hash: Hash,
    /// Target block number
    pub target_block_number: BlockNumber,
    /// Minimum finality level attested
    pub min_finality: CollatorFinalityLevel,
    /// Individual attestations
    pub attestations: Vec<CrossChainAttestation>,
    /// Total stake weight
    pub total_stake: Balance,
}

impl MultiSigAttestation {
    /// Create new multi-sig attestation
    pub fn new(
        target_para: ParaId,
        target_block_hash: Hash,
        target_block_number: BlockNumber,
    ) -> Self {
        Self {
            target_para,
            target_block_hash,
            target_block_number,
            min_finality: CollatorFinalityLevel::Irreversible,
            attestations: Vec::new(),
            total_stake: 0,
        }
    }

    /// Add attestation
    pub fn add_attestation(
        &mut self,
        attestation: CrossChainAttestation,
        stake: Balance,
    ) -> AsfResult<()> {
        // Verify attestation matches
        if attestation.target_para != self.target_para
            || attestation.target_block_hash != self.target_block_hash
            || attestation.target_block_number != self.target_block_number
        {
            return Err(AsfError::InvalidCertificate("Attestation mismatch"));
        }

        // Check for duplicate
        if self.attestations.iter().any(|a| a.collator == attestation.collator) {
            return Err(AsfError::DuplicateVote);
        }

        // Update minimum finality
        if attestation.finality_level < self.min_finality {
            self.min_finality = attestation.finality_level;
        }

        self.attestations.push(attestation);
        self.total_stake += stake;
        Ok(())
    }

    /// Check if multi-sig meets threshold
    pub fn meets_threshold(&self, required_stake: Balance) -> bool {
        self.total_stake >= required_stake
            && self.min_finality >= CollatorFinalityLevel::Strong
    }

    /// Get attestation hash
    pub fn multisig_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&self.target_para.to_le_bytes());
        data.extend_from_slice(self.target_block_hash.as_ref());
        data.extend_from_slice(&self.target_block_number.to_le_bytes());
        for att in &self.attestations {
            data.extend_from_slice(&att.attestation_hash().as_ref());
        }
        BlakeTwo256::hash(&data)
    }
}

/// Bridge security deposit configuration
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BridgeSecurityConfig {
    /// Minimum stake required for attestations
    pub min_attestation_stake: Balance,
    /// Slash amount for false attestations
    pub slash_amount: Balance,
    /// Challenge period (in relay chain blocks)
    pub challenge_period: BlockNumber,
    /// Minimum finality level required
    pub min_finality_level: CollatorFinalityLevel,
}

impl Default for BridgeSecurityConfig {
    fn default() -> Self {
        Self {
            min_attestation_stake: 1_000_000_000_000, // 1M ETR
            slash_amount: 10_000_000_000_000, // 10M ETR (10x deposit)
            challenge_period: 100, // 100 relay chain blocks (~10 minutes)
            min_finality_level: CollatorFinalityLevel::Strong,
        }
    }
}

/// Bridge attestation record
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct BridgeAttestationRecord {
    /// Multi-sig attestation
    pub attestation: MultiSigAttestation,
    /// When attestation was submitted
    pub submitted_at: BlockNumber,
    /// Challenge deadline
    pub challenge_deadline: BlockNumber,
    /// Whether attestation was challenged
    pub challenged: bool,
    /// Whether attestation is finalized
    pub finalized: bool,
}

/// Manages bridge attestations and security
pub struct BridgeSecurityManager {
    config: BridgeSecurityConfig,
    /// Pending attestations (block_hash -> record)
    pending: BTreeMap<Hash, BridgeAttestationRecord>,
    /// Finalized attestations
    finalized: BTreeMap<Hash, MultiSigAttestation>,
}

impl BridgeSecurityManager {
    /// Create new bridge security manager
    pub fn new(config: BridgeSecurityConfig) -> Self {
        Self {
            config,
            pending: BTreeMap::new(),
            finalized: BTreeMap::new(),
        }
    }

    /// Submit bridge attestation
    pub fn submit_attestation(
        &mut self,
        attestation: MultiSigAttestation,
        current_relay_block: BlockNumber,
    ) -> AsfResult<()> {
        // Verify meets minimum requirements
        if !attestation.meets_threshold(self.config.min_attestation_stake) {
            return Err(AsfError::InsufficientStake {
                got: attestation.total_stake,
                need: self.config.min_attestation_stake,
            });
        }

        if attestation.min_finality < self.config.min_finality_level {
            return Err(AsfError::InvalidCertificate("Insufficient finality"));
        }

        let block_hash = attestation.target_block_hash;

        // Check if already exists
        if self.pending.contains_key(&block_hash) || self.finalized.contains_key(&block_hash) {
            return Err(AsfError::DuplicateVote);
        }

        // Create record with challenge period
        let record = BridgeAttestationRecord {
            attestation,
            submitted_at: current_relay_block,
            challenge_deadline: current_relay_block + self.config.challenge_period,
            challenged: false,
            finalized: false,
        };

        self.pending.insert(block_hash, record);
        Ok(())
    }

    /// Challenge an attestation (if fraudulent)
    pub fn challenge_attestation(
        &mut self,
        block_hash: Hash,
        current_relay_block: BlockNumber,
    ) -> AsfResult<()> {
        let record = self.pending.get_mut(&block_hash)
            .ok_or(AsfError::BlockNotFound)?;

        if current_relay_block > record.challenge_deadline {
            return Err(AsfError::InvalidCertificate("Challenge period expired"));
        }

        if record.challenged {
            return Err(AsfError::DuplicateVote);
        }

        record.challenged = true;
        Ok(())
    }

    /// Finalize attestation after challenge period
    pub fn finalize_attestation(
        &mut self,
        block_hash: Hash,
        current_relay_block: BlockNumber,
    ) -> AsfResult<()> {
        let mut record = self.pending.remove(&block_hash)
            .ok_or(AsfError::BlockNotFound)?;

        if current_relay_block < record.challenge_deadline {
            // Put it back
            self.pending.insert(block_hash, record);
            return Err(AsfError::InvalidCertificate("Challenge period not ended"));
        }

        if record.challenged {
            // Challenged attestations are not finalized
            return Err(AsfError::SafetyViolation("Attestation was challenged"));
        }

        record.finalized = true;
        self.finalized.insert(block_hash, record.attestation);
        Ok(())
    }

    /// Get pending attestation
    pub fn get_pending(&self, block_hash: &Hash) -> Option<&BridgeAttestationRecord> {
        self.pending.get(block_hash)
    }

    /// Get finalized attestation
    pub fn get_finalized(&self, block_hash: &Hash) -> Option<&MultiSigAttestation> {
        self.finalized.get(block_hash)
    }

    /// Process expired attestations
    pub fn process_expired(&mut self, current_relay_block: BlockNumber) -> Vec<Hash> {
        let mut finalized = Vec::new();

        let expired: Vec<_> = self.pending.iter()
            .filter(|(_, record)| {
                current_relay_block >= record.challenge_deadline && !record.challenged
            })
            .map(|(hash, _)| *hash)
            .collect();

        for hash in expired {
            if self.finalize_attestation(hash, current_relay_block).is_ok() {
                finalized.push(hash);
            }
        }

        finalized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    #[test]
    fn test_cross_chain_attestation() {
        let collator = AccountId32::new([1u8; 32]);
        let block_hash = H256::random();

        let attestation = CrossChainAttestation::new(
            1000, // source para
            2000, // target para
            block_hash,
            100,
            CollatorFinalityLevel::Strong,
            collator,
            5000, // relay block
        );

        let hash1 = attestation.attestation_hash();
        let hash2 = attestation.attestation_hash();
        assert_eq!(hash1, hash2); // Deterministic
    }

    #[test]
    fn test_multisig_attestation() {
        let block_hash = H256::random();
        let mut multisig = MultiSigAttestation::new(2000, block_hash, 100);

        // Add attestations from 3 collators
        for i in 0..3 {
            let collator = AccountId32::new([i as u8; 32]);
            let attestation = CrossChainAttestation::new(
                1000,
                2000,
                block_hash,
                100,
                CollatorFinalityLevel::Strong,
                collator,
                5000,
            );
            multisig.add_attestation(attestation, 500_000_000_000).unwrap();
        }

        assert_eq!(multisig.attestations.len(), 3);
        assert_eq!(multisig.total_stake, 1_500_000_000_000);
        assert!(multisig.meets_threshold(1_000_000_000_000));
    }

    #[test]
    fn test_bridge_security_manager() {
        let config = BridgeSecurityConfig::default();
        let mut manager = BridgeSecurityManager::new(config);

        let block_hash = H256::random();
        let mut multisig = MultiSigAttestation::new(2000, block_hash, 100);

        // Add sufficient attestations
        for i in 0..5 {
            let collator = AccountId32::new([i as u8; 32]);
            let attestation = CrossChainAttestation::new(
                1000,
                2000,
                block_hash,
                100,
                CollatorFinalityLevel::Strong,
                collator,
                5000,
            );
            multisig.add_attestation(attestation, 300_000_000_000).unwrap();
        }

        // Submit attestation
        manager.submit_attestation(multisig, 1000).unwrap();

        // Should be pending
        assert!(manager.get_pending(&block_hash).is_some());
        assert!(manager.get_finalized(&block_hash).is_none());

        // Cannot finalize before challenge period
        assert!(manager.finalize_attestation(block_hash, 1050).is_err());

        // Finalize after challenge period
        manager.finalize_attestation(block_hash, 1101).unwrap();
        assert!(manager.get_finalized(&block_hash).is_some());
    }

    #[test]
    fn test_attestation_challenge() {
        let config = BridgeSecurityConfig::default();
        let mut manager = BridgeSecurityManager::new(config);

        let block_hash = H256::random();
        let mut multisig = MultiSigAttestation::new(2000, block_hash, 100);

        for i in 0..5 {
            let collator = AccountId32::new([i as u8; 32]);
            let attestation = CrossChainAttestation::new(
                1000,
                2000,
                block_hash,
                100,
                CollatorFinalityLevel::Strong,
                collator,
                5000,
            );
            multisig.add_attestation(attestation, 300_000_000_000).unwrap();
        }

        manager.submit_attestation(multisig, 1000).unwrap();

        // Challenge the attestation
        manager.challenge_attestation(block_hash, 1050).unwrap();

        // Should fail to finalize
        assert!(manager.finalize_attestation(block_hash, 1101).is_err());
    }
}
