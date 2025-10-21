//! EVM Opcode Definitions and Handlers for ETWasm VM
//!
//! This module provides Ethereum Virtual Machine opcode compatibility
//! for the ETWasm VM, enabling execution of EVM bytecode on Ëtrid.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use etwasm_gas_metering::VMw;

/// EVM Opcode byte value
pub type Opcode = u8;

/// ============================================================================
/// ARITHMETIC OPERATIONS (0x00 - 0x0F)
/// ============================================================================

pub const STOP: Opcode = 0x00;
pub const ADD: Opcode = 0x01;
pub const MUL: Opcode = 0x02;
pub const SUB: Opcode = 0x03;
pub const DIV: Opcode = 0x04;
pub const SDIV: Opcode = 0x05;
pub const MOD: Opcode = 0x06;
pub const SMOD: Opcode = 0x07;
pub const ADDMOD: Opcode = 0x08;
pub const MULMOD: Opcode = 0x09;
pub const EXP: Opcode = 0x0A;
pub const SIGNEXTEND: Opcode = 0x0B;

/// ============================================================================
/// COMPARISON & BITWISE LOGIC OPERATIONS (0x10 - 0x1F)
/// ============================================================================

pub const LT: Opcode = 0x10;
pub const GT: Opcode = 0x11;
pub const SLT: Opcode = 0x12;
pub const SGT: Opcode = 0x13;
pub const EQ: Opcode = 0x14;
pub const ISZERO: Opcode = 0x15;
pub const AND: Opcode = 0x16;
pub const OR: Opcode = 0x17;
pub const XOR: Opcode = 0x18;
pub const NOT: Opcode = 0x19;
pub const BYTE: Opcode = 0x1A;
pub const SHL: Opcode = 0x1B;
pub const SHR: Opcode = 0x1C;
pub const SAR: Opcode = 0x1D;

/// ============================================================================
/// SHA3 (0x20)
/// ============================================================================

pub const SHA3: Opcode = 0x20;

/// ============================================================================
/// ENVIRONMENTAL INFORMATION (0x30 - 0x3F)
/// ============================================================================

pub const ADDRESS: Opcode = 0x30;
pub const BALANCE: Opcode = 0x31;
pub const ORIGIN: Opcode = 0x32;
pub const CALLER: Opcode = 0x33;
pub const CALLVALUE: Opcode = 0x34;
pub const CALLDATALOAD: Opcode = 0x35;
pub const CALLDATASIZE: Opcode = 0x36;
pub const CALLDATACOPY: Opcode = 0x37;
pub const CODESIZE: Opcode = 0x38;
pub const CODECOPY: Opcode = 0x39;
pub const GASPRICE: Opcode = 0x3A;
pub const EXTCODESIZE: Opcode = 0x3B;
pub const EXTCODECOPY: Opcode = 0x3C;
pub const RETURNDATASIZE: Opcode = 0x3D;
pub const RETURNDATACOPY: Opcode = 0x3E;
pub const EXTCODEHASH: Opcode = 0x3F;

/// ============================================================================
/// BLOCK INFORMATION (0x40 - 0x4F)
/// ============================================================================

pub const BLOCKHASH: Opcode = 0x40;
pub const COINBASE: Opcode = 0x41;
pub const TIMESTAMP: Opcode = 0x42;
pub const NUMBER: Opcode = 0x43;
pub const DIFFICULTY: Opcode = 0x44;
pub const GASLIMIT: Opcode = 0x45;
pub const CHAINID: Opcode = 0x46;
pub const SELFBALANCE: Opcode = 0x47;
pub const BASEFEE: Opcode = 0x48;

/// ============================================================================
/// STACK, MEMORY, STORAGE AND FLOW OPERATIONS (0x50 - 0x5F)
/// ============================================================================

