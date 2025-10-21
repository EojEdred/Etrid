# Phase 3 - CCTP-Style Bridge Protocol for EDSC

**Date**: 2025-10-20
**Status**: 🔄 **PLANNING & DESIGN**
**Objective**: Enable EDSC cross-chain transfers to external blockchains (Ethereum, Solana, Bitcoin, etc.)

---

## 🎯 Phase 3 Overview

### Goal
Implement a Circle CCTP (Cross-Chain Transfer Protocol) style bridge that allows EDSC to be transferred between Ëtrid's PBC-EDSC chain and external blockchains like Ethereum, Solana, Polygon, BNB Chain, etc.

### Key Principles (Following CCTP Model)

1. **Burn-and-Mint Architecture**
   - Burn EDSC on source chain
   - Mint equivalent EDSC on destination chain
   - No wrapped tokens (native EDSC on all chains)

2. **Attestation-Based Security**
   - Multiple independent attesters verify burn transactions
   - Threshold signature scheme (M-of-N multisig)
   - Off-chain attestation service
   - On-chain verification

3. **Permissionless Relaying**
   - Anyone can relay attestations
   - Relayers compete for speed
   - No relayer trust required

4. **Nonce-Based Ordering**
   - Sequential message nonces prevent replay attacks
   - Per-source-domain nonce tracking
   - Idempotent message processing

---

## 🏗️ Architecture Components

### 1. Core Pallets (Substrate/Ëtrid Side)

#### A. pallet-edsc-bridge-token-messenger
**Purpose**: Main interface for cross-chain EDSC transfers

**Key Functions:**
- `burn_edsc_for_external_chain()` - User burns EDSC, gets attestation request
- `receive_and_mint()` - Receives attestation, mints EDSC on Ëtrid
- Message format creation and validation
- Nonce management
- Domain registry (Ethereum = 0, Solana = 1, etc.)

**Storage:**
- `BurnMessages`: Map nonce → BurnMessage
- `MessageNonce`: Per-domain nonce counter
- `UsedNonces`: Processed message tracking
- `DomainConfigs`: Supported external chains
- `MaxBurnAmount`: Per-transaction limits
- `DailyBurnVolume`: Rate limiting

#### B. pallet-edsc-bridge-attestation
**Purpose**: Manages attestation verification and attester registry

**Key Functions:**
- `register_attester()` - Governance adds new attesters
- `remove_attester()` - Governance removes compromised attesters
- `verify_attestation()` - Validates attester signatures
- `set_threshold()` - Update M-of-N threshold
- Signature aggregation and verification

**Storage:**
- `Attesters`: Registered attester public keys
- `AttestationThreshold`: M-of-N (e.g., 3-of-5)
- `AttesterStatus`: Active/Inactive
- `AttestationSignatures`: Collected signatures per message
- `AttesterReputation`: Performance tracking

#### C. pallet-edsc-bridge-transmitter
**Purpose**: Message transmission to external chains (optional - could be off-chain)

**Key Functions:**
- `queue_outbound_message()` - Queue for relayers
- `mark_message_relayed()` - Track delivery
- Relayer incentives and tracking

**Storage:**
- `OutboundQueue`: Messages awaiting relay
- `RelayerRewards`: Fee distribution
- `MessageDeliveryStatus`: Tracking

### 2. External Chain Components

#### A. Smart Contracts (Ethereum/EVM Chains)

**TokenMessenger.sol**
```solidity
contract EDSCTokenMessenger {
    // Burn EDSC on Ethereum, create attestation request
    function burnEDSC(
        uint256 amount,
        uint32 destinationDomain,  // Ëtrid = 2
        bytes32 mintRecipient,     // Substrate account
        bytes32 burnToken          // EDSC token address
    ) external returns (uint64 nonce);

    // Mint EDSC on Ethereum from Ëtrid
    function receiveMessage(
        bytes calldata message,
        bytes calldata attestation
    ) external returns (bool success);
}
```

**MessageTransmitter.sol**
```solidity
contract EDSCMessageTransmitter {
    // Verify attestation signatures
    function verifyAttestation(
        bytes calldata message,
        bytes calldata attestation
    ) internal returns (bool);

    // Execute minting after verification
    function _handleReceiveMessage(
        uint32 sourceDomain,
        bytes32 sender,
        bytes calldata messageBody
    ) internal;
}
```

**EDSC.sol (ERC-20)**
```solidity
contract EDSC is ERC20Burnable {
    // Standard ERC-20 with burn/mint controlled by TokenMessenger
    function mint(address to, uint256 amount) external onlyMinter;
    function burn(uint256 amount) external;
}
```

#### B. Solana Programs

