# ĒTRID Integration Test Framework - Implementation Guide

**For:** External Developer
**Purpose:** Implement comprehensive integration tests for the complete ĒTRID contract system
**Estimated Time:** 8-12 hours
**Difficulty:** Intermediate

---

## Overview

You will create integration tests that verify the complete ĒTRID contract ecosystem works end-to-end. Unlike unit tests (which test individual contracts), integration tests verify that multiple contracts work together correctly.

**What you're testing:**
- 47 deployed contracts working together
- Multi-step flows (e.g., BTC → wBTC → ÉTR)
- Cross-contract interactions
- Permission enforcement
- Error handling across boundaries

**Technology Stack:**
- Rust + ink! e2e testing framework
- cargo-contract for deployment
- Substrate node (substrate-contracts-node)

---

## Prerequisites

### Install Required Tools

```bash
# Install substrate-contracts-node (lightweight node for testing)
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git

# Install cargo-contract (if not already installed)
cargo install cargo-contract --force

# Verify installations
contracts-node --version
cargo contract --version
```

### Start Test Node

```bash
# In a separate terminal, start the test node
contracts-node --dev --tmp

# Should output: "Running in --dev mode, RPC CORS has been disabled."
# Node will run on ws://127.0.0.1:9944
```

---

## File Structure to Create

```
contracts/
└── tests/
    ├── README.md                          ← Test documentation
    ├── Cargo.toml                         ← Workspace for all tests
    │
    ├── common/                            ← Shared test utilities
    │   ├── mod.rs
    │   ├── setup.rs                       ← Deploy all contracts
    │   ├── helpers.rs                     ← Helper functions
    │   └── constants.rs                   ← Test constants
    │
    ├── integration/
    │   ├── test_two_tier_flow.rs          ← Test 1 (Priority 1)
    │   ├── test_edsc_minting.rs           ← Test 2 (Priority 1)
    │   ├── test_intent_router.rs          ← Test 3 (Priority 1)
    │   ├── test_bridge_attestation.rs     ← Test 4 (Priority 2)
    │   ├── test_permissions.rs            ← Test 5 (Priority 2)
    │   ├── test_edge_cases.rs             ← Test 6 (Priority 3)
    │   └── test_performance.rs            ← Test 7 (Priority 3)
    │
    └── e2e/
        ├── test_full_user_flow.rs         ← Complete user journey
        └── test_multi_user.rs             ← Concurrent operations
```

---

## Step-by-Step Implementation

### Step 1: Create Test Workspace

**File:** `contracts/tests/Cargo.toml`

```toml
[workspace]
members = [
    "integration",
    "e2e",
]

[workspace.package]
version = "0.1.0"
authors = ["ĒTRID Team"]
edition = "2021"

[workspace.dependencies]
ink = { version = "5.0.0", default-features = false }
ink_e2e = "5.0.0"
scale = { package = "parity-scale-codec", version = "3" }
scale-info = { version = "2.6" }

# Test dependencies
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

### Step 2: Create Common Test Utilities

**File:** `contracts/tests/common/mod.rs`

```rust
pub mod setup;
pub mod helpers;
pub mod constants;

pub use setup::*;
pub use helpers::*;
pub use constants::*;
```

**File:** `contracts/tests/common/constants.rs`

```rust
/// Test constants used across all integration tests

// Amounts (18 decimals)
pub const ONE_ETR: u128 = 1_000_000_000_000_000_000;
pub const ONE_BTC: u128 = 100_000_000; // 8 decimals
pub const ONE_USDC: u128 = 1_000_000; // 6 decimals

// Test accounts
pub const ALICE: &str = "//Alice";
pub const BOB: &str = "//Bob";
pub const CHARLIE: &str = "//Charlie";

// Pool allocations (from WIRING_CONFIGURATION.md)
pub const BTC_ETR_ALLOCATION: u128 = 845_750_000 * ONE_ETR;
pub const BTC_VIRTUAL_RESERVE: u128 = 3383 * ONE_BTC;

