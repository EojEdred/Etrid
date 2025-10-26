//! Integration Tests for Ëtrid Protocol
//!
//! Comprehensive integration tests covering:
//! - Bridge System: All 12 bridge pallets across 3 configuration groups
//!   - Group A (Authority-based): BTC, ADA
//!   - Group B (Fee-based): ETH, XLM, XRP, BNB, TRX, LINK, SOL, SC-USDT
//!   - Group C (PalletId-based): DOGE, MATIC
//! - EDSC Workflow: End-to-end stablecoin lifecycle tests
//!   - Collateral → Minting → Redemption
//!   - Circuit breakers and queue processing
//!   - Multi-asset reserve management
//!   - Cross-pallet state consistency

pub mod bridge_tests;
pub mod common;
pub mod edsc_workflow_tests;

// Re-export test utilities
pub use common::*;