pub const POP: Opcode = 0x50;
pub const MLOAD: Opcode = 0x51;
pub const MSTORE: Opcode = 0x52;
pub const MSTORE8: Opcode = 0x53;
pub const SLOAD: Opcode = 0x54;
pub const SSTORE: Opcode = 0x55;
pub const JUMP: Opcode = 0x56;
pub const JUMPI: Opcode = 0x57;
pub const PC: Opcode = 0x58;
pub const MSIZE: Opcode = 0x59;
pub const GAS: Opcode = 0x5A;
pub const JUMPDEST: Opcode = 0x5B;

/// ============================================================================
/// PUSH OPERATIONS (0x60 - 0x7F)
/// ============================================================================

pub const PUSH1: Opcode = 0x60;
pub const PUSH2: Opcode = 0x61;
pub const PUSH3: Opcode = 0x62;
pub const PUSH4: Opcode = 0x63;
pub const PUSH5: Opcode = 0x64;
pub const PUSH6: Opcode = 0x65;
pub const PUSH7: Opcode = 0x66;
pub const PUSH8: Opcode = 0x67;
pub const PUSH9: Opcode = 0x68;
pub const PUSH10: Opcode = 0x69;
pub const PUSH11: Opcode = 0x6A;
pub const PUSH12: Opcode = 0x6B;
pub const PUSH13: Opcode = 0x6C;
pub const PUSH14: Opcode = 0x6D;
pub const PUSH15: Opcode = 0x6E;
pub const PUSH16: Opcode = 0x6F;
pub const PUSH17: Opcode = 0x70;
pub const PUSH18: Opcode = 0x71;
pub const PUSH19: Opcode = 0x72;
pub const PUSH20: Opcode = 0x73;
pub const PUSH21: Opcode = 0x74;
pub const PUSH22: Opcode = 0x75;
pub const PUSH23: Opcode = 0x76;
pub const PUSH24: Opcode = 0x77;
pub const PUSH25: Opcode = 0x78;
pub const PUSH26: Opcode = 0x79;
pub const PUSH27: Opcode = 0x7A;
pub const PUSH28: Opcode = 0x7B;
pub const PUSH29: Opcode = 0x7C;
pub const PUSH30: Opcode = 0x7D;
pub const PUSH31: Opcode = 0x7E;
pub const PUSH32: Opcode = 0x7F;

/// ============================================================================
/// DUP OPERATIONS (0x80 - 0x8F)
/// ============================================================================

pub const DUP1: Opcode = 0x80;
pub const DUP2: Opcode = 0x81;
pub const DUP3: Opcode = 0x82;
pub const DUP4: Opcode = 0x83;
pub const DUP5: Opcode = 0x84;
pub const DUP6: Opcode = 0x85;
pub const DUP7: Opcode = 0x86;
pub const DUP8: Opcode = 0x87;
pub const DUP9: Opcode = 0x88;
pub const DUP10: Opcode = 0x89;
pub const DUP11: Opcode = 0x8A;
pub const DUP12: Opcode = 0x8B;
pub const DUP13: Opcode = 0x8C;
pub const DUP14: Opcode = 0x8D;
pub const DUP15: Opcode = 0x8E;
pub const DUP16: Opcode = 0x8F;

/// ============================================================================
/// SWAP OPERATIONS (0x90 - 0x9F)
/// ============================================================================

pub const SWAP1: Opcode = 0x90;
pub const SWAP2: Opcode = 0x91;
pub const SWAP3: Opcode = 0x92;
pub const SWAP4: Opcode = 0x93;
pub const SWAP5: Opcode = 0x94;
pub const SWAP6: Opcode = 0x95;
pub const SWAP7: Opcode = 0x96;
pub const SWAP8: Opcode = 0x97;
pub const SWAP9: Opcode = 0x98;
pub const SWAP10: Opcode = 0x99;
pub const SWAP11: Opcode = 0x9A;
pub const SWAP12: Opcode = 0x9B;
pub const SWAP13: Opcode = 0x9C;
pub const SWAP14: Opcode = 0x9D;
pub const SWAP15: Opcode = 0x9E;
pub const SWAP16: Opcode = 0x9F;

/// ============================================================================
/// LOGGING OPERATIONS (0xA0 - 0xA4)
/// ============================================================================

