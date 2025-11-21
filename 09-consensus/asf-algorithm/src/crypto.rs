//! # Cryptographic Utilities for ASF Consensus
//!
//! This module provides cryptographic signature and verification utilities
//! for securing votes, certificates, and PPFA seals in production.

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{crypto::UncheckedFrom, ed25519, sr25519, Pair as PairTrait};
use sp_runtime::traits::Verify;

use crate::{AsfError, AsfResult, Hash, ValidatorId};

// ═══════════════════════════════════════════════════════════════════════════════
// SIGNATURE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Signature scheme used for consensus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureScheme {
    /// Sr25519 (Schnorr over Ristretto25519)
    Sr25519,
    /// Ed25519 (EdDSA over Edwards25519)
    Ed25519,
}

/// A cryptographic signature
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Signature {
    /// Sr25519 signature
    Sr25519(sr25519::Signature),
    /// Ed25519 signature
    Ed25519(ed25519::Signature),
}

impl Default for Signature {
    /// ⚠️ SECURITY WARNING: Default signature is NOT cryptographically valid
    ///
    /// This implementation exists ONLY for:
    /// - Deserialization of incomplete data structures
    /// - Testing purposes with `#[cfg(test)]`
    ///
    /// ❌ NEVER use Default::default() in production code
    /// ✅ ALWAYS create signatures using proper Sr25519 signing:
    ///
    /// ```ignore
    /// use sp_core::{sr25519, Pair};
    /// let (pair, _) = sr25519::Pair::generate();
    /// let signature = Signature::Sr25519(pair.sign(message));
    /// ```
    ///
    /// Any code using Default::default() for signatures in consensus
    /// logic represents a CRITICAL SECURITY VULNERABILITY.
    fn default() -> Self {
        Signature::Sr25519(sr25519::Signature::from_raw([0u8; 64]))
    }
}

