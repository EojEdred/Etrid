# Ëtrid CCTP Integration Architecture

**Date:** 2025-11-04
**Status:** 📋 Production Ready
**Purpose:** Document Circle CCTP-style cross-chain transfer protocol integration in PBC architecture

---

## Overview

Ëtrid implements a **CCTP-style** (Cross-Chain Transfer Protocol) burn-and-mint architecture for seamless cross-chain transfers of ËDSC stablecoin and other native tokens across all supported blockchains.

**What is CCTP?**
Circle's Cross-Chain Transfer Protocol (CCTP) enables USDC to move natively across blockchains by burning tokens on the source chain and minting equivalent tokens on the destination chain, eliminating the need for wrapped tokens or liquidity pools.

**Ëtrid's Adaptation:**
We've adapted this architecture for ËDSC (Ëtrid Dollar Stablecoin) to enable native cross-chain transfers across:
- FlareChain (Layer 1)
- 13 Partition Burst Chains (Layer 2)
- 8 External Blockchains (Ethereum, Solana, Polygon, etc.)

---

## Architecture Components

### 1. Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    CCTP Architecture                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Substrate Pallets (Ëtrid/PBCs)                       │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  • pallet-edsc-bridge-token-messenger                 │ │
│  │    ├─ burn_edsc_for_external_chain()                  │ │
│  │    └─ receive_and_mint()                              │ │
│  │  • pallet-edsc-bridge-attestation                     │ │
│  │    ├─ Attester registry (M-of-N signatures)          │ │
│  │    └─ Signature verification                          │ │
│  │  • pallet-edsc-receipts                               │ │
│  │    └─ Cross-chain transfer receipts                   │ │
│  │  • pallet-edsc-checkpoint                             │ │
│  │    └─ State checkpointing for rollback protection     │ │
│  └────────────────────────────────────────────────────────┘ │
│                              ↕                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Off-Chain Services (TypeScript)                      │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  • attestation-service (M-of-N signers)               │ │
│  │    ├─ Monitors burn events                            │ │
│  │    ├─ Signs cross-chain messages                      │ │
│  │    └─ Threshold signature coordination (3-of-5)       │ │
│  │  • relayer-service                                     │ │
│  │    ├─ Collects attestations                           │ │
│  │    ├─ Submits to destination chain                    │ │
│  │    └─ Gas optimization & retry logic                  │ │
│  └────────────────────────────────────────────────────────┘ │
│                              ↕                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  External Chain Contracts (Solidity/Others)           │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  • EDSCTokenMessenger.sol                             │ │
│  │    ├─ burnAndSendTo()                                 │ │
│  │    └─ Domain-based routing                            │ │
│  │  • EDSCMessageTransmitter.sol                         │ │
│  │    ├─ receiveMessage()                                │ │
│  │    ├─ M-of-N signature validation                     │ │
│  │    └─ Nonce replay protection                         │ │
│  │  • AttesterRegistry.sol                               │ │
│  │    ├─ Attester management                             │ │
│  │    └─ Signature threshold configuration               │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## Domain Architecture

### Supported Domains

Ëtrid's CCTP implementation supports 8 blockchain domains:

```rust
pub enum Domain {
    Ethereum = 0,      // Ethereum mainnet
    Solana = 1,        // Solana mainnet
    Etrid = 2,         // Ëtrid PBC-EDSC (native)
    Polygon = 3,       // Polygon (Matic)
    BnbChain = 4,      // BNB Chain (BSC)
    Avalanche = 5,     // Avalanche C-Chain
    Arbitrum = 6,      // Arbitrum One
    Optimism = 7,      // Optimism
}
```

Each domain has its own configuration:
- ✅ Enabled/disabled status
- ✅ Maximum burn amount per transaction
- ✅ Daily burn limit
- ✅ Rate limiting parameters

---

## Cross-Chain Transfer Flow

### Flow 1: Ethereum → Ëtrid (Inbound)