**edsc_token_messenger.rs** (Solana Program)
- Similar burn/mint logic using SPL tokens
- Cross-Program Invocation (CPI) to Token Program
- PDA-based authority

#### C. Bitcoin Integration

**Lightning Network HTLC-based approach**
- EDSC ↔ BTC atomic swaps
- Time-locked contracts
- Hash verification

---

## 📋 Message Format

### Cross-Chain Message Structure

```rust
/// Universal message format for EDSC cross-chain transfers
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct CrossChainMessage {
    /// Version for protocol upgrades
    pub version: u32,

    /// Source domain identifier (0=Ethereum, 1=Solana, 2=Ëtrid, etc.)
    pub source_domain: u32,

    /// Destination domain identifier
    pub destination_domain: u32,

    /// Unique nonce (per source domain)
    pub nonce: u64,

    /// Message sender (Substrate account or external address bytes)
    pub sender: BoundedVec<u8, ConstU32<64>>,

    /// Message recipient
    pub recipient: BoundedVec<u8, ConstU32<64>>,

    /// Destination token address (for external chains)
    pub destination_caller: BoundedVec<u8, ConstU32<64>>,

    /// Message body (amount, token address, etc.)
    pub message_body: BoundedVec<u8, ConstU32<512>>,
}

/// Message body for EDSC burn/mint
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct BurnMessage {
    /// Version
    pub version: u32,

    /// Token address being burned (EDSC contract on source chain)
    pub burn_token: BoundedVec<u8, ConstU32<64>>,

    /// Mint recipient on destination chain
    pub mint_recipient: BoundedVec<u8, ConstU32<64>>,

    /// Amount to burn/mint (with 18 decimals)
    pub amount: u128,

    /// Message sender
    pub message_sender: BoundedVec<u8, ConstU32<64>>,
}
```

### Attestation Format

```rust
/// Attestation signed by attesters
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Attestation {
    /// Hash of the message being attested
    pub message_hash: H256,

    /// Aggregated signatures (or individual signatures)
    pub signatures: Vec<AttesterSignature>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct AttesterSignature {
    /// Attester's public key
    pub attester: AccountId,

    /// Signature over message hash
    pub signature: BoundedVec<u8, ConstU32<65>>,  // ECDSA or sr25519
}
```

---

## 🔄 Complete Flow Examples

### Example 1: Ethereum → Ëtrid (Bring EDSC to native chain)

```
User on Ethereum:

1. User calls EDSCTokenMessenger.burnEDSC(
     amount: 1000 EDSC,
     destinationDomain: 2,  // Ëtrid
     mintRecipient: 0x5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB  // Substrate account
   )

2. Ethereum contract:
   - Burns 1000 EDSC from user's balance
   - Emits MessageSent event with nonce
   - Stores message in contract

3. Off-chain Attesters (5 independent nodes):
   - Monitor Ethereum for MessageSent events
   - Verify burn transaction finality (12+ confirmations)
   - Each attester signs the message hash
   - Submit signatures to attestation API

4. Attestation Service:
   - Collects signatures from attesters
   - Once threshold reached (3-of-5), creates attestation
   - Makes attestation available via API

5. Relayer (permissionless):
   - Fetches message + attestation from API
   - Calls Ëtrid: pallet_edsc_bridge_token_messenger::receive_and_mint()
   - Pays gas on Ëtrid, gets small fee

6. Ëtrid pallet_edsc_bridge_token_messenger:
   - Verify attestation signatures (3-of-5 valid)
   - Check nonce not already used
   - Verify message format
   - Call pallet_edsc_token::mint(recipient, 1000 EDSC)
   - Mark nonce as used
   - Emit MessageReceived event

Result: User receives 1000 EDSC on Ëtrid native chain
```

### Example 2: Ëtrid → Ethereum (Send EDSC to Ethereum)

```
User on Ëtrid:

1. User calls pallet_edsc_bridge_token_messenger::burn_edsc_for_external_chain(
     amount: 500 EDSC,
     destination_domain: 0,  // Ethereum
     recipient: 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb  // Ethereum address
   )

2. Ëtrid pallet_edsc_bridge_token_messenger:
   - Verify user has 500 EDSC balance
   - Check daily burn limits not exceeded
   - Call pallet_edsc_token::burn(user, 500 EDSC)
   - Create CrossChainMessage with nonce
   - Store message in BurnMessages
   - Emit BurnMessageSent event

3. Off-chain Attesters:
   - Monitor Ëtrid for BurnMessageSent events
   - Verify block finality
   - Each attester signs the message hash
   - Submit signatures to attestation API

4. Attestation Service:
   - Collects signatures
   - Creates attestation once threshold reached
   - Makes available via API

5. Relayer:
   - Fetches message + attestation
   - Calls Ethereum: EDSCMessageTransmitter.receiveMessage(message, attestation)
   - Pays gas on Ethereum

6. Ethereum EDSCMessageTransmitter:
   - Verify attestation (3-of-5 signatures valid)
   - Check nonce not used
   - Parse message body
   - Call EDSC.mint(recipient, 500 EDSC)
   - Mark nonce as used
   - Emit MessageReceived event

Result: User receives 500 EDSC on Ethereum
```

