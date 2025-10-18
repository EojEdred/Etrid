//! Integration Tests for Ëtrid Bridge System
//!
//! Tests all 12 bridge pallets across 3 configuration groups:
//! - Group A (Authority-based): BTC, ADA
//! - Group B (Fee-based): ETH, XLM, XRP, BNB, TRX, LINK, SOL, SC-USDT
//! - Group C (PalletId-based): DOGE, MATIC

pub mod bridge_tests;
pub mod common;

// Re-export test utilities
pub use common::*;
