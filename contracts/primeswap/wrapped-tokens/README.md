# Wrapped Token Template

## Overview

The Wrapped Token template provides ERC20-like tokens that represent external currencies (BTC, ETH, SOL, etc.) locked in Tier 1 reserve pools.

## Purpose

- **Represent external assets** on the ĒTRID blockchain
- **Enable trustless trading** in Tier 2 AMM pools
- **Maintain 1:1 backing** with locked currencies
- **Support standard transfers** and approvals
- **Controlled minting/burning** by authorized Tier 1 pools

## Contract: `wrapped-token`

### Supported Wrapped Tokens

| Symbol | Name | Decimals | Backing |
|--------|------|----------|---------|
| wBTC | Wrapped Bitcoin | 8 | 1:1 BTC |
| wETH | Wrapped Ethereum | 18 | 1:1 ETH |
| wSOL | Wrapped Solana | 9 | 1:1 SOL |
| wBNB | Wrapped BNB | 18 | 1:1 BNB |
| wTRX | Wrapped Tron | 6 | 1:1 TRX |
| wXRP | Wrapped Ripple | 6 | 1:1 XRP |
| wADA | Wrapped Cardano | 6 | 1:1 ADA |
| wDOGE | Wrapped Dogecoin | 8 | 1:1 DOGE |
| wLINK | Wrapped Chainlink | 18 | 1:1 LINK |
| wXLM | Wrapped Stellar | 7 | 1:1 XLM |
| wMATIC | Wrapped Polygon | 18 | 1:1 MATIC |

### Key Features

1. **ERC20 Compatible**: Standard transfer, approve, transferFrom
2. **Controlled Minting**: Only authorized minters can mint
3. **Public Burning**: Anyone can burn their own tokens
4. **Minter Burning**: Authorized minters can burn from any account
5. **Emergency Pause**: Owner can pause all operations
6. **Audit Trail**: Complete minting/burning history

### Core Functions

#### Standard ERC20

**Transfer**
```rust
token.transfer(recipient, amount)?;
```

**Approve & TransferFrom**
```rust
// Approve spender
token.approve(spender, amount)?;

// Spender transfers
token.transfer_from(owner, recipient, amount)?;
```

**Allowance Management**
```rust
// Increase allowance (safer than approve)
token.increase_allowance(spender, amount)?;

// Decrease allowance
token.decrease_allowance(spender, amount)?;
```

**Queries**
```rust
let balance = token.balance_of(account);
let allowance = token.allowance(owner, spender);
let supply = token.total_supply();
let name = token.name();
let symbol = token.symbol();
let decimals = token.decimals();
```

#### Minting (Authorized Only)

**Mint to Account**
```rust
// Only owner or authorized minter can call
token.mint(recipient, amount)?;
```

**Example: Tier 1 Pool Minting wBTC**
```rust
// When user locks 1 BTC
tier1_pool.lock_and_mint(100_000_000)?;
  ↓
wbtc_token.mint(tier2_pool, 100_000_000)?;
  ↓
// 1 wBTC minted to Tier 2 pool
```

#### Burning

**Burn Own Tokens**
```rust
// Anyone can burn their own tokens
token.burn(amount)?;
```

**Burn From Account (Authorized)**
```rust
// Only owner or authorized minter
token.burn_from(account, amount)?;
```

**Example: Tier 1 Pool Burning wBTC**
```rust
// When user redeems 0.5 BTC
tier2_pool.swap_etr_for_wrapped(...)? // User gets 0.5 wBTC
  ↓
tier1_pool.burn_and_release(50_000_000, user)?;
  ↓
wbtc_token.burn_from(tier1_pool, 50_000_000)?;
  ↓
// 0.5 wBTC burned, 0.5 BTC released
```

### Minter Management

**Add Minter (Owner Only)**
```rust
token.add_minter(tier1_pool_address)?;
// Now tier1_pool can mint/burn
```

**Remove Minter (Owner Only)**
```rust
token.remove_minter(old_minter)?;
```

**Check Minter Status**
```rust
let is_minter = token.is_minter(address);
```

### Storage Structure

```rust
pub struct WrappedToken {
    total_supply: Balance,
    balances: Mapping<AccountId, Balance>,
    allowances: Mapping<(AccountId, AccountId), Balance>,
    name: String,
    symbol: String,
    decimals: u8,
    owner: AccountId,
    minters: Mapping<AccountId, bool>,
    paused: bool,
    total_minted: Balance,
    total_burned: Balance,
}
```

### Events

```rust
// Standard ERC20 events
event Transfer {
    from: Option<AccountId>,
    to: Option<AccountId>,
    value: Balance,
}

event Approval {
    owner: AccountId,
    spender: AccountId,
    value: Balance,
}

// Minting events
event Mint {
    to: AccountId,
    minter: AccountId,
    value: Balance,
    timestamp: Timestamp,
}

// Burning events
event Burn {
    from: AccountId,
    burner: AccountId,
    value: Balance,
    timestamp: Timestamp,
}

// Access control events
event MinterAdded {
    minter: AccountId,
    added_by: AccountId,
    timestamp: Timestamp,
}

event MinterRemoved {
    minter: AccountId,
    removed_by: AccountId,
    timestamp: Timestamp,
}
```