pub const ETH_ETR_ALLOCATION: u128 = 191_400_000 * ONE_ETR;
pub const ETH_VIRTUAL_RESERVE: u128 = 95_700_000_000_000_000_000;

// Timeouts
pub const BLOCK_TIME: u64 = 6000; // 6 seconds
pub const DEFAULT_TIMEOUT: u64 = 60 * BLOCK_TIME; // 60 blocks

// Fees
pub const SWAP_FEE_BPS: u128 = 30; // 0.3%
pub const PLATFORM_FEE_BPS: u128 = 30; // 0.3%
```

**File:** `contracts/tests/common/setup.rs`

```rust
use ink_e2e::ContractsBackend;
use anyhow::Result;

/// Deployed contract addresses
#[derive(Clone, Debug)]
pub struct DeployedContracts {
    pub address_registry: ink_e2e::AccountId,
    pub wrapped_tokens: std::collections::HashMap<String, ink_e2e::AccountId>,
    pub tier1_pools: std::collections::HashMap<String, ink_e2e::AccountId>,
    pub tier2_pools: std::collections::HashMap<String, ink_e2e::AccountId>,
    pub edsc_system: EdscSystem,
    pub intent_router_system: IntentRouterSystem,
}

#[derive(Clone, Debug)]
pub struct EdscSystem {
    pub edsc_token: ink_e2e::AccountId,
    pub reserve_vault: ink_e2e::AccountId,
    pub minting_engine: ink_e2e::AccountId,
    pub peg_stabilizer: ink_e2e::AccountId,
    pub external_swap_router: ink_e2e::AccountId,
}

#[derive(Clone, Debug)]
pub struct IntentRouterSystem {
    pub intent_router: ink_e2e::AccountId,
    pub auto_swap_executor: ink_e2e::AccountId,
    pub two_tier_bridge_router: ink_e2e::AccountId,
    pub stablecoin_router: ink_e2e::AccountId,
}

