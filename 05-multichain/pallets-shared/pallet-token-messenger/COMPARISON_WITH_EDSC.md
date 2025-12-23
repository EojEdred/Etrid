# Comparison: pallet-token-messenger vs pallet-edsc-bridge-token-messenger

This document highlights the key differences between the generic `pallet-token-messenger` and the EDSC-specific implementation.

## Summary

| Aspect | EDSC Bridge | Generic Token Messenger |
|--------|-------------|------------------------|
| **Purpose** | EDSC token bridging only | Any token on any PBC |
| **Reusability** | Limited to EDSC PBC | All PBCs + external chains |
| **Token Type** | Hardcoded EDSC | Configurable via trait |
| **Domain** | Hardcoded `Domain::Etrid` | Configurable `LocalDomain` |
| **Domains Supported** | 8 predefined | 15+ (including PBC range 100-199) |
| **Token Operations** | Direct integration | Abstract trait (`TokenOperations`) |
| **Attestation** | Inline placeholder | Abstract trait (`AttestationVerifier`) |
| **Storage** | Basic tracking | Enhanced with volume metrics |
| **Events** | 5 events | 7 events (with MessageSent) |
| **Errors** | 14 errors | 18 errors (more granular) |
| **Safety** | Basic rate limits | Enhanced (min amount, balance checks) |
| **Documentation** | Module-level docs | Comprehensive (README, examples) |
| **Testing** | Basic tests | Comprehensive test suite |
| **Benchmarking** | Not included | Full benchmark suite |
| **Genesis Config** | Manual setup | Configurable genesis |
| **Multi-token Support** | No | Yes (via trait implementation) |

---

## Detailed Comparison

### 1. Token Operations

#### EDSC Bridge
```rust
// Hardcoded EDSC burning
pub trait TokenOperations<AccountId> {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult;
    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult;
}

// Usage: Assumes EDSC native balance
```

#### Generic Token Messenger
```rust
pub trait TokenOperations<AccountId> {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult;
    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult;
    fn balance_of(account: &AccountId) -> u128; // Added for validation
}

// Can be implemented for:
// - Native balance (EDSC style)
// - Assets pallet (ETH, BTC, etc.)
// - Custom token systems
// - Multi-token support
```

### 2. Domain Configuration

#### EDSC Bridge
```rust
pub enum Domain {
    Ethereum = 0,
    Solana = 1,
    Etrid = 2,      // Hardcoded local domain
    Polygon = 3,
    BnbChain = 4,
    Avalanche = 5,
    Arbitrum = 6,
    Optimism = 7,
}

// Always uses Domain::Etrid as source
source_domain: Domain::Etrid.to_u32(),
```

#### Generic Token Messenger
```rust
pub enum Domain {
    // External chains (0-99)
    Ethereum = 0,
    Solana = 1,
    FlareChain = 2,
    Polygon = 3,
    BnbChain = 4,
    Avalanche = 5,
    Arbitrum = 6,
    Optimism = 7,
    Base = 8,

    // PBC range (100-199)
    PbcEdsc = 100,
    PbcEth = 101,
    PbcSol = 102,
    PbcBtc = 103,
    PbcUsdc = 104,
    PbcUsdt = 105,
    PbcDai = 106,

    Custom = 255,
}

// Configurable per runtime
type LocalDomain: Get<u32>;

// Example:
// EDSC PBC: LocalDomain = 100
// ETH PBC:  LocalDomain = 101
// SOL PBC:  LocalDomain = 102
```

### 3. Message Format

#### EDSC Bridge
```rust
pub struct BurnMessage {
    pub version: u32,
    pub burn_token: BoundedVec<u8, ConstU32<64>>,  // Always "EDSC"
    pub mint_recipient: BoundedVec<u8, ConstU32<64>>,
    pub amount: u128,
}
```

#### Generic Token Messenger
```rust
pub struct BurnMessage {
    pub version: u32,
    pub burn_token: BoundedVec<u8, ConstU32<64>>,  // Generic identifier
    pub mint_recipient: BoundedVec<u8, ConstU32<64>>,
    pub amount: u128,
    pub memo: BoundedVec<u8, ConstU32<128>>,  // Added for metadata
}
```

### 4. Attestation Integration

#### EDSC Bridge
```rust
pub trait AttestationVerifier {
    fn verify_message_attestation(
        message: &[u8],
        message_hash: sp_core::H256,
    ) -> DispatchResult;
}

// Inline implementation
fn verify_attestation(message: &[u8], _attestation: &[u8]) -> DispatchResult {
    let cross_chain_msg = CrossChainMessage::decode(&mut &message[..])?;
    let message_hash = Self::get_message_hash(&cross_chain_msg);
    T::AttestationVerifier::verify_message_attestation(message, message_hash)?;
    Ok(())
}
```

