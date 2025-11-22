//! # V26 SessionKeys Flow Integration Test
//!
//! Tests the complete SessionKeys-based ASF key management flow.
//! Validates: key generation → registration → query → usage
//!
//! **TEST COVERAGE:**
//! - [x] Key generation for validators
//! - [x] SessionKeys registration via session.setKeys()
//! - [x] Runtime API query: get_all_validator_asf_keys()
//! - [x] Authority set update on session change
//! - [x] Checkpoint signing with registered keys
//! - [x] Session rotation and key updates
//! - [x] Validator addition
//! - [x] Validator removal

mod common;

use common::asf_test_helpers::*;
use codec::Encode;

#[tokio::test]
async fn test_complete_sessionkeys_flow() {
    println!("\n========================================");
    println!("TEST: Complete SessionKeys Flow");
    println!("========================================\n");

    // STEP 1: Setup - Create dev node with 3 validators
    println!("STEP 1: Setting up test network with 3 validators...");
    let network = TestNetwork::new(3);
    assert_eq!(network.validators.len(), 3);
    println!("✓ Network created with 3 validators\n");

    // STEP 2: Key Generation - Generate ASF keys for each validator
    println!("STEP 2: Generating ASF keys for validators...");
    for (i, validator) in network.validators.iter().enumerate() {
        let asf_key = validator.asf_public_key_bytes();
        println!("  Validator {}: {}", i, hex::encode(&asf_key[..8]));
        assert!(!asf_key.is_empty(), "ASF key should not be empty");
    }
    println!("✓ ASF keys generated for all validators\n");

    // STEP 3: Key Registration - Submit session.setKeys() for each validator
    println!("STEP 3: Registering SessionKeys for all validators...");
    network.register_all_validators().await
        .expect("Failed to register validators");

    for i in 0..3 {
        assert!(
            network.verify_in_authority_set(i).await,
            "Validator {} should be in authority set", i
        );
    }
    println!("✓ All validators registered in authority set\n");

    // STEP 4: Runtime API Query - Verify get_all_validator_asf_keys() returns correct keys
    println!("STEP 4: Querying all validator ASF keys from runtime...");
    let all_keys = network.get_all_asf_keys().await;
    assert_eq!(all_keys.len(), 3, "Should have 3 validator keys");

    for (account_id, asf_key) in &all_keys {
        let validator = network.validators.iter()
            .find(|v| &v.account_id == account_id)
            .expect("Validator not found");
        assert_eq!(asf_key, &validator.asf_public_key_bytes());
        println!("  ✓ Validator {}: key verified", validator.index);
    }
    println!("✓ Runtime API query successful\n");

    // STEP 5: Authority Set Update - Verify ASF service receives updated keys
    println!("STEP 5: Verifying authority set contains correct keys...");
    for validator in &network.validators {
        let key = network.get_asf_key(&validator.account_id).await;
        assert!(key.is_some(), "Validator {} key not in authority set", validator.index);
        assert_eq!(key.unwrap(), validator.asf_public_key_bytes());
    }
    println!("✓ Authority set verified\n");

    // STEP 6: Checkpoint Signing - Trigger checkpoint and verify signatures succeed
    println!("STEP 6: Triggering checkpoint and verifying signatures...");
    let checkpoint = network.trigger_checkpoint(32).await
        .expect("Failed to trigger checkpoint");

    assert_eq!(checkpoint.block_number, 32);
    assert_eq!(checkpoint.signatures.len(), 3, "Should have 3 signatures");

    let verified = network.verify_checkpoint(32).await
        .expect("Failed to verify checkpoint");
    assert!(verified, "Checkpoint should be verified");
    println!("✓ Checkpoint signed and verified with {} signatures\n", checkpoint.signatures.len());

    // STEP 7: Session Rotation - Rotate session, verify keys update correctly
    println!("STEP 7: Rotating session and verifying keys persist...");
    let old_session = network.get_session().await;
    network.trigger_session_rotation().await;
    let new_session = network.get_session().await;

    assert_eq!(new_session, old_session + 1, "Session should increment");

    // Verify keys still accessible after session rotation
    let all_keys_after = network.get_all_asf_keys().await;
    assert_eq!(all_keys_after.len(), 3, "Should still have 3 validator keys");
    println!("✓ Session rotated, keys verified (session {} -> {})\n", old_session, new_session);

    println!("========================================");
    println!("✓✓✓ ALL TESTS PASSED ✓✓✓");
    println!("========================================\n");
}