/// Deploy all contracts in the correct order
/// This mirrors the deploy.sh script but in Rust
pub async fn deploy_all_contracts<Client: ContractsBackend>(
    client: &mut ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<DeployedContracts> {
    tracing::info!("Starting full contract deployment...");

    // Phase 1: Foundation
    let address_registry = deploy_address_registry(client).await?;
    let wrapped_tokens = deploy_wrapped_tokens(client).await?;
    register_wrapped_tokens(client, &address_registry, &wrapped_tokens).await?;

    // Phase 2: Tier 1 Pools
    let tier1_pools = deploy_tier1_pools(client, &wrapped_tokens).await?;
    grant_minter_roles(client, &wrapped_tokens, &tier1_pools).await?;
    register_tier1_pools(client, &address_registry, &tier1_pools).await?;

    // Phase 3: Tier 2 Pools
    let tier2_pools = deploy_tier2_pools(client, &wrapped_tokens).await?;
    initialize_tier2_pools(client, &tier2_pools).await?;
    wire_tier1_to_tier2(client, &tier1_pools, &tier2_pools).await?;
    register_tier2_pools(client, &address_registry, &tier2_pools).await?;

    // Phase 4: EDSC System
    let edsc_system = deploy_edsc_system(client).await?;
    seed_edsc_reserves(client, &edsc_system).await?;
    register_edsc_system(client, &address_registry, &edsc_system).await?;

    // Phase 5: Intent Router System
    let intent_router_system = deploy_intent_router_system(client, &edsc_system).await?;
    grant_router_permissions(client, &intent_router_system, &tier2_pools, &edsc_system).await?;
    register_intent_router_system(client, &address_registry, &intent_router_system).await?;

    tracing::info!("All contracts deployed successfully");

    Ok(DeployedContracts {
        address_registry,
        wrapped_tokens,
        tier1_pools,
        tier2_pools,
        edsc_system,
        intent_router_system,
    })
}

// TODO: Implement each deployment function
// See contracts/scripts/deploy_phase*.sh for reference

async fn deploy_address_registry<Client: ContractsBackend>(
    client: &mut ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<ink_e2e::AccountId> {
    // 1. Build address-registry contract
    // 2. Instantiate with new() constructor
    // 3. Return contract address
    todo!("Implement address registry deployment")
}

async fn deploy_wrapped_tokens<Client: ContractsBackend>(
    client: &mut ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<std::collections::HashMap<String, ink_e2e::AccountId>> {
    // 1. Deploy 11 wrapped tokens (wBTC, wETH, etc.)
    // 2. For each: instantiate with new(name, symbol, decimals)
    // 3. Return HashMap of symbol -> address
    todo!("Implement wrapped token deployment")
}

// TODO: Implement remaining deployment functions
// Reference: contracts/scripts/deploy_phase*.sh
```

**File:** `contracts/tests/common/helpers.rs`

```rust
use ink_e2e::ContractsBackend;
use anyhow::Result;

/// Helper to call a contract method
pub async fn call_contract<Client: ContractsBackend>(
    client: &mut ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
    contract: &ink_e2e::AccountId,
    message: &str,
    args: Vec<String>,
    caller: &str,
) -> Result<Vec<u8>> {
    // Use cargo-contract call API
    // This is a wrapper to make tests cleaner
    todo!("Implement contract call helper")
}

/// Helper to assert balance
pub async fn assert_balance<Client: ContractsBackend>(
    client: &mut ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
    token: &ink_e2e::AccountId,
    account: &str,
    expected: u128,
) -> Result<()> {
    // 1. Call token.balance_of(account)
    // 2. Assert equals expected
    todo!("Implement balance assertion")
}

/// Helper to calculate expected swap output
pub fn calculate_swap_output(
    amount_in: u128,
    reserve_in: u128,
    reserve_out: u128,
    fee_bps: u128,
) -> u128 {
    // Constant product formula with fee
    // amount_out = (amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)
    let amount_with_fee = amount_in * (10000 - fee_bps) / 10000;
    let numerator = amount_with_fee * reserve_out;
    let denominator = reserve_in + amount_with_fee;
    numerator / denominator
}

/// Helper to wait for blocks
pub async fn wait_blocks(n: u32) {
    tokio::time::sleep(tokio::time::Duration::from_millis(n as u64 * 6000)).await;
}
```

---

### Step 3: Implement Priority 1 Tests

**File:** `contracts/tests/integration/Cargo.toml`

```toml
[package]
name = "etrid-integration-tests"
version.workspace = true
authors.workspace = true
edition.workspace = true

[dependencies]
ink.workspace = true
ink_e2e.workspace = true
scale.workspace = true
tokio.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[dev-dependencies]
ink_e2e = "5.0.0"

[[test]]
name = "integration_tests"
path = "lib.rs"
harness = true
```

**File:** `contracts/tests/integration/test_two_tier_flow.rs`

```rust
//! Test 1: Two-Tier Liquidity Flow
//!
//! Tests the complete flow:
//! 1. External BTC locked in Tier 1 pool
//! 2. wBTC minted 1:1
//! 3. wBTC swapped for ÉTR in Tier 2 pool
//! 4. User receives ÉTR
//! 5. Reverse flow: ÉTR → wBTC → BTC

use super::common::*;

#[ink_e2e::test]
async fn test_btc_to_etr_flow<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    // Setup: Deploy all contracts
    let contracts = deploy_all_contracts(&mut client).await?;

    // Step 1: User locks 1 BTC in Tier 1 pool
    let lock_amount = ONE_BTC;
    call_contract(
        &mut client,
        &contracts.tier1_pools["BTC"],
        "lock_and_mint",
        vec![lock_amount.to_string()],
        ALICE,
    ).await?;

    // Verify: wBTC minted 1:1
    assert_balance(
        &mut client,
        &contracts.wrapped_tokens["wBTC"],
        ALICE,
        lock_amount,
    ).await?;

    // Step 2: Tier 1 pool forwards wBTC to Tier 2 pool
    // (This happens automatically in the contract)

    // Step 3: Tier 2 pool swaps wBTC → ÉTR
    let expected_etr_out = calculate_swap_output(
        lock_amount,
        BTC_VIRTUAL_RESERVE,
        BTC_ETR_ALLOCATION,
        SWAP_FEE_BPS,
    );

    call_contract(
        &mut client,
        &contracts.tier2_pools["BTC"],
        "swap_wrapped_for_etr",
        vec![lock_amount.to_string(), expected_etr_out.to_string()],
        ALICE,
    ).await?;

    // Verify: User received ÉTR (minus fees)
    assert_balance(
        &mut client,
        &contracts.etr_token,
        ALICE,
        expected_etr_out,
    ).await?;

    // Step 4: Verify Tier 1 pool still has BTC locked
    let tier1_reserves = call_contract(
        &mut client,
        &contracts.tier1_pools["BTC"],
        "get_total_reserves",
        vec![],
        ALICE,
    ).await?;
    assert_eq!(decode_u128(&tier1_reserves)?, lock_amount);

    Ok(())
}

#[ink_e2e::test]
async fn test_etr_to_btc_flow<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    // Setup
    let contracts = deploy_all_contracts(&mut client).await?;

    // TODO: Implement reverse flow
    // 1. User has ÉTR
    // 2. Swap ÉTR → wBTC in Tier 2
    // 3. Burn wBTC in Tier 1
    // 4. Release BTC to user

    Ok(())
}

// TODO: Add more test cases:
// - test_multiple_currencies() - Test all 11 currencies
// - test_slippage_protection() - Ensure swaps fail if output < min
// - test_deadline_enforcement() - Ensure swaps fail after deadline
```

**File:** `contracts/tests/integration/test_edsc_minting.rs`

```rust
//! Test 2: EDSC Transaction-Driven Minting
//!
//! Tests:
//! 1. User purchases EDSC with USDC
//! 2. USDC deposited to reserve vault
//! 3. EDSC minted 1:1
//! 4. Reserve ratio maintained at 100%

use super::common::*;

#[ink_e2e::test]
async fn test_usdc_to_edsc_minting<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    let contracts = deploy_all_contracts(&mut client).await?;

    // Step 1: User routes USDC purchase to EDSC minting engine
    let usdc_amount = 1000 * ONE_USDC; // 1000 USDC
    call_contract(
        &mut client,
        &contracts.edsc_system.minting_engine,
        "route_stablecoin_purchase",
        vec![
            "1".to_string(), // source_pbc (BTC-PBC)
            ALICE.to_string(),
            contracts.usdc_token.to_string(),
            usdc_amount.to_string(),
        ],
        ALICE,
    ).await?;

    // Verify: USDC deposited to reserve vault
    let vault_usdc = call_contract(
        &mut client,
        &contracts.edsc_system.reserve_vault,
        "get_usdc_balance",
        vec![],
        ALICE,
    ).await?;
    assert_eq!(decode_u128(&vault_usdc)?, 50_000_000 * ONE_USDC + usdc_amount);

    // Verify: EDSC minted 1:1 (assuming 1 USDC = 1 EDSC)
    assert_balance(
        &mut client,
        &contracts.edsc_system.edsc_token,
        ALICE,
        usdc_amount * 1_000_000_000_000, // Convert 6 decimals to 18 decimals
    ).await?;

    // Verify: Reserve ratio still 100%
    let reserve_ratio = call_contract(
        &mut client,
        &contracts.edsc_system.reserve_vault,
        "get_reserve_ratio",
        vec![],
        ALICE,
    ).await?;
    assert_eq!(decode_u128(&reserve_ratio)?, 100);

    Ok(())
}

