# Token Messenger Integration Guide

This guide shows how to integrate `pallet-token-messenger` with the `PbcTokenOperations` implementation in your PBC runtime.

## Overview

The `pallet-token-messenger` requires a `TokenOperations` implementation to perform burn and mint operations on the native token. We provide two ready-to-use implementations:

1. **`PbcTokenOperations<T>`** - Simplified implementation that works with any runtime that has `pallet_balances`
2. **`BalancesTokenOps<T>`** - Full-featured implementation with custom configuration

## Quick Start: Using PbcTokenOperations

The easiest way to integrate is using `PbcTokenOperations`, which automatically works with your existing `pallet_balances` configuration.

### Step 1: Add Dependencies

In your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-token-messenger = { path = "../../../pallets-shared/pallet-token-messenger", default-features = false }

[features]
std = [
    # ... other pallets
    "pallet-token-messenger/std",
]
```

### Step 2: Configure Runtime

In your runtime's `src/lib.rs`:

```rust
use pallet_token_messenger::{self as token_messenger};

// Define your local domain ID (must be unique per PBC)
parameter_types! {
    // Domain IDs:
    // 100 = EDSC PBC
    // 101 = ETH PBC
    // 102 = SOL PBC
    // 103 = BTC PBC
    // etc.
    pub const LocalDomain: u32 = 101; // Example: ETH PBC
}

impl token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    // Use the ready-made PbcTokenOperations implementation
    type TokenOperations = token_messenger::PbcTokenOperations<Runtime>;

    // Use pallet-bridge-attestation for message verification
    type AttestationVerifier = pallet_bridge_attestation::Pallet<Runtime>;

    // Weight configuration
    type WeightInfo = token_messenger::weights::SubstrateWeight<Runtime>;

    // Message configuration
    type MaxMessageBodySize = ConstU32<512>;

    // Per-transaction limits (default, can be overridden per domain)
    type MaxBurnAmount = ConstU128<{ 1_000_000 * UNITS }>; // 1M tokens
    type MinBurnAmount = ConstU128<{ UNITS / 100 }>; // 0.01 tokens

    // Daily limit across all domains (default)
    type DailyBurnCap = ConstU128<{ 10_000_000 * UNITS }>; // 10M tokens per day

    // Message timeout for cleanup (in blocks)
    type MessageTimeout = ConstU32<14400>; // ~24 hours at 6s/block

    // Blocks per day for daily limit reset
    type BlocksPerDay = ConstU32<14400>; // 14400 blocks * 6s = 24h

    // Local domain identifier
    type LocalDomain = LocalDomain;
}
```

### Step 3: Add to construct_runtime!

```rust
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... existing pallets
        Balances: pallet_balances,

        // Add TokenMessenger pallet
        TokenMessenger: token_messenger,

        // Required: Bridge attestation pallet
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

### Step 4: Configure Domains (Governance)

After runtime is deployed, configure which domains are supported:

```rust
// Example: Enable transfers to EDSC PBC (domain 100)
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: 100, // EDSC PBC
    enabled: true,
    max_burn_amount: 500_000 * UNITS, // 500K tokens per tx
    daily_burn_limit: 5_000_000 * UNITS, // 5M tokens per day
    min_burn_amount: UNITS / 100, // 0.01 tokens minimum
)?;

// Example: Enable transfers to Ethereum (domain 0)
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: 0, // Ethereum
    enabled: true,
    max_burn_amount: 100_000 * UNITS, // 100K tokens per tx
    daily_burn_limit: 1_000_000 * UNITS, // 1M tokens per day
    min_burn_amount: UNITS / 10, // 0.1 tokens minimum
)?;
```

## How Token Operations Work

### Burn Operation (Outbound Transfer)

When a user calls `deposit_for_burn()`:

```rust
// User burns 100 tokens to send to EDSC PBC
TokenMessenger::deposit_for_burn(
    RuntimeOrigin::signed(sender),
    amount: 100 * UNITS, // 100 tokens
    destination_domain: 100, // EDSC PBC
    mint_recipient: recipient_account_bytes,
)?;
```

Internally, `PbcTokenOperations::burn_tokens()` is called:
1. Converts `u128` amount to `Balance` type
2. Calls `Currency::withdraw()` from `pallet_balances`
3. Removes tokens from sender's account (burns them)
4. Tokens are gone from this chain's circulation

### Mint Operation (Inbound Transfer)

When a relayer delivers a message with `receive_message()`:

```rust
// Relayer delivers cross-chain message
TokenMessenger::receive_message(
    RuntimeOrigin::signed(relayer),
    message: encoded_message,
    attestation: attester_signatures,
)?;
```

Internally, `PbcTokenOperations::mint_tokens()` is called:
1. Verifies attestation first (security)
2. Converts `u128` amount to `Balance` type
3. Calls `Currency::deposit_creating()` from `pallet_balances`
4. Creates new tokens in recipient's account
5. Increases total issuance on this chain

### Balance Check

Before burning, the pallet checks if user has sufficient balance:

```rust
let balance = TokenOperations::balance_of(&sender);
ensure!(balance >= amount, Error::<T>::InsufficientBalance);
```

## Complete Runtime Example

Here's a complete example for ETH PBC:

