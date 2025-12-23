# ĒTRID Contract Integration & Deployment Plan

**Date:** December 8, 2025
**Status:** Pre-deployment Planning
**Target:** Testnet → Mainnet

---

## Overview

This document outlines the integration and deployment strategy for all ĒTRID ink! contracts and Substrate pallets.

---

## Component Status

| Component | Contracts | Status | LOC | Tests |
|-----------|-----------|--------|-----|-------|
| **PrimeSwap Pools** | 3 | ✅ Complete | 2,508 | 45+ |
| **EDSC Reserve** | 5 | 🔄 In Progress | ~2,500 | ~50 |
| **Intent Router** | 4 | 🔄 In Progress | ~2,000 | ~40 |
| **Bridge Pallets** | 2 | 🔄 In Progress | ~1,000 | ~20 |
| **Total** | 14 | 35% Done | ~8,000 | ~155 |

---

## Integration Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    ĒTRID INTEGRATION LAYERS                      │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  LAYER 1: EXTERNAL CHAIN BRIDGES                                │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  pallet-bridge-tracker (Substrate pallet)              │     │
│  │  ├─ Track deposits/withdrawals                         │     │
│  │  ├─ M-of-N attestation                                 │     │
│  │  └─ Emit: BridgeDeposit event                          │     │
│  └────────────────────────────────────────────────────────┘     │
│                           ↓                                      │
│  LAYER 2: TIER 1 RESERVE POOLS (ink! contracts)                 │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  ExternalCurrencyPool (11 instances)                   │     │
│  │  ├─ Listen: BridgeDeposit event                        │     │
│  │  ├─ Lock external currency                             │     │
│  │  ├─ Mint wrapped token 1:1                             │     │
│  │  └─ Send wToken to Tier 2                              │     │
│  └────────────────────────────────────────────────────────┘     │
│                           ↓                                      │
│  LAYER 3: TIER 2 TRADING POOLS (ink! contracts)                 │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  ETRWrappedPool (11 instances)                         │     │
│  │  ├─ Receive wTokens from Tier 1                        │     │
│  │  ├─ Provide ÉTR/wToken liquidity (AMM)                 │     │
│  │  └─ Enable swaps                                       │     │
│  └────────────────────────────────────────────────────────┘     │
│                           ↓                                      │
│  LAYER 4: INTENT ROUTER (ink! contracts)                        │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  IntentRouter + AutoSwapExecutor                       │     │
│  │  ├─ Abstract away Layers 1-3                           │     │
│  │  ├─ User calls: convertToEtr(BTC, amount)              │     │
│  │  ├─ Orchestrate: Bridge → T1 → T2 → User               │     │
│  │  └─ Hide wrapped tokens completely                     │     │
│  └────────────────────────────────────────────────────────┘     │
│                           ↓                                      │
│  LAYER 5: EDSC STABLECOIN (parallel to Layers 1-4)              │
│  ┌────────────────────────────────────────────────────────┐     │
│  │  EDSCMintingEngine + EDSCReserveVault                  │     │
│  │  ├─ Route stablecoin purchases from ALL PBCs           │     │
│  │  ├─ Accumulate USDC/USDT reserves                      │     │
│  │  ├─ Mint EDSC 1:1                                      │     │
│  │  └─ EDSCPegStabilizer maintains $1 peg                 │     │
│  └────────────────────────────────────────────────────────┘     │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Deployment Phases

### Phase 1: Foundation Layer (Week 1)

**Deploy Base Infrastructure**

1. Deploy Substrate pallets to Primearc runtime
   - pallet-bridge-tracker
   - pallet-state-verifier

2. Deploy wrapped token contracts (11 tokens)
   - wBTC, wETH, wSOL, wBNB, wTRX, wXRP, wADA, wDOGE, wLINK, wXLM, wMATIC
   - Each with 18 decimals (except wBTC: 8 decimals)

**Verification:**
```bash
# Test wrapped token deployment
cargo test -p wrapped-token-template

# Verify on testnet
./scripts/verify-wrapped-tokens.sh
```

---

### Phase 2: Reserve Layer (Week 2)

**Deploy Tier 1 Reserve Pools**

1. Deploy 11 ExternalCurrencyPool instances
   - One per currency
   - Configure multi-sig addresses (3-of-5)
   - Set rate limits

2. Wire to wrapped tokens
   - Grant minter role to each pool
   - Configure tier2_pool addresses (will be set in Phase 3)

**Configuration:**
```rust
// Example: BTC Reserve Pool
ExternalCurrencyPool::new(
    "Bitcoin".to_string(),
    wBTC_address,
    tier2_btc_pool, // Set in Phase 3
    multisig_address,
    1_000_000_000_000_000, // 0.01 BTC max tx
    10_000_000_000_000_000, // 0.1 BTC daily limit
)
```

---

### Phase 3: Trading Layer (Week 2-3)

**Deploy Tier 2 Trading Pools**

