# EDSC Ethereum Contracts

Ethereum smart contracts for the EDSC CCTP-style bridge.

## Overview

This directory contains the Solidity smart contracts that enable cross-chain EDSC transfers between Ëtrid and Ethereum.

### Contracts

1. **EDSC.sol** - ERC-20 token contract for EDSC on Ethereum
2. **AttesterRegistry.sol** - Manages attesters and verifies M-of-N signatures
3. **EDSCMessageTransmitter.sol** - Receives messages from Ëtrid and mints EDSC
4. **EDSCTokenMessenger.sol** - Burns EDSC on Ethereum to send to Ëtrid

## Architecture

```
Ëtrid → Ethereum Flow:
1. User burns EDSC on Ëtrid via TokenMessenger pallet
2. Attesters sign the burn message
3. Relayer calls EDSCMessageTransmitter.receiveMessage()
4. Contract verifies signatures and mints EDSC

Ethereum → Ëtrid Flow:
1. User approves EDSC to EDSCTokenMessenger
2. User calls burnAndSend() with Ëtrid address
3. Contract burns EDSC and emits MessageSent event
4. Attesters sign the message
5. Relayer delivers to Ëtrid BridgeAttestation pallet
6. Ëtrid mints EDSC to recipient
```

## Installation

```bash
cd contracts/ethereum
npm install
```

## Compilation

```bash
npm run compile
```

## Testing

```bash
npm run test
```

## Deployment

### Local Network

```bash
# Terminal 1: Start local Hardhat node
npx hardhat node

# Terminal 2: Deploy contracts
npm run deploy:local
```

### Sepolia Testnet

1. Set up `.env` file:
```bash
cp .env.example .env
# Edit .env with your credentials
```

2. Deploy:
```bash
npm run deploy:sepolia
```

### Verification

After deployment, verify contracts on Etherscan:

```bash
npx hardhat verify --network sepolia <CONTRACT_ADDRESS> <CONSTRUCTOR_ARGS>
```

## Configuration

### Rate Limits

- **Max Burn Per Transaction**: 1,000,000 EDSC
- **Daily Burn Limit**: 10,000,000 EDSC

### Attestation Threshold

- **Default**: 3-of-5 signatures
- **Configurable** per domain via governance

## Security

### Audit Status

⚠️ **Not yet audited** - Do not use in production

### Key Security Features

- M-of-N threshold signature verification
- Nonce-based replay protection
- Rate limiting (per-tx and daily)
- Pausable contracts
- 2-step ownership transfer
- Domain validation

## Contract Addresses

### Sepolia Testnet

(To be deployed)

### Ethereum Mainnet

(To be deployed after audit)

## License

Apache-2.0
