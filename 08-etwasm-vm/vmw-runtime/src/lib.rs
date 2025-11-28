#![cfg_attr(not(feature = "std"), no_std)]

//! # VMw Metering Runtime
//!
//! Comprehensive gas metering system for ETWasm VM with:
//! - Detailed opcode cost tables
//! - Dynamic gas pricing based on resource usage
//! - Execution limits and safeguards
//! - Resource tracking (CPU, memory, storage I/O)
//! - Gas refund mechanisms

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub use etwasm_gas_metering::{Balance, VMw, WATTS_PER_ETRID};

mod opcode_costs;
mod execution_limits;
mod resource_tracker;
mod gas_refunds;

pub use opcode_costs::*;
pub use execution_limits::*;
pub use resource_tracker::*;
pub use gas_refunds::*;

/// ============================================================================
/// COMPREHENSIVE VMw METERING RUNTIME
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct VmwMeteringRuntime {
    /// Opcode cost calculator
    pub opcode_costs: OpcodeCostTable,
    /// Execution limits tracker
    pub limits: ExecutionLimits,
    /// Resource usage tracker
    pub resources: ResourceTracker,
    /// Gas refund manager
    pub refunds: GasRefundManager,
    /// Total gas consumed
    pub gas_consumed: VMw,
    /// Total gas refunded
    pub gas_refunded: VMw,
}

impl VmwMeteringRuntime {
    /// Create new metering runtime with default settings
    pub fn new(gas_limit: VMw) -> Self {
        Self {
            opcode_costs: OpcodeCostTable::new(),
            limits: ExecutionLimits::new(gas_limit),
            resources: ResourceTracker::new(),
            refunds: GasRefundManager::new(),
            gas_consumed: 0,
            gas_refunded: 0,
        }
    }

    /// Create with custom operation price
    pub fn with_op_price(gas_limit: VMw, op_price: u32) -> Self {
        Self {
            opcode_costs: OpcodeCostTable::with_price(op_price),
            limits: ExecutionLimits::new(gas_limit),
            resources: ResourceTracker::new(),
            refunds: GasRefundManager::new(),
            gas_consumed: 0,
            gas_refunded: 0,
        }
    }

    /// Charge gas for an opcode execution
    pub fn charge_opcode(&mut self, opcode: u8) -> Result<VMw, MeteringError> {
        // Check execution limits
        self.limits.check_can_execute()?;

        // Calculate opcode cost
        let cost = self.opcode_costs.get_opcode_cost(opcode);

        // Check if we have enough gas
        let gas_remaining = self.get_gas_remaining();
        if gas_remaining < cost {
            return Err(MeteringError::OutOfGas);
        }

        // Consume gas
        self.gas_consumed = self.gas_consumed.saturating_add(cost);

        // Track execution
        self.limits.increment_instruction_count();

        Ok(cost)
    }

    /// Charge gas for memory expansion
    pub fn charge_memory_expansion(&mut self, new_size: u64) -> Result<VMw, MeteringError> {
        let cost = self.opcode_costs.calculate_memory_cost(new_size);

        // Check gas
        let gas_remaining = self.get_gas_remaining();
        if gas_remaining < cost {
            return Err(MeteringError::OutOfGas);
        }

        // Track memory allocation
        self.resources.allocate_memory(new_size)?;
        self.gas_consumed = self.gas_consumed.saturating_add(cost);

        Ok(cost)
    }

    /// Charge gas for storage write
    pub fn charge_storage_write(
        &mut self,
        original_value: [u8; 32],
        current_value: [u8; 32],
        new_value: [u8; 32],
    ) -> Result<VMw, MeteringError> {
        let (cost, refund) = self.opcode_costs.calculate_storage_cost(
            original_value,
            current_value,
            new_value,
        );

        // Check gas
        let gas_remaining = self.get_gas_remaining();
        if gas_remaining < cost {
            return Err(MeteringError::OutOfGas);
        }

        // Track storage I/O
        self.resources.track_storage_write()?;
        self.gas_consumed = self.gas_consumed.saturating_add(cost);

        // Register refund if applicable
        if refund > 0 {
            self.refunds.add_refund(refund);
        }

        Ok(cost)
    }

    /// Charge gas for storage read
    pub fn charge_storage_read(&mut self) -> Result<VMw, MeteringError> {
        let cost = self.opcode_costs.get_storage_read_cost();

        // Check gas
        let gas_remaining = self.get_gas_remaining();
        if gas_remaining < cost {
            return Err(MeteringError::OutOfGas);
        }

        // Track storage I/O
        self.resources.track_storage_read()?;
        self.gas_consumed = self.gas_consumed.saturating_add(cost);

        Ok(cost)
    }

    /// Charge gas for contract call
    pub fn charge_call(
        &mut self,
        value_transfer: bool,
        account_exists: bool,
    ) -> Result<VMw, MeteringError> {
        let cost = self.opcode_costs.get_call_cost(value_transfer, account_exists);

        // Check gas
        let gas_remaining = self.get_gas_remaining();
        if gas_remaining < cost {
            return Err(MeteringError::OutOfGas);
        }

        // Check call depth
        self.limits.increment_call_depth()?;
        self.gas_consumed = self.gas_consumed.saturating_add(cost);

        Ok(cost)
    }

    /// Decrement call depth when returning from call
    pub fn return_from_call(&mut self) {
        self.limits.decrement_call_depth();
    }

    /// Finalize execution and apply refunds
    pub fn finalize(&mut self) -> VMw {
        // Calculate maximum refund (50% of gas consumed)
        let max_refund = self.gas_consumed / 2;
        let actual_refund = self.refunds.get_total_refund().min(max_refund);

        self.gas_refunded = actual_refund;
        self.gas_consumed.saturating_sub(actual_refund)
    }

