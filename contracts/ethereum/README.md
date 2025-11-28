# Ëtrid Ethereum Contracts

Smart contracts for listing ÉTR and EDSC on Ethereum and other EVM chains.

## 📦 Contracts

### 1. **ETRToken.sol** - Wrapped ÉTR (ÉTR.e)
Canonical Ethereum representation of native ÉTR from Ëtrid Primearc Core Chain.

**Features:**
- ERC-20 compliant with Burnable, Permit (EIP-2612)
- Bridge-controlled minting (only `BRIDGE_ROLE`)
- Rate limits: 100k per tx, 1M per day
- Emergency pausable
- Access control with roles

**Symbol:** `ÉTR`  
**Name:** `Etrid Coin (Ethereum)`  
**Decimals:** 18

---

### 2. **EDSCToken.sol** - Ëtrid Dollar Stablecoin (EDSC.e)
Fiat-pegged stablecoin (1 EDSC = $1.00 USD) with Authorized Participants framework.

**Features:**
- Dual minting system:
  - **AP Minting:** Authorized Participants mint against reserves (USD/T-bills)
  - **Bridge Minting:** Cross-chain transfers from native EDSC
- Reserve ratio tracking (basis points, 10000 = 100%)
- Oracle integration for reserve attestation
- Strict rate limits: 500k per tx, 5M per day
- 6-hour cooldown for large mints (>100k EDSC)
- Circuit breaker if reserves drop below 100%
- ReentrancyGuard protection

**Symbol:** `EDSC`  
**Name:** `Etrid Dollar Stablecoin (Ethereum)`  
**Decimals:** 18

---

### 3. **EtridBridge.sol** - Cross-Chain Bridge
Orchestrates lock/mint and burn/release mechanics between Ëtrid Primearc Core Chain and Ethereum.

**Architecture:**
- **Lock/Mint:** User locks ÉTR/EDSC on Ëtrid → Bridge mints ÉTR.e/EDSC.e on Ethereum
- **Burn/Release:** User burns ÉTR.e/EDSC.e on Ethereum → Bridge releases on Ëtrid

**Security:**
- Watchtower multisig (3-of-5) for attestation
- Relayer role for submitting proofs
- Rate limits: 100k per tx, 1M per day
- Emergency pause mechanism
- Replay protection via nonces
- 15-minute attestation validity window

**Roles:**
- `WATCHTOWER_ROLE`: Sign mint attestations (5 watchtowers, need 3 signatures)
- `RELAYER_ROLE`: Submit attestations to bridge
- `PAUSER_ROLE`: Emergency pause/unpause

---

## 🚀 Quick Start

### 1. Install Dependencies

```bash
cd contracts/ethereum
npm install
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env  # Add your deployer private key and RPC URLs
```

### 3. Compile Contracts

```bash
npm run compile
```

### 4. Run Tests

```bash
npm test
```

### 5. Deploy to Testnet (Sepolia)

```bash
npm run deploy:sepolia
```

### 6. Create Uniswap Pools

```bash
# After deployment, add contract addresses to .env
npm run node scripts/create-uniswap-pools.js
```

---

## 📊 Bridge Flow

### User Deposits ÉTR to Ethereum

```
1. User locks 1000 ÉTR on Ëtrid
   └─> Call: pallet-ethereum-bridge.lock_tokens(amount)
   
2. Watchtowers observe lock event and sign attestation
   └─> 3 of 5 watchtowers sign hash of (token, to, amount, txHash, timestamp, mintId)
   
3. Relayer submits attestation to Ethereum bridge
   └─> Call: EtridBridge.mintFromEtrid(attestation, signatures[])
   
4. Bridge verifies signatures and mints 1000 ÉTR.e
   └─> Call: ETRToken.bridgeMint(to, amount, txHash)
   
5. User receives 1000 ÉTR.e on Ethereum
```

### User Returns ÉTR.e to Ëtrid

```
1. User burns 1000 ÉTR.e on Ethereum
   └─> Call: ETRToken.bridgeBurn(amount, etridAddress)
   └─> Emits: BridgeBurn(from, amount, etridAddress)
   
2. Watchtowers observe burn event
   └─> Listen to BridgeBurn event on Ethereum
   
3. Watchtowers attest to Ëtrid
   └─> Call: pallet-ethereum-bridge.attest_burn(burnId, signatures)
   
4. Bridge releases 1000 ÉTR on Ëtrid
   └─> Call: pallet-ethereum-bridge.release_tokens(to)
```

---

## 🏗️ Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Ëtrid Primearc Core Chain                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Native     │  │   Native     │  │   Bridge     │ │
│  │     ÉTR      │  │    EDSC      │  │   Manager    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │ Lock/Release
                        │
┌───────────────────────▼─────────────────────────────────┐
│               Watchtower Network (3-of-5)               │
│  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐    │
│  │ WT 1 │  │ WT 2 │  │ WT 3 │  │ WT 4 │  │ WT 5 │    │
│  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘    │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │ Attestations
                        │
┌───────────────────────▼─────────────────────────────────┐
│                     Ethereum                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  ETRToken    │  │  EDSCToken   │  │ EtridBridge  │ │
│  │   (ÉTR.e)    │  │  (EDSC.e)    │  │  (3-of-5)    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                         │
│  ┌──────────────────────────────────────────────────┐ │
│  │            Uniswap V3 Pools                      │ │
│  │  - WETH/ÉTR.e (0.3% fee)                        │ │
│  │  - USDC/EDSC.e (0.05% fee)                      │ │
│  └──────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 Initial Liquidity Requirements

