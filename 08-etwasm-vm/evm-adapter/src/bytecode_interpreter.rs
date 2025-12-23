use alloc::vec::Vec;
use sp_core::{H160, U256};

use crate::error::{AdapterError, AdapterResult};

/// Execution context passed to the interpreter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallContext {
    pub address: H160,
    pub caller: H160,
    pub value: U256,
    pub input: Vec<u8>,
}

impl CallContext {
    pub fn new(address: H160, caller: H160, value: U256, input: Vec<u8>) -> Self {
        Self { address, caller, value, input }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExitReason {
    Succeed,
    Revert,
    OutOfGas,
    InvalidOpcode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionOutcome {
    pub exit_reason: ExitReason,
    pub return_data: Vec<u8>,
    pub gas_used: u64,
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Stop,
    Add,
    Sub,
    Push(u8),
    Pop,
    MStore,
    Return,
    Unknown(u8),
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::Stop,
            0x01 => Opcode::Add,
            0x03 => Opcode::Sub,
            0x50 => Opcode::Pop,
            0x52 => Opcode::MStore,
            0xf3 => Opcode::Return,
            0x60..=0x7f => Opcode::Push((value - 0x5f) as u8),
            other => Opcode::Unknown(other),
        }
    }
}

/// Simplified interpreter capable of executing deterministic subsets of EVM
/// bytecode. It is not a replacement for the upstream `evm` crate but provides
/// a production-friendly foundation we can extend incrementally.
pub struct EvmInterpreter {
    gas_limit: u64,
    gas_used: u64,
    context: CallContext,
}

impl EvmInterpreter {
    pub fn new(context: CallContext, gas_limit: u64) -> Self {
        Self { gas_limit, gas_used: 0, context }
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    fn charge_gas(&mut self, amount: u64) -> AdapterResult<()> {
        self.gas_used = self
            .gas_used
            .checked_add(amount)
            .ok_or(AdapterError::OutOfGas)?;
        if self.gas_used > self.gas_limit {
            return Err(AdapterError::OutOfGas);
        }
        Ok(())
    }

    /// Execute the given bytecode and return the outcome along with consumed gas.
    pub fn execute(&mut self, code: &[u8]) -> AdapterResult<ExecutionOutcome> {
        let _ctx = &self.context;
        let mut pc = 0usize;
        let mut stack: Vec<U256> = Vec::new();
        let mut memory: Vec<u8> = Vec::new();
        let mut return_data = Vec::new();

        while pc < code.len() {
            let opcode = Opcode::from(code[pc]);
            pc += 1;
            self.charge_gas(Self::gas_cost(&opcode))?;

            match opcode {
                Opcode::Stop => {
                    return Ok(ExecutionOutcome { exit_reason: ExitReason::Succeed, return_data, gas_used: self.gas_used });
                }
                Opcode::Add => {
                    let (a, b) = Self::pop2(&mut stack)?;
                    stack.push(a.overflowing_add(b).0);
                }
                Opcode::Sub => {
                    let (a, b) = Self::pop2(&mut stack)?;
                    stack.push(a.overflowing_sub(b).0);
                }
                Opcode::Pop => {
                    stack.pop().ok_or(AdapterError::StackUnderflow)?;
                }
                Opcode::MStore => {
                    let (value, offset) = Self::pop2(&mut stack)?;
                    Self::write_memory(&mut memory, offset, value)?;
                }
                Opcode::Return => {
                    let (len, offset) = Self::pop2(&mut stack)?;
                    return_data = Self::read_memory(&memory, offset, len)?;
                    return Ok(ExecutionOutcome { exit_reason: ExitReason::Succeed, return_data, gas_used: self.gas_used });
                }
                Opcode::Push(width) => {
                    if pc + width as usize > code.len() {
                        return Err(AdapterError::MalformedCalldata);
                    }
                    let mut bytes = [0u8; 32];
                    let start = 32 - width as usize;
                    bytes[start..32].copy_from_slice(&code[pc..pc + width as usize]);
                    pc += width as usize;
                    stack.push(U256::from_big_endian(&bytes));
                }
                Opcode::Unknown(op) => {
                    return Err(AdapterError::InvalidOpcode(op));
                }
            }
        }

        Ok(ExecutionOutcome { exit_reason: ExitReason::Succeed, return_data, gas_used: self.gas_used })
    }

    fn gas_cost(op: &Opcode) -> u64 {
        match op {
            Opcode::Stop => 0,
            Opcode::Push(_) => 2,
            Opcode::Add | Opcode::Sub => 3,
            Opcode::Pop => 2,
            Opcode::MStore => 3,
            Opcode::Return => 3,
            Opcode::Unknown(_) => 1,
        }
    }

    fn pop2(stack: &mut Vec<U256>) -> AdapterResult<(U256, U256)> {
        let b = stack.pop().ok_or(AdapterError::StackUnderflow)?;
        let a = stack.pop().ok_or(AdapterError::StackUnderflow)?;
        Ok((a, b))
    }

    fn as_usize(value: U256) -> AdapterResult<usize> {
        let max = U256::from(usize::MAX as u128);
        if value > max {
            return Err(AdapterError::MalformedCalldata);
        }
        Ok(value.low_u128() as usize)
    }

    fn write_memory(memory: &mut Vec<u8>, offset: U256, value: U256) -> AdapterResult<()> {
        let offset = Self::as_usize(offset)?;
        if memory.len() < offset + 32 {
            memory.resize(offset + 32, 0);
        }
        let bytes = value.to_big_endian();
        memory[offset..offset + 32].copy_from_slice(&bytes);
        Ok(())
    }

    fn read_memory(memory: &[u8], offset: U256, len: U256) -> AdapterResult<Vec<u8>> {
        let offset = Self::as_usize(offset)?;
        let len = Self::as_usize(len)?;
        if offset + len > memory.len() {
            return Err(AdapterError::MalformedCalldata);
        }
        Ok(memory[offset..offset + len].to_vec())
    }
}
