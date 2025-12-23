# Quick Reference Card: pallet-bridge-attestation

## 30-Second Overview

Generic Substrate pallet for M-of-N threshold signature verification in cross-chain bridges. Supports both ECDSA (EVM) and SR25519 (Substrate) signatures.

## Quick Start

```rust
// 1. Add to Cargo.toml
pallet-bridge-attestation = { path = "../../pallets-shared/pallet-bridge-attestation", default-features = false }

// 2. Configure runtime
parameter_types! {
    pub const YourChainId: u32 = 100;
}

impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = YourChainId;
    type MaxAttesters = ConstU32<100>;
    type MaxAttestersPerMessage = ConstU32<20>;
    type MinSignatureThreshold = ConstU32<2>;
    type AttestationMaxAge = ConstU32<1000>;
    type WeightInfo = ();
}

// 3. Use in your bridge pallet
BridgeAttestation::<T>::verify_attestation_for_message(&message, hash)?;
```

## Common Commands

### Governance (sudo/governance required)

```javascript
// Register attester
api.tx.sudo.sudo(
  api.tx.bridgeAttestation.registerAttester(publicKey)
)

// Configure threshold (2 of 5)
api.tx.sudo.sudo(
  api.tx.bridgeAttestation.configureThreshold(null, 2, 5)
)

// Disable attester
api.tx.sudo.sudo(
  api.tx.bridgeAttestation.disableAttester(attesterId)
)

// Emergency pause
api.tx.sudo.sudo(
  api.tx.bridgeAttestation.pauseAttestation()
)
```

### Public Operations (anyone can call)

```javascript
// Submit signature
api.tx.bridgeAttestation.submitSignature(
  attesterId,
  messageHash,
  signature,
  sourceChainId,
  destChainId,
  nonce
)

// Verify attestation
api.tx.bridgeAttestation.verifyAttestation(
  message,
  messageHash
)
```

## Key Functions (for other pallets)

```rust
// Verify attestation (consumes nonce)
BridgeAttestation::<T>::verify_attestation_for_message(
    &message,
    message_hash
)?;

// Check validity (read-only, doesn't consume nonce)
let is_valid = BridgeAttestation::<T>::is_attestation_valid(message_hash);

// Get chain ID
let chain_id = BridgeAttestation::<T>::get_chain_id();

// Generate nonce
let nonce = BridgeAttestation::<T>::get_and_increment_nonce();

// Hash message
let hash = BridgeAttestation::<T>::hash_message(&message);

// Get threshold
let threshold = BridgeAttestation::<T>::get_threshold_for_domain(domain_id);

// Get active attesters
let attesters = BridgeAttestation::<T>::get_active_attesters();
```

## Key Storage Items

| Storage | Type | Description |
|---------|------|-------------|
| `Attesters` | `Map<u32, AttesterInfo>` | Attester registry |
| `ActiveAttesterCount` | `u32` | Active attester count |
| `Attestations` | `Map<H256, Attestation>` | Message attestations |
| `GlobalThreshold` | `ThresholdConfig` | Global M-of-N config |
| `IsPaused` | `bool` | Emergency pause state |
| `AttestationNonce` | `u64` | Next nonce |
| `UsedNonces` | `Map<u64, bool>` | Replay protection |

## Common Queries

```javascript
// Get attester info
const attester = await api.query.bridgeAttestation.attesters(id);

// Check active count
const count = await api.query.bridgeAttestation.activeAttesterCount();

// Get attestation
const att = await api.query.bridgeAttestation.attestations(hash);

// Check threshold
const threshold = await api.query.bridgeAttestation.globalThreshold();

// Check if paused
const paused = await api.query.bridgeAttestation.isPaused();

// Get current nonce
const nonce = await api.query.bridgeAttestation.attestationNonce();
```

## Events to Watch

```rust
AttesterRegistered { attester_id, public_key }
SignatureSubmitted { attester_id, message_hash, nonce }
AttestationThresholdReached { message_hash, signature_count, nonce }
AttestationVerified { message_hash, signature_count, source_chain_id, destination_chain_id }
ThresholdUpdated { new_threshold }
AttestationPaused
AttestationUnpaused
```

## Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `AttesterNotFound` | Invalid attester ID | Check attester exists |
| `InsufficientSignatures` | Not enough signatures | Wait for more attesters |
| `NonceAlreadyUsed` | Replay attack | Use fresh nonce |
| `InvalidSignature` | Sig verification failed | Check key/sig match |
| `AttestationExpired` | Too old | Increase `AttestationMaxAge` |
| `AttestationPaused` | Service paused | Wait for unpause |

## Signature Types

| Type | Public Key Size | Signature Size | Use Case |
|------|----------------|----------------|----------|
| ECDSA (compressed) | 33 bytes | 65 bytes | EVM bridges |
| ECDSA (uncompressed) | 65 bytes | 65 bytes | EVM bridges |
| SR25519 | 32 bytes | 64 bytes | Substrate bridges |

## Threshold Guidelines

| Security Level | M of N | Example | Notes |
|----------------|--------|---------|-------|
| Minimum | 1 of 3 | 1/3 | Not recommended for production |
| Standard | 2 of 3 | 2/3 | Good for testnet |
| Production | 5 of 7 | ~71% | Recommended minimum |
| High Security | 10 of 15 | 67% | Large bridge |
| Maximum | 20 of 30 | 67% | Critical infrastructure |

**Rule of thumb**: M ≥ 2/3 * N for Byzantine fault tolerance

## Chain ID Reference

