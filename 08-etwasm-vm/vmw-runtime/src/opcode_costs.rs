//! Comprehensive opcode cost table with EVM Berlin/London fork compatibility

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use crate::VMw;

/// ============================================================================
/// OPCODE COST TABLE
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct OpcodeCostTable {
    /// Base operation price multiplier
    pub op_price: u32,
    /// Dynamic gas adjustment factor (for congestion-based pricing)
    pub dynamic_factor: u32,
}

impl OpcodeCostTable {
    /// Create new cost table with default pricing
    pub fn new() -> Self {
        Self {
            op_price: 1,
            dynamic_factor: 100, // 100 = 1.0x (percentage)
        }
    }

    /// Create with custom operation price
    pub fn with_price(op_price: u32) -> Self {
        Self {
            op_price,
            dynamic_factor: 100,
        }
    }

    /// Set dynamic gas adjustment (percentage: 100 = 1.0x, 200 = 2.0x)
    pub fn set_dynamic_factor(&mut self, factor: u32) {
        self.dynamic_factor = factor.max(50).min(500); // 0.5x to 5.0x range
    }

    /// Get gas cost for an opcode with dynamic pricing
    pub fn get_opcode_cost(&self, opcode: u8) -> VMw {
        let base_cost = self.get_base_opcode_cost(opcode);
        self.apply_dynamic_pricing(base_cost)
    }

    /// Get base opcode cost (Berlin/London fork costs)
    pub fn get_base_opcode_cost(&self, opcode: u8) -> VMw {
        match opcode {
            // 0x00-0x0F: Arithmetic operations
            0x00 => 0,      // STOP
            0x01 => 3,      // ADD
            0x02 => 5,      // MUL
            0x03 => 3,      // SUB
            0x04 => 5,      // DIV
            0x05 => 5,      // SDIV
            0x06 => 5,      // MOD
            0x07 => 5,      // SMOD
            0x08 => 8,      // ADDMOD
            0x09 => 8,      // MULMOD
            0x0A => 10,     // EXP (base, dynamic cost added per byte)
            0x0B => 5,      // SIGNEXTEND

            // 0x10-0x1F: Comparison & bitwise logic
            0x10 => 3,      // LT
            0x11 => 3,      // GT
            0x12 => 3,      // SLT
            0x13 => 3,      // SGT
            0x14 => 3,      // EQ
            0x15 => 3,      // ISZERO
            0x16 => 3,      // AND
            0x17 => 3,      // OR
            0x18 => 3,      // XOR
            0x19 => 3,      // NOT
            0x1A => 3,      // BYTE
            0x1B => 3,      // SHL
            0x1C => 3,      // SHR
            0x1D => 3,      // SAR

            // 0x20: SHA3
            0x20 => 30,     // SHA3 (base, plus 6 per word)

            // 0x30-0x3F: Environmental information
            0x30 => 2,      // ADDRESS
            0x31 => 100,    // BALANCE (warm access)
            0x32 => 2,      // ORIGIN
            0x33 => 2,      // CALLER
            0x34 => 2,      // CALLVALUE
            0x35 => 3,      // CALLDATALOAD
            0x36 => 2,      // CALLDATASIZE
            0x37 => 3,      // CALLDATACOPY (base, plus memory expansion)
            0x38 => 2,      // CODESIZE
            0x39 => 3,      // CODECOPY (base, plus memory expansion)
            0x3A => 2,      // GASPRICE
            0x3B => 100,    // EXTCODESIZE (warm access)
            0x3C => 100,    // EXTCODECOPY (warm access, base)
            0x3D => 2,      // RETURNDATASIZE
            0x3E => 3,      // RETURNDATACOPY

            // 0x40-0x48: Block information
            0x40 => 20,     // BLOCKHASH
            0x41 => 2,      // COINBASE
            0x42 => 2,      // TIMESTAMP
            0x43 => 2,      // NUMBER
            0x44 => 2,      // DIFFICULTY (PREVRANDAO post-merge)
            0x45 => 2,      // GASLIMIT
            0x46 => 2,      // CHAINID
            0x47 => 2,      // SELFBALANCE
            0x48 => 2,      // BASEFEE

            // 0x50-0x5F: Stack, memory, storage, and flow operations
            0x50 => 2,      // POP
            0x51 => 3,      // MLOAD
            0x52 => 3,      // MSTORE
            0x53 => 3,      // MSTORE8
            0x54 => 2100,   // SLOAD (cold access)
            0x55 => 20000,  // SSTORE (cold access, worst case - will be refined)
            0x56 => 8,      // JUMP
            0x57 => 10,     // JUMPI
            0x58 => 2,      // PC
            0x59 => 2,      // MSIZE
            0x5A => 2,      // GAS
            0x5B => 1,      // JUMPDEST

            // 0x60-0x7F: PUSH operations
            0x60..=0x7F => 3, // PUSH1 - PUSH32

            // 0x80-0x8F: DUP operations
            0x80..=0x8F => 3, // DUP1 - DUP16

            // 0x90-0x9F: SWAP operations
            0x90..=0x9F => 3, // SWAP1 - SWAP16

            // 0xA0-0xA4: LOG operations
            0xA0 => 375,    // LOG0 (base, plus topic and data costs)
            0xA1 => 750,    // LOG1
            0xA2 => 1125,   // LOG2
            0xA3 => 1500,   // LOG3
            0xA4 => 1875,   // LOG4

            // 0xF0-0xFF: System operations
            0xF0 => 32000,  // CREATE
            0xF1 => 2600,   // CALL (warm access, base)
            0xF2 => 2600,   // CALLCODE (deprecated but supported)
            0xF3 => 0,      // RETURN
            0xF4 => 2600,   // DELEGATECALL (warm access)
            0xF5 => 32000,  // CREATE2
            0xFA => 2600,   // STATICCALL (warm access)
            0xFD => 0,      // REVERT
            0xFE => 0,      // INVALID
            0xFF => 5000,   // SELFDESTRUCT (base, refund may apply)

            // Unknown opcodes
            _ => 0,
        }
    }