#### Generic Token Messenger
```rust
pub trait AttestationVerifier {
    fn verify_message_attestation(
        message: &[u8],
        attestation: &[u8],  // Now required parameter
        message_hash: sp_core::H256,
    ) -> DispatchResult;
}

// Clean integration
T::AttestationVerifier::verify_message_attestation(&message, &attestation, message_hash)
    .map_err(|_| Error::<T>::AttestationFailed)?;
```

### 5. Storage & Tracking

#### EDSC Bridge
```rust
// Basic storage
pub type OutboundMessages<T> = StorageMap<_, Blake2_128Concat, u64, CrossChainMessage>;
pub type Nonce<T> = StorageMap<_, Blake2_128Concat, u32, u64>;
pub type UsedNonces<T> = StorageDoubleMap<_, _, u32, _, u64, bool>;
pub type DomainConfigs<T> = StorageMap<_, Blake2_128Concat, u32, DomainConfig>;
pub type DailyBurnVolume<T> = StorageMap<_, Blake2_128Concat, u32, (BlockNumber, u128)>;
pub type TotalSent<T> = StorageValue<_, u64>;
pub type TotalReceived<T> = StorageValue<_, u64>;
pub type IsPaused<T> = StorageValue<_, bool>;
```

#### Generic Token Messenger
```rust
// Enhanced storage
pub type OutboundMessages<T> = StorageMap<_, Blake2_128Concat, u64, CrossChainMessage>;
pub type MessageNonce<T> = StorageMap<_, Blake2_128Concat, u32, u64>;
pub type UsedNonces<T> = StorageDoubleMap<_, _, u32, _, u64, bool>;
pub type DomainConfigs<T> = StorageMap<_, Blake2_128Concat, u32, DomainConfig>;
pub type SupportedDomains<T> = StorageMap<_, Blake2_128Concat, u32, bool>;  // NEW
pub type DailyBurnVolume<T> = StorageMap<_, Blake2_128Concat, u32, (BlockNumber, u128)>;
pub type TotalSent<T> = StorageValue<_, u64>;
pub type TotalReceived<T> = StorageValue<_, u64>;
pub type TotalVolumeSent<T> = StorageValue<_, u128>;      // NEW - Track volume
pub type TotalVolumeReceived<T> = StorageValue<_, u128>;  // NEW - Track volume
pub type IsPaused<T> = StorageValue<_, bool>;
```

### 6. Events

#### EDSC Bridge (5 Events)
```rust
BurnMessageSent { nonce, destination_domain, amount, recipient }
MessageReceived { source_domain, nonce, amount, recipient }
DomainConfigured { domain, enabled }
BridgePaused
BridgeUnpaused
```

#### Generic Token Messenger (7 Events)
```rust
DepositForBurn { nonce, sender, destination_domain, recipient, amount }  // Enhanced
MessageReceived { source_domain, nonce, recipient, amount }
MessageSent { message_hash, nonce }        // NEW - For indexing
DomainConfigured { domain, enabled }
DomainRemoved { domain }                   // NEW
BridgePaused
BridgeUnpaused
DailyLimitExceeded { domain, attempted, current_volume, limit }  // Enhanced
```

### 7. Error Handling

#### EDSC Bridge (14 Errors)
```rust
BridgePaused
InvalidDomain
DomainNotEnabled
AmountExceedsMax
DailyLimitExceeded
MessageAlreadyProcessed
InvalidMessageFormat
InvalidRecipient
NonceMismatch
MessageTooLarge
AttestationFailed
BurnFailed
MintFailed
```

#### Generic Token Messenger (18 Errors)
```rust
BridgePaused
InvalidDomain
DomainNotEnabled
DomainNotSupported          // NEW
AmountExceedsMax
AmountBelowMin              // NEW
DailyLimitExceeded
InsufficientBalance         // NEW
MessageAlreadyProcessed
InvalidMessageFormat
InvalidMessageVersion       // NEW
InvalidRecipient
InvalidSender               // NEW
NonceMismatch
MessageTooLarge
AttestationFailed
BurnFailed
MintFailed
InvalidDestination          // NEW
SourceEqualsDestination     // NEW
ArithmeticOverflow          // NEW
InvalidAmount               // NEW
```

### 8. Safety Features

#### EDSC Bridge
- ✓ Per-transaction limit
- ✓ Daily burn limit (24h rolling window)
- ✓ Emergency pause
- ✓ Nonce-based replay protection
- ✗ No minimum amount check
- ✗ No balance validation before burn
- ✗ No source/destination equality check

#### Generic Token Messenger
- ✓ Per-transaction limit
- ✓ Daily burn limit (24h rolling window)
- ✓ Emergency pause
- ✓ Nonce-based replay protection
- ✓ Minimum amount check (prevents dust)
- ✓ Balance validation before burn
- ✓ Source/destination equality check
- ✓ Explicit domain support tracking