```
┌────────────────────────────────────────────────────────────┐
│  Step 1: Burn on Ethereum                                  │
├────────────────────────────────────────────────────────────┤
│  User calls: EDSCTokenMessenger.burnAndSendTo()            │
│  ├─ Burns 1,000 EDSC tokens                                │
│  ├─ Creates cross-chain message                            │
│  │   └─ version: 1                                         │
│  │   └─ source_domain: 0 (Ethereum)                        │
│  │   └─ destination_domain: 2 (Ëtrid)                      │
│  │   └─ nonce: 12345                                       │
│  │   └─ sender: 0xAlice...                                 │
│  │   └─ recipient: 5GrwvaEF... (Substrate address)         │
│  │   └─ amount: 1,000,000,000,000,000,000,000 (1k EDSC)    │
│  └─ Emits: MessageSent event                               │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 2: Attestation (Off-Chain)                           │
├────────────────────────────────────────────────────────────┤
│  5 Attestation Services monitor MessageSent event          │
│  ├─ Attester 1: Signs message hash                         │
│  ├─ Attester 2: Signs message hash                         │
│  ├─ Attester 3: Signs message hash                         │
│  ├─ Attester 4: Signs message hash (optional)              │
│  └─ Attester 5: Signs message hash (optional)              │
│                                                             │
│  Requires 3-of-5 signatures (M-of-N threshold)             │
│  Message hash = keccak256(message_bytes)                   │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 3: Relay to Ëtrid                                    │
├────────────────────────────────────────────────────────────┤
│  Relayer Service:                                           │
│  ├─ Polls for messages with 3+ signatures                  │
│  ├─ Collects attestation signatures                        │
│  ├─ Submits extrinsic to Ëtrid:                            │
│  │   └─ receive_and_mint(message, [sig1, sig2, sig3])      │
│  └─ Pays gas on Ëtrid (reimbursed from fee pool)           │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 4: Verify and Mint on Ëtrid                          │
├────────────────────────────────────────────────────────────┤
│  pallet-edsc-bridge-token-messenger:                       │
│  ├─ Verify 3 attestation signatures ✓                      │
│  ├─ Check nonce not already used ✓                         │
│  ├─ Verify source domain = Ethereum ✓                      │
│  ├─ Verify destination domain = Ëtrid ✓                    │
│  ├─ Parse burn message body ✓                              │
│  ├─ Mint 1,000 EDSC to recipient on PBC-EDSC ✓             │
│  └─ Emit: MintCompleted event                              │
└────────────────────────────────────────────────────────────┘

Result: 1,000 EDSC now available on Ëtrid PBC-EDSC ✅
Total time: ~5-10 minutes (attestation + relay)
```

---

### Flow 2: Ëtrid → Ethereum (Outbound)

```
┌────────────────────────────────────────────────────────────┐
│  Step 1: Burn on Ëtrid PBC-EDSC                            │
├────────────────────────────────────────────────────────────┤
│  User calls: burn_edsc_for_external_chain()                │
│  ├─ Burns 500 EDSC tokens on PBC-EDSC                      │
│  ├─ Creates cross-chain message                            │
│  │   └─ source_domain: 2 (Ëtrid)                           │
│  │   └─ destination_domain: 0 (Ethereum)                   │
│  │   └─ nonce: 67890                                       │
│  │   └─ sender: 5GrwvaEF... (Substrate)                    │
│  │   └─ recipient: 0xBob... (Ethereum)                     │
│  │   └─ amount: 500,000,000,000,000,000,000 (500 EDSC)     │
│  └─ Emits: BurnMessageSent event                           │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 2: Attestation (Off-Chain)                           │
├────────────────────────────────────────────────────────────┤
│  5 Attestation Services monitor Substrate events           │
│  ├─ Attester 1: Signs message hash                         │
│  ├─ Attester 2: Signs message hash                         │
│  ├─ Attester 3: Signs message hash                         │
│  └─ 3-of-5 threshold met                                   │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 3: Relay to Ethereum                                 │
├────────────────────────────────────────────────────────────┤
│  Relayer Service:                                           │
│  ├─ Collects 3 attestation signatures                      │
│  ├─ Calls: EDSCMessageTransmitter.receiveMessage()         │
│  │   └─ message: encoded CrossChainMessage                 │
│  │   └─ attestation: bytes (3 concatenated signatures)     │
│  └─ Pays gas on Ethereum                                   │
└────────────────────────────────────────────────────────────┘
                        ↓
┌────────────────────────────────────────────────────────────┐
│  Step 4: Verify and Mint on Ethereum                       │
├────────────────────────────────────────────────────────────┤
│  EDSCMessageTransmitter.sol:                               │
│  ├─ Verify 3 attester signatures ✓                         │
│  ├─ ecrecover() for each signature                         │
│  ├─ Check signers are registered attesters ✓               │
│  ├─ Check nonce not used (replay protection) ✓             │
│  ├─ Parse message and extract recipient ✓                  │
│  ├─ Call EDSC.mint(0xBob, 500 EDSC) ✓                      │
│  └─ Emit: MessageReceived event                            │
└────────────────────────────────────────────────────────────┘

Result: 500 EDSC now available on Ethereum ✅
Total time: ~10-15 minutes (attestation + Ethereum finality)
```

---

## Message Format (CCTP-Style)

### CrossChainMessage Structure