    /// Apply dynamic pricing based on network conditions
    fn apply_dynamic_pricing(&self, base_cost: VMw) -> VMw {
        let adjusted = (base_cost as u128)
            .saturating_mul(self.dynamic_factor as u128)
            .saturating_div(100);
        (adjusted as VMw).saturating_mul(self.op_price as VMw)
    }

    /// Calculate memory expansion cost
    /// Formula: memory_cost = (size_in_words^2 / 512) + (3 * size_in_words)
    pub fn calculate_memory_cost(&self, new_size_bytes: u64) -> VMw {
        if new_size_bytes == 0 {
            return 0;
        }

        // Convert to words (32-byte chunks, rounded up)
        let size_in_words = (new_size_bytes + 31) / 32;

        // Quadratic cost: size^2 / 512
        let quadratic_cost = (size_in_words * size_in_words) / 512;

        // Linear cost: 3 * size
        let linear_cost = 3 * size_in_words;

        let total_cost = (quadratic_cost + linear_cost) as VMw;
        self.apply_dynamic_pricing(total_cost)
    }

    /// Calculate storage operation cost with EIP-2200 gas metering
    /// Returns (cost, refund)
    pub fn calculate_storage_cost(
        &self,
        original_value: [u8; 32],
        current_value: [u8; 32],
        new_value: [u8; 32],
    ) -> (VMw, VMw) {
        let is_zero = |v: &[u8; 32]| v.iter().all(|&b| b == 0);

        let original_is_zero = is_zero(&original_value);
        let current_is_zero = is_zero(&current_value);
        let new_is_zero = is_zero(&new_value);

        // EIP-2200: Detailed gas metering for SSTORE
        let (base_cost, refund) = if current_value == new_value {
            // No-op: same value
            (100, 0)
        } else if original_value == current_value {
            // First write in transaction
            if original_is_zero {
                // 0 -> non-zero: set storage
                (20000, 0)
            } else if new_is_zero {
                // non-zero -> 0: clear storage (refund applies)
                (5000, 15000)
            } else {
                // non-zero -> non-zero: modify storage
                (5000, 0)
            }
        } else {
            // Subsequent write in transaction
            if original_is_zero {
                if current_is_zero {
                    // Should not happen (0 -> 0)
                    (100, 0)
                } else if new_is_zero {
                    // 0 -> non-zero -> 0: refund set cost
                    (100, 19900)
                } else {
                    // 0 -> non-zero -> different non-zero
                    (100, 0)
                }
            } else {
                if current_is_zero && !new_is_zero {
                    // non-zero -> 0 -> non-zero: remove refund
                    (100, 0) // Note: should remove 15000 refund
                } else if !current_is_zero && new_is_zero {
                    // non-zero -> non-zero -> 0: add refund
                    (100, 15000)
                } else {
                    // non-zero -> non-zero -> non-zero
                    (100, 0)
                }
            }
        };

        (self.apply_dynamic_pricing(base_cost), refund)
    }

    /// Get storage read cost (SLOAD)
    /// - Cold access: 2100 gas
    /// - Warm access: 100 gas
    pub fn get_storage_read_cost(&self) -> VMw {
        self.apply_dynamic_pricing(2100) // Assuming cold access
    }

