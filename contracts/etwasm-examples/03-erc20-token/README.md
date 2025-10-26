# ERC20 Token Contract

**Difficulty**: ⭐⭐⭐ Advanced
**Time to Complete**: 1-2 hours

---

## 📖 What You'll Learn

This contract implements the complete ERC20 standard for fungible tokens:
- ✅ **ERC20 standard interface** (transfer, approve, transferFrom)
- ✅ **Balance tracking** (Mapping per account)
- ✅ **Allowance mechanism** (delegated transfers)
- ✅ **Minting** (owner creates new tokens)
- ✅ **Burning** (destroy tokens)
- ✅ **Events** (Transfer, Approval indexed by accounts)
- ✅ **Metadata** (name, symbol, decimals)
- ✅ **Safety functions** (increase/decrease allowance)
- ✅ **Comprehensive error handling**

---

## 🏗️ Contract Overview

The ERC20 Token contract is the standard for fungible tokens on Ëtrid, compatible with Ethereum's ERC20.

### Storage
```rust
total_supply: Balance                           // Total tokens in existence
balances: Mapping<AccountId, Balance>           // Each account's balance
allowances: Mapping<(owner, spender), Balance>  // Spending permissions
name: String                                    // Token name
symbol: String                                  // Token symbol (ticker)
decimals: u8                                    // Decimal places
owner: AccountId                                // Contract owner
```

### Core Functions

| Function | Type | Access | Description |
|----------|------|--------|-------------|
| `new(supply, name, symbol, decimals)` | Constructor | Public | Create token |
| `name()` | Query | Public | Get token name |
| `symbol()` | Query | Public | Get token symbol |
| `decimals()` | Query | Public | Get decimals |
| `total_supply()` | Query | Public | Get total supply |
| `balance_of(owner)` | Query | Public | Get account balance |
| `allowance(owner, spender)` | Query | Public | Get spending allowance |
| `owner()` | Query | Public | Get contract owner |
| `transfer(to, value)` | Transaction | Public | Send tokens |
| `approve(spender, value)` | Transaction | Public | Allow spending |
| `transfer_from(from, to, value)` | Transaction | Public | Send on behalf |
| `increase_allowance(spender, value)` | Transaction | Public | Safely increase allowance |
| `decrease_allowance(spender, value)` | Transaction | Public | Safely decrease allowance |
| `mint(to, value)` | Transaction | Owner Only | Create new tokens |
| `burn(value)` | Transaction | Public | Destroy own tokens |
| `transfer_ownership(new_owner)` | Transaction | Owner Only | Change owner |

### Events
- `Transfer(from, to, value)` - Tokens transferred (indexed by from/to)
- `Approval(owner, spender, value)` - Allowance set (indexed by owner/spender)
- `Mint(to, value)` - Tokens minted (indexed by recipient)
- `Burn(from, value)` - Tokens burned (indexed by burner)

### Errors
- `InsufficientBalance` - Not enough tokens
- `InsufficientAllowance` - Not enough allowance
- `TransferToZeroAddress` - Cannot send to 0x0
- `ApproveToZeroAddress` - Cannot approve 0x0
- `NotOwner` - Caller not owner
- `Overflow` - Arithmetic overflow
- `ZeroAmount` - Amount is zero

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 03-erc20-token
cargo contract build --release
```

### 2. Deploy Contract

```bash
# Deploy with 1 million tokens, 18 decimals
cargo contract instantiate \
  --constructor new \
  --args 1000000000000000000000000 "My Token" "MTK" 18 \
  --suri //Alice
```

**Note**: `1000000000000000000000000` = 1,000,000 tokens with 18 decimals

### 3. Interact with Contract

```bash
# Get token info
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message name \
  --suri //Alice \
  --dry-run

cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message symbol \
  --suri //Alice \
  --dry-run

cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message total_supply \
  --suri //Alice \
  --dry-run

# Check balance
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message balance_of \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --suri //Alice \
  --dry-run

# Transfer 100 tokens (with 18 decimals = 100000000000000000000)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message transfer \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 100000000000000000000 \
  --suri //Alice

# Approve Bob to spend 50 tokens
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message approve \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 50000000000000000000 \
  --suri //Alice

# Check allowance
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message allowance \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty \
  --suri //Alice \
  --dry-run

# Transfer from (as approved spender)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message transfer_from \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FLSigC9HGgKVZdwjRuSpzL4YZTYCBz9VYYzc8bDJsGTAQHT 25000000000000000000 \
  --suri //Bob

# Mint new tokens (owner only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message mint \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 1000000000000000000000 \
  --suri //Alice

# Burn your tokens
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message burn \
  --args 500000000000000000000 \
  --suri //Alice
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

**Expected output**: All 19 tests pass

**Tests cover**:
- Token creation and metadata
- Balance tracking
- Transfer functionality
- Approval mechanism
- TransferFrom with allowances
- Increase/decrease allowance
- Minting (owner only)
- Burning
- Access control
- Complex multi-step scenarios

### Run Integration Tests (E2E)

```bash
# Start local node
substrate-contracts-node --dev --tmp

# Run E2E tests
cargo test --features e2e-tests
```

---

## 📝 Code Walkthrough

### Token Decimals

```rust
// If decimals = 18, then:
// 1 token = 1_000_000_000_000_000_000 (1e18)
// 0.5 tokens = 500_000_000_000_000_000

// Example: 1,000,000 tokens with 18 decimals
let total_supply = 1_000_000 * 10u128.pow(18);
```

**Why decimals?** Allows fractional tokens (e.g., 0.5 tokens)

### Transfer Function

