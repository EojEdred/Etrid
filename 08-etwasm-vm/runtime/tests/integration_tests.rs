//! Integration tests for ETWasm Runtime
//!
//! Tests the complete contract runtime including storage, calls, events, and lifecycle.

use etwasm_runtime::*;
use sp_core::H256;

#[test]
fn test_full_contract_lifecycle() {
    // 1. Deploy a contract
    let mut manager = ContractLifecycleManager::new();
    let deployer = [1u8; 32];
    let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3]; // Simple RETURN

    let params = DeploymentParams::new(deployer, bytecode.clone(), 100000)
        .with_upgradeable()
        .with_initial_balance(1000);

    let result = manager.deploy_contract(params, 100, 1000);
    assert!(result.success);

    let contract_address = result.contract_address.unwrap();

    // 2. Verify contract is active
    let metadata = manager.get_contract(&contract_address).unwrap();
    assert!(metadata.is_callable());
    assert_eq!(metadata.version, 1);

    // 3. Pause the contract
    manager.pause_contract(contract_address, deployer).unwrap();
    let metadata = manager.get_contract(&contract_address).unwrap();
    assert!(!metadata.is_callable());

    // 4. Resume the contract
    manager.resume_contract(contract_address, deployer).unwrap();
    let metadata = manager.get_contract(&contract_address).unwrap();
    assert!(metadata.is_callable());

    // 5. Upgrade the contract
    let new_bytecode = vec![0x60, 0x01, 0x60, 0x00, 0xf3];
    let upgrade_params = UpgradeParams {
        contract_address,
        new_bytecode,
        migration_script: None,
        gas_limit: 100000,
    };

    let gas_used = manager.upgrade_contract(upgrade_params, deployer, 101);
    assert!(gas_used.is_ok());

    let metadata = manager.get_contract(&contract_address).unwrap();
    assert_eq!(metadata.version, 2);

    // 6. Rollback upgrade
    manager.rollback_upgrade(contract_address, deployer).unwrap();
    let metadata = manager.get_contract(&contract_address).unwrap();
    assert_eq!(metadata.version, 1);

    // 7. Destroy the contract
    let destruction_params = DestructionParams {
        contract_address,
        beneficiary: [2u8; 32],
    };

    let balance = manager.destroy_contract(destruction_params, deployer);
    assert!(balance.is_ok());

    let metadata = manager.get_contract(&contract_address).unwrap();
    assert!(metadata.is_destroyed());
}

#[test]
fn test_storage_with_merkle_proofs() {
    let mut storage = AdvancedInMemoryStorage::new(0);

    // 1. Write some data
    let key1 = H256::from_low_u64_be(1);
    let key2 = H256::from_low_u64_be(2);
    let value1 = H256::from_low_u64_be(42);
    let value2 = H256::from_low_u64_be(43);

    storage.write_with_deposit(key1, value1, MINIMUM_STORAGE_DEPOSIT).unwrap();
    storage.write_with_deposit(key2, value2, MINIMUM_STORAGE_DEPOSIT).unwrap();

    // 2. Get storage commitment
    let commitment = storage.get_commitment();
    assert_eq!(commitment.slot_count, 2);
    assert_ne!(commitment.root, H256::zero());

    // 3. Generate merkle proof
    let proof = storage.generate_proof(&key1);
    assert!(proof.is_some());

    let proof = proof.unwrap();
    assert_eq!(proof.key, key1);
    assert_eq!(proof.value, value1);

    // 4. Test access tracking (EIP-2929)
    let (val, mode) = storage.read_with_access(&key1);
    assert_eq!(val, Some(value1));
    assert_eq!(mode, StorageAccessMode::Cold);

    let (val, mode) = storage.read_with_access(&key1);
    assert_eq!(val, Some(value1));
    assert_eq!(mode, StorageAccessMode::Warm);

    // 5. Clear cache and verify cold again
    storage.clear_access_cache();
    let (val, mode) = storage.read_with_access(&key1);
    assert_eq!(val, Some(value1));
    assert_eq!(mode, StorageAccessMode::Cold);
}

