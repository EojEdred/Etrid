# Pallet Token Messenger

A generic, reusable CCTP-style (Cross-Chain Transfer Protocol) token messenger pallet for burn-and-mint bridging across all Partition Burst Chains (PBCs) and external blockchains in the Ëtrid multichain ecosystem.

## Overview

This pallet provides a production-ready implementation for cross-chain token transfers using a burn-and-mint architecture. Unlike chain-specific implementations (e.g., `pallet-edsc-bridge-token-messenger`), this pallet is fully generic and can be configured for any token on any PBC.

### Key Features

- **Generic Token Support**: Not limited to any specific token (EDSC, ETH, BTC, etc.)
- **Burn-and-Mint Architecture**: Tokens are burned on source chain and minted on destination
- **Replay Protection**: Nonce-based message ordering prevents replay attacks
- **Domain Registry**: Support for both PBCs and external chains (Ethereum, Solana, etc.)
- **Rate Limiting**: Per-transaction and daily limits for safety
- **Emergency Controls**: Governance-controlled pause functionality
- **Attestation Security**: Integration with `pallet-bridge-attestation` for M-of-N signature verification
- **CCTP Compatible**: Message format compatible with Circle's CCTP

## Architecture

### Message Flow

#### Outbound Transfer (Source → Destination)

```text
┌─────────────────────────────────────────────────────────────────┐
│ Source Chain (e.g., EDSC PBC)                                   │
│                                                                   │
│  User Account                                                    │
│       │                                                          │
│       │ deposit_for_burn(amount, dest_domain, recipient)        │
│       ▼                                                          │
│  ┌─────────────────────────────────────┐                        │
│  │ pallet-token-messenger              │                        │
│  │                                     │                        │
│  │ 1. Validate domain & limits         │                        │
│  │ 2. Burn tokens from sender          │                        │
│  │ 3. Create CrossChainMessage         │                        │
│  │ 4. Increment nonce                  │                        │
│  │ 5. Store outbound message           │                        │
│  │ 6. Emit DepositForBurn event        │                        │
│  └─────────────────────────────────────┘                        │
│       │                                                          │
└───────┼──────────────────────────────────────────────────────────┘
        │
        │ Off-chain: Attesters sign message_hash
        ▼
  ┌──────────────────────────┐
  │ Relayer Service          │
  │                          │
  │ - Collects attestations  │
  │ - Delivers to destination│
  └──────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ Destination Chain (e.g., Ethereum)                              │
│                                                                   │
│  ┌─────────────────────────────────────┐                        │
│  │ pallet-token-messenger              │                        │
│  │                                     │                        │
│  │ 1. Verify attestation signatures    │                        │
│  │ 2. Check nonce not used             │                        │
│  │ 3. Validate destination             │                        │
│  │ 4. Mark nonce as used               │                        │
│  │ 5. Mint tokens to recipient         │                        │
│  │ 6. Emit MessageReceived event       │                        │
│  └─────────────────────────────────────┘                        │
│       │                                                          │
│       ▼                                                          │
│  Recipient Account (receives minted tokens)                     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Message Format

Messages follow a standardized format compatible with Circle's CCTP:

```rust
CrossChainMessage {
    version: u32,                      // Message format version (1)
    source_domain: u32,                // Source blockchain ID
    destination_domain: u32,           // Destination blockchain ID
    nonce: u64,                        // Unique message nonce
    sender: BoundedVec<u8, 64>,       // Sender address on source
    recipient: BoundedVec<u8, 64>,    // Recipient address on destination
    message_body: BurnMessage {
        version: u32,
        burn_token: BoundedVec<u8, 64>,      // Token identifier
        mint_recipient: BoundedVec<u8, 64>,  // Mint recipient
        amount: u128,                         // Amount (with decimals)
        memo: BoundedVec<u8, 128>,           // Optional metadata
    }
}
```

### Domain IDs

Domains uniquely identify blockchains in the ecosystem:

| Domain ID | Blockchain | Type |
|-----------|------------|------|
| 0 | Ethereum Mainnet | External |
| 1 | Solana Mainnet | External |
| 2 | Ëtrid FlareChain | Relay Chain |
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
| 255 | Custom | Custom |

## Integration Guide

### 1. Add Dependency

Add to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-token-messenger = { path = "../../05-multichain/pallets-shared/pallet-token-messenger", default-features = false }

[features]
std = [
    "pallet-token-messenger/std",
]
```

### 2. Implement Token Operations

Implement the `TokenOperations` trait for your token:

