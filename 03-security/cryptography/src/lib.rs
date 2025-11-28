//! Cryptography Primitives
//! 
//! Core cryptographic operations: signing, encryption, hashing, KDF.

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier, SignatureError};
use sha2::{Sha256, Digest};
use x25519_dalek::{StaticSecret, PublicKey};

/// Sign a message with Ed25519
pub fn sign_message(message: &[u8], signing_key: &SigningKey) -> Signature {
    signing_key.sign(message)
}

/// Verify a signature with Ed25519
pub fn verify_signature(message: &[u8], signature: &Signature, verifying_key: &VerifyingKey) -> Result<(), SignatureError> {
    verifying_key.verify(message, signature)
}

/// Generate Ed25519 key pair
pub fn generate_ed25519_keypair() -> (SigningKey, VerifyingKey) {
    use rand::RngCore;
    let mut csprng = rand::thread_rng();
    let mut secret_key_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_key_bytes);
    let signing_key = SigningKey::from_bytes(&secret_key_bytes);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Generate X25519 key pair (for ECIES)
pub fn generate_x25519_keypair() -> (StaticSecret, PublicKey) {
    let secret = StaticSecret::random_from_rng(rand::thread_rng());
    let public = PublicKey::from(&secret);
    (secret, public)
}

/// Hash message with SHA-256
pub fn hash_sha256(message: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(message);
    hasher.finalize().to_vec()
}

/// Key derivation function (HKDF-SHA256)
pub fn derive_key(salt: &[u8], input: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    use hkdf::Hkdf;
    let hk = Hkdf::<sha2::Sha256>::new(Some(salt), input);
    let mut okm = vec![0u8; length];
    hk.expand(info, &mut okm).unwrap();
    okm
}

/// Commitment scheme (hash-based)
pub fn commit(message: &[u8], randomness: &[u8]) -> Vec<u8> {
    let mut data = message.to_vec();
    data.extend_from_slice(randomness);
    hash_sha256(&data)
}

/// Verify commitment
pub fn verify_commitment(commitment: &[u8], message: &[u8], randomness: &[u8]) -> bool {
    let computed = commit(message, randomness);
    computed == commitment
}

/// Cryptographic primitives manager
pub struct CryptoManager;

impl CryptoManager {
    pub fn sign(message: &[u8]) -> (Signature, VerifyingKey) {
        let (sk, vk) = generate_ed25519_keypair();
        let sig = sign_message(message, &sk);
        (sig, vk)
    }

    pub fn hash_data(data: &[u8]) -> Vec<u8> {
        hash_sha256(data)
    }

    pub fn kdf(salt: &[u8], input: &[u8], length: usize) -> Vec<u8> {
        derive_key(salt, input, b"ETRID", length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() {
        let (sk, vk) = generate_ed25519_keypair();
        let msg = b"test message";
        let sig = sign_message(msg, &sk);
        assert!(verify_signature(msg, &sig, &vk).is_ok());
    }

    #[test]
    fn test_ed25519_invalid_signature() {
        let (_, vk) = generate_ed25519_keypair();
        let msg = b"test message";
        let (sk2, _) = generate_ed25519_keypair();
        let sig = sign_message(msg, &sk2);
        // Should fail because wrong key
        let result = verify_signature(b"different", &sig, &vk);
        assert!(result.is_err());
    }

    #[test]
    fn test_x25519_keypair() {
        let (secret, public) = generate_x25519_keypair();
        let derived = PublicKey::from(&secret);
        assert_eq!(derived.as_bytes(), public.as_bytes());
    }

    #[test]
    fn test_sha256_hash() {
        let msg = b"test";
        let hash1 = hash_sha256(msg);
        let hash2 = hash_sha256(msg);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_kdf() {
        let salt = b"salt";
        let input = b"input";
        let key = derive_key(salt, input, b"ETRID", 32);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_commitment() {
        let msg = b"secret";
        let rand = b"random123";
        let comm = commit(msg, rand);
        
        assert!(verify_commitment(&comm, msg, rand));
        assert!(!verify_commitment(&comm, b"wrong", rand));
    }
}
