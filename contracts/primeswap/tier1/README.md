# Tier 1: External Currency Reserve Pool

## Overview

The External Currency Reserve Pool is the foundation of the PrimeSwap two-tier architecture. It provides secure, 1:1 backed wrapped tokens for external currencies (BTC, ETH, SOL, etc.).

## Purpose

- **Lock external currencies** in a secure reserve vault
- **Mint wrapped tokens** 1:1 with locked assets
- **Guarantee 100% backing** at all times
- **Enable redemptions** through multi-sig controlled releases
- **Maintain proof of reserves** for transparency

## Contract: `external-currency-pool`

### Key Features

1. **One-sided Pool**: Only holds the external currency (e.g., BTC)
2. **1:1 Backing**: Every wrapped token is backed by exactly 1 unit of the external currency
3. **Multi-sig Security**: Withdrawals require multi-sig approval (3-of-5)
4. **Rate Limiting**: Transaction and daily withdrawal limits
5. **Emergency Pause**: Can be paused in case of security concerns
6. **Audit Trail**: Tracks all deposits, withdrawals, and reserves

### Supported Currencies

The contract template supports all 11 external currencies:

- **Bitcoin (BTC)** → wBTC
- **Ethereum (ETH)** → wETH
- **Solana (SOL)** → wSOL
- **BNB Chain (BNB)** → wBNB
- **Tron (TRX)** → wTRX
- **Ripple (XRP)** → wXRP
- **Cardano (ADA)** → wADA
- **Dogecoin (DOGE)** → wDOGE
- **Chainlink (LINK)** → wLINK
- **Stellar (XLM)** → wXLM
- **Polygon (MATIC)** → wMATIC

### Core Functions

#### `lock_and_mint(amount: Balance) -> Result<Balance>`

Locks external currency and mints wrapped tokens 1:1.

**Example:**
```rust
// User locks 1 BTC
let wrapped_amount = pool.lock_and_mint(100_000_000)?; // 1 BTC (8 decimals)
// Returns: 100_000_000 wBTC minted
```

**Security Checks:**
- Contract must not be paused
- Amount must be non-zero
- Amount must not exceed transaction limit
- No overflow in reserves

#### `burn_and_release(amount: Balance, recipient: AccountId) -> Result<()>`

Burns wrapped tokens and releases external currency (multi-sig only).

**Example:**
```rust
// Multi-sig approves release of 0.5 BTC
pool.burn_and_release(50_000_000, user_address)?;
// Burns 0.5 wBTC, releases 0.5 BTC to user
```

**Security Checks:**
- Contract must not be paused
- Caller must be multi-sig
- Amount must be non-zero
- Sufficient reserves available
- Daily limit not exceeded

#### `get_reserve_ratio() -> u128`

Returns the reserve ratio (should always be 100%).

**Example:**
```rust
let ratio = pool.get_reserve_ratio();
// Returns: 1_000_000 (= 100%)
```

### Storage Structure

```rust
pub struct ExternalCurrencyPool {
    external_currency: String,        // "BTC"
    wrapped_token: AccountId,         // wBTC contract address
    total_reserves: Balance,          // Total BTC locked
    tier2_pool: AccountId,            // Tier 2 trading pool
    multi_sig: AccountId,             // 3-of-5 multi-sig wallet
    owner: AccountId,                 // Contract owner
    paused: bool,                     // Emergency pause
    user_reserves: Mapping<...>,      // Per-user reserves
    total_minted: Balance,            // Total wBTC minted
    max_tx_limit: Balance,            // Max per transaction
    daily_limit: Balance,             // Max daily withdrawals
}
```

### Events

```rust
// Emitted when currency is locked and wrapped tokens minted
event Locked {
    user: AccountId,
    amount: Balance,
    wrapped_minted: Balance,
    timestamp: Timestamp,
}

// Emitted when wrapped tokens burned and currency released
event Released {
    user: AccountId,
    amount: Balance,
    wrapped_burned: Balance,
    timestamp: Timestamp,
}

// Emitted on reserve updates
event ReserveUpdate {
    total_reserves: Balance,
    total_minted: Balance,
    reserve_ratio: u128,
    timestamp: Timestamp,
}
```

### Security Features

1. **Multi-sig Withdrawals**: Only approved multi-sig can release funds
2. **Rate Limiting**:
   - Max transaction limit (configurable)
   - Daily withdrawal limit (configurable)
3. **Emergency Pause**: Owner can pause all operations
4. **Overflow Protection**: All arithmetic uses checked operations
5. **Reserve Verification**: Constant 1:1 ratio maintained

### Integration with Tier 2

```
┌────────────────────────────────────────────────┐
│ TIER 1: External Currency Reserve Pool        │
│                                                │
│ BTC locked → wBTC minted → sent to Tier 2 ───┐│
│                                                ││
└────────────────────────────────────────────────┘│
                                                  │
                                                  ↓
┌────────────────────────────────────────────────┐
│ TIER 2: ÉTR/wBTC Trading Pool                 │
│                                                │
│ wBTC swapped for ÉTR → User receives ÉTR      │
└────────────────────────────────────────────────┘
```

### Deployment

Each currency requires its own Tier 1 pool instance:

```bash
# Deploy BTC pool
cargo contract build --release
cargo contract instantiate \
  --constructor new \
  --args "BTC" <wBTC_address> <tier2_address> <multisig_address> 1000000 10000000

# Deploy ETH pool
cargo contract instantiate \
  --constructor new \
  --args "ETH" <wETH_address> <tier2_address> <multisig_address> 1000000 10000000

# ... deploy all 11 pools
```

### Testing

Run the comprehensive test suite:

```bash
cargo test
```

The contract includes 15+ unit tests covering:
- Lock and mint operations
- Burn and release operations
- Reserve ratio maintenance
- Access control
- Pause/unpause functionality
- Rate limiting
- Error handling

### Admin Operations

#### Pause/Unpause
```rust
pool.pause()?;    // Emergency stop
pool.unpause()?;  // Resume operations
```

#### Update Multi-sig
```rust
pool.update_multi_sig(new_multisig_address)?;
```

#### Update Limits
```rust
pool.update_max_tx_limit(new_limit)?;
pool.update_daily_limit(new_limit)?;
```

## Architecture Benefits

1. **Security**: Multi-sig control prevents unauthorized withdrawals
2. **Transparency**: All reserves are verifiable on-chain
3. **Trustless**: 1:1 backing mathematically enforced
4. **Auditable**: Complete history of all operations
5. **Scalable**: Same template for all 11 currencies

## Next Steps

1. Deploy all 11 Tier 1 pools
2. Set up multi-sig wallets (3-of-5)
3. Configure transaction and daily limits
4. Connect to Tier 2 trading pools
5. Integrate with bridge infrastructure

## License

MIT License - ĒTRID Project
