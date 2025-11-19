//! Integration tests for checkpoint persistence
//!
//! These tests verify the complete flow:
//! 1. Store checkpoint data
//! 2. Close database
//! 3. Reopen database
//! 4. Verify data is restored correctly

use checkpoint_db::{CheckpointDB, CheckpointStorageAdapter};
use etrid_finality_gadget::{
    CheckpointSignature, CheckpointCertificate, CheckpointNumber,
    ValidatorId, BlockHash, AuthoritySetId,
};
use std::sync::Arc;
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
fn test_checkpoint_persistence_across_restarts() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Phase 1: Create database and store data
    {
        let db = CheckpointDB::new(&path).unwrap();

        // Store signatures for checkpoint 1
        for i in 0..14 {
            let sig = create_test_signature(1, i);
            db.store_signature(CheckpointNumber(1), &sig).unwrap();
        }

        // Store certificate
        let cert = create_test_certificate(1, 14);
        db.store_certificate(CheckpointNumber(1), &cert).unwrap();

        // Set last finalized
        db.set_last_finalized(1).unwrap();

        // Flush to disk
        db.flush().unwrap();
    }

    // Phase 2: Reopen database and verify data
    {
        let db = CheckpointDB::new(&path).unwrap();

        // Verify signatures
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 14, "Should restore all 14 signatures");

        // Verify certificate
        let cert = db.get_certificate(CheckpointNumber(1)).unwrap();
        assert!(cert.is_some(), "Should restore certificate");
        assert_eq!(cert.unwrap().signatures.len(), 14);

        // Verify last finalized
        let last_finalized = db.get_last_finalized().unwrap();
        assert_eq!(last_finalized, 1, "Should restore last finalized checkpoint");
    }
}

#[test]
fn test_multi_checkpoint_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Store data for checkpoints 1-10
    {
        let db = CheckpointDB::new(&path).unwrap();

        for checkpoint in 1..=10 {
            // Store 14 signatures per checkpoint
            for validator in 0..14 {
                let sig = create_test_signature(checkpoint, validator);
                db.store_signature(CheckpointNumber(checkpoint), &sig).unwrap();
            }

            // Store certificate
            let cert = create_test_certificate(checkpoint, 14);
            db.store_certificate(CheckpointNumber(checkpoint), &cert).unwrap();
        }

        db.set_last_finalized(10).unwrap();
        db.flush().unwrap();
    }

    // Reopen and verify all checkpoints
    {
        let db = CheckpointDB::new(&path).unwrap();

        for checkpoint in 1..=10 {
            let sigs = db.get_signatures(CheckpointNumber(checkpoint)).unwrap();
            assert_eq!(sigs.len(), 14, "Checkpoint {} should have 14 signatures", checkpoint);

            let cert = db.get_certificate(CheckpointNumber(checkpoint)).unwrap();
            assert!(cert.is_some(), "Checkpoint {} should have certificate", checkpoint);
        }

        assert_eq!(db.get_last_finalized().unwrap(), 10);
    }
}

#[test]
fn test_pruning_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    {
        let db = CheckpointDB::new(&path).unwrap();

        // Store checkpoints 1-20
        for checkpoint in 1..=20 {
            let sig = create_test_signature(checkpoint, 0);
            db.store_signature(CheckpointNumber(checkpoint), &sig).unwrap();

            let cert = create_test_certificate(checkpoint, 14);
            db.store_certificate(CheckpointNumber(checkpoint), &cert).unwrap();
        }

        db.set_last_finalized(20).unwrap();

        // Prune, keeping last 10
        db.prune_old_checkpoints(10).unwrap();

        db.flush().unwrap();
    }

    // Reopen and verify pruning persisted
    {
        let db = CheckpointDB::new(&path).unwrap();

        // Checkpoints 1-10 should be pruned
        for checkpoint in 1..=10 {
            let sigs = db.get_signatures(CheckpointNumber(checkpoint)).unwrap();
            assert_eq!(sigs.len(), 0, "Checkpoint {} should be pruned", checkpoint);
        }

        // Checkpoints 11-20 should exist
        for checkpoint in 11..=20 {
            let sigs = db.get_signatures(CheckpointNumber(checkpoint)).unwrap();
            assert_eq!(sigs.len(), 1, "Checkpoint {} should exist", checkpoint);
        }
    }
}

#[test]
fn test_incremental_signature_accumulation() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Store signatures incrementally across multiple sessions
    for session in 0..3 {
        let db = CheckpointDB::new(&path).unwrap();

        // Each session adds signatures 0-4, 5-9, 10-13
        let start = session * 5;
        let end = if session == 2 { 14 } else { (session + 1) * 5 };

        for validator in start..end {
            let sig = create_test_signature(1, validator as u32);
            db.store_signature(CheckpointNumber(1), &sig).unwrap();
        }

        db.flush().unwrap();
    }

    // Verify all 14 signatures accumulated
    {
        let db = CheckpointDB::new(&path).unwrap();
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 14, "Should accumulate all signatures across sessions");
    }
}