| Chain | ID | Type |
|-------|-----|------|
| Ethereum Mainnet | 1 | EVM |
| BSC Mainnet | 56 | EVM |
| Polygon | 137 | EVM |
| Avalanche C-Chain | 43114 | EVM |
| Fantom | 250 | EVM |
| Arbitrum One | 42161 | EVM |
| Optimism | 10 | EVM |
| Polkadot | 0 | Substrate |
| Kusama | 2 | Substrate |
| FlareChain (Etrid) | 1000 | Substrate |

## Configuration Templates

### Small Testnet Bridge
```rust
type ChainId = ConstU32<999>; // Testnet
type MaxAttesters = ConstU32<10>;
type MaxAttestersPerMessage = ConstU32<5>;
type MinSignatureThreshold = ConstU32<2>; // 2 of 5
type AttestationMaxAge = ConstU32<100>; // ~3 minutes
```

### Production EVM Bridge
```rust
type ChainId = ConstU32<1>; // Ethereum
type MaxAttesters = ConstU32<100>;
type MaxAttestersPerMessage = ConstU32<20>;
type MinSignatureThreshold = ConstU32<14>; // 14 of 20 (70%)
type AttestationMaxAge = ConstU32<1000>; // ~2 hours
```

### High-Security Substrate Bridge
```rust
type ChainId = ConstU32<0>; // Polkadot
type MaxAttesters = ConstU32<200>;
type MaxAttestersPerMessage = ConstU32<30>;
type MinSignatureThreshold = ConstU32<20>; // 20 of 30 (67%)
type AttestationMaxAge = ConstU32<600>; // ~1 hour
```

## Integration Checklist

- [ ] Add pallet dependency to Cargo.toml
- [ ] Configure runtime parameters
- [ ] Implement Config trait
- [ ] Add to construct_runtime!
- [ ] Register initial attesters
- [ ] Set threshold configuration
- [ ] Deploy attester services
- [ ] Test signature submission
- [ ] Test attestation verification
- [ ] Monitor events and metrics
- [ ] Set up governance procedures
- [ ] Document emergency procedures

## Performance Tips

1. **Batch Operations**: Submit multiple signatures in parallel
2. **Cache Threshold**: Cache threshold config to reduce storage reads
3. **Prune Old Nonces**: Implement nonce cleanup for old entries
4. **Weight Optimization**: Run benchmarks to get accurate weights
5. **Event Filtering**: Use event filters to reduce RPC load

## Security Checklist

- [ ] M ≥ 2/3 * N threshold configured
- [ ] Attesters geographically distributed
- [ ] Attester keys secured (HSM/cold storage)
- [ ] Monitoring alerts configured
- [ ] Emergency pause procedure documented
- [ ] Key rotation schedule established
- [ ] Incident response plan ready
- [ ] Regular security audits scheduled

## File Locations

```
pallets-shared/pallet-bridge-attestation/
├── Cargo.toml                 # Dependencies
├── README.md                  # Full documentation
├── INTEGRATION_GUIDE.md       # Step-by-step guide
├── CHANGES_FROM_EDSC.md      # Migration guide
├── QUICK_REFERENCE.md        # This file
└── src/
    ├── lib.rs                # Main pallet code (1053 lines)
    ├── weights.rs            # Weight functions (89 lines)
    ├── mock.rs               # Test runtime (85 lines)
    └── tests.rs              # Test suite (337 lines)
```

## Support & Resources

- **Full Docs**: [README.md](./README.md)
- **Integration Guide**: [INTEGRATION_GUIDE.md](./INTEGRATION_GUIDE.md)
- **Migration Guide**: [CHANGES_FROM_EDSC.md](./CHANGES_FROM_EDSC.md)
- **EDSC Example**: `05-multichain/bridges/protocols/edsc-bridge/`
- **Substrate Docs**: https://docs.substrate.io

## Quick Debug Commands

```bash
# Check compilation
cargo check -p pallet-bridge-attestation

# Run tests
cargo test -p pallet-bridge-attestation

# Run specific test
cargo test -p pallet-bridge-attestation register_attester_works

# Build with benchmarks
cargo build --release --features runtime-benchmarks

# Check code coverage
cargo tarpaulin -p pallet-bridge-attestation
```

## Common Patterns

### Pattern 1: Sending a message
```rust
// 1. Create message
let message = encode_bridge_message(...);

// 2. Get nonce
let nonce = BridgeAttestation::<T>::get_and_increment_nonce();

// 3. Hash message
let hash = BridgeAttestation::<T>::hash_message(&message);

// 4. Emit event (attesters watch this)
Self::deposit_event(Event::MessageSent { hash, nonce });
```

### Pattern 2: Receiving a message
```rust
// 1. Verify attestation
BridgeAttestation::<T>::verify_attestation_for_message(&message, hash)?;

// 2. Decode message
let decoded = decode_bridge_message(&message)?;

// 3. Process message
Self::process_message(decoded)?;
```

### Pattern 3: Attester service
```javascript
// 1. Listen for MessageSent events
api.query.system.events((events) => {
  for (event of events) {
    if (event.method === 'MessageSent') {
      // 2. Validate message
      const isValid = await validateMessage(event.hash);

      if (isValid) {
        // 3. Sign hash
        const signature = signHash(event.hash);

        // 4. Submit signature
        await api.tx.bridgeAttestation.submitSignature(
          attesterId,
          event.hash,
          signature,
          sourceChain,
          destChain,
          event.nonce
        ).signAndSend(attesterAccount);
      }
    }
  }
});
```

## One-Liners

```bash
# Count active attesters
cast call BRIDGE_ATTESTATION "activeAttesterCount()" --rpc-url $RPC

# Get threshold
cast call BRIDGE_ATTESTATION "globalThreshold()" --rpc-url $RPC

# Check if paused
cast call BRIDGE_ATTESTATION "isPaused()" --rpc-url $RPC
```

---

**Version**: 0.1.0
**Last Updated**: 2025-12-04
**Maintainer**: Etrid Team
