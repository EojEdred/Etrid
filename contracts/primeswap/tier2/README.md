# Tier 2: ÉTR/Wrapped Token Trading Pool

## Overview

The ÉTR/Wrapped Token Trading Pool is the second tier of the PrimeSwap architecture. It provides AMM-based trading liquidity between ÉTR and wrapped tokens (wBTC, wETH, etc.).

## Purpose

- **Enable trading** between ÉTR and wrapped tokens
- **Provide liquidity** using VirtualReserveAMM mechanism
- **Price discovery** through constant product formula (x × y = k)
- **Generate fees** from swaps (0.3% per swap)
- **Distribute ÉTR** to users converting from external currencies

## Contract: `etr-wrapped-pool`

### Key Features

1. **Dual-sided Pool**: Holds both ÉTR and wrapped tokens
2. **VirtualReserveAMM**: Uses virtual reserves to bootstrap liquidity
3. **Constant Product Formula**: (x + Δx)(y - Δy) = k
4. **0.3% Swap Fee**: Industry-standard fee on all swaps
5. **Slippage Protection**: Min output amount enforcement
6. **Emergency Pause**: Can be paused for security

### Pool Allocations

Total ÉTR allocated: **1,250,000,000 ÉTR** across 11 pools

| Pool | ÉTR Allocation | Percentage | Virtual Reserve |
|------|----------------|------------|-----------------|
| ÉTR/wBTC | 845,750,000 | 67.66% | 33.83 BTC |
| ÉTR/wETH | 191,400,000 | 15.31% | 95.7 ETH |
| ÉTR/wXRP | 62,400,000 | 4.99% | 312,000 XRP |
| ÉTR/wSOL | 44,500,000 | 3.56% | 2,225 SOL |
| ÉTR/wBNB | 40,000,000 | 3.20% | 160 BNB |
| ÉTR/wDOGE | 26,800,000 | 2.14% | 2,680,000 DOGE |
| ÉTR/wADA | 15,600,000 | 1.25% | 78,000 ADA |
| ÉTR/wLINK | 8,900,000 | 0.71% | 890 LINK |
| ÉTR/wTRX | 6,600,000 | 0.53% | 660,000 TRX |
| ÉTR/wXLM | 4,500,000 | 0.36% | 45,000 XLM |
| ÉTR/wMATIC | 3,500,000 | 0.28% | 7,000 MATIC |

### Core Functions

#### `initialize_pool(etr_amount: Balance) -> Result<()>`

Initialize the pool with ÉTR allocation.

**Example:**
```rust
// Initialize ÉTR/wBTC pool with 845.75M ÉTR
pool.initialize_pool(845_750_000_000_000)?; // 18 decimals
// Sets k = ÉTR × (wrapped_real + wrapped_virtual)
```

**This must be called once before any swaps.**

#### `swap_etr_for_wrapped(amount_etr: Balance, min_out: Balance) -> Result<Balance>`

Swap ÉTR for wrapped tokens (e.g., ÉTR → wBTC).

**Example:**
```rust
// Swap 25,000 ÉTR for wBTC
let wbtc_received = pool.swap_etr_for_wrapped(
    25_000_000_000_000_000_000_000, // 25,000 ÉTR
    990_000, // Min 0.0099 BTC (slippage protection)
)?;
// Returns: ~1.0 wBTC (after 0.3% fee)
```

**Flow:**
1. User sends ÉTR
2. 0.3% fee deducted
3. AMM calculates output
4. Check slippage protection
5. Transfer wBTC to user

#### `swap_wrapped_for_etr(amount_wrapped: Balance, min_out: Balance) -> Result<Balance>`

Swap wrapped tokens for ÉTR (e.g., wBTC → ÉTR).

**Example:**
```rust
// Swap 0.5 wBTC for ÉTR
let etr_received = pool.swap_wrapped_for_etr(
    50_000_000, // 0.5 BTC (8 decimals)
    12_000_000_000_000_000_000_000, // Min 12,000 ÉTR
)?;
// Returns: ~12,500 ÉTR (after 0.3% fee)
```

#### `get_amount_out(amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance`

Calculate expected output for a given input (read-only).

**Example:**
```rust
// How much wBTC for 10,000 ÉTR?
let expected_wbtc = pool.get_amount_out(
    10_000_000_000_000_000_000_000, // 10k ÉTR
    pool.etr_reserve(),
    pool.wrapped_reserve_real() + pool.wrapped_reserve_virtual(),
);
```

#### `get_price() -> u128`

Get current price (ÉTR per wrapped token).

**Example:**
```rust
let price = pool.get_price();
// Returns: 25_000_000_000 (= 25,000 ÉTR per 1 BTC, scaled)
```

### AMM Formula

The constant product AMM formula:

```
k = x × y

Where:
- k = AMM constant (set at initialization)
- x = ÉTR reserve
- y = wrapped reserve (real + virtual)

On swap:
(x + Δx) × (y - Δy) = k

Solving for Δy:
Δy = y - k / (x + Δx)
```

**Virtual Reserve Mechanism:**

```
Total Wrapped Reserve = Real Reserve + Virtual Reserve

Example (wBTC pool):
- Virtual: 33.83 BTC (fixed)
- Real: starts at 0, grows with swaps
- Total: 33.83 + Real

This bootstraps liquidity without requiring actual wrapped tokens upfront.
```

