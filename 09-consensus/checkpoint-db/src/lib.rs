//! # Checkpoint Database
//!
//! Persistent storage for ËTRID checkpoint finality state to survive node restarts.
//!
//! ## Architecture
//!
//! Uses RocksDB with column families for efficient key-value storage:
//! - **signatures**: checkpoint_number -> Vec<CheckpointSignature>
//! - **certificates**: checkpoint_number -> CheckpointCertificate
//! - **metadata**: "last_finalized" -> CheckpointNumber
//!
//! ## Recovery Strategy
//!
//! On node restart:
//! 1. Load last finalized checkpoint from metadata
//! 2. Load recent signatures (last 100 checkpoints)
//! 3. Load recent certificates (last 100 checkpoints)
//! 4. Resume consensus from recovered state
//!
//! ## Data Pruning
//!
//! Old checkpoint data is pruned to prevent unbounded growth:
//! - Configurable retention (default: 1000 checkpoints)
//! - Automatic pruning after finality advances
//! - Manual pruning via `prune_old_checkpoints()`

pub mod adapter;
pub use adapter::CheckpointStorageAdapter;

use std::path::Path;
use std::sync::Arc;
use rocksdb::{DB, ColumnFamilyDescriptor, Options, WriteOptions, ReadOptions};
use codec::{Encode, Decode};
use etrid_finality_gadget::{CheckpointSignature, CheckpointCertificate, CheckpointNumber};

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum CheckpointDbError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),

    #[error("Codec decode error: {0}")]
    Decode(#[from] codec::Error),

    #[error("Bincode error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Column family not found: {0}")]
    ColumnFamilyNotFound(String),
}

pub type Result<T> = std::result::Result<T, CheckpointDbError>;

// ============================================================================
// COLUMN FAMILY NAMES
// ============================================================================

const CF_SIGNATURES: &str = "signatures";
const CF_CERTIFICATES: &str = "certificates";
const CF_METADATA: &str = "metadata";

const KEY_LAST_FINALIZED: &[u8] = b"last_finalized";

// ============================================================================
// CHECKPOINT DATABASE
// ============================================================================

/// Persistent storage for checkpoint finality state
pub struct CheckpointDB {
    db: Arc<DB>,
}

impl CheckpointDB {
    /// Open or create a checkpoint database at the specified path
    ///
    /// # Arguments
    /// * `path` - Directory path for the database
    ///
    /// # Returns
    /// * `Ok(CheckpointDB)` - Successfully opened database
    /// * `Err(CheckpointDbError)` - Failed to open database
    pub fn new(path: &Path) -> Result<Self> {
        tracing::info!("🗄️  Opening checkpoint database at {:?}", path);

        // Create column family descriptors
        let cf_signatures = ColumnFamilyDescriptor::new(CF_SIGNATURES, Options::default());
        let cf_certificates = ColumnFamilyDescriptor::new(CF_CERTIFICATES, Options::default());
        let cf_metadata = ColumnFamilyDescriptor::new(CF_METADATA, Options::default());

        // Database options
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        // Open database with column families
        let db = DB::open_cf_descriptors(
            &db_opts,
            path,
            vec![cf_signatures, cf_certificates, cf_metadata],
        )?;

        tracing::info!("✅ Checkpoint database opened successfully");

        Ok(Self {
            db: Arc::new(db),
        })
    }

    // ========== SIGNATURE OPERATIONS ==========

    /// Store a checkpoint signature
    ///
    /// Signatures are appended to the existing list for the checkpoint.
    pub fn store_signature(
        &self,
        checkpoint: CheckpointNumber,
        signature: &CheckpointSignature,
    ) -> Result<()> {
        let cf = self.get_cf(CF_SIGNATURES)?;
        let key = checkpoint.0.to_le_bytes();

        // Get existing signatures
        let mut signatures = self.get_signatures_internal(checkpoint)?;

        // Check for duplicate
        if signatures.iter().any(|sig| {
            sig.validator_id == signature.validator_id &&
            sig.checkpoint_number == signature.checkpoint_number
        }) {
            // Signature already stored, skip
            return Ok(());
        }

        // Append new signature
        signatures.push(signature.clone());

        // Encode and store
        let encoded = signatures.encode();
        self.db.put_cf(cf, &key, &encoded)?;

        tracing::debug!(
            "📝 Stored signature: checkpoint={}, validator={}, total={}",
            checkpoint.0,
            signature.validator_id.0,
            signatures.len()
        );

        Ok(())
    }

