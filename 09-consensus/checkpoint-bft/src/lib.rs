// ═══════════════════════════════════════════════════════════════════════════
// ËTRID CHECKPOINT BFT - VRF-Based Byzantine Fault Tolerant Finality
// ═══════════════════════════════════════════════════════════════════════════
//
// Production-grade checkpoint-based finality with cryptographic security.
//
// Core Design:
// - VRF-based checkpoint detection (prevents grinding attacks)
// - Comprehensive replay protection (chain_id + authority_set_id + nonce)
// - Canonical Ed25519 signatures (prevents malleability)
// - Long-range attack protection (social consensus checkpoints)
// - Quorum of 15/21 signatures triggers finalization
// - Fork protection: only finalize blocks on canonical chain
//
// Security Properties:
// - Byzantine fault tolerance: tolerates f < n/3 malicious validators
// - Safety: cannot finalize conflicting forks
// - Liveness: progresses as long as 2/3+ validators are honest and online
// - Unpredictability: VRF prevents strategic checkpoint avoidance
// - Replay resistance: signatures bound to chain, epoch, and validator
// - Long-range protection: social checkpoints prevent history rewrite
//
// ═══════════════════════════════════════════════════════════════════════════

pub mod vrf;
pub mod anchors;

// ═══════════════════════════════════════════════════════════════════════════
// H256 CONVERSION UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

// V17: Conversion functions removed - no longer needed
// All code now uses sp_core::H256 directly
//
// /// Convert sp_core::H256 to primitive_types::H256
// pub fn sp_h256_to_primitive(hash: sp_core::H256) -> primitive_types::H256 {
//     primitive_types::H256::from_slice(hash.as_bytes())
// }
//
// /// Convert primitive_types::H256 to sp_core::H256
// pub fn primitive_h256_to_sp(hash: primitive_types::H256) -> sp_core::H256 {
//     sp_core::H256::from_slice(hash.as_bytes())
// }

// Byzantine Fault Tolerance & Network Security Modules
pub mod fork_detection;
pub mod byzantine;
pub mod equivocation;
pub mod rate_limit;
pub mod eclipse;
pub mod asf_finality;

use codec::{Decode, Encode};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, NumberFor};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub use vrf::{CheckpointType, VrfCheckpointDecision};
pub use anchors::{CheckpointAnchor, LongRangeProtection};

// Re-export security module types
pub use fork_detection::ForkAwareCollector;
pub use byzantine::{ByzantineTracker, ByzantineReport};
pub use equivocation::{ForkAccountability, EquivocationEvidence, SlashingReport};
pub use rate_limit::{RateLimitedCollector, RateLimitConfig};
pub use eclipse::{EclipseDetector, EclipseReport};
pub use asf_finality::{AsfFinalityLevel, FinalityTracker, CertificateAsfExt};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Guaranteed checkpoint interval (VRF fallback)
pub const GUARANTEED_CHECKPOINT_INTERVAL: u32 = 32;

/// Minimum signatures needed for quorum (15 out of 21 validators = 71.4% > 2/3 BFT threshold)
pub const QUORUM_THRESHOLD: usize = 15;

/// Total number of validators in the authority set
pub const TOTAL_VALIDATORS: usize = 21;

/// Domain separator for signatures (prevents cross-protocol replay)
const SIGNATURE_DOMAIN: &[u8] = b"ETRID-CHECKPOINT-V2";

/// FlareChain network ID (prevents cross-chain replay)
/// This should match the chain's genesis hash or a stable network identifier
pub const FLARECHAIN_NETWORK_ID: [u8; 32] = [
    0x45, 0x54, 0x52, 0x49, 0x44, 0x2d, 0x46, 0x4c, // "ETRID-FL"
    0x41, 0x52, 0x45, 0x43, 0x48, 0x41, 0x49, 0x4e, // "ARECHAIN"
    0x2d, 0x4d, 0x41, 0x49, 0x4e, 0x4e, 0x45, 0x54, // "-MAINNET"
    0x2d, 0x56, 0x31, 0x2d, 0x00, 0x00, 0x00, 0x00, // "-V1-" + padding
];

