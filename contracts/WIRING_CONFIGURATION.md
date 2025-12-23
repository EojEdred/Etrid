# ĒTRID Contract Wiring Configuration

**Date:** December 9, 2025
**Status:** Deployment Configuration
**Purpose:** Defines all contract interconnections, permissions, and deployment order

---

## Table of Contents

1. [Deployment Order](#deployment-order)
2. [Permission Matrix](#permission-matrix)
3. [Role Assignments](#role-assignments)
4. [Contract Dependencies](#contract-dependencies)
5. [Configuration Parameters](#configuration-parameters)
6. [Verification Checklist](#verification-checklist)

---

## Deployment Order

**CRITICAL:** Contracts must be deployed in this exact order to satisfy dependencies.

### Phase 1: Foundation Layer
Deploy contracts with no dependencies first.

```
1. Address Registry           (No dependencies)
2. 11x Wrapped Tokens         (No dependencies)
   - wBTC, wETH, wSOL, wBNB, wTRX
   - wXRP, wADA, wDOGE, wLINK, wXLM, wMATIC
```

**Actions after Phase 1:**
```bash
# Register all wrapped tokens in Address Registry
registry.register_all_wrapped_tokens(
    ["wBTC", "wETH", "wSOL", "wBNB", "wTRX", "wXRP", "wADA", "wDOGE", "wLINK", "wXLM", "wMATIC"],
    [wbtc_addr, weth_addr, wsol_addr, wbnb_addr, wtrx_addr, wxrp_addr, wada_addr, wdoge_addr, wlink_addr, wxlm_addr, wmatic_addr]
)
```

### Phase 2: Tier 1 Reserve Pools
Deploy Tier 1 pools that depend on wrapped tokens.

```
3. 11x Tier 1 Pools (ExternalCurrencyPool)
   Depends on: Wrapped Token addresses

   For each currency (BTC, ETH, SOL, etc.):
   - Deploy ExternalCurrencyPool(
       currency_name,
       wrapped_token_address,    // From Address Registry
       tier2_pool_address,       // Will be set in Phase 3
       multisig_address,
       max_tx_limit,
       daily_withdrawal_limit
     )
```

**Actions after Phase 2:**
```bash
# Register all Tier 1 pools
registry.register_all_tier1_pools(
    ["BTC", "ETH", "SOL", "BNB", "TRX", "XRP", "ADA", "DOGE", "LINK", "XLM", "MATIC"],
    [btc_t1_addr, eth_t1_addr, sol_t1_addr, bnb_t1_addr, trx_t1_addr, xrp_t1_addr, ada_t1_addr, doge_t1_addr, link_t1_addr, xlm_t1_addr, matic_t1_addr]
)

# Grant MINTER role to each Tier 1 pool on its wrapped token
wBTC.grant_role(MINTER_ROLE, btc_tier1_pool)
wETH.grant_role(MINTER_ROLE, eth_tier1_pool)
# ... repeat for all 11 currencies
```

### Phase 3: Tier 2 Trading Pools
Deploy Tier 2 pools and wire to Tier 1.

```
4. 11x Tier 2 Pools (ETRWrappedPool)
   Depends on: Wrapped Token addresses, ÉTR token

   For each currency:
   - Deploy ETRWrappedPool(
       wrapped_token_address,
       etr_allocation,           // From pool allocation table
       virtual_reserve_amount    // From pool allocation table
     )
   - Initialize pool with ÉTR liquidity
```

**Pool Allocations:**
| Currency | ÉTR Allocation | Virtual Reserve |
|----------|---------------|-----------------|
| wBTC | 845,750,000 ÉTR | 33.83 BTC |
| wETH | 191,400,000 ÉTR | 95.7 ETH |
| wXRP | 62,400,000 ÉTR | 312,000 XRP |
| wSOL | 44,500,000 ÉTR | 2,225 SOL |
| wBNB | 40,000,000 ÉTR | 160 BNB |
| wDOGE | 26,800,000 ÉTR | 2,680,000 DOGE |
| wADA | 15,600,000 ÉTR | 78,000 ADA |
| wLINK | 8,900,000 ÉTR | 890 LINK |
| wTRX | 6,600,000 ÉTR | 660,000 TRX |
| wXLM | 4,500,000 ÉTR | 45,000 XLM |
| wMATIC | 3,500,000 ÉTR | 7,000 MATIC |

**Actions after Phase 3:**
```bash
# Register all Tier 2 pools
registry.register_all_tier2_pools(
    ["BTC", "ETH", "SOL", "BNB", "TRX", "XRP", "ADA", "DOGE", "LINK", "XLM", "MATIC"],
    [btc_t2_addr, eth_t2_addr, sol_t2_addr, bnb_t2_addr, trx_t2_addr, xrp_t2_addr, ada_t2_addr, doge_t2_addr, link_t2_addr, xlm_t2_addr, matic_t2_addr]
)

# Wire Tier 1 → Tier 2 connections
btc_tier1_pool.set_tier2_pool(btc_tier2_pool)
eth_tier1_pool.set_tier2_pool(eth_tier2_pool)
# ... repeat for all 11 currencies

# Initialize Tier 2 pools with ÉTR
transfer_etr(btc_tier2_pool, 845_750_000 * UNITS)
btc_tier2_pool.initialize_pool(845_750_000 * UNITS)
# ... repeat for all 11 currencies
```

### Phase 4: EDSC Stablecoin System
Deploy EDSC reserve infrastructure.

```
5. EDSCToken                  (No dependencies)
6. EDSCReserveVault           (Depends on: stablecoin addresses)
7. EDSCMintingEngine          (Depends on: EDSCToken, ReserveVault)
8. EDSCPegStabilizer          (Depends on: EDSCToken, oracle)
9. EDSCExternalSwapRouter     (Depends on: 1inch/ParaSwap API)
```

**Actions after Phase 4:**
```bash
# Register EDSC system
registry.register_edsc_token(edsc_token_addr)
registry.register_edsc_reserve_vault(reserve_vault_addr)
registry.register_edsc_minting_engine(minting_engine_addr)
registry.register_edsc_peg_stabilizer(peg_stabilizer_addr)
registry.register_edsc_external_swap_router(external_swap_router_addr)

# Grant MINTER role to minting engine
edsc_token.grant_role(MINTER_ROLE, minting_engine_addr)

# Seed initial reserves
reserve_vault.deposit_usdc(50_000_000 * UNITS)  // 50M USDC
reserve_vault.deposit_usdt(30_000_000 * UNITS)  // 30M USDT
reserve_vault.deposit_dai(20_000_000 * UNITS)   // 20M DAI

# Mint initial EDSC supply
minting_engine.mint_initial_supply(100_000_000 * UNITS)  // 100M EDSC

# Verify 1:1 backing
assert_eq!(reserve_vault.get_reserve_ratio(), 100)
```

### Phase 5: Intent Router System
Deploy user-facing abstraction layer.

```
10. TwoTierBridgeRouter       (Depends on: Tier1 pools, Tier2 pools, Bridge pallets)
11. AutoSwapExecutor          (Depends on: TwoTierBridgeRouter)
12. StablecoinRouter          (Depends on: EDSC system, XCMP)
13. IntentRouter              (Depends on: All routers)
```

**Actions after Phase 5:**
```bash
# Register Intent Router system
registry.register_two_tier_bridge_router(bridge_router_addr)
registry.register_auto_swap_executor(auto_swap_executor_addr)
registry.register_stablecoin_router(stablecoin_router_addr)
registry.register_intent_router(intent_router_addr)

# Wire AutoSwapExecutor
auto_swap_executor.set_bridge_router(bridge_router_addr)
auto_swap_executor.set_address_registry(registry_addr)

# Wire IntentRouter
intent_router.set_auto_swap_executor(auto_swap_executor_addr)
intent_router.set_stablecoin_router(stablecoin_router_addr)
intent_router.set_address_registry(registry_addr)

# Grant CALLER permissions
auto_swap_executor.grant_role(CALLER_ROLE, intent_router_addr)
bridge_router.grant_role(CALLER_ROLE, auto_swap_executor_addr)
```

### Phase 6: Bridge Infrastructure (Substrate Pallets)
Deploy bridge tracking pallets to Primearc runtime.

```
14. pallet-bridge-tracker      (Runtime upgrade)
15. pallet-state-verifier      (Runtime upgrade)
```

**Actions after Phase 6:**
```bash
# Configure bridge validators (multi-sig 3-of-5 or 5-of-9)
bridge_tracker.add_authorized_validator(validator1_addr)
bridge_tracker.add_authorized_validator(validator2_addr)
bridge_tracker.add_authorized_validator(validator3_addr)
bridge_tracker.add_authorized_validator(validator4_addr)
bridge_tracker.add_authorized_validator(validator5_addr)

# Set signature threshold
bridge_tracker.set_signature_threshold(3)  // 3-of-5

# Wire bridge → Tier 1 pools
two_tier_bridge_router.configure_bridge_pallet(bridge_tracker_pallet_id)
```

---

## Permission Matrix

Defines who can call what across the entire system.

### Wrapped Tokens (wBTC, wETH, etc.)

| Function | Caller | Permission |
|----------|--------|------------|
| `mint()` | Tier 1 Pool | MINTER_ROLE |
| `burn()` | Tier 1 Pool | BURNER_ROLE |
| `transfer()` | Any user | PUBLIC |
| `approve()` | Any user | PUBLIC |

### Tier 1 Pools (ExternalCurrencyPool)

| Function | Caller | Permission |
|----------|--------|------------|
| `lock_and_mint()` | TwoTierBridgeRouter | CALLER_ROLE |
| `burn_and_release()` | TwoTierBridgeRouter | CALLER_ROLE |
| `pause()` | Multi-sig wallet | ADMIN_ROLE |
| `set_tier2_pool()` | Deployer (once) | OWNER |

### Tier 2 Pools (ETRWrappedPool)

| Function | Caller | Permission |
|----------|--------|------------|
| `initialize_pool()` | Deployer (once) | OWNER |
| `swap_etr_for_wrapped()` | AutoSwapExecutor | CALLER_ROLE |
| `swap_wrapped_for_etr()` | AutoSwapExecutor | CALLER_ROLE |
| `add_liquidity()` | Protocol only | RESTRICTED |
| `pause()` | Multi-sig wallet | ADMIN_ROLE |

### EDSC System

| Function | Caller | Permission |
|----------|--------|------------|
| `edsc_token.mint()` | EDSCMintingEngine | MINTER_ROLE |
| `edsc_token.burn()` | EDSCPegStabilizer | BURNER_ROLE |
| `reserve_vault.deposit()` | EDSCMintingEngine | DEPOSITOR_ROLE |
| `reserve_vault.rebalance()` | EDSCMintingEngine | REBALANCER_ROLE |
| `minting_engine.route_purchase()` | StablecoinRouter | CALLER_ROLE |
| `peg_stabilizer.stabilize()` | Automated (cron) | PUBLIC |

### Intent Router System

| Function | Caller | Permission |
|----------|--------|------------|
| `intent_router.convert_to_etr()` | Any user | PUBLIC |
| `intent_router.convert_from_etr()` | Any user | PUBLIC |
| `auto_swap_executor.execute_swap()` | IntentRouter | CALLER_ROLE |
| `bridge_router.route_message()` | AutoSwapExecutor | CALLER_ROLE |
| `stablecoin_router.route_to_edsc()` | IntentRouter | CALLER_ROLE |

### Bridge Pallets

| Function | Caller | Permission |
|----------|--------|------------|
| `verify_deposit()` | Authorized validators | VALIDATOR_ROLE |
| `submit_attestation()` | Authorized validators | VALIDATOR_ROLE |
| `add_validator()` | Governance | ROOT |
| `set_threshold()` | Governance | ROOT |

---

## Role Assignments

Complete list of all roles across all contracts.

### Wrapped Tokens

**Contract:** wBTC, wETH, wSOL, wBNB, wTRX, wXRP, wADA, wDOGE, wLINK, wXLM, wMATIC

```rust
// MINTER_ROLE: Can mint new wrapped tokens
wBTC.grant_role(MINTER_ROLE, btc_tier1_pool_addr)

// BURNER_ROLE: Can burn wrapped tokens
wBTC.grant_role(BURNER_ROLE, btc_tier1_pool_addr)

// ADMIN_ROLE: Can grant/revoke roles, pause contract
wBTC.grant_role(ADMIN_ROLE, multisig_wallet_addr)

// DEFAULT_ADMIN_ROLE: Owner (can renounce after setup)
// Initially: deployer_addr
// Final: multisig_wallet_addr or burn to 0x0
```

### Tier 1 Pools

**Contract:** 11x ExternalCurrencyPool

```rust
// CALLER_ROLE: Can call lock_and_mint, burn_and_release
btc_tier1_pool.grant_role(CALLER_ROLE, two_tier_bridge_router_addr)

// ADMIN_ROLE: Can pause, set limits
btc_tier1_pool.grant_role(ADMIN_ROLE, multisig_wallet_addr)

// OWNER: Can set tier2_pool (one-time), transfer ownership
// Initially: deployer_addr
// Final: multisig_wallet_addr
```

### Tier 2 Pools

**Contract:** 11x ETRWrappedPool

```rust
// CALLER_ROLE: Can execute swaps
btc_tier2_pool.grant_role(CALLER_ROLE, auto_swap_executor_addr)

// ADMIN_ROLE: Can pause, adjust fees
btc_tier2_pool.grant_role(ADMIN_ROLE, multisig_wallet_addr)

// OWNER: Can initialize pool, add liquidity
// Initially: deployer_addr
// Final: protocol_treasury_addr (managed by governance)
```

### EDSC System

**Contract:** EDSCToken

```rust
// MINTER_ROLE: Can mint new EDSC
edsc_token.grant_role(MINTER_ROLE, edsc_minting_engine_addr)

// BURNER_ROLE: Can burn EDSC
edsc_token.grant_role(BURNER_ROLE, edsc_peg_stabilizer_addr)

// ADMIN_ROLE: Can pause token
edsc_token.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

**Contract:** EDSCReserveVault

```rust
// DEPOSITOR_ROLE: Can deposit stablecoins
reserve_vault.grant_role(DEPOSITOR_ROLE, edsc_minting_engine_addr)

// REBALANCER_ROLE: Can trigger rebalancing
reserve_vault.grant_role(REBALANCER_ROLE, edsc_minting_engine_addr)

// ADMIN_ROLE: Can pause, set thresholds
reserve_vault.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

**Contract:** EDSCMintingEngine

```rust
// CALLER_ROLE: Can route purchases
minting_engine.grant_role(CALLER_ROLE, stablecoin_router_addr)

// ADMIN_ROLE: Can pause, configure
minting_engine.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

### Intent Router System

**Contract:** IntentRouter

```rust
// ADMIN_ROLE: Can pause, set fees
intent_router.grant_role(ADMIN_ROLE, multisig_wallet_addr)

// PUBLIC: All users can call convert functions (no role needed)
```

**Contract:** AutoSwapExecutor

```rust
// CALLER_ROLE: Can execute swaps
auto_swap_executor.grant_role(CALLER_ROLE, intent_router_addr)

// ADMIN_ROLE: Can pause
auto_swap_executor.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

**Contract:** TwoTierBridgeRouter

```rust
// CALLER_ROLE: Can route messages
bridge_router.grant_role(CALLER_ROLE, auto_swap_executor_addr)

// ADMIN_ROLE: Can pause, configure bridges
bridge_router.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

**Contract:** StablecoinRouter

```rust
// CALLER_ROLE: Can route to EDSC
stablecoin_router.grant_role(CALLER_ROLE, intent_router_addr)

// ADMIN_ROLE: Can pause, configure
stablecoin_router.grant_role(ADMIN_ROLE, multisig_wallet_addr)
```

### Address Registry

```rust
// OWNER: Can register/update addresses, pause
// Initially: deployer_addr
// Final: multisig_wallet_addr

// NOTE: No other roles - only owner can modify registry
```

---

## Contract Dependencies

Visual dependency graph showing which contracts need to know about which others.

```
┌─────────────────────────────────────────────────────────────┐
│                      DEPENDENCY GRAPH                       │
└─────────────────────────────────────────────────────────────┘

AddressRegistry (Central Hub)
    ↓ (queries all addresses)
    │
    ├─→ IntentRouter
    │       ├─→ AutoSwapExecutor
    │       │       ├─→ TwoTierBridgeRouter
    │       │       │       ├─→ Bridge Pallets
    │       │       │       ├─→ Tier 1 Pools
    │       │       │       └─→ Tier 2 Pools
    │       │       └─→ Address Registry
    │       │
    │       └─→ StablecoinRouter
    │               ├─→ EDSCMintingEngine
    │               │       ├─→ EDSCToken
    │               │       ├─→ EDSCReserveVault
    │               │       └─→ EDSCExternalSwapRouter
    │               └─→ XCMP (cross-PBC messaging)
    │
    ├─→ Tier 1 Pools (11x)
    │       ├─→ Wrapped Tokens (mint/burn)
    │       └─→ Tier 2 Pools (forward wrapped tokens)
    │
    ├─→ Tier 2 Pools (11x)
    │       └─→ Wrapped Tokens (transfer)
    │
    └─→ EDSC System
            ├─→ EDSCToken
            ├─→ EDSCReserveVault
            │       └─→ Stablecoins (USDC, USDT, DAI)
            ├─→ EDSCMintingEngine
            ├─→ EDSCPegStabilizer
            │       └─→ Oracle (Chainlink)
            └─→ EDSCExternalSwapRouter
                    └─→ 1inch/ParaSwap API
```

### Required Address Storage

Each contract must store certain addresses:

**IntentRouter:**
```rust
auto_swap_executor: AccountId,
stablecoin_router: AccountId,
address_registry: AccountId,
```

**AutoSwapExecutor:**
```rust
two_tier_bridge_router: AccountId,
address_registry: AccountId,
```

**TwoTierBridgeRouter:**
```rust
bridge_pallet_id: u32,          // Primearc pallet index
address_registry: AccountId,
```

**StablecoinRouter:**
```rust
edsc_minting_engine: AccountId,
xcmp_channel_id: u32,           // EDSC-PBC parachain ID
```

**Tier 1 Pool:**
```rust
wrapped_token: AccountId,       // e.g., wBTC
tier2_pool: AccountId,          // BTC Tier 2 pool
multisig_address: AccountId,
```

**Tier 2 Pool:**
```rust
wrapped_token: AccountId,       // e.g., wBTC
etr_token: AccountId,           // Native ÉTR
```

**EDSCMintingEngine:**
```rust
edsc_token: AccountId,
reserve_vault: AccountId,
external_swap_router: AccountId,
```

**EDSCPegStabilizer:**
```rust
edsc_token: AccountId,
oracle_address: AccountId,      // Chainlink price feed
```

**EDSCReserveVault:**
```rust
usdc_token: AccountId,
usdt_token: AccountId,
dai_token: AccountId,
```

---

## Configuration Parameters

Initial settings for each contract upon deployment.

### Wrapped Tokens

```rust
WrappedToken {
    name: "Wrapped Bitcoin",
    symbol: "wBTC",
    decimals: 8,                // Bitcoin native decimals
    initial_supply: 0,          // Minted on-demand
}
```

**Decimals by Currency:**
- wBTC: 8 decimals
- All others: 18 decimals

### Tier 1 Pools (ExternalCurrencyPool)

```rust
ExternalCurrencyPool {
    currency_name: "Bitcoin",
    wrapped_token: wbtc_addr,
    tier2_pool: btc_tier2_addr,
    multisig_address: "5D... (3-of-5 multi-sig)",
    max_tx_limit: 10 * BTC_UNITS,           // 10 BTC max per tx
    daily_withdrawal_limit: 100 * BTC_UNITS, // 100 BTC daily limit
    paused: false,
}
```

**Transaction Limits by Currency:**
| Currency | Max Per TX | Daily Limit |
|----------|-----------|-------------|
| BTC | 10 BTC | 100 BTC |
| ETH | 100 ETH | 1,000 ETH |
| SOL | 10,000 SOL | 100,000 SOL |
| BNB | 1,000 BNB | 10,000 BNB |
| TRX | 1M TRX | 10M TRX |
| XRP | 100K XRP | 1M XRP |
| ADA | 100K ADA | 1M ADA |
| DOGE | 10M DOGE | 100M DOGE |
| LINK | 10K LINK | 100K LINK |
| XLM | 100K XLM | 1M XLM |
| MATIC | 100K MATIC | 1M MATIC |

### Tier 2 Pools (ETRWrappedPool)

```rust
ETRWrappedPool {
    wrapped_token: wbtc_addr,
    etr_reserve: 845_750_000 * UNITS,
    virtual_reserve: 33.83 * BTC_UNITS,
    swap_fee_bps: 30,                    // 0.3% fee
    paused: false,
}
```

### EDSC System

**EDSCToken:**
```rust
EDSCToken {
    name: "Ëtrid Dollar Stablecoin",
    symbol: "EDSC",
    decimals: 18,
    initial_supply: 100_000_000 * UNITS, // 100M EDSC
}
```

**EDSCReserveVault:**
```rust
EDSCReserveVault {
    usdc_token: usdc_addr,
    usdt_token: usdt_addr,
    dai_token: dai_addr,
    target_usdc_allocation: 50,          // 50%
    target_usdt_allocation: 30,          // 30%
    target_dai_allocation: 20,           // 20%
    rebalance_threshold: 5,              // ±5% deviation triggers rebalance
    initial_usdc: 50_000_000 * UNITS,
    initial_usdt: 30_000_000 * UNITS,
    initial_dai: 20_000_000 * UNITS,
}
```

**EDSCMintingEngine:**
```rust
EDSCMintingEngine {
    edsc_token: edsc_token_addr,
    reserve_vault: reserve_vault_addr,
    external_swap_router: swap_router_addr,
    // Cross-PBC routing (14 PBCs route purchases to EDSC pool)
    pbc_channels: [
        (1, btc_pbc_parachain_id),
        (2, eth_pbc_parachain_id),
        // ... all 14 PBCs
    ],
}
```

**EDSCPegStabilizer:**
```rust
EDSCPegStabilizer {
    edsc_token: edsc_token_addr,
    oracle: chainlink_eth_usd_feed,
    target_price: 1_000_000,             // $1.00 (6 decimals)
    deviation_threshold: 20_000,         // ±2% ($0.98 - $1.02)
    circuit_breaker_threshold: 100_000,  // ±10% ($0.90 - $1.10)
    auto_stabilize_enabled: true,
}
```

**EDSCExternalSwapRouter:**
```rust
EDSCExternalSwapRouter {
    one_inch_api: "https://api.1inch.dev/...",
    paraswap_api: "https://api.paraswap.io/...",
    max_slippage_bps: 100,               // 1% max slippage
    supported_chains: [
        ChainId::Ethereum,
        ChainId::BinanceSmartChain,
        ChainId::Polygon,
    ],
}
```

### Intent Router System

**IntentRouter:**
```rust
IntentRouter {
    auto_swap_executor: executor_addr,
    stablecoin_router: stablecoin_router_addr,
    address_registry: registry_addr,
    platform_fee_bps: 30,                // 0.3% platform fee
    fee_recipient: treasury_addr,
    paused: false,
}
```

**AutoSwapExecutor:**
```rust
AutoSwapExecutor {
    two_tier_bridge_router: bridge_router_addr,
    address_registry: registry_addr,
    max_hops: 3,                         // Max 3 swaps per intent
    deadline_buffer: 600,                // 10 minutes default deadline
}
```

**TwoTierBridgeRouter:**
```rust
TwoTierBridgeRouter {
    bridge_pallet_id: 42,                // pallet-bridge-tracker index
    address_registry: registry_addr,
    supported_chains: [
        ChainId::Bitcoin,
        ChainId::Ethereum,
        ChainId::Solana,
        // ... all 11 chains
    ],
}
```

**StablecoinRouter:**
```rust
StablecoinRouter {
    edsc_minting_engine: minting_engine_addr,
    edsc_pbc_parachain_id: 14,           // EDSC-PBC parachain ID
    xcmp_version: 3,                     // XCMv3
}
```

### Bridge Pallets

**pallet-bridge-tracker:**
```rust
BridgeTracker {
    signature_threshold: 3,              // 3-of-5 validators
    authorized_validators: [
        validator1_addr,
        validator2_addr,
        validator3_addr,
        validator4_addr,
        validator5_addr,
    ],
    deposit_confirmation_blocks: 6,      // 6 blocks on external chain
    withdrawal_delay_blocks: 100,        // ~10 minutes delay
}
```

**pallet-state-verifier:**
```rust
StateVerifier {
    reconciliation_frequency: 1000,      // Every 1000 blocks (~4 hours)
    max_discrepancy_threshold: 1_000,    // 0.001% discrepancy tolerance
    circuit_breaker_enabled: true,
    circuit_breaker_threshold: 10_000,   // 0.01% discrepancy triggers pause
}
```

---

## Verification Checklist

Use this checklist to verify all wiring is complete.

### Phase 1: Foundation
- [ ] Address Registry deployed
- [ ] 11 wrapped tokens deployed
- [ ] All wrapped tokens registered in Address Registry
- [ ] Verify: `registry.get_wrapped_token("wBTC")` returns correct address

### Phase 2: Tier 1
- [ ] 11 Tier 1 pools deployed
- [ ] All Tier 1 pools registered in Address Registry
- [ ] MINTER_ROLE granted to each Tier 1 pool on its wrapped token
- [ ] BURNER_ROLE granted to each Tier 1 pool on its wrapped token
- [ ] Multi-sig address set for each Tier 1 pool
- [ ] Verify: `wBTC.has_role(MINTER_ROLE, btc_tier1_pool)` returns true

### Phase 3: Tier 2
- [ ] 11 Tier 2 pools deployed
- [ ] All Tier 2 pools registered in Address Registry
- [ ] Each Tier 1 pool wired to its Tier 2 pool
- [ ] ÉTR liquidity deposited to all Tier 2 pools
- [ ] All Tier 2 pools initialized
- [ ] Verify: `btc_tier1_pool.get_tier2_pool()` returns correct address
- [ ] Verify: `btc_tier2_pool.get_reserves()` shows correct ÉTR amount

### Phase 4: EDSC
- [ ] EDSCToken deployed
- [ ] EDSCReserveVault deployed
- [ ] EDSCMintingEngine deployed
- [ ] EDSCPegStabilizer deployed
- [ ] EDSCExternalSwapRouter deployed
- [ ] All EDSC contracts registered in Address Registry
- [ ] MINTER_ROLE granted to minting engine
- [ ] BURNER_ROLE granted to peg stabilizer
- [ ] Initial reserves deposited (50M USDC, 30M USDT, 20M DAI)
- [ ] Initial EDSC minted (100M)
- [ ] Verify: `reserve_vault.get_reserve_ratio()` returns 100 (1:1 backing)

### Phase 5: Intent Router
- [ ] TwoTierBridgeRouter deployed
- [ ] AutoSwapExecutor deployed
- [ ] StablecoinRouter deployed
- [ ] IntentRouter deployed
- [ ] All routers registered in Address Registry
- [ ] CALLER_ROLE granted: IntentRouter → AutoSwapExecutor
- [ ] CALLER_ROLE granted: AutoSwapExecutor → TwoTierBridgeRouter
- [ ] CALLER_ROLE granted: IntentRouter → StablecoinRouter
- [ ] Verify: `intent_router.get_auto_swap_executor()` returns correct address

### Phase 6: Bridge Pallets
- [ ] pallet-bridge-tracker deployed to Primearc runtime
- [ ] pallet-state-verifier deployed to Primearc runtime
- [ ] 5 authorized validators registered
- [ ] Signature threshold set to 3 (3-of-5)
- [ ] TwoTierBridgeRouter configured with bridge pallet ID
- [ ] Verify: `bridge_tracker.get_validator_count()` returns 5
- [ ] Verify: `bridge_tracker.get_threshold()` returns 3

### Integration Tests
- [ ] Test: Lock BTC → Mint wBTC → Swap to ÉTR (full flow)
- [ ] Test: ÉTR → Swap to wBTC → Burn → Release BTC (reverse flow)
- [ ] Test: Purchase EDSC with USDC (transaction-driven minting)
- [ ] Test: Purchase EDSC with BTC (external swap + minting)
- [ ] Test: EDSC peg stabilization (price deviation triggers)
- [ ] Test: Multi-hop swap through Intent Router
- [ ] Test: Bridge attestation verification (3-of-5 signatures)
- [ ] Test: State reconciliation detects discrepancy
- [ ] Test: Circuit breaker pauses on critical error
- [ ] Test: Cross-PBC stablecoin routing via XCMP

### Security Verification
- [ ] All contracts paused → unpause after verification
- [ ] All admin roles transferred to multi-sig wallet
- [ ] All deployer keys rotated or burned
- [ ] Rate limits configured on all Tier 1 pools
- [ ] Emergency contacts documented
- [ ] Monitoring and alerting configured
- [ ] Incident response plan documented

---

## Configuration Files

### addresses.json
Store all deployed addresses for easy reference.

```json
{
  "address_registry": "5D...",
  "wrapped_tokens": {
    "wBTC": "5E...",
    "wETH": "5F...",
    "wSOL": "5G...",
    "wBNB": "5H...",
    "wTRX": "5I...",
    "wXRP": "5J...",
    "wADA": "5K...",
    "wDOGE": "5L...",
    "wLINK": "5M...",
    "wXLM": "5N...",
    "wMATIC": "5O..."
  },
  "tier1_pools": {
    "BTC": "5P...",
    "ETH": "5Q...",
    "SOL": "5R...",
    "BNB": "5S...",
    "TRX": "5T...",
    "XRP": "5U...",
    "ADA": "5V...",
    "DOGE": "5W...",
    "LINK": "5X...",
    "XLM": "5Y...",
    "MATIC": "5Z..."
  },
  "tier2_pools": {
    "BTC": "5a...",
    "ETH": "5b...",
    "SOL": "5c...",
    "BNB": "5d...",
    "TRX": "5e...",
    "XRP": "5f...",
    "ADA": "5g...",
    "DOGE": "5h...",
    "LINK": "5i...",
    "XLM": "5j...",
    "MATIC": "5k..."
  },
  "edsc_system": {
    "edsc_token": "5l...",
    "reserve_vault": "5m...",
    "minting_engine": "5n...",
    "peg_stabilizer": "5o...",
    "external_swap_router": "5p..."
  },
  "intent_router_system": {
    "intent_router": "5q...",
    "auto_swap_executor": "5r...",
    "two_tier_bridge_router": "5s...",
    "stablecoin_router": "5t..."
  },
  "multisig_wallet": "5u...",
  "treasury": "5v..."
}
```

---

**Status:** Configuration Complete - Ready for deployment scripts
**Next:** Create automated deployment scripts that implement this wiring
**Timeline:** Ready for testnet deployment after script creation