    /// Get all signatures for a checkpoint
    pub fn get_signatures(&self, checkpoint: CheckpointNumber) -> Result<Vec<CheckpointSignature>> {
        self.get_signatures_internal(checkpoint)
    }

    /// Internal method to get signatures without extra error handling
    fn get_signatures_internal(&self, checkpoint: CheckpointNumber) -> Result<Vec<CheckpointSignature>> {
        let cf = self.get_cf(CF_SIGNATURES)?;
        let key = checkpoint.0.to_le_bytes();

        match self.db.get_cf(cf, &key)? {
            Some(value) => {
                let signatures = Vec::<CheckpointSignature>::decode(&mut &value[..])?;
                Ok(signatures)
            }
            None => Ok(Vec::new()),
        }
    }

    // ========== CERTIFICATE OPERATIONS ==========

    /// Store a checkpoint certificate
    ///
    /// Overwrites any existing certificate for the checkpoint.
    pub fn store_certificate(
        &self,
        checkpoint: CheckpointNumber,
        certificate: &CheckpointCertificate,
    ) -> Result<()> {
        let cf = self.get_cf(CF_CERTIFICATES)?;
        let key = checkpoint.0.to_le_bytes();

        let encoded = certificate.encode();
        self.db.put_cf(cf, &key, &encoded)?;

        tracing::info!(
            "📜 Stored certificate: checkpoint={}, signatures={}",
            checkpoint.0,
            certificate.signatures.len()
        );

        Ok(())
    }

    /// Get certificate for a checkpoint
    pub fn get_certificate(
        &self,
        checkpoint: CheckpointNumber,
    ) -> Result<Option<CheckpointCertificate>> {
        let cf = self.get_cf(CF_CERTIFICATES)?;
        let key = checkpoint.0.to_le_bytes();

        match self.db.get_cf(cf, &key)? {
            Some(value) => {
                let cert = CheckpointCertificate::decode(&mut &value[..])?;
                Ok(Some(cert))
            }
            None => Ok(None),
        }
    }

    // ========== METADATA OPERATIONS ==========

