# ETH PBC - EDSC Bridge Integration

**Date**: November 2, 2025
**Purpose**: Enable cross-chain reward transfers between ETH PBC and FlareChain via EDSC

---

## Overview

This integration allows users who stake LP tokens on ETH PBC's MasterChef contract to bridge their ETR rewards to FlareChain using the EDSC (Ëtrid Decentralized Stablecoin) burn-and-mint bridge mechanism.

### Architecture

```
ETH PBC (EVM Chain)          EDSC Bridge Network          FlareChain (Substrate)
┌─────────────────┐         ┌────────────────┐          ┌──────────────────┐
│                 │         │                │          │                  │
│  MasterChef     │         │  TokenMessenger│          │  EDSC Pallet     │
│  - Stake LP     │         │  - Burn EDSC   │          │  - Mint EDSC     │
│  - Earn ETR     │◄────────┤  - 3-of-5      │◄─────────┤  - Attestation   │
│                 │         │    Multisig    │          │                  │
│  BridgeAdapter  │         │                │          │                  │
│  - Bridge ETR   │────────►│  Oracle Network│─────────►│  Receive ETR     │
│                 │         │  - Validators  │          │                  │
└─────────────────┘         └────────────────┘          └──────────────────┘
```

### Components

1. **ETHPBCBridgeAdapter.sol** - Solidity contract for bridging rewards
2. **deploy-eth-pbc-bridge.ts** - Deployment script
3. **EDSC TokenMessenger** - Existing cross-chain messaging protocol
4. **EDSC Oracle Network** - 5 validators with 3-of-5 multisig attestation

---

## Smart Contract: ETHPBCBridgeAdapter

### Location
`/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/edsc-bridge/ethereum-contracts/ETHPBCBridgeAdapter.sol`

### Key Functions

#### User Functions

**bridgeRewards(uint256 amount, bytes32 destinationAddress)**
- Bridge ETR rewards from ETH PBC to FlareChain
- Burns EDSC equivalent on ETH PBC
- Mints EDSC on FlareChain at destination address
- Returns: `transferId` for tracking

**harvestAndBridge(uint256 poolId, bool bridgeToFlareChain, bytes32 destinationAddress)**
- Harvest MasterChef rewards
- Optionally bridge them to FlareChain in one transaction
- Convenience function for seamless UX

**receiveRewards(address recipient, uint256 amount, bytes32 transferId, bytes attestation)**
- Receive bridged rewards on ETH PBC (from FlareChain)
- Requires 3-of-5 multisig attestation
- Mints EDSC tokens to recipient

#### View Functions

**getNonce(address user)** - Get user's current nonce for bridge operations
**isTransferCompleted(bytes32 transferId)** - Check if transfer has been completed

### Events

- `RewardBridged` - Emitted when rewards are sent to FlareChain
- `RewardReceived` - Emitted when rewards arrive from FlareChain

---

## Deployment

### Prerequisites

```bash
# Set environment variables
export EDSC_TOKEN_ADDRESS="0x..."      # EDSC stablecoin on ETH PBC
export ETR_TOKEN_ADDRESS="0x..."       # ETR reward token on ETH PBC
export TOKEN_MESSENGER_ADDRESS="0x..." # EDSC TokenMessenger contract
export MASTERCHEF_ADDRESS="0x..."      # MasterChef LP staking contract
```

### Deploy

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/edsc-bridge/ethereum-contracts

# Deploy to ETH PBC
npx hardhat run deploy-eth-pbc-bridge.ts --network eth-pbc

# Deployment info will be saved to:
# deployments/eth-pbc-bridge-{chainId}.json
```

### Post-Deployment Steps

1. **Configure TokenMessenger**
   - Add BridgeAdapter as authorized caller
   - Register with EDSC oracle network

2. **Update Web App**
   - Add bridge adapter address to `.env`
   - Update ETH_PBC_WEB_INTEGRATION_HANDOFF.md with contract address

3. **Test Bridge Flow**
   - Stake LP tokens on MasterChef
   - Earn ETR rewards
   - Bridge rewards to FlareChain
   - Verify receipt on FlareChain

---

## Security Features

### 1. M-of-N Attestation (3-of-5 Multisig)
- Requires 3 signatures from 5 registered oracle validators
- Prevents single point of failure
- Signature verification via `ecrecover`

### 2. Reentrancy Protection
- All external calls protected with `nonReentrant` modifier
- Uses OpenZeppelin's ReentrancyGuard

### 3. Nonce System
- Each user has unique nonce counter
- Prevents replay attacks
- Transfer IDs include nonce + timestamp

### 4. Access Control
- Owner-only admin functions
- Immutable critical addresses (EDSC, ETR tokens)
- Role-based oracle registry

### 5. Transfer Completion Tracking
- Mapping prevents double-spending
- Once completed, transfer cannot be replayed

---

## Cross-Chain Flow

### ETH PBC → FlareChain

1. User has ETR rewards in wallet (from MasterChef)
2. User calls `bridgeRewards(amount, flareChainAddress)`
3. Contract transfers ETR from user
4. Contract calls TokenMessenger.depositForBurn()
5. EDSC burned on ETH PBC
6. Oracle network observes burn event
7. 3 of 5 oracles sign attestation
8. EDSC minted on FlareChain to destination address

### FlareChain → ETH PBC

1. User initiates transfer on FlareChain
2. EDSC burned on FlareChain
3. Oracle network observes burn event
4. 3 of 5 oracles create attestation
5. User (or relayer) calls `receiveRewards()` on ETH PBC
6. Contract verifies attestation signatures
7. EDSC minted on ETH PBC to recipient

---

## Integration with MasterChef

The bridge adapter integrates seamlessly with MasterChef:

```solidity
// Option 1: Harvest rewards on ETH PBC
MasterChef.harvest(poolId);

