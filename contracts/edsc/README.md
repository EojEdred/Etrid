# EDSC Algorithmic Reserve System

**Etrid Dollar Stablecoin (EDSC)** - A fully-collateralized algorithmic stablecoin backed 1:1 by multi-asset reserves.

## Overview

EDSC is a transaction-driven stablecoin system that maintains a $1.00 peg through:
- **Multi-asset reserves** (50% USDC, 30% USDT, 20% DAI)
- **Algorithmic peg stabilization** (±2% deviation threshold)
- **Transaction-flow minting** (no user deposits required)
- **Autonomous rebalancing** via external swap aggregators

## Architecture

See [EDSC_RESERVE_ARCHITECTURE.md](./EDSC_RESERVE_ARCHITECTURE.md) for detailed system design.

## Contracts

### Core Contracts

1. **EDSC Token** (`core/edsc-token/`)
   - ERC20-compatible stablecoin
   - Initial supply: 100M EDSC
   - Restricted minting (only via minting engine)

2. **Reserve Vault** (`core/reserve-vault/`)
   - Multi-asset reserve management
   - Target allocation: 50% USDC, 30% USDT, 20% DAI
   - Automatic rebalancing

3. **Minting Engine** (`core/minting-engine/`)
   - Transaction-driven EDSC minting
   - 1:1 backing guarantee
   - Cross-PBC routing support

### Stabilization Contracts

4. **Peg Stabilizer** (`stabilization/peg-stabilizer/`)
   - Maintains 1 EDSC = $1.00
   - Oracle price feed integration
   - Algorithmic buy/burn or mint/sell

5. **External Swap Router** (`stabilization/external-swap-router/`)
   - Interface for swap aggregators (1inch, ParaSwap)
   - Volatile asset → USDC conversion
   - Slippage protection

## Building Contracts

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ink! CLI
cargo install cargo-contract --force
```

### Build Individual Contracts

```bash
# Build EDSC Token
cd core/edsc-token
cargo contract build --release

# Build Reserve Vault
cd ../reserve-vault
cargo contract build --release

# Build Minting Engine
cd ../minting-engine
cargo contract build --release

# Build Peg Stabilizer
cd ../../stabilization/peg-stabilizer
cargo contract build --release

# Build External Swap Router
cd ../external-swap-router
cargo contract build --release
```

### Build All Contracts

```bash
# From contracts/edsc/ directory
for contract in core/*/  stabilization/*/; do
    echo "Building $contract..."
    (cd "$contract" && cargo contract build --release)
done
```

### Run Tests

```bash
# Test EDSC Token
cd core/edsc-token
cargo test

# Test all contracts
for contract in core/*/  stabilization/*/; do
    echo "Testing $contract..."
    (cd "$contract" && cargo test)
done
```

## Deployment

### 1. Deploy Contracts

```bash
# Deploy to local node (Substrate with pallet-contracts)
cargo contract instantiate \
    --constructor new \
    --args <constructor_args> \
    --suri //Alice \
    target/ink/edsc_token.contract
```

### 2. Configure Permissions

```bash
# Set minting engine in EDSC Token
edsc_token.set_minting_engine(minting_engine_address)

# Authorize minting engine in Reserve Vault
reserve_vault.authorize_depositor(minting_engine_address)
```

### 3. Initialize Reserves

```bash
# Deposit initial reserves (100M total)
reserve_vault.deposit_usdc(50_000_000 * 10^18)  # 50M USDC
reserve_vault.deposit_usdt(30_000_000 * 10^18)  # 30M USDT
reserve_vault.deposit_dai(20_000_000 * 10^18)   # 20M DAI
```

### 4. Enable Operations

```bash
# Enable minting
minting_engine.toggle_minting(true)

# Enable stabilization
peg_stabilizer.toggle_stabilization(true)
```

## Usage Examples

### Mint EDSC with USDC

```rust
// User sends 1000 USDC
let edsc_amount = minting_engine.mint_with_usdc(1000 * 10^18);
// Returns: 1000 EDSC (1:1 ratio)
```

### Mint EDSC with BTC (Auto-swap)

```rust
// User sends 0.02 BTC (worth ~$1000)
let edsc_amount = minting_engine.mint_with_btc(0.02 * 10^8);
// Auto-swaps BTC → USDC (99% after 0.5% slippage)
// Returns: ~990 EDSC
```

### Check Peg Status

```rust
// Check if stabilization is needed
let (price, needs_action) = peg_stabilizer.check_peg();
// Returns: ($1.00, false) if stable
```

### Stabilize Peg

```rust
// Automatic stabilization (called by keeper/cron)
peg_stabilizer.stabilize();
// Executes buy/burn or mint/sell to restore $1.00 peg
```

## Security Features

- **Reentrancy Protection** - Guards on all state-changing functions
- **Access Control** - Owner, minting engine, authorized depositors
- **Rate Limiting** - Max 1M EDSC per transaction
- **Circuit Breaker** - Auto-pause at ±10% deviation
- **Reserve-First Guarantee** - Deposit succeeds before minting
- **Overflow Protection** - Checked arithmetic throughout

## Testing Strategy

### Unit Tests (35+ tests included)

```bash
cargo test --all-features
```

### Integration Tests

```bash
# Test cross-contract interactions
cargo test --features e2e-tests
```

### Economic Attack Scenarios

- Flash loan attacks
- Oracle manipulation
- Reserve drain attempts
- Peg manipulation

## Oracle Integration (Next Steps)

See [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) for detailed oracle integration guide.

**Primary Oracle:** Chainlink EDSC/USD
**Secondary Oracle:** Band Protocol
**Tertiary Oracle:** Custom TWAP (Uniswap V3)

## Monitoring

### Key Metrics to Monitor

1. **Reserve Ratio** - Should always be ≥100%
2. **EDSC Price** - Should stay within ±2% of $1.00
3. **Reserve Allocation** - 50% USDC, 30% USDT, 20% DAI
4. **Minting Volume** - Track daily minting activity
5. **Stabilization Actions** - Count buy/burn vs mint/sell

### Alerts

- Reserve ratio < 100%
- Price deviation > ±5%
- Circuit breaker triggered
- Oracle data stale (>1 hour)
- Large mints (>100k EDSC)

## Governance

Future DAO governance will control:
- Deviation threshold adjustments
- Reserve allocation targets
- Oracle address updates
- Circuit breaker parameters
- Emergency pause/unpause

## License

MIT License - See LICENSE file

## Contact

- **Team:** Etrid Development Team
- **Email:** dev@etrid.io
- **Docs:** [https://docs.etrid.io](https://docs.etrid.io)

---

**Status:** IMPLEMENTATION COMPLETE
**Version:** 1.0.0
**Last Updated:** 2025-12-08