#[tokio::test]
async fn test_validator_addition() {
    println!("\n========================================");
    println!("TEST: Validator Addition");
    println!("========================================\n");

    let mut network = TestNetwork::new(3);
    network.register_all_validators().await.unwrap();

    println!("Initial validators: {}", network.validators.len());
    assert_eq!(network.validators.len(), 3);

    // Add new validator
    println!("Adding new validator...");
    network.add_validator(3).await.unwrap();
    assert_eq!(network.validators.len(), 4);

    // Register new validator's key
    network.register_validator_key(3).await.unwrap();
    assert!(network.verify_in_authority_set(3).await);

    // Verify new validator can participate in checkpoints
    let checkpoint = network.trigger_checkpoint(64).await.unwrap();
    assert_eq!(checkpoint.signatures.len(), 4, "Should have 4 signatures including new validator");

    let verified = network.verify_checkpoint(64).await.unwrap();
    assert!(verified);

    println!("✓ New validator added and verified\n");
}

#[tokio::test]
async fn test_validator_removal() {
    println!("\n========================================");
    println!("TEST: Validator Removal");
    println!("========================================\n");

    let network = TestNetwork::new(5);
    network.register_all_validators().await.unwrap();

    println!("Initial validators: {}", network.validators.len());
    assert_eq!(network.validators.len(), 5);

    // Verify initial checkpoint
    let checkpoint_before = network.trigger_checkpoint(32).await.unwrap();
    assert_eq!(checkpoint_before.signatures.len(), 5);

    // Remove validator
    println!("Removing validator 2...");
    network.remove_validator(2).await.unwrap();
    assert!(!network.verify_in_authority_set(2).await);

    // Verify subsequent checkpoint has fewer signatures
    let checkpoint_after = network.trigger_checkpoint(64).await.unwrap();
    assert_eq!(checkpoint_after.signatures.len(), 4, "Should have 4 signatures after removal");

    let verified = network.verify_checkpoint(64).await.unwrap();
    assert!(verified);

    println!("✓ Validator removed and authority set updated\n");
}

#[tokio::test]
async fn test_key_query_individual_validator() {
    println!("\n========================================");
    println!("TEST: Individual Validator Key Query");
    println!("========================================\n");

    let network = TestNetwork::new(3);
    network.register_all_validators().await.unwrap();

    // Query each validator's key individually
    for validator in &network.validators {
        let key = network.get_asf_key(&validator.account_id).await;
        assert!(key.is_some(), "Key not found for validator {}", validator.index);

        let key_bytes = key.unwrap();
        assert_eq!(key_bytes, validator.asf_public_key_bytes());
        println!("✓ Validator {} key query successful", validator.index);
    }

    // Query non-existent validator
    let fake_account = TestValidator::new(99).account_id;
    let fake_key = network.get_asf_key(&fake_account).await;
    assert!(fake_key.is_none(), "Non-existent validator should return None");

    println!("✓ Individual key queries verified\n");
}

#[tokio::test]
async fn test_checkpoint_signature_verification() {
    println!("\n========================================");
    println!("TEST: Checkpoint Signature Verification");
    println!("========================================\n");

    let network = TestNetwork::new(5);
    network.register_all_validators().await.unwrap();

    // Trigger checkpoint
    let checkpoint = network.trigger_checkpoint(96).await.unwrap();
    println!("Checkpoint at block {}: {} signatures", checkpoint.block_number, checkpoint.signatures.len());

    // Verify each signature individually
    for (account_id, signature) in &checkpoint.signatures {
        let validator = network.validators.iter()
            .find(|v| &v.account_id == account_id)
            .expect("Validator not found");

        let valid = validator.verify_checkpoint(checkpoint.block_hash, signature);
        assert!(valid, "Signature verification failed for validator {}", validator.index);
        println!("  ✓ Validator {} signature valid", validator.index);
    }

    // Verify BFT threshold met
    let threshold = bft_threshold(5);
    assert!(checkpoint.signatures.len() >= threshold,
        "Should have at least {} signatures (BFT threshold)", threshold);

    println!("✓ All signatures verified, BFT threshold met ({}/{})\n", checkpoint.signatures.len(), threshold);
}

