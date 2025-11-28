//! Comprehensive Reentrancy Protection Tests
//!
//! This test suite validates the reentrancy protection mechanisms in the ETWasm VM.
//! It covers direct reentrancy, indirect reentrancy, complex chains, max depth,
//! state locking, and cleanup scenarios.

use etwasm_runtime::{
    ExecutionContext, ExecutionError, ExecutionResult, StateLock,
    InMemoryStorage, host_functions::*,
};
use sp_core::H256;

// ============================================================================
// TEST 1: Direct Reentrancy - Contract A calls itself (A -> A)
// ============================================================================

#[test]
fn test_direct_reentrancy_blocked() {
    let mut ctx = ExecutionContext::default();
    let contract_a = [1u8; 32];

    // First call to contract A - should succeed
    assert!(ctx.enter_call(contract_a).is_ok());
    assert!(ctx.is_in_call_stack(&contract_a));
    assert_eq!(ctx.call_depth(), 1);

    // Contract A tries to call itself - should fail with ReentrancyDetected
    let result = ctx.enter_call(contract_a);
    assert_eq!(result, Err(ExecutionError::ReentrancyDetected));

    // Call depth should not increase
    assert_eq!(ctx.call_depth(), 1);

    // Cleanup
    ctx.exit_call(&contract_a);
    assert_eq!(ctx.call_depth(), 0);
}

// ============================================================================
// TEST 2: Indirect Reentrancy - A calls B, B calls A (A -> B -> A)
// ============================================================================

#[test]
fn test_indirect_reentrancy_blocked() {
    let mut ctx = ExecutionContext::default();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // A calls B - should succeed
    assert!(ctx.enter_call(contract_a).is_ok());
    assert_eq!(ctx.call_depth(), 1);

    assert!(ctx.enter_call(contract_b).is_ok());
    assert_eq!(ctx.call_depth(), 2);

    // B tries to call A (reentrancy) - should fail
    let result = ctx.enter_call(contract_a);
    assert_eq!(result, Err(ExecutionError::ReentrancyDetected));

    // Call depth should not increase
    assert_eq!(ctx.call_depth(), 2);

    // Cleanup
    ctx.exit_call(&contract_b);
    ctx.exit_call(&contract_a);
    assert_eq!(ctx.call_depth(), 0);
}

// ============================================================================
// TEST 3: Complex Reentrancy Chain - A -> B -> C -> A
// ============================================================================

#[test]
fn test_complex_reentrancy_chain_blocked() {
    let mut ctx = ExecutionContext::default();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];
    let contract_c = [3u8; 32];

    // A -> B -> C
    assert!(ctx.enter_call(contract_a).is_ok());
    assert!(ctx.enter_call(contract_b).is_ok());
    assert!(ctx.enter_call(contract_c).is_ok());
    assert_eq!(ctx.call_depth(), 3);

    // C tries to call A (reentrancy) - should fail
    let result = ctx.enter_call(contract_a);
    assert_eq!(result, Err(ExecutionError::ReentrancyDetected));

    // C tries to call B (reentrancy) - should also fail
    let result = ctx.enter_call(contract_b);
    assert_eq!(result, Err(ExecutionError::ReentrancyDetected));

    // Cleanup
    ctx.exit_call(&contract_c);
    ctx.exit_call(&contract_b);
    ctx.exit_call(&contract_a);
    assert_eq!(ctx.call_depth(), 0);
}

// ============================================================================
// TEST 4: Maximum Call Depth Enforcement
// ============================================================================

#[test]
fn test_max_call_depth_enforced() {
    let mut ctx = ExecutionContext::default();
    ctx.max_depth = 5; // Set max depth to 5 for testing

    // Should allow exactly max_depth calls
    for i in 0..5 {
        let contract = [i as u8; 32];
        assert!(ctx.enter_call(contract).is_ok());
    }
    assert_eq!(ctx.call_depth(), 5);

    // 6th call should fail - max depth exceeded
    let contract_6 = [6u8; 32];
    let result = ctx.enter_call(contract_6);
    assert_eq!(result, Err(ExecutionError::MaxCallDepthExceeded));

    // Cleanup
    for i in (0..5).rev() {
        let contract = [i as u8; 32];
        ctx.exit_call(&contract);
    }
    assert_eq!(ctx.call_depth(), 0);
}

