//! Host Functions - Reentrancy-Protected External Call Interface
//!
//! This module provides host functions for smart contracts to interact with
//! external contracts and accounts, with comprehensive reentrancy protection.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use crate::{ExecutionContext, ExecutionError, StateLock, Storage};
use sp_core::H256;

/// Host function: Call another contract
///
/// This function enables a contract to call another contract while enforcing
/// the Checks-Effects-Interactions (CEI) pattern through reentrancy protection.
///
/// # Security Guarantees
///
/// 1. Checks: Verifies the target account is not locked
/// 2. Checks: Verifies no reentrancy (target not in call stack)
/// 3. Effects: Locks the calling contract's state
/// 4. Interactions: Executes the external call
/// 5. Cleanup: Unlocks state and exits call
///
/// # Arguments
///
/// * `ctx` - Mutable execution context (tracks call stack)
/// * `state_lock` - State lock manager
/// * `target` - Target contract address
/// * `value` - Value to transfer (in wei)
/// * `gas` - Gas limit for the call
/// * `input` - Input data for the call
///
/// # Returns
///
/// `Ok(return_data)` on success, `Err(ExecutionError)` on failure
///
/// # Errors
///
/// * `ExecutionError::AccountLocked` - Target account is currently executing
/// * `ExecutionError::ReentrancyDetected` - Reentrancy attempt detected
/// * `ExecutionError::MaxCallDepthExceeded` - Maximum call depth reached
/// * `ExecutionError::OutOfGas` - Insufficient gas for the call
pub fn host_call(
    ctx: &mut ExecutionContext,
    state_lock: &mut StateLock,
    target: [u8; 32],
    value: u128,
    gas: u64,
    _input: Vec<u8>,
) -> Result<Vec<u8>, ExecutionError> {
    // 1. CHECKS: Verify target is not locked
    if state_lock.is_locked(&target) {
        return Err(ExecutionError::AccountLocked);
    }

    // 2. CHECKS: Verify no reentrancy - enter call tracking
    ctx.enter_call(target)?;

    // 3. EFFECTS: Lock the calling contract's state
    state_lock.lock(&ctx.address);

    // 4. INTERACTIONS: Execute the call
    // NOTE: In a full implementation, this would dispatch to the interpreter
    // For now, we simulate the call and return success
    let result = simulate_contract_call(target, value, gas);

    // 5. CLEANUP: Always unlock state and exit call (even on error)
    state_lock.unlock(&ctx.address);
    ctx.exit_call(&target);

    result
}

/// Host function: Transfer value to an account
///
/// Transfers value from the current contract to a target account.
/// This function enforces state locking to prevent reentrancy during transfers.
///
/// # Arguments
///
/// * `ctx` - Execution context (immutable for safety)
/// * `state_lock` - State lock manager
/// * `to` - Recipient address
/// * `amount` - Amount to transfer (in wei)
///
/// # Returns
///
/// `Ok(())` on success, `Err(ExecutionError)` on failure
///
/// # Errors
///
/// * `ExecutionError::AccountLocked` - Recipient account is currently executing
/// * `ExecutionError::OutOfGas` - Insufficient gas for the transfer
pub fn host_transfer(
    ctx: &ExecutionContext,
    state_lock: &StateLock,
    to: [u8; 32],
    amount: u128,
) -> Result<(), ExecutionError> {
    // CRITICAL: Prevent transfers to locked accounts (ongoing execution)
    // This prevents an attacker from manipulating balances during execution
    if state_lock.is_locked(&to) {
        return Err(ExecutionError::AccountLocked);
    }

    // Execute transfer
    transfer_balance(&ctx.address, &to, amount)
}

/// Host function: Get balance of an account
///
/// Returns the balance of a specified account. This is a read-only operation
/// that does not require state locking.
///
/// # Arguments
///
/// * `account` - Account address to query
///
/// # Returns
///
/// The account balance in wei
pub fn host_balance(account: [u8; 32]) -> u128 {
    // In a real implementation, this would query the account storage
    // For now, return a placeholder value
    get_account_balance(&account)
}

/// Host function: Get storage value
///
/// Reads a storage value from the current contract's storage.
/// This is a read-only operation.
///
/// # Arguments
///
/// * `ctx` - Execution context
/// * `storage` - Storage backend
/// * `key` - Storage key
///
/// # Returns
///
/// The storage value, or H256::zero() if not found
pub fn host_sload<S: Storage>(
    ctx: &ExecutionContext,
    storage: &S,
    key: H256,
) -> H256 {
    storage.read(&key).unwrap_or(H256::zero())
}