// Option 2: Harvest and bridge in one transaction
BridgeAdapter.harvestAndBridge(
    poolId,
    true,                    // bridge to FlareChain
    flareChainAddress        // destination
);
```

---

## Configuration

### Chain IDs

- **ETH PBC**: 8888 (update after genesis)
- **FlareChain Domain**: 1 (EDSC bridge domain ID)

### Contract Addresses

Update after deployment:
- **BridgeAdapter**: TBD
- **TokenMessenger**: TBD
- **EDSC Token**: TBD
- **ETR Token**: TBD
- **MasterChef**: TBD

---

## Testing

### Local Testing

```bash
# Start ETH PBC node
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/eth-pbc-collator
./target/release/eth-pbc-collator --dev --tmp

# Run tests
npx hardhat test test/ETHPBCBridgeAdapter.test.ts
```

### Integration Testing

1. **Deploy Contracts**
   - Deploy MasterChef
   - Deploy EDSC token
   - Deploy ETR token
   - Deploy TokenMessenger
   - Deploy BridgeAdapter

2. **Configure System**
   - Set up oracle network (5 validators)
   - Register bridge adapter with TokenMessenger
   - Create LP pool in MasterChef
   - Fund MasterChef with ETR rewards

3. **Test User Flow**
   - Stake LP tokens
   - Wait for rewards to accrue
   - Harvest rewards
   - Bridge to FlareChain
   - Verify receipt on FlareChain
   - Bridge back to ETH PBC
   - Verify receipt on ETH PBC

---

## Troubleshooting

### Common Issues

**"TransferFailed" error**
- Check TokenMessenger approval
- Verify EDSC token balance
- Ensure bridge adapter is registered as authorized caller

**"Unauthorized" error on receiveRewards**
- Verify attestation has 3 valid signatures
- Check oracle addresses are registered
- Ensure no duplicate signers in attestation

**"AlreadyCompleted" error**
- Transfer has already been processed
- Check `isTransferCompleted(transferId)`
- Use new transfer with fresh nonce

---

## Monitoring

### Events to Monitor

```javascript
// Bridge transactions outbound
BridgeAdapter.on("RewardBridged", (user, amount, transferId, nonce) => {
  console.log(`Bridge initiated: ${transferId}`)
  console.log(`User: ${user}, Amount: ${amount}`)
})

// Bridge transactions inbound
BridgeAdapter.on("RewardReceived", (user, amount, transferId) => {
  console.log(`Bridge completed: ${transferId}`)
  console.log(`Recipient: ${user}, Amount: ${amount}`)
})
```

### Metrics

- Total volume bridged (ETH PBC → FlareChain)
- Total volume bridged (FlareChain → ETH PBC)
- Average bridge time
- Failed transfers
- Oracle uptime

---

## Future Enhancements

1. **Automated Relayer** - Auto-relay attestations without user intervention
2. **Fee Optimization** - Dynamic fee adjustment based on network conditions
3. **Batch Transfers** - Multiple users in single attestation for gas savings
4. **Emergency Pause** - Circuit breaker for security incidents
5. **Multi-Token Support** - Bridge other tokens beyond ETR/EDSC

---

## References

- ETH PBC Collator: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/eth-pbc-collator`
- MasterChef Contract: `/Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/contracts/MasterChef.sol`
- EDSC Pallets: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/edsc-bridge/substrate-pallets/`
- Web Integration Guide: `/Users/macbook/Desktop/etrid/ETH_PBC_WEB_INTEGRATION_HANDOFF.md`

---

**Status**: Implementation Complete - Ready for Deployment Testing
**Next Step**: Deploy MasterChef and Bridge Adapter to ETH PBC testnet