// ============================================================================
// TEST 5: Legitimate Call Chain Allowed (No Reentrancy)
// ============================================================================

#[test]
fn test_legitimate_call_chain_allowed() {
    let mut ctx = ExecutionContext::default();
    let contracts: Vec<[u8; 32]> = (0..5).map(|i| [i as u8; 32]).collect();

    // Sequential calls with no reentrancy should all succeed
    for (i, contract) in contracts.iter().enumerate() {
        assert!(ctx.enter_call(*contract).is_ok());
        assert_eq!(ctx.call_depth(), (i + 1) as u32);
        assert!(ctx.is_in_call_stack(contract));
    }

    // All contracts should be in the call stack
    for contract in &contracts {
        assert!(ctx.is_in_call_stack(contract));
    }

    // Cleanup in reverse order
    for (i, contract) in contracts.iter().enumerate().rev() {
        ctx.exit_call(contract);
        assert_eq!(ctx.call_depth(), i as u32);
    }
    assert_eq!(ctx.call_depth(), 0);
}

// ============================================================================
// TEST 6: State Lock Prevents Concurrent Modification
// ============================================================================

#[test]
fn test_state_lock_prevents_concurrent_modification() {
    let mut state_lock = StateLock::new();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // Lock contract A (simulating it's making an external call)
    state_lock.lock(&contract_a);
    assert!(state_lock.is_locked(&contract_a));

    // Contract B should not be able to modify A's state while locked
    // (This is enforced at the host function level)
    assert!(state_lock.is_locked(&contract_a));

    // Contract B's state should be independent
    assert!(!state_lock.is_locked(&contract_b));

    // After A's call completes, unlock
    state_lock.unlock(&contract_a);
    assert!(!state_lock.is_locked(&contract_a));
}

// ============================================================================
// TEST 7: Transfer to Locked Account Blocked
// ============================================================================

#[test]
fn test_transfer_to_locked_account_blocked() {
    let ctx = ExecutionContext::default();
    let mut state_lock = StateLock::new();
    let recipient = [1u8; 32];

    // Lock recipient (it's currently executing)
    state_lock.lock(&recipient);

    // Transfer should fail - recipient is locked
    let result = host_transfer(&ctx, &state_lock, recipient, 1000);
    assert_eq!(result, Err(ExecutionError::AccountLocked));

    // Unlock and retry
    state_lock.unlock(&recipient);
    let result = host_transfer(&ctx, &state_lock, recipient, 1000);
    assert!(result.is_ok());
}

// ============================================================================
// TEST 8: Storage Write to Locked Contract Blocked
// ============================================================================

#[test]
fn test_storage_write_to_locked_contract_blocked() {
    let ctx = ExecutionContext::default();
    let mut state_lock = StateLock::new();
    let mut storage = InMemoryStorage::default();

    // Lock the contract (it's making an external call)
    state_lock.lock(&ctx.address);

    // Storage write should fail - contract is locked
    let result = host_sstore(&ctx, &state_lock, &mut storage, H256::zero(), H256::from_low_u64_be(42));
    assert_eq!(result, Err(ExecutionError::AccountLocked));

    // Unlock and retry
    state_lock.unlock(&ctx.address);
    let result = host_sstore(&ctx, &state_lock, &mut storage, H256::zero(), H256::from_low_u64_be(42));
    assert!(result.is_ok());

    // Verify storage was written
    let value = host_sload(&ctx, &storage, H256::zero());
    assert_eq!(value, H256::from_low_u64_be(42));
}

// ============================================================================
// TEST 9: Self-Destruct While Locked Blocked
// ============================================================================

#[test]
fn test_selfdestruct_while_locked_blocked() {
    let ctx = ExecutionContext::default();
    let mut state_lock = StateLock::new();
    let beneficiary = [1u8; 32];

    // Lock the contract
    state_lock.lock(&ctx.address);

    // Self-destruct should fail - contract is locked
    let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
    assert_eq!(result, Err(ExecutionError::AccountLocked));

    // Unlock and retry
    state_lock.unlock(&ctx.address);
    let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
    assert!(result.is_ok());
}

// ============================================================================
// TEST 10: Self-Destruct to Locked Beneficiary Blocked
// ============================================================================

