# Generic Bridge Attestation Pallet

A **generic, reusable** Substrate pallet providing M-of-N threshold signature verification for cross-chain bridge messages. This pallet can be used by **any PBC (Partition Burst Chain)** in the Etrid ecosystem for secure cross-chain attestation.

## Overview

This pallet implements a CCTP (Cross-Chain Transfer Protocol) style attestation system with:
- **M-of-N Threshold Signatures**: Requires M valid signatures from N registered attesters
- **Multi-Signature Support**: Both ECDSA (Ethereum-compatible) and SR25519 (Substrate native)
- **Byzantine Fault Tolerance**: Continues operating even if some attesters fail
- **Governance Controlled**: Attesters managed via root/governance
- **Replay Protection**: Nonce-based system prevents replay attacks
- **Chain-Agnostic**: Configurable ChainId for any source/destination chain

## Key Features

### 1. Attester Registry
- Register/remove attesters with unique public keys
- Enable/disable attesters without permanent removal
- Track attester statistics (messages signed, last activity)
- Support for both ECDSA and SR25519 public keys

### 2. Signature Verification
- **ECDSA Verification**: For EVM-compatible bridges (Ethereum, BSC, etc.)
- **SR25519 Verification**: For Substrate-native bridges (Polkadot, Kusama, etc.)
- Automatic signature type detection based on key/signature length
- Cryptographic verification using sp_io primitives

### 3. Threshold Management
- Global threshold configuration (applies to all domains)
- Per-domain threshold configuration for fine-grained control
- Dynamic threshold updates via governance
- Automatic validation of M ≤ N

### 4. Security Features
- **Signature Deduplication**: Prevents same attester signing twice
- **Nonce-Based Replay Protection**: Each message has unique nonce
- **Attestation Expiry**: Configurable max age for attestations
- **Emergency Pause**: Governance can pause all attestation operations
- **Active Attester Tracking**: Only active attesters can sign

## Usage

### Runtime Configuration

Add to your PBC runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-bridge-attestation = { path = "../../pallets-shared/pallet-bridge-attestation", default-features = false }

[features]
std = [
    "pallet-bridge-attestation/std",
    # ... other pallets
]
```

### Runtime Implementation

```rust
use frame_support::traits::{ConstU32, ConstU64};

parameter_types! {
    // Your PBC's unique chain ID
    pub const YourChainId: u32 = 100;

    // Maximum attesters in registry
    pub const MaxAttesters: u32 = 100;

    // Maximum signatures per message
    pub const MaxAttestersPerMessage: u32 = 20;

    // Minimum signatures required (M in M-of-N)
    pub const MinSignatureThreshold: u32 = 2;

    // Attestation expiry (blocks)
    pub const AttestationMaxAge: u64 = 1000;
}

impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = YourChainId;
    type MaxAttesters = MaxAttesters;
    type MaxAttestersPerMessage = MaxAttestersPerMessage;
    type MinSignatureThreshold = MinSignatureThreshold;
    type AttestationMaxAge = AttestationMaxAge;
    type WeightInfo = ();
}

construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

## Extrinsics

### Governance Operations (require root)

#### `register_attester(public_key: Vec<u8>)`
Register a new attester with their public key.
- **public_key**: 32 bytes (SR25519) or 33 bytes (ECDSA compressed)

#### `disable_attester(attester_id: u32)`
Temporarily disable an attester (can be re-enabled).

#### `enable_attester(attester_id: u32)`
Re-enable a previously disabled attester.

#### `remove_attester(attester_id: u32)`
Permanently remove an attester from registry.

#### `configure_threshold(domain_id: Option<u32>, min_signatures: u32, total_attesters: u32)`
Configure M-of-N threshold for a domain (None = global).

#### `update_threshold(new_threshold: u32)`
Update the global signature threshold.

#### `pause_attestation()`
Emergency pause all attestation operations.

#### `unpause_attestation()`
Resume attestation operations after pause.

### Public Operations (permissionless)

#### `submit_signature(attester_id: u32, message_hash: H256, signature: Vec<u8>, source_chain_id: u32, destination_chain_id: u32, nonce: u64)`
Submit a signature for a cross-chain message.
- **signature**: 65 bytes (ECDSA) or 64 bytes (SR25519)

#### `verify_attestation(message: Vec<u8>, message_hash: H256)`
Verify that a message has sufficient valid signatures.

## Storage

| Name | Type | Description |
|------|------|-------------|
| `Attesters` | `Map<u32, AttesterInfo>` | Registered attesters by ID |
| `AttesterByPubkey` | `Map<Vec<u8>, u32>` | Public key → Attester ID lookup |
| `NextAttesterId` | `u32` | Next available attester ID |
| `ActiveAttesterCount` | `u32` | Count of active attesters |
| `Attestations` | `Map<H256, Attestation>` | Message attestations |
| `ThresholdConfigs` | `Map<u32, ThresholdConfig>` | Per-domain thresholds |
| `GlobalThreshold` | `ThresholdConfig` | Global threshold config |
| `IsPaused` | `bool` | Emergency pause flag |
| `TotalAttestations` | `u64` | Total verified attestations |
| `AttestationNonce` | `u64` | Current nonce counter |
| `UsedNonces` | `Map<u64, bool>` | Used nonces (replay protection) |

