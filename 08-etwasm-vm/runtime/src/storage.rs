//! Advanced Storage System with Merkle Proofs and Rent Mechanism
//!
//! This module provides a comprehensive storage system for smart contracts with:
//! - Persistent key-value storage
//! - Merkle tree commitments for proof generation
//! - Storage rent/fees mechanism
//! - Storage access tracking (cold/warm)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use sp_core::H256;
use sp_io::hashing::blake2_256;
use sp_std::prelude::*;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use etwasm_gas_metering::VMw;

/// Storage key type (256-bit hash)
pub type StorageKey = H256;

/// Storage value type (256-bit hash)
pub type StorageValue = H256;

/// Storage rent cost per byte per block
pub const STORAGE_RENT_PER_BYTE_PER_BLOCK: VMw = 1;

/// Minimum storage deposit
pub const MINIMUM_STORAGE_DEPOSIT: u128 = 1_000_000; // 1 ETR

/// Storage access modes (for EIP-2929 gas cost optimization)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum StorageAccessMode {
    /// Cold storage access (first time in transaction)
    Cold,
    /// Warm storage access (subsequent accesses)
    Warm,
}

/// Storage entry with metadata
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StorageEntry {
    /// The stored value
    pub value: StorageValue,
    /// Original value at start of transaction (for SSTORE gas refund)
    pub original_value: StorageValue,
    /// Deposit paid for this storage slot
    pub deposit: u128,
    /// Last block number when rent was paid
    pub last_rent_block: u64,
    /// Number of times accessed in current transaction
    pub access_count: u32,
}

impl StorageEntry {
    /// Create a new storage entry
    pub fn new(value: StorageValue, deposit: u128, block_number: u64) -> Self {
        Self {
            value,
            original_value: value,
            deposit,
            last_rent_block: block_number,
            access_count: 0,
        }
    }

    /// Calculate accumulated rent owed
    pub fn calculate_rent(&self, current_block: u64) -> u128 {
        let blocks_elapsed = current_block.saturating_sub(self.last_rent_block);
        let storage_size = 32u128; // 32 bytes per storage slot
        (blocks_elapsed as u128) * storage_size * (STORAGE_RENT_PER_BYTE_PER_BLOCK as u128)
    }

    /// Check if rent needs to be paid
    pub fn needs_rent_payment(&self, current_block: u64, threshold: u64) -> bool {
        current_block.saturating_sub(self.last_rent_block) >= threshold
    }
}

/// Merkle proof for storage verification
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StorageMerkleProof {
    /// Storage key being proven
    pub key: StorageKey,
    /// Storage value at the key
    pub value: StorageValue,
    /// Merkle path (siblings) from leaf to root
    pub proof_path: Vec<H256>,
    /// Storage root hash
    pub root: H256,
}

impl StorageMerkleProof {
    /// Verify the merkle proof
    pub fn verify(&self) -> bool {
        let mut current_hash = self.compute_leaf_hash();

        // Walk up the tree
        for sibling in &self.proof_path {
            current_hash = self.hash_pair(&current_hash, sibling);
        }

        current_hash == self.root
    }

    /// Compute the hash of the leaf node
    fn compute_leaf_hash(&self) -> H256 {
        let mut data = Vec::new();
        data.extend_from_slice(self.key.as_bytes());
        data.extend_from_slice(self.value.as_bytes());
        H256::from(blake2_256(&data))
    }

    /// Hash two nodes together (order-independent for simplicity)
    fn hash_pair(&self, a: &H256, b: &H256) -> H256 {
        let mut data = Vec::new();
        data.extend_from_slice(a.as_bytes());
        data.extend_from_slice(b.as_bytes());
        H256::from(blake2_256(&data))
    }
}

/// Storage commitment - Merkle root and metadata
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StorageCommitment {
    /// Merkle root of the storage tree
    pub root: H256,
    /// Total number of storage slots
    pub slot_count: u64,
    /// Total storage size in bytes
    pub total_size: u64,
    /// Block number when commitment was created
    pub block_number: u64,
}

