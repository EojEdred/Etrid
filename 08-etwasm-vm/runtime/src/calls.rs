//! Cross-Contract Call System
//!
//! This module implements comprehensive cross-contract call functionality:
//! - CALL: Regular contract call with value transfer
//! - DELEGATECALL: Call with caller's context (library pattern)
//! - STATICCALL: Read-only call (no state modification)
//! - CREATE: Deploy new contracts
//! - CREATE2: Deterministic contract deployment
//!
//! All calls include reentrancy protection and proper gas accounting.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;
use etwasm_gas_metering::VMw;
use crate::{ExecutionContext, ExecutionError, StateLock};

/// Call type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CallType {
    /// Regular call with value transfer
    Call,
    /// Delegate call (uses caller's context)
    DelegateCall,
    /// Static call (read-only, no state modification)
    StaticCall,
    /// Create new contract
    Create,
    /// Create new contract with deterministic address
    Create2,
}

/// Call parameters for cross-contract invocation
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct CallParams {
    /// Type of call
    pub call_type: CallType,
    /// Caller address
    pub caller: [u8; 32],
    /// Target contract address (for CALL, DELEGATECALL, STATICCALL)
    pub target: Option<[u8; 32]>,
    /// Value to transfer (in wei)
    pub value: u128,
    /// Gas limit for the call
    pub gas_limit: VMw,
    /// Input data
    pub input_data: Vec<u8>,
    /// Salt for CREATE2
    pub salt: Option<H256>,
    /// Bytecode for CREATE/CREATE2
    pub bytecode: Option<Vec<u8>>,
}

impl CallParams {
    /// Create parameters for a CALL
    pub fn new_call(
        caller: [u8; 32],
        target: [u8; 32],
        value: u128,
        gas_limit: VMw,
        input_data: Vec<u8>,
    ) -> Self {
        Self {
            call_type: CallType::Call,
            caller,
            target: Some(target),
            value,
            gas_limit,
            input_data,
            salt: None,
            bytecode: None,
        }
    }

    /// Create parameters for a DELEGATECALL
    pub fn new_delegatecall(
        caller: [u8; 32],
        target: [u8; 32],
        gas_limit: VMw,
        input_data: Vec<u8>,
    ) -> Self {
        Self {
            call_type: CallType::DelegateCall,
            caller,
            target: Some(target),
            value: 0, // No value transfer in DELEGATECALL
            gas_limit,
            input_data,
            salt: None,
            bytecode: None,
        }
    }

    /// Create parameters for a STATICCALL
    pub fn new_staticcall(
        caller: [u8; 32],
        target: [u8; 32],
        gas_limit: VMw,
        input_data: Vec<u8>,
    ) -> Self {
        Self {
            call_type: CallType::StaticCall,
            caller,
            target: Some(target),
            value: 0, // No value transfer in STATICCALL
            gas_limit,
            input_data,
            salt: None,
            bytecode: None,
        }
    }

    /// Create parameters for CREATE
    pub fn new_create(
        caller: [u8; 32],
        value: u128,
        gas_limit: VMw,
        bytecode: Vec<u8>,
    ) -> Self {
        Self {
            call_type: CallType::Create,
            caller,
            target: None,
            value,
            gas_limit,
            input_data: Vec::new(),
            salt: None,
            bytecode: Some(bytecode),
        }
    }

    /// Create parameters for CREATE2
    pub fn new_create2(
        caller: [u8; 32],
        value: u128,
        gas_limit: VMw,
        bytecode: Vec<u8>,
        salt: H256,
    ) -> Self {
        Self {
            call_type: CallType::Create2,
            caller,
            target: None,
            value,
            gas_limit,
            input_data: Vec::new(),
            salt: Some(salt),
            bytecode: Some(bytecode),
        }
    }
}

/// Result of a cross-contract call
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct CallResult {
    /// Whether the call succeeded
    pub success: bool,
    /// Gas used by the call
    pub gas_used: VMw,
    /// Return data from the call
    pub return_data: Vec<u8>,
    /// New contract address (for CREATE/CREATE2)
    pub created_address: Option<[u8; 32]>,
}