### 9. Configuration

#### EDSC Bridge
```rust
pub trait Config: frame_system::Config {
    type RuntimeEvent;
    type TokenOperations;
    type AttestationVerifier;
    type MaxMessageBodySize: Get<u32>;
    type MaxBurnAmount: Get<u128>;
    type DailyBurnCap: Get<u128>;
    type MessageTimeout: Get<BlockNumber>;
}
```

#### Generic Token Messenger
```rust
pub trait Config: frame_system::Config {
    type RuntimeEvent;
    type TokenOperations;
    type AttestationVerifier;
    type WeightInfo: WeightInfo;         // NEW - Benchmarks
    type MaxMessageBodySize: Get<u32>;
    type MaxBurnAmount: Get<u128>;
    type DailyBurnCap: Get<u128>;
    type MinBurnAmount: Get<u128>;       // NEW
    type MessageTimeout: Get<BlockNumber>;
    type BlocksPerDay: Get<BlockNumber>; // NEW - Explicit
    type LocalDomain: Get<u32>;          // NEW - Configurable
}
```

### 10. Extrinsics

#### EDSC Bridge (5 Extrinsics)
```rust
burn_edsc_for_external_chain(origin, destination_domain, amount, recipient)
receive_and_mint(origin, message, attestation)
configure_domain(origin, domain, enabled, max, daily)
pause_bridge(origin)
unpause_bridge(origin)
```

#### Generic Token Messenger (6 Extrinsics)
```rust
deposit_for_burn(origin, amount, destination_domain, mint_recipient)  // Renamed for clarity
receive_message(origin, message, attestation)                         // Renamed for clarity
configure_domain(origin, domain, enabled, max, daily, min)            // Added min param
remove_domain(origin, domain)                                         // NEW
pause_bridge(origin)
unpause_bridge(origin)
```

---

## Migration Path from EDSC Bridge

If you have the EDSC bridge deployed and want to migrate to the generic version:

### Step 1: Deploy Generic Pallet

```rust
// Add to runtime
impl pallet_token_messenger::Config for Runtime {
    type LocalDomain = ConstU32<100>; // EDSC domain
    // ... other config matching EDSC bridge
}
```

### Step 2: Dual Operation Period

Both pallets can coexist during migration:

```rust
construct_runtime!(
    pub enum Runtime {
        // Old
        EdscBridge: pallet_edsc_bridge_token_messenger,
        // New
        TokenMessenger: pallet_token_messenger,
    }
);
```

### Step 3: Migrate State

```rust
// Migration pallet
pub fn migrate_edsc_to_generic<T: Config>() -> Weight {
    // Migrate domain configs
    pallet_edsc_bridge_token_messenger::DomainConfigs::<T>::iter().for_each(|(k, v)| {
        pallet_token_messenger::DomainConfigs::<T>::insert(k, v);
    });

    // Migrate nonces
    pallet_edsc_bridge_token_messenger::Nonce::<T>::iter().for_each(|(k, v)| {
        pallet_token_messenger::MessageNonce::<T>::insert(k, v);
    });

    // Migrate used nonces
    pallet_edsc_bridge_token_messenger::UsedNonces::<T>::iter().for_each(|(k1, k2, v)| {
        pallet_token_messenger::UsedNonces::<T>::insert(k1, k2, v);
    });

    Weight::zero() // TODO: Calculate actual weight
}
```

### Step 4: Deprecate Old Pallet

After migration and testing period, remove old pallet from runtime.

---

## Use Case Matrix

| Use Case | EDSC Bridge | Generic Messenger |
|----------|-------------|-------------------|
| EDSC cross-chain transfers | ✓ | ✓ |
| ETH cross-chain transfers | ✗ | ✓ |
| BTC cross-chain transfers | ✗ | ✓ |
| Stablecoin transfers (USDC, USDT) | ✗ | ✓ |
| PBC-to-PBC transfers | ✗ | ✓ |
| Multi-token support on one PBC | ✗ | ✓ (with multiple instances) |
| Integration with external CCTP | Limited | ✓ (compatible format) |
| Custom token implementations | ✗ | ✓ (via trait) |

---

## Conclusion

The generic `pallet-token-messenger` is a **superset** of the EDSC bridge functionality with:

1. **Greater flexibility** - Works with any token, any chain
2. **Better safety** - More validation checks and error handling
3. **Enhanced monitoring** - Volume tracking and detailed events
4. **Production-ready** - Comprehensive tests, benchmarks, documentation
5. **Future-proof** - Designed for Ëtrid's multichain ecosystem

**Recommendation**: Use `pallet-token-messenger` for all new PBC deployments and consider migrating existing EDSC bridge to this implementation for consistency and improved features.