```rust
use pallet_token_messenger::TokenOperations;

pub struct EdscTokenOperations;

impl TokenOperations<AccountId> for EdscTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Burn from native balance or asset pallet
        Balances::burn_from(account, amount.try_into().unwrap())?;
        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Mint to native balance or asset pallet
        Balances::mint_into(account, amount.try_into().unwrap())?;
        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        Balances::free_balance(account).into()
    }
}
```

### 3. Implement Attestation Verifier

Integrate with `pallet-bridge-attestation`:

```rust
use pallet_token_messenger::AttestationVerifier;

pub struct BridgeAttestationVerifier;

impl AttestationVerifier for BridgeAttestationVerifier {
    fn verify_message_attestation(
        message: &[u8],
        attestation: &[u8],
        message_hash: H256,
    ) -> DispatchResult {
        // Delegate to pallet-bridge-attestation
        pallet_bridge_attestation::Pallet::<Runtime>::verify_attestation(
            message_hash,
            attestation,
        )
    }
}
```

### 4. Configure Runtime

Add to your runtime:

```rust
parameter_types! {
    pub const MaxMessageBodySize: u32 = 512;
    pub const MaxBurnAmount: u128 = 1_000_000 * UNITS; // 1M tokens
    pub const DailyBurnCap: u128 = 10_000_000 * UNITS; // 10M tokens
    pub const MinBurnAmount: u128 = 1_000_000_000_000_000; // 0.001 tokens
    pub const MessageTimeout: BlockNumber = 14400; // 1 day
    pub const BlocksPerDay: BlockNumber = 14400; // 6s blocks
    pub const LocalDomain: u32 = 100; // Your PBC's domain ID
}

impl pallet_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = EdscTokenOperations;
    type AttestationVerifier = BridgeAttestationVerifier;
    type WeightInfo = pallet_token_messenger::weights::SubstrateWeight<Runtime>;
    type MaxMessageBodySize = MaxMessageBodySize;
    type MaxBurnAmount = MaxBurnAmount;
    type DailyBurnCap = DailyBurnCap;
    type MinBurnAmount = MinBurnAmount;
    type MessageTimeout = MessageTimeout;
    type BlocksPerDay = BlocksPerDay;
    type LocalDomain = LocalDomain;
}

construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        TokenMessenger: pallet_token_messenger,
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

### 5. Configure Domains (Governance)

After deployment, configure supported domains:

```rust
// Configure Ethereum as destination
TokenMessenger::configure_domain(
    Origin::root(),
    0, // Ethereum domain ID
    true, // enabled
    1_000_000 * UNITS, // max per transaction
    10_000_000 * UNITS, // daily limit
    1_000_000_000_000_000, // minimum (0.001 tokens)
)?;

// Configure another PBC
TokenMessenger::configure_domain(
    Origin::root(),
    101, // ETH-PBC domain ID
    true,
    5_000_000 * UNITS,
    50_000_000 * UNITS,
    1_000_000_000_000_000,
)?;
```

## Usage Examples

### User: Burn Tokens for Cross-Chain Transfer

```rust
// User wants to send 100 tokens from EDSC PBC to Ethereum
let amount = 100 * UNITS; // 100 tokens with 18 decimals
let destination_domain = 0; // Ethereum
let recipient = eth_address_bytes; // 20-byte Ethereum address

TokenMessenger::deposit_for_burn(
    Origin::signed(user_account),
    amount,
    destination_domain,
    recipient,
)?;

// Event emitted: DepositForBurn
// - Tokens are burned from user's account
// - Message stored with unique nonce
// - Attesters sign off-chain
// - Relayer delivers to Ethereum
```

### Relayer: Deliver Message to Destination

```rust
// Relayer monitors source chain events
// Collects attestations from M-of-N attesters
// Delivers to destination chain

let message = encoded_cross_chain_message;
let attestation = collected_signatures;

TokenMessenger::receive_message(
    Origin::signed(relayer_account),
    message,
    attestation,
)?;

// Event emitted: MessageReceived
// - Attestation verified
// - Nonce marked as used
// - Tokens minted to recipient
```

### Governance: Emergency Pause

```rust
// In case of security issue, governance can pause all operations
TokenMessenger::pause_bridge(Origin::root())?;

// All deposit_for_burn and receive_message calls will fail