    /// Get remaining gas
    pub fn get_gas_remaining(&self) -> VMw {
        self.limits.gas_limit.saturating_sub(self.gas_consumed)
    }

    /// Get execution statistics
    pub fn get_stats(&self) -> ExecutionStats {
        ExecutionStats {
            gas_consumed: self.gas_consumed,
            gas_refunded: self.gas_refunded,
            instructions_executed: self.limits.instruction_count,
            memory_allocated: self.resources.memory_allocated,
            storage_reads: self.resources.storage_reads,
            storage_writes: self.resources.storage_writes,
            call_depth: self.limits.call_depth,
            execution_time_ms: self.limits.execution_time_ms,
        }
    }

    /// Reset for new execution
    pub fn reset(&mut self, gas_limit: VMw) {
        self.limits = ExecutionLimits::new(gas_limit);
        self.resources = ResourceTracker::new();
        self.refunds = GasRefundManager::new();
        self.gas_consumed = 0;
        self.gas_refunded = 0;
    }
}

/// ============================================================================
/// EXECUTION STATISTICS
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct ExecutionStats {
    pub gas_consumed: VMw,
    pub gas_refunded: VMw,
    pub instructions_executed: u64,
    pub memory_allocated: u64,
    pub storage_reads: u32,
    pub storage_writes: u32,
    pub call_depth: u32,
    pub execution_time_ms: u64,
}

impl ExecutionStats {
    pub fn net_gas_used(&self) -> VMw {
        self.gas_consumed.saturating_sub(self.gas_refunded)
    }

    pub fn gas_per_instruction(&self) -> u64 {
        if self.instructions_executed == 0 {
            return 0;
        }
        self.gas_consumed / self.instructions_executed
    }
}

/// ============================================================================
/// METERING ERRORS
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum MeteringError {
    OutOfGas,
    ExecutionTimeout,
    StackDepthExceeded,
    CallDepthExceeded,
    MemoryLimitExceeded,
    StorageIOLimitExceeded,
    InvalidOpcode,
}

impl MeteringError {
    pub fn message(&self) -> &'static str {
        match self {
            MeteringError::OutOfGas => "Out of gas",
            MeteringError::ExecutionTimeout => "Execution timeout exceeded",
            MeteringError::StackDepthExceeded => "Stack depth limit exceeded",
            MeteringError::CallDepthExceeded => "Call depth limit exceeded",
            MeteringError::MemoryLimitExceeded => "Memory limit exceeded",
            MeteringError::StorageIOLimitExceeded => "Storage I/O limit exceeded",
            MeteringError::InvalidOpcode => "Invalid opcode",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_opcode_charging() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        // Charge for ADD opcode (should cost 3 gas)
        let cost = runtime.charge_opcode(0x01).unwrap();
        assert_eq!(cost, 3);
        assert_eq!(runtime.gas_consumed, 3);
        assert_eq!(runtime.get_gas_remaining(), 99_997);
    }

    #[test]
    fn test_out_of_gas() {
        let mut runtime = VmwMeteringRuntime::new(5);

        // First opcode succeeds (costs 3 gas)
        assert!(runtime.charge_opcode(0x01).is_ok());

        // Second opcode should fail (not enough gas: 2 remaining, need 3)
        let result = runtime.charge_opcode(0x01);
        assert_eq!(result, Err(MeteringError::OutOfGas));
    }

    #[test]
    fn test_memory_expansion() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        // Expand to 32 bytes
        let cost = runtime.charge_memory_expansion(32).unwrap();
        assert!(cost > 0);
        assert_eq!(runtime.resources.memory_allocated, 32);
    }

    #[test]
    fn test_storage_operations() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        // Storage read
        let read_cost = runtime.charge_storage_read().unwrap();
        assert!(read_cost > 0);
        assert_eq!(runtime.resources.storage_reads, 1);

        // Storage write (zero to non-zero)
        let zero = [0u8; 32];
        let value = [1u8; 32];
        let write_cost = runtime.charge_storage_write(zero, zero, value).unwrap();
        assert!(write_cost > 0);
        assert_eq!(runtime.resources.storage_writes, 1);
    }

    #[test]
    fn test_gas_refunds() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        // Write non-zero value
        let value = [1u8; 32];
        let zero = [0u8; 32];

        // Clear storage (should generate refund)
        runtime.charge_storage_write(value, value, zero).unwrap();

        let net_gas = runtime.finalize();
        assert!(runtime.gas_refunded > 0);
        assert!(net_gas < runtime.gas_consumed);
    }

    #[test]
    fn test_call_depth() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        // Increment call depth
        assert!(runtime.charge_call(false, true).is_ok());
        assert_eq!(runtime.limits.call_depth, 1);

        // Return from call
        runtime.return_from_call();
        assert_eq!(runtime.limits.call_depth, 0);
    }

    #[test]
    fn test_execution_stats() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        runtime.charge_opcode(0x01).unwrap();
        runtime.charge_opcode(0x02).unwrap();
        runtime.charge_storage_read().unwrap();

        let stats = runtime.get_stats();
        assert_eq!(stats.instructions_executed, 2);
        assert_eq!(stats.storage_reads, 1);
        assert!(stats.gas_consumed > 0);
    }

    #[test]
    fn test_reset() {
        let mut runtime = VmwMeteringRuntime::new(100_000);

        runtime.charge_opcode(0x01).unwrap();
        runtime.charge_storage_read().unwrap();

        assert!(runtime.gas_consumed > 0);

        runtime.reset(50_000);
        assert_eq!(runtime.gas_consumed, 0);
        assert_eq!(runtime.limits.gas_limit, 50_000);
        assert_eq!(runtime.resources.storage_reads, 0);
    }
}