#[test]
fn test_selfdestruct_to_locked_beneficiary_blocked() {
    let ctx = ExecutionContext::default();
    let mut state_lock = StateLock::new();
    let beneficiary = [1u8; 32];

    // Lock the beneficiary
    state_lock.lock(&beneficiary);

    // Self-destruct should fail - beneficiary is locked
    let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
    assert_eq!(result, Err(ExecutionError::AccountLocked));

    // Unlock and retry
    state_lock.unlock(&beneficiary);
    let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
    assert!(result.is_ok());
}

// ============================================================================
// TEST 11: Contract Creation While Locked Blocked
// ============================================================================

#[test]
fn test_create_while_locked_blocked() {
    let ctx = ExecutionContext::default();
    let mut state_lock = StateLock::new();
    let code = vec![0x60, 0x00, 0x60, 0x00, 0xf3]; // Simple bytecode

    // Lock the contract
    state_lock.lock(&ctx.address);

    // Create should fail - contract is locked
    let result = host_create(&ctx, &state_lock, 0, code.clone(), 100000);
    assert_eq!(result, Err(ExecutionError::AccountLocked));

    // Unlock and retry
    state_lock.unlock(&ctx.address);
    let result = host_create(&ctx, &state_lock, 0, code, 100000);
    assert!(result.is_ok());
}

// ============================================================================
// TEST 12: Cleanup on Revert - Call Stack Cleared
// ============================================================================

#[test]
fn test_cleanup_on_revert_call_stack_cleared() {
    let mut ctx = ExecutionContext::default();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // Build call stack
    ctx.enter_call(contract_a).unwrap();
    ctx.enter_call(contract_b).unwrap();
    assert_eq!(ctx.call_depth(), 2);

    // Simulate revert by manually cleaning up
    ctx.exit_call(&contract_b);
    ctx.exit_call(&contract_a);

    // Call stack should be empty
    assert_eq!(ctx.call_depth(), 0);
    assert!(!ctx.is_in_call_stack(&contract_a));
    assert!(!ctx.is_in_call_stack(&contract_b));
}

// ============================================================================
// TEST 13: Cleanup on Revert - State Locks Released
// ============================================================================

#[test]
fn test_cleanup_on_revert_locks_released() {
    let mut state_lock = StateLock::new();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // Lock both contracts
    state_lock.lock(&contract_a);
    state_lock.lock(&contract_b);
    assert_eq!(state_lock.locked_count(), 2);

    // Simulate revert by unlocking
    state_lock.unlock(&contract_b);
    state_lock.unlock(&contract_a);

    // All locks should be released
    assert_eq!(state_lock.locked_count(), 0);
    assert!(!state_lock.is_locked(&contract_a));
    assert!(!state_lock.is_locked(&contract_b));
}

// ============================================================================
// TEST 14: Nested Locking (Same Contract Multiple Times)
// ============================================================================

#[test]
fn test_nested_locking_same_contract() {
    let mut state_lock = StateLock::new();
    let contract = [1u8; 32];

    // Lock multiple times (simulating nested calls with same contract)
    state_lock.lock(&contract);
    state_lock.lock(&contract);
    state_lock.lock(&contract);
    assert_eq!(state_lock.lock_count(&contract), 3);
    assert!(state_lock.is_locked(&contract));

    // Unlock once - still locked
    state_lock.unlock(&contract);
    assert_eq!(state_lock.lock_count(&contract), 2);
    assert!(state_lock.is_locked(&contract));

    // Unlock all
    state_lock.unlock(&contract);
    state_lock.unlock(&contract);
    assert_eq!(state_lock.lock_count(&contract), 0);
    assert!(!state_lock.is_locked(&contract));
}

// ============================================================================
// TEST 15: Multiple Independent Call Chains
// ============================================================================

