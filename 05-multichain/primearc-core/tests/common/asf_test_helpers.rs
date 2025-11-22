//! # ASF Test Helpers
//!
//! Reusable test utilities for V26 SessionKeys integration testing.
//! Provides abstractions for validators, networks, and common test operations.

use codec::{Decode, Encode};
use sp_core::{sr25519, Pair, Public};
use sp_keyring::AccountKeyring;
use sp_runtime::{AccountId32, traits::IdentifyAccount};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AccountId = AccountId32;
pub type Hash = sp_core::H256;
pub type Signature = sr25519::Signature;

/// Represents a test validator with ASF keypair and node
#[derive(Clone)]
pub struct TestValidator {
    /// Validator's account ID
    pub account_id: AccountId,
    /// ASF signing keypair (sr25519)
    pub asf_keypair: sr25519::Pair,
    /// Session keypair for session.setKeys()
    pub session_keypair: sr25519::Pair,
    /// Validator stake weight
    pub stake: u128,
    /// Unique identifier for this validator
    pub index: usize,
}

impl TestValidator {
    /// Create a new test validator with generated keys
    pub fn new(index: usize) -> Self {
        let account_keyring = match index {
            0 => AccountKeyring::Alice,
            1 => AccountKeyring::Bob,
            2 => AccountKeyring::Charlie,
            3 => AccountKeyring::Dave,
            4 => AccountKeyring::Eve,
            5 => AccountKeyring::Ferdie,
            _ => {
                // For validators beyond the keyring, generate from seed
                let seed = format!("//Validator{}", index);
                return Self::from_seed(&seed, index);
            }
        };

        let account_id = account_keyring.to_account_id();
        let asf_keypair = sr25519::Pair::from_string(&format!("//{}//asf", account_keyring), None)
            .expect("Failed to generate ASF keypair");
        let session_keypair = account_keyring.pair();

        Self {
            account_id,
            asf_keypair,
            session_keypair,
            stake: 10_000_000,
            index,
        }
    }

    /// Create validator from seed string
    pub fn from_seed(seed: &str, index: usize) -> Self {
        let session_keypair = sr25519::Pair::from_string(seed, None)
            .expect("Failed to generate session keypair");
        let asf_keypair = sr25519::Pair::from_string(&format!("{}//asf", seed), None)
            .expect("Failed to generate ASF keypair");
        let account_id = AccountId32::from(session_keypair.public());

        Self {
            account_id,
            asf_keypair,
            session_keypair,
            stake: 10_000_000,
            index,
        }
    }

    /// Get the ASF public key
    pub fn asf_public_key(&self) -> sr25519::Public {
        self.asf_keypair.public()
    }

    /// Get the ASF public key as encoded bytes
    pub fn asf_public_key_bytes(&self) -> Vec<u8> {
        self.asf_keypair.public().encode()
    }

    /// Sign a checkpoint with ASF key
    pub fn sign_checkpoint(&self, block_hash: Hash) -> Signature {
        let message = block_hash.encode();
        self.asf_keypair.sign(&message)
    }

    /// Verify a checkpoint signature
    pub fn verify_checkpoint(&self, block_hash: Hash, signature: &Signature) -> bool {
        let message = block_hash.encode();
        sp_core::sr25519::Pair::verify(signature, &message, &self.asf_keypair.public())
    }

    /// Create SessionKeys structure for setKeys() extrinsic
    pub fn create_session_keys(&self) -> Vec<u8> {
        // SessionKeys structure: just ASF key in V26
        self.asf_keypair.public().encode()
    }
}

/// Represents a test network with multiple validators
pub struct TestNetwork {
    /// All validators in the network
    pub validators: Vec<TestValidator>,
    /// Current session/epoch number
    pub session: Arc<Mutex<u32>>,
    /// Authority set (validators with registered keys)
    pub authority_set: Arc<Mutex<HashMap<AccountId, Vec<u8>>>>,
    /// Simulated checkpoints
    pub checkpoints: Arc<Mutex<HashMap<u32, CheckpointData>>>,
}