```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
    let from = self.env().caller();

    // Validation
    if value == 0 { return Err(Error::ZeroAmount); }
    if to == ZERO_ADDRESS { return Err(Error::TransferToZeroAddress); }

    // Check balance
    let from_balance = self.balance_of(from);
    if from_balance < value { return Err(Error::InsufficientBalance); }

    // Update balances
    self.balances.insert(&from, &(from_balance - value));
    let to_balance = self.balance_of(to);
    self.balances.insert(&to, &(to_balance + value));

    // Emit event
    self.env().emit_event(Transfer { from: Some(from), to: Some(to), value });

    Ok(())
}
```

### Allowance Mechanism

```rust
// Alice approves Bob to spend 100 tokens
alice.approve(bob, 100);

// Bob can now transfer up to 100 of Alice's tokens
bob.transfer_from(alice, charlie, 50);

// Remaining allowance: 50
```

**Use case**: DEX contracts, payment processors, automated transfers

### Safe Allowance Functions

```rust
// Problem: Race condition with approve()
// Solution: Use increase/decrease_allowance

// Instead of:
token.approve(spender, 100);  // Could be front-run

// Use:
token.increase_allowance(spender, 50);  // Safer
token.decrease_allowance(spender, 25);  // Safer
```

### Minting vs. Initial Supply

```rust
// Option 1: Mint all tokens at creation
let token = Erc20Token::new(1_000_000, "Token", "TKN", 18);

// Option 2: Start with 0, mint later
let mut token = Erc20Token::new(0, "Token", "TKN", 18);
token.mint(alice, 500_000);  // Owner only
token.mint(bob, 500_000);
```

### Burning Tokens

```rust
// Anyone can burn their own tokens
token.burn(100);  // Reduces total_supply

// Use cases:
// - Deflationary tokenomics
// - Remove excess supply
// - Burn fees/taxes
```

---

## 💡 Try It Yourself

### Challenge 1: Add Pausable
Add a `paused: bool` flag that prevents transfers when true (owner can toggle).

**Hint**: Check `paused` at start of `transfer()` and `transfer_from()`.

### Challenge 2: Add Transaction Fee
Charge a 1% fee on transfers (sent to owner).

**Hint**: Deduct 1% from transfer amount, send to owner.

### Challenge 3: Add Snapshot
Implement `snapshot()` to record all balances at a point in time.

**Hint**: Store `Vec<(AccountId, Balance)>` with block number.

### Challenge 4: Add Vesting
Lock tokens for a period, gradually unlocking over time.

**Hint**: Store `locked_until: Mapping<AccountId, BlockNumber>`.

### Challenge 5: Add Whitelist
Only allow transfers to/from whitelisted addresses.

**Hint**: `Mapping<AccountId, bool>` for whitelist status.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~80,000 | One-time |
| `balance_of()` | ~150 | Read from mapping |
| `transfer()` | ~4,000 | 2 balance updates + event |
| `approve()` | ~3,000 | 1 allowance update + event |
| `transfer_from()` | ~5,500 | 3 updates (balances + allowance) + event |
| `mint()` | ~4,500 | 1 balance + supply update + events |
| `burn()` | ~4,000 | 1 balance + supply update + events |

**Optimization tips**:
- Batch transfers cost more but save on transaction overhead
- Read operations (balance_of, allowance) are cheap
- Events add ~500 VMw each

---

## 🌐 Real-World Use Cases

### 1. Utility Token
```rust
// ÉTR token on Ëtrid
Erc20Token::new(
    1_000_000_000 * 10u128.pow(18),  // 1 billion tokens
    "Ëtrid".to_string(),
    "ÉTR".to_string(),
    18
)
```

### 2. Governance Token
```rust
// Used for voting in DAOs
Erc20Token::new(
    10_000_000 * 10u128.pow(18),
    "Governance Token".to_string(),
    "GOV".to_string(),
    18
)
```

### 3. Stablecoin
```rust
// EDSC stablecoin
Erc20Token::new(
    0,  // Mint as needed
    "Ëtrid Dollar Stablecoin".to_string(),
    "EDSC".to_string(),
    6  // Common for stablecoins
)
```

### 4. Reward Token
```rust
// Distribute to users as rewards
token.mint(user, reward_amount);
```

---

## 📊 Comparison to Previous Examples

| Feature | Hello World | Counter | ERC20 Token |
|---------|-------------|---------|-------------|
| Storage complexity | Simple | Medium | Complex |
| Mappings | None | 1 | 2 (nested) |
| Events | Basic | Indexed | Multiple indexed |
| Access control | None | Owner only | Owner + allowances |
| Error handling | Basic | Moderate | Comprehensive |
| Real-world usage | Learning | Demo | Production-ready |

---

## 🐛 Common Issues

### "InsufficientBalance: not enough tokens"
**Cause**: Trying to transfer more than you have
**Solution**: Check `balance_of()` before transferring

### "InsufficientAllowance: not enough allowance"
**Cause**: Spender trying to transfer more than approved
**Solution**: Increase allowance with `approve()` or `increase_allowance()`

### "Overflow: arithmetic overflow"
**Cause**: Balance or supply would exceed u128::MAX
**Solution**: Use smaller amounts or implement balance caps

### "Decimals confusion"
**Cause**: Forgetting to multiply by 10^decimals
**Solution**: Always use: `amount * 10^decimals` for user-facing values

---

## 📚 Next Steps

After mastering this contract:
1. **Move to Simple DAO** (`04-simple-dao`) - Use tokens for governance
2. **Build a DEX** - Create token swap contract
3. **Deploy to Ember testnet** - Launch your own token!

---

## 📖 Resources

- ERC20 Standard: https://eips.ethereum.org/EIPS/eip-20
- ink! Token example: https://use.ink/examples/erc20
- OpenZeppelin ERC20: https://docs.openzeppelin.com/contracts/erc20

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy Token Building! 🪙🎉**
