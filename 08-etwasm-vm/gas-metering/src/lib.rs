//! # ETWasm Gas Metering
//!
//! This module re-exports VMW gas metering from `etrid-vmw-gas`, the canonical
//! source of truth for VMW economics in the ËTRID native currency system.
//!
//! ## Architecture
//!
//! ```text
//! 06-native-currency/vmw-gas/   ← Canonical source (VMW economics)
//!          ↑
//! 08-etwasm-vm/gas-metering/    ← This module (re-exports for VM consumers)
//!          ↑
//! 08-etwasm-vm/opcodes/
//! 08-etwasm-vm/runtime/
//! 08-etwasm-vm/pallet/
//! ```
//!
//! ## Why Re-export?
//!
//! - VMW is an **economic concept** (computation pricing, fees, conversion to ÉTR)
//! - Economics belong in the Native Currency module (06), not the VM (08)
//! - This wrapper maintains backwards compatibility for VM consumers
//! - Single source of truth prevents drift between modules
//!
//! ## Usage
//!
//! ```rust,ignore
//! use etwasm_gas_metering::{VMw, GasMeter, FeeCalculator};
//! use etwasm_gas_metering::{VMW_BLOCK_LIMIT, WATTS_PER_ETRID};
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export everything from the canonical VMW gas economics module
pub use etrid_vmw_gas::*;
