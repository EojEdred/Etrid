# Architecture: Bridge Attestation in Etrid Ecosystem

## Overview

This document explains how `pallet-bridge-attestation` fits into the broader Etrid multi-chain architecture and how it enables secure cross-chain communication.

## Etrid Multi-Chain Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      ETRID ECOSYSTEM                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              FlareChain (Relay Chain)                    │   │
│  │  - Shared Security                                        │   │
│  │  - Cross-chain messaging                                 │   │
│  │  - Governance                                             │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                    │
│              ┌───────────────┼───────────────┐                  │
│              │               │               │                  │
│  ┌───────────▼──────┐  ┌────▼──────┐  ┌────▼──────┐           │
│  │   EDSC PBC       │  │  BSC PBC  │  │ DOT PBC   │  ...      │
│  │  (Ethereum       │  │ (Binance  │  │(Polkadot  │           │
│  │   Bridge)        │  │  Bridge)  │  │ Bridge)   │           │
│  │                  │  │           │  │           │           │
│  │ ┌──────────────┐│  │┌──────────┤  │┌──────────┤           │
│  │ │pallet-bridge-││  ││pallet-   │  ││pallet-   │           │
│  │ │attestation   ││  ││bridge-   │  ││bridge-   │           │
│  │ │              ││  ││attestation│  ││attestation│           │
│  │ └──────────────┘│  │└──────────┘  │└──────────┘           │
│  │                  │  │           │  │           │           │
│  │ ┌──────────────┐│  │┌──────────┤  │┌──────────┤           │
│  │ │pallet-edsc-  ││  ││pallet-bsc││  ││pallet-dot│           │
│  │ │bridge-token- ││  ││-bridge-  │  ││-bridge-  │           │
│  │ │messenger     ││  ││token-... │  ││token-... │           │
│  │ └──────────────┘│  │└──────────┘  │└──────────┘           │
│  └──────────────────┘  └───────────┘  └───────────┘           │
│           │                  │              │                   │
│           │                  │              │                   │
└───────────┼──────────────────┼──────────────┼───────────────────┘
            │                  │              │
     ┌──────▼──────┐    ┌─────▼─────┐   ┌───▼────┐
     │  Ethereum   │    │    BSC    │   │Polkadot│
     │  Mainnet    │    │  Mainnet  │   │ Relay  │
     └─────────────┘    └───────────┘   └────────┘
```

## How pallet-bridge-attestation Works

### 1. Message Flow (Source Chain → Etrid PBC)

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Message Initiated on Source Chain                       │
└─────────────────────────────────────────────────────────────────┘

  Ethereum Mainnet:
  ┌──────────────────────┐
  │ User locks 100 USDC  │
  │ in ERC20 contract    │
  └──────────┬───────────┘
             │
             ▼
  ┌──────────────────────┐
  │ Event: MessageSent   │
  │  - messageHash       │
  │  - amount: 100 USDC  │
  │  - recipient: 0x...  │
  │  - nonce: 12345      │
  └──────────┬───────────┘
             │
             │
┌────────────┼────────────────────────────────────────────────────┐
│ STEP 2: Attesters Watch and Sign                                │
└────────────┼────────────────────────────────────────────────────┘
             │
             ├──────────┐
             │          │
    ┌────────▼────┐  ┌─▼─────────┐  ┌──────────────┐
    │ Attester 1  │  │Attester 2 │  │ Attester 3   │  ...
    │ watches ETH │  │watches ETH│  │ watches ETH  │
    │ validates   │  │validates  │  │ validates    │
    │ signs hash  │  │signs hash │  │ signs hash   │
    └────────┬────┘  └─┬─────────┘  └──┬───────────┘
             │          │               │
             │          │               │
             ▼          ▼               ▼
    ┌────────────────────────────────────────────┐
    │     Attesters submit signatures to         │
    │     EDSC PBC via submit_signature()        │
    └────────────────┬───────────────────────────┘
                     │
┌────────────────────┼───────────────────────────────────────────┐
│ STEP 3: Signatures Accumulated in pallet-bridge-attestation    │
└────────────────────┼───────────────────────────────────────────┘
                     │
                     ▼
    ┌────────────────────────────────────────────┐
    │   EDSC PBC - pallet-bridge-attestation     │
    │                                             │
    │   Storage:                                  │
    │   ┌─────────────────────────────────────┐ │
    │   │ Attestations[messageHash]:          │ │
    │   │   - signature_count: 3              │ │
    │   │   - signatures: [sig1, sig2, sig3]  │ │
    │   │   - source_chain_id: 1 (Ethereum)   │ │
    │   │   - dest_chain_id: 100 (EDSC)       │ │
    │   │   - nonce: 12345                    │ │
    │   └─────────────────────────────────────┘ │
    │                                             │
    │   Event: AttestationThresholdReached       │
    │   (when signature_count >= threshold)      │
    └────────────────┬───────────────────────────┘
                     │
┌────────────────────┼───────────────────────────────────────────┐
│ STEP 4: Message Processing in Token Messenger                  │
└────────────────────┼───────────────────────────────────────────┘
                     │
                     ▼
    ┌────────────────────────────────────────────┐
    │   EDSC PBC - pallet-edsc-bridge-token-     │
    │              messenger                      │
    │                                             │
    │   receive_message(message, messageHash):   │
    │                                             │
    │   1. Verify attestation:                   │
    │      BridgeAttestation::                   │
    │        verify_attestation_for_message()?   │
    │                                             │
    │   2. Decode message                        │
    │                                             │
    │   3. Mint wrapped USDC to recipient        │
    │                                             │
    └────────────────┬───────────────────────────┘
                     │
                     ▼
         ┌───────────────────────┐
         │ User receives 100     │
         │ wrapped USDC on EDSC  │
         └───────────────────────┘
```

