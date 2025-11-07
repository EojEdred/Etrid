# Quick Start Guide

This guide will walk you through deploying Ëtrid contracts to ETH PBC in under 10 minutes.

## Prerequisites

- Node.js 18+ installed
- ETH PBC node running (or RPC endpoint)
- Private key with ETH PBC test tokens

## Step 1: Installation (2 minutes)

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/unified-contracts
npm install
```

## Step 2: Configuration (2 minutes)

Create `.env` file:

```bash
cp .env.example .env
```

Edit `.env` with your details:

```bash
# Required fields
DEPLOYER_PRIVATE_KEY=your_private_key_here
ETH_PBC_RPC=http://163.192.125.23:9944

# Optional: Oracle addresses (can use defaults for testing)
ORACLE_1=0xYourOracle1Address
ORACLE_2=0xYourOracle2Address
ORACLE_3=0xYourOracle3Address
ORACLE_4=0xYourOracle4Address
ORACLE_5=0xYourOracle5Address
```

## Step 3: Compile Contracts (1 minute)

```bash
npm run compile
```

Expected output:
```
Compiled 15 Solidity files successfully
```

## Step 4: Start ETH PBC Node (if local)

If running a local ETH PBC node:

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/eth-pbc-collator

./target/release/eth-pbc-collator \
  --dev \
  --tmp \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-external
```

If using the existing RPC node at `163.192.125.23:9944`, skip this step.

## Step 5: Deploy Contracts (3 minutes)

```bash
npm run deploy:eth-pbc
```

This will deploy:
1. WrappedETR token
2. EDSC stablecoin
3. TokenMessenger bridge
4. MasterChef farming
5. ETHPBCBridgeAdapter

Expected output:
```
╔════════════════════════════════════════════════════════════════╗
║        Ëtrid Unified Contract Deployment System                ║
╚════════════════════════════════════════════════════════════════╝

📊 Deployment Information:
  Network: ethPBC
  Chain ID: 42069
  Deployer: 0x...
  Balance: 100.0 ETH
  Domain ID: 2

🔷 Phase 1: Deploying Token Contracts

📝 Deploying WrappedETR...
  ✅ WrappedETR deployed to: 0x...

📝 Deploying EDSC...
  ✅ EDSC deployed to: 0x...

🌉 Phase 2: Deploying Bridge Infrastructure

📝 Deploying TokenMessenger...
  ✅ TokenMessenger deployed to: 0x...

💰 Phase 4: Deploying DeFi Contracts

📝 Deploying MasterChef...
  ✅ MasterChef deployed to: 0x...

📝 Deploying ETHPBCBridgeAdapter...
  ✅ ETHPBCBridgeAdapter deployed to: 0x...

🎉 Deployment completed successfully!
```

## Step 6: Configure Oracle Network (2 minutes)

```bash
npx hardhat run scripts/configure-oracles.js --network ethPBC
```

This adds the 5 oracle addresses to TokenMessenger for 3-of-5 multisig.

## Step 7: Verify Deployment

Check the deployment info:

```bash
cat deployments/ethPBC-42069.json
```

Should show all deployed contract addresses.

## Testing Your Deployment

### Test 1: Check Token Balances

```bash
npx hardhat console --network ethPBC
```

```javascript
const wrappedETR = await ethers.getContractAt("WrappedETR", "0xYourWrappedETRAddress");
await wrappedETR.name(); // Should return "Wrapped ETR"
await wrappedETR.symbol(); // Should return "wETR"
await wrappedETR.totalSupply(); // Should return 0
```

### Test 2: Check MasterChef

```javascript
const masterChef = await ethers.getContractAt("MasterChef", "0xYourMasterChefAddress");
await masterChef.rewardPerBlock(); // Should return 1000000000000000000 (1 ETR)
```

### Test 3: Check Oracle Configuration

```javascript
const tokenMessenger = await ethers.getContractAt("TokenMessenger", "0xYourTokenMessengerAddress");
await tokenMessenger.oracleCount(); // Should return 5
```

## Next Steps

### For Development
- Run tests: `npm test`
- Deploy to testnets: `npm run deploy:all-testnets`
- Add liquidity pools to MasterChef
- Test cross-chain transfers

### For Production
1. Deploy to all mainnets: `npm run deploy:all-mainnets`
2. Verify contracts on block explorers
3. Set up multi-sig admin (Gnosis Safe)
4. Configure production oracle nodes
5. Fund MasterChef with ETR rewards
6. Update frontend with contract addresses

## Troubleshooting

### Error: insufficient funds
Make sure your deployer address has enough ETH PBC tokens. You need at least 0.1 ETH for gas.

### Error: nonce too low
Clear your local nonce cache:
```bash
rm -rf cache/
npm run compile
```

### Error: cannot connect to node
Check that ETH PBC node is running:
```bash
curl -X POST http://163.192.125.23:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Error: contract verification failed
This is normal on ETH PBC since there's no block explorer yet. Verification only works on public chains like Ethereum/BSC.

## Support

- Documentation: See README.md
- Issues: Create an issue in the repo
- Discord: Join the Ëtrid community

## Summary

You've successfully deployed:
- ✅ WrappedETR token (bridge-compatible ERC20)
- ✅ EDSC stablecoin (cross-chain)
- ✅ TokenMessenger (bridge infrastructure)
- ✅ MasterChef (yield farming)
- ✅ BridgeAdapter (harvest & bridge)

Total deployment time: **~10 minutes**

Your contracts are now live on ETH PBC! 🎉