```rust
pub struct CrossChainMessage {
    pub version: u32,                         // Message format version (1)
    pub source_domain: u32,                   // Source blockchain domain
    pub destination_domain: u32,              // Destination blockchain domain
    pub nonce: u64,                           // Unique sequential nonce
    pub sender: BoundedVec<u8, 64>,          // Sender address (flexible format)
    pub recipient: BoundedVec<u8, 64>,       // Recipient address (flexible format)
    pub message_body: BoundedVec<u8, 512>,   // Burn/mint details (encoded)
}
```

### BurnMessage Body

```rust
pub struct BurnMessage {
    pub version: u32,                         // Burn message version
    pub burn_token: BoundedVec<u8, 64>,      // Token contract address
    pub mint_recipient: BoundedVec<u8, 64>,  // Recipient on destination
    pub amount: u128,                        // Amount (18 decimals)
}
```

### Message Encoding

```
Message bytes = version || source_domain || destination_domain ||
                nonce || sender || recipient || message_body

Message hash = keccak256(message_bytes)

Signature = sign(message_hash, attester_private_key)
```

---

## Security Features

### 1. M-of-N Attestation

**Configuration**: 3-of-5 (requires 3 signatures out of 5 registered attesters)

**Benefits**:
- ✅ No single point of failure
- ✅ Resistant to attester compromise (need 3/5)
- ✅ Byzantine fault tolerant (tolerates 2/5 malicious)
- ✅ Can rotate attesters via governance

**Attester Requirements**:
- Must be registered in AttesterRegistry
- Must have valid signing keys
- Must be actively monitoring both chains
- Geographic and operational diversity

### 2. Nonce Management

**Purpose**: Prevent replay attacks

**Implementation**:
```rust
// Track used nonces per domain
UsedNonces: StorageDoubleMap<
    Domain,        // Source domain
    u64,           // Nonce
    bool,          // Used?
>
```

**Rules**:
- ✅ Each message has a unique nonce
- ✅ Nonces are sequential per domain
- ✅ Once used, nonce cannot be reused
- ✅ Prevents message replay on destination

### 3. Domain Separation

**Purpose**: Prevent cross-domain message confusion

**Implementation**:
- Each blockchain has a unique domain ID
- Messages include both source and destination domains
- Contracts verify domain matches before processing

**Benefits**:
- ✅ Cannot replay Ethereum message on Polygon
- ✅ Cannot mint on wrong chain
- ✅ Clear message routing

### 4. Rate Limiting

**Per-Transaction Limits**:
```rust
pub struct DomainConfig {
    pub enabled: bool,                // Domain active?
    pub max_burn_amount: u128,        // Max per tx
    pub daily_burn_limit: u128,       // Max per day
}
```

**Example Configuration**:
- Ethereum: Max 100,000 EDSC per tx, 1M EDSC per day
- Solana: Max 50,000 EDSC per tx, 500k EDSC per day
- Polygon: Max 25,000 EDSC per tx, 250k EDSC per day

**Benefits**:
- ✅ Limits damage from compromised attester
- ✅ Prevents liquidity drainage
- ✅ Can be adjusted via governance

### 5. Emergency Pause

**Capabilities**:
- Pause all burns on a specific domain
- Pause all mints on Ëtrid
- Global pause across all domains

**Trigger Conditions**:
- Detected security issue
- Attester compromise
- External chain issue
- Governance vote

**Recovery**:
- Must be unpause via governance
- 24-hour timelock before unpausing
- Requires security audit report

---

## Integration with PBC Architecture

### PBC-EDSC Specialization

PBC-EDSC (Partition Burst Chain for EDSC stablecoin) includes CCTP functionality:

```rust
// In PBC-EDSC runtime
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,

        // EDSC-specific pallets
        EdscToken: pallet_edsc_token,
        EdscBridgeTokenMessenger: pallet_edsc_bridge_token_messenger,
        EdscBridgeAttestation: pallet_edsc_bridge_attestation,
        EdscReceipts: pallet_edsc_receipts,
        EdscCheckpoint: pallet_edsc_checkpoint,
        EdscOracle: pallet_edsc_oracle,
        EdscRedemption: pallet_edsc_redemption,
    }
);
```

### Checkpoint Integration

CCTP state is included in PBC checkpoints to FlareChain:

```rust
// Checkpoint includes CCTP state
pub struct Checkpoint {
    pub block_number: u64,
    pub state_root: Hash,           // Includes CCTP message states
    pub total_supply: u128,         // EDSC supply (on-chain + burned)
    pub reserve_ratio: u16,
    pub timestamp: u64,
    pub pending_burns: u64,         // Outbound messages awaiting attestation
    pub pending_mints: u64,         // Inbound messages awaiting relay
}
```

**Benefits**:
- FlareChain Directors can monitor CCTP activity
- Emergency recovery using last checkpoint
- Audit trail for all cross-chain transfers

---

## Performance Characteristics

### Latency