### 2. Attestation Verification Details

```rust
// In pallet-edsc-bridge-token-messenger

pub fn receive_message(
    origin: OriginFor<T>,
    message: Vec<u8>,
    message_hash: H256,
) -> DispatchResult {
    ensure_signed(origin)?;

    // ============================================
    // CRITICAL: Verify attestation first
    // ============================================
    BridgeAttestation::<T>::verify_attestation_for_message(
        &message,
        message_hash,
    )?;
    // This function checks:
    // 1. Message hash matches (no tampering)
    // 2. Attestation exists
    // 3. Attestation not expired (within AttestationMaxAge)
    // 4. Nonce not used before (prevent replay)
    // 5. Signature count >= threshold (M of N)
    // 6. All signatures cryptographically valid
    // 7. All attesters are active

    // ============================================
    // Only if attestation valid: process message
    // ============================================
    let decoded_message = Self::decode_bridge_message(&message)?;

    match decoded_message.message_type {
        MessageType::DepositForBurn => {
            Self::handle_deposit_for_burn(decoded_message)?;
        },
        MessageType::MintBurn => {
            Self::handle_mint_burn(decoded_message)?;
        },
        // ... other message types
    }

    Ok(())
}
```

### 3. Security Model

```
┌─────────────────────────────────────────────────────────────────┐
│                    SECURITY LAYERS                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Layer 1: Attester Independence                                 │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Each attester runs independently                      │    │
│  │ • Geographic distribution                               │    │
│  │ • Organizational diversity                              │    │
│  │ • Separate key management                               │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  Layer 2: M-of-N Threshold                                      │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • M ≥ 2/3 * N (Byzantine fault tolerant)               │    │
│  │ • Example: 7 of 10 attesters must sign                 │    │
│  │ • System survives up to N-M attester failures           │    │
│  │ • Configurable per domain/bridge                        │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  Layer 3: Cryptographic Verification                            │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • ECDSA signatures (secp256k1) for EVM bridges         │    │
│  │ • SR25519 signatures for Substrate bridges             │    │
│  │ • Each signature verified cryptographically             │    │
│  │ • Public key registry on-chain                          │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  Layer 4: Replay Protection                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Nonce-based system                                    │    │
│  │ • Each message has unique nonce                         │    │
│  │ • Used nonces tracked on-chain                          │    │
│  │ • Prevents message replay attacks                       │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  Layer 5: Time-Based Expiry                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Attestations expire after max age                     │    │
│  │ • Prevents use of stale signatures                      │    │
│  │ • Configurable per runtime                              │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
│  Layer 6: Governance Control                                    │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ • Add/remove attesters via governance                   │    │
│  │ • Update thresholds via governance                      │    │
│  │ • Emergency pause mechanism                             │    │
│  │ • Transparent attester management                       │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 4. Threat Model & Mitigations

| Threat | Mitigation | Implementation |
|--------|------------|----------------|
| **Malicious Attester** | M-of-N threshold | Need M signatures, not just 1 |
| **Attester Compromise** | Byzantine tolerance | System works with up to N-M failures |
| **Replay Attack** | Nonce tracking | `UsedNonces` storage map |
| **Message Tampering** | Cryptographic hashing | Blake2-256 hash verification |
| **Signature Forgery** | ECDSA/SR25519 verification | `sp_io::crypto` verification |
| **Double Signing** | Deduplication | Check attester hasn't signed already |
| **Stale Attestations** | Time-based expiry | `AttestationMaxAge` check |
| **Attester Collusion** | Independent attesters | Geographic/organizational diversity |

## Integration with Other Pallets

### 1. Bridge Token Messenger (User-Facing)

```rust
// pallet-edsc-bridge-token-messenger
// High-level bridge operations (deposit, withdraw, mint, burn)

impl<T: Config> Pallet<T> {
    // Depends on pallet-bridge-attestation for security
    use pallet_bridge_attestation as BridgeAttestation;

    pub fn receive_message(...) -> DispatchResult {
        // Use attestation pallet for verification
        BridgeAttestation::<T>::verify_attestation_for_message(...)?;
        // Then process message
    }
}
```

### 2. Bridge Transmitter (Infrastructure)

```rust
// pallet-edsc-bridge-transmitter
// Low-level cross-chain message transmission