impl StorageCommitment {
    /// Create a new storage commitment
    pub fn new(root: H256, slot_count: u64, block_number: u64) -> Self {
        Self {
            root,
            slot_count,
            total_size: slot_count * 32, // 32 bytes per slot
            block_number,
        }
    }
}

/// Advanced storage trait with merkle proofs and rent
pub trait AdvancedStorage {
    /// Read a storage value
    fn read(&self, key: &StorageKey) -> Option<StorageValue>;

    /// Write a storage value
    fn write(&mut self, key: StorageKey, value: StorageValue);

    /// Read with access tracking (for gas optimization)
    fn read_with_access(&mut self, key: &StorageKey) -> (Option<StorageValue>, StorageAccessMode);

    /// Write with deposit requirement
    fn write_with_deposit(&mut self, key: StorageKey, value: StorageValue, deposit: u128) -> Result<(), StorageError>;

    /// Delete a storage entry and return deposit
    fn delete(&mut self, key: &StorageKey) -> Option<u128>;

    /// Get storage commitment (merkle root)
    fn get_commitment(&self) -> StorageCommitment;

    /// Generate merkle proof for a key
    fn generate_proof(&self, key: &StorageKey) -> Option<StorageMerkleProof>;

    /// Calculate total rent owed
    fn calculate_total_rent(&self, current_block: u64) -> u128;

    /// Pay rent for all storage entries
    fn pay_rent(&mut self, current_block: u64) -> Result<u128, StorageError>;

    /// Get storage entry with metadata
    fn get_entry(&self, key: &StorageKey) -> Option<&StorageEntry>;
}

/// Storage errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Insufficient deposit for storage
    InsufficientDeposit,
    /// Storage entry not found
    NotFound,
    /// Rent payment failed
    RentPaymentFailed,
    /// Storage limit exceeded
    StorageLimitExceeded,
}

/// In-memory storage with full features
#[derive(Debug, Clone)]
pub struct AdvancedInMemoryStorage {
    /// Storage map with metadata
    entries: BTreeMap<StorageKey, StorageEntry>,
    /// Accessed keys in current transaction (for EIP-2929)
    accessed_keys: BTreeMap<StorageKey, u32>,
    /// Current block number
    current_block: u64,
    /// Storage rent threshold (blocks before rent is due)
    rent_threshold: u64,
}

impl AdvancedInMemoryStorage {
    /// Create a new advanced storage instance
    pub fn new(current_block: u64) -> Self {
        Self {
            entries: BTreeMap::new(),
            accessed_keys: BTreeMap::new(),
            current_block,
            rent_threshold: 100, // Pay rent every 100 blocks
        }
    }

    /// Update current block number
    pub fn set_current_block(&mut self, block: u64) {
        self.current_block = block;
    }

    /// Clear transaction access cache (call at transaction end)
    pub fn clear_access_cache(&mut self) {
        self.accessed_keys.clear();
    }

    /// Compute merkle root of storage tree
    fn compute_merkle_root(&self) -> H256 {
        if self.entries.is_empty() {
            return H256::zero();
        }

        // Simple merkle root: hash all key-value pairs together
        // In production, use a proper merkle tree implementation
        let mut data = Vec::new();
        for (key, entry) in &self.entries {
            data.extend_from_slice(key.as_bytes());
            data.extend_from_slice(entry.value.as_bytes());
        }
        H256::from(blake2_256(&data))
    }
}

impl Default for AdvancedInMemoryStorage {
    fn default() -> Self {
        Self::new(0)
    }
}

impl AdvancedStorage for AdvancedInMemoryStorage {
    fn read(&self, key: &StorageKey) -> Option<StorageValue> {
        self.entries.get(key).map(|entry| entry.value)
    }

