# Escrow Contract

**Difficulty**: ⭐⭐⭐⭐ Advanced
**Time to Complete**: 2-3 hours

---

## 📖 What You'll Learn

This contract implements a trustless escrow system for secure transactions:
- ✅ **Three-party system** (buyer, seller, arbiter)
- ✅ **Fund locking** (secure payment holding)
- ✅ **State machine** (AwaitingPayment → AwaitingDelivery → Complete/Refunded)
- ✅ **Delivery confirmation** (buyer releases funds)
- ✅ **Timeout refunds** (automatic refund if seller doesn't deliver)
- ✅ **Dispute resolution** (arbiter decides fund allocation)
- ✅ **Payable functions** (receiving and transferring funds)
- ✅ **Comprehensive events** (all state changes logged)
- ✅ **Robust error handling**

---

## 🏗️ Contract Overview

The Escrow contract enables secure peer-to-peer transactions with third-party arbitration.

### Storage
```rust
escrows: Mapping<u32, EscrowDetails>  // All escrow agreements
next_escrow_id: u32                   // Auto-incrementing ID
balances: Mapping<u32, Balance>       // Funds held per escrow
```

### Escrow Details Structure
```rust
pub struct EscrowDetails {
    id: u32,                // Unique identifier
    buyer: AccountId,       // Purchaser
    seller: AccountId,      // Provider
    arbiter: AccountId,     // Dispute resolver
    amount: Balance,        // Escrow amount
    description: String,    // Transaction details
    state: EscrowState,     // Current status
    created_at: Timestamp,  // Creation time
    timeout_at: Timestamp,  // Refund deadline
}
```

### Escrow Lifecycle
```
1. Create Escrow (buyer creates agreement)
         ↓
2. AwaitingPayment (waiting for buyer deposit)
         ↓
3. Deposit (buyer sends funds)
         ↓
4. AwaitingDelivery (seller delivers goods/services)
         ↓
5a. Confirm Delivery (buyer releases to seller) → Complete
5b. Raise Dispute → Disputed → Arbiter resolves
5c. Timeout Refund (deadline passed) → Refunded
```

### States

| State | Description | Next States |
|-------|-------------|-------------|
| `AwaitingPayment` | Created, waiting for deposit | AwaitingDelivery |
| `AwaitingDelivery` | Paid, waiting for delivery | Complete, Refunded, Disputed |
| `Complete` | Funds released to seller | Final state |
| `Refunded` | Funds returned to buyer | Final state |
| `Disputed` | Dispute raised, needs arbiter | Complete, Refunded |

### Core Functions

| Function | Type | Access | Description |
|----------|------|--------|-------------|
| `new()` | Constructor | Public | Create escrow contract |
| `create_escrow(seller, arbiter, amount, description, timeout_days)` | Transaction | Public | Create new escrow |
| `get_escrow(id)` | Query | Public | Get escrow details |
| `escrow_count()` | Query | Public | Get total escrows |
| `deposit(id)` | Transaction (Payable) | Buyer Only | Deposit funds |
| `confirm_delivery(id)` | Transaction | Buyer Only | Release to seller |
| `request_refund(id)` | Transaction | Buyer Only | Refund after timeout |
| `raise_dispute(id)` | Transaction | Buyer/Seller | Raise dispute |
| `resolve_dispute(id, release_to_seller)` | Transaction | Arbiter Only | Resolve dispute |
| `get_balance(id)` | Query | Public | Get escrow balance |

### Events
- `EscrowCreated(id, buyer, seller, arbiter, amount)` - New escrow
- `PaymentDeposited(id, buyer, amount)` - Funds deposited
- `DeliveryConfirmed(id, buyer)` - Buyer confirmed
- `FundsReleased(id, seller, amount)` - Paid to seller
- `FundsRefunded(id, buyer, amount)` - Refunded to buyer
- `DisputeRaised(id, raised_by)` - Dispute created
- `DisputeResolved(id, arbiter, release_to_seller)` - Dispute settled

### Errors
- `NotBuyer` - Caller is not the buyer
- `NotSeller` - Caller is not the seller
- `NotArbiter` - Caller is not the arbiter
- `NotAuthorized` - Caller not authorized for action
- `InvalidState` - Invalid state for action
- `InvalidAmount` - Incorrect amount
- `InsufficientFunds` - Not enough funds
- `EscrowNotFound` - Invalid escrow ID
- `AlreadyPaid` - Already deposited
- `NotPaid` - Not yet deposited
- `TimeoutNotReached` - Cannot refund before timeout
- `AlreadyComplete` - Already finalized
- `EmptyDescription` - Description required
- `TransferFailed` - Fund transfer failed

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 05-escrow
cargo contract build --release
```

### 2. Deploy Contract

```bash
cargo contract instantiate \
  --constructor new \
  --suri //Alice
```

### 3. Interact with Contract

```bash
# Create escrow (Alice buys from Bob, Charlie is arbiter)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message create_escrow \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 5FLSigC9HGgKVZdwjRuSpzL4YZTYCBz9VYYzc8bDJsGTAQHT 1000000000000000000 "Website development" 7 \
  --suri //Alice

# Get escrow details
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_escrow \
  --args 0 \
  --suri //Alice \
  --dry-run

# Deposit funds (buyer sends 1 ETR)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message deposit \
  --args 0 \
  --value 1000000000000000000 \
  --suri //Alice

# Check escrow balance
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_balance \
  --args 0 \
  --suri //Alice \
  --dry-run

# Confirm delivery (buyer releases funds)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message confirm_delivery \
  --args 0 \
  --suri //Alice

# Raise dispute (if issue occurs)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message raise_dispute \
  --args 0 \
  --suri //Alice

# Resolve dispute (arbiter decides)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message resolve_dispute \
  --args 0 true \
  --suri //Charlie

# Request refund (after timeout)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message request_refund \
  --args 0 \
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
- Escrow creation
- Fund deposit
- Delivery confirmation
- Refund requests
- Dispute raising
- Dispute resolution
- Access control
- State transitions
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

### Creating an Escrow

```rust
// Alice (buyer) creates escrow with Bob (seller) and Charlie (arbiter)
let escrow_id = escrow.create_escrow(
    bob,           // seller
    charlie,       // arbiter
    1000,          // amount (1000 base units)
    "Website development".into(),
    7              // timeout in days
)?;

// State: AwaitingPayment
```

### Depositing Funds

```rust
// Buyer deposits exactly the escrow amount
// #[ink(message, payable)] allows receiving funds
escrow.deposit(escrow_id)?;

// Funds now locked in contract
// State: AwaitingPayment → AwaitingDelivery
```

**Important**: Must send exactly the escrow amount with the transaction.

### Happy Path: Delivery Confirmed

```rust
// Seller delivers goods/services
// Buyer confirms delivery
escrow.confirm_delivery(escrow_id)?;

// Funds automatically transferred to seller
// State: AwaitingDelivery → Complete
```

### Timeout Refund

```rust
// If seller doesn't deliver before timeout
// Buyer can request refund

// Check if timeout passed
let now = current_timestamp();
if now > escrow.timeout_at {
    escrow.request_refund(escrow_id)?;
    // Funds returned to buyer
    // State: AwaitingDelivery → Refunded
}
```

### Dispute Resolution

```rust
// Either party raises dispute
escrow.raise_dispute(escrow_id)?;
// State: AwaitingDelivery → Disputed

// Arbiter investigates and decides
arbiter.resolve_dispute(
    escrow_id,
    true  // true = release to seller, false = refund to buyer
)?;

// Funds transferred per arbiter decision
// State: Disputed → Complete/Refunded
```

---

## 💡 Try It Yourself

### Challenge 1: Add Partial Releases
Allow buyer to release funds in stages (e.g., 50% upfront, 50% on completion).

**Hint**: Add `released_amount: Balance` to `EscrowDetails`, track partial releases.

### Challenge 2: Add Milestone System
Break escrow into multiple milestones with separate confirmations.

**Hint**: Store `Vec<Milestone>` with amount and status per milestone.

### Challenge 3: Add Seller Deposits
Require seller to deposit a stake (e.g., 10% of amount) for commitment.

**Hint**: Add `seller_deposit: Balance`, require deposit before buyer payment.

### Challenge 4: Add Arbiter Fees
Charge a percentage fee to arbiter for dispute resolution.

**Hint**: Deduct fee before final transfer, send to arbiter.

### Challenge 5: Add Review System
Allow buyer and seller to leave ratings after completion.

**Hint**: Add `buyer_rating: Option<u8>`, `seller_rating: Option<u8>`.

### Challenge 6: Add Multi-Signature
Require multiple arbiters to agree on dispute resolution.

**Hint**: Store `Vec<AccountId>` arbiters, track votes, require threshold.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~180,000 | One-time |
| `create_escrow()` | ~10,000 | Store escrow details + event |
| `deposit()` | ~8,000 | Receive funds + update state |
| `confirm_delivery()` | ~12,000 | Transfer funds + 2 events |
| `raise_dispute()` | ~6,000 | Update state + event |
| `resolve_dispute()` | ~14,000 | Transfer funds + events |
| `request_refund()` | ~12,000 | Transfer funds + event |
| `get_escrow()` | ~400 | Read escrow details |

**Optimization tips**:
- Keep descriptions concise to reduce storage
- Archive completed escrows off-chain
- Batch multiple escrows if possible
- Use indexed events for efficient querying

---

## 🌐 Real-World Use Cases

### 1. Freelance Work
```rust
// Client hires developer
escrow.create_escrow(
    developer,
    trusted_third_party,
    5000 * ETR,
    "Build mobile app",
    30  // 30-day deadline
);

// Developer delivers → client confirms → developer paid
```

### 2. E-Commerce
```rust
// Buyer purchases physical goods
escrow.create_escrow(
    seller,
    platform_arbiter,
    100 * ETR,
    "Laptop Model X",
    14  // 14 days for delivery
);

// Seller ships → buyer receives → confirms → seller paid
```

### 3. Real Estate Deposits
```rust
// Buyer makes deposit on property
escrow.create_escrow(
    seller,
    lawyer,
    50000 * ETR,
    "Property deposit for 123 Main St",
    90  // 90-day closing period
);
```

### 4. Domain Name Sales
```rust
// Buyer purchases domain
escrow.create_escrow(
    domain_seller,
    registrar,
    1000 * ETR,
    "Transfer of example.com domain",
    7
);
```

### 5. Service Subscriptions
```rust
// Pay for annual service upfront
escrow.create_escrow(
    service_provider,
    consumer_protection_org,
    1200 * ETR,
    "Annual software subscription",
    365
);
```

---

## 📊 Comparison to Previous Examples

| Feature | ERC20 | Simple DAO | Escrow |
|---------|-------|------------|--------|
| Storage complexity | High | Very High | High |
| State management | Balances | Proposal lifecycle | Escrow lifecycle |
| Time-based logic | None | Voting periods | Timeouts |
| Fund handling | Token transfers | None | Native token locking |
| Multi-party | Approvals | Voting | Buyer/Seller/Arbiter |
| Dispute handling | None | None | Full arbitration |
| Real-world usage | Tokens | Governance | Secure payments |

---

## 🐛 Common Issues

### "InvalidAmount: incorrect amount"
**Cause**: Deposited amount doesn't match escrow amount
**Solution**: Send exactly the amount specified in `create_escrow()`

### "NotBuyer: caller is not the buyer"
**Cause**: Non-buyer trying to perform buyer-only action
**Solution**: Use buyer account for deposit/confirm/refund

### "InvalidState: invalid state for action"
**Cause**: Action not valid in current state
**Solution**: Check current state with `get_escrow()`, follow lifecycle

### "TimeoutNotReached: cannot refund before timeout"
**Cause**: Requesting refund before deadline
**Solution**: Wait for timeout period to expire

### "TransferFailed: fund transfer failed"
**Cause**: Contract can't send funds (insufficient balance)
**Solution**: Ensure contract has sufficient balance (should be automatic)

---

## 🔒 Security Considerations

### Reentrancy Protection
The contract updates state before transferring funds to prevent reentrancy attacks:

```rust
// ✅ SAFE: State updated first
escrow.state = EscrowState::Complete;
self.escrows.insert(&escrow_id, &escrow);
self.balances.insert(&escrow_id, &0);
self.env().transfer(seller, balance)?;  // Transfer last
```

### Access Control
All sensitive functions check caller authorization:

```rust
// Only buyer can confirm delivery
if caller != escrow.buyer {
    return Err(Error::NotBuyer);
}
```

### State Machine Enforcement
Functions validate state before executing:

```rust
// Must be in AwaitingDelivery to confirm
if escrow.state != EscrowState::AwaitingDelivery {
    return Err(Error::InvalidState);
}
```

### Amount Validation
Deposits must match exactly:

```rust
let deposited = self.env().transferred_value();
if deposited != escrow.amount {
    return Err(Error::InvalidAmount);
}
```

---

## 🔗 Integrating with Other Contracts

### Example: Escrow + ERC20 Tokens

```rust
// Instead of native ETR, use ERC20 tokens
// Modify to accept token contract address
// Use token.transfer_from() instead of self.env().transfer()

pub fn create_escrow(
    &mut self,
    token_contract: AccountId,
    seller: AccountId,
    arbiter: AccountId,
    amount: Balance,
    description: String,
    timeout_days: u32,
) -> Result<u32> {
    // ...
}
```

### Example: Escrow + Reputation System

```rust
// Track successful escrows per user
// Build reputation score
pub struct UserReputation {
    successful_escrows: u32,
    disputed_escrows: u32,
    average_rating: u8,
}
```

---

## 📚 Next Steps

After mastering this contract:
1. **Add ERC20 support** - Escrow any token, not just ETR
2. **Build marketplace** - Integrate escrow into e-commerce platform
3. **Add insurance** - Optional insurance pool for disputes
4. **Deploy to Ember testnet** - Launch your escrow service!

---

## 📖 Resources

- Escrow concepts: https://en.wikipedia.org/wiki/Escrow
- Smart contract security: https://consensys.github.io/smart-contract-best-practices/
- ink! security: https://use.ink/basics/contract-security
- Substrate contracts: https://docs.substrate.io/tutorials/smart-contracts/

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy Escrow Building! 🔒💰**