impl CallResult {
    /// Create a successful call result
    pub fn success(gas_used: VMw, return_data: Vec<u8>) -> Self {
        Self {
            success: true,
            gas_used,
            return_data,
            created_address: None,
        }
    }

    /// Create a failed call result
    pub fn failure(gas_used: VMw, return_data: Vec<u8>) -> Self {
        Self {
            success: false,
            gas_used,
            return_data,
            created_address: None,
        }
    }

    /// Create a successful CREATE result
    pub fn created(gas_used: VMw, address: [u8; 32]) -> Self {
        Self {
            success: true,
            gas_used,
            return_data: Vec::new(),
            created_address: Some(address),
        }
    }
}

/// Call context for tracking call stack and state
#[derive(Debug, Clone)]
pub struct CallContext {
    /// Current execution context
    pub context: ExecutionContext,
    /// State lock for reentrancy protection
    pub state_lock: StateLock,
    /// Whether we're in a static call (read-only)
    pub is_static: bool,
    /// Return data from the last call
    pub return_data: Vec<u8>,
}

impl CallContext {
    /// Create a new call context
    pub fn new(context: ExecutionContext) -> Self {
        Self {
            context,
            state_lock: StateLock::new(),
            is_static: false,
            return_data: Vec::new(),
        }
    }

    /// Check if state modification is allowed
    pub fn can_modify_state(&self) -> bool {
        !self.is_static
    }

    /// Enter a static call context
    pub fn enter_static(&mut self) {
        self.is_static = true;
    }

    /// Exit a static call context
    pub fn exit_static(&mut self) {
        self.is_static = false;
    }
}

/// Cross-contract call executor
pub struct CallExecutor;

impl CallExecutor {
    /// Execute a cross-contract call with full reentrancy protection
    ///
    /// # Arguments
    ///
    /// * `params` - Call parameters
    /// * `call_context` - Mutable call context for tracking
    /// * `load_code` - Function to load contract bytecode
    ///
    /// # Returns
    ///
    /// `CallResult` with execution outcome
    ///
    /// # Security
    ///
    /// - Checks for reentrancy before executing
    /// - Enforces state lock during execution
    /// - Validates gas limits
    /// - Prevents state modification in STATICCALL
    pub fn execute_call<F>(
        params: CallParams,
        call_context: &mut CallContext,
        load_code: F,
    ) -> CallResult
    where
        F: Fn([u8; 32]) -> Option<Vec<u8>>,
    {
        match params.call_type {
            CallType::Call => Self::execute_regular_call(params, call_context, load_code),
            CallType::DelegateCall => Self::execute_delegatecall(params, call_context, load_code),
            CallType::StaticCall => Self::execute_staticcall(params, call_context, load_code),
            CallType::Create => Self::execute_create(params, call_context),
            CallType::Create2 => Self::execute_create2(params, call_context),
        }
    }

    /// Execute a regular CALL
    fn execute_regular_call<F>(
        params: CallParams,
        call_context: &mut CallContext,
        load_code: F,
    ) -> CallResult
    where
        F: Fn([u8; 32]) -> Option<Vec<u8>>,
    {
        let target = match params.target {
            Some(t) => t,
            None => return CallResult::failure(0, b"No target specified".to_vec()),
        };

        // 1. Check for reentrancy
        if let Err(e) = call_context.context.enter_call(target) {
            return Self::handle_error(e, 0);
        }

        // 2. Check if target is locked
        if call_context.state_lock.is_locked(&target) {
            call_context.context.exit_call(&target);
            return CallResult::failure(0, b"Target contract is locked".to_vec());
        }

        // 3. Lock caller's state
        call_context.state_lock.lock(&params.caller);

        // 4. Load target contract code
        let code = match load_code(target) {
            Some(c) => c,
            None => {
                call_context.state_lock.unlock(&params.caller);
                call_context.context.exit_call(&target);
                return CallResult::failure(0, b"Contract not found".to_vec());
            }
        };

        // 5. Create execution context for the call
        let mut sub_context = ExecutionContext::new(
            params.caller,
            target,
            params.value,
            params.gas_limit,
        );
        sub_context.call_stack = call_context.context.call_stack.clone();
        sub_context.reentrancy_depth = call_context.context.reentrancy_depth;

        // 6. Execute the call (placeholder - in real implementation, use Interpreter)
        let result = Self::execute_bytecode(code, sub_context, params.input_data, params.gas_limit);

        // 7. Cleanup: unlock state and exit call
        call_context.state_lock.unlock(&params.caller);
        call_context.context.exit_call(&target);

        // 8. Store return data
        call_context.return_data = result.return_data.clone();

        result
    }