    /// Get call cost based on parameters
    pub fn get_call_cost(&self, value_transfer: bool, account_exists: bool) -> VMw {
        let mut cost = 2600; // Base warm access cost

        if value_transfer {
            cost += 9000; // Value transfer cost
            if !account_exists {
                cost += 25000; // New account creation cost
            }
        }

        self.apply_dynamic_pricing(cost)
    }

    /// Calculate EXP opcode cost
    /// Base: 10 gas
    /// Dynamic: 50 gas per byte of exponent
    pub fn calculate_exp_cost(&self, exponent_bytes: usize) -> VMw {
        let base = 10;
        let dynamic = 50 * exponent_bytes as VMw;
        self.apply_dynamic_pricing(base + dynamic)
    }

    /// Calculate SHA3 cost
    /// Base: 30 gas
    /// Dynamic: 6 gas per word (32 bytes)
    pub fn calculate_sha3_cost(&self, data_size_bytes: usize) -> VMw {
        let base = 30;
        let words = (data_size_bytes + 31) / 32;
        let dynamic = 6 * words as VMw;
        self.apply_dynamic_pricing(base + dynamic)
    }

    /// Calculate LOG cost
    /// Base: 375 * (topics + 1)
    /// Dynamic: 8 gas per byte of data
    pub fn calculate_log_cost(&self, topics: u8, data_size_bytes: usize) -> VMw {
        let base = 375 * (1 + topics as VMw);
        let dynamic = 8 * data_size_bytes as VMw;
        self.apply_dynamic_pricing(base + dynamic)
    }

    /// Calculate COPY operation cost (CALLDATACOPY, CODECOPY, etc.)
    /// Base: 3 gas
    /// Dynamic: 3 gas per word
    pub fn calculate_copy_cost(&self, length_bytes: usize) -> VMw {
        let base = 3;
        let words = (length_bytes + 31) / 32;
        let dynamic = 3 * words as VMw;
        self.apply_dynamic_pricing(base + dynamic)
    }
}