/// Host function: Set storage value
///
/// Writes a storage value to the current contract's storage.
/// State must not be locked to prevent reentrancy attacks.
///
/// # Arguments
///
/// * `ctx` - Execution context
/// * `state_lock` - State lock manager
/// * `storage` - Storage backend
/// * `key` - Storage key
/// * `value` - Storage value
///
/// # Returns
///
/// `Ok(())` on success, `Err(ExecutionError)` if state is locked
///
/// # Errors
///
/// * `ExecutionError::AccountLocked` - Contract state is locked during external call
pub fn host_sstore<S: Storage>(
    ctx: &ExecutionContext,
    state_lock: &StateLock,
    storage: &mut S,
    key: H256,
    value: H256,
) -> Result<(), ExecutionError> {
    // CRITICAL: Prevent storage writes to locked contracts
    // This enforces the "Effects" part of CEI pattern
    if state_lock.is_locked(&ctx.address) {
        return Err(ExecutionError::AccountLocked);
    }

    storage.write(key, value);
    Ok(())
}

/// Host function: Self-destruct the contract
///
/// Destroys the current contract and transfers remaining balance to beneficiary.
/// This is a highly destructive operation that requires careful reentrancy protection.
///
/// # Arguments
///
/// * `ctx` - Execution context
/// * `state_lock` - State lock manager
/// * `beneficiary` - Address to receive remaining balance
///
/// # Returns
///
/// `Ok(())` on success, `Err(ExecutionError)` on failure
///
/// # Errors
///
/// * `ExecutionError::AccountLocked` - Contract or beneficiary is locked
pub fn host_selfdestruct(
    ctx: &ExecutionContext,
    state_lock: &StateLock,
    beneficiary: [u8; 32],
) -> Result<(), ExecutionError> {
    // CRITICAL: Cannot self-destruct while locked (during external call)
    if state_lock.is_locked(&ctx.address) {
        return Err(ExecutionError::AccountLocked);
    }

    // CRITICAL: Cannot send funds to locked account
    if state_lock.is_locked(&beneficiary) {
        return Err(ExecutionError::AccountLocked);
    }

    // Execute self-destruct
    // In real implementation, this would:
    // 1. Transfer remaining balance to beneficiary
    // 2. Mark contract for deletion
    // 3. Clear storage
    Ok(())
}

/// Host function: Create a new contract
///
/// Deploys a new contract with the given bytecode.
///
/// # Arguments
///
/// * `ctx` - Execution context
/// * `state_lock` - State lock manager
/// * `value` - Value to transfer to new contract
/// * `code` - Contract bytecode
/// * `gas` - Gas limit for deployment
///
/// # Returns
///
/// `Ok(new_address)` on success, `Err(ExecutionError)` on failure
///
/// # Errors
///
/// * `ExecutionError::AccountLocked` - Creator contract is locked
/// * `ExecutionError::OutOfGas` - Insufficient gas for deployment
pub fn host_create(
    ctx: &ExecutionContext,
    state_lock: &StateLock,
    value: u128,
    code: Vec<u8>,
    gas: u64,
) -> Result<[u8; 32], ExecutionError> {
    // CRITICAL: Cannot create contracts while locked
    if state_lock.is_locked(&ctx.address) {
        return Err(ExecutionError::AccountLocked);
    }

    // In real implementation, this would:
    // 1. Generate new contract address
    // 2. Deploy code to new address
    // 3. Transfer value to new contract
    // 4. Execute constructor

    // For now, return a placeholder address
    Ok([0u8; 32])
}

// ============================================================================
// HELPER FUNCTIONS (Placeholders for real implementations)
// ============================================================================

/// Simulate a contract call (placeholder)
fn simulate_contract_call(
    _target: [u8; 32],
    _value: u128,
    _gas: u64,
) -> Result<Vec<u8>, ExecutionError> {
    // In a real implementation, this would:
    // 1. Load target contract code
    // 2. Create new interpreter instance
    // 3. Execute with gas limit
    // 4. Return result
    Ok(Vec::new())
}

/// Transfer balance between accounts (placeholder)
fn transfer_balance(
    _from: &[u8; 32],
    _to: &[u8; 32],
    _amount: u128,
) -> Result<(), ExecutionError> {
    // In a real implementation, this would:
    // 1. Check sender has sufficient balance
    // 2. Deduct from sender
    // 3. Add to recipient
    // 4. Emit transfer event
    Ok(())
}