#[ink_e2e::test]
async fn test_btc_to_edsc_with_external_swap<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    // TODO: Test BTC → USDC (via 1inch) → EDSC minting
    // This tests the external_swap_router integration
    Ok(())
}

// TODO: Add more test cases:
// - test_cross_pbc_routing() - Route from all 14 PBCs
// - test_reserve_rebalancing() - Trigger rebalance when allocation drifts >5%
// - test_peg_stabilization() - Test mint/burn to maintain $1 peg
```

**File:** `contracts/tests/integration/test_intent_router.rs`

```rust
//! Test 3: Intent Router User Abstraction
//!
//! Tests:
//! 1. User calls single function: convertToEtr(BTC, amount)
//! 2. Router orchestrates: Bridge → Tier1 → Tier2
//! 3. User receives ÉTR without seeing wBTC

use super::common::*;

#[ink_e2e::test]
async fn test_intent_router_btc_to_etr<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    let contracts = deploy_all_contracts(&mut client).await?;

    // User balance before
    let etr_before = get_balance(&mut client, &contracts.etr_token, ALICE).await?;

    // Single call to Intent Router
    let btc_amount = ONE_BTC;
    let min_etr_out = calculate_swap_output(
        btc_amount,
        BTC_VIRTUAL_RESERVE,
        BTC_ETR_ALLOCATION,
        SWAP_FEE_BPS + PLATFORM_FEE_BPS,
    );

    call_contract(
        &mut client,
        &contracts.intent_router_system.intent_router,
        "convert_to_etr",
        vec![
            contracts.wrapped_tokens["wBTC"].to_string(), // source_currency
            btc_amount.to_string(),
            min_etr_out.to_string(),
            (now() + DEFAULT_TIMEOUT).to_string(), // deadline
        ],
        ALICE,
    ).await?;

    // User balance after
    let etr_after = get_balance(&mut client, &contracts.etr_token, ALICE).await?;

    // Verify: User received ÉTR
    assert!(etr_after > etr_before);
    assert!(etr_after >= etr_before + min_etr_out);

    // Verify: User never received wBTC (it was hidden)
    let wbtc_balance = get_balance(
        &mut client,
        &contracts.wrapped_tokens["wBTC"],
        ALICE,
    ).await?;
    assert_eq!(wbtc_balance, 0);

    Ok(())
}

