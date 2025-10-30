//! ETWasm Runtime - WASM Execution Environment
//!
//! This module provides the WebAssembly execution runtime for ETWasm VM,
//! enabling execution of EVM-compatible smart contracts on Ëtrid.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;
use sp_std::collections::btree_set::BTreeSet;
use etwasm_gas_metering::VMw;
use etwasm_opcodes::*;

pub mod state_lock;
pub mod host_functions;
pub mod storage;
pub mod calls;
pub mod events;
pub mod lifecycle;

pub use state_lock::StateLock;
pub use host_functions::*;
pub use storage::*;
pub use calls::*;
pub use events::*;
pub use lifecycle::*;

/// Maximum stack depth for WASM execution
pub const MAX_STACK_DEPTH: usize = 1024;

/// Maximum memory pages (1 page = 64KB)
pub const MAX_MEMORY_PAGES: u32 = 256; // 16MB max

/// EVM word size (256 bits = 32 bytes)
pub const EVM_WORD_SIZE: usize = 32;

/// ============================================================================
/// EXECUTION CONTEXT
/// ============================================================================

/// Execution context for smart contract calls with reentrancy protection
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Caller address
    pub caller: [u8; 32],
    /// Contract address being executed
    pub address: [u8; 32],
    /// Value sent with the call (in VMw units)
    pub value: u128,
    /// Gas limit for this execution
    pub gas_limit: VMw,
    /// Gas price
    pub gas_price: u32,
    /// Block number
    pub block_number: u64,
    /// Block timestamp
    pub timestamp: u64,
    /// Chain ID
    pub chain_id: u64,
    /// Call stack for reentrancy detection (tracks active contract calls)
    pub call_stack: BTreeSet<[u8; 32]>,
    /// Current call depth
    pub reentrancy_depth: u32,
    /// Maximum allowed call depth
    pub max_depth: u32,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            caller: [0u8; 32],
            address: [0u8; 32],
            value: 0,
            gas_limit: 1_000_000,
            gas_price: 1,
            block_number: 0,
            timestamp: 0,
            chain_id: 2, // Ëtrid chain ID
            call_stack: BTreeSet::new(),
            reentrancy_depth: 0,
            max_depth: 10, // Default maximum call depth
        }
    }
}

impl ExecutionContext {
    /// Create a new execution context with specified parameters
    pub fn new(
        caller: [u8; 32],
        address: [u8; 32],
        value: u128,
        gas_limit: VMw,
    ) -> Self {
        Self {
            caller,
            address,
            value,
            gas_limit,
            gas_price: 1,
            block_number: 0,
            timestamp: 0,
            chain_id: 2,
            call_stack: BTreeSet::new(),
            reentrancy_depth: 0,
            max_depth: 10,
        }
    }

    /// Enter a call - check for reentrancy and update call stack
    ///
    /// # Arguments
    ///
    /// * `target` - The contract address being called
    ///
    /// # Returns
    ///
    /// `Ok(())` if the call is allowed, `Err(ExecutionError)` if reentrancy is detected
    /// or max depth is exceeded
    ///
    /// # Errors
    ///
    /// * `ExecutionError::ReentrancyDetected` - If target is already in the call stack
    /// * `ExecutionError::MaxCallDepthExceeded` - If max call depth would be exceeded
    pub fn enter_call(&mut self, target: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if target is already in call stack (direct reentrancy)
        if self.call_stack.contains(&target) {
            return Err(ExecutionError::ReentrancyDetected);
        }

        // Check max depth
        if self.reentrancy_depth >= self.max_depth {
            return Err(ExecutionError::MaxCallDepthExceeded);
        }

        self.call_stack.insert(target);
        self.reentrancy_depth += 1;
        Ok(())
    }

    /// Exit a call - remove from call stack
    ///
    /// # Arguments
    ///
    /// * `target` - The contract address to remove from the call stack
    pub fn exit_call(&mut self, target: &[u8; 32]) {
        self.call_stack.remove(target);
        self.reentrancy_depth = self.reentrancy_depth.saturating_sub(1);
    }

    /// Check if a contract is currently in the call stack
    ///
    /// # Arguments
    ///
    /// * `target` - The contract address to check
    ///
    /// # Returns
    ///
    /// `true` if the contract is in the call stack, `false` otherwise
    pub fn is_in_call_stack(&self, target: &[u8; 32]) -> bool {
        self.call_stack.contains(target)
    }