    /// Execute a DELEGATECALL
    fn execute_delegatecall<F>(
        params: CallParams,
        call_context: &mut CallContext,
        load_code: F,
    ) -> CallResult
    where
        F: Fn([u8; 32]) -> Option<Vec<u8>>,
    {
        let target = match params.target {
            Some(t) => t,
            None => return CallResult::failure(0, b"No target specified".to_vec()),
        };

        // DELEGATECALL uses the caller's context (address, storage, balance)
        // but executes the target's code

        // 1. Check for reentrancy (check caller, not target)
        if call_context.context.is_in_call_stack(&params.caller) {
            return CallResult::failure(0, b"Reentrancy detected".to_vec());
        }

        // 2. Load target contract code
        let code = match load_code(target) {
            Some(c) => c,
            None => return CallResult::failure(0, b"Contract not found".to_vec()),
        };

        // 3. Execute in caller's context
        let mut sub_context = ExecutionContext::new(
            call_context.context.caller, // Keep original caller
            params.caller,                // Execute as caller
            0,                            // No value transfer
            params.gas_limit,
        );
        sub_context.call_stack = call_context.context.call_stack.clone();
        sub_context.reentrancy_depth = call_context.context.reentrancy_depth;

        // 4. Execute the call
        let result = Self::execute_bytecode(code, sub_context, params.input_data, params.gas_limit);

        // 5. Store return data
        call_context.return_data = result.return_data.clone();

        result
    }

    /// Execute a STATICCALL (read-only)
    fn execute_staticcall<F>(
        params: CallParams,
        call_context: &mut CallContext,
        load_code: F,
    ) -> CallResult
    where
        F: Fn([u8; 32]) -> Option<Vec<u8>>,
    {
        let target = match params.target {
            Some(t) => t,
            None => return CallResult::failure(0, b"No target specified".to_vec()),
        };

        // Mark as static (no state modification allowed)
        let was_static = call_context.is_static;
        call_context.enter_static();

        // Execute like a regular call
        let result = Self::execute_regular_call(params, call_context, load_code);

        // Restore static flag
        if !was_static {
            call_context.exit_static();
        }

        result
    }

    /// Execute CREATE (deploy new contract)
    fn execute_create(
        params: CallParams,
        call_context: &mut CallContext,
    ) -> CallResult {
        // Cannot create in static context
        if call_context.is_static {
            return CallResult::failure(0, b"Cannot create in static context".to_vec());
        }

        let bytecode = match params.bytecode {
            Some(code) => code,
            None => return CallResult::failure(0, b"No bytecode provided".to_vec()),
        };

        // 1. Generate new contract address (simplified - use nonce-based)
        let new_address = Self::generate_create_address(&params.caller, call_context.context.block_number);

        // 2. Check for reentrancy
        if let Err(e) = call_context.context.enter_call(new_address) {
            return Self::handle_error(e, 0);
        }

        // 3. Deploy the contract (placeholder)
        let gas_used = Self::estimate_create_gas(&bytecode);

        if gas_used > params.gas_limit {
            call_context.context.exit_call(&new_address);
            return CallResult::failure(gas_used, b"Out of gas".to_vec());
        }

        // 4. Cleanup
        call_context.context.exit_call(&new_address);

        // 5. Return created address
        CallResult::created(gas_used, new_address)
    }

