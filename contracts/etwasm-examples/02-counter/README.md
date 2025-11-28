# Counter Contract

**Difficulty**: ⭐⭐ Intermediate
**Time to Complete**: 30-45 minutes

---

## 📖 What You'll Learn

This contract builds on Hello World with more advanced patterns:
- ✅ **Mapping storage** (key-value pairs)
- ✅ **Per-user state** (each user has their own counter)
- ✅ **Global state** (shared across all users)
- ✅ **Access control** (owner-only functions)
- ✅ **Event indexing** (searchable events)
- ✅ **Overflow/underflow protection**
- ✅ **Batch operations**
- ✅ **Multiple test scenarios**

---

## 🏗️ Contract Overview

The Counter contract tracks both global and per-user counters. Each user can increment/decrement their own counter, and the global counter aggregates all users.

### Storage
```rust
global_count: u64                      // Total across all users
user_counts: Mapping<AccountId, u64>   // Per-user counters
owner: AccountId                       // Contract owner
total_users: u32                       // Unique users count
```

### Functions

| Function | Type | Access | Description |
|----------|------|--------|-------------|
| `new()` | Constructor | Public | Create with counter at 0 |
| `new_with_value(n)` | Constructor | Public | Create with counter at n |
| `get_global()` | Query | Public | Get total count (all users) |
| `get_mine()` | Query | Public | Get caller's count |
| `get_user_count(user)` | Query | Public | Get specific user's count |
| `get_total_users()` | Query | Public | Get number of unique users |
| `get_owner()` | Query | Public | Get contract owner |
| `increment()` | Transaction | Public | Increment by 1 |
| `increment_by(n)` | Transaction | Public | Increment by n |
| `decrement()` | Transaction | Public | Decrement by 1 |
| `decrement_by(n)` | Transaction | Public | Decrement by n |
| `reset_mine()` | Transaction | Public | Reset own counter to 0 |
| `reset_global()` | Transaction | Owner Only | Reset global counter |
| `transfer_ownership(new)` | Transaction | Owner Only | Change owner |
| `batch_increment(count)` | Transaction | Public | Increment multiple times |

### Events
- `Incremented` - When a user increments (indexed by user)
- `Decremented` - When a user decrements (indexed by user)
- `GlobalReset` - When owner resets global counter
- `NewUser` - When a new user interacts with contract

### Errors
- `NotOwner` - Caller is not the contract owner
- `Underflow` - Cannot decrement below zero
- `Overflow` - Counter would exceed u64::MAX
- `InvalidAmount` - Amount is zero

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 02-counter
cargo contract build --release
```

### 2. Deploy Contract

```bash
# Deploy with default (0)
cargo contract instantiate \
  --constructor new \
  --suri //Alice

# Or with initial value
cargo contract instantiate \
  --constructor new_with_value \
  --args 100 \
  --suri //Alice
```

### 3. Interact with Contract

```bash
# Increment by 1
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message increment \
  --suri //Alice

# Increment by 10
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message increment_by \
  --args 10 \
  --suri //Alice

# Get your count
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_mine \
  --suri //Alice \
  --dry-run

# Get global count
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_global \
  --suri //Alice \
  --dry-run

# Decrement
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message decrement \
  --suri //Alice

# Reset your counter
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message reset_mine \
  --suri //Alice

# Reset global (owner only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message reset_global \
  --suri //Alice
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

**Expected output**: All 17 tests pass

**Tests cover**:
- Basic increment/decrement
- Overflow/underflow protection
- Access control (owner vs non-owner)
- Multi-user scenarios
- Batch operations
- Error handling

### Run Integration Tests (E2E)

```bash
# Start local node
substrate-contracts-node --dev --tmp

# Run E2E tests
cargo test --features e2e-tests
```

---

## 📝 Code Walkthrough

### Mapping Storage

```rust
#[ink(storage)]
pub struct Counter {
    // Mapping: key → value
    user_counts: Mapping<AccountId, u64>,
}

// Read from mapping
let count = self.user_counts.get(&caller).unwrap_or(0);

// Write to mapping
self.user_counts.insert(&caller, &new_value);
```

