# EDSC-PBC Runtime

**Purpose:** Dedicated Partition Burst Chain for Ëtrid Dollar Stablecoin (EDSC)

## Overview

EDSC-PBC is the 13th PBC in the Ëtrid multichain, responsible for:
- EDSC token management (mint/burn/transfer)
- Receipt-based redemption system (SBT tokens)
- Multi-path redemption engine
- TWAP oracle aggregation
- Circuit breakers and safety controls
- Checkpoint synchronization with FlareChain

## Architecture

```
PBC-EDSC (This Chain)
├─ pallet-edsc-token          # ERC20-like EDSC token
├─ pallet-edsc-receipts        # SBT receipt registry
├─ pallet-edsc-redemption      # 3-path redemption engine
├─ pallet-edsc-oracle          # TWAP price aggregation
├─ pallet-edsc-checkpoint      # State commits to main chain
└─ pallet-circuit-breaker      # Safety controls

FlareChain (Main Chain)
├─ pallet-reserve-vault        # On-chain collateral storage
├─ pallet-custodian-registry   # Off-chain reserve agents
├─ pallet-reserve-oracle       # Aggregate reserve reporting
└─ pallet-pbc-bridge           # Checkpoint verification
```

## Key Features

### 1. Three Redemption Paths

**Path 1: SBT Receipt (No Fee)**
- User provides on-chain receipt from verified purchase
- Redeems at recorded purchase price
- Instant, no dynamic fees

**Path 2: Signed Attestation (Dynamic Fee)**
- Exchange/merchant provides signed proof
- TWAP calculated at purchase time
- Dynamic fee prevents arbitrage

**Path 3: Fallback TWAP (Highest Fee)**
- No proof required
- Uses current 24h TWAP
- Strictest per-wallet caps

### 2. Peg Defense Mechanisms

- **Dynamic Fees:** Remove arbitrage profit during depegs
- **Circuit Breakers:** Pause redemptions if volume exceeds caps
- **Reserve Ratio Enforcement:** Maintain 110-130% collateralization
- **Automated Buybacks:** Protocol buys EDSC when price < $1

### 3. Oracle System

- Multi-source TWAP (Binance, Coinbase, Uniswap, PancakeSwap, Curve)
- Outlier removal (> 2% from median)
- 24h primary window, 7-day fallback
- Off-chain worker for price fetching

## Building

```bash
# Build runtime WASM
cargo build --release -p edsc-pbc-runtime

# Build collator
cargo build --release -p edsc-pbc-collator

# Generate chain spec
./target/release/edsc-pbc-collator build-spec --disable-default-bootnode --chain dev > edsc-pbc-spec.json
```

## Running

```bash
# Development mode
./target/release/edsc-pbc-collator \
    --dev \
    --rpc-port 9955 \
    --port 30343 \
    --relay-chain-rpc-url ws://127.0.0.1:9944

# With FlareChain relay
./target/release/edsc-pbc-collator \
    --collator \
    --base-path /tmp/edsc-pbc \
    --rpc-port 9955 \
    --relay-chain-rpc-url ws://127.0.0.1:9944
```

## Testing

```bash
# Unit tests
cargo test -p pallet-edsc-token
cargo test -p pallet-edsc-redemption
cargo test -p pallet-edsc-oracle

# Integration tests
./test_edsc_integration.sh
```

## Parameters (Production)

| Parameter | Value | Purpose |
|---|---|---|
| `MIN_FEE` | 0.25% | Minimum redemption fee |
| `SAFETY_MULTIPLIER` | 1.2 | Fee calculation multiplier |
| `TWAP_WINDOW` | 24 hours | Primary TWAP window |
| `RESERVE_RATIO_TARGET` | 120% | Optimal collateralization |
| `PER_TX_CAP` | 50,000 EDSC | Max per transaction |
| `DAILY_CAP` | 0.5% of supply | Max daily redemptions |

## Documentation

- [EDSC Implementation Plan](../../EDSC_IMPLEMENTATION_PLAN.md)
- [EDSC-PBT Design](../../edsc-pbt.md)
- [Development Roadmap](../../DEVELOPMENT_ROADMAP.md)

## Status

**Phase 0:** Directory structure created ✅
**Phase 1:** Core pallets (token, receipts, redemption) - Pending
**Phase 2:** Oracle integration - Pending
**Phase 3:** Reserve system - Pending
**Phase 4:** Custodian integration - Pending
**Phase 5:** Testing & audit - Pending
**Phase 6:** Production deployment - Pending

---

**Created:** October 19, 2025
**Estimated Completion:** 14 weeks