```rust
// eth-pbc-runtime/src/lib.rs

use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU128},
};
use pallet_token_messenger::{self as token_messenger};

// Token unit definitions
pub const UNITS: Balance = 1_000_000_000_000_000_000; // 10^18 (18 decimals)
pub const MILLIUNITS: Balance = 1_000_000_000_000_000; // 10^15
pub const MICROUNITS: Balance = 1_000_000_000_000; // 10^12

// ETH PBC domain ID
parameter_types! {
    pub const LocalDomain: u32 = 101; // ETH PBC
}

// Token Messenger Configuration
impl token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = token_messenger::PbcTokenOperations<Runtime>;
    type AttestationVerifier = pallet_bridge_attestation::Pallet<Runtime>;
    type WeightInfo = token_messenger::weights::SubstrateWeight<Runtime>;

    // Message limits
    type MaxMessageBodySize = ConstU32<512>;

    // Default burn limits (can be overridden per domain)
    type MaxBurnAmount = ConstU128<{ 1_000_000 * UNITS }>; // 1M tokens max per tx
    type DailyBurnCap = ConstU128<{ 10_000_000 * UNITS }>; // 10M tokens max per day
    type MinBurnAmount = ConstU128<{ UNITS / 100 }>; // 0.01 tokens minimum

    // Timing configuration
    type MessageTimeout = ConstU32<14400>; // 24 hours
    type BlocksPerDay = ConstU32<14400>; // 6s per block

    // Domain identifier
    type LocalDomain = LocalDomain;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,

        // Bridge pallets
        TokenMessenger: token_messenger,
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

## Advanced: Custom TokenOperations Implementation

If you need custom behavior (e.g., burning fees, custom token types), you can implement `TokenOperations` directly:

```rust
use pallet_token_messenger::TokenOperations;
use frame_support::traits::Currency;

pub struct CustomTokenOps<T>(PhantomData<T>);

impl<T: Config> TokenOperations<T::AccountId> for CustomTokenOps<T>
where
    T: pallet_balances::Config,
{
    fn burn_tokens(account: &T::AccountId, amount: u128) -> DispatchResult {
        // Custom burn logic
        let balance_amount: <T as pallet_balances::Config>::Balance =
            amount.saturated_into();

        // Example: Burn 99% and send 1% to treasury
        let burn_amount = balance_amount.saturating_mul(99) / 100;
        let fee_amount = balance_amount.saturating_sub(burn_amount);

        // Withdraw burn amount
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::withdraw(
            account,
            burn_amount,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )?;

        // Transfer fee to treasury
        let treasury = T::TreasuryAccount::get();
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::transfer(
            account,
            &treasury,
            fee_amount,
            ExistenceRequirement::KeepAlive,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &T::AccountId, amount: u128) -> DispatchResult {
        // Standard mint
        let balance_amount: <T as pallet_balances::Config>::Balance =
            amount.saturated_into();

        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::deposit_creating(
            account,
            balance_amount,
        );

        Ok(())
    }

    fn balance_of(account: &T::AccountId) -> u128 {
        let balance = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(account);
        balance.saturated_into()
    }
}
```

## Testing

The `token_ops` module includes comprehensive tests. To run them:

```bash
cd 05-multichain/pallets-shared/pallet-token-messenger
cargo test token_ops
```

## Domain ID Reference

| Domain ID | Chain | Type |
|-----------|-------|------|
| 0 | Ethereum | External |
| 1 | Solana | External |
| 2 | Primearc Core | Relay Chain |
| 3 | Polygon | External |
| 4 | BNB Chain | External |
| 5 | Avalanche | External |
| 6 | Arbitrum | External |
| 7 | Optimism | External |
| 8 | Base | External |
| 100 | EDSC PBC | PBC |
| 101 | ETH PBC | PBC |
| 102 | SOL PBC | PBC |
| 103 | BTC PBC | PBC |
| 104 | USDC PBC | PBC |
| 105 | USDT PBC | PBC |
| 106 | DAI PBC | PBC |

## Security Considerations

1. **Rate Limits**: Always configure appropriate rate limits per domain
2. **Attestation**: Ensure `pallet-bridge-attestation` is properly configured with trusted attesters
3. **Emergency Pause**: Governance should be prepared to pause bridge if issues detected
4. **Balance Checks**: TokenOperations automatically checks balance before burning
5. **Nonce Protection**: Built-in replay protection via nonce tracking

## Common Issues

### Issue: "type TokenOperations = ();"

**Problem**: Runtime still using placeholder implementation

**Solution**: Update runtime config to use `PbcTokenOperations`:
```rust
type TokenOperations = token_messenger::PbcTokenOperations<Runtime>;
```

### Issue: Burn fails with "InsufficientBalance"

**Problem**: User doesn't have enough tokens

**Solution**: Check balance before calling `deposit_for_burn()`:
```rust
let balance = TokenMessenger::balance_of(&account);
ensure!(balance >= amount, "Insufficient balance");
```

### Issue: "DomainNotEnabled" error

**Problem**: Target domain not configured or disabled

**Solution**: Configure domain via governance:
```rust
TokenMessenger::configure_domain(
    RuntimeOrigin::root(),
    domain: target_domain,
    enabled: true,
    max_burn_amount: limit,
    daily_burn_limit: daily_limit,
    min_burn_amount: min,
)?;
```

## Next Steps

1. Deploy runtime with `TokenMessenger` pallet
2. Configure supported domains via governance
3. Set up attestation infrastructure (attesters)
4. Deploy relayer infrastructure to deliver messages
5. Test with small amounts first
6. Monitor bridge operations via events

## Resources

- [pallet-token-messenger source](./src/lib.rs)
- [Token operations implementation](./src/token_ops.rs)
- [Attestation integration](./src/attestation.rs)
- [Tests](./src/tests.rs)