**Key Points**:
- `Mapping` is like a HashMap but optimized for blockchain storage
- `get()` returns `Option<T>` (use `unwrap_or(default)`)
- Each entry costs storage rent
- More efficient than `Vec` for sparse data

### Access Control

```rust
#[ink(message)]
pub fn reset_global(&mut self) -> Result<()> {
    let caller = self.env().caller();
    if caller != self.owner {
        return Err(Error::NotOwner);
    }
    // ... owner-only logic
}
```

**Pattern**: Check `self.env().caller()` and return error if unauthorized

### Overflow/Underflow Protection

```rust
// Safe addition (returns None on overflow)
let new_value = current.checked_add(amount)
    .ok_or(Error::Overflow)?;

// Safe subtraction (returns None on underflow)
let new_value = current.checked_sub(amount)
    .ok_or(Error::Underflow)?;
```

**Important**: Always use `checked_*` methods for arithmetic!

### Event Indexing

```rust
#[ink(event)]
pub struct Incremented {
    #[ink(topic)]  // Indexed for fast lookup
    by: AccountId,
    amount: u64,    // Not indexed
    new_value: u64, // Not indexed
}
```

**Use Cases**:
- Index fields you'll search by (user addresses, IDs)
- Don't index large data (wastes gas)

---

## 💡 Try It Yourself

### Challenge 1: Add Limits
Add a `max_value: u64` field that prevents incrementing beyond a limit.

**Hint**: Add to constructor and check in `increment_by()`.

### Challenge 2: Track History
Store a `Vec<u64>` of the last 10 values for each user.

**Hint**: Use a bounded `Vec` to avoid unbounded growth.

### Challenge 3: Leaderboard
Add a function `get_top_users(n: u32)` that returns the top N users by count.

**Hint**: This is expensive on-chain! Consider off-chain indexing.

### Challenge 4: Cooldown Period
Prevent users from incrementing more than once per minute.

**Hint**: Store last increment timestamp per user.

### Challenge 5: Multiplier
Add a `multiplier: u32` that multiplies increment amounts.

**Hint**: Owner can change multiplier, affects all users.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~60,000 | One-time |
| `get_mine()` | ~100 | Read from mapping |
| `increment()` | ~3,000 | Write to mapping + emit event |
| `increment_by(100)` | ~3,500 | Slightly more expensive |
| `batch_increment(10)` | ~30,000 | 10× increment cost |
| `reset_global()` | ~2,500 | Owner-only write |

**Optimization Tips**:
- Use batch operations sparingly (expensive!)
- Read operations are cheap (no state changes)
- Mappings are more efficient than iterating `Vec`

---

## 🆚 Comparison to Hello World

| Feature | Hello World | Counter |
|---------|-------------|---------|
| Storage type | Single `String` | `Mapping` + scalars |
| State complexity | Simple | Per-user state |
| Access control | None | Owner permissions |
| Events | Basic | Indexed by user |
| Error handling | Simple | Overflow/underflow |
| Tests | 7 | 17 |

---

## 🐛 Common Issues

### "Underflow: cannot decrement below zero"
**Cause**: Trying to decrement when counter is 0
**Solution**: Check value before decrementing or handle error

### "NotOwner: caller is not the owner"
**Cause**: Non-owner trying to call `reset_global()`
**Solution**: Call from owner account or use `transfer_ownership()`

### "Overflow: counter would exceed maximum"
**Cause**: Incrementing would exceed u64::MAX
**Solution**: Use smaller increments or reset counter

---

## 📚 Next Steps

After mastering this contract:
1. **Move to ERC20 Token** (`03-erc20-token`) - Learn token standards
2. **Try the challenges above** - Extend this contract
3. **Deploy to Ember testnet** - Test with real network

---

## 📖 Resources

- ink! Mapping docs: https://use.ink/datastructures/mapping
- Access control patterns: https://use.ink/basics/access-control
- Event topics: https://use.ink/macros-attributes/event

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy Coding! 🎉**