    /// Get the current call depth
    pub fn call_depth(&self) -> u32 {
        self.reentrancy_depth
    }
}

/// ============================================================================
/// EXECUTION RESULT
/// ============================================================================

/// Result of contract execution
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum ExecutionResult {
    /// Successful execution with return data
    Success {
        gas_used: VMw,
        return_data: Vec<u8>,
    },
    /// Execution reverted
    Revert {
        gas_used: VMw,
        reason: Vec<u8>,
    },
    /// Out of gas
    OutOfGas {
        gas_used: VMw,
    },
    /// Stack overflow/underflow
    StackError,
    /// Invalid opcode
    InvalidOpcode(u8),
    /// Invalid jump destination
    InvalidJump,
    /// Reentrancy detected - contract tried to call itself
    ReentrancyDetected,
    /// Maximum call depth exceeded
    MaxCallDepthExceeded,
    /// Account is locked (state modification blocked during execution)
    AccountLocked,
    /// Other execution error
    Error(Vec<u8>),
}

/// Execution errors for internal use
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError {
    /// Reentrancy detected
    ReentrancyDetected,
    /// Maximum call depth exceeded
    MaxCallDepthExceeded,
    /// Account is locked
    AccountLocked,
    /// Out of gas
    OutOfGas,
    /// Stack error
    StackError,
    /// Invalid opcode
    InvalidOpcode(u8),
    /// Invalid jump
    InvalidJump,
    /// Generic error
    Error(&'static str),
}

impl ExecutionResult {
    pub fn is_success(&self) -> bool {
        matches!(self, ExecutionResult::Success { .. })
    }

    pub fn gas_used(&self) -> VMw {
        match self {
            ExecutionResult::Success { gas_used, .. } => *gas_used,
            ExecutionResult::Revert { gas_used, .. } => *gas_used,
            ExecutionResult::OutOfGas { gas_used } => *gas_used,
            _ => 0,
        }
    }
}

/// ============================================================================
/// STACK IMPLEMENTATION
/// ============================================================================

/// EVM-compatible stack (256-bit words)
#[derive(Debug, Clone)]
pub struct Stack {
    items: Vec<[u8; EVM_WORD_SIZE]>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(MAX_STACK_DEPTH),
        }
    }

    pub fn push(&mut self, value: [u8; EVM_WORD_SIZE]) -> Result<(), &'static str> {
        if self.items.len() >= MAX_STACK_DEPTH {
            return Err("Stack overflow");
        }
        self.items.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<[u8; EVM_WORD_SIZE], &'static str> {
        self.items.pop().ok_or("Stack underflow")
    }

    pub fn peek(&self) -> Result<&[u8; EVM_WORD_SIZE], &'static str> {
        self.items.last().ok_or("Stack underflow")
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn dup(&mut self, position: usize) -> Result<(), &'static str> {
        if position == 0 || position > self.items.len() {
            return Err("Invalid DUP position");
        }
        let idx = self.items.len() - position;
        let value = self.items[idx];
        self.push(value)
    }

    pub fn swap(&mut self, position: usize) -> Result<(), &'static str> {
        if position == 0 || position >= self.items.len() {
            return Err("Invalid SWAP position");
        }
        let len = self.items.len();
        self.items.swap(len - 1, len - 1 - position);
        Ok(())
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// MEMORY IMPLEMENTATION
/// ============================================================================

/// EVM-compatible memory
#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn expand(&mut self, offset: usize, size: usize) -> Result<(), &'static str> {
        let required = offset + size;
        if required > self.data.len() {
            // Check max memory limit (16MB)
            if required > (MAX_MEMORY_PAGES as usize) * 65536 {
                return Err("Memory limit exceeded");
            }
            self.data.resize(required, 0);
        }
        Ok(())
    }

    pub fn store(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        self.expand(offset, data.len())?;
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn load(&self, offset: usize, size: usize) -> Result<Vec<u8>, &'static str> {
        if offset + size > self.data.len() {
            return Err("Memory out of bounds");
        }
        Ok(self.data[offset..offset + size].to_vec())
    }

    pub fn store_byte(&mut self, offset: usize, byte: u8) -> Result<(), &'static str> {
        self.expand(offset, 1)?;
        self.data[offset] = byte;
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// STORAGE INTERFACE
/// ============================================================================