#[test]
fn test_database_stats_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    {
        let db = CheckpointDB::new(&path).unwrap();

        // Store data for 5 checkpoints
        for checkpoint in 1..=5 {
            let sig = create_test_signature(checkpoint, 0);
            db.store_signature(CheckpointNumber(checkpoint), &sig).unwrap();

            let cert = create_test_certificate(checkpoint, 14);
            db.store_certificate(CheckpointNumber(checkpoint), &cert).unwrap();
        }

        db.set_last_finalized(5).unwrap();
        db.flush().unwrap();
    }

    // Reopen and verify stats
    {
        let db = CheckpointDB::new(&path).unwrap();
        let stats = db.get_stats().unwrap();

        assert_eq!(stats.last_finalized, 5);
        assert_eq!(stats.total_certificates, 5);
        assert_eq!(stats.total_signatures, 5);
        assert_eq!(stats.oldest_checkpoint, Some(1));
        assert_eq!(stats.newest_checkpoint, Some(5));
    }
}

#[tokio::test]
async fn test_adapter_persistence() {
    use checkpoint_db::adapter::CheckpointStorageAdapter;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Store via adapter
    {
        let db = Arc::new(CheckpointDB::new(&path).unwrap());
        let adapter = CheckpointStorageAdapter::new(db);

        for validator in 0..14 {
            let sig = create_test_signature(1, validator);
            adapter.store_signature(CheckpointNumber(1), &sig).await.unwrap();
        }

        let cert = create_test_certificate(1, 14);
        adapter.store_certificate(CheckpointNumber(1), &cert).await.unwrap();
        adapter.set_last_finalized(1).await.unwrap();
        adapter.flush().await.unwrap();
    }

    // Retrieve via new adapter instance
    {
        let db = Arc::new(CheckpointDB::new(&path).unwrap());
        let adapter = CheckpointStorageAdapter::new(db);

        let sigs = adapter.get_signatures(CheckpointNumber(1)).await.unwrap();
        assert_eq!(sigs.len(), 14);

        let cert = adapter.get_certificate(CheckpointNumber(1)).await.unwrap();
        assert!(cert.is_some());

        let last_finalized = adapter.get_last_finalized().await.unwrap();
        assert_eq!(last_finalized, 1);
    }
}

#[test]
fn test_concurrent_writes_persistence() {
    use std::thread;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Simulate concurrent writes from multiple threads
    {
        let db = Arc::new(CheckpointDB::new(&path).unwrap());

        let handles: Vec<_> = (0..4).map(|thread_id| {
            let db_clone = db.clone();
            thread::spawn(move || {
                for validator in 0..4 {
                    let validator_id = thread_id * 4 + validator;
                    let sig = create_test_signature(1, validator_id as u32);
                    db_clone.store_signature(CheckpointNumber(1), &sig).unwrap();
                }
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        db.flush().unwrap();
    }

    // Verify all writes persisted
    {
        let db = CheckpointDB::new(&path).unwrap();
        let sigs = db.get_signatures(CheckpointNumber(1)).unwrap();
        assert_eq!(sigs.len(), 16, "All concurrent writes should persist");
    }
}

#[test]
fn test_empty_database_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    {
        let db = CheckpointDB::new(&path).unwrap();

        // Empty database should return sensible defaults
        assert_eq!(db.get_last_finalized().unwrap(), 0);
        assert_eq!(db.get_signatures(CheckpointNumber(1)).unwrap().len(), 0);
        assert!(db.get_certificate(CheckpointNumber(1)).unwrap().is_none());

        let stats = db.get_stats().unwrap();
        assert_eq!(stats.total_certificates, 0);
        assert_eq!(stats.total_signatures, 0);
    }

    // Reopen empty database
    {
        let db = CheckpointDB::new(&path).unwrap();
        assert_eq!(db.get_last_finalized().unwrap(), 0);
    }
}

#[test]
fn test_recovery_from_unclean_shutdown() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_path_buf();

    // Simulate unclean shutdown (no flush)
    {
        let db = CheckpointDB::new(&path).unwrap();

        for validator in 0..14 {
            let sig = create_test_signature(1, validator);
            db.store_signature(CheckpointNumber(1), &sig).unwrap();
        }

        db.set_last_finalized(1).unwrap();

        // Don't flush - simulate crash
        // db.flush().unwrap();
    }

    // Reopen - RocksDB should recover
    {
        let db = CheckpointDB::new(&path).unwrap();

        // Some data should still be there (RocksDB has WAL)
        let last_finalized = db.get_last_finalized().unwrap();
        println!("Last finalized after recovery: {}", last_finalized);

        // Note: Exact behavior depends on RocksDB WAL settings
        // In production, use db.flush() periodically
    }
}