// TODO: Add more test cases:
// - test_intent_router_slippage_protection()
// - test_intent_router_deadline_expired()
// - test_intent_router_multi_hop() - Route through multiple pools
```

---

### Step 4: Implement Priority 2 Tests

**File:** `contracts/tests/integration/test_bridge_attestation.rs`

```rust
//! Test 4: Bridge Attestation
//!
//! Tests:
//! 1. External deposit detected by relayers
//! 2. M-of-N signatures collected (3-of-5 or 5-of-9)
//! 3. Deposit verified
//! 4. Tier 1 pool unlocks

use super::common::*;

#[ink_e2e::test]
async fn test_bridge_deposit_verification<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    // TODO: Simulate bridge deposit
    // 1. Create mock deposit message
    // 2. Collect 3 signatures from validators
    // 3. Submit to bridge_tracker
    // 4. Verify deposit marked as verified
    Ok(())
}

// TODO: Add more test cases:
// - test_insufficient_signatures() - Fails with 2-of-5
// - test_invalid_signature() - Rejects bad signature
// - test_replay_attack_prevention() - Same deposit can't be verified twice
```

**File:** `contracts/tests/integration/test_permissions.rs`

```rust
//! Test 5: Permission Enforcement
//!
//! Tests:
//! 1. Only authorized callers can execute restricted functions
//! 2. Unauthorized calls are rejected
//! 3. Role grants/revokes work correctly

use super::common::*;