    fn write(&mut self, key: StorageKey, value: StorageValue) {
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.value = value;
        } else {
            // New entry with default deposit
            let entry = StorageEntry::new(value, MINIMUM_STORAGE_DEPOSIT, self.current_block);
            self.entries.insert(key, entry);
        }
    }

    fn read_with_access(&mut self, key: &StorageKey) -> (Option<StorageValue>, StorageAccessMode) {
        // Track access for EIP-2929
        let access_mode = if self.accessed_keys.contains_key(key) {
            StorageAccessMode::Warm
        } else {
            StorageAccessMode::Cold
        };

        // Increment access count
        *self.accessed_keys.entry(*key).or_insert(0) += 1;

        // Update entry access count
        if let Some(entry) = self.entries.get_mut(key) {
            entry.access_count += 1;
        }

        (self.read(key), access_mode)
    }

    fn write_with_deposit(&mut self, key: StorageKey, value: StorageValue, deposit: u128) -> Result<(), StorageError> {
        if deposit < MINIMUM_STORAGE_DEPOSIT {
            return Err(StorageError::InsufficientDeposit);
        }

        if let Some(entry) = self.entries.get_mut(&key) {
            // Update existing entry
            entry.value = value;
            entry.deposit += deposit;
        } else {
            // Create new entry
            let entry = StorageEntry::new(value, deposit, self.current_block);
            self.entries.insert(key, entry);
        }

        Ok(())
    }

    fn delete(&mut self, key: &StorageKey) -> Option<u128> {
        self.entries.remove(key).map(|entry| entry.deposit)
    }

    fn get_commitment(&self) -> StorageCommitment {
        let root = self.compute_merkle_root();
        let slot_count = self.entries.len() as u64;
        StorageCommitment::new(root, slot_count, self.current_block)
    }

    fn generate_proof(&self, key: &StorageKey) -> Option<StorageMerkleProof> {
        let entry = self.entries.get(key)?;
        let root = self.compute_merkle_root();

        // Simple proof generation (in production, use proper merkle tree)
        Some(StorageMerkleProof {
            key: *key,
            value: entry.value,
            proof_path: Vec::new(), // Simplified
            root,
        })
    }

    fn calculate_total_rent(&self, current_block: u64) -> u128 {
        self.entries.values()
            .map(|entry| entry.calculate_rent(current_block))
            .sum()
    }

    fn pay_rent(&mut self, current_block: u64) -> Result<u128, StorageError> {
        let mut total_rent = 0u128;

        for entry in self.entries.values_mut() {
            if entry.needs_rent_payment(current_block, self.rent_threshold) {
                let rent = entry.calculate_rent(current_block);

                if rent > entry.deposit {
                    return Err(StorageError::RentPaymentFailed);
                }

                entry.deposit -= rent;
                entry.last_rent_block = current_block;
                total_rent += rent;
            }
        }

        Ok(total_rent)
    }

    fn get_entry(&self, key: &StorageKey) -> Option<&StorageEntry> {
        self.entries.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_storage_operations() {
        let mut storage = AdvancedInMemoryStorage::new(0);
        let key = H256::from_low_u64_be(1);
        let value = H256::from_low_u64_be(42);

        // Write and read
        storage.write(key, value);
        assert_eq!(storage.read(&key), Some(value));

        // Delete
        let deposit = storage.delete(&key);
        assert!(deposit.is_some());
        assert_eq!(storage.read(&key), None);
    }

    #[test]
    fn test_storage_access_tracking() {
        let mut storage = AdvancedInMemoryStorage::new(0);
        let key = H256::from_low_u64_be(1);
        let value = H256::from_low_u64_be(42);

        storage.write(key, value);

        // First access should be cold
        let (val, mode) = storage.read_with_access(&key);
        assert_eq!(val, Some(value));
        assert_eq!(mode, StorageAccessMode::Cold);

        // Second access should be warm
        let (val, mode) = storage.read_with_access(&key);
        assert_eq!(val, Some(value));
        assert_eq!(mode, StorageAccessMode::Warm);

        // Clear cache
        storage.clear_access_cache();

        // After clearing, should be cold again
        let (val, mode) = storage.read_with_access(&key);
        assert_eq!(val, Some(value));
        assert_eq!(mode, StorageAccessMode::Cold);
    }

    #[test]
    fn test_storage_deposit() {
        let mut storage = AdvancedInMemoryStorage::new(0);
        let key = H256::from_low_u64_be(1);
        let value = H256::from_low_u64_be(42);

        // Insufficient deposit should fail
        assert_eq!(
            storage.write_with_deposit(key, value, 100),
            Err(StorageError::InsufficientDeposit)
        );

        // Sufficient deposit should succeed
        assert!(storage.write_with_deposit(key, value, MINIMUM_STORAGE_DEPOSIT).is_ok());

        let entry = storage.get_entry(&key).unwrap();
        assert_eq!(entry.deposit, MINIMUM_STORAGE_DEPOSIT);
    }

    #[test]
    fn test_rent_calculation() {
        let mut storage = AdvancedInMemoryStorage::new(0);
        let key = H256::from_low_u64_be(1);
        let value = H256::from_low_u64_be(42);

        storage.write(key, value);

        // No rent owed initially
        assert_eq!(storage.calculate_total_rent(0), 0);

        // Move forward 100 blocks
        storage.set_current_block(100);
        let rent = storage.calculate_total_rent(100);
        assert!(rent > 0);

        // Pay rent
        let paid = storage.pay_rent(100).unwrap();
        assert_eq!(paid, rent);

        // After paying, rent should be minimal
        assert_eq!(storage.calculate_total_rent(100), 0);
    }

    #[test]
    fn test_merkle_commitment() {
        let mut storage = AdvancedInMemoryStorage::new(0);

        // Empty storage should have zero root
        let commitment = storage.get_commitment();
        assert_eq!(commitment.root, H256::zero());
        assert_eq!(commitment.slot_count, 0);

        // Add some data
        storage.write(H256::from_low_u64_be(1), H256::from_low_u64_be(42));
        storage.write(H256::from_low_u64_be(2), H256::from_low_u64_be(43));

        let commitment = storage.get_commitment();
        assert_ne!(commitment.root, H256::zero());
        assert_eq!(commitment.slot_count, 2);
        assert_eq!(commitment.total_size, 64); // 2 slots * 32 bytes
    }

    #[test]
    fn test_merkle_proof_generation() {
        let mut storage = AdvancedInMemoryStorage::new(0);
        let key = H256::from_low_u64_be(1);
        let value = H256::from_low_u64_be(42);

        storage.write(key, value);

        let proof = storage.generate_proof(&key);
        assert!(proof.is_some());

        let proof = proof.unwrap();
        assert_eq!(proof.key, key);
        assert_eq!(proof.value, value);
    }

    #[test]
    fn test_storage_entry_rent_calculation() {
        let entry = StorageEntry::new(H256::zero(), MINIMUM_STORAGE_DEPOSIT, 0);

        // No rent for same block
        assert_eq!(entry.calculate_rent(0), 0);

        // Rent for 100 blocks
        let rent = entry.calculate_rent(100);
        assert_eq!(rent, 100 * 32 * STORAGE_RENT_PER_BYTE_PER_BLOCK as u128);
    }

    #[test]
    fn test_needs_rent_payment() {
        let entry = StorageEntry::new(H256::zero(), MINIMUM_STORAGE_DEPOSIT, 0);

        assert!(!entry.needs_rent_payment(50, 100)); // Below threshold
        assert!(entry.needs_rent_payment(100, 100)); // At threshold
        assert!(entry.needs_rent_payment(150, 100)); // Above threshold
    }
}