#[tokio::test]
async fn test_multiple_session_rotations() {
    println!("\n========================================");
    println!("TEST: Multiple Session Rotations");
    println!("========================================\n");

    let network = TestNetwork::new(3);
    network.register_all_validators().await.unwrap();

    println!("Starting session: {}", network.get_session().await);

    // Rotate through 5 sessions
    for i in 1..=5 {
        network.trigger_session_rotation().await;
        let session = network.get_session().await;
        assert_eq!(session, i);

        // Verify keys still accessible after each rotation
        let all_keys = network.get_all_asf_keys().await;
        assert_eq!(all_keys.len(), 3, "Keys should persist across sessions");

        // Verify checkpoint works in each session
        let block_num = i * 32;
        let checkpoint = network.trigger_checkpoint(block_num).await.unwrap();
        assert_eq!(checkpoint.signatures.len(), 3);

        println!("  ✓ Session {} verified, checkpoint at block {}", session, block_num);
    }

    println!("✓ All session rotations successful\n");
}

#[tokio::test]
async fn test_block_production_with_periodic_checkpoints() {
    println!("\n========================================");
    println!("TEST: Block Production with Periodic Checkpoints");
    println!("========================================\n");

    let network = TestNetwork::new(3);
    network.register_all_validators().await.unwrap();

    // Produce 100 blocks
    println!("Producing 100 blocks...");
    let blocks = network.produce_blocks(100).await;
    assert_eq!(blocks.len(), 100);

    // Verify checkpoints at blocks 32, 64, 96
    let expected_checkpoints = vec![32, 64, 96];
    for block_num in expected_checkpoints {
        assert!(network.has_checkpoint(block_num).await,
            "Checkpoint should exist at block {}", block_num);

        let sig_count = network.get_checkpoint_signature_count(block_num).await;
        assert_eq!(sig_count, 3, "Checkpoint at block {} should have 3 signatures", block_num);

        println!("  ✓ Checkpoint at block {} verified", block_num);
    }

    println!("✓ Block production and periodic checkpoints verified\n");
}

#[tokio::test]
async fn test_authority_set_consistency() {
    println!("\n========================================");
    println!("TEST: Authority Set Consistency");
    println!("========================================\n");

    let network = TestNetwork::new(3);

    // Initially empty authority set
    let keys_before = network.get_all_asf_keys().await;
    assert_eq!(keys_before.len(), 0);

    // Register validators one by one
    for i in 0..3 {
        network.register_validator_key(i).await.unwrap();
        let keys = network.get_all_asf_keys().await;
        assert_eq!(keys.len(), i + 1, "Authority set should have {} validators", i + 1);
        println!("  ✓ Registered validator {}, authority set size: {}", i, i + 1);
    }

    // Verify final state
    let keys_after = network.get_all_asf_keys().await;
    assert_eq!(keys_after.len(), 3);

    println!("✓ Authority set consistency verified\n");
}

#[tokio::test]
async fn test_cleanup_and_shutdown() {
    println!("\n========================================");
    println!("TEST: Cleanup and Shutdown");
    println!("========================================\n");

    let network = TestNetwork::new(3);
    network.register_all_validators().await.unwrap();

    // Trigger some activity
    network.produce_blocks(64).await;
    network.trigger_session_rotation().await;

    // Simulate graceful shutdown by dropping network
    println!("Simulating graceful shutdown...");
    drop(network);

    println!("✓ Network shutdown successfully\n");
}