---

## 🔐 Security Model

### Attestation Security

**Threat: Compromised Attester**
- Mitigation: M-of-N threshold (3-of-5)
- Single compromised attester cannot forge attestations

**Threat: Replay Attacks**
- Mitigation: Nonce tracking on both chains
- Each message can only be processed once

**Threat: Message Tampering**
- Mitigation: Cryptographic signatures over message hash
- Any modification invalidates signatures

**Threat: Front-Running**
- Mitigation: Permissionless relaying means anyone can relay
- No single relayer monopoly

### Rate Limiting

**Per-Transaction Limits**
```rust
parameter_types! {
    pub const MaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC
}
```

**Daily Volume Limits**
```rust
parameter_types! {
    pub const DailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000;  // 10M EDSC per day
}
```

**Emergency Pause**
- Governance can pause bridge operations
- Circuit breaker integration

---

## 📊 Domain Registry

### Supported Chains (Initial)

| Domain ID | Chain | Token Standard | Status |
|-----------|-------|----------------|--------|
| 0 | Ethereum | ERC-20 | Planned |
| 1 | Solana | SPL Token | Planned |
| 2 | Ëtrid (PBC-EDSC) | Native | Live |
| 3 | Polygon | ERC-20 | Planned |
| 4 | BNB Chain | BEP-20 | Planned |
| 5 | Avalanche | ERC-20 | Planned |
| 6 | Arbitrum | ERC-20 | Planned |
| 7 | Optimism | ERC-20 | Planned |

---

## 🎯 Phase 3 Development Roadmap

### Phase 3.1: Core Substrate Pallets (Week 1)
- [ ] Build pallet-edsc-bridge-token-messenger
- [ ] Build pallet-edsc-bridge-attestation
- [ ] Implement message encoding/decoding
- [ ] Add to workspace and verify compilation

### Phase 3.2: Runtime Integration (Week 1)
- [ ] Add pallets to PBC-EDSC runtime
- [ ] Configure parameters
- [ ] Verify compilation

### Phase 3.3: Smart Contract Development (Week 2)
- [ ] Write EDSCTokenMessenger.sol (Ethereum)
- [ ] Write EDSCMessageTransmitter.sol
- [ ] Write EDSC.sol (ERC-20 with burn/mint)
- [ ] Unit tests with Hardhat

### Phase 3.4: Attestation Service (Week 2-3)
- [ ] Off-chain attester node (Rust/TypeScript)
- [ ] Event monitoring (Ethereum, Ëtrid)
- [ ] Signature generation and aggregation
- [ ] REST API for attestation retrieval

### Phase 3.5: Relayer Service (Week 3)
- [ ] Permissionless relayer implementation
- [ ] Message fetching from attestation API
- [ ] Transaction submission to both chains
- [ ] Fee optimization

### Phase 3.6: Testing & Audit (Week 4)
- [ ] Integration tests (testnet)
- [ ] Security audit
- [ ] Stress testing
- [ ] Documentation

---

## 💡 Implementation Priorities

### Immediate (This Session)
1. ✅ Create Phase 3 plan
2. Build pallet-edsc-bridge-token-messenger skeleton
3. Build pallet-edsc-bridge-attestation skeleton
4. Define message formats
5. Add to workspace

### Short-Term (Next Session)
6. Implement burn/mint logic
7. Implement signature verification
8. Runtime integration
9. Basic testing

### Medium-Term
10. Ethereum smart contracts
11. Attestation service
12. Relayer service
13. Testnet deployment

---

## 📚 References

### Circle CCTP Documentation
- Architecture: https://developers.circle.com/stablecoins/docs/cctp-protocol-contract
- Message Format: https://developers.circle.com/stablecoins/docs/message-format
- Attestation: https://developers.circle.com/stablecoins/docs/cctp-attestation-service

### Similar Implementations
- LayerZero: Cross-chain messaging protocol
- Wormhole: Guardian network attestation
- Axelar: Cross-chain gateway protocol

---

**Phase 3 Plan Status**: ✅ COMPLETE
**Next Step**: Begin implementation of pallet-edsc-bridge-token-messenger

---

**END OF PHASE 3 PLANNING DOCUMENT**