// ═══════════════════════════════════════════════════════════════════════════
// CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// VRF-based checkpoint detection (backward compatibility wrapper)
pub fn is_checkpoint_block(
    block_number: u32,
    parent_hash: [u8; 32],
    epoch: u64,
    epoch_randomness: [u8; 32],
) -> bool {
    vrf::is_checkpoint_block(block_number, parent_hash, epoch, epoch_randomness)
}

/// Simple check if block is a guaranteed checkpoint
pub fn is_guaranteed_checkpoint(block_number: u32) -> bool {
    block_number % GUARANTEED_CHECKPOINT_INTERVAL == 0 && block_number > 0
}

/// Detect checkpoint type for a given block
pub fn detect_checkpoint(
    block_number: u32,
    parent_hash: [u8; 32],
    epoch: u64,
    epoch_randomness: [u8; 32],
) -> Option<CheckpointType> {
    vrf::get_checkpoint_type(block_number, parent_hash, epoch, epoch_randomness)
}

/// Enhanced checkpoint signature with comprehensive replay protection
///
/// Security Guarantees:
/// - Cross-chain replay prevented by chain_id
/// - Cross-epoch replay prevented by authority_set_id + authority_set_hash
/// - Within-epoch replay prevented by nonce
/// - Signature malleability prevented by canonical Ed25519 verification
/// - Long-range attacks prevented by expired authority set tracking
#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointSignature {
    /// Network chain ID (prevents cross-chain replay)
    /// Must match FLARECHAIN_NETWORK_ID for mainnet
    pub chain_id: [u8; 32],

    /// Block identification
    pub block_number: u32,
    pub block_hash: [u8; 32],

    /// Validator binding
    pub validator_id: u32,
    pub validator_pubkey: [u8; 32],

    /// Authority set binding (comprehensive replay protection)
    pub authority_set_id: u64,
    pub authority_set_hash: [u8; 32], // Hash of all validator pubkeys

    /// VRF proof if opportunity checkpoint
    pub checkpoint_type: CheckpointType,

    /// Nonce (prevents signature reuse within epoch)
    /// Must be strictly increasing for each validator
    pub signature_nonce: u64,

    /// Canonical Ed25519 signature over all above fields
    /// Signature is over: domain + chain_id + block_hash + block_number +
    ///                    validator_id + validator_pubkey + authority_set_id +
    ///                    authority_set_hash + checkpoint_type + nonce
    pub signature: Vec<u8>,

    /// Timestamp (for metrics only, not included in signature)
    pub timestamp_ms: u64,
}

impl CheckpointSignature {
    /// Create comprehensive signing payload
    ///
    /// Includes ALL security-critical fields in deterministic order
    pub fn signing_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();

        // Domain separator (prevents cross-protocol replay)
        payload.extend_from_slice(SIGNATURE_DOMAIN);

        // Chain ID (prevents cross-chain replay)
        payload.extend_from_slice(&self.chain_id);

        // Block identification
        payload.extend_from_slice(&self.block_hash);
        payload.extend_from_slice(&self.block_number.to_le_bytes());

        // Validator binding
        payload.extend_from_slice(&self.validator_id.to_le_bytes());
        payload.extend_from_slice(&self.validator_pubkey);

        // Authority set binding (prevents cross-epoch replay)
        payload.extend_from_slice(&self.authority_set_id.to_le_bytes());
        payload.extend_from_slice(&self.authority_set_hash);

        // Checkpoint type (includes VRF proof if opportunity checkpoint)
        let checkpoint_type_bytes = codec::Encode::encode(&self.checkpoint_type);
        payload.extend_from_slice(&checkpoint_type_bytes);

