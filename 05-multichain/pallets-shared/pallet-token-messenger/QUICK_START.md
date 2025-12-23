# Token Messenger: Quick Start Guide

## 5-Minute Setup for PBC Runtimes

### Step 1: Add to Runtime Dependencies

**File**: `your-pbc-runtime/Cargo.toml`

```toml
[dependencies]
# ... other dependencies
pallet-token-messenger = { path = "../../../pallets-shared/pallet-token-messenger", default-features = false }
pallet-bridge-attestation = { path = "../../../pallets-shared/pallet-bridge-attestation", default-features = false }

[features]
std = [
    # ... other features
    "pallet-token-messenger/std",
    "pallet-bridge-attestation/std",
]
```

### Step 2: Configure Runtime

**File**: `your-pbc-runtime/src/lib.rs`

Add this configuration block:

```rust
use pallet_token_messenger::{self as token_messenger};

// Set your PBC's unique domain ID
parameter_types! {
    pub const LocalDomain: u32 = 101; // Change this! 100=EDSC, 101=ETH, 102=SOL, etc.
}

// Configure the token messenger
impl token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = token_messenger::PbcTokenOperations<Runtime>;
    type AttestationVerifier = pallet_bridge_attestation::Pallet<Runtime>;
    type WeightInfo = token_messenger::weights::SubstrateWeight<Runtime>;
    type MaxMessageBodySize = ConstU32<512>;
    type MaxBurnAmount = ConstU128<{ 1_000_000 * UNITS }>;
    type DailyBurnCap = ConstU128<{ 10_000_000 * UNITS }>;
    type MinBurnAmount = ConstU128<{ UNITS / 100 }>;
    type MessageTimeout = ConstU32<14400>;
    type BlocksPerDay = ConstU32<14400>;
    type LocalDomain = LocalDomain;
}
```

### Step 3: Add to construct_runtime!

**File**: `your-pbc-runtime/src/lib.rs`

```rust
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... existing pallets
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,

        // Add these two pallets:
        TokenMessenger: token_messenger,
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

### Step 4: Build Runtime

```bash
cd your-pbc-runtime
cargo build --release
```

### Step 5: Configure Domains (via Governance)

After deploying the runtime, configure which chains can send/receive:

```rust
// Enable transfers to/from EDSC PBC
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: 100, // EDSC
    enabled: true,
    max_burn_amount: 500_000 * UNITS,
    daily_burn_limit: 5_000_000 * UNITS,
    min_burn_amount: UNITS / 100,
)?;

// Enable transfers to/from Ethereum
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: 0, // Ethereum
    enabled: true,
    max_burn_amount: 100_000 * UNITS,
    daily_burn_limit: 1_000_000 * UNITS,
    min_burn_amount: UNITS / 10,
)?;
```

## That's It!

Your PBC now has full cross-chain token transfer capability:

- ✅ Users can burn tokens on your chain to send to other chains
- ✅ Relayers can deliver messages to mint tokens on your chain
- ✅ Automatic balance validation and security checks
- ✅ Rate limiting and emergency pause controls

## Usage Examples

### User Burns Tokens (Outbound)

```rust
// Send 100 tokens from ETH PBC to EDSC PBC
TokenMessenger::deposit_for_burn(
    RuntimeOrigin::signed(sender),
    amount: 100 * UNITS,
    destination_domain: 100, // EDSC PBC
    mint_recipient: recipient_account.encode(),
)?;
```

### Relayer Delivers Message (Inbound)

```rust
// Deliver cross-chain message with attestation
TokenMessenger::receive_message(
    RuntimeOrigin::signed(relayer),
    message: encoded_cross_chain_message,
    attestation: attester_signatures,
)?;
```

## Domain ID Reference

| ID | Chain | Notes |
|----|-------|-------|
| 0 | Ethereum | External chain |
| 1 | Solana | External chain |
| 2 | Primearc Core | Relay chain |
| 100 | EDSC PBC | Euro Digital Stable Coin |
| 101 | ETH PBC | Ethereum bridge |
| 102 | SOL PBC | Solana bridge |
| 103 | BTC PBC | Bitcoin bridge |
| 104 | USDC PBC | USDC bridge |
| 105 | USDT PBC | USDT bridge |
| 106 | DAI PBC | DAI bridge |

**Important**: Each PBC must have a unique domain ID!

## Configuration Parameters Explained

```rust
type MaxBurnAmount = ConstU128<{ 1_000_000 * UNITS }>;
// Maximum tokens per single transaction
// Example: 1M tokens max per burn

