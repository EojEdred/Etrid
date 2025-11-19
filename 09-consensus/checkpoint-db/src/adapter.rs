//! # Checkpoint Storage Adapter
//!
//! Adapter that implements the CheckpointStorage trait from finality-gadget
//! using the CheckpointDB backend.
//!
//! This bridges the async trait interface with the sync RocksDB operations.

use crate::{CheckpointDB, Result as DbResult};
use etrid_finality_gadget::{
    CheckpointSignature, CheckpointCertificate, CheckpointNumber,
};
use std::sync::Arc;

/// Adapter that implements async CheckpointStorage trait using sync CheckpointDB
pub struct CheckpointStorageAdapter {
    db: Arc<CheckpointDB>,
}

impl CheckpointStorageAdapter {
    /// Create a new adapter wrapping a CheckpointDB instance
    pub fn new(db: Arc<CheckpointDB>) -> Self {
        Self { db }
    }

    /// Convert database error to string
    fn map_err(err: crate::CheckpointDbError) -> String {
        format!("Database error: {}", err)
    }
}

#[async_trait::async_trait]
impl etrid_finality_gadget::persistence::CheckpointStorage for CheckpointStorageAdapter {
    async fn store_signature(
        &self,
        checkpoint: CheckpointNumber,
        signature: &CheckpointSignature,
    ) -> Result<(), String> {
        self.db
            .store_signature(checkpoint, signature)
            .map_err(Self::map_err)
    }

    async fn store_certificate(
        &self,
        checkpoint: CheckpointNumber,
        certificate: &CheckpointCertificate,
    ) -> Result<(), String> {
        self.db
            .store_certificate(checkpoint, certificate)
            .map_err(Self::map_err)
    }

    async fn get_signatures(
        &self,
        checkpoint: CheckpointNumber,
    ) -> Result<Vec<CheckpointSignature>, String> {
        self.db
            .get_signatures(checkpoint)
            .map_err(Self::map_err)
    }

    async fn get_certificate(
        &self,
        checkpoint: CheckpointNumber,
    ) -> Result<Option<CheckpointCertificate>, String> {
        self.db
            .get_certificate(checkpoint)
            .map_err(Self::map_err)
    }

    async fn get_last_finalized(&self) -> Result<u64, String> {
        self.db
            .get_last_finalized()
            .map_err(Self::map_err)
    }

    async fn set_last_finalized(&self, checkpoint: u64) -> Result<(), String> {
        self.db
            .set_last_finalized(checkpoint)
            .map_err(Self::map_err)
    }

    async fn prune_old_checkpoints(&self, keep_last_n: u64) -> Result<(), String> {
        self.db
            .prune_old_checkpoints(keep_last_n)
            .map_err(|e| format!("Prune failed: {}", e))
    }

    async fn flush(&self) -> Result<(), String> {
        self.db
            .flush()
            .map_err(Self::map_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use etrid_finality_gadget::{ValidatorId, BlockHash, AuthoritySetId};
    use tempfile::TempDir;

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

    #[tokio::test]
    async fn test_adapter_signature_storage() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(CheckpointDB::new(temp_dir.path()).unwrap());
        let adapter = CheckpointStorageAdapter::new(db);

        let sig = create_test_signature(1, 0);

        // Store via adapter
        adapter.store_signature(CheckpointNumber(1), &sig).await.unwrap();

        // Retrieve via adapter
        let sigs = adapter.get_signatures(CheckpointNumber(1)).await.unwrap();
        assert_eq!(sigs.len(), 1);
        assert_eq!(sigs[0].validator_id.0, 0);
    }

    #[tokio::test]
    async fn test_adapter_last_finalized() {
        let temp_dir = TempDir::new().unwrap();
        let db = Arc::new(CheckpointDB::new(temp_dir.path()).unwrap());
        let adapter = CheckpointStorageAdapter::new(db);

        // Initial value
        let initial = adapter.get_last_finalized().await.unwrap();
        assert_eq!(initial, 0);

        // Set value
        adapter.set_last_finalized(42).await.unwrap();

        // Verify
        let updated = adapter.get_last_finalized().await.unwrap();
        assert_eq!(updated, 42);
    }
}
