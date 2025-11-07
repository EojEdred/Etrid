# Ëtrid Unified Contracts

> Multi-chain smart contract deployment system for the Ëtrid ecosystem

## Overview

This repository contains all smart contracts for Ëtrid's multi-chain infrastructure, including:

- **Token Contracts**: WrappedETR, EDSC stablecoin
- **Bridge Infrastructure**: TokenMessenger with 3-of-5 oracle attestation
- **DeFi Contracts**: MasterChef yield farming, bridge adapters
- **Multi-chain Deployment**: Automated deployment to 7+ EVM chains

## Architecture

```
FlareChain (Substrate) ← Native ETR
    ↕
ETH PBC (EVM) ← MasterChef + Bridge
    ↕
TokenMessenger (EDSC Bridge)
    ↕
External EVM Chains (7 chains)
```

## Quick Start

### 1. Installation

```bash
npm install
```

### 2. Configuration

Copy the example environment file and configure:

```bash
cp .env.example .env
# Edit .env with your private keys and RPC endpoints
```

### 3. Compile Contracts

```bash
npm run compile
```

### 4. Run Tests

```bash
npm test
```

### 5. Deploy to Local Node

```bash
# Terminal 1: Start local node
npm run node

# Terminal 2: Deploy contracts
npm run deploy:local
```

## Deployment

### Single Chain Deployment

Deploy to ETH PBC:
```bash
npm run deploy:eth-pbc
```

Deploy to Ethereum:
```bash
npm run deploy:ethereum
```

Deploy to BNB Chain:
```bash
npm run deploy:bsc
```

### Multi-Chain Deployment

Deploy to all testnets:
```bash
npm run deploy:all-testnets
```

Deploy to all mainnets (use with caution):
```bash
npm run deploy:all-mainnets
```

## Contract Addresses

After deployment, contract addresses are saved to `deployments/[network]-[chainId].json`

### ETH PBC
- WrappedETR: `0x...`
- EDSC: `0x...`
- TokenMessenger: `0x...`
- MasterChef: `0x...`
- BridgeAdapter: `0x...`

### Ethereum
- WrappedETR: `0x...`
- EDSC: `0x...`
- TokenMessenger: `0x...`

## Post-Deployment Configuration

### 1. Configure Oracle Network

```bash
npx hardhat run scripts/configure-oracles.js --network ethPBC
```

### 2. Verify Contracts

```bash
npm run verify:ethereum
npm run verify:bsc
```

### 3. Test Cross-Chain Transfers

```bash
npx hardhat test test/cross-chain.test.js --network ethPBC
```

## Contract Overview

### WrappedETR.sol
ERC20 wrapper for native ETR token with bridge minting/burning capabilities.

**Features:**
- Mint/burn for bridge operations
- Role-based access control
- Emergency pause functionality
- 10B max supply cap

### EDSC.sol
Ëtrid Decentralized Stablecoin for cross-chain transfers.

**Features:**
- Burn-and-mint bridge mechanism
- Daily mint rate limiting
- 3-of-5 oracle attestation
- 6 decimals (USDC compatible)

### TokenMessenger.sol
Cross-chain messaging protocol with M-of-N attestation.

**Features:**
- 3-of-5 multisig oracle network
- Replay attack prevention
- Hourly rate limiting per user
- Multiple token support

### MasterChef.sol
Yield farming contract for LP token staking.

**Features:**
- Multiple pool support
- Configurable reward rates
- Emergency withdraw
- Owner controls

### ETHPBCBridgeAdapter.sol
Bridge adapter connecting MasterChef rewards to FlareChain.

**Features:**
- Harvest and bridge in one transaction
- EDSC integration
- Attestation verification
- Nonce tracking

## Security

### Audit Status
- ⏳ Pending audit by [Auditor Name]

### Security Features
- Multi-sig admin controls (Gnosis Safe recommended)
- Emergency pause mechanisms
- Rate limiting
- Replay attack prevention
- 3-of-5 oracle attestation

### Bug Bounty
Contact: security@etrid.io

## Testing

Run the full test suite:
```bash
npm test
```

Run with coverage:
```bash
npm run test:coverage
```

Run gas reporter:
```bash
REPORT_GAS=true npm test
```

## Networks

### Testnets
- ETH PBC Testnet
- Ethereum Sepolia
- BNB Testnet
- Polygon Mumbai
- Arbitrum Sepolia
- Base Sepolia

### Mainnets
- ETH PBC
- Ethereum
- BNB Chain
- Polygon
- Arbitrum
- Base
- Optimism

## Scripts

- `npm run compile` - Compile all contracts
- `npm test` - Run tests
- `npm run deploy:local` - Deploy to local node
- `npm run deploy:eth-pbc` - Deploy to ETH PBC
- `npm run deploy:all-testnets` - Deploy to all testnets
- `npm run verify:ethereum` - Verify on Etherscan
- `npm run node` - Start local Hardhat node
- `npm run clean` - Clean artifacts

## Development

### Project Structure

```
unified-contracts/
├── contracts/
│   ├── tokens/              # Token contracts
│   │   ├── WrappedETR.sol
│   │   └── EDSC.sol
│   ├── bridges/             # Bridge contracts
│   │   ├── TokenMessenger.sol
│   │   └── ETHPBCBridgeAdapter.sol
│   └── defi/                # DeFi contracts
│       └── MasterChef.sol
├── scripts/                 # Deployment scripts
│   ├── deploy-all.js
│   ├── deploy-multi-chain.js
│   └── configure-oracles.js
├── test/                    # Tests
├── deployments/             # Deployment records
├── hardhat.config.ts        # Hardhat configuration
└── package.json
```

### Adding a New Chain

1. Add network config to `hardhat.config.ts`
2. Add to chain list in `scripts/deploy-multi-chain.js`
3. Set domain ID in `scripts/deploy-all.js`
4. Configure RPC in `.env`
5. Deploy and test

## Support

- Documentation: https://docs.etrid.io
- Discord: https://discord.gg/etrid
- GitHub: https://github.com/etrid/etrid

## License

Apache-2.0

## Contributors

Built by the Ëtrid team with ❤️