pub const LOG0: Opcode = 0xA0;
pub const LOG1: Opcode = 0xA1;
pub const LOG2: Opcode = 0xA2;
pub const LOG3: Opcode = 0xA3;
pub const LOG4: Opcode = 0xA4;

/// ============================================================================
/// SYSTEM OPERATIONS (0xF0 - 0xFF)
/// ============================================================================

pub const CREATE: Opcode = 0xF0;
pub const CALL: Opcode = 0xF1;
pub const CALLCODE: Opcode = 0xF2;
pub const RETURN: Opcode = 0xF3;
pub const DELEGATECALL: Opcode = 0xF4;
pub const CREATE2: Opcode = 0xF5;
pub const STATICCALL: Opcode = 0xFA;
pub const REVERT: Opcode = 0xFD;
pub const INVALID: Opcode = 0xFE;
pub const SELFDESTRUCT: Opcode = 0xFF;

/// ============================================================================
/// OPCODE GAS COSTS (Berlin/London Fork Compatibility)
/// ============================================================================

/// Get the gas cost for a given opcode
pub fn get_opcode_gas_cost(opcode: Opcode) -> VMw {
    match opcode {
        // Zero gas
        STOP => 0,

        // Base operations (3 gas)
        ADD | SUB | NOT | LT | GT | SLT | SGT | EQ | ISZERO | AND | OR | XOR | BYTE |
        SHL | SHR | SAR | ADDRESS | ORIGIN | CALLER | CALLVALUE | CALLDATASIZE |
        CODESIZE | GASPRICE | RETURNDATASIZE | COINBASE | TIMESTAMP | NUMBER |
        DIFFICULTY | GASLIMIT | CHAINID | SELFBALANCE | BASEFEE | POP | PC | MSIZE |
        GAS => 3,

        // Arithmetic operations (5 gas)
        MUL | DIV | SDIV | MOD | SMOD => 5,

        // ADDMOD, MULMOD (8 gas)
        ADDMOD | MULMOD => 8,

        // SIGNEXTEND (5 gas)
        SIGNEXTEND => 5,

        // EXP (10 gas base + 50 per byte)
        EXP => 10,

        // SHA3 (30 gas base + 6 per word)
        SHA3 => 30,

        // BALANCE (2600 gas - cold, 100 gas warm)
        BALANCE => 2600,

        // EXTCODESIZE, EXTCODEHASH (2600 gas cold, 100 warm)
        EXTCODESIZE | EXTCODEHASH => 2600,

        // EXTCODECOPY (2600 gas cold + 3 per byte)
        EXTCODECOPY => 2600,

        // BLOCKHASH (20 gas)
        BLOCKHASH => 20,

        // Memory operations
        MLOAD | MSTORE | MSTORE8 => 3,

        // Storage operations (Berlin/London)
        SLOAD => 2100,  // Cold SLOAD
        SSTORE => 20000, // Cold SSTORE (worst case)

        // JUMP operations
        JUMP => 8,
        JUMPI => 10,
        JUMPDEST => 1,

        // PUSH operations (3 gas)
        PUSH1..=PUSH32 => 3,

        // DUP operations (3 gas)
        DUP1..=DUP16 => 3,

        // SWAP operations (3 gas)
        SWAP1..=SWAP16 => 3,

        // LOG operations (375 gas base + 375 per topic + 8 per byte)
        LOG0 => 375,
        LOG1 => 750,
        LOG2 => 1125,
        LOG3 => 1500,
        LOG4 => 1875,

        // CALLDATALOAD, CALLDATACOPY (3 gas base)
        CALLDATALOAD => 3,
        CALLDATACOPY | CODECOPY | RETURNDATACOPY => 3,

        // Contract creation and calls
        CREATE => 32000,
        CREATE2 => 32000,
        CALL => 2600,  // Minimum, varies based on value transfer
        CALLCODE => 2600,
        DELEGATECALL => 2600,
        STATICCALL => 2600,

        // RETURN, REVERT (0 gas base, memory expansion cost)
        RETURN | REVERT => 0,

        // SELFDESTRUCT (5000 gas base)
        SELFDESTRUCT => 5000,

        // Invalid or unknown opcode
        INVALID | _ => 0,
    }
}

