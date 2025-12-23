# Bridge Deployment Checklist

## Overview

This document outlines the deployment requirements for the Etrid cross-chain bridge system. The bridge supports two token types:

1. **ETR (Native Token)** - Lock/Unlock architecture via `pallet-etr-lock`
2. **EDSC (Stablecoin)** - Burn/Mint architecture via CCTP-style Token Messenger

---

## Architecture Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                    PRIMEARC CORE CHAIN                          │
├─────────────────────────────────────────────────────────────────┤
│  pallet-etr-lock           │  pallet-edsc-bridge-token-messenger │
│  (Lock ETR for bridging)   │  (Burn/Mint EDSC cross-chain)       │
│                            │                                      │
│  pallet-edsc-token         │  pallet-edsc-bridge-attestation      │
│  (EDSC token operations)   │  (M-of-N signature verification)     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ XCMP / DETR P2P
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    PARTITION BURST CHAINS                        │
│  ETH-PBC │ SOL-PBC │ BNB-PBC │ TRX-PBC │ XRP-PBC │ ...          │
│  (Bridge to specific external networks)                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Relayer / Watchtower
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    EXTERNAL CHAINS (EVM)                         │
├─────────────────────────────────────────────────────────────────┤
│  ETR_Ethereum.sol          │  EDSC.sol                           │
│  EDSC_Ethereum.sol         │  EDSCTokenMessenger.sol             │
│  WrappedETR.sol            │  EDSCMessageTransmitter.sol         │
│  EtridBridge.sol           │  AttesterRegistry.sol               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Domain IDs

| Domain | ID | Description |
|--------|----|-----------------------------|
| Ethereum | 0 | Ethereum mainnet |
| Solana | 1 | Solana mainnet |
| PrimearcCore | 2 | Etrid relay chain |
| Polygon | 3 | Polygon PoS |
| Arbitrum | 4 | Arbitrum One |
| BNBChain | 5 | BNB Smart Chain |
| Tron | 6 | Tron mainnet |
| XRP | 7 | XRP Ledger |
| Dogecoin | 8 | Dogecoin mainnet |
| Bitcoin | 9 | Bitcoin mainnet |

---

## Deployment Order

### Phase 1: Substrate Side (Already Complete)

- [x] `pallet-edsc-token` - EDSC token operations
- [x] `pallet-edsc-bridge-attestation` - Attester registry
- [x] `pallet-edsc-bridge-token-messenger` - Cross-chain messaging
- [x] `pallet-etr-lock` - ETR locking for bridges
- [x] Runtime configuration in `primearc-core-chain/runtime/src/lib.rs`

### Phase 2: EVM Contracts

#### Step 1: Deploy Token Contracts

| Contract | Network | Purpose |
|----------|---------|---------|
| `ETR_Ethereum.sol` | Ethereum | Wrapped ETR (ÉTR.e) |
| `EDSC.sol` | Ethereum | EDSC for MessageTransmitter |
| `EDSC_Ethereum.sol` | Ethereum | EDSC with AP support |
| `WrappedETR.sol` | BSC/Polygon/etc | wETR on other chains |

**Constructor Parameters:**
```solidity
// ETR_Ethereum
constructor(address admin, address bridge)

// EDSC
constructor(address _owner)

// EDSC_Ethereum
constructor(address admin, address bridge)

// WrappedETR
constructor(address admin)
```

#### Step 2: Deploy Infrastructure Contracts

| Contract | Network | Purpose |
|----------|---------|---------|
| `AttesterRegistry.sol` | Ethereum | Attester management |
| `EDSCMessageTransmitter.sol` | Ethereum | Receive messages from Primearc |
| `EDSCTokenMessenger.sol` | Ethereum | Send messages to Primearc |
| `EtridBridge.sol` | Ethereum | ETR bridge with watchtowers |

**Constructor Parameters:**
```solidity
// AttesterRegistry
constructor(address _owner, uint32 _minSignatures, uint32 _totalAttesters)
// Recommended: 3-of-5 threshold

// EDSCMessageTransmitter
constructor(address _owner, address _edscToken, address _attesterRegistry)

// EDSCTokenMessenger
constructor(address _owner, address _edscToken)

// EtridBridge
constructor(address admin, address _etrToken, address _edscToken, address[] initialWatchtowers)
```

#### Step 3: Post-Deployment Configuration

```solidity
// 1. Set MessageTransmitter on EDSC token
edsc.setMessageTransmitter(messageTransmitterAddress);

// 2. Add bridge role to WrappedETR
wrappedETR.addBridge(etridBridgeAddress);

// 3. Register attesters
attesterRegistry.registerAttester(attester1);
attesterRegistry.registerAttester(attester2);
// ... up to 5 attesters

// 4. Configure domain thresholds (optional, global is default)
attesterRegistry.configureThreshold(2, 3, 5); // Primearc domain: 3-of-5
```

---

## Configuration Parameters

### Primearc Side (Substrate)

```rust
// Token Messenger Config
parameter_types! {
    pub const PrimearcTokenMessengerMaxMessageBodySize: u32 = 512;
    pub const PrimearcTokenMessengerMaxBurnAmount: u128 = 1_000_000_000_000_000_000_000_000;  // 1M EDSC
    pub const PrimearcTokenMessengerDailyBurnCap: u128 = 10_000_000_000_000_000_000_000_000;  // 10M EDSC/day
    pub const PrimearcTokenMessengerMinBurnAmount: u128 = 1_000_000_000_000_000_000;  // 1 EDSC min
    pub const PrimearcTokenMessengerMessageTimeout: u32 = 1000;  // blocks
    pub const PrimearcTokenMessengerBlocksPerDay: u32 = 14_400;  // ~24h at 6s blocks
    pub const PrimearcLocalDomain: u32 = 2;  // Domain::PrimearcCore
}

// Attestation Config
parameter_types! {
    pub const PrimearcMaxAttesters: u32 = 10;
    pub const PrimearcMinThreshold: u32 = 3;  // 3-of-5 minimum
    pub const PrimearcAttestationTimeout: u32 = 1000;  // blocks
}
```