#[test]
fn test_storage_rent_system() {
    let mut storage = AdvancedInMemoryStorage::new(0);
    let key = H256::from_low_u64_be(1);
    let value = H256::from_low_u64_be(42);

    // 1. Write with deposit
    let deposit = MINIMUM_STORAGE_DEPOSIT * 2;
    storage.write_with_deposit(key, value, deposit).unwrap();

    // 2. No rent owed initially
    assert_eq!(storage.calculate_total_rent(0), 0);

    // 3. Advance 100 blocks
    storage.set_current_block(100);
    let rent_owed = storage.calculate_total_rent(100);
    assert!(rent_owed > 0);

    // 4. Pay rent
    let rent_paid = storage.pay_rent(100).unwrap();
    assert_eq!(rent_paid, rent_owed);

    // 5. After paying, no rent owed
    assert_eq!(storage.calculate_total_rent(100), 0);

    // 6. Check deposit was reduced
    let entry = storage.get_entry(&key).unwrap();
    assert!(entry.deposit < deposit);
}

#[test]
fn test_cross_contract_calls() {
    let caller = [1u8; 32];
    let target = [2u8; 32];

    // Mock code loader
    let load_code = |_address: [u8; 32]| -> Option<Vec<u8>> {
        Some(vec![0x60, 0x00, 0x60, 0x00, 0xf3])
    };

    // 1. Test CALL
    let params = CallParams::new_call(caller, target, 100, 100000, vec![1, 2, 3]);
    let mut call_context = CallContext::new(ExecutionContext::default());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(result.success);

    // 2. Test DELEGATECALL
    let params = CallParams::new_delegatecall(caller, target, 100000, vec![1, 2, 3]);
    let mut call_context = CallContext::new(ExecutionContext::default());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(result.success);

    // 3. Test STATICCALL
    let params = CallParams::new_staticcall(caller, target, 100000, vec![1, 2, 3]);
    let mut call_context = CallContext::new(ExecutionContext::default());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(result.success);
}

#[test]
fn test_contract_creation() {
    let caller = [1u8; 32];
    let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];

    let load_code = |_address: [u8; 32]| -> Option<Vec<u8>> {
        Some(vec![0x60, 0x00, 0x60, 0x00, 0xf3])
    };

    // 1. Test CREATE
    let params = CallParams::new_create(caller, 0, 100000, bytecode.clone());
    let mut call_context = CallContext::new(ExecutionContext::default());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(result.success);
    assert!(result.created_address.is_some());

    // 2. Test CREATE2 (deterministic)
    let salt = H256::from_low_u64_be(42);
    let params1 = CallParams::new_create2(caller, 0, 100000, bytecode.clone(), salt);
    let params2 = CallParams::new_create2(caller, 0, 100000, bytecode.clone(), salt);

    let mut call_context = CallContext::new(ExecutionContext::default());
    let result1 = CallExecutor::execute_call(params1, &mut call_context, load_code);

    let mut call_context = CallContext::new(ExecutionContext::default());
    let result2 = CallExecutor::execute_call(params2, &mut call_context, load_code);

    // Addresses should be the same
    assert_eq!(result1.created_address, result2.created_address);
}

