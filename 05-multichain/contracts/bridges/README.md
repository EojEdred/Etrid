# ETRID Cross-Chain Bridge Contracts

Bridge contracts for cross-chain token transfers with 5-of-9 Director attestation.

## Architecture

```
┌──────────────┐     ┌───────────────┐     ┌──────────────┐
│   Source     │     │   Directors   │     │ Destination  │
│   Chain      │────▶│   (5 of 9)    │────▶│   Chain      │
└──────────────┘     └───────────────┘     └──────────────┘
       │                    │                     │
   Lock/Burn           Attestation           Mint/Unlock
```

## Bridge Flow

1. **Lock** - User locks tokens on source chain
2. **Observe** - Directors observe lock event
3. **Sign** - Each Director signs attestation message
4. **Collect** - Relayer collects 5+ signatures
5. **Claim** - User/relayer submits attestation to destination
6. **Mint/Unlock** - Tokens minted or unlocked on destination

## Contracts by Chain Type

### EVM Chains (Ethereum, Arbitrum, Polygon, Avalanche, Base, BSC, Optimism)

**Location:** `evm/`

**Contracts:**
- `EtridBridge.sol` - Main bridge contract
- `WrappedToken.sol` - Wrapped token template
- `WrappedTokenFactory.sol` - Factory for deploying wrapped tokens

**Deployment:**
```bash
cd evm/
npm install
npx hardhat compile
DEPLOYER_PRIVATE_KEY=0x... npx hardhat run scripts/deploy.ts --network ethereum
```

### Solana

**Location:** `solana/`

**Program:**
- `etrid_bridge` - Anchor program for lock/unlock/mint/burn

**Deployment:**
```bash
cd solana/
anchor build
anchor deploy --provider.cluster mainnet
```

### TRON

**Location:** `tron/`

**Contracts:**
- `TronBridge.sol` - TVM-compatible bridge contract

**Deployment:**
```bash
cd tron/
npm install -g tronbox
tronbox compile
tronbox migrate --network mainnet
```

## Domain IDs

Each chain has a unique domain ID for cross-chain messaging:

| Chain | Domain ID | Type |
|-------|-----------|------|
| Primearc Core Chain | 1 | Substrate |
| Ethereum | 100 | EVM |
| Arbitrum | 101 | EVM |
| Polygon | 102 | EVM |
| Avalanche | 103 | EVM |
| Base | 104 | EVM |
| BSC | 105 | EVM |
| Optimism | 106 | EVM |
| Solana | 200 | SVM |
| TRON | 195 | TVM |
| Bitcoin | 300 | UTXO |
| XRP | 301 | XRPL |
| Stellar | 302 | Stellar |

## PBC Domains

| PBC | Domain ID |
|-----|-----------|
| BTC-PBC | 10 |
| ETH-PBC | 11 |
| SOL-PBC | 12 |
| XRP-PBC | 13 |
| BNB-PBC | 14 |
| TRX-PBC | 15 |
| XLM-PBC | 16 |
| DOGE-PBC | 17 |
| ADA-PBC | 18 |
| LTC-PBC | 19 |
| SC-USDT-PBC | 20 |

## Attestation

All bridge operations require 5-of-9 Director attestation:

```solidity
// Message hash format
bytes32 messageHash = keccak256(abi.encodePacked(
    requestId,
    sourceDomain,
    destDomain,
    recipient,
    token,
    amount
));

// Verify attestation
attesterRegistry.verifyAttestation(messageHash, signatures);
```

## Security Features

- **5-of-9 Threshold**: No single point of failure
- **Nonce Protection**: Prevents replay attacks
- **Daily Limits**: Rate limiting per token
- **Pausable**: Emergency pause capability
- **Fee Mechanism**: Configurable bridge fees (max 5%)

## Integration with AttesterRegistry

Bridge contracts integrate with the AttesterRegistry deployed on each chain:

```solidity
interface IAttesterRegistry {
    function verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) external returns (bool);
}
```

See `../attestation/` for AttesterRegistry contracts.

## Token Support

### Native Tokens
- Lock native tokens (ETH, BNB, SOL, TRX, etc.)
- Receive wrapped version on destination

### Wrapped Tokens
- Burn wrapped tokens to bridge back
- Unlock original tokens on source chain

### Token Mappings
Each bridge maintains mappings between:
- Original token -> Wrapped token
- Source chain -> Destination chain

## Fees

Default bridge fee: **0.1%** (10 basis points)
Maximum allowed: **5%** (500 basis points)

Fees are:
- Deducted at lock time
- Sent to fee recipient address
- Configurable by admin

## Deployment Checklist

1. [ ] Deploy AttesterRegistry on chain
2. [ ] Register all 9 Directors
3. [ ] Deploy EtridBridge with AttesterRegistry address
4. [ ] Deploy WrappedTokenFactory
5. [ ] Create wrapped tokens for each supported asset
6. [ ] Add supported tokens to bridge
7. [ ] Set daily limits
8. [ ] Set fee recipient
9. [ ] Test end-to-end flow