### EVM Side (Solidity)

```solidity
// EDSC Token Messenger
uint256 public maxBurnAmount = 1_000_000 ether;        // 1M EDSC per tx
uint256 public dailyBurnLimit = 10_000_000 ether;      // 10M EDSC per day
uint256 public constant BLOCKS_PER_DAY = 7200;         // ~24h at 12s blocks

// Etrid Bridge
uint256 public constant MAX_BRIDGE_PER_TX = 100_000 * 10**18;   // 100k ETR
uint256 public constant MAX_BRIDGE_PER_DAY = 1_000_000 * 10**18; // 1M ETR
uint256 public constant MIN_SIGNATURES = 3;
uint256 public constant MAX_WATCHTOWERS = 5;
uint256 public constant ATTESTATION_TIMEOUT = 15 minutes;

// ETR Token
uint256 public constant MAX_SUPPLY = 100_000_000_000 * 10**18;  // 100B ETR
uint256 public constant MAX_MINT_PER_TX = 100_000 * 10**18;     // 100k ETR
uint256 public constant MAX_MINT_PER_DAY = 1_000_000 * 10**18;  // 1M ETR
```

---

## Security Requirements

### Watchtower/Attester Setup

1. **Minimum 5 attesters** for production
2. **Geographically distributed** across different regions
3. **Different cloud providers** (AWS, GCP, Azure, self-hosted)
4. **Hardware security modules (HSM)** for key storage
5. **3-of-5 threshold** for message signing

### Rate Limits

| Parameter | Value | Purpose |
|-----------|-------|---------|
| Max per TX | 100k-1M tokens | Prevent single-tx exploits |
| Daily limit | 1M-10M tokens | Limit daily exposure |
| Attestation timeout | 15 min | Prevent stale attestations |
| Cooldown (large mints) | 6 hours | Slow down large operations |

### Emergency Controls

- **Pausable contracts** - All bridges can be paused
- **Circuit breaker** - Auto-pause on reserve ratio drop
- **Multisig admin** - 3-of-5 for admin operations

---

## Deployed Contract Addresses (Production)

| Chain | Contract | Address |
|-------|----------|---------|
| Solana | ETR SPL Token | CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4 |
| BSC | wETR BEP-20 | 0xcc9b37fed77a01329502f8844620577742eb0dc6 |
| Polygon | wETR ERC-20 | 0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb |
| Ethereum | ETR ERC-20 | 0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb |
| Arbitrum | ETR ERC-20 | 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3 |

---

## Files Modified in This Integration

### Substrate (Rust)

1. `/05-multichain/primearc-core-chain/runtime/src/lib.rs`
   - Added `EdscTokenOperations` trait implementation
   - Added `EdscBridgeAttestationVerifier` trait implementation
   - Configured `pallet_edsc_bridge_token_messenger`
   - Renamed `FlareMax*` to `PrimearcMax*` parameters

2. `/05-multichain/pallets-shared/pallet-token-messenger/src/lib.rs`
   - Renamed `FlareChain` to `PrimearcCore` in Domain enum

3. `/05-multichain/partition-burst-chains/pbc-chains/*/collator/src/service.rs`
   - Updated comments to reference "Primearc Core Chain"

### Solidity

1. `/contracts/ethereum/wrapped-etr/WrappedETR.sol`
   - Added `bridgeMint()` function for EtridBridge compatibility

---

## Testing Checklist

- [ ] Unit tests for pallet-edsc-bridge-token-messenger
- [ ] Unit tests for pallet-edsc-bridge-attestation
- [ ] Integration tests for Primearc ↔ PBC communication
- [ ] E2E test: Lock ETR on Primearc → Mint wETR on Ethereum
- [ ] E2E test: Burn wETR on Ethereum → Unlock ETR on Primearc
- [ ] E2E test: Burn EDSC on Primearc → Mint EDSC on Ethereum
- [ ] E2E test: Burn EDSC on Ethereum → Mint EDSC on Primearc
- [ ] Stress test: Rate limiting enforcement
- [ ] Security test: Invalid attestation rejection
- [ ] Security test: Replay attack prevention

---

## Monitoring & Alerts

### Key Metrics

1. `bridge_volume_total` - Total bridged volume per chain
2. `bridge_pending_messages` - Queue depth
3. `bridge_attestation_latency` - Time to collect signatures
4. `bridge_daily_volume` - 24h rolling volume
5. `bridge_reserve_ratio` - EDSC backing ratio

### Alert Thresholds

| Metric | Warning | Critical |
|--------|---------|----------|
| Volume spike | >200% of 24h avg | >500% of 24h avg |
| Queue depth | >50 pending | >200 pending |
| Attestation latency | >5 min | >10 min |
| Reserve ratio | <110% | <100% |

---

## Next Steps

1. **Deploy attesters** - Set up 5 watchtower nodes
2. **Configure relayers** - Deploy relayer service for each PBC
3. **Test on testnet** - Full E2E testing on Sepolia/Mumbai
4. **Security audit** - External audit of bridge contracts
5. **Mainnet deployment** - Staged rollout with low limits
6. **Gradual limit increase** - Raise limits after stability period