#[test]
fn test_event_system() {
    let mut store = EventStore::new();
    let address1 = [1u8; 32];
    let address2 = [2u8; 32];
    let topic1 = H256::from_low_u64_be(1);
    let topic2 = H256::from_low_u64_be(2);

    // 1. Emit events
    let log1 = EventLog::new(address1, vec![topic1], vec![1, 2, 3], 100, 0, 0).unwrap();
    let log2 = EventLog::new(address1, vec![topic2], vec![4, 5, 6], 100, 0, 1).unwrap();
    let log3 = EventLog::new(address2, vec![topic1], vec![7, 8, 9], 101, 0, 0).unwrap();

    store.add_event(log1).unwrap();
    store.add_event(log2).unwrap();
    store.add_event(log3).unwrap();

    assert_eq!(store.event_count(), 3);

    // 2. Query by address
    let events = store.get_events_by_address(&address1);
    assert_eq!(events.len(), 2);

    // 3. Query by topic
    let events = store.get_events_by_topic(&topic1);
    assert_eq!(events.len(), 2);

    // 4. Query with filter
    let filter = EventFilter::new(100, 101)
        .with_address(address1)
        .with_topic(Some(topic1));
    let results = store.query_events(&filter);
    assert_eq!(results.len(), 1);

    // 5. Query with limit
    let filter = EventFilter::new(100, 101).with_limit(2);
    let results = store.query_events(&filter);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_bloom_filter_optimization() {
    let mut store = EventStore::new();
    let address = [1u8; 32];
    let topic = H256::from_low_u64_be(42);

    // Add event
    let log = EventLog::new(address, vec![topic], vec![], 100, 0, 0).unwrap();
    store.add_event(log).unwrap();

    // Get bloom filter
    let bloom = store.get_block_bloom(100);
    assert!(bloom.is_some());

    let bloom = bloom.unwrap();
    assert!(bloom.contains(&address));
    assert!(bloom.contains(topic.as_bytes()));
    assert!(!bloom.contains(b"nonexistent"));
}

#[test]
fn test_event_pruning() {
    let mut store = EventStore::new();
    let address = [1u8; 32];

    // Add events at different blocks
    store.add_event(EventLog::new(address, vec![], vec![], 100, 0, 0).unwrap()).unwrap();
    store.add_event(EventLog::new(address, vec![], vec![], 200, 0, 0).unwrap()).unwrap();
    store.add_event(EventLog::new(address, vec![], vec![], 300, 0, 0).unwrap()).unwrap();

    assert_eq!(store.event_count(), 3);

    // Prune events before block 200
    store.prune_before_block(200);

    let filter = EventFilter::new(0, 1000);
    let results = store.query_events(&filter);
    assert_eq!(results.len(), 2); // Only blocks 200 and 300 remain
}

#[test]
fn test_integrated_contract_with_events() {
    // This test demonstrates a complete contract scenario:
    // 1. Deploy contract
    // 2. Execute calls
    // 3. Emit events
    // 4. Query events
    // 5. Upgrade contract
    // 6. Emit more events

    let mut manager = ContractLifecycleManager::new();
    let mut event_store = EventStore::new();
    let owner = [1u8; 32];

    // Deploy contract
    let params = DeploymentParams::new(owner, vec![0x60, 0x00], 100000)
        .with_upgradeable();
    let result = manager.deploy_contract(params, 100, 1000);
    assert!(result.success);

    let contract_address = result.contract_address.unwrap();

    // Emit deployment event
    let deployment_topic = H256::from_low_u64_be(0xDEAD);
    let log = EventLog::new(
        contract_address,
        vec![deployment_topic],
        contract_address.to_vec(),
        100,
        0,
        0
    ).unwrap();
    event_store.add_event(log).unwrap();

    // Upgrade contract
    let upgrade_params = UpgradeParams {
        contract_address,
        new_bytecode: vec![0x60, 0x01],
        migration_script: None,
        gas_limit: 100000,
    };
    manager.upgrade_contract(upgrade_params, owner, 101).unwrap();

    // Emit upgrade event
    let upgrade_topic = H256::from_low_u64_be(0xBEEF);
    let log = EventLog::new(
        contract_address,
        vec![upgrade_topic],
        vec![2], // version 2
        101,
        0,
        0
    ).unwrap();
    event_store.add_event(log).unwrap();

    // Query all events for this contract
    let events = event_store.get_events_by_address(&contract_address);
    assert_eq!(events.len(), 2);

    // Verify events are in correct order
    assert_eq!(events[0].topics[0], deployment_topic);
    assert_eq!(events[1].topics[0], upgrade_topic);
}

#[test]
fn test_reentrancy_protection_with_calls() {
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    let mut call_context = CallContext::new(ExecutionContext::default());

    // A calls B - should work
    call_context.context.enter_call(contract_a).unwrap();
    call_context.context.enter_call(contract_b).unwrap();

    // B tries to call A again (reentrancy) - should fail
    let result = call_context.context.enter_call(contract_a);
    assert!(result.is_err());

    // Cleanup
    call_context.context.exit_call(&contract_b);
    call_context.context.exit_call(&contract_a);

    // Now A can be called again (not in call stack anymore)
    let result = call_context.context.enter_call(contract_a);
    assert!(result.is_ok());
}

#[test]
fn test_static_call_restrictions() {
    let caller = [1u8; 32];
    let bytecode = vec![0x60, 0x00];

    let load_code = |_address: [u8; 32]| -> Option<Vec<u8>> {
        Some(vec![0x60, 0x00, 0x60, 0x00, 0xf3])
    };

    // 1. Enter static context
    let mut call_context = CallContext::new(ExecutionContext::default());
    call_context.enter_static();

    // 2. Try to CREATE in static context - should fail
    let params = CallParams::new_create(caller, 0, 100000, bytecode.clone());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(!result.success);

    // 3. Try CREATE2 in static context - should fail
    let params = CallParams::new_create2(caller, 0, 100000, bytecode, H256::zero());
    let result = CallExecutor::execute_call(params, &mut call_context, load_code);
    assert!(!result.success);
}
