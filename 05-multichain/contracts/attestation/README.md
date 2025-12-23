# ËTRID Attestation Contracts

9-Director attestation system for cross-chain bridge operations.

## Overview

The attestation system uses **5-of-9** threshold signatures from the 9 Directors to validate cross-chain bridge requests.

## Contracts by Chain

### EVM Chains
All EVM-compatible chains use the same `AttesterRegistry.sol` contract:

| Chain | Chain ID | Status |
|-------|----------|--------|
| Ethereum | 1 | Ready to deploy |
| Arbitrum | 42161 | Ready to deploy |
| Polygon | 137 | Ready to deploy |
| Avalanche | 43114 | Ready to deploy |
| Base | 8453 | **Deployed** |
| BSC | 56 | **Deployed** |
| Optimism | 10 | Ready to deploy |

**Deployment:**
```bash
cd evm/
npm install
npx hardhat compile
DEPLOYER_PRIVATE_KEY=0x... npx hardhat run scripts/deploy-all-networks.ts --network ethereum
```

### Solana
Anchor program for Solana attestation.

**Location:** `solana/`

**Deployment:**
```bash
cd solana/
anchor build
anchor deploy --provider.cluster mainnet
```

### TRON
TVM-compatible contract for TRON network.

**Location:** `tron/`

Uses TronBox for deployment.

### Non-EVM Chains (Off-chain Attestation)

These chains use off-chain attestation with multisig:

| Chain | Type | Config File |
|-------|------|-------------|
| Bitcoin | P2WSH Multisig | `bitcoin/config.json` |
| XRP | XRPL MultiSign | `xrp/config.json` |
| Stellar | Stellar Multisig | `stellar/config.json` |

## 9 Directors

Each Director has an ECDSA compressed public key (33 bytes) that is registered on all chains:

| Director | Label |
|----------|-------|
| 1 | Gizzi (AI Overseer) |
| 2 | EojEdred (Founder) |
| 3-9 | Validators 3-9 |

## Attestation Flow

1. User initiates bridge request on source chain
2. Relayer detects request and broadcasts to Directors
3. Each Director signs the bridge request message
4. Signatures collected (need 5+ of 9)
5. Attestation submitted to destination chain
6. Tokens minted/unlocked on destination

## Security

- 5-of-9 threshold prevents single point of failure
- ECDSA signatures verified on-chain
- Nonce-based replay protection
- Processed attestation tracking