    /// Get the last finalized checkpoint number
    pub fn get_last_finalized(&self) -> Result<u64> {
        let cf = self.get_cf(CF_METADATA)?;

        match self.db.get_cf(cf, KEY_LAST_FINALIZED)? {
            Some(value) => {
                if value.len() != 8 {
                    return Err(CheckpointDbError::InvalidData(
                        format!("Expected 8 bytes for last_finalized, got {}", value.len())
                    ));
                }
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&value);
                Ok(u64::from_le_bytes(bytes))
            }
            None => Ok(0), // No finalized checkpoint yet
        }
    }

    /// Set the last finalized checkpoint number
    pub fn set_last_finalized(&self, checkpoint: u64) -> Result<()> {
        let cf = self.get_cf(CF_METADATA)?;
        let value = checkpoint.to_le_bytes();

        self.db.put_cf(cf, KEY_LAST_FINALIZED, &value)?;

        tracing::info!("🏁 Updated last finalized checkpoint: {}", checkpoint);

        Ok(())
    }

    // ========== PRUNING OPERATIONS ==========

    /// Prune old checkpoint data, keeping the last N checkpoints
    ///
    /// # Arguments
    /// * `keep_last_n` - Number of recent checkpoints to keep (e.g., 1000)
    ///
    /// # Returns
    /// Number of checkpoints pruned
    pub fn prune_old_checkpoints(&self, keep_last_n: u64) -> Result<usize> {
        let last_finalized = self.get_last_finalized()?;

        if last_finalized <= keep_last_n {
            // Nothing to prune yet
            return Ok(0);
        }

        let cutoff = last_finalized - keep_last_n;
        let mut pruned = 0;

        tracing::info!(
            "🧹 Pruning checkpoints older than {} (keeping last {})",
            cutoff,
            keep_last_n
        );

        // Prune signatures
        let cf_sigs = self.get_cf(CF_SIGNATURES)?;
        for checkpoint in 0..cutoff {
            let key = checkpoint.to_le_bytes();
            if self.db.get_cf(cf_sigs, &key)?.is_some() {
                self.db.delete_cf(cf_sigs, &key)?;
                pruned += 1;
            }
        }

        // Prune certificates
        let cf_certs = self.get_cf(CF_CERTIFICATES)?;
        for checkpoint in 0..cutoff {
            let key = checkpoint.to_le_bytes();
            self.db.delete_cf(cf_certs, &key)?;
        }

        tracing::info!("✅ Pruned {} old checkpoints", pruned);

        Ok(pruned)
    }

    /// Get all checkpoint numbers that have certificates
    ///
    /// Useful for recovery and debugging.
    pub fn get_all_certificate_checkpoints(&self) -> Result<Vec<u64>> {
        let cf = self.get_cf(CF_CERTIFICATES)?;
        let mut checkpoints = Vec::new();

        let iter = self.db.iterator_cf(cf, rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, _) = item?;
            if key.len() == 8 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&key);
                checkpoints.push(u64::from_le_bytes(bytes));
            }
        }

        checkpoints.sort();
        Ok(checkpoints)
    }

    // ========== BATCH OPERATIONS ==========

    /// Store multiple signatures atomically
    pub fn store_signatures_batch(
        &self,
        signatures: &[(CheckpointNumber, CheckpointSignature)],
    ) -> Result<()> {
        let cf = self.get_cf(CF_SIGNATURES)?;
        let mut batch = rocksdb::WriteBatch::default();

        for (checkpoint, signature) in signatures {
            // Get existing signatures
            let mut sigs = self.get_signatures_internal(*checkpoint)?;

            // Check for duplicate
            if !sigs.iter().any(|sig| {
                sig.validator_id == signature.validator_id &&
                sig.checkpoint_number == signature.checkpoint_number
            }) {
                sigs.push(signature.clone());
                let key = checkpoint.0.to_le_bytes();
                let encoded = sigs.encode();
                batch.put_cf(cf, &key, &encoded);
            }
        }

        self.db.write(batch)?;

        tracing::debug!("📦 Batch stored {} signatures", signatures.len());

        Ok(())
    }

    // ========== HELPER METHODS ==========

    /// Get column family handle
    fn get_cf(&self, name: &str) -> Result<&rocksdb::ColumnFamily> {
        self.db
            .cf_handle(name)
            .ok_or_else(|| CheckpointDbError::ColumnFamilyNotFound(name.to_string()))
    }

    /// Flush all data to disk
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        tracing::debug!("💾 Flushed checkpoint database to disk");
        Ok(())
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<CheckpointDbStats> {
        let last_finalized = self.get_last_finalized()?;
        let certificate_checkpoints = self.get_all_certificate_checkpoints()?;
        let total_certificates = certificate_checkpoints.len();

        // Count total signatures across all checkpoints
        let cf_sigs = self.get_cf(CF_SIGNATURES)?;
        let mut total_signatures = 0;
        let iter = self.db.iterator_cf(cf_sigs, rocksdb::IteratorMode::Start);

        for item in iter {
            let (_, value) = item?;
            if let Ok(sigs) = Vec::<CheckpointSignature>::decode(&mut &value[..]) {
                total_signatures += sigs.len();
            }
        }

        Ok(CheckpointDbStats {
            last_finalized,
            total_certificates,
            total_signatures,
            oldest_checkpoint: certificate_checkpoints.first().copied(),
            newest_checkpoint: certificate_checkpoints.last().copied(),
        })
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

#[derive(Debug, Clone)]
pub struct CheckpointDbStats {
    pub last_finalized: u64,
    pub total_certificates: usize,
    pub total_signatures: usize,
    pub oldest_checkpoint: Option<u64>,
    pub newest_checkpoint: Option<u64>,
}

impl std::fmt::Display for CheckpointDbStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CheckpointDB Stats: last_finalized={}, certificates={}, signatures={}, range={:?}-{:?}",
            self.last_finalized,
            self.total_certificates,
            self.total_signatures,
            self.oldest_checkpoint,
            self.newest_checkpoint
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use etrid_finality_gadget::{ValidatorId, BlockHash, AuthoritySetId};

    fn create_test_db() -> (TempDir, CheckpointDB) {
        let temp_dir = TempDir::new().unwrap();
        let db = CheckpointDB::new(temp_dir.path()).unwrap();
        (temp_dir, db)
    }

    fn create_test_signature(checkpoint: u64, validator: u32) -> CheckpointSignature {
        CheckpointSignature {
            validator_id: ValidatorId(validator),
            checkpoint_number: CheckpointNumber(checkpoint),
            block_hash: BlockHash::from_bytes([checkpoint as u8; 32]),
            authority_set_id: AuthoritySetId(0),
            signature: vec![validator as u8; 64],
            timestamp: checkpoint * 1000,
        }
    }

    fn create_test_certificate(checkpoint: u64, num_sigs: usize) -> CheckpointCertificate {
        let mut signatures = Vec::new();
        for i in 0..num_sigs {
            signatures.push((ValidatorId(i as u32), vec![i as u8; 64]));
        }

        CheckpointCertificate {
            checkpoint_number: CheckpointNumber(checkpoint),
            block_hash: BlockHash::from_bytes([checkpoint as u8; 32]),
            authority_set_id: AuthoritySetId(0),
            signatures,
            timestamp: checkpoint * 1000,
        }
    }

    #[test]
    fn test_signature_storage_and_retrieval() {
        let (_dir, db) = create_test_db();

        let sig1 = create_test_signature(1, 0);
        let sig2 = create_test_signature(1, 1);

        // Store signatures
        db.store_signature(CheckpointNumber(1), &sig1).unwrap();
        db.store_signature(CheckpointNumber(1), &sig2).unwrap();

        // Retrieve signatures
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 2);
        assert_eq!(sigs[0].validator_id.0, 0);
        assert_eq!(sigs[1].validator_id.0, 1);
    }

    #[test]
    fn test_duplicate_signature_handling() {
        let (_dir, db) = create_test_db();

        let sig1 = create_test_signature(1, 0);
        let sig1_dup = create_test_signature(1, 0);

        // Store same signature twice
        db.store_signature(CheckpointNumber(1), &sig1).unwrap();
        db.store_signature(CheckpointNumber(1), &sig1_dup).unwrap();

        // Should only have one signature
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 1);
    }

    #[test]
    fn test_certificate_storage_and_retrieval() {
        let (_dir, db) = create_test_db();

        let cert = create_test_certificate(1, 14);

        // Store certificate
        db.store_certificate(CheckpointNumber(1), &cert).unwrap();

        // Retrieve certificate
        let retrieved = db.get_certificate(CheckpointNumber(1)).unwrap();
        assert!(retrieved.is_some());

        let retrieved_cert = retrieved.unwrap();
        assert_eq!(retrieved_cert.checkpoint_number.0, 1);
        assert_eq!(retrieved_cert.signatures.len(), 14);
    }

    #[test]
    fn test_last_finalized_tracking() {
        let (_dir, db) = create_test_db();

        // Initial value should be 0
        assert_eq!(db.get_last_finalized().unwrap(), 0);

        // Set last finalized
        db.set_last_finalized(42).unwrap();
        assert_eq!(db.get_last_finalized().unwrap(), 42);

        // Update last finalized
        db.set_last_finalized(100).unwrap();
        assert_eq!(db.get_last_finalized().unwrap(), 100);
    }

    #[test]
    fn test_pruning() {
        let (_dir, db) = create_test_db();

        // Store checkpoints 1-20
        for i in 1..=20 {
            let sig = create_test_signature(i, 0);
            db.store_signature(CheckpointNumber(i), &sig).unwrap();

            let cert = create_test_certificate(i, 14);
            db.store_certificate(CheckpointNumber(i), &cert).unwrap();
        }

        // Set last finalized to 20
        db.set_last_finalized(20).unwrap();

        // Prune, keeping last 10
        let pruned = db.prune_old_checkpoints(10).unwrap();
        assert_eq!(pruned, 10); // Should prune checkpoints 1-10

        // Verify checkpoints 1-10 are gone
        for i in 1..=10 {
            let sigs = db.get_signatures(CheckpointNumber(i)).unwrap();
            assert_eq!(sigs.len(), 0);

            let cert = db.get_certificate(CheckpointNumber(i)).unwrap();
            assert!(cert.is_none());
        }

        // Verify checkpoints 11-20 still exist
        for i in 11..=20 {
            let sigs = db.get_signatures(CheckpointNumber(i)).unwrap();
            assert_eq!(sigs.len(), 1);

            let cert = db.get_certificate(CheckpointNumber(i)).unwrap();
            assert!(cert.is_some());
        }
    }

    #[test]
    fn test_batch_signature_storage() {
        let (_dir, db) = create_test_db();

        let mut batch = Vec::new();
        for i in 0..5 {
            let sig = create_test_signature(1, i);
            batch.push((CheckpointNumber(1), sig));
        }

        // Store batch
        db.store_signatures_batch(&batch).unwrap();

        // Verify all stored
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 5);
    }

    #[test]
    fn test_get_all_certificate_checkpoints() {
        let (_dir, db) = create_test_db();

        // Store certificates for checkpoints 1, 5, 10
        for checkpoint in [1, 5, 10] {
            let cert = create_test_certificate(checkpoint, 14);
            db.store_certificate(CheckpointNumber(checkpoint), &cert).unwrap();
        }

        let checkpoints = db.get_all_certificate_checkpoints().unwrap();
        assert_eq!(checkpoints, vec![1, 5, 10]);
    }

    #[test]
    fn test_database_stats() {
        let (_dir, db) = create_test_db();

        // Store some data
        for i in 1..=5 {
            let sig = create_test_signature(i, 0);
            db.store_signature(CheckpointNumber(i), &sig).unwrap();

            let cert = create_test_certificate(i, 14);
            db.store_certificate(CheckpointNumber(i), &cert).unwrap();
        }

        db.set_last_finalized(5).unwrap();

        let stats = db.get_stats().unwrap();
        assert_eq!(stats.last_finalized, 5);
        assert_eq!(stats.total_certificates, 5);
        assert_eq!(stats.total_signatures, 5);
        assert_eq!(stats.oldest_checkpoint, Some(1));
        assert_eq!(stats.newest_checkpoint, Some(5));
    }

    #[test]
    fn test_persistence_across_reopens() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();

        // Open database, store data, close
        {
            let db = CheckpointDB::new(&path).unwrap();
            let sig = create_test_signature(1, 0);
            db.store_signature(CheckpointNumber(1), &sig).unwrap();
            db.set_last_finalized(1).unwrap();
            db.flush().unwrap();
        }

        // Reopen database, verify data persisted
        {
            let db = CheckpointDB::new(&path).unwrap();
            let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
            assert_eq!(sigs.len(), 1);
            assert_eq!(db.get_last_finalized().unwrap(), 1);
        }
    }

    #[test]
    fn test_empty_checkpoint_retrieval() {
        let (_dir, db) = create_test_db();

        // Should return empty vec for non-existent checkpoint
        let sigs = db.get_signatures(CheckpointNumber(999)).unwrap();
        assert_eq!(sigs.len(), 0);

        // Should return None for non-existent certificate
        let cert = db.get_certificate(CheckpointNumber(999)).unwrap();
        assert!(cert.is_none());
    }
}