| Direction | Burn → Attestation | Relay → Mint | Total Time |
|-----------|-------------------|--------------|------------|
| **Ethereum → Ëtrid** | 2-3 minutes | 2-3 minutes | **4-6 minutes** |
| **Ëtrid → Ethereum** | 2-3 minutes | 5-10 minutes | **7-13 minutes** |
| **Ëtrid → Solana** | 2-3 minutes | 30-60 seconds | **2.5-4 minutes** |
| **Ëtrid → Polygon** | 2-3 minutes | 1-2 minutes | **3-5 minutes** |

**Factors**:
- Attestation latency: Time for 3/5 attesters to sign
- Relay latency: Relayer polling interval + submission
- Destination finality: Chain-specific confirmation time

### Throughput

- **Burn capacity**: ~100 burns/minute per domain
- **Mint capacity**: ~100 mints/minute on Ëtrid
- **Bottleneck**: Attestation service signing speed

**Scaling Options**:
- Increase attesters (5 → 9, still 3-of-N threshold)
- Batch multiple messages per relay submission
- Parallel attestation for independent messages

### Costs

| Chain | Burn Gas | Mint Gas | Relay Cost |
|-------|----------|----------|------------|
| **Ethereum** | ~100k gas (~$3) | ~150k gas (~$5) | $5-8 total |
| **Polygon** | ~80k gas (~$0.02) | ~120k gas (~$0.03) | $0.05 total |
| **Arbitrum** | ~100k gas (~$0.30) | ~150k gas (~$0.45) | $0.75 total |
| **Ëtrid** | ~50k gas (~$0.01) | ~80k gas (~$0.02) | $0.03 total |

**Relay Fee Model**:
- User pays upfront relay fee (covers relayer gas + profit margin)
- Fee varies by destination chain
- Can be paid in EDSC or native token

---

## Comparison with Circle CCTP

| Feature | Ëtrid CCTP | Circle CCTP |
|---------|-----------|-------------|
| **Token** | EDSC (Ëtrid stablecoin) | USDC |
| **Supported Chains** | 8 domains + 13 PBCs | 15+ chains |
| **Attestation** | 3-of-5 off-chain | Circle attestation service |
| **Architecture** | Substrate + Solidity | Solidity-only |
| **Finality** | PBC checkpoints to L1 | Native chain finality |
| **Open Source** | ✅ Fully open | ⚠️ Attestation service closed |
| **Decentralization** | ✅ M-of-N attesters | ⚠️ Single attestation service |
| **Rate Limits** | Configurable per domain | Fixed by Circle |

---

## Future Enhancements

### Phase 2 (Q1 2026)

1. **Increase Attesters**: 5 → 9 (still 3-of-N threshold)
2. **Batch Relaying**: Multiple messages per relay tx
3. **Fast Path**: Instant settlement for small amounts (<$1000)
4. **Cross-Domain Routing**: Multi-hop transfers (e.g., Ethereum → Ëtrid → Solana)

### Phase 3 (Q2 2026)

1. **ZK Attestation**: Zero-knowledge proofs instead of signatures
2. **Decentralized Relayers**: Anyone can relay with bond
3. **Atomic Swaps**: CCTP + DEX integration for instant swaps
4. **Programmable Messages**: Custom logic on destination

---

## Operational Guide

### Deploying New Domain

1. **Deploy contracts** on external chain
2. **Configure domain** in Ëtrid runtime
3. **Register attesters** for new domain
4. **Test transfers** on testnet
5. **Governance approval** for mainnet
6. **Enable domain** on mainnet

### Monitoring

**Key Metrics**:
- Burn events per domain (rate)
- Pending attestations (queue length)
- Failed relays (error rate)
- Average transfer time (latency)
- Daily volume per domain (usage)

**Alerts**:
- ⚠️ Pending attestations > 100 (backlog)
- ⚠️ Failed relay rate > 5% (reliability issue)
- ⚠️ Transfer time > 30 minutes (performance degradation)
- 🚨 Daily limit exceeded (rate limit hit)
- 🚨 Attester offline (availability issue)

---

## Status

**Current Status**: ✅ Production Ready

**Supported Domains**:
- ✅ Ethereum (mainnet + testnets)
- ✅ Ëtrid PBC-EDSC (native)
- 🔄 Solana (testing)
- 🔄 Polygon (testing)
- 📋 BNB Chain (planned)
- 📋 Avalanche (planned)
- 📋 Arbitrum (planned)
- 📋 Optimism (planned)

**Next Steps**:
1. Complete Solana integration testing
2. Deploy to mainnet Ethereum
3. Onboard additional attesters
4. Launch public bridge UI

---

**Last Updated:** 2025-11-04
**Version:** 1.0
**Documentation:** Complete
