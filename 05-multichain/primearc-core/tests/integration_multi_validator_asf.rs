//! # Multi-Validator ASF Test (21 Validators)
//!
//! Tests SessionKeys and ASF consensus with production-scale validator set.
//! Matches mainnet configuration of 21 validators.
//!
//! **TEST COVERAGE:**
//! - [x] 21-validator network initialization
//! - [x] ASF key registration for all 21 validators
//! - [x] Block production for 100+ blocks
//! - [x] Checkpoint generation every 32 blocks
//! - [x] BFT threshold verification (15/21 signatures)
//! - [x] Validator rotation with partial set changes
//! - [x] ASF consensus continuity during validator changes

mod common;

use common::asf_test_helpers::*;

const MAINNET_VALIDATOR_COUNT: usize = 21;
const CHECKPOINT_INTERVAL: u32 = 32;
const BFT_THRESHOLD_21: usize = 15; // (21 * 2/3) + 1 = 15

#[tokio::test]
async fn test_21_validator_network_initialization() {
    println!("\n========================================");
    println!("TEST: 21-Validator Network Initialization");
    println!("========================================\n");

    println!("Creating network with {} validators...", MAINNET_VALIDATOR_COUNT);
    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);

    assert_eq!(network.validators.len(), MAINNET_VALIDATOR_COUNT);
    println!("✓ Network created with {} validators", MAINNET_VALIDATOR_COUNT);

    // Verify each validator has unique keys
    println!("\nVerifying unique ASF keys for all validators...");
    let mut keys_set = std::collections::HashSet::new();

    for validator in &network.validators {
        let key = validator.asf_public_key_bytes();
        assert!(!keys_set.contains(&key), "Duplicate ASF key detected");
        keys_set.insert(key);
    }

    println!("✓ All {} validators have unique ASF keys\n", MAINNET_VALIDATOR_COUNT);

    // Verify BFT threshold calculation
    let threshold = bft_threshold(MAINNET_VALIDATOR_COUNT);
    assert_eq!(threshold, BFT_THRESHOLD_21);
    println!("✓ BFT threshold: {} validators\n", threshold);
}

#[tokio::test]
async fn test_21_validator_registration() {
    println!("\n========================================");
    println!("TEST: 21-Validator SessionKeys Registration");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);

    println!("Registering SessionKeys for {} validators...", MAINNET_VALIDATOR_COUNT);
    let start = std::time::Instant::now();

    network.register_all_validators().await
        .expect("Failed to register validators");

    let duration = start.elapsed();
    println!("✓ Registration completed in {:?}", duration);

    // Verify all validators in authority set
    println!("\nVerifying authority set...");
    for i in 0..MAINNET_VALIDATOR_COUNT {
        assert!(
            network.verify_in_authority_set(i).await,
            "Validator {} not in authority set", i
        );
    }

    let all_keys = network.get_all_asf_keys().await;
    assert_eq!(all_keys.len(), MAINNET_VALIDATOR_COUNT);
    println!("✓ All {} validators in authority set\n", MAINNET_VALIDATOR_COUNT);
}

#[tokio::test]
async fn test_100_block_production() {
    println!("\n========================================");
    println!("TEST: 100-Block Production with 21 Validators");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    println!("Producing 100 blocks...");
    let start = std::time::Instant::now();

    let blocks = network.produce_blocks(100).await;

    let duration = start.elapsed();
    assert_eq!(blocks.len(), 100);

    println!("✓ Produced 100 blocks in {:?}", duration);
    println!("  Average: {:?} per block", duration / 100);

    // Verify checkpoints
    let expected_checkpoints = vec![32, 64, 96];
    println!("\nVerifying checkpoints...");

    for block_num in expected_checkpoints {
        assert!(network.has_checkpoint(block_num).await,
            "Checkpoint should exist at block {}", block_num);

        let sig_count = network.get_checkpoint_signature_count(block_num).await;
        assert_eq!(sig_count, MAINNET_VALIDATOR_COUNT,
            "Checkpoint at block {} should have {} signatures", block_num, MAINNET_VALIDATOR_COUNT);

        println!("  ✓ Block {}: {} signatures", block_num, sig_count);
    }

    println!("\n✓ All checkpoints verified\n");
}

#[tokio::test]
async fn test_bft_threshold_verification() {
    println!("\n========================================");
    println!("TEST: BFT Threshold Verification (15/21)");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    let checkpoint = network.trigger_checkpoint(CHECKPOINT_INTERVAL).await.unwrap();

    println!("Checkpoint at block {}:", checkpoint.block_number);
    println!("  Total signatures: {}", checkpoint.signatures.len());
    println!("  BFT threshold: {}", BFT_THRESHOLD_21);
    println!("  Threshold met: {}", checkpoint.signatures.len() >= BFT_THRESHOLD_21);

    assert!(checkpoint.signatures.len() >= BFT_THRESHOLD_21,
        "Should have at least {} signatures", BFT_THRESHOLD_21);

    // Verify checkpoint
    let verified = network.verify_checkpoint(CHECKPOINT_INTERVAL).await.unwrap();
    assert!(verified);

    println!("\n✓ BFT threshold met and verified\n");
}