#[ink_e2e::test]
async fn test_minter_role_enforcement<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    let contracts = deploy_all_contracts(&mut client).await?;

    // Tier 1 pool should have MINTER_ROLE on wBTC
    let result = call_contract(
        &mut client,
        &contracts.wrapped_tokens["wBTC"],
        "mint",
        vec![ALICE.to_string(), ONE_BTC.to_string()],
        &contracts.tier1_pools["BTC"].to_string(),
    ).await;
    assert!(result.is_ok());

    // Random account should NOT have MINTER_ROLE
    let result = call_contract(
        &mut client,
        &contracts.wrapped_tokens["wBTC"],
        "mint",
        vec![ALICE.to_string(), ONE_BTC.to_string()],
        BOB,
    ).await;
    assert!(result.is_err());

    Ok(())
}

// TODO: Add more test cases:
// - test_caller_role_enforcement() - Only IntentRouter can call AutoSwapExecutor
// - test_admin_role_enforcement() - Only admin can pause
// - test_ownership_transfer() - Transfer to multi-sig
```

---

### Step 5: Implement Priority 3 Tests

**File:** `contracts/tests/integration/test_edge_cases.rs`

```rust
//! Test 6: Edge Cases & Error Handling
//!
//! Tests:
//! 1. Zero amount swaps
//! 2. Overflow/underflow protection
//! 3. Reentrancy protection
//! 4. Circuit breaker activation

// TODO: Implement edge case tests
// - test_zero_amount_swap()
// - test_overflow_protection()
// - test_paused_contract()
// - test_circuit_breaker()
```

**File:** `contracts/tests/integration/test_performance.rs`

```rust
//! Test 7: Performance & Load Testing
//!
//! Tests:
//! 1. Gas costs for common operations
//! 2. Pool performance with large reserves
//! 3. Batch operations

// TODO: Implement performance tests
// - test_swap_gas_cost()
// - test_large_swap_performance()
// - test_batch_operations()
```

---

### Step 6: Implement E2E Tests

**File:** `contracts/tests/e2e/test_full_user_flow.rs`

```rust
//! End-to-End Test: Complete User Journey
//!
//! Simulates a real user:
//! 1. Deposits BTC via bridge
//! 2. Swaps BTC → ÉTR
//! 3. Purchases EDSC with ÉTR
//! 4. Swaps EDSC back to ÉTR
//! 5. Swaps ÉTR → BTC
//! 6. Withdraws BTC via bridge

use super::common::*;

#[ink_e2e::test]
async fn test_complete_user_journey<Client: ContractsBackend>(
    mut client: ink_e2e::Client<ink_e2e::PolkadotConfig, Client>,
) -> Result<()> {
    // TODO: Implement complete multi-step user journey
    // This should take 5-10 minutes to run
    Ok(())
}
```

**File:** `contracts/tests/e2e/test_multi_user.rs`

```rust
//! End-to-End Test: Multiple Users Concurrently
//!
//! Tests:
//! 1. Alice swaps BTC → ÉTR
//! 2. Bob swaps ETH → ÉTR (same time)
//! 3. Charlie purchases EDSC (same time)
//! 4. All operations succeed without conflicts

// TODO: Implement concurrent user tests
// Use tokio::spawn to run operations in parallel
```

---

## Test Execution Guide

### Run All Tests

```bash
cd contracts/tests

# Run integration tests
cargo test --package etrid-integration-tests

# Run e2e tests
cargo test --package etrid-e2e-tests

# Run all tests
cargo test --all

# Run with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_btc_to_etr_flow -- --nocapture
```

### Expected Results

**All tests should pass:**
```
test test_two_tier_flow::test_btc_to_etr_flow ... ok
test test_two_tier_flow::test_etr_to_btc_flow ... ok
test test_edsc_minting::test_usdc_to_edsc_minting ... ok
test test_intent_router::test_intent_router_btc_to_etr ... ok
test test_bridge_attestation::test_bridge_deposit_verification ... ok
test test_permissions::test_minter_role_enforcement ... ok