### Storage Structure

```rust
pub struct EtrWrappedPool {
    config: PoolConfig,               // Pool configuration
    etr_token: AccountId,             // ÉTR contract
    wrapped_token: AccountId,         // wBTC contract
    etr_reserve: Balance,             // Real ÉTR in pool
    wrapped_reserve_real: Balance,    // Real wBTC in pool
    wrapped_reserve_virtual: Balance, // Virtual wBTC
    k: u128,                          // AMM constant
    owner: AccountId,
    paused: bool,
    swap_fee_bps: u16,                // 30 = 0.3%
    total_fees_etr: Balance,
    total_fees_wrapped: Balance,
    total_swaps: u64,
}
```

### Events

```rust
// Emitted when pool is initialized
event PoolInitialized {
    currency: String,
    etr_reserve: Balance,
    wrapped_virtual: Balance,
    k: u128,
    timestamp: Timestamp,
}

// Emitted on each swap
event SwapExecuted {
    user: AccountId,
    token_in: SwapDirection,
    amount_in: Balance,
    amount_out: Balance,
    fee: Balance,
    timestamp: Timestamp,
}

// Emitted on reserve updates
event ReserveUpdate {
    etr_reserve: Balance,
    wrapped_reserve_real: Balance,
    wrapped_reserve_virtual: Balance,
    k: u128,
    timestamp: Timestamp,
}
```

### Fee Structure

**Swap Fee: 0.3%** (30 basis points)

```rust
Fee calculation:
fee = amount_in × 30 / 10000
amount_after_fee = amount_in - fee

Example:
Input: 1,000 ÉTR
Fee: 3 ÉTR (0.3%)
After fee: 997 ÉTR (used for AMM calculation)
```

Fees accumulate and can be collected by the owner.

### Integration Flow

**User converts BTC → ÉTR:**

```
1. User locks 1 BTC in Tier 1
   ↓
2. Tier 1 mints 1 wBTC
   ↓
3. Tier 2 receives 1 wBTC
   ↓
4. Tier 2 swaps: 1 wBTC → 24,974 ÉTR (after 0.3% fee)
   ↓
5. User receives 24,974 ÉTR
```

**User converts ÉTR → BTC:**

```
1. User sends 25,000 ÉTR to Tier 2
   ↓
2. Tier 2 swaps: 25,000 ÉTR → 0.997 wBTC (after 0.3% fee)
   ↓
3. Tier 2 sends 0.997 wBTC to Tier 1
   ↓
4. Tier 1 burns 0.997 wBTC
   ↓
5. Multi-sig releases 0.997 BTC to user
```

### Deployment

Deploy one pool per wrapped currency:

```bash
# Build contract
cargo contract build --release

# Deploy ÉTR/wBTC pool
cargo contract instantiate \
  --constructor new \
  --args "wBTC" 845750000000000000000000000 33830000000 <etr_token> <wbtc_token>

# Initialize with ÉTR
cargo contract call \
  --contract <pool_address> \
  --message initialize_pool \
  --args 845750000000000000000000000
```

### Testing

Run the comprehensive test suite:

```bash
cargo test
```

Tests cover:
- Pool initialization
- ÉTR → wrapped swaps
- Wrapped → ÉTR swaps
- AMM formula correctness
- Fee calculations
- Slippage protection
- Pause functionality
- Price queries
- Edge cases

### Admin Operations

#### Pause/Unpause
```rust
pool.pause()?;
pool.unpause()?;
```

#### Update Swap Fee
```rust
pool.update_swap_fee(50)?; // Change to 0.5%
```

#### Collect Fees
```rust
let (etr_fees, wrapped_fees) = pool.collect_fees()?;
```

## Price Impact Examples

**ÉTR/wBTC Pool (845.75M ÉTR, 33.83 BTC virtual)**

Initial price: 1 BTC = 25,000 ÉTR

| ÉTR Input | wBTC Output | Price Impact |
|-----------|-------------|--------------|
| 1,000 | 0.00040 | ~0.12% |
| 10,000 | 0.00397 | ~1.2% |
| 100,000 | 0.0394 | ~11.5% |
| 1,000,000 | 0.382 | ~61% |

**Tip:** Larger trades have higher price impact due to AMM formula.

## Security Features

1. **Slippage Protection**: Enforced via `min_out` parameter
2. **Overflow Protection**: All arithmetic uses checked operations
3. **Pause Mechanism**: Emergency stop for security issues
4. **Fee Limits**: Prevents excessive fee changes
5. **Initialization Lock**: Can only be initialized once

## Best Practices

1. **Always set min_out**: Protect against front-running
2. **Check price first**: Use `get_amount_out()` before swapping
3. **Monitor reserves**: Track pool health via events
4. **Gradual fee collection**: Don't drain all fees at once
5. **Test on testnet**: Verify swaps before mainnet

## Next Steps

1. Deploy all 11 Tier 2 pools
2. Initialize each with ÉTR allocation
3. Connect to Tier 1 pools
4. Set up monitoring/alerts
5. Deploy UI for swaps

## License

MIT License - ĒTRID Project