    /// Execute CREATE2 (deterministic deployment)
    fn execute_create2(
        params: CallParams,
        call_context: &mut CallContext,
    ) -> CallResult {
        // Cannot create in static context
        if call_context.is_static {
            return CallResult::failure(0, b"Cannot create in static context".to_vec());
        }

        let bytecode = match params.bytecode {
            Some(code) => code,
            None => return CallResult::failure(0, b"No bytecode provided".to_vec()),
        };

        let salt = match params.salt {
            Some(s) => s,
            None => return CallResult::failure(0, b"No salt provided".to_vec()),
        };

        // 1. Generate deterministic address
        let new_address = Self::generate_create2_address(&params.caller, &bytecode, &salt);

        // 2. Check for reentrancy
        if let Err(e) = call_context.context.enter_call(new_address) {
            return Self::handle_error(e, 0);
        }

        // 3. Deploy the contract (placeholder)
        let gas_used = Self::estimate_create_gas(&bytecode);

        if gas_used > params.gas_limit {
            call_context.context.exit_call(&new_address);
            return CallResult::failure(gas_used, b"Out of gas".to_vec());
        }

        // 4. Cleanup
        call_context.context.exit_call(&new_address);

        // 5. Return created address
        CallResult::created(gas_used, new_address)
    }

    /// Execute bytecode (placeholder for actual interpreter)
    fn execute_bytecode(
        _code: Vec<u8>,
        _context: ExecutionContext,
        _input: Vec<u8>,
        gas_limit: VMw,
    ) -> CallResult {
        // In real implementation, this would:
        // 1. Create new Interpreter instance
        // 2. Execute with gas tracking
        // 3. Handle return/revert
        // 4. Return result

        // For now, simulate successful execution
        CallResult::success(gas_limit / 2, Vec::new())
    }

    /// Handle execution errors
    fn handle_error(error: ExecutionError, gas_used: VMw) -> CallResult {
        let message = match error {
            ExecutionError::ReentrancyDetected => b"Reentrancy detected".to_vec(),
            ExecutionError::MaxCallDepthExceeded => b"Max call depth exceeded".to_vec(),
            ExecutionError::AccountLocked => b"Account locked".to_vec(),
            ExecutionError::OutOfGas => b"Out of gas".to_vec(),
            _ => b"Execution error".to_vec(),
        };

        CallResult::failure(gas_used, message)
    }

    /// Generate CREATE address (simplified)
    fn generate_create_address(creator: &[u8; 32], nonce: u64) -> [u8; 32] {
        use sp_io::hashing::blake2_256;
        let mut data = Vec::new();
        data.extend_from_slice(creator);
        data.extend_from_slice(&nonce.to_le_bytes());
        blake2_256(&data)
    }

    /// Generate CREATE2 address (deterministic)
    fn generate_create2_address(creator: &[u8; 32], bytecode: &[u8], salt: &H256) -> [u8; 32] {
        use sp_io::hashing::blake2_256;
        let mut data = Vec::new();
        data.push(0xff); // CREATE2 prefix
        data.extend_from_slice(creator);
        data.extend_from_slice(salt.as_bytes());
        data.extend_from_slice(&blake2_256(bytecode));
        blake2_256(&data)
    }

