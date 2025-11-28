# EDSC Cross-Chain Bridge

The EDSC (Ëtrid Stablecoin) bridge enables cross-chain transfers between Ëtrid and Ethereum (and other EVM chains) using a CCTP-style burn-and-mint architecture with M-of-N attestation.

## Architecture

```
EDSC Bridge
├── substrate-pallets/           Ëtrid/Substrate pallets
│   ├── pallet-edsc-token/       EDSC ERC-20 token on Substrate
│   ├── pallet-edsc-bridge-token-messenger/   Burn/mint operations
│   ├── pallet-edsc-bridge-attestation/       Signature validation
│   ├── pallet-edsc-receipts/    Cross-chain receipts
│   ├── pallet-edsc-redemption/  Token redemption
│   ├── pallet-edsc-oracle/      Price feeds
│   └── pallet-edsc-checkpoint/  State checkpointing
├── ethereum-contracts/ → ../../contracts/ethereum   (symlink)
│   ├── EDSC.sol                 EDSC token contract
│   ├── EDSCTokenMessenger.sol   Burn/mint controller
│   ├── EDSCMessageTransmitter.sol  Message handling
│   └── AttesterRegistry.sol     Attester management
└── services/ → ../../../services   (symlink)
    ├── attestation-service/     Off-chain attestation
    └── relayer-service/         Message relay
```

## Components

### Substrate Pallets

**pallet-edsc-token**
- ERC-20 compatible EDSC token on Substrate
- Minting and burning capabilities
- Cross-chain balance tracking

**pallet-edsc-bridge-token-messenger**
- Burn tokens on source chain
- Mint tokens on destination chain
- Nonce management

**pallet-edsc-bridge-attestation**
- M-of-N signature validation (3-of-5)
- Attester registry
- Message verification

**pallet-edsc-receipts**
- Cross-chain transfer receipts
- Proof of burn/mint

**pallet-edsc-redemption**
- Token redemption logic
- Reserve backing

**pallet-edsc-oracle**
- Price feeds for EDSC
- Multi-source aggregation

**pallet-edsc-checkpoint**
- State checkpointing
- Rollback protection

### Ethereum Contracts

**EDSC.sol**
- ERC-20 EDSC token
- Controlled minting (only MessageTransmitter)
- Emergency pause

**EDSCTokenMessenger.sol**
- `burnAndSendTo()` - Burn tokens and emit cross-chain message
- Domain-based routing (Ethereum=0, Solana=1, Ëtrid=2)

**EDSCMessageTransmitter.sol**
- `receiveMessage()` - Verify attestations and mint tokens
- M-of-N signature validation
- Nonce replay protection

**AttesterRegistry.sol**
- Attester management
- Signature threshold configuration
- Owner controls

### Services

**attestation-service** (TypeScript)
- Monitors burn events on both chains
- Signs cross-chain messages
- Threshold signature coordination
- Prometheus metrics

**relayer-service** (TypeScript)
- Polls for fully-attested messages
- Submits to destination chain
- Automatic retry logic
- Gas optimization

## Flow

### Ethereum → Ëtrid

1. User calls `burnAndSendTo()` on Ethereum
2. EDSC burned, `MessageSent` event emitted
3. 3 attestation services detect event and sign message
4. Relayer collects 3 signatures
5. Relayer calls `receiveMessage()` on Ëtrid
6. Substrate pallet verifies signatures and mints EDSC

### Ëtrid → Ethereum

1. User calls `burn_and_send()` extrinsic
2. EDSC burned, event emitted
3. 3 attestation services detect and sign
4. Relayer submits to Ethereum
5. MessageTransmitter verifies and mints

## Security

- **M-of-N Attestation**: Requires 3 out of 5 attesters to sign
- **Nonce Management**: Prevents replay attacks
- **Domain Separation**: Chain-specific message formats
- **Pause Controls**: Emergency shutdown capability
- **Owner Controls**: Restricted administrative functions

## Local Testing

See [DOCKER_SETUP_COMPLETE.md](../../../../DOCKER_SETUP_COMPLETE.md) for full Docker stack.

```bash
# Start entire bridge stack
docker-compose -f docker-compose.bridge.yml up

# Test transfer
cd contracts/ethereum
npx hardhat run scripts/test-transfer.js --network localhost
```

## Testnet Deployment

See [EMBER_DEPLOYMENT_CHECKLIST.md](../../../../EMBER_DEPLOYMENT_CHECKLIST.md)

## Differences from Other Bridges

Unlike the simpler protocol bridges in this directory (bitcoin-bridge, ethereum-bridge, etc.), the EDSC bridge is a full **native stablecoin bridge** with:

- Native token (EDSC) on both chains
- Attestation service infrastructure
- Complex multi-signature validation
- Production-grade monitoring

Other bridges are protocol adapters for external chains. EDSC is Ëtrid's native stablecoin.

## Related Documentation

- [EDSC Bridge Testing Results](../../../../EDSC_BRIDGE_TEST_RESULTS.md)
- [Docker Setup Guide](../../../../DOCKER_SETUP.md)
- [Deployment Plan](../../../../EMBER_DEPLOYMENT_PLAN.md)

## Status

✅ **Fully Functional**
- Local testing complete
- End-to-end transfers working
- Docker stack operational
- Ready for Ember testnet deployment