### Security Features

1. **Access Control**:
   - Only authorized minters can mint
   - Only authorized minters can burn from others
   - Only owner can add/remove minters

2. **Pause Mechanism**:
   - Owner can pause all operations
   - Prevents transfers, approvals, minting during pause
   - Emergency security measure

3. **Overflow Protection**:
   - All arithmetic uses checked operations
   - Prevents integer overflow attacks

4. **Zero Address Protection**:
   - Cannot transfer to zero address
   - Cannot approve zero address
   - Cannot mint to zero address

### Deployment

Deploy one wrapped token per external currency:

```bash
# Build contract
cargo contract build --release

# Deploy wBTC
cargo contract instantiate \
  --constructor new \
  --args "Wrapped Bitcoin" "wBTC" 8

# Deploy wETH
cargo contract instantiate \
  --constructor new \
  --args "Wrapped Ethereum" "wETH" 18

# Deploy wSOL
cargo contract instantiate \
  --constructor new \
  --args "Wrapped Solana" "wSOL" 9

# ... deploy all 11 wrapped tokens
```

### Post-Deployment Setup

```rust
// 1. Add Tier 1 pool as minter
wbtc.add_minter(tier1_btc_pool)?;

// 2. Optionally add Tier 2 pool (for burning)
wbtc.add_minter(tier2_btc_pool)?;

// 3. Verify minters
assert!(wbtc.is_minter(tier1_btc_pool));
```

### Testing

Run the comprehensive test suite:

```bash
cargo test
```

Tests cover:
- Token creation
- Minting (authorized)
- Burning (self and from)
- Transfers
- Approvals and allowances
- Minter management
- Pause functionality
- Error cases
- Edge cases

### Usage Examples

**Example 1: User Deposits BTC**
```rust
// 1. User locks 2.5 BTC in Tier 1
tier1_pool.lock_and_mint(250_000_000)?;

// 2. Tier 1 mints 2.5 wBTC
wbtc.mint(tier2_pool, 250_000_000)?;

// 3. Verify
assert_eq!(wbtc.total_supply(), 250_000_000);
assert_eq!(wbtc.balance_of(tier2_pool), 250_000_000);
```

**Example 2: User Swaps wBTC for ÉTR**
```rust
// 1. User has 1 wBTC
assert_eq!(wbtc.balance_of(user), 100_000_000);

// 2. User approves Tier 2 pool
wbtc.approve(tier2_pool, 100_000_000)?;

// 3. Swap executed
tier2_pool.swap_wrapped_for_etr(100_000_000, min_etr)?;

// 4. wBTC transferred to pool
assert_eq!(wbtc.balance_of(user), 0);
assert_eq!(wbtc.balance_of(tier2_pool), 100_000_000);
```

**Example 3: User Redeems for BTC**
```rust
// 1. User swaps ÉTR for wBTC
tier2_pool.swap_etr_for_wrapped(etr_amount, min_wbtc)?;

// 2. Tier 1 burns wBTC and releases BTC
tier1_pool.burn_and_release(wbtc_amount, user)?;
wbtc.burn_from(tier1_pool, wbtc_amount)?;

// 3. Multi-sig releases actual BTC to user
// (off-chain process)
```

### Admin Operations

**Pause/Unpause**
```rust
wbtc.pause()?;    // Emergency stop
wbtc.unpause()?;  // Resume
```

**Transfer Ownership**
```rust
wbtc.transfer_ownership(new_owner)?;
```

**Audit Queries**
```rust
let total_minted = wbtc.total_minted();
let total_burned = wbtc.total_burned();
let net_supply = wbtc.total_supply();

// Should match Tier 1 reserves
assert_eq!(net_supply, tier1_pool.total_reserves());
```

### Integration Checklist

- [ ] Deploy wrapped token contract
- [ ] Deploy Tier 1 reserve pool
- [ ] Add Tier 1 pool as minter
- [ ] Deploy Tier 2 trading pool
- [ ] Verify minting works
- [ ] Verify burning works
- [ ] Test full deposit flow
- [ ] Test full redemption flow
- [ ] Set up monitoring
- [ ] Audit reserves match supply

### Best Practices

1. **Minimize Minters**: Only add trusted contracts as minters
2. **Regular Audits**: Verify total_supply matches Tier 1 reserves
3. **Monitor Events**: Track all Mint/Burn events
4. **Use Pause Sparingly**: Only for emergencies
5. **Test Thoroughly**: Full integration tests before mainnet

### Decimal Precision

Different tokens use different decimal places:

```rust
BTC:  8 decimals  → 1 BTC = 100_000_000
ETH:  18 decimals → 1 ETH = 1_000_000_000_000_000_000
SOL:  9 decimals  → 1 SOL = 1_000_000_000
XRP:  6 decimals  → 1 XRP = 1_000_000
DOGE: 8 decimals  → 1 DOGE = 100_000_000
```

Always use the correct decimals when instantiating!

## License

MIT License - ĒTRID Project