/// Opcode information
#[derive(Debug, Clone, PartialEq, Eq, TypeInfo)]
pub struct OpcodeInfo {
    pub opcode: Opcode,
    pub name: &'static str,
    pub stack_input: u8,
    pub stack_output: u8,
    pub gas_cost: VMw,
}

/// Get opcode info by byte value
pub fn get_opcode_info(opcode: Opcode) -> Option<OpcodeInfo> {
    let info = match opcode {
        STOP => OpcodeInfo { opcode, name: "STOP", stack_input: 0, stack_output: 0, gas_cost: 0 },
        ADD => OpcodeInfo { opcode, name: "ADD", stack_input: 2, stack_output: 1, gas_cost: 3 },
        MUL => OpcodeInfo { opcode, name: "MUL", stack_input: 2, stack_output: 1, gas_cost: 5 },
        SUB => OpcodeInfo { opcode, name: "SUB", stack_input: 2, stack_output: 1, gas_cost: 5 },
        DIV => OpcodeInfo { opcode, name: "DIV", stack_input: 2, stack_output: 1, gas_cost: 5 },
        SDIV => OpcodeInfo { opcode, name: "SDIV", stack_input: 2, stack_output: 1, gas_cost: 5 },
        MOD => OpcodeInfo { opcode, name: "MOD", stack_input: 2, stack_output: 1, gas_cost: 5 },
        SMOD => OpcodeInfo { opcode, name: "SMOD", stack_input: 2, stack_output: 1, gas_cost: 5 },
        ADDMOD => OpcodeInfo { opcode, name: "ADDMOD", stack_input: 3, stack_output: 1, gas_cost: 8 },
        MULMOD => OpcodeInfo { opcode, name: "MULMOD", stack_input: 3, stack_output: 1, gas_cost: 8 },
        EXP => OpcodeInfo { opcode, name: "EXP", stack_input: 2, stack_output: 1, gas_cost: 10 },
        SIGNEXTEND => OpcodeInfo { opcode, name: "SIGNEXTEND", stack_input: 2, stack_output: 1, gas_cost: 5 },

        LT => OpcodeInfo { opcode, name: "LT", stack_input: 2, stack_output: 1, gas_cost: 3 },
        GT => OpcodeInfo { opcode, name: "GT", stack_input: 2, stack_output: 1, gas_cost: 3 },
        SLT => OpcodeInfo { opcode, name: "SLT", stack_input: 2, stack_output: 1, gas_cost: 3 },
        SGT => OpcodeInfo { opcode, name: "SGT", stack_input: 2, stack_output: 1, gas_cost: 3 },
        EQ => OpcodeInfo { opcode, name: "EQ", stack_input: 2, stack_output: 1, gas_cost: 3 },
        ISZERO => OpcodeInfo { opcode, name: "ISZERO", stack_input: 1, stack_output: 1, gas_cost: 3 },
        AND => OpcodeInfo { opcode, name: "AND", stack_input: 2, stack_output: 1, gas_cost: 3 },
        OR => OpcodeInfo { opcode, name: "OR", stack_input: 2, stack_output: 1, gas_cost: 3 },
        XOR => OpcodeInfo { opcode, name: "XOR", stack_input: 2, stack_output: 1, gas_cost: 3 },
        NOT => OpcodeInfo { opcode, name: "NOT", stack_input: 1, stack_output: 1, gas_cost: 3 },
        BYTE => OpcodeInfo { opcode, name: "BYTE", stack_input: 2, stack_output: 1, gas_cost: 3 },
        SHL => OpcodeInfo { opcode, name: "SHL", stack_input: 2, stack_output: 1, gas_cost: 3 },
        SHR => OpcodeInfo { opcode, name: "SHR", stack_input: 2, stack_output: 1, gas_cost: 3 },
        SAR => OpcodeInfo { opcode, name: "SAR", stack_input: 2, stack_output: 1, gas_cost: 3 },

        SHA3 => OpcodeInfo { opcode, name: "SHA3", stack_input: 2, stack_output: 1, gas_cost: 30 },

        SLOAD => OpcodeInfo { opcode, name: "SLOAD", stack_input: 1, stack_output: 1, gas_cost: 2100 },
        SSTORE => OpcodeInfo { opcode, name: "SSTORE", stack_input: 2, stack_output: 0, gas_cost: 20000 },

        JUMP => OpcodeInfo { opcode, name: "JUMP", stack_input: 1, stack_output: 0, gas_cost: 8 },
        JUMPI => OpcodeInfo { opcode, name: "JUMPI", stack_input: 2, stack_output: 0, gas_cost: 10 },
        JUMPDEST => OpcodeInfo { opcode, name: "JUMPDEST", stack_input: 0, stack_output: 0, gas_cost: 1 },

        RETURN => OpcodeInfo { opcode, name: "RETURN", stack_input: 2, stack_output: 0, gas_cost: 0 },
        REVERT => OpcodeInfo { opcode, name: "REVERT", stack_input: 2, stack_output: 0, gas_cost: 0 },

        CALL => OpcodeInfo { opcode, name: "CALL", stack_input: 7, stack_output: 1, gas_cost: 2600 },
        DELEGATECALL => OpcodeInfo { opcode, name: "DELEGATECALL", stack_input: 6, stack_output: 1, gas_cost: 2600 },
        STATICCALL => OpcodeInfo { opcode, name: "STATICCALL", stack_input: 6, stack_output: 1, gas_cost: 2600 },

        CREATE => OpcodeInfo { opcode, name: "CREATE", stack_input: 3, stack_output: 1, gas_cost: 32000 },
        CREATE2 => OpcodeInfo { opcode, name: "CREATE2", stack_input: 4, stack_output: 1, gas_cost: 32000 },

        SELFDESTRUCT => OpcodeInfo { opcode, name: "SELFDESTRUCT", stack_input: 1, stack_output: 0, gas_cost: 5000 },

        _ => return None,
    };
    Some(info)
}