// Resume when safe
TokenMessenger::unpause_bridge(Origin::root())?;
```

## Safety Features

### 1. Rate Limiting

- **Per-Transaction Limit**: Configured per domain (e.g., 1M tokens max)
- **Daily Limit**: Rolling 24-hour window (e.g., 10M tokens per day)
- **Minimum Amount**: Prevents dust attacks (e.g., 0.001 tokens min)

### 2. Replay Protection

- Each message has a unique `(source_domain, nonce)` pair
- Used nonces tracked in `UsedNonces` storage
- Attempting to process same message twice fails with `MessageAlreadyProcessed`

### 3. Attestation Security

- Messages must be signed by M-of-N configured attesters
- Integration with `pallet-bridge-attestation` for verification
- Invalid attestations fail with `AttestationFailed`

### 4. Emergency Controls

- Governance can pause all bridge operations
- Useful for security incidents or upgrades
- Resume when issue resolved

### 5. Domain Validation

- Only configured and enabled domains can be used
- Source cannot equal destination (prevents self-transfers)
- Invalid domains fail immediately

## Events

### DepositForBurn
Emitted when tokens are burned for cross-chain transfer.
```rust
DepositForBurn {
    nonce: u64,
    sender: AccountId,
    destination_domain: u32,
    recipient: Vec<u8>,
    amount: u128,
}
```

### MessageReceived
Emitted when tokens are minted from incoming message.
```rust
MessageReceived {
    source_domain: u32,
    nonce: u64,
    recipient: AccountId,
    amount: u128,
}
```

### MessageSent
Emitted for message indexing.
```rust
MessageSent {
    message_hash: H256,
    nonce: u64,
}
```

### DomainConfigured
Emitted when domain settings updated.
```rust
DomainConfigured {
    domain: u32,
    enabled: bool,
}
```

### BridgePaused / BridgeUnpaused
Emitted when bridge pause state changes.

### DailyLimitExceeded
Informational event when daily limit hit.
```rust
DailyLimitExceeded {
    domain: u32,
    attempted: u128,
    current_volume: u128,
    limit: u128,
}
```

## Errors

| Error | Description |
|-------|-------------|
| `BridgePaused` | Bridge is in emergency pause |
| `InvalidDomain` | Domain ID not configured |
| `DomainNotEnabled` | Domain exists but is disabled |
| `DomainNotSupported` | Source domain not in supported list |
| `AmountExceedsMax` | Amount exceeds per-tx limit |
| `AmountBelowMin` | Amount below minimum |
| `DailyLimitExceeded` | Daily limit would be exceeded |
| `InsufficientBalance` | Sender doesn't have enough tokens |
| `MessageAlreadyProcessed` | Replay attack detected |
| `InvalidMessageFormat` | Cannot decode message |
| `InvalidRecipient` | Recipient address invalid |
| `AttestationFailed` | Attestation verification failed |
| `BurnFailed` | Token burn operation failed |
| `MintFailed` | Token mint operation failed |
| `InvalidDestination` | Message not for this chain |
| `SourceEqualsDestination` | Cannot transfer to same chain |

## Testing

Run tests:
```bash
cargo test -p pallet-token-messenger
```

Run benchmarks:
```bash
cargo bench -p pallet-token-messenger
```

Generate weights:
```bash
./target/release/etrid-node benchmark pallet \
    --chain=dev \
    --pallet=pallet_token_messenger \
    --extrinsic='*' \
    --steps=50 \
    --repeat=20 \
    --output=./pallets-shared/pallet-token-messenger/src/weights.rs
```

## Differences from EDSC-Specific Implementation

| Feature | EDSC Bridge | Generic Token Messenger |
|---------|-------------|------------------------|
| Token | EDSC only | Any token (configurable) |
| Scope | EDSC PBC | All PBCs + external chains |
| Domain | Hardcoded `Etrid` | Configurable `LocalDomain` |
| Integration | Specific to EDSC | Generic `TokenOperations` trait |
| Reusability | Limited | High (any PBC can use) |
| Message Format | EDSC-specific | Generic with token field |
| Statistics | Basic | Enhanced (volume tracking) |
| Documentation | Moderate | Comprehensive |

## Production Checklist

Before deploying to mainnet:

- [ ] Configure all supported domains with appropriate limits
- [ ] Set up attesters in `pallet-bridge-attestation`
- [ ] Deploy and test relayer infrastructure
- [ ] Set appropriate rate limits based on token supply
- [ ] Configure emergency pause signatories
- [ ] Monitor events and daily volumes
- [ ] Set up alerting for anomalous behavior
- [ ] Document recovery procedures
- [ ] Test emergency pause/unpause
- [ ] Audit smart contracts on external chains

## License

Licensed under MIT OR Apache-2.0

## Maintainers

Ëtrid Team - [https://github.com/etrid/etrid](https://github.com/etrid/etrid)