/// Get account balance (placeholder)
fn get_account_balance(_account: &[u8; 32]) -> u128 {
    // In a real implementation, this would query the state
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryStorage;

    #[test]
    fn test_host_call_prevents_reentrancy() {
        let mut ctx = ExecutionContext::default();
        let mut state_lock = StateLock::new();
        let target = [1u8; 32];

        // First call should succeed
        ctx.enter_call(target).unwrap();

        // Second call to same target should fail (reentrancy)
        let result = ctx.enter_call(target);
        assert_eq!(result, Err(ExecutionError::ReentrancyDetected));

        // Cleanup
        ctx.exit_call(&target);
    }

    #[test]
    fn test_host_call_respects_max_depth() {
        let mut ctx = ExecutionContext::default();
        ctx.max_depth = 3;

        // Should allow up to max_depth calls
        assert!(ctx.enter_call([1u8; 32]).is_ok());
        assert!(ctx.enter_call([2u8; 32]).is_ok());
        assert!(ctx.enter_call([3u8; 32]).is_ok());

        // Exceeding max_depth should fail
        let result = ctx.enter_call([4u8; 32]);
        assert_eq!(result, Err(ExecutionError::MaxCallDepthExceeded));
    }

    #[test]
    fn test_host_transfer_blocks_locked_accounts() {
        let ctx = ExecutionContext::default();
        let mut state_lock = StateLock::new();
        let recipient = [1u8; 32];

        // Lock the recipient
        state_lock.lock(&recipient);

        // Transfer should fail
        let result = host_transfer(&ctx, &state_lock, recipient, 100);
        assert_eq!(result, Err(ExecutionError::AccountLocked));

        // Unlock and try again
        state_lock.unlock(&recipient);
        let result = host_transfer(&ctx, &state_lock, recipient, 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_host_sstore_blocks_locked_contracts() {
        let ctx = ExecutionContext::default();
        let mut state_lock = StateLock::new();
        let mut storage = InMemoryStorage::default();

        // Lock the contract
        state_lock.lock(&ctx.address);

        // Storage write should fail
        let result = host_sstore(&ctx, &state_lock, &mut storage, H256::zero(), H256::zero());
        assert_eq!(result, Err(ExecutionError::AccountLocked));

        // Unlock and try again
        state_lock.unlock(&ctx.address);
        let result = host_sstore(&ctx, &state_lock, &mut storage, H256::zero(), H256::zero());
        assert!(result.is_ok());
    }

    #[test]
    fn test_host_selfdestruct_blocks_locked_accounts() {
        let ctx = ExecutionContext::default();
        let mut state_lock = StateLock::new();
        let beneficiary = [1u8; 32];

        // Lock the contract - should fail
        state_lock.lock(&ctx.address);
        let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
        assert_eq!(result, Err(ExecutionError::AccountLocked));
        state_lock.unlock(&ctx.address);

        // Lock the beneficiary - should fail
        state_lock.lock(&beneficiary);
        let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
        assert_eq!(result, Err(ExecutionError::AccountLocked));
        state_lock.unlock(&beneficiary);

        // Both unlocked - should succeed
        let result = host_selfdestruct(&ctx, &state_lock, beneficiary);
        assert!(result.is_ok());
    }

    #[test]
    fn test_host_create_blocks_locked_contracts() {
        let ctx = ExecutionContext::default();
        let mut state_lock = StateLock::new();
        let code = vec![0x60, 0x00, 0x60, 0x00, 0xf3]; // Simple bytecode

        // Lock the contract
        state_lock.lock(&ctx.address);

        // Create should fail
        let result = host_create(&ctx, &state_lock, 0, code.clone(), 100000);
        assert_eq!(result, Err(ExecutionError::AccountLocked));

        // Unlock and try again
        state_lock.unlock(&ctx.address);
        let result = host_create(&ctx, &state_lock, 0, code, 100000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_calls_allowed_without_reentrancy() {
        let mut ctx = ExecutionContext::default();

        // A -> B -> C (no reentrancy)
        let contract_a = [1u8; 32];
        let contract_b = [2u8; 32];
        let contract_c = [3u8; 32];

        assert!(ctx.enter_call(contract_a).is_ok());
        assert_eq!(ctx.call_depth(), 1);

        assert!(ctx.enter_call(contract_b).is_ok());
        assert_eq!(ctx.call_depth(), 2);

        assert!(ctx.enter_call(contract_c).is_ok());
        assert_eq!(ctx.call_depth(), 3);

        // All contracts in stack
        assert!(ctx.is_in_call_stack(&contract_a));
        assert!(ctx.is_in_call_stack(&contract_b));
        assert!(ctx.is_in_call_stack(&contract_c));

        // Cleanup
        ctx.exit_call(&contract_c);
        ctx.exit_call(&contract_b);
        ctx.exit_call(&contract_a);

        assert_eq!(ctx.call_depth(), 0);
    }
}