/// Check if opcode is valid
pub fn is_valid_opcode(opcode: Opcode) -> bool {
    get_opcode_info(opcode).is_some()
}

/// Check if opcode is a PUSH instruction
pub fn is_push_opcode(opcode: Opcode) -> bool {
    (PUSH1..=PUSH32).contains(&opcode)
}

/// Get number of bytes to push for PUSH opcode
pub fn get_push_bytes(opcode: Opcode) -> Option<u8> {
    if is_push_opcode(opcode) {
        Some(opcode - PUSH1 + 1)
    } else {
        None
    }
}

/// Check if opcode is a DUP instruction
pub fn is_dup_opcode(opcode: Opcode) -> bool {
    (DUP1..=DUP16).contains(&opcode)
}

/// Check if opcode is a SWAP instruction
pub fn is_swap_opcode(opcode: Opcode) -> bool {
    (SWAP1..=SWAP16).contains(&opcode)
}

/// Check if opcode is a LOG instruction
pub fn is_log_opcode(opcode: Opcode) -> bool {
    (LOG0..=LOG4).contains(&opcode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_gas_costs() {
        assert_eq!(get_opcode_gas_cost(ADD), 3);
        assert_eq!(get_opcode_gas_cost(MUL), 5);
        assert_eq!(get_opcode_gas_cost(SLOAD), 2100);
        assert_eq!(get_opcode_gas_cost(SSTORE), 20000);
    }

    #[test]
    fn test_push_opcodes() {
        assert!(is_push_opcode(PUSH1));
        assert!(is_push_opcode(PUSH32));
        assert!(!is_push_opcode(ADD));
        assert_eq!(get_push_bytes(PUSH1), Some(1));
        assert_eq!(get_push_bytes(PUSH32), Some(32));
    }

    #[test]
    fn test_opcode_info() {
        let info = get_opcode_info(ADD).unwrap();
        assert_eq!(info.name, "ADD");
        assert_eq!(info.stack_input, 2);
        assert_eq!(info.stack_output, 1);
    }
}