test result: ok. 20 passed; 0 failed; 0 ignored
```

---

## Priority Implementation Order

### Phase 1 (Critical - Implement First)
1. `common/setup.rs` - Contract deployment functions
2. `common/helpers.rs` - Test helper functions
3. `test_two_tier_flow.rs` - Basic liquidity flow
4. `test_edsc_minting.rs` - Stablecoin minting
5. `test_intent_router.rs` - User abstraction

**Estimated Time:** 4-6 hours

### Phase 2 (Important - Implement Second)
6. `test_bridge_attestation.rs` - Bridge security
7. `test_permissions.rs` - Access control
8. `test_edge_cases.rs` - Error handling

**Estimated Time:** 2-3 hours

### Phase 3 (Nice to Have - Implement Last)
9. `test_performance.rs` - Performance benchmarks
10. `test_full_user_flow.rs` - Complete E2E
11. `test_multi_user.rs` - Concurrent operations

**Estimated Time:** 2-3 hours

---

## Success Criteria

### Tests Must Verify

✅ **Functional Requirements:**
- [ ] BTC → wBTC → ÉTR flow works end-to-end
- [ ] Reverse flow (ÉTR → wBTC → BTC) works
- [ ] EDSC minting maintains 1:1 reserve ratio
- [ ] Intent Router hides wrapped tokens from users
- [ ] Bridge attestation requires M-of-N signatures
- [ ] All permissions enforced correctly

✅ **Non-Functional Requirements:**
- [ ] Gas costs within acceptable limits
- [ ] No reentrancy vulnerabilities
- [ ] Overflow/underflow protection works
- [ ] Circuit breakers trigger correctly
- [ ] Performance acceptable under load

✅ **Coverage:**
- [ ] >80% code coverage across all contracts
- [ ] All critical paths tested
- [ ] All error cases tested
- [ ] All permission checks tested

---

## Common Issues & Solutions

### Issue 1: Contract Deployment Fails
**Solution:** Check that substrate-contracts-node is running and accessible at ws://127.0.0.1:9944

### Issue 2: Test Times Out
**Solution:** Increase timeout in test or optimize contract deployment

### Issue 3: Balance Assertions Fail
**Solution:** Check decimal conversions (BTC=8, USDC=6, ÉTR=18 decimals)

### Issue 4: Permission Denied Errors
**Solution:** Verify roles were granted correctly in setup phase

---

## Documentation to Reference

1. **Architecture Docs:**
   - `contracts/TWO_TIER_ARCHITECTURE.md`
   - `contracts/EDSC_RESERVE_ARCHITECTURE.md`
   - `contracts/INTENT_ROUTER_ARCHITECTURE.md`

2. **Wiring Specs:**
   - `contracts/WIRING_CONFIGURATION.md` - Permission matrix
   - `contracts/INTEGRATION_PLAN.md` - Deployment order

3. **Deployment Scripts:**
   - `contracts/scripts/deploy_phase*.sh` - Deployment reference
   - `contracts/scripts/README.md` - Deployment guide

4. **Contract Source:**
   - `contracts/primeswap/` - Pool contracts
   - `contracts/edsc/` - Stablecoin contracts
   - `contracts/intent-router/` - Router contracts

---

## Questions?

If you get stuck, refer to:
1. ink! e2e testing docs: https://use.ink/basics/contract-testing
2. Existing unit tests in each contract's `lib.rs`
3. Deployment scripts in `contracts/scripts/`

**Contact:** Open an issue on GitHub or ask in Discord

---

## Deliverables

When complete, you should have:

1. ✅ All test files implemented
2. ✅ `cargo test --all` passes
3. ✅ README.md documenting how to run tests
4. ✅ Test coverage report
5. ✅ Any discovered bugs documented as GitHub issues

**Estimated Total Time:** 8-12 hours

Good luck! 🚀