type DailyBurnCap = ConstU128<{ 10_000_000 * UNITS }>;
// Total tokens that can be burned per day across all domains
// Resets every 24 hours

type MinBurnAmount = ConstU128<{ UNITS / 100 }>;
// Minimum transfer amount (prevents dust attacks)
// Example: 0.01 tokens minimum

type MessageTimeout = ConstU32<14400>;
// How many blocks before old messages can be cleaned up
// 14400 blocks = 24 hours at 6s/block

type BlocksPerDay = ConstU32<14400>;
// Number of blocks in 24 hours (for daily limit reset)
// Adjust based on your block time
```

## What Gets Installed

When you configure `TokenMessenger`, you get:

1. **Token Operations**: Automatic burn/mint via `PbcTokenOperations`
   - Burns: Uses `pallet_balances::Currency::withdraw()`
   - Mints: Uses `pallet_balances::Currency::deposit_creating()`
   - Balance checks: Uses `pallet_balances::Currency::free_balance()`

2. **Security Features**:
   - ✅ Attestation verification (via `pallet-bridge-attestation`)
   - ✅ Replay protection (nonce tracking)
   - ✅ Rate limiting (per-tx and daily)
   - ✅ Emergency pause capability
   - ✅ Balance validation

3. **Storage**:
   - Outbound messages (for indexing)
   - Used nonces (replay protection)
   - Domain configs (limits and settings)
   - Daily volume tracking
   - Statistics (total sent/received)

4. **Extrinsics**:
   - `deposit_for_burn()` - Users burn tokens
   - `receive_message()` - Relayers deliver messages
   - `configure_domain()` - Governance configures chains
   - `pause_bridge()` - Emergency stop
   - `unpause_bridge()` - Resume operations

## Need Help?

- **Full integration guide**: See `INTEGRATION_GUIDE.md`
- **Implementation details**: See `TOKEN_OPERATIONS_IMPLEMENTATION.md`
- **Source code**: See `src/token_ops.rs`
- **Tests**: Run `cargo test token_ops`

## Common Mistakes

❌ **Forgot to set unique LocalDomain**
```rust
// BAD: All PBCs using same domain
pub const LocalDomain: u32 = 100;
```

✅ **Each PBC needs unique domain**
```rust
// GOOD: ETH PBC uses 101
pub const LocalDomain: u32 = 101;
```

---

❌ **Forgot to add BridgeAttestation pallet**
```rust
// BAD: Only added TokenMessenger
TokenMessenger: token_messenger,
```

✅ **Both pallets required**
```rust
// GOOD: Both pallets present
TokenMessenger: token_messenger,
BridgeAttestation: pallet_bridge_attestation,
```

---

❌ **Didn't configure any domains**
```rust
// BAD: No domains configured, bridge won't work
```

✅ **Configure at least one domain**
```rust
// GOOD: Enable transfers to EDSC
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: 100,
    enabled: true,
    // ... limits
)?;
```

## You're Done!

Your PBC now has production-ready cross-chain token transfers powered by `PbcTokenOperations`. The implementation automatically handles:

- Token burning (decrease issuance)
- Token minting (increase issuance)
- Balance validation
- Type conversions (u128 ↔ Balance)
- Account lifecycle (creation/deletion)

No additional code needed!