/// Storage interface for contract state
pub trait Storage {
    fn read(&self, key: &H256) -> Option<H256>;
    fn write(&mut self, key: H256, value: H256);
}

/// In-memory storage implementation (for testing)
#[derive(Debug, Clone, Default)]
pub struct InMemoryStorage {
    map: sp_std::collections::btree_map::BTreeMap<H256, H256>,
}

impl Storage for InMemoryStorage {
    fn read(&self, key: &H256) -> Option<H256> {
        self.map.get(key).copied()
    }

    fn write(&mut self, key: H256, value: H256) {
        self.map.insert(key, value);
    }
}

/// ============================================================================
/// INTERPRETER
/// ============================================================================

/// EVM bytecode interpreter with reentrancy protection
pub struct Interpreter<S: Storage> {
    /// Execution context (with call stack tracking)
    pub context: ExecutionContext,
    /// Stack
    pub stack: Stack,
    /// Memory
    pub memory: Memory,
    /// Storage
    pub storage: S,
    /// State lock for reentrancy protection
    pub state_lock: StateLock,
    /// Gas remaining
    pub gas_remaining: VMw,
    /// Program counter
    pub pc: usize,
    /// Bytecode being executed
    pub code: Vec<u8>,
    /// Return data
    pub return_data: Vec<u8>,
}

impl<S: Storage> Interpreter<S> {
    /// Create a new interpreter instance with reentrancy protection
    ///
    /// # Arguments
    ///
    /// * `context` - Execution context with call tracking
    /// * `code` - Contract bytecode to execute
    /// * `storage` - Storage backend for contract state
    ///
    /// # Returns
    ///
    /// A new Interpreter instance ready for execution
    pub fn new(
        context: ExecutionContext,
        code: Vec<u8>,
        storage: S,
    ) -> Self {
        let gas_remaining = context.gas_limit;
        Self {
            context,
            stack: Stack::new(),
            memory: Memory::new(),
            storage,
            state_lock: StateLock::new(),
            gas_remaining,
            pc: 0,
            code,
            return_data: Vec::new(),
        }
    }

    /// Create a new interpreter with explicit state lock (for nested calls)
    ///
    /// # Arguments
    ///
    /// * `context` - Execution context with call tracking
    /// * `code` - Contract bytecode to execute
    /// * `storage` - Storage backend for contract state
    /// * `state_lock` - Shared state lock from parent call
    ///
    /// # Returns
    ///
    /// A new Interpreter instance ready for execution
    pub fn new_with_lock(
        context: ExecutionContext,
        code: Vec<u8>,
        storage: S,
        state_lock: StateLock,
    ) -> Self {
        let gas_remaining = context.gas_limit;
        Self {
            context,
            stack: Stack::new(),
            memory: Memory::new(),
            storage,
            state_lock,
            gas_remaining,
            pc: 0,
            code,
            return_data: Vec::new(),
        }
    }

    /// Execute the bytecode
    pub fn execute(mut self) -> ExecutionResult {
        loop {
            // Check if execution is complete
            if self.pc >= self.code.len() {
                return ExecutionResult::Success {
                    gas_used: self.context.gas_limit - self.gas_remaining,
                    return_data: self.return_data,
                };
            }

            // Fetch opcode
            let opcode = self.code[self.pc];
            self.pc += 1;

            // Get gas cost
            let gas_cost = get_opcode_gas_cost(opcode);

            // Check gas
            if self.gas_remaining < gas_cost {
                return ExecutionResult::OutOfGas {
                    gas_used: self.context.gas_limit,
                };
            }
            self.gas_remaining -= gas_cost;

            // Execute opcode
            match self.execute_opcode(opcode) {
                Ok(OpcodeResult::Continue) => continue,
                Ok(OpcodeResult::Stop) => {
                    return ExecutionResult::Success {
                        gas_used: self.context.gas_limit - self.gas_remaining,
                        return_data: self.return_data,
                    };
                }
                Ok(OpcodeResult::Return(data)) => {
                    return ExecutionResult::Success {
                        gas_used: self.context.gas_limit - self.gas_remaining,
                        return_data: data,
                    };
                }
                Ok(OpcodeResult::Revert(reason)) => {
                    return ExecutionResult::Revert {
                        gas_used: self.context.gas_limit - self.gas_remaining,
                        reason,
                    };
                }
                Err(e) => {
                    return ExecutionResult::Error(e.as_bytes().to_vec());
                }
            }
        }
    }

