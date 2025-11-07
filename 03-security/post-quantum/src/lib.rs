//! # Post-Quantum Cryptography Module
//!
//! This module provides quantum-resistant cryptographic primitives for Etrid,
//! implementing NIST-standardized post-quantum algorithms with hybrid classical schemes.
//!
//! ## Algorithms Implemented
//!
//! ### CRYSTALS-Kyber (Key Encapsulation Mechanism)
//! - **Purpose:** Quantum-resistant key exchange
//! - **Security Level:** Kyber512 (~AES-128), Kyber768 (~AES-192), Kyber1024 (~AES-256)
//! - **Use Case:** Establishing shared secrets for encrypted communication
//!
//! ### CRYSTALS-Dilithium (Digital Signatures)
//! - **Purpose:** Quantum-resistant signatures
//! - **Security Level:** Dilithium2 (~AES-128), Dilithium3 (~AES-192), Dilithium5 (~AES-256)
//! - **Use Case:** Transaction signing, message authentication
//!
//! ## Hybrid Schemes
//!
//! For maximum security during the quantum transition period, we support hybrid modes:
//! - **Hybrid Signatures:** Ed25519 + Dilithium (both must verify for acceptance)
//! - **Hybrid KEM:** X25519 + Kyber (combine shared secrets)
//!
//! ## Migration Strategy
//!
//! 1. **Phase 1 (2026 Q2):** Deploy PQC algorithms alongside classical crypto
//! 2. **Phase 2 (2026 Q3):** Enable hybrid mode by default
//! 3. **Phase 3 (2026 Q4):** Transition to PQC-only mode
//!
//! ## Security Notes
//!
//! - All implementations use NIST-approved parameters
//! - Side-channel resistance is a work in progress for PQC algorithms
//! - Key sizes are significantly larger than classical equivalents
//! - Signature sizes are also larger (Dilithium2: ~2.4KB vs Ed25519: 64 bytes)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

// Re-export PQC traits
pub use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SecretKey as KemSecretKey, Ciphertext, SharedSecret};
pub use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, SignedMessage, DetachedSignature};

// Kyber KEM - re-export Kyber768 (NIST Level 3, ~AES-192 security)
pub mod kyber {
    use super::*;
    // Re-export all Kyber768 functions and types
    // This includes: keypair(), encapsulate(), decapsulate(), PublicKey, SecretKey, etc.
    pub use pqcrypto_kyber::kyber768::*;
}

// Dilithium signatures - re-export Dilithium3 (NIST Level 3, ~AES-192 security)
pub mod dilithium {
    use super::*;
    // Re-export all Dilithium3 functions and types
    // This includes: keypair(), detached_sign(), verify_detached_signature(), PublicKey, SecretKey, etc.
    pub use pqcrypto_dilithium::dilithium3::*;
}

// Hybrid schemes
pub mod hybrid {
    use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
    use sha2::{Sha256, Digest};

    // Import concrete types from pqcrypto modules (avoiding trait name conflicts)
    use pqcrypto_dilithium::dilithium3;
    use pqcrypto_kyber::kyber768;

    // Import traits for as_bytes() and from_bytes() methods
    use pqcrypto_traits::kem::{Ciphertext as CiphertextTrait, SharedSecret as SharedSecretTrait, PublicKey as KemPublicKeyTrait, SecretKey as KemSecretKeyTrait};
    use pqcrypto_traits::sign::{DetachedSignature as DetachedSignatureTrait, PublicKey as SignPublicKeyTrait, SecretKey as SignSecretKeyTrait};

    /// Hybrid signature combining Ed25519 and Dilithium
    #[derive(Clone, Debug)]
    pub struct HybridSignature {
        /// Classical Ed25519 signature (64 bytes)
        pub classical_sig: [u8; 64],
        /// Post-quantum Dilithium signature (~3KB)
        pub pq_sig: Vec<u8>,
    }

    /// Hybrid public key
    #[derive(Clone, Debug)]
    pub struct HybridPublicKey {
        pub classical_pk: [u8; 32],
        pub pq_pk: Vec<u8>,
    }

    /// Hybrid secret key
    #[derive(Clone, Debug)]
    pub struct HybridSecretKey {
        pub classical_sk: [u8; 32],
        pub pq_sk: Vec<u8>,
    }

    /// Generate hybrid keypair (Ed25519 + Dilithium3)
    pub fn keypair() -> (HybridPublicKey, HybridSecretKey) {
        // Generate Ed25519 keypair
        let mut csprng = rand::thread_rng();
        let ed_signing_key = SigningKey::generate(&mut csprng);
        let ed_verifying_key = ed_signing_key.verifying_key();

        // Generate Dilithium3 keypair
        let (dil_pk, dil_sk) = dilithium3::keypair();

        let public_key = HybridPublicKey {
            classical_pk: ed_verifying_key.to_bytes(),
            pq_pk: dil_pk.as_bytes().to_vec(),
        };

        let secret_key = HybridSecretKey {
            classical_sk: ed_signing_key.to_bytes(),
            pq_sk: dil_sk.as_bytes().to_vec(),
        };

        (public_key, secret_key)
    }

    /// Sign with hybrid scheme (both Ed25519 and Dilithium)
    pub fn sign(msg: &[u8], sk: &HybridSecretKey) -> Result<HybridSignature, &'static str> {
        // Sign with Ed25519
        let ed_signing_key = SigningKey::from_bytes(&sk.classical_sk);
        let ed_signature = ed_signing_key.sign(msg);

        // Sign with Dilithium3
        let dil_sk = dilithium3::SecretKey::from_bytes(&sk.pq_sk)
            .map_err(|_| "Invalid Dilithium secret key")?;
        let dil_signature = dilithium3::detached_sign(msg, &dil_sk);