impl Signature {
    /// Get signature as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Signature::Sr25519(sig) => sig.as_ref(),
            Signature::Ed25519(sig) => sig.as_ref(),
        }
    }

    /// Create from bytes (Sr25519)
    pub fn from_sr25519_bytes(bytes: [u8; 64]) -> Self {
        Signature::Sr25519(sr25519::Signature::from(bytes))
    }

    /// Create from bytes (Ed25519)
    pub fn from_ed25519_bytes(bytes: [u8; 64]) -> Self {
        Signature::Ed25519(ed25519::Signature::from(bytes))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SIGNING UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Sign data using a keypair
pub trait SignData {
    /// Sign the given message
    fn sign(&self, message: &[u8]) -> Signature;
}

// Test-only implementations for Pair types (which have Pair::generate())
#[cfg(test)]
impl SignData for sr25519::Pair {
    fn sign(&self, message: &[u8]) -> Signature {
        Signature::Sr25519(PairTrait::sign(self, message))
    }
}

#[cfg(test)]
impl SignData for ed25519::Pair {
    fn sign(&self, message: &[u8]) -> Signature {
        Signature::Ed25519(PairTrait::sign(self, message))
    }
}

/// Verify a signature against a public key
pub fn verify_signature(
    signature: &Signature,
    message: &[u8],
    public_key: &ValidatorId,
) -> AsfResult<()> {
    let verified = match signature {
        Signature::Sr25519(sig) => {
            // Convert AccountId32 to Sr25519 Public
            let bytes: [u8; 32] = public_key.clone().into();
            let public = sr25519::Public::unchecked_from(bytes);
            sig.verify(message, &public)
        }
        Signature::Ed25519(sig) => {
            // Convert AccountId32 to Ed25519 Public
            let bytes: [u8; 32] = public_key.clone().into();
            let public = ed25519::Public::unchecked_from(bytes);
            sig.verify(message, &public)
        }
    };

    if verified {
        Ok(())
    } else {
        Err(AsfError::InvalidSignature)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTE SIGNING
// ═══════════════════════════════════════════════════════════════════════════════

/// Data to be signed for a vote
#[derive(Debug, Clone, Encode, Decode)]
pub struct VoteSigningData {
    /// Block hash being voted on
    pub block_hash: Hash,
    /// Block number
    pub block_number: u64,
    /// Consensus phase
    pub phase: u8,
    /// Epoch
    pub epoch: u32,
    /// Timestamp
    pub timestamp: u64,
}

impl VoteSigningData {
    /// Get the message to sign
    pub fn message(&self) -> Vec<u8> {
        self.encode()
    }
}

/// Sign vote data
pub fn sign_vote<P: SignData>(
    pair: &P,
    block_hash: Hash,
    block_number: u64,
    phase: u8,
    epoch: u32,
    timestamp: u64,
) -> Signature {
    let data = VoteSigningData {
        block_hash,
        block_number,
        phase,
        epoch,
        timestamp,
    };
    pair.sign(&data.message())
}

/// Verify vote signature
pub fn verify_vote_signature(
    signature: &Signature,
    block_hash: Hash,
    block_number: u64,
    phase: u8,
    epoch: u32,
    timestamp: u64,
    validator: &ValidatorId,
) -> AsfResult<()> {
    let data = VoteSigningData {
        block_hash,
        block_number,
        phase,
        epoch,
        timestamp,
    };
    verify_signature(signature, &data.message(), validator)
}

// ═══════════════════════════════════════════════════════════════════════════════
// PPFA SEAL SIGNING
// ═══════════════════════════════════════════════════════════════════════════════

/// Data to be signed for a PPFA seal
#[derive(Debug, Clone, Encode, Decode)]
pub struct SealSigningData {
    /// Slot number
    pub slot: u64,
    /// PPFA index
    pub ppfa_index: u32,
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: Hash,
    /// Epoch
    pub epoch: u32,
}

impl SealSigningData {
    /// Get the message to sign
    pub fn message(&self) -> Vec<u8> {
        self.encode()
    }
}

/// Sign PPFA seal data
pub fn sign_seal<P: SignData>(
    pair: &P,
    slot: u64,
    ppfa_index: u32,
    block_number: u64,
    block_hash: Hash,
    epoch: u32,
) -> Signature {
    let data = SealSigningData {
        slot,
        ppfa_index,
        block_number,
        block_hash,
        epoch,
    };
    pair.sign(&data.message())
}

/// Verify PPFA seal signature
pub fn verify_seal_signature(
    signature: &Signature,
    slot: u64,
    ppfa_index: u32,
    block_number: u64,
    block_hash: Hash,
    epoch: u32,
    validator: &ValidatorId,
) -> AsfResult<()> {
    let data = SealSigningData {
        slot,
        ppfa_index,
        block_number,
        block_hash,
        epoch,
    };
    verify_signature(signature, &data.message(), validator)
}

// ═══════════════════════════════════════════════════════════════════════════════
// AGGREGATE SIGNATURE (BLS-like for efficiency)
// ═══════════════════════════════════════════════════════════════════════════════

/// Aggregate signature for efficient certificate verification
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct AggregateSignature {
    /// Individual signatures that have been aggregated
    pub signatures: Vec<Signature>,
    /// Validators who signed
    pub signers: Vec<ValidatorId>,
}

impl AggregateSignature {
    /// Create a new aggregate signature
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
            signers: Vec::new(),
        }
    }

    /// Add a signature to the aggregate
    pub fn add_signature(&mut self, signature: Signature, signer: ValidatorId) {
        self.signatures.push(signature);
        self.signers.push(signer);
    }

    /// Verify all signatures in the aggregate
    pub fn verify_all(&self, message: &[u8]) -> AsfResult<()> {
        if self.signatures.len() != self.signers.len() {
            return Err(AsfError::InvalidCertificate("Signature count mismatch"));
        }

        for (signature, signer) in self.signatures.iter().zip(self.signers.iter()) {
            verify_signature(signature, message, signer)?;
        }

        Ok(())
    }

    /// Get number of signatures
    pub fn count(&self) -> usize {
        self.signatures.len()
    }
}

impl Default for AggregateSignature {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::Pair as _;

    #[test]
    fn test_sr25519_signing() {
        let (pair, _) = sr25519::Pair::generate();
        let message = b"test message";

        let signature = SignData::sign(&pair, message);
        let public = pair.public();

        let validator_id = ValidatorId::from(public.0);
        assert!(verify_signature(&signature, message, &validator_id).is_ok());
    }

    #[test]
    fn test_ed25519_signing() {
        let (pair, _) = ed25519::Pair::generate();
        let message = b"test message";

        let signature = SignData::sign(&pair, message);
        let public = pair.public();

        let validator_id = ValidatorId::from(public.0);
        assert!(verify_signature(&signature, message, &validator_id).is_ok());
    }

    #[test]
    fn test_vote_signing() {
        let (pair, _) = sr25519::Pair::generate();
        let block_hash = Hash::default();
        let public = pair.public();
        let validator_id = ValidatorId::from(public.0);

        let signature = sign_vote(&pair, block_hash, 1, 0, 1, 1000);

        assert!(verify_vote_signature(
            &signature,
            block_hash,
            1,
            0,
            1,
            1000,
            &validator_id
        ).is_ok());
    }

    #[test]
    fn test_seal_signing() {
        let (pair, _) = sr25519::Pair::generate();
        let block_hash = Hash::default();
        let public = pair.public();
        let validator_id = ValidatorId::from(public.0);

        let signature = sign_seal(&pair, 100, 5, 50, block_hash, 1);

        assert!(verify_seal_signature(
            &signature,
            100,
            5,
            50,
            block_hash,
            1,
            &validator_id
        ).is_ok());
    }

    #[test]
    fn test_aggregate_signature() {
        let (pair1, _) = sr25519::Pair::generate();
        let (pair2, _) = sr25519::Pair::generate();
        let message = b"test message";

        let sig1 = SignData::sign(&pair1, message);
        let sig2 = SignData::sign(&pair2, message);

        let validator1 = ValidatorId::from(pair1.public().0);
        let validator2 = ValidatorId::from(pair2.public().0);

        let mut aggregate = AggregateSignature::new();
        aggregate.add_signature(sig1, validator1);
        aggregate.add_signature(sig2, validator2);

        assert_eq!(aggregate.count(), 2);
        assert!(aggregate.verify_all(message).is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let (pair1, _) = sr25519::Pair::generate();
        let (pair2, _) = sr25519::Pair::generate();
        let message = b"test message";

        let signature = SignData::sign(&pair1, message);
        let wrong_validator = ValidatorId::from(pair2.public().0);

        assert!(verify_signature(&signature, message, &wrong_validator).is_err());
    }
}
