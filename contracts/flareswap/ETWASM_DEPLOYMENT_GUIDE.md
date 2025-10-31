# FlareSwap Deployment to ËtwasmVM

Complete guide for deploying FlareSwap DEX to the Ëtrid blockchain using ËtwasmVM.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Understanding the Integration](#understanding-the-integration)
3. [Deployment Steps](#deployment-steps)
4. [Testing the DEX](#testing-the-dex)
5. [Troubleshooting](#troubleshooting)
6. [API Reference](#api-reference)

---

## Prerequisites

### 1. Running Ëtrid Node

You need a running Ëtrid blockchain node with ËtwasmVM pallet enabled:

```bash
# From the etrid root directory
cd /Users/macbook/Desktop/etrid

# Start the node in development mode
./target/release/flarechain-node --dev --tmp
```

**OR** connect to an existing network:
- **Testnet**: `wss://testnet.etrid.org`
- **Mainnet**: `wss://rpc.etrid.org`

### 2. Dependencies

Ensure all dependencies are installed:

```bash
cd /Users/macbook/Desktop/etrid/contracts/flareswap
npm install
```

### 3. Compiled Contracts

Contracts should already be compiled. To recompile:

```bash
npx hardhat compile
```

---

## Understanding the Integration

### ËtwasmVM Architecture

```
Solidity Code → EVM Bytecode → ËtwasmVM Interpreter → Blockchain State
```

ËtwasmVM provides **full EVM compatibility** on Substrate:

- **✅ 150+ EVM Opcodes** - Complete Ethereum opcode support
- **✅ Berlin/London Fork** - Latest EVM gas costs
- **✅ VMw Gas System** - 1 ÉTR = 1,000,000 VMw
- **✅ Persistent Storage** - Contract state in blockchain storage
- **✅ Stack Execution** - 1024-depth stack, 256-bit words

### FlareSwap Contracts

The deployment will install:

1. **WETH** - Wrapped ETR (native token wrapper)
2. **FlareSwapFactory** - Creates trading pairs
3. **FlareSwapRouter** - User-facing swap interface

---

## Deployment Steps

### Step 1: Prepare Deployment Environment

Set up your deployer account:

**For local development (using Alice):**
```bash
export NETWORK=local
# No mnemonic needed for local
```

**For testnet/mainnet:**
```bash
export NETWORK=testnet  # or mainnet
export DEPLOYER_MNEMONIC="your twelve word mnemonic phrase here"
```

### Step 2: Run Deployment Script

```bash
cd /Users/macbook/Desktop/etrid/contracts/flareswap
node scripts/deploy-etwasm.js --network=local
```

**Expected output:**

```
╔════════════════════════════════════════════╗
║  FlareSwap Deployment to ËtwasmVM         ║
╚════════════════════════════════════════════╝

Network: Local development node
Endpoint: ws://127.0.0.1:9944

🔗 Connecting to Ëtrid blockchain...
✓ Connected to chain: Development
✓ Node version: flarechain-1.0.0

👤 Using Alice account for deployment
✓ Deployer address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
💰 Deployer balance: 1.0000 METR

═══ Step 1: Deploy WETH ═══
📦 Deploying WETH...
  Bytecode size: 1234 bytes
  Deploying to ËtwasmVM...
  ✓ Included in block: 0x1234...
  ✓ Finalized in block: 0x1234...
  ✓ Contract deployed at: 5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX
  ✓ Code hash: 0xabcd...

═══ Step 2: Deploy FlareSwapFactory ═══
📦 Deploying FlareSwapFactory...
  ...

═══ Step 3: Deploy FlareSwapRouter ═══
📦 Deploying FlareSwapRouter...
  ...

╔════════════════════════════════════════════╗
║  🎉 DEPLOYMENT SUCCESSFUL!                ║
╚════════════════════════════════════════════╝

Contract Addresses:
  WETH:     5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX
  Factory:  5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
  Router:   5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty

Deployment info saved to: deployments-etwasm-local.json
```

### Step 3: Verify Deployment

Check the deployment info file:

```bash
cat deployments-etwasm-local.json
```

```json
{
  "network": "local",
  "chainName": "Development",
  "deployer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "timestamp": "2025-10-30T12:00:00.000Z",
  "contracts": {
    "weth": {
      "address": "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
      "tx": "0x1234..."
    },
    "factory": {
      "address": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
      "tx": "0x5678..."
    },
    "router": {
      "address": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "tx": "0x9abc..."
    }
  }
}
```

---

## Testing the DEX

### 1. Create a Trading Pair

```bash
# TODO: Create testing script
node scripts/test-dex.js --create-pair TOKEN_A TOKEN_B
```

### 2. Add Liquidity

```bash
node scripts/test-dex.js --add-liquidity TOKEN_A TOKEN_B 1000 1000
```

### 3. Execute Swap

```bash
node scripts/test-dex.js --swap TOKEN_A TOKEN_B 100
```

### 4. Check Via Polkadot.js Apps

Visit [https://polkadot.js.org/apps/](https://polkadot.js.org/apps/) and connect to your node:

1. **Settings** → **Developer** → Connect to `ws://localhost:9944`
2. **Developer** → **Extrinsics**
3. Select `etwasmVm` → `callContract`
4. Enter contract address and call data

---

## Troubleshooting

### Error: "Contract code exceeds maximum size"

**Solution**: The ËtwasmVM pallet has a max code size limit (default 1MB). If contracts exceed this:

1. Check `pallet-etwasm-vm` configuration in runtime
2. Increase `MaxCodeSize` parameter
3. Recompile runtime

### Error: "Connection refused to ws://127.0.0.1:9944"

**Solution**: Ensure the Ëtrid node is running:

```bash
cd /Users/macbook/Desktop/etrid
./target/release/flarechain-node --dev --tmp
```

### Error: "Gas limit exceeded"

**Solution**: Increase gas limit in deployment script:

```javascript
const DEPLOY_GAS_LIMIT = 50_000_000; // Increase to 50M VMw
```

### Error: "Invalid bytecode"

**Solution**: Ensure contracts are compiled with the correct Solidity version (0.8.20):

```bash
cd /Users/macbook/Desktop/etrid/contracts/flareswap
npx hardhat clean
npx hardhat compile
```

---

## API Reference

### Deploy Contract

**Extrinsic**: `etwasmVm.deployContract(code)`

```javascript
const tx = api.tx.etwasmVm.deployContract(bytecode);
await tx.signAndSend(signer);
```

**Parameters:**
- `code: Vec<u8>` - EVM bytecode (max 1MB)

**Events:**
- `etwasmVm.ContractDeployed(deployer, address, code_hash)`

### Call Contract

**Extrinsic**: `etwasmVm.callContract(address, data, gas_limit)`

```javascript
const tx = api.tx.etwasmVm.callContract(
  contractAddress,
  callData,
  10_000_000 // 10M VMw gas
);
await tx.signAndSend(signer);
```

**Parameters:**
- `contract_addr: AccountId` - Contract address
- `input_data: Vec<u8>` - ABI-encoded call data
- `gas_limit: Option<VMw>` - Gas limit (default: 10M)

**Events:**
- `etwasmVm.ContractExecuted(contract, gas_used, success)`
- `etwasmVm.ContractReverted(contract, reason, gas_used)`

---

## Next Steps

After deployment:

1. **Create Initial Pairs**
   - ÉTR/USDT
   - ÉTR/wBTC
   - ÉTR/wETH

2. **Add Liquidity Incentives**
   - Set up LP reward pools
   - Configure farming contracts

3. **Frontend Integration**
   - Connect UI to deployed contracts
   - Test all swap functions
   - Verify liquidity operations

4. **Bridge Integration**
   - Connect PBC bridges to FlareSwap
   - Enable cross-chain swaps
   - Test ÉTR ↔ PBC token swaps

---

## Security Notes

⚠️ **Important**:

- **Testnet First**: Always deploy to testnet before mainnet
- **Audit Contracts**: Get professional audit before mainnet deployment
- **Small Initial Liquidity**: Start with small amounts to test
- **Monitor Gas Usage**: Watch for unexpected gas consumption
- **Backup Mnemonics**: Keep deployer keys secure

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                  Ëtrid FlareChain                        │
│                                                           │
│  ┌────────────────────────────────────────────────────┐ │
│  │              ËtwasmVM Pallet                       │ │
│  │  - deploy_contract()                               │ │
│  │  - call_contract()                                 │ │
│  │  - EVM Interpreter (150+ opcodes)                  │ │
│  └────────────────────────────────────────────────────┘ │
│                          │                               │
│                          ↓                               │
│  ┌─────────────────┬──────────────────┬──────────────┐ │
│  │  WETH Contract  │ Factory Contract │    Router    │ │
│  │  (Wraps ÉTR)   │ (Creates Pairs)  │  (Swaps)     │ │
│  └─────────────────┴──────────────────┴──────────────┘ │
│                          │                               │
│                          ↓                               │
│  ┌────────────────────────────────────────────────────┐ │
│  │              Trading Pairs (Pools)                  │ │
│  │  - ÉTR/wBTC    - ÉTR/wETH    - ÉTR/USDT          │ │
│  │  - wBTC/wETH   - wSOL/USDT   - etc...             │ │
│  └────────────────────────────────────────────────────┘ │
│                          │                               │
└──────────────────────────┼───────────────────────────────┘
                           ↓
            ┌──────────────────────────────┐
            │   PBC Bridges (Cross-chain)  │
            │  - BTC  - ETH  - SOL  - BNB  │
            │  - TRX  - XRP  - etc...      │
            └──────────────────────────────┘
```

---

## Support

For issues or questions:
- **GitHub**: https://github.com/etrid/flareswap/issues
- **Discord**: https://discord.gg/etrid
- **Docs**: https://docs.etrid.org

---

**Deployment completed by**: Claude Code Assistant
**Date**: October 30, 2025
**Status**: ✅ Ready for deployment