impl Default for OpcodeCostTable {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// OPCODE CATEGORIES FOR ANALYSIS
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcodeCategory {
    Arithmetic,
    Comparison,
    BitwiseLogic,
    Environmental,
    BlockInfo,
    Stack,
    Memory,
    Storage,
    ControlFlow,
    Push,
    Dup,
    Swap,
    Log,
    System,
    Invalid,
}

impl OpcodeCategory {
    pub fn from_opcode(opcode: u8) -> Self {
        match opcode {
            0x01..=0x0B => OpcodeCategory::Arithmetic,
            0x10..=0x15 | 0x1E..=0x1F => OpcodeCategory::Comparison,
            0x16..=0x1D => OpcodeCategory::BitwiseLogic,
            0x30..=0x3E => OpcodeCategory::Environmental,
            0x40..=0x48 => OpcodeCategory::BlockInfo,
            0x50 | 0x56..=0x5B => OpcodeCategory::ControlFlow,
            0x51..=0x53 | 0x59 => OpcodeCategory::Memory,
            0x54..=0x55 => OpcodeCategory::Storage,
            0x60..=0x7F => OpcodeCategory::Push,
            0x80..=0x8F => OpcodeCategory::Dup,
            0x90..=0x9F => OpcodeCategory::Swap,
            0xA0..=0xA4 => OpcodeCategory::Log,
            0xF0..=0xFF => OpcodeCategory::System,
            _ => OpcodeCategory::Invalid,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            OpcodeCategory::Arithmetic => "Arithmetic",
            OpcodeCategory::Comparison => "Comparison",
            OpcodeCategory::BitwiseLogic => "Bitwise Logic",
            OpcodeCategory::Environmental => "Environmental Info",
            OpcodeCategory::BlockInfo => "Block Info",
            OpcodeCategory::Stack => "Stack",
            OpcodeCategory::Memory => "Memory",
            OpcodeCategory::Storage => "Storage",
            OpcodeCategory::ControlFlow => "Control Flow",
            OpcodeCategory::Push => "Push",
            OpcodeCategory::Dup => "Duplicate",
            OpcodeCategory::Swap => "Swap",
            OpcodeCategory::Log => "Logging",
            OpcodeCategory::System => "System",
            OpcodeCategory::Invalid => "Invalid",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_opcode_costs() {
        let table = OpcodeCostTable::new();

        assert_eq!(table.get_base_opcode_cost(0x01), 3);  // ADD
        assert_eq!(table.get_base_opcode_cost(0x02), 5);  // MUL
        assert_eq!(table.get_base_opcode_cost(0x54), 2100);  // SLOAD
        assert_eq!(table.get_base_opcode_cost(0x55), 20000);  // SSTORE
    }

    #[test]
    fn test_dynamic_pricing() {
        let mut table = OpcodeCostTable::new();

        // Normal pricing (1.0x)
        assert_eq!(table.get_opcode_cost(0x01), 3);

        // 2x dynamic factor
        table.set_dynamic_factor(200);
        assert_eq!(table.get_opcode_cost(0x01), 6);

        // 0.5x dynamic factor
        table.set_dynamic_factor(50);
        assert_eq!(table.get_opcode_cost(0x01), 1);
    }

    #[test]
    fn test_memory_cost() {
        let table = OpcodeCostTable::new();

        // Small memory allocation
        let cost_32 = table.calculate_memory_cost(32);
        assert!(cost_32 > 0);

        // Larger allocation should cost more
        let cost_1024 = table.calculate_memory_cost(1024);
        assert!(cost_1024 > cost_32);

        // Quadratic growth
        let cost_2048 = table.calculate_memory_cost(2048);
        assert!(cost_2048 > cost_1024 * 2);
    }

    #[test]
    fn test_storage_cost_zero_to_nonzero() {
        let table = OpcodeCostTable::new();
        let zero = [0u8; 32];
        let value = [1u8; 32];

        let (cost, refund) = table.calculate_storage_cost(zero, zero, value);
        assert_eq!(cost, 20000);  // Set storage
        assert_eq!(refund, 0);
    }

    #[test]
    fn test_storage_cost_nonzero_to_zero() {
        let table = OpcodeCostTable::new();
        let zero = [0u8; 32];
        let value = [1u8; 32];

        let (cost, refund) = table.calculate_storage_cost(value, value, zero);
        assert_eq!(cost, 5000);   // Clear storage
        assert_eq!(refund, 15000); // Refund for clearing
    }

    #[test]
    fn test_storage_cost_nonzero_to_nonzero() {
        let table = OpcodeCostTable::new();
        let value1 = [1u8; 32];
        let value2 = [2u8; 32];

        let (cost, refund) = table.calculate_storage_cost(value1, value1, value2);
        assert_eq!(cost, 5000);  // Modify storage
        assert_eq!(refund, 0);
    }

    #[test]
    fn test_call_costs() {
        let table = OpcodeCostTable::new();

        // Simple call (no value, existing account)
        let cost1 = table.get_call_cost(false, true);
        assert_eq!(cost1, 2600);

        // Call with value transfer
        let cost2 = table.get_call_cost(true, true);
        assert_eq!(cost2, 11600); // 2600 + 9000

        // Call creating new account
        let cost3 = table.get_call_cost(true, false);
        assert_eq!(cost3, 36600); // 2600 + 9000 + 25000
    }

    #[test]
    fn test_exp_cost() {
        let table = OpcodeCostTable::new();

        // EXP with 1 byte exponent
        let cost1 = table.calculate_exp_cost(1);
        assert_eq!(cost1, 60); // 10 + 50

        // EXP with 4 byte exponent
        let cost4 = table.calculate_exp_cost(4);
        assert_eq!(cost4, 210); // 10 + 200
    }

    #[test]
    fn test_sha3_cost() {
        let table = OpcodeCostTable::new();

        // SHA3 of 32 bytes (1 word)
        let cost32 = table.calculate_sha3_cost(32);
        assert_eq!(cost32, 36); // 30 + 6

        // SHA3 of 64 bytes (2 words)
        let cost64 = table.calculate_sha3_cost(64);
        assert_eq!(cost64, 42); // 30 + 12
    }

    #[test]
    fn test_log_cost() {
        let table = OpcodeCostTable::new();

        // LOG0 with 32 bytes data
        let cost_log0 = table.calculate_log_cost(0, 32);
        assert_eq!(cost_log0, 375 + 256); // 375 + (8 * 32)

        // LOG4 with 100 bytes data
        let cost_log4 = table.calculate_log_cost(4, 100);
        assert_eq!(cost_log4, 1875 + 800); // (375 * 5) + (8 * 100)
    }

    #[test]
    fn test_opcode_categories() {
        assert_eq!(OpcodeCategory::from_opcode(0x01), OpcodeCategory::Arithmetic);
        assert_eq!(OpcodeCategory::from_opcode(0x54), OpcodeCategory::Storage);
        assert_eq!(OpcodeCategory::from_opcode(0x60), OpcodeCategory::Push);
        assert_eq!(OpcodeCategory::from_opcode(0xF1), OpcodeCategory::System);
    }
}