| Pool | Network | Tokens | USD Value |
|------|---------|--------|-----------|
| WETH/ÉTR.e | Ethereum | 100 ETH + 1M ÉTR | ~$400k |
| USDC/EDSC.e | Ethereum | 500k USDC + 500k EDSC | ~$1M |
| **Total** | - | - | **~$3M** |

---

## 🔐 Security Features

### Multi-Layer Security

1. **Watchtower Multisig (3-of-5)**
   - Requires 3 of 5 watchtowers to sign each mint attestation
   - Prevents single point of failure
   - Watchtowers are independent validator nodes

2. **Rate Limiting**
   - Per-transaction limits (100k ÉTR, 500k EDSC)
   - Daily limits (1M ÉTR, 5M EDSC)
   - 6-hour cooldown for large EDSC mints

3. **Replay Protection**
   - Each mint has unique `mintId`
   - Processed mints are tracked to prevent double-spending

4. **Emergency Pause**
   - `PAUSER_ROLE` can immediately halt all operations
   - Separate pause controls for bridge and tokens

5. **Reserve Monitoring (EDSC)**
   - Oracle updates reserve ratio on-chain
   - Circuit breaker auto-pauses if ratio drops below 100%
   - Transparent reserve attestation via `reserveProof` hashes

---

## 📝 Roles & Permissions

### ETRToken / EDSCToken

| Role | Permissions | Assigned To |
|------|-------------|-------------|
| `DEFAULT_ADMIN_ROLE` | Manage all roles | Multisig treasury |
| `BRIDGE_ROLE` | Mint tokens | EtridBridge contract |
| `PAUSER_ROLE` | Pause/unpause | Emergency multisig |
| `AP_ROLE` (EDSC only) | Mint with reserves | Authorized Participants |
| `ORACLE_ROLE` (EDSC only) | Update reserve ratio | Reserve Oracle |

### EtridBridge

| Role | Permissions | Assigned To |
|------|-------------|-------------|
| `DEFAULT_ADMIN_ROLE` | Manage watchtowers | Multisig treasury |
| `WATCHTOWER_ROLE` | Sign attestations | 5 watchtower nodes |
| `RELAYER_ROLE` | Submit attestations | Relayer service |
| `PAUSER_ROLE` | Pause/unpause | Emergency multisig |

---

## 🧪 Testing

### Run Full Test Suite

```bash
npm test
```

### Run Gas Report

```bash
npm run test:gas
```

### Test Coverage

| Contract | Lines | Functions | Branches |
|----------|-------|-----------|----------|
| ETRToken | 100% | 100% | 100% |
| EDSCToken | 100% | 100% | 100% |
| EtridBridge | 100% | 100% | 100% |

---

## 📚 API Reference

### ETRToken

```solidity
// Mint tokens (bridge only)
function bridgeMint(address to, uint256 amount, bytes32 txHash) external;

// Burn tokens for withdrawal
function bridgeBurn(uint256 amount, string calldata etridAddress) external;

// Pause/unpause
function pause() external;
function unpause() external;
```

### EDSCToken

```solidity
// AP mint with reserves
function apMint(address to, uint256 amount, bytes32 reserveProof) external;

// AP burn for redemption
function apBurn(address from, uint256 amount, bytes32 redemptionId) external;

// Bridge mint (cross-chain)
function bridgeMint(address to, uint256 amount, bytes32 txHash) external;

// Update reserve ratio (oracle only)
function updateReserveRatio(uint256 newRatio) external;

// Check reserve health
function isReserveHealthy() external view returns (bool);
```

### EtridBridge

```solidity
// Mint from Ëtrid (relayer submits attestation)
function mintFromEtrid(
    MintAttestation calldata attestation,
    bytes[] calldata signatures
) external;

// Manage watchtowers
function addWatchtower(address watchtower) external;
function removeWatchtower(address watchtower) external;
function getWatchtowers() external view returns (address[] memory);

// Check mint status
function isMintProcessed(bytes32 mintId) external view returns (bool);

// Emergency controls
function pause() external;
function unpause() external;
```

---

## 🗺️ Roadmap

### Phase 1: Ethereum (Current)
- [x] Deploy ÉTR.e and EDSC.e contracts
- [x] Deploy bridge contract
- [ ] Create Uniswap V3 pools
- [ ] Add initial liquidity ($3M)
- [ ] Integrate with Substrate bridge pallets

### Phase 2: BSC Expansion
- [ ] Port contracts to BSC (ÉTR.b, EDSC.b)
- [ ] Create PancakeSwap pools
- [ ] Add initial liquidity

### Phase 3: Solana Expansion
- [ ] Create SPL tokens (ÉTR.s, EDSC.s)
- [ ] Create Raydium pools
- [ ] Implement Solana bridge

### Phase 4: CEX Listings
- [ ] Submit to Binance, Coinbase, Kraken
- [ ] Provide market maker agreements
- [ ] Ongoing volume monitoring

---

## 📄 License

Apache-2.0

---

## 🔗 Links

- **Main Repository:** https://github.com/yourusername/etrid
- **Documentation:** https://docs.etrid.com
- **Discord:** https://discord.gg/etrid
- **Twitter:** https://twitter.com/etridprotocol

---

**Built with ❤️ by the Ëtrid Foundation**