1. Deploy 11 ETRWrappedPool instances with allocations:
```rust
// Pool initialization with ÉTR allocations
pools = [
    ("wBTC", 845_750_000 * UNITS, 33.83 * BTC_UNITS),
    ("wETH", 191_400_000 * UNITS, 95.7 * ETH_UNITS),
    ("wXRP", 62_400_000 * UNITS, 312_000 * XRP_UNITS),
    ("wSOL", 44_500_000 * UNITS, 2_225 * SOL_UNITS),
    ("wBNB", 40_000_000 * UNITS, 160 * BNB_UNITS),
    ("wDOGE", 26_800_000 * UNITS, 2_680_000 * DOGE_UNITS),
    ("wADA", 15_600_000 * UNITS, 78_000 * ADA_UNITS),
    ("wLINK", 8_900_000 * UNITS, 890 * LINK_UNITS),
    ("wTRX", 6_600_000 * UNITS, 660_000 * TRX_UNITS),
    ("wXLM", 4_500_000 * UNITS, 45_000 * XLM_UNITS),
    ("wMATIC", 3_500_000 * UNITS, 7_000 * MATIC_UNITS),
]
```

2. Initialize each pool
   - Transfer ÉTR allocation to pool
   - Call `initialize_pool(etr_amount)`
   - Verify k constant calculated correctly

3. Update Tier 1 pools with Tier 2 addresses

---

### Phase 4: User Interface Layer (Week 3)

**Deploy Intent Router System**

1. Deploy IntentRouter
   - Configure all Tier 1/Tier 2 pool addresses
   - Set platform fee (0.3%)
   - Set slippage defaults

2. Deploy AutoSwapExecutor
   - Link to IntentRouter
   - Configure all route mappings

3. Deploy TwoTierBridgeRouter
   - Link bridge pallets to pools
   - Configure routing logic

4. Deploy StablecoinRouter
   - Configure EDSC-PBC destination
   - Set up XCMP messaging

**Integration Test:**
```rust
// Full flow test: BTC → ÉTR
#[test]
fn test_full_btc_to_etr_flow() {
    // 1. Lock BTC on Bitcoin (external)
    // 2. Bridge relayer submits attestation
    // 3. pallet-bridge-tracker verifies
    // 4. Tier 1 pool mints wBTC
    // 5. Tier 2 pool swaps wBTC → ÉTR
    // 6. User receives ÉTR

    assert_eq!(user_etr_balance, expected_amount);
}
```

---

### Phase 5: EDSC Stablecoin (Week 4)

**Deploy EDSC Reserve System**

1. Deploy EDSCToken
   - Mint initial 100M EDSC to reserve vault

2. Deploy EDSCReserveVault
   - Initialize with 50M USDC, 30M USDT, 20M DAI
   - Set rebalancing parameters

3. Deploy EDSCMintingEngine
   - Grant minter role to engine
   - Configure cross-PBC routing

4. Deploy EDSCPegStabilizer
   - Connect to oracle (Chainlink)
   - Set ±2% deviation threshold

5. Deploy ExternalSwapRouter
   - Configure 1inch API integration
   - Set slippage limits

**Initial Reserve Setup:**
```rust
// Seed the reserve
reserve_vault.deposit_usdc(50_000_000 * UNITS);
reserve_vault.deposit_usdt(30_000_000 * UNITS);
reserve_vault.deposit_dai(20_000_000 * UNITS);

// Verify 1:1 backing
assert_eq!(reserve_vault.get_reserve_ratio(), 100);
```

---

## Integration Wiring

### Contract Address Registry

Create a central registry contract to store all addresses:

```rust
#[ink::contract]
mod address_registry {
    #[ink(storage)]
    pub struct AddressRegistry {
        // Wrapped tokens
        wrapped_tokens: Mapping<String, AccountId>,

        // Tier 1 pools
        tier1_pools: Mapping<String, AccountId>,

        // Tier 2 pools
        tier2_pools: Mapping<String, AccountId>,

        // Routers
        intent_router: AccountId,
        auto_swap_executor: AccountId,
        bridge_router: AccountId,
        stablecoin_router: AccountId,

        // EDSC
        edsc_token: AccountId,
        edsc_minting_engine: AccountId,
        edsc_reserve_vault: AccountId,
        edsc_peg_stabilizer: AccountId,
    }

    // Getters for all addresses
    // Only owner can update
}
```

---

## Testing Strategy

### Unit Tests (Per Contract)

Each contract has 10-15 unit tests covering:
- Happy paths
- Error conditions
- Edge cases
- Access control
- Overflow protection

**Run all unit tests:**
```bash
cd contracts/primeswap && cargo test
cd contracts/edsc && cargo test
cd contracts/intent-router && cargo test
cd 05-multichain/bridges/pallets && cargo test
```

### Integration Tests (Cross-Contract)

1. **Tier 1 → Tier 2 Integration**
   - Lock → Mint → Swap flow
   - Burn → Release flow

2. **Intent Router → Pools Integration**
   - Single-call BTC → ÉTR conversion
   - Slippage protection
   - Deadline enforcement

3. **Bridge → Tier 1 Integration**
   - Deposit event handling
   - M-of-N attestation
   - State reconciliation