#[tokio::test]
async fn test_multiple_checkpoints_21_validators() {
    println!("\n========================================");
    println!("TEST: Multiple Checkpoints with 21 Validators");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    println!("Creating 5 checkpoints...");

    for i in 1..=5 {
        let block_num = i * CHECKPOINT_INTERVAL;
        println!("\nCheckpoint {} at block {}:", i, block_num);

        let checkpoint = network.trigger_checkpoint(block_num).await.unwrap();
        println!("  Signatures: {}", checkpoint.signatures.len());

        assert_eq!(checkpoint.signatures.len(), MAINNET_VALIDATOR_COUNT);

        let verified = network.verify_checkpoint(block_num).await.unwrap();
        assert!(verified);
        println!("  ✓ Verified");
    }

    println!("\n✓ All 5 checkpoints created and verified\n");
}

#[tokio::test]
async fn test_validator_rotation_partial() {
    println!("\n========================================");
    println!("TEST: Partial Validator Rotation (21 Validators)");
    println!("========================================\n");

    let mut network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    // Initial checkpoint
    println!("Creating initial checkpoint...");
    let checkpoint1 = network.trigger_checkpoint(32).await.unwrap();
    assert_eq!(checkpoint1.signatures.len(), MAINNET_VALIDATOR_COUNT);
    println!("  ✓ Initial checkpoint: {} signatures\n", checkpoint1.signatures.len());

    // Simulate validator rotation: remove 5 validators
    println!("Rotating validators: removing 5, keeping 16...");
    for i in 0..5 {
        network.remove_validator(i).await.unwrap();
    }

    let remaining_validators = MAINNET_VALIDATOR_COUNT - 5;
    let all_keys = network.get_all_asf_keys().await;
    assert_eq!(all_keys.len(), remaining_validators);
    println!("  Authority set size: {} validators", all_keys.len());

    // Checkpoint with reduced set
    let checkpoint2 = network.trigger_checkpoint(64).await.unwrap();
    assert_eq!(checkpoint2.signatures.len(), remaining_validators);
    println!("  ✓ Checkpoint after rotation: {} signatures\n", checkpoint2.signatures.len());

    // Verify BFT threshold still met
    let threshold = bft_threshold(remaining_validators);
    println!("BFT threshold for {} validators: {}", remaining_validators, threshold);
    assert!(checkpoint2.signatures.len() >= threshold);

    let verified = network.verify_checkpoint(64).await.unwrap();
    assert!(verified);
    println!("✓ Consensus maintained with reduced validator set\n");
}

#[tokio::test]
async fn test_validator_addition_to_21() {
    println!("\n========================================");
    println!("TEST: Validator Addition to 21-Validator Set");
    println!("========================================\n");

    let mut network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    // Add 3 new validators
    println!("Adding 3 new validators to the set...");
    for i in MAINNET_VALIDATOR_COUNT..(MAINNET_VALIDATOR_COUNT + 3) {
        network.add_validator(i).await.unwrap();
        network.register_validator_key(i).await.unwrap();
        println!("  ✓ Added validator {}", i);
    }

    let all_keys = network.get_all_asf_keys().await;
    assert_eq!(all_keys.len(), MAINNET_VALIDATOR_COUNT + 3);
    println!("\nAuthority set size: {} validators", all_keys.len());

    // Checkpoint with expanded set
    let checkpoint = network.trigger_checkpoint(32).await.unwrap();
    assert_eq!(checkpoint.signatures.len(), MAINNET_VALIDATOR_COUNT + 3);
    println!("Checkpoint signatures: {}", checkpoint.signatures.len());

    let verified = network.verify_checkpoint(32).await.unwrap();
    assert!(verified);
    println!("✓ Consensus maintained with expanded validator set\n");
}

#[tokio::test]
async fn test_21_validator_session_rotation() {
    println!("\n========================================");
    println!("TEST: Session Rotation with 21 Validators");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    println!("Testing session rotations...");

    for session in 1..=3 {
        println!("\nSession {}:", session);

        network.trigger_session_rotation().await;
        assert_eq!(network.get_session().await, session);

        // Verify authority set intact
        let all_keys = network.get_all_asf_keys().await;
        assert_eq!(all_keys.len(), MAINNET_VALIDATOR_COUNT);
        println!("  ✓ Authority set: {} validators", all_keys.len());

        // Checkpoint in each session
        let block_num = session * CHECKPOINT_INTERVAL;
        let checkpoint = network.trigger_checkpoint(block_num).await.unwrap();
        assert_eq!(checkpoint.signatures.len(), MAINNET_VALIDATOR_COUNT);
        println!("  ✓ Checkpoint: {} signatures", checkpoint.signatures.len());
    }

    println!("\n✓ All session rotations successful\n");
}