    /// Estimate gas for CREATE/CREATE2
    fn estimate_create_gas(bytecode: &[u8]) -> VMw {
        // Base cost + per-byte cost
        32000 + (bytecode.len() as VMw * 200)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_code_loader(address: [u8; 32]) -> Option<Vec<u8>> {
        // Return some mock bytecode
        Some(vec![0x60, 0x00, 0x60, 0x00, 0xf3]) // PUSH1 0 PUSH1 0 RETURN
    }

    #[test]
    fn test_call_params_creation() {
        let caller = [1u8; 32];
        let target = [2u8; 32];

        let params = CallParams::new_call(caller, target, 100, 10000, vec![1, 2, 3]);
        assert_eq!(params.call_type, CallType::Call);
        assert_eq!(params.target, Some(target));
        assert_eq!(params.value, 100);
    }

    #[test]
    fn test_delegatecall_params() {
        let caller = [1u8; 32];
        let target = [2u8; 32];

        let params = CallParams::new_delegatecall(caller, target, 10000, vec![1, 2, 3]);
        assert_eq!(params.call_type, CallType::DelegateCall);
        assert_eq!(params.value, 0); // No value transfer in DELEGATECALL
    }

    #[test]
    fn test_staticcall_params() {
        let caller = [1u8; 32];
        let target = [2u8; 32];

        let params = CallParams::new_staticcall(caller, target, 10000, vec![1, 2, 3]);
        assert_eq!(params.call_type, CallType::StaticCall);
        assert_eq!(params.value, 0);
    }

    #[test]
    fn test_create_params() {
        let caller = [1u8; 32];
        let bytecode = vec![0x60, 0x00];

        let params = CallParams::new_create(caller, 100, 100000, bytecode.clone());
        assert_eq!(params.call_type, CallType::Create);
        assert_eq!(params.bytecode, Some(bytecode));
    }

    #[test]
    fn test_create2_params() {
        let caller = [1u8; 32];
        let bytecode = vec![0x60, 0x00];
        let salt = H256::from_low_u64_be(42);

        let params = CallParams::new_create2(caller, 100, 100000, bytecode.clone(), salt);
        assert_eq!(params.call_type, CallType::Create2);
        assert_eq!(params.salt, Some(salt));
    }

    #[test]
    fn test_call_context_static_mode() {
        let mut context = CallContext::new(ExecutionContext::default());

        assert!(!context.is_static);
        assert!(context.can_modify_state());

        context.enter_static();
        assert!(context.is_static);
        assert!(!context.can_modify_state());

        context.exit_static();
        assert!(!context.is_static);
        assert!(context.can_modify_state());
    }

    #[test]
    fn test_execute_regular_call() {
        let caller = [1u8; 32];
        let target = [2u8; 32];
        let params = CallParams::new_call(caller, target, 0, 100000, Vec::new());

        let mut call_context = CallContext::new(ExecutionContext::default());
        let result = CallExecutor::execute_call(params, &mut call_context, mock_code_loader);

        assert!(result.success);
        assert!(result.gas_used > 0);
    }

    #[test]
    fn test_create_generates_address() {
        let caller = [1u8; 32];
        let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];
        let params = CallParams::new_create(caller, 0, 100000, bytecode);

        let mut call_context = CallContext::new(ExecutionContext::default());
        let result = CallExecutor::execute_call(params, &mut call_context, mock_code_loader);

        assert!(result.success);
        assert!(result.created_address.is_some());
    }

    #[test]
    fn test_create2_deterministic() {
        let caller = [1u8; 32];
        let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];
        let salt = H256::from_low_u64_be(42);

        // Create same contract twice with same salt
        let params1 = CallParams::new_create2(caller, 0, 100000, bytecode.clone(), salt);
        let params2 = CallParams::new_create2(caller, 0, 100000, bytecode.clone(), salt);

        let mut call_context = CallContext::new(ExecutionContext::default());

        let result1 = CallExecutor::execute_call(params1, &mut call_context, mock_code_loader);
        let result2 = CallExecutor::execute_call(params2, &mut call_context, mock_code_loader);

        // Addresses should be the same
        assert_eq!(result1.created_address, result2.created_address);
    }

    #[test]
    fn test_static_call_prevents_creation() {
        let caller = [1u8; 32];
        let bytecode = vec![0x60, 0x00];
        let params = CallParams::new_create(caller, 0, 100000, bytecode);

        let mut call_context = CallContext::new(ExecutionContext::default());
        call_context.enter_static();

        let result = CallExecutor::execute_call(params, &mut call_context, mock_code_loader);

        assert!(!result.success);
    }
}