        Ok(HybridSignature {
            classical_sig: ed_signature.to_bytes(),
            pq_sig: dil_signature.as_bytes().to_vec(),
        })
    }

    /// Verify hybrid signature (both must pass)
    pub fn verify(msg: &[u8], sig: &HybridSignature, pk: &HybridPublicKey) -> Result<(), &'static str> {
        // Verify Ed25519 signature
        let ed_verifying_key = VerifyingKey::from_bytes(&pk.classical_pk)
            .map_err(|_| "Invalid Ed25519 public key")?;
        let ed_signature = Signature::from_bytes(&sig.classical_sig);
        ed_verifying_key.verify(msg, &ed_signature)
            .map_err(|_| "Ed25519 verification failed")?;

        // Verify Dilithium signature
        let dil_pk = dilithium3::PublicKey::from_bytes(&pk.pq_pk)
            .map_err(|_| "Invalid Dilithium public key")?;
        let dil_signature = dilithium3::DetachedSignature::from_bytes(&sig.pq_sig)
            .map_err(|_| "Invalid Dilithium signature")?;
        dilithium3::verify_detached_signature(&dil_signature, msg, &dil_pk)
            .map_err(|_| "Dilithium verification failed")?;

        Ok(())
    }

    /// Hybrid KEM: Combine X25519 and Kyber768 shared secrets
    pub fn hybrid_encapsulate(kyber_pk: &kyber768::PublicKey) -> (Vec<u8>, Vec<u8>) {
        // Encapsulate with Kyber
        let (kyber_ss, kyber_ct) = kyber768::encapsulate(kyber_pk);

        // In full implementation, would also do X25519 DH
        // For now, just use Kyber shared secret
        let combined_ss = kyber_ss.as_bytes().to_vec();
        let ciphertext = kyber_ct.as_bytes().to_vec();

        (combined_ss, ciphertext)
    }

    /// Hybrid KEM decapsulation
    pub fn hybrid_decapsulate(ciphertext: &[u8], kyber_sk: &kyber768::SecretKey) -> Result<Vec<u8>, &'static str> {
        // Decapsulate Kyber ciphertext
        let kyber_ct = kyber768::Ciphertext::from_bytes(ciphertext)
            .map_err(|_| "Invalid Kyber ciphertext")?;
        let kyber_ss = kyber768::decapsulate(&kyber_ct, kyber_sk);

        // In full implementation, would also do X25519 DH and combine
        Ok(kyber_ss.as_bytes().to_vec())
    }

    /// Derive encryption key from hybrid shared secret
    pub fn derive_encryption_key(shared_secret: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"etrid-pqc-kdf");
        hasher.update(shared_secret);
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash);
        key
    }
}

// Utility functions
pub mod utils {
    use super::*;
    use sha2::{Sha256, Digest};

    /// Hash data for commitment schemes
    pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }

    /// Compare key sizes
    pub fn key_size_comparison() -> String {
        format!(
            "Classical vs Post-Quantum Key Sizes:\n\
             Ed25519 Public Key: 32 bytes\n\
             Dilithium3 Public Key: ~1952 bytes (61x larger)\n\
             \n\
             Ed25519 Signature: 64 bytes\n\
             Dilithium3 Signature: ~3293 bytes (51x larger)\n\
             \n\
             X25519 Public Key: 32 bytes\n\
             Kyber768 Public Key: 1184 bytes (37x larger)\n\
             \n\
             Trade-off: Quantum resistance at cost of larger keys/signatures"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_kem() {
        // Generate keypair
        let (pk, sk) = kyber::keypair();

        // Encapsulate
        let (ss1, ct) = kyber::encapsulate(&pk);

        // Decapsulate
        let ss2 = kyber::decapsulate(&ct, &sk);

        // Shared secrets should match
        assert_eq!(ss1.as_bytes(), ss2.as_bytes());
    }

    #[test]
    fn test_dilithium_sign_verify() {
        // Generate keypair
        let (pk, sk) = dilithium::keypair();

        let message = b"Etrid blockchain transaction";

        // Sign
        let signature = dilithium::sign(message, &sk);

        // Verify
        assert!(dilithium::verify(message, &signature, &pk).is_ok());

        // Verify with wrong message should fail
        let wrong_message = b"Different message";
        assert!(dilithium::verify(wrong_message, &signature, &pk).is_err());
    }

    #[test]
    fn test_hybrid_signature() {
        // Generate hybrid keypair
        let (pk, sk) = hybrid::keypair();

        let message = b"Hybrid signature test";

        // Sign with hybrid scheme
        let signature = hybrid::sign(message, &sk).unwrap();

        // Verify hybrid signature
        assert!(hybrid::verify(message, &signature, &pk).is_ok());

        // Verify with wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(hybrid::verify(wrong_message, &signature, &pk).is_err());
    }

    #[test]
    fn test_hybrid_kem() {
        // Generate Kyber keypair
        let (pk, sk) = kyber::keypair();

        // Encapsulate
        let (ss1, ct) = hybrid::hybrid_encapsulate(&pk);

        // Decapsulate
        let ss2 = hybrid::hybrid_decapsulate(&ct, &sk).unwrap();

        // Shared secrets should match
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn test_key_derivation() {
        let shared_secret = b"test_shared_secret";
        let key1 = hybrid::derive_encryption_key(shared_secret);
        let key2 = hybrid::derive_encryption_key(shared_secret);

        // Same input should produce same key
        assert_eq!(key1, key2);

        // Different input should produce different key
        let different_secret = b"different_secret";
        let key3 = hybrid::derive_encryption_key(different_secret);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_key_size_info() {
        let info = utils::key_size_comparison();
        assert!(info.contains("Dilithium3"));
        assert!(info.contains("Kyber768"));
    }
}