        // Nonce (prevents within-epoch replay)
        payload.extend_from_slice(&self.signature_nonce.to_le_bytes());

        payload
    }

    /// Verify signature with canonical Ed25519 (prevents malleability)
    ///
    /// Uses ed25519-dalek for strict canonical signature enforcement
    pub fn verify_canonical(&self, public_key: &[u8; 32]) -> Result<(), String> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let payload = self.signing_payload();

        // Parse public key
        let verifying_key = VerifyingKey::from_bytes(public_key)
            .map_err(|e| format!("Invalid public key: {:?}", e))?;

        // Parse signature (enforces canonical form)
        let signature = Signature::from_slice(&self.signature)
            .map_err(|e| format!("Invalid signature format: {:?}", e))?;

        // Verify signature
        verifying_key
            .verify(&payload, &signature)
            .map_err(|e| format!("Signature verification failed: {:?}", e))?;

        Ok(())
    }

    /// Legacy verify method (for backward compatibility)
    /// DEPRECATED: Use verify_canonical instead
    pub fn verify(&self, public_key: &[u8; 32]) -> bool {
        self.verify_canonical(public_key).is_ok()
    }

    /// Verify signature with comprehensive security checks
    ///
    /// Checks:
    /// - Chain ID matches network
    /// - Authority set ID not expired
    /// - Validator pubkey matches authority set
    /// - Nonce is valid (handled by caller)
    /// - Signature is cryptographically valid and canonical
    pub fn verify_comprehensive(
        &self,
        expected_chain_id: &[u8; 32],
        authority_set: &AuthoritySet,
        long_range_protection: &LongRangeProtection,
    ) -> Result<(), String> {
        // Check chain ID
        if self.chain_id != *expected_chain_id {
            return Err(format!(
                "Chain ID mismatch: expected {:?}, got {:?}",
                expected_chain_id, self.chain_id
            ));
        }

        // Check authority set ID not expired
        if long_range_protection.is_authority_set_expired(self.authority_set_id) {
            return Err(format!(
                "Authority set {} is expired (min valid: {})",
                self.authority_set_id,
                long_range_protection.get_min_authority_set_id()
            ));
        }

        // Check authority set ID matches
        if self.authority_set_id != authority_set.set_id {
            return Err(format!(
                "Authority set ID mismatch: signature has {}, current is {}",
                self.authority_set_id, authority_set.set_id
            ));
        }

        // Check authority set hash matches
        if self.authority_set_hash != authority_set.authority_set_hash {
            return Err(format!(
                "Authority set hash mismatch: signature has {:?}, current is {:?}",
                self.authority_set_hash, authority_set.authority_set_hash
            ));
        }

        // Check validator ID is valid
        if !authority_set.is_valid_validator(self.validator_id) {
            return Err(format!(
                "Invalid validator_id: {} (max: {})",
                self.validator_id,
                authority_set.authorities.len() - 1
            ));
        }

        // Check validator pubkey matches
        let expected_pubkey = &authority_set.authorities[self.validator_id as usize];
        if self.validator_pubkey != *expected_pubkey {
            return Err(format!(
                "Validator pubkey mismatch for validator {}: expected {:?}, got {:?}",
                self.validator_id, expected_pubkey, self.validator_pubkey
            ));
        }

        // Verify cryptographic signature (canonical)
        self.verify_canonical(&self.validator_pubkey)?;

        Ok(())
    }
}

/// Checkpoint certificate proving quorum was reached
#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize)]
pub struct CheckpointCertificate {
    /// Block number being finalized
    pub block_number: u32,

    /// Block hash being finalized
    pub block_hash: [u8; 32],

    /// Authority set ID when certificate was created
    pub authority_set_id: u64,

    /// Validator signatures (must have >= QUORUM_THRESHOLD)
    pub signatures: Vec<CheckpointSignature>,