/// Checkpoint data for testing
#[derive(Clone, Debug)]
pub struct CheckpointData {
    pub block_number: u32,
    pub block_hash: Hash,
    pub signatures: Vec<(AccountId, Signature)>,
    pub verified: bool,
}

impl TestNetwork {
    /// Create a new test network with n validators
    pub fn new(n: usize) -> Self {
        let validators = (0..n).map(|i| TestValidator::new(i)).collect();

        Self {
            validators,
            session: Arc::new(Mutex::new(0)),
            authority_set: Arc::new(Mutex::new(HashMap::new())),
            checkpoints: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register ASF keys for a specific validator
    pub async fn register_validator_key(&self, validator_index: usize) -> Result<(), String> {
        if validator_index >= self.validators.len() {
            return Err("Validator index out of bounds".to_string());
        }

        let validator = &self.validators[validator_index];
        let mut authority_set = self.authority_set.lock().await;

        authority_set.insert(
            validator.account_id.clone(),
            validator.asf_public_key_bytes(),
        );

        Ok(())
    }

    /// Register ASF keys for all validators
    pub async fn register_all_validators(&self) -> Result<(), String> {
        for i in 0..self.validators.len() {
            self.register_validator_key(i).await?;
        }
        Ok(())
    }

    /// Verify a validator is in the authority set
    pub async fn verify_in_authority_set(&self, validator_index: usize) -> bool {
        if validator_index >= self.validators.len() {
            return false;
        }

        let validator = &self.validators[validator_index];
        let authority_set = self.authority_set.lock().await;

        authority_set.contains_key(&validator.account_id)
    }

    /// Get ASF key for a validator from authority set
    pub async fn get_asf_key(&self, account_id: &AccountId) -> Option<Vec<u8>> {
        let authority_set = self.authority_set.lock().await;
        authority_set.get(account_id).cloned()
    }

    /// Get all validator ASF keys
    pub async fn get_all_asf_keys(&self) -> Vec<(AccountId, Vec<u8>)> {
        let authority_set = self.authority_set.lock().await;
        authority_set.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Simulate a checkpoint at a block
    pub async fn trigger_checkpoint(&self, block_number: u32) -> Result<CheckpointData, String> {
        let block_hash = Hash::from_low_u64_be(block_number as u64);
        let mut signatures = Vec::new();

        // Collect signatures from all validators in authority set
        let authority_set = self.authority_set.lock().await;
        for (account_id, _) in authority_set.iter() {
            if let Some(validator) = self.validators.iter().find(|v| &v.account_id == account_id) {
                let signature = validator.sign_checkpoint(block_hash);
                signatures.push((account_id.clone(), signature));
            }
        }
        drop(authority_set);

        let checkpoint = CheckpointData {
            block_number,
            block_hash,
            signatures,
            verified: false,
        };

        let mut checkpoints = self.checkpoints.lock().await;
        checkpoints.insert(block_number, checkpoint.clone());

        Ok(checkpoint)
    }

    /// Verify checkpoint signatures
    pub async fn verify_checkpoint(&self, block_number: u32) -> Result<bool, String> {
        let checkpoints = self.checkpoints.lock().await;
        let checkpoint = checkpoints.get(&block_number)
            .ok_or_else(|| "Checkpoint not found".to_string())?;

        let mut verified_count = 0;
        for (account_id, signature) in &checkpoint.signatures {
            if let Some(validator) = self.validators.iter().find(|v| &v.account_id == account_id) {
                if validator.verify_checkpoint(checkpoint.block_hash, signature) {
                    verified_count += 1;
                }
            }
        }

        // BFT threshold: 2/3 + 1
        let total_validators = checkpoint.signatures.len();
        let threshold = (total_validators * 2 / 3) + 1;

        Ok(verified_count >= threshold)
    }

    /// Rotate session (simulates session change)
    pub async fn trigger_session_rotation(&self) {
        let mut session = self.session.lock().await;
        *session += 1;
    }

    /// Get current session number
    pub async fn get_session(&self) -> u32 {
        let session = self.session.lock().await;
        *session
    }

    /// Add a new validator to the network
    pub async fn add_validator(&mut self, index: usize) -> Result<(), String> {
        let validator = TestValidator::new(index);
        self.validators.push(validator);
        Ok(())
    }

    /// Remove a validator from the network
    pub async fn remove_validator(&self, validator_index: usize) -> Result<(), String> {
        if validator_index >= self.validators.len() {
            return Err("Validator index out of bounds".to_string());
        }

        let validator = &self.validators[validator_index];
        let mut authority_set = self.authority_set.lock().await;
        authority_set.remove(&validator.account_id);

        Ok(())
    }

    /// Simulate block production for n blocks
    pub async fn produce_blocks(&self, n: u32) -> Vec<Hash> {
        let mut blocks = Vec::new();
        for i in 1..=n {
            let block_hash = Hash::from_low_u64_be(i as u64);
            blocks.push(block_hash);

            // Trigger checkpoint every 32 blocks
            if i % 32 == 0 {
                let _ = self.trigger_checkpoint(i).await;
            }
        }
        blocks
    }

    /// Check if checkpoint exists at block number
    pub async fn has_checkpoint(&self, block_number: u32) -> bool {
        let checkpoints = self.checkpoints.lock().await;
        checkpoints.contains_key(&block_number)
    }

    /// Get checkpoint signature count
    pub async fn get_checkpoint_signature_count(&self, block_number: u32) -> usize {
        let checkpoints = self.checkpoints.lock().await;
        checkpoints.get(&block_number)
            .map(|c| c.signatures.len())
            .unwrap_or(0)
    }
}

/// Helper function to calculate BFT threshold
pub fn bft_threshold(total_validators: usize) -> usize {
    (total_validators * 2 / 3) + 1
}

/// Helper function to generate test block hash
pub fn test_block_hash(block_number: u32) -> Hash {
    Hash::from_low_u64_be(block_number as u64)
}

/// Helper function to sleep for testing
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = TestValidator::new(0);
        assert_eq!(validator.stake, 10_000_000);
        assert_eq!(validator.index, 0);
        assert!(!validator.asf_public_key_bytes().is_empty());
    }

    #[test]
    fn test_checkpoint_signing() {
        let validator = TestValidator::new(0);
        let block_hash = test_block_hash(100);

        let signature = validator.sign_checkpoint(block_hash);
        assert!(validator.verify_checkpoint(block_hash, &signature));
    }

    #[tokio::test]
    async fn test_network_creation() {
        let network = TestNetwork::new(5);
        assert_eq!(network.validators.len(), 5);
        assert_eq!(network.get_session().await, 0);
    }

    #[tokio::test]
    async fn test_validator_registration() {
        let network = TestNetwork::new(3);

        assert!(!network.verify_in_authority_set(0).await);
        network.register_validator_key(0).await.unwrap();
        assert!(network.verify_in_authority_set(0).await);
    }

    #[tokio::test]
    async fn test_checkpoint_creation() {
        let network = TestNetwork::new(3);
        network.register_all_validators().await.unwrap();

        let checkpoint = network.trigger_checkpoint(32).await.unwrap();
        assert_eq!(checkpoint.block_number, 32);
        assert_eq!(checkpoint.signatures.len(), 3);
    }

    #[tokio::test]
    async fn test_checkpoint_verification() {
        let network = TestNetwork::new(5);
        network.register_all_validators().await.unwrap();

        network.trigger_checkpoint(64).await.unwrap();
        let verified = network.verify_checkpoint(64).await.unwrap();
        assert!(verified);
    }

    #[test]
    fn test_bft_threshold() {
        assert_eq!(bft_threshold(21), 15); // 21 * 2/3 + 1 = 15
        assert_eq!(bft_threshold(10), 7);  // 10 * 2/3 + 1 = 7
        assert_eq!(bft_threshold(3), 3);   // 3 * 2/3 + 1 = 3
    }
}