    fn execute_opcode(&mut self, opcode: u8) -> Result<OpcodeResult, &'static str> {
        match opcode {
            STOP => Ok(OpcodeResult::Stop),

            // Arithmetic
            ADD => self.op_add(),
            MUL => self.op_mul(),
            SUB => self.op_sub(),
            DIV => self.op_div(),
            MOD => self.op_mod(),

            // Comparison
            LT => self.op_lt(),
            GT => self.op_gt(),
            EQ => self.op_eq(),
            ISZERO => self.op_iszero(),

            // Bitwise
            AND => self.op_and(),
            OR => self.op_or(),
            XOR => self.op_xor(),
            NOT => self.op_not(),
            BYTE => self.op_byte(),

            // Stack operations
            POP => self.op_pop(),
            PUSH1..=PUSH32 => self.op_push(opcode),
            DUP1..=DUP16 => self.op_dup(opcode),
            SWAP1..=SWAP16 => self.op_swap(opcode),

            // Memory operations
            MLOAD => self.op_mload(),
            MSTORE => self.op_mstore(),
            MSTORE8 => self.op_mstore8(),

            // Storage operations
            SLOAD => self.op_sload(),
            SSTORE => self.op_sstore(),

            // Control flow
            JUMP => self.op_jump(),
            JUMPI => self.op_jumpi(),
            JUMPDEST => Ok(OpcodeResult::Continue),
            PC => self.op_pc(),

            // Return
            RETURN => self.op_return(),
            REVERT => self.op_revert(),

            // Context
            ADDRESS => self.op_address(),
            CALLER => self.op_caller(),
            CALLVALUE => self.op_callvalue(),
            NUMBER => self.op_number(),
            TIMESTAMP => self.op_timestamp(),
            CHAINID => self.op_chainid(),
            GAS => self.op_gas(),

            _ => Err("Invalid or unsupported opcode"),
        }
    }

    // Arithmetic operations
    fn op_add(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_add(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_mul(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_mul(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_sub(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_sub(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_div(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_div(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_mod(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_mod(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    // Comparison operations
    fn op_lt(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = if u256_lt(&a, &b) { u256_one() } else { u256_zero() };
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_gt(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = if u256_gt(&a, &b) { u256_one() } else { u256_zero() };
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_eq(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = if a == b { u256_one() } else { u256_zero() };
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_iszero(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let result = if u256_is_zero(&a) { u256_one() } else { u256_zero() };
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    // Bitwise operations
    fn op_and(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_and(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_or(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_or(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_xor(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = u256_xor(&a, &b);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_not(&mut self) -> Result<OpcodeResult, &'static str> {
        let a = self.stack.pop()?;
        let result = u256_not(&a);
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_byte(&mut self) -> Result<OpcodeResult, &'static str> {
        let i = self.stack.pop()?;
        let x = self.stack.pop()?;
        let idx = u256_to_u32(&i) as usize;
        let result = if idx < 32 {
            let mut r = u256_zero();
            r[31] = x[idx];
            r
        } else {
            u256_zero()
        };
        self.stack.push(result)?;
        Ok(OpcodeResult::Continue)
    }

    // Stack operations
    fn op_pop(&mut self) -> Result<OpcodeResult, &'static str> {
        self.stack.pop()?;
        Ok(OpcodeResult::Continue)
    }

    fn op_push(&mut self, opcode: u8) -> Result<OpcodeResult, &'static str> {
        let num_bytes = (opcode - PUSH1 + 1) as usize;
        let mut value = u256_zero();
        for i in 0..num_bytes {
            if self.pc < self.code.len() {
                value[32 - num_bytes + i] = self.code[self.pc];
                self.pc += 1;
            }
        }
        self.stack.push(value)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_dup(&mut self, opcode: u8) -> Result<OpcodeResult, &'static str> {
        let position = (opcode - DUP1 + 1) as usize;
        self.stack.dup(position)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_swap(&mut self, opcode: u8) -> Result<OpcodeResult, &'static str> {
        let position = (opcode - SWAP1 + 1) as usize;
        self.stack.swap(position)?;
        Ok(OpcodeResult::Continue)
    }

    // Memory operations
    fn op_mload(&mut self) -> Result<OpcodeResult, &'static str> {
        let offset = u256_to_usize(&self.stack.pop()?);
        let data = self.memory.load(offset, 32)?;
        let mut value = u256_zero();
        value[..data.len()].copy_from_slice(&data);
        self.stack.push(value)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_mstore(&mut self) -> Result<OpcodeResult, &'static str> {
        let offset = u256_to_usize(&self.stack.pop()?);
        let value = self.stack.pop()?;
        self.memory.store(offset, &value)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_mstore8(&mut self) -> Result<OpcodeResult, &'static str> {
        let offset = u256_to_usize(&self.stack.pop()?);
        let value = self.stack.pop()?;
        self.memory.store_byte(offset, value[31])?;
        Ok(OpcodeResult::Continue)
    }

    // Storage operations
    fn op_sload(&mut self) -> Result<OpcodeResult, &'static str> {
        let key_bytes = self.stack.pop()?;
        let key = H256::from_slice(&key_bytes);
        let value = self.storage.read(&key).unwrap_or(H256::zero());
        self.stack.push(value.0)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_sstore(&mut self) -> Result<OpcodeResult, &'static str> {
        let key_bytes = self.stack.pop()?;
        let value_bytes = self.stack.pop()?;
        let key = H256::from_slice(&key_bytes);
        let value = H256::from_slice(&value_bytes);
        self.storage.write(key, value);
        Ok(OpcodeResult::Continue)
    }

    // Control flow
    fn op_jump(&mut self) -> Result<OpcodeResult, &'static str> {
        let dest = u256_to_usize(&self.stack.pop()?);
        if dest >= self.code.len() || self.code[dest] != JUMPDEST {
            return Err("Invalid jump destination");
        }
        self.pc = dest;
        Ok(OpcodeResult::Continue)
    }

    fn op_jumpi(&mut self) -> Result<OpcodeResult, &'static str> {
        let dest = u256_to_usize(&self.stack.pop()?);
        let condition = self.stack.pop()?;
        if !u256_is_zero(&condition) {
            if dest >= self.code.len() || self.code[dest] != JUMPDEST {
                return Err("Invalid jump destination");
            }
            self.pc = dest;
        }
        Ok(OpcodeResult::Continue)
    }

    fn op_pc(&mut self) -> Result<OpcodeResult, &'static str> {
        let pc_value = usize_to_u256(self.pc - 1);
        self.stack.push(pc_value)?;
        Ok(OpcodeResult::Continue)
    }

    // Return operations
    fn op_return(&mut self) -> Result<OpcodeResult, &'static str> {
        let offset = u256_to_usize(&self.stack.pop()?);
        let size = u256_to_usize(&self.stack.pop()?);
        let data = self.memory.load(offset, size)?;
        Ok(OpcodeResult::Return(data))
    }

    fn op_revert(&mut self) -> Result<OpcodeResult, &'static str> {
        let offset = u256_to_usize(&self.stack.pop()?);
        let size = u256_to_usize(&self.stack.pop()?);
        let reason = self.memory.load(offset, size)?;
        Ok(OpcodeResult::Revert(reason))
    }

    // Context operations
    fn op_address(&mut self) -> Result<OpcodeResult, &'static str> {
        self.stack.push(self.context.address)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_caller(&mut self) -> Result<OpcodeResult, &'static str> {
        self.stack.push(self.context.caller)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_callvalue(&mut self) -> Result<OpcodeResult, &'static str> {
        let value = u128_to_u256(self.context.value);
        self.stack.push(value)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_number(&mut self) -> Result<OpcodeResult, &'static str> {
        let number = u64_to_u256(self.context.block_number);
        self.stack.push(number)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_timestamp(&mut self) -> Result<OpcodeResult, &'static str> {
        let timestamp = u64_to_u256(self.context.timestamp);
        self.stack.push(timestamp)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_chainid(&mut self) -> Result<OpcodeResult, &'static str> {
        let chain_id = u64_to_u256(self.context.chain_id);
        self.stack.push(chain_id)?;
        Ok(OpcodeResult::Continue)
    }

    fn op_gas(&mut self) -> Result<OpcodeResult, &'static str> {
        let gas = u64_to_u256(self.gas_remaining);
        self.stack.push(gas)?;
        Ok(OpcodeResult::Continue)
    }
}

/// Opcode execution result
enum OpcodeResult {
    Continue,
    Stop,
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

/// ============================================================================
/// U256 HELPER FUNCTIONS
/// ============================================================================

fn u256_zero() -> [u8; 32] {
    [0u8; 32]
}

fn u256_one() -> [u8; 32] {
    let mut r = [0u8; 32];
    r[31] = 1;
    r
}

fn u256_is_zero(value: &[u8; 32]) -> bool {
    value.iter().all(|&b| b == 0)
}

fn u256_add(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut carry = 0u16;
    for i in (0..32).rev() {
        let sum = a[i] as u16 + b[i] as u16 + carry;
        result[i] = sum as u8;
        carry = sum >> 8;
    }
    result
}

fn u256_sub(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut borrow = 0i16;
    for i in (0..32).rev() {
        let diff = a[i] as i16 - b[i] as i16 - borrow;
        result[i] = diff as u8;
        borrow = if diff < 0 { 1 } else { 0 };
    }
    result
}

fn u256_mul(_a: &[u8; 32], _b: &[u8; 32]) -> [u8; 32] {
    // Simplified multiplication (placeholder)
    let result = [0u8; 32];
    // TODO: Implement proper 256-bit multiplication
    result
}

fn u256_div(_a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    if u256_is_zero(b) {
        return u256_zero();
    }
    // Simplified division (placeholder)
    let result = [0u8; 32];
    // TODO: Implement proper 256-bit division
    result
}

fn u256_mod(_a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    if u256_is_zero(b) {
        return u256_zero();
    }
    // TODO: Implement proper 256-bit modulo
    u256_zero()
}

fn u256_and(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] & b[i];
    }
    result
}

fn u256_or(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] | b[i];
    }
    result
}

fn u256_xor(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = a[i] ^ b[i];
    }
    result
}

fn u256_not(a: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = !a[i];
    }
    result
}

fn u256_lt(a: &[u8; 32], b: &[u8; 32]) -> bool {
    for i in 0..32 {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
    }
    false
}

fn u256_gt(a: &[u8; 32], b: &[u8; 32]) -> bool {
    for i in 0..32 {
        if a[i] > b[i] {
            return true;
        }
        if a[i] < b[i] {
            return false;
        }
    }
    false
}

fn u256_to_usize(value: &[u8; 32]) -> usize {
    let mut result = 0usize;
    for i in 24..32 {
        result = (result << 8) | (value[i] as usize);
    }
    result
}

fn u256_to_u32(value: &[u8; 32]) -> u32 {
    let mut result = 0u32;
    for i in 28..32 {
        result = (result << 8) | (value[i] as u32);
    }
    result
}

fn usize_to_u256(value: usize) -> [u8; 32] {
    let mut result = [0u8; 32];
    let bytes = value.to_be_bytes();
    result[32 - bytes.len()..].copy_from_slice(&bytes);
    result
}

fn u64_to_u256(value: u64) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[24..].copy_from_slice(&value.to_be_bytes());
    result
}

fn u128_to_u256(value: u128) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[16..].copy_from_slice(&value.to_be_bytes());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u256_operations() {
        let a = u256_one();
        let b = u256_one();
        let sum = u256_add(&a, &b);
        assert_eq!(sum[31], 2);

        assert!(u256_is_zero(&u256_zero()));
        assert!(!u256_is_zero(&u256_one()));
    }

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        let value = u256_one();

        stack.push(value).unwrap();
        assert_eq!(stack.len(), 1);

        let popped = stack.pop().unwrap();
        assert_eq!(popped, value);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new();
        let data = vec![1, 2, 3, 4];

        memory.store(0, &data).unwrap();
        let loaded = memory.load(0, 4).unwrap();
        assert_eq!(loaded, data);
    }
}