## Events

- `AttesterRegistered { attester_id, public_key }`
- `AttesterStatusChanged { attester_id, old_status, new_status }`
- `AttesterRemoved { attester_id }`
- `SignatureSubmitted { attester_id, message_hash, nonce }`
- `AttestationThresholdReached { message_hash, signature_count, nonce }`
- `AttestationVerified { message_hash, signature_count, source_chain_id, destination_chain_id }`
- `ThresholdConfigUpdated { domain_id, min_signatures, total_attesters }`
- `ThresholdUpdated { new_threshold }`
- `AttestationPaused`
- `AttestationUnpaused`

## Errors

- `AttestationPaused` - Operations disabled during pause
- `AttesterNotFound` - Attester ID doesn't exist
- `AttesterAlreadyExists` - Public key already registered
- `AttesterNotActive` - Attester is disabled/removed
- `MaxAttestersReached` - Registry is full
- `InvalidSignature` - Signature verification failed
- `InvalidPublicKey` - Invalid key format/length
- `SignatureAlreadySubmitted` - Attester already signed this message
- `AttestationNotFound` - No attestation exists for message
- `AttestationExpired` - Attestation too old
- `InsufficientSignatures` - Not enough signatures (< M)
- `InvalidThreshold` - Invalid M-of-N configuration
- `MessageHashMismatch` - Computed hash doesn't match
- `InvalidChainId` - Chain ID mismatch
- `NonceAlreadyUsed` - Replay attack detected
- `InvalidSignatureLength` - Signature wrong size
- `UnsupportedSignatureType` - Unsupported key/sig combination

## Helper Functions

### Public API (callable by other pallets)

```rust
// Hash a message using Blake2-256
pub fn hash_message(message: &[u8]) -> H256

// Get this runtime's chain ID
pub fn get_chain_id() -> u32

// Get and increment attestation nonce
pub fn get_and_increment_nonce() -> u64

// Get threshold for a specific domain
pub fn get_threshold_for_domain(domain: u32) -> u32

// Verify attestation (called by other pallets)
pub fn verify_attestation_for_message(message: &[u8], message_hash: H256) -> DispatchResult

// Check if attestation is valid without consuming nonce
pub fn is_attestation_valid(message_hash: H256) -> bool

// Get list of active attester IDs
pub fn get_active_attesters() -> Vec<u32>
```

## Integration Example

### In your bridge token messenger pallet:

```rust
use pallet_bridge_attestation as BridgeAttestation;

// In your receive_message extrinsic:
pub fn receive_message(
    origin: OriginFor<T>,
    message: Vec<u8>,
    message_hash: H256,
) -> DispatchResult {
    ensure_signed(origin)?;

    // Verify attestation before processing
    BridgeAttestation::<T>::verify_attestation_for_message(&message, message_hash)?;

    // Process the message...
    Self::process_bridge_message(message)?;

    Ok(())
}
```

## Chain-Specific Configurations

### For EDSC Bridge (Ethereum-compatible):
```rust
pub const EDSCChainId: u32 = 1; // Ethereum mainnet
pub const EDSCMinThreshold: u32 = 2;
pub const EDSCMaxAttesters: u32 = 50;
```

### For BSC Bridge:
```rust
pub const BSCChainId: u32 = 56; // BSC mainnet
pub const BSCMinThreshold: u32 = 3;
pub const BSCMaxAttesters: u32 = 100;
```

### For Polkadot Bridge:
```rust
pub const PolkadotChainId: u32 = 0; // Polkadot relay chain
pub const PolkadotMinThreshold: u32 = 5;
pub const PolkadotMaxAttesters: u32 = 200;
```

## Security Considerations

1. **Attester Selection**: Choose geographically and organizationally distributed attesters
2. **Threshold Selection**: M should be ≥ 2/3 of N for Byzantine fault tolerance
3. **Key Management**: Attesters must securely manage their private keys
4. **Regular Rotation**: Rotate attesters periodically to prevent key compromise
5. **Monitoring**: Monitor attester activity and signature rates
6. **Emergency Response**: Use pause mechanism if compromise detected

## Testing

Run the test suite:

```bash
cargo test -p pallet-bridge-attestation
```

## Benchmarking

Generate weight functions (TODO):

```bash
cargo run --release --features runtime-benchmarks \
    --bin YOUR_NODE \
    -- benchmark pallet \
    --chain=dev \
    --pallet=pallet_bridge_attestation \
    --extrinsic='*' \
    --steps=50 \
    --repeat=20 \
    --output=./pallets-shared/pallet-bridge-attestation/src/weights.rs
```

## Differences from EDSC-Specific Version

This generic pallet improves upon the EDSC-specific implementation with:

1. **Configurable ChainId**: Each PBC can set its own chain ID
2. **Enhanced Signature Support**: Better handling of both ECDSA and SR25519
3. **Improved Nonce System**: More robust replay attack prevention
4. **Domain-Specific Thresholds**: Per-domain M-of-N configurations
5. **Helper Functions**: Rich public API for other pallets
6. **Better Documentation**: Comprehensive inline docs and README
7. **Weight Traits**: Proper WeightInfo trait for benchmarking

## License

MIT OR Apache-2.0

## Contributing

See the main Etrid repository for contribution guidelines.