4. **EDSC Cross-PBC Integration**
   - Stablecoin purchase routing
   - XCMP message handling
   - Reserve accumulation

### End-to-End Tests (Full System)

```bash
# E2E test suite
./scripts/e2e-tests.sh

# Tests:
# 1. External user locks 1 BTC on Bitcoin
# 2. Bridge relayers detect and attest
# 3. Tier 1 pool mints wBTC
# 4. Tier 2 pool swaps wBTC → ÉTR
# 5. User receives ~24,974 ÉTR
# 6. Verify all state updates
```

---

## Security Audits

### Pre-Audit Checklist

- [ ] All unit tests passing (100% coverage)
- [ ] Integration tests passing
- [ ] E2E tests passing
- [ ] Manual security review completed
- [ ] Overflow protection verified
- [ ] Access control verified
- [ ] Reentrancy protection verified

### Audit Partners

Potential auditors:
1. Trail of Bits (Substrate specialist)
2. OpenZeppelin (ink! specialist)
3. Halborn (blockchain security)

**Timeline:** 4-6 weeks
**Cost:** $50k-$150k depending on scope

---

## Mainnet Deployment Checklist

### Pre-Deployment

- [ ] Security audit complete and issues resolved
- [ ] Testnet running stable for 30+ days
- [ ] All integration tests passing
- [ ] Multi-sig wallets configured (3-of-5)
- [ ] Bridge relayers deployed (5+ per chain)
- [ ] Oracle feeds configured
- [ ] Rate limits set appropriately
- [ ] Emergency pause mechanisms tested
- [ ] Monitoring and alerting configured

### Deployment Day

**Hour 0-2: Deploy Base Layer**
1. Deploy pallets to Primearc (governance upgrade)
2. Deploy wrapped tokens
3. Verify deployments

**Hour 2-4: Deploy Reserve Layer**
4. Deploy Tier 1 pools
5. Configure multi-sig
6. Grant minter permissions

**Hour 4-6: Deploy Trading Layer**
7. Deploy Tier 2 pools
8. Initialize with ÉTR allocations
9. Wire Tier 1 → Tier 2

**Hour 6-8: Deploy User Interface**
10. Deploy Intent Router system
11. Wire all integrations
12. Deploy address registry

**Hour 8-10: Deploy EDSC**
13. Deploy EDSC contracts
14. Seed initial reserves
15. Configure cross-PBC routing

**Hour 10-12: Verification**
16. Run full E2E test suite
17. Verify all state
18. Enable monitoring
19. Announce launch

---

## Monitoring & Maintenance

### Key Metrics to Monitor

1. **Tier 1 Pools**
   - Reserve ratio (must = 100%)
   - Daily withdrawal volume
   - Multi-sig transaction queue

2. **Tier 2 Pools**
   - ÉTR/wToken prices
   - Swap volume
   - Fees collected
   - Price impact

3. **EDSC**
   - Peg deviation from $1
   - Reserve ratio
   - Minting/burning volume
   - Circulating supply

4. **Intent Router**
   - Conversion success rate
   - Average slippage
   - Transaction volume
   - Failed transactions

### Alert Thresholds

- Reserve ratio < 99% → CRITICAL
- Peg deviation > ±5% → WARNING
- Peg deviation > ±10% → CRITICAL (auto-pause)
- Daily withdrawal > limit → WARNING
- Failed attestation → WARNING
- Price impact > 5% → WARNING

---

## Rollback Procedures

### Emergency Pause

All contracts have emergency pause:
```rust
// Pause all operations
tier1_pool.pause()?;
tier2_pool.pause()?;
intent_router.pause()?;
edsc_minting_engine.pause()?;
```

### Contract Upgrade

ink! contracts can be upgraded via:
1. Deploy new version
2. Migrate state
3. Update address registry
4. Deprecate old version

### Rollback Scenarios

**Scenario 1: Bug in Tier 2 Pool**
- Pause affected pool
- Users can withdraw from Tier 1
- Deploy fixed version
- Resume operations

**Scenario 2: Oracle Failure (EDSC)**
- EDSCPegStabilizer auto-pauses
- Manual peg maintenance until oracle restored
- Resume algorithmic operation

**Scenario 3: Bridge Compromise**
- Pause all bridge operations
- Multi-sig review all pending withdrawals
- Investigate and patch
- Resume with enhanced security

---

## Post-Deployment Roadmap

### Month 1: Stabilization
- Monitor all metrics 24/7
- Quick fixes for minor issues
- Optimize gas costs
- Gather user feedback

### Month 2-3: Enhancement
- Add more pools (if demand)
- Improve swap routing
- Integrate more oracles
- Enhance monitoring

### Month 4-6: Ecosystem Growth
- List EDSC on external exchanges
- Partner integrations
- Developer tooling
- Marketing push

---

**Status:** Planning Complete - Awaiting Agent 2 & 3 completion
**Next:** Execute deployment phases once all contracts ready
**Target:** Testnet launch Q1 2026, Mainnet Q2 2026