#[test]
fn test_multiple_independent_call_chains() {
    let mut ctx1 = ExecutionContext::default();
    let mut ctx2 = ExecutionContext::default();

    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];
    let contract_c = [3u8; 32];
    let contract_d = [4u8; 32];

    // Chain 1: A -> B
    ctx1.enter_call(contract_a).unwrap();
    ctx1.enter_call(contract_b).unwrap();
    assert_eq!(ctx1.call_depth(), 2);

    // Chain 2: C -> D (independent)
    ctx2.enter_call(contract_c).unwrap();
    ctx2.enter_call(contract_d).unwrap();
    assert_eq!(ctx2.call_depth(), 2);

    // Both chains should be independent
    assert!(ctx1.is_in_call_stack(&contract_a));
    assert!(ctx1.is_in_call_stack(&contract_b));
    assert!(!ctx1.is_in_call_stack(&contract_c));
    assert!(!ctx1.is_in_call_stack(&contract_d));

    assert!(ctx2.is_in_call_stack(&contract_c));
    assert!(ctx2.is_in_call_stack(&contract_d));
    assert!(!ctx2.is_in_call_stack(&contract_a));
    assert!(!ctx2.is_in_call_stack(&contract_b));
}

// ============================================================================
// TEST 16: Call After Cleanup (Ensure State Reset)
// ============================================================================

#[test]
fn test_call_after_cleanup() {
    let mut ctx = ExecutionContext::default();
    let contract = [1u8; 32];

    // First call
    ctx.enter_call(contract).unwrap();
    ctx.exit_call(&contract);
    assert_eq!(ctx.call_depth(), 0);

    // Should be able to call same contract again
    let result = ctx.enter_call(contract);
    assert!(result.is_ok());
    assert_eq!(ctx.call_depth(), 1);

    // Cleanup
    ctx.exit_call(&contract);
}

// ============================================================================
// TEST 17: Reentrancy Detection with Default Max Depth
// ============================================================================

#[test]
fn test_default_max_depth_is_10() {
    let ctx = ExecutionContext::default();
    assert_eq!(ctx.max_depth, 10);

    let mut ctx = ExecutionContext::default();

    // Should allow 10 calls
    for i in 0..10 {
        let contract = [i as u8; 32];
        assert!(ctx.enter_call(contract).is_ok());
    }
    assert_eq!(ctx.call_depth(), 10);

    // 11th call should fail
    let contract_11 = [11u8; 32];
    let result = ctx.enter_call(contract_11);
    assert_eq!(result, Err(ExecutionError::MaxCallDepthExceeded));
}

// ============================================================================
// TEST 18: State Lock Clear Function
// ============================================================================

#[test]
fn test_state_lock_clear() {
    let mut state_lock = StateLock::new();
    let contracts: Vec<[u8; 32]> = (0..5).map(|i| [i as u8; 32]).collect();

    // Lock multiple contracts
    for contract in &contracts {
        state_lock.lock(contract);
    }
    assert_eq!(state_lock.locked_count(), 5);

    // Clear all locks
    state_lock.clear();
    assert_eq!(state_lock.locked_count(), 0);

    // All should be unlocked
    for contract in &contracts {
        assert!(!state_lock.is_locked(contract));
    }
}

// ============================================================================
// TEST 19: Read-Only Operations Don't Require Unlocked State
// ============================================================================

#[test]
fn test_read_operations_with_locked_state() {
    let ctx = ExecutionContext::default();
    let state_lock = StateLock::new();
    let storage = InMemoryStorage::default();

    // Lock the contract
    // Note: We're using a separate state_lock instance to test read operations
    let mut locked_state = StateLock::new();
    locked_state.lock(&ctx.address);

    // Read operations should still work (they don't check locks)
    let value = host_sload(&ctx, &storage, H256::zero());
    assert_eq!(value, H256::zero());

    let balance = host_balance([1u8; 32]);
    assert_eq!(balance, 0); // Placeholder returns 0
}

// ============================================================================
// TEST 20: Parallel Locks Don't Interfere
// ============================================================================

#[test]
fn test_parallel_locks_independent() {
    let mut state_lock = StateLock::new();
    let contract_a = [1u8; 32];
    let contract_b = [2u8; 32];

    // Lock A
    state_lock.lock(&contract_a);
    assert!(state_lock.is_locked(&contract_a));
    assert!(!state_lock.is_locked(&contract_b));

    // Lock B
    state_lock.lock(&contract_b);
    assert!(state_lock.is_locked(&contract_a));
    assert!(state_lock.is_locked(&contract_b));

    // Unlock A - B should remain locked
    state_lock.unlock(&contract_a);
    assert!(!state_lock.is_locked(&contract_a));
    assert!(state_lock.is_locked(&contract_b));

    // Unlock B
    state_lock.unlock(&contract_b);
    assert!(!state_lock.is_locked(&contract_b));
}