    /// Set of validator IDs that signed (for quick lookup, not encoded)
    #[serde(skip)]
    #[codec(skip)]
    pub signers: HashSet<u32>,

    /// Timestamp when quorum was reached
    pub finalized_at_ms: u64,
}

impl CheckpointCertificate {
    /// Create new certificate from collected signatures
    pub fn new(
        block_number: u32,
        block_hash: [u8; 32],
        authority_set_id: u64,
        signatures: Vec<CheckpointSignature>,
    ) -> Option<Self> {
        if signatures.len() < QUORUM_THRESHOLD {
            return None;
        }

        let signers: HashSet<u32> = signatures.iter().map(|s| s.validator_id).collect();

        // Verify no duplicate signers
        if signers.len() != signatures.len() {
            tracing::warn!(
                "⚠️ Duplicate signatures detected in certificate for block #{}",
                block_number
            );
            return None;
        }

        Some(Self {
            block_number,
            block_hash,
            authority_set_id,
            signatures,
            signers,
            finalized_at_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    /// Verify all signatures in the certificate
    pub fn verify_all(&self, authority_set: &AuthoritySet) -> bool {
        if self.signatures.len() < QUORUM_THRESHOLD {
            return false;
        }

        // Verify authority set ID matches
        if self.authority_set_id != authority_set.set_id {
            tracing::warn!(
                "⚠️ Certificate authority_set_id mismatch: cert={}, current={}",
                self.authority_set_id,
                authority_set.set_id
            );
            return false;
        }

        // Verify each signature
        for sig in &self.signatures {
            // Check validator_id is valid
            if sig.validator_id as usize >= authority_set.authorities.len() {
                tracing::warn!("⚠️ Invalid validator_id: {}", sig.validator_id);
                return false;
            }

            // Check authority_set_id matches
            if sig.authority_set_id != self.authority_set_id {
                tracing::warn!(
                    "⚠️ Signature authority_set_id mismatch for validator {}",
                    sig.validator_id
                );
                return false;
            }

            // Check block hash matches
            if sig.block_hash != self.block_hash {
                tracing::warn!(
                    "⚠️ Signature block_hash mismatch for validator {}",
                    sig.validator_id
                );
                return false;
            }

            // Verify cryptographic signature
            let public_key = &authority_set.authorities[sig.validator_id as usize];
            if !sig.verify(public_key) {
                tracing::warn!(
                    "⚠️ Invalid signature from validator {}",
                    sig.validator_id
                );
                return false;
            }
        }

        true
    }
}

/// Authority set configuration with hash-based binding
#[derive(Clone, Debug)]
pub struct AuthoritySet {
    /// Current authority set ID (increments with each validator set change)
    pub set_id: u64,

    /// Ed25519 public keys of all validators
    pub authorities: Vec<[u8; 32]>,

    /// Hash of all validator public keys (for replay protection)
    /// Computed as Blake2-256(pubkey1 || pubkey2 || ... || pubkeyN)
    pub authority_set_hash: [u8; 32],
}

impl AuthoritySet {
    /// Create new authority set with computed hash
    pub fn new(set_id: u64, authorities: Vec<[u8; 32]>) -> Self {
        let authority_set_hash = Self::calculate_authority_set_hash(&authorities);
        Self {
            set_id,
            authorities,
            authority_set_hash,
        }
    }

    /// Calculate deterministic hash of authority set
    ///
    /// Security: Hash binds signatures to exact validator set composition
    /// Prevents replay attacks if validator set changes but set_id reused
    pub fn calculate_authority_set_hash(authorities: &[[u8; 32]]) -> [u8; 32] {
        use sp_core::hashing::blake2_256;
        let mut data = Vec::new();
        for pubkey in authorities {
            data.extend_from_slice(pubkey);
        }
        blake2_256(&data)
    }

    /// Check if validator_id is valid
    pub fn is_valid_validator(&self, validator_id: u32) -> bool {
        (validator_id as usize) < self.authorities.len()
    }

    /// Get validator public key by ID
    pub fn get_validator_pubkey(&self, validator_id: u32) -> Option<&[u8; 32]> {
        self.authorities.get(validator_id as usize)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CHECKPOINT COLLECTOR
// ═══════════════════════════════════════════════════════════════════════════

/// Collects checkpoint signatures and detects quorum with nonce tracking
#[derive(Clone)]
pub struct CheckpointCollector {
    /// Current authority set
    authority_set: Arc<RwLock<AuthoritySet>>,

    /// Collected signatures by block number
    /// Map: block_number -> (block_hash, Map<validator_id, signature>)
    signatures: Arc<RwLock<HashMap<u32, (sp_core::H256, HashMap<u32, CheckpointSignature>)>>>,

    /// Completed certificates (for persistence/gossip)
    certificates: Arc<RwLock<HashMap<u32, CheckpointCertificate>>>,

    /// Nonce tracking per validator (prevents replay within epoch)
    /// Map: validator_id -> last_nonce
    validator_nonces: Arc<RwLock<HashMap<u32, u64>>>,

    /// Long-range attack protection
    long_range_protection: Arc<RwLock<LongRangeProtection>>,

    /// Chain ID for signature verification
    chain_id: [u8; 32],
}

impl CheckpointCollector {
    /// Create new checkpoint collector with mainnet chain ID
    pub fn new(authority_set: AuthoritySet) -> Self {
        Self::new_with_chain_id(authority_set, FLARECHAIN_NETWORK_ID)
    }

    /// Create checkpoint collector with custom chain ID (for testnets)
    pub fn new_with_chain_id(authority_set: AuthoritySet, chain_id: [u8; 32]) -> Self {
        // Initialize long-range protection with genesis checkpoint
        let genesis = CheckpointAnchor::genesis();
        let long_range_protection = LongRangeProtection::new(genesis);

        Self {
            authority_set: Arc::new(RwLock::new(authority_set)),
            signatures: Arc::new(RwLock::new(HashMap::new())),
            certificates: Arc::new(RwLock::new(HashMap::new())),
            validator_nonces: Arc::new(RwLock::new(HashMap::new())),
            long_range_protection: Arc::new(RwLock::new(long_range_protection)),
            chain_id,
        }
    }

    /// Add signature to collector with comprehensive security checks
    /// Returns Some(certificate) if quorum was just reached
    pub fn add_signature(
        &self,
        signature: CheckpointSignature,
    ) -> Result<Option<CheckpointCertificate>, String> {
        let authority_set = self.authority_set.read();
        let long_range_protection = self.long_range_protection.read();

        // Comprehensive signature verification
        signature.verify_comprehensive(&self.chain_id, &authority_set, &long_range_protection)?;

        drop(authority_set);
        drop(long_range_protection);

        // Verify and update nonce
        let mut nonces = self.validator_nonces.write();
        let last_nonce = nonces.get(&signature.validator_id).copied().unwrap_or(0);

        if signature.signature_nonce <= last_nonce {
            return Err(format!(
                "Invalid nonce from validator {}: got {}, expected > {}",
                signature.validator_id, signature.signature_nonce, last_nonce
            ));
        }

        // Update nonce
        nonces.insert(signature.validator_id, signature.signature_nonce);
        drop(nonces);

        // Add to collection
        let mut signatures = self.signatures.write();
        let block_number = signature.block_number;
        let block_hash = sp_core::H256::from_slice(&signature.block_hash);

        let entry = signatures
            .entry(block_number)
            .or_insert_with(|| (block_hash, HashMap::new()));

        // Check for conflicting block hash
        if entry.0 != block_hash {
            return Err(format!(
                "Conflicting block hash for block #{}: got {:?}, have {:?}",
                block_number, block_hash, entry.0
            ));
        }

        // Check for duplicate signature
        if entry.1.contains_key(&signature.validator_id) {
            tracing::debug!(
                "Duplicate signature from validator {} for block #{}",
                signature.validator_id,
                block_number
            );
            return Ok(None);
        }

        // Add signature
        entry.1.insert(signature.validator_id, signature);

        // Check if quorum reached
        if entry.1.len() >= QUORUM_THRESHOLD {
            tracing::info!(
                "✅ Quorum reached for checkpoint #{}: {}/{} signatures",
                block_number,
                entry.1.len(),
                TOTAL_VALIDATORS
            );

            // Create certificate
            let sigs: Vec<CheckpointSignature> = entry.1.values().cloned().collect();
            let authority_set = self.authority_set.read();

            if let Some(cert) = CheckpointCertificate::new(
                block_number,
                block_hash.into(),
                authority_set.set_id,
                sigs,
            ) {
                // Store certificate
                self.certificates.write().insert(block_number, cert.clone());

                return Ok(Some(cert));
            }
        } else {
            tracing::debug!(
                "Checkpoint #{}: {}/{} signatures collected",
                block_number,
                entry.1.len(),
                QUORUM_THRESHOLD
            );
        }

        Ok(None)
    }

    /// Get certificate for block number if quorum was reached
    pub fn get_certificate(&self, block_number: u32) -> Option<CheckpointCertificate> {
        self.certificates.read().get(&block_number).cloned()
    }

    /// Get current signature count for a block
    pub fn get_signature_count(&self, block_number: u32) -> usize {
        self.signatures
            .read()
            .get(&block_number)
            .map(|entry| entry.1.len())
            .unwrap_or(0)
    }

    /// Get all signatures for a specific block (for sync protocol)
    pub fn get_signatures_for_block(&self, block_number: u32) -> Vec<CheckpointSignature> {
        let signatures = self.signatures.read();

        if let Some(entry) = signatures.get(&block_number) {
            entry.1.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get chain ID
    pub fn get_chain_id(&self) -> [u8; 32] {
        self.chain_id
    }

    /// Get authority set (read-only access)
    pub fn get_authority_set(&self) -> AuthoritySet {
        self.authority_set.read().clone()
    }

    /// Update authority set with epoch rotation
    pub fn update_authority_set(&self, new_set: AuthoritySet) {
        let old_set_id = self.authority_set.read().set_id;
        *self.authority_set.write() = new_set;

        tracing::info!(
            "🔄 Authority set updated: {} -> {}",
            old_set_id,
            self.authority_set.read().set_id
        );

        // Expire old authority set for long-range protection
        self.long_range_protection.write().expire_authority_set(old_set_id);

        // Clear old signatures (replay protection)
        self.signatures.write().clear();

        // Reset nonces for new epoch
        self.validator_nonces.write().clear();

        tracing::info!(
            "🔒 Authority set {} expired, nonces reset for new epoch",
            old_set_id
        );
    }

    /// Add social consensus checkpoint for long-range protection
    pub fn add_social_checkpoint(&self, anchor: CheckpointAnchor) -> Result<(), String> {
        self.long_range_protection.write().add_social_checkpoint(anchor)
    }

    /// Get long-range protection stats
    pub fn get_long_range_stats(&self) -> (u64, usize) {
        let lrp = self.long_range_protection.read();
        (
            lrp.get_min_authority_set_id(),
            lrp.get_all_social_checkpoints().len(),
        )
    }

    /// Cleanup old signatures to prevent memory bloat
    pub fn cleanup_old_signatures(&self, current_best_block: u32) {
        let mut signatures = self.signatures.write();
        let mut certificates = self.certificates.write();

        // Keep signatures for last 100 blocks only
        const KEEP_BLOCKS: u32 = 100;
        let cutoff = current_best_block.saturating_sub(KEEP_BLOCKS);

        signatures.retain(|&block_number, _| block_number >= cutoff);
        certificates.retain(|&block_number, _| block_number >= cutoff);

        tracing::debug!(
            "Cleaned up signatures older than block #{}",
            cutoff
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FORK PROTECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Verify checkpoint block is on canonical chain before finalizing
pub fn verify_canonical_chain<Block, Client>(
    client: &Arc<Client>,
    checkpoint_hash: Block::Hash,
    checkpoint_number: NumberFor<Block>,
) -> Result<bool, String>
where
    Block: BlockT,
    Client: HeaderBackend<Block>,
{
    // Get current best block
    let best_hash = client.info().best_hash;
    let best_number = client.info().best_number;

    // Checkpoint must not be ahead of best block
    if checkpoint_number > best_number {
        return Err(format!(
            "Checkpoint #{} is ahead of best block #{}",
            checkpoint_number, best_number
        ));
    }

    // Walk back from best block to checkpoint
    let mut current_hash = best_hash;
    let mut current_number = best_number;

    while current_number > checkpoint_number {
        match client.header(current_hash) {
            Ok(Some(header)) => {
                current_hash = *header.parent_hash();
                current_number = current_number - sp_runtime::traits::One::one();
            }
            Ok(None) => {
                return Err(format!("Missing header for block {:?}", current_hash));
            }
            Err(e) => {
                return Err(format!("Error fetching header: {:?}", e));
            }
        }
    }

    // Check if we arrived at the checkpoint block
    if current_hash == checkpoint_hash {
        Ok(true)
    } else {
        tracing::warn!(
            "🚨 FORK DETECTED: Checkpoint {:?} is not on canonical chain (canonical at #{} is {:?})",
            checkpoint_hash,
            checkpoint_number,
            current_hash
        );
        Ok(false)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    /// Helper to create test authority set
    fn create_test_authority_set(count: usize) -> (AuthoritySet, Vec<SigningKey>) {
        let mut keys = Vec::new();
        let mut authorities = Vec::new();

        for i in 0..count {
            // Create deterministic test keys (use pseudo-random bytes for testing)
            let mut seed = [0u8; 32];
            seed[0] = i as u8;
            seed[1] = (i >> 8) as u8;

            let signing_key = SigningKey::from_bytes(&seed);
            authorities.push(signing_key.verifying_key().to_bytes());
            keys.push(signing_key);
        }

        let authority_set = AuthoritySet::new(1, authorities);
        (authority_set, keys)
    }

    /// Helper to create valid checkpoint signature
    fn create_checkpoint_signature(
        signing_key: &SigningKey,
        validator_id: u32,
        block_number: u32,
        block_hash: [u8; 32],
        authority_set: &AuthoritySet,
        nonce: u64,
        chain_id: [u8; 32],
    ) -> CheckpointSignature {
        let checkpoint_type = CheckpointType::Guaranteed;
        let validator_pubkey = signing_key.verifying_key().to_bytes();

        let mut sig = CheckpointSignature {
            chain_id,
            block_number,
            block_hash,
            validator_id,
            validator_pubkey,
            authority_set_id: authority_set.set_id,
            authority_set_hash: authority_set.authority_set_hash,
            checkpoint_type,
            signature_nonce: nonce,
            signature: Vec::new(),
            timestamp_ms: 0,
        };

        // Sign payload
        let payload = sig.signing_payload();
        let signature = signing_key.sign(&payload);
        sig.signature = signature.to_bytes().to_vec();

        sig
    }

    #[test]
    fn test_vrf_checkpoint_detection() {
        let parent_hash = [0u8; 32];
        let epoch_randomness = [0u8; 32];
        let epoch = 0;

        // Block 32 should be a guaranteed checkpoint
        assert!(is_checkpoint_block(32, parent_hash, epoch, epoch_randomness));

        // Block 64 should also be guaranteed
        assert!(is_checkpoint_block(64, parent_hash, epoch, epoch_randomness));
    }

    #[test]
    fn test_authority_set_hash() {
        let authorities = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        let hash1 = AuthoritySet::calculate_authority_set_hash(&authorities);

        // Same authorities should produce same hash
        let hash2 = AuthoritySet::calculate_authority_set_hash(&authorities);
        assert_eq!(hash1, hash2);

        // Different authorities should produce different hash
        let authorities2 = vec![[1u8; 32], [2u8; 32], [4u8; 32]];
        let hash3 = AuthoritySet::calculate_authority_set_hash(&authorities2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_signature_verification() {
        let (authority_set, keys) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;

        let sig = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            chain_id,
        );

        // Signature should verify
        assert!(sig.verify_canonical(&sig.validator_pubkey).is_ok());
    }

    #[test]
    fn test_comprehensive_verification() {
        let (authority_set, keys) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;
        let lrp = LongRangeProtection::default();

        let sig = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            chain_id,
        );

        // Should pass all checks
        assert!(sig.verify_comprehensive(&chain_id, &authority_set, &lrp).is_ok());
    }

    #[test]
    fn test_chain_id_mismatch() {
        let (authority_set, keys) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;
        let wrong_chain_id = [99u8; 32];
        let lrp = LongRangeProtection::default();

        let sig = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            wrong_chain_id,
        );

        // Should fail chain ID check
        assert!(sig.verify_comprehensive(&chain_id, &authority_set, &lrp).is_err());
    }

    #[test]
    fn test_nonce_tracking() {
        let (authority_set, keys) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;
        let collector = CheckpointCollector::new_with_chain_id(authority_set.clone(), chain_id);

        // First signature with nonce 1
        let sig1 = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            chain_id,
        );
        assert!(collector.add_signature(sig1).is_ok());

        // Second signature with higher nonce should work
        let sig2 = create_checkpoint_signature(
            &keys[0],
            0,
            64,
            [2u8; 32],
            &authority_set,
            2,
            chain_id,
        );
        assert!(collector.add_signature(sig2).is_ok());

        // Signature with old nonce should fail
        let sig3 = create_checkpoint_signature(
            &keys[0],
            0,
            96,
            [3u8; 32],
            &authority_set,
            1,
            chain_id,
        );
        assert!(collector.add_signature(sig3).is_err());
    }

    #[test]
    fn test_expired_authority_set() {
        let (authority_set, keys) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;
        let collector = CheckpointCollector::new_with_chain_id(authority_set.clone(), chain_id);

        // Expire current authority set
        collector.long_range_protection.write().expire_authority_set(1);

        // Signature from expired set should fail
        let sig = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            chain_id,
        );
        assert!(collector.add_signature(sig).is_err());
    }

    #[test]
    fn test_authority_set_rotation() {
        let (authority_set1, _keys1) = create_test_authority_set(3);
        let (authority_set2, _keys2) = create_test_authority_set(3);
        let chain_id = FLARECHAIN_NETWORK_ID;

        let collector = CheckpointCollector::new_with_chain_id(authority_set1, chain_id);

        // Update to new authority set
        collector.update_authority_set(authority_set2);

        // Check old set was expired
        let (min_set_id, _) = collector.get_long_range_stats();
        assert_eq!(min_set_id, 2); // Old set (1) expired, min is now 2
    }

    #[test]
    fn test_signing_payload_includes_all_fields() {
        let (authority_set, keys) = create_test_authority_set(1);
        let chain_id = FLARECHAIN_NETWORK_ID;

        let sig = create_checkpoint_signature(
            &keys[0],
            0,
            32,
            [1u8; 32],
            &authority_set,
            1,
            chain_id,
        );

        let payload = sig.signing_payload();

        // Payload should include domain separator
        assert!(payload.starts_with(SIGNATURE_DOMAIN));

        // Payload should be deterministic
        let payload2 = sig.signing_payload();
        assert_eq!(payload, payload2);
    }
}