#[tokio::test]
async fn test_21_validator_byzantine_tolerance() {
    println!("\n========================================");
    println!("TEST: Byzantine Tolerance with 21 Validators");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    // Maximum Byzantine validators: 6 (21 - 15 threshold = 6)
    let max_byzantine = MAINNET_VALIDATOR_COUNT - BFT_THRESHOLD_21;
    println!("Maximum Byzantine validators tolerated: {}", max_byzantine);
    println!("Simulating {} Byzantine validators...\n", max_byzantine);

    // Remove Byzantine validators from authority set
    for i in 0..max_byzantine {
        network.remove_validator(i).await.unwrap();
        println!("  Validator {} acting Byzantine (removed)", i);
    }

    let honest_validators = MAINNET_VALIDATOR_COUNT - max_byzantine;
    let all_keys = network.get_all_asf_keys().await;
    assert_eq!(all_keys.len(), honest_validators);
    println!("\nHonest validators: {}", honest_validators);
    println!("BFT threshold: {}", BFT_THRESHOLD_21);

    // Should still reach consensus with exactly BFT threshold
    assert_eq!(honest_validators, BFT_THRESHOLD_21);

    let checkpoint = network.trigger_checkpoint(32).await.unwrap();
    assert_eq!(checkpoint.signatures.len(), BFT_THRESHOLD_21);

    let verified = network.verify_checkpoint(32).await.unwrap();
    assert!(verified);

    println!("\n✓ Consensus maintained with maximum Byzantine tolerance\n");
}

#[tokio::test]
async fn test_21_validator_stress_test() {
    println!("\n========================================");
    println!("TEST: 21-Validator Stress Test (200 Blocks)");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    println!("Producing 200 blocks with periodic checkpoints...");
    let start = std::time::Instant::now();

    let blocks = network.produce_blocks(200).await;

    let duration = start.elapsed();
    println!("✓ Produced {} blocks in {:?}", blocks.len(), duration);
    println!("  Average: {:?} per block\n", duration / blocks.len() as u32);

    // Verify all checkpoints (every 32 blocks)
    let checkpoint_blocks: Vec<u32> = (1..=6).map(|i| i * 32).collect();
    println!("Verifying {} checkpoints...", checkpoint_blocks.len());

    for block_num in checkpoint_blocks {
        assert!(network.has_checkpoint(block_num).await);
        let sig_count = network.get_checkpoint_signature_count(block_num).await;
        assert_eq!(sig_count, MAINNET_VALIDATOR_COUNT);
        println!("  ✓ Block {}: {} signatures", block_num, sig_count);
    }

    println!("\n✓ Stress test completed successfully\n");
}

#[tokio::test]
async fn test_21_validator_signature_uniqueness() {
    println!("\n========================================");
    println!("TEST: Signature Uniqueness (21 Validators)");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    let checkpoint = network.trigger_checkpoint(32).await.unwrap();

    println!("Verifying signature uniqueness...");
    let mut signatures_set = std::collections::HashSet::new();

    for (i, (account_id, signature)) in checkpoint.signatures.iter().enumerate() {
        let sig_bytes = signature.encode();
        assert!(!signatures_set.contains(&sig_bytes),
            "Duplicate signature detected at index {}", i);
        signatures_set.insert(sig_bytes);

        // Also verify account uniqueness
        let account_count = checkpoint.signatures.iter()
            .filter(|(acc, _)| acc == account_id)
            .count();
        assert_eq!(account_count, 1, "Duplicate account in signatures");
    }

    println!("✓ All {} signatures are unique\n", checkpoint.signatures.len());
}

#[tokio::test]
async fn test_21_validator_parallel_checkpoints() {
    println!("\n========================================");
    println!("TEST: Parallel Checkpoint Processing");
    println!("========================================\n");

    let network = TestNetwork::new(MAINNET_VALIDATOR_COUNT);
    network.register_all_validators().await.unwrap();

    println!("Creating 3 checkpoints in parallel...");
    let start = std::time::Instant::now();

    // Trigger multiple checkpoints concurrently
    let checkpoint_blocks = vec![32, 64, 96];
    let mut handles = vec![];

    for block_num in checkpoint_blocks {
        let network_clone = network.clone();
        let handle = tokio::spawn(async move {
            network_clone.trigger_checkpoint(block_num).await
        });
        handles.push((block_num, handle));
    }

    // Wait for all checkpoints
    for (block_num, handle) in handles {
        let checkpoint = handle.await.unwrap().unwrap();
        assert_eq!(checkpoint.block_number, block_num);
        assert_eq!(checkpoint.signatures.len(), MAINNET_VALIDATOR_COUNT);
        println!("  ✓ Checkpoint {}: {} signatures", block_num, checkpoint.signatures.len());
    }

    let duration = start.elapsed();
    println!("\n✓ 3 parallel checkpoints completed in {:?}\n", duration);
}

impl Clone for TestNetwork {
    fn clone(&self) -> Self {
        Self {
            validators: self.validators.clone(),
            session: Arc::clone(&self.session),
            authority_set: Arc::clone(&self.authority_set),
            checkpoints: Arc::clone(&self.checkpoints),
        }
    }
}