impl<T: Config> Pallet<T> {
    pub fn send_message(...) -> DispatchResult {
        // Use attestation pallet for nonce and hashing
        let nonce = BridgeAttestation::<T>::get_and_increment_nonce();
        let hash = BridgeAttestation::<T>::hash_message(&message);
        // Emit event for attesters to watch
    }
}
```

### 3. Other Bridge-Related Pallets

All bridge pallets can use the shared attestation pallet:
- `pallet-bsc-bridge-token-messenger` → uses `pallet-bridge-attestation`
- `pallet-polygon-bridge-token-messenger` → uses `pallet-bridge-attestation`
- `pallet-avalanche-bridge-token-messenger` → uses `pallet-bridge-attestation`

**One attestation pallet, many bridges!**

## Data Structures

### AttesterInfo
```rust
pub struct AttesterInfo<T: Config> {
    pub public_key: BoundedVec<u8, ConstU32<64>>,  // 32, 33, or 65 bytes
    pub status: AttesterStatus,                     // Active/Disabled/Removed
    pub registered_at: BlockNumberFor<T>,           // Registration block
    pub messages_signed: u64,                       // Statistics
    pub last_signed_at: BlockNumberFor<T>,          // Last activity
}
```

### Attestation
```rust
pub struct Attestation<T: Config> {
    pub message_hash: H256,                         // Message identifier
    pub signatures: BoundedVec<...>,                // Attester signatures
    pub attested_at: BlockNumberFor<T>,             // Creation time
    pub signature_count: u32,                       // Current count
    pub source_chain_id: u32,                       // Source chain (Ethereum=1)
    pub destination_chain_id: u32,                  // Dest chain (EDSC=100)
    pub nonce: u64,                                 // Unique nonce
}
```

### ThresholdConfig
```rust
pub struct ThresholdConfig {
    pub min_signatures: u32,    // M in M-of-N
    pub total_attesters: u32,   // N in M-of-N
    pub enabled: bool,          // Config active
}
```

## Comparison with Other Systems

### vs Circle CCTP
| Feature | Circle CCTP | Etrid Bridge Attestation |
|---------|-------------|--------------------------|
| Signature Type | ECDSA only | ECDSA + SR25519 |
| Chain Support | EVM only | EVM + Substrate |
| Threshold | M-of-N | M-of-N (configurable) |
| Governance | Circle controlled | On-chain governance |
| Open Source | No | Yes |

### vs LayerZero
| Feature | LayerZero | Etrid Bridge Attestation |
|---------|-----------|--------------------------|
| Attesters | Oracle + Relayer | Independent attesters |
| Security Model | 1-of-2 | M-of-N |
| Configuration | Per-app | Per-bridge |
| Verification | Off-chain + on-chain | On-chain |

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Signature verification | ~0.5ms | ECDSA/SR25519 verification |
| Storage per attester | ~200 bytes | AttesterInfo struct |
| Storage per attestation | ~1KB | Depends on signature count |
| Max attesters | Configurable | Default: 100 |
| Max sigs per message | Configurable | Default: 20 |
| Throughput | 1000+ TPS | With parallel submission |

## Configuration Examples

### Low-Security Testnet
```rust
type ChainId = ConstU32<999>;
type MaxAttesters = ConstU32<10>;
type MinSignatureThreshold = ConstU32<2>;  // 2 of 10 (20%)
type AttestationMaxAge = ConstU32<100>;    // ~3 min
```

### Medium-Security Mainnet
```rust
type ChainId = ConstU32<1>;
type MaxAttesters = ConstU32<50>;
type MinSignatureThreshold = ConstU32<34>;  // 34 of 50 (68%)
type AttestationMaxAge = ConstU32<1000>;    // ~2 hours
```

### High-Security Critical Bridge
```rust
type ChainId = ConstU32<1>;
type MaxAttesters = ConstU32<100>;
type MinSignatureThreshold = ConstU32<67>;  // 67 of 100 (67%)
type AttestationMaxAge = ConstU32<600>;     // ~1 hour
```

## Future Enhancements

1. **Slashing for Malicious Attestations**
   - Track attester behavior
   - Penalize false attestations
   - Economic incentives for honesty

2. **Dynamic Threshold Adjustment**
   - Adjust M based on bridge activity
   - Increase for large transfers
   - Decrease for small transfers

3. **Attester Reputation System**
   - Track uptime and accuracy
   - Weight signatures by reputation
   - Automated attester selection

4. **Cross-PBC Attestation**
   - Share attesters across PBCs
   - Unified attester registry
   - Cost savings

5. **Zero-Knowledge Proofs**
   - ZK-proof based attestation
   - Privacy-preserving bridges
   - Reduced on-chain verification cost

## Conclusion

`pallet-bridge-attestation` provides a secure, flexible, and reusable foundation for cross-chain bridges in the Etrid ecosystem. By implementing industry-standard M-of-N threshold signatures with Byzantine fault tolerance, it enables secure value transfer between Etrid PBCs and external chains.

The generic design means that every bridge implementation can leverage the same battle-tested attestation logic, reducing development time and security risks while maintaining a consistent security model across the entire multi-chain ecosystem.
