# FlareChain Bridge Deployment - Complete Handoff Document

**Date**: 2025-11-01
**Status**: Ready for ETR lock account setup and PBC contract deployment
**Network**: Etrid FlareChain Mainnet
**Validators**: 21/21 Online

---

## CURRENT STATUS

✅ **Completed:**
- FlareChain runtime with pallet-etr-lock integrated (commit 97d3f581)
- All 12 bridge pallets integrated (Ethereum, Polygon, BNB, Solana, Bitcoin, Cardano, Stellar, XRP, Doge, Tron, Chainlink, USDT)
- Node.js setup scripts created and dependencies installed
- All 21 validators running on mainnet

⚠️ **Pending Issue:**
- WebSocket connection on VM1 (98.71.91.84:9944) needs to be enabled for external connections
- Current error: `API-WS: disconnected from ws://98.71.91.84:9944: 1002`
- Service restart failed when attempting to add ws-external flags

🚀 **Next Steps:**
1. Fix WebSocket connection on VM1
2. Set ETR lock account
3. Deploy PBC smart contracts
4. Configure relayer service
5. Test bridge with small amounts

---

## PHASE 1: FIX WEBSOCKET CONNECTION

### Problem
The validator node on VM1 (98.71.91.84) is not exposing WebSocket for external connections, preventing Node.js scripts from interacting with the chain.

### Solution A: Enable WebSocket on Existing Validator (VM1)

```bash
# SSH into VM1 (EojEdred validator)
ssh azureuser@20.69.26.209

# Edit systemd service
sudo nano /etc/systemd/system/etrid-validator.service

# Modify ExecStart to include WebSocket flags:
# Note: Check which flags your binary supports by running:
/usr/local/bin/etrid-validator --help | grep -i ws

# Try adding these flags (adjust based on help output):
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "EojEdred-Validator-Bootnode" \
  --chain=/home/azureuser/chainspec-21validator-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --rpc-cors all \
  --rpc-port 9944 \
  --ws-port 9944 \
  --ws-external \
  --rpc-methods Unsafe \
  --port 30333

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart etrid-validator

# Check status
sudo systemctl status etrid-validator
sudo journalctl -u etrid-validator -f

# Test connection
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://98.71.91.84:9944
```

### Solution B: Deploy Dedicated RPC Node (Recommended)

Instead of exposing the validator, deploy a dedicated RPC node on a separate VM:

```bash
# On a new VM or existing non-validator VM
scp azureuser@20.69.26.209:/home/azureuser/chainspec-21validator-raw.json ./
scp azureuser@20.69.26.209:/usr/local/bin/etrid-validator ./etrid-rpc

# Create RPC service
sudo mv etrid-rpc /usr/local/bin/
sudo chmod +x /usr/local/bin/etrid-rpc

sudo nano /etc/systemd/system/etrid-rpc.service
```

Service file content:
```ini
[Unit]
Description=Etrid FlareChain RPC Node
After=network.target

[Service]
Type=simple
User=azureuser
ExecStart=/usr/local/bin/etrid-rpc \
  --rpc-external \
  --ws-external \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --chain=/home/azureuser/chainspec-21validator-raw.json \
  --base-path /home/azureuser/.local/share/etrid-rpc \
  --rpc-port 9944 \
  --ws-port 9944 \
  --port 30333 \
  --name "Etrid-RPC-Node"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Start the RPC node:
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-rpc
sudo systemctl start etrid-rpc
sudo systemctl status etrid-rpc

# Update FLARECHAIN_WS in your environment
export FLARECHAIN_WS="ws://<RPC_NODE_IP>:9944"
```

---

## PHASE 2: SET ETR LOCK ACCOUNT

### Prerequisites
- WebSocket connection working (Phase 1 completed)
- Foundation sudo account seed phrase
- Node.js environment ready

### Step 1: Verify Dependencies

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Check that dependencies are installed
ls node_modules/@polkadot/api

# If not installed:
npm install
```

### Step 2: Test Connection

```bash
# Test connection to FlareChain
node check-lock-status.js
```

Expected output:
```
🔍 Checking ETR Lock Status

📡 Connected to: Etrid FlareChain
   Version: 0.1.0

🔐 Lock Account:
   ❌ Not configured
```

### Step 3: Set Lock Account (Option A - Dedicated Account - Recommended)

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Create dedicated bridge lock account
SUDO_SEED="<YOUR_FOUNDATION_SUDO_SEED_PHRASE>" \
LOCK_ACCOUNT_TYPE="dedicated" \
node set-lock-account.js
```

**IMPORTANT**: Save the lock account seed phrase when displayed:
```
⚠️  SAVE THIS SEED: //EtrBridgeLock
```

This seed is needed for emergency recovery of locked funds.

### Step 4: Fund Lock Account

```bash
# The lock account needs ETR for transaction fees
# Send 100-1000 ETR to the lock account address shown in output

# Via Polkadot.js Apps:
# 1. Go to https://polkadot.js.org/apps/
# 2. Connect to ws://98.71.91.84:9944
# 3. Accounts → Transfer
# 4. Send 1000 ETR to lock account address

# Or use script to transfer from foundation account
# (Create if needed)
```

### Step 5: Verify Lock Account Setup

```bash
node check-lock-status.js
```

Expected output:
```
🔍 Checking ETR Lock Status

📡 Connected to: Etrid FlareChain
   Version: 0.1.0

🔐 Lock Account:
   Address: 5EtrBridgeLock...
   Balance: 1000.0 ETR

💰 Total Locked: 0 ETR

📊 Locked per Chain:
   (No locks yet)
```

---

## PHASE 3: DEPLOY PBC SMART CONTRACTS

### Supported Chains

**Layer 2 (EVM):**
- Base (ChainId: 0)
- Arbitrum (ChainId: 1)
- Optimism (ChainId: 2)
- Polygon (ChainId: 3)

**Layer 1 (EVM):**
- Ethereum (ChainId: 10)
- BNB Chain (ChainId: 11)
- Avalanche (ChainId: 12)

**Layer 1 (Non-EVM):**
- Solana (ChainId: 13)
- Bitcoin (ChainId: 20)
- Cardano (ChainId: 21)
- Stellar (ChainId: 22)
- Ripple/XRP (ChainId: 23)
- Dogecoin (ChainId: 24)
- Tron (ChainId: 25)
- Chainlink (ChainId: 26)

**Stablecoin:**
- USDT Bridge (ChainId: 30)

### EVM Chain Deployment (Base, Arbitrum, Polygon, BSC, Ethereum, Avalanche, Optimism)

#### Smart Contract: WrappedETR.sol

```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract WrappedETR is ERC20, Ownable {
    address public relayer;

    event BridgeMint(address indexed to, uint256 amount, bytes32 flareChainTx);
    event BridgeBurn(address indexed from, uint256 amount, string flareChainRecipient);

    constructor() ERC20("Wrapped ETR", "wETR") Ownable(msg.sender) {}

    function setRelayer(address _relayer) external onlyOwner {
        relayer = _relayer;
    }

    function bridgeMint(address to, uint256 amount, bytes32 flareChainTx) external {
        require(msg.sender == relayer, "Only relayer can mint");
        _mint(to, amount);
        emit BridgeMint(to, amount, flareChainTx);
    }

    function bridgeBurn(uint256 amount, string calldata flareChainRecipient) external {
        _burn(msg.sender, amount);
        emit BridgeBurn(msg.sender, amount, flareChainRecipient);
    }

    function decimals() public pure override returns (uint8) {
        return 18; // Match ETR decimals
    }
}
```

#### Deployment Script (Hardhat)

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment

# Create deployment project if not exists
mkdir -p pbc-contracts
cd pbc-contracts

npm init -y
npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox
npm install @openzeppelin/contracts

npx hardhat init
```

Create `contracts/WrappedETR.sol` with contract above.

Create `scripts/deploy.js`:
```javascript
const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying WrappedETR with account:", deployer.address);

  const WrappedETR = await hre.ethers.getContractFactory("WrappedETR");
  const wETR = await WrappedETR.deploy();
  await wETR.waitForDeployment();

  const address = await wETR.getAddress();
  console.log("WrappedETR deployed to:", address);

  // Set relayer (you'll need to deploy relayer service first)
  // await wETR.setRelayer(RELAYER_ADDRESS);

  return address;
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

Create `hardhat.config.js`:
```javascript
require("@nomicfoundation/hardhat-toolbox");

const PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY;

module.exports = {
  solidity: "0.8.20",
  networks: {
    base: {
      url: "https://mainnet.base.org",
      accounts: [PRIVATE_KEY],
      chainId: 8453
    },
    arbitrum: {
      url: "https://arb1.arbitrum.io/rpc",
      accounts: [PRIVATE_KEY],
      chainId: 42161
    },
    optimism: {
      url: "https://mainnet.optimism.io",
      accounts: [PRIVATE_KEY],
      chainId: 10
    },
    polygon: {
      url: "https://polygon-rpc.com",
      accounts: [PRIVATE_KEY],
      chainId: 137
    },
    ethereum: {
      url: `https://mainnet.infura.io/v3/${process.env.INFURA_KEY}`,
      accounts: [PRIVATE_KEY],
      chainId: 1
    },
    bsc: {
      url: "https://bsc-dataseed.binance.org",
      accounts: [PRIVATE_KEY],
      chainId: 56
    },
    avalanche: {
      url: "https://api.avax.network/ext/bc/C/rpc",
      accounts: [PRIVATE_KEY],
      chainId: 43114
    }
  },
  etherscan: {
    apiKey: {
      base: process.env.BASESCAN_API_KEY,
      arbitrumOne: process.env.ARBISCAN_API_KEY,
      optimisticEthereum: process.env.OPTIMISM_API_KEY,
      polygon: process.env.POLYGONSCAN_API_KEY,
      mainnet: process.env.ETHERSCAN_API_KEY,
      bsc: process.env.BSCSCAN_API_KEY,
      avalanche: process.env.SNOWTRACE_API_KEY
    }
  }
};
```

#### Deploy to All EVM Chains

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/pbc-contracts

# Set your deployer private key
export DEPLOYER_PRIVATE_KEY="your_private_key_here"

# Deploy to Base
npx hardhat run scripts/deploy.js --network base

# Deploy to Arbitrum
npx hardhat run scripts/deploy.js --network arbitrum

# Deploy to Optimism
npx hardhat run scripts/deploy.js --network optimism

# Deploy to Polygon
npx hardhat run scripts/deploy.js --network polygon

# Deploy to Ethereum (expensive, deploy last)
npx hardhat run scripts/deploy.js --network ethereum

# Deploy to BSC
npx hardhat run scripts/deploy.js --network bsc

# Deploy to Avalanche
npx hardhat run scripts/deploy.js --network avalanche
```

**Save all deployed contract addresses!** You'll need them for the relayer configuration.

#### Verify Contracts on Block Explorers

```bash
# Base
npx hardhat verify --network base <CONTRACT_ADDRESS>

# Arbitrum
npx hardhat verify --network arbitrum <CONTRACT_ADDRESS>

# Optimism
npx hardhat verify --network optimism <CONTRACT_ADDRESS>

# Polygon
npx hardhat verify --network polygon <CONTRACT_ADDRESS>

# Ethereum
npx hardhat verify --network ethereum <CONTRACT_ADDRESS>

# BSC
npx hardhat verify --network bsc <CONTRACT_ADDRESS>

# Avalanche
npx hardhat verify --network avalanche <CONTRACT_ADDRESS>
```

### Non-EVM Chain Deployment

#### Solana (SPL Token)

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Create SPL token
spl-token create-token --decimals 18

# Create token account
spl-token create-account <TOKEN_ADDRESS>

# Set up bridge program (Anchor framework)
# This requires a custom Solana program - beyond scope of this quick guide
# Recommend using Wormhole or similar bridge protocol for Solana
```

#### Bitcoin (BRC-20 or Runes)

Bitcoin bridging requires:
1. Multisig wallet on Bitcoin network
2. Relayer watching Bitcoin transactions
3. Complex verification logic

**Recommended**: Use existing bridge infrastructure (Wormhole, LayerZero) or deploy later after EVM chains are operational.

#### Other Non-EVM Chains

For Cardano, Stellar, XRP, Dogecoin, Tron:
- Each requires chain-specific smart contract deployment
- Recommend phased rollout: Start with EVM chains
- Deploy non-EVM bridges in Phase 2 after EVM bridges are proven

---

## PHASE 4: CONFIGURE RELAYER SERVICE

The relayer watches for bridge events on external chains and triggers transactions on FlareChain.

### Relayer Architecture

```
External Chain (e.g., Base)
    ↓ (User burns wETR)
BridgeBurn event emitted
    ↓
Relayer watches events
    ↓
Relayer calls FlareChain extrinsic
    ↓
process_etr_burn_from_ethereum()
    ↓
ETR unlocked and sent to user
```

### Create Relayer Service

Create `/Users/macbook/Desktop/etrid/services/bridge-relayer/index.js`:

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ethers } = require('ethers');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

// Configuration
const FLARECHAIN_WS = process.env.FLARECHAIN_WS || 'ws://98.71.91.84:9944';
const RELAYER_SEED = process.env.RELAYER_SEED; // Relayer account on FlareChain
const CHAINS = {
  ethereum: {
    rpc: process.env.ETH_RPC || 'https://mainnet.infura.io/v3/YOUR_KEY',
    contract: process.env.ETH_CONTRACT_ADDRESS,
    chainId: 10
  },
  base: {
    rpc: process.env.BASE_RPC || 'https://mainnet.base.org',
    contract: process.env.BASE_CONTRACT_ADDRESS,
    chainId: 0
  },
  // Add other chains...
};

const WRAPPED_ETR_ABI = [
  "event BridgeBurn(address indexed from, uint256 amount, string flareChainRecipient)",
  "event BridgeMint(address indexed to, uint256 amount, bytes32 flareChainTx)"
];

class BridgeRelayer {
  constructor() {
    this.flareApi = null;
    this.relayerAccount = null;
    this.providers = {};
    this.contracts = {};
    this.processedTxs = new Set();
  }

  async init() {
    console.log('🚀 Initializing Bridge Relayer...');

    // Connect to FlareChain
    await cryptoWaitReady();
    const provider = new WsProvider(FLARECHAIN_WS);
    this.flareApi = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: 'sr25519' });
    this.relayerAccount = keyring.addFromUri(RELAYER_SEED);

    console.log(`✅ Connected to FlareChain: ${await this.flareApi.rpc.system.chain()}`);
    console.log(`🔑 Relayer Account: ${this.relayerAccount.address}`);

    // Connect to external chains
    for (const [name, config] of Object.entries(CHAINS)) {
      this.providers[name] = new ethers.JsonRpcProvider(config.rpc);
      this.contracts[name] = new ethers.Contract(
        config.contract,
        WRAPPED_ETR_ABI,
        this.providers[name]
      );
      console.log(`✅ Connected to ${name}: ${config.contract}`);
    }
  }

  async watchBridgeBurns() {
    console.log('👀 Watching for bridge burn events...');

    for (const [name, contract] of Object.entries(this.contracts)) {
      contract.on('BridgeBurn', async (from, amount, flareRecipient, event) => {
        const txHash = event.log.transactionHash;

        if (this.processedTxs.has(txHash)) {
          console.log(`⏭️  Already processed: ${txHash}`);
          return;
        }

        console.log(`🔥 Burn detected on ${name}:`);
        console.log(`   From: ${from}`);
        console.log(`   Amount: ${ethers.formatEther(amount)} wETR`);
        console.log(`   FlareChain Recipient: ${flareRecipient}`);
        console.log(`   Tx: ${txHash}`);

        await this.processUnlock(name, flareRecipient, amount, txHash);
      });
    }
  }

  async processUnlock(chainName, recipient, amount, burnTxHash) {
    try {
      const chainConfig = CHAINS[chainName];
      const chainId = chainConfig.chainId;

      // Determine which bridge pallet to call based on chain
      let bridgePallet;
      switch(chainId) {
        case 10: bridgePallet = 'ethereumBridge'; break;
        case 0: bridgePallet = 'baseBridge'; break;
        case 1: bridgePallet = 'arbitrumBridge'; break;
        case 3: bridgePallet = 'polygonBridge'; break;
        case 11: bridgePallet = 'bnbBridge'; break;
        // Add other chains...
        default:
          console.error(`❌ Unknown chain ID: ${chainId}`);
          return;
      }

      // Convert burn tx hash to bytes32
      const burnTxBytes = burnTxHash.replace('0x', '');

      // Call process_etr_burn_from_xxx on FlareChain
      const tx = this.flareApi.tx[bridgePallet].processEtrBurnFromEthereum(
        recipient,
        amount.toString(),
        `0x${burnTxBytes}`
      );

      console.log(`🔄 Submitting unlock transaction to FlareChain...`);

      await tx.signAndSend(this.relayerAccount, ({ status, events }) => {
        if (status.isInBlock) {
          console.log(`✅ Unlock in block: ${status.asInBlock.toHex()}`);
          this.processedTxs.add(burnTxHash);
        }

        if (status.isFinalized) {
          console.log(`✅ Unlock finalized: ${status.asFinalized.toHex()}`);

          events.forEach(({ event }) => {
            if (event.section === 'system' && event.method === 'ExtrinsicFailed') {
              console.error(`❌ Unlock failed:`, event.data.toString());
            }
          });
        }
      });

    } catch (error) {
      console.error(`❌ Error processing unlock:`, error);
    }
  }

  async start() {
    await this.init();
    await this.watchBridgeBurns();
    console.log('✅ Relayer running...');
  }
}

// Run relayer
const relayer = new BridgeRelayer();
relayer.start().catch(console.error);
```

### Create Relayer Package

```bash
cd /Users/macbook/Desktop/etrid/services/bridge-relayer

cat > package.json << 'EOF'
{
  "name": "etrid-bridge-relayer",
  "version": "1.0.0",
  "description": "FlareChain PBC Bridge Relayer Service",
  "main": "index.js",
  "scripts": {
    "start": "node index.js"
  },
  "dependencies": {
    "@polkadot/api": "^10.11.2",
    "@polkadot/util": "^12.6.2",
    "@polkadot/util-crypto": "^12.6.2",
    "@polkadot/keyring": "^12.6.2",
    "ethers": "^6.9.0"
  }
}
EOF

npm install
```

### Deploy Relayer Service

```bash
# Create relayer account on FlareChain
# Via Polkadot.js Apps or:
cd /Users/macbook/Desktop/etrid/scripts
node -e "
const { Keyring } = require('@polkadot/keyring');
const { cryptoWaitReady } = require('@polkadot/util-crypto');
(async () => {
  await cryptoWaitReady();
  const keyring = new Keyring({ type: 'sr25519' });
  const account = keyring.addFromUri('//BridgeRelayer');
  console.log('Relayer Address:', account.address);
  console.log('Seed: //BridgeRelayer');
})();
"

# Fund relayer account with ETR for transaction fees
# Send 1000 ETR to relayer address

# Set relayer on deployed contracts
cd /Users/macbook/Desktop/etrid/dex-deployment/pbc-contracts

# For each chain:
npx hardhat console --network base
> const WrappedETR = await ethers.getContractAt("WrappedETR", "<BASE_CONTRACT_ADDRESS>")
> await WrappedETR.setRelayer("<RELAYER_EOA_ADDRESS>")

# Repeat for all chains

# Start relayer service
cd /Users/macbook/Desktop/etrid/services/bridge-relayer

FLARECHAIN_WS="ws://98.71.91.84:9944" \
RELAYER_SEED="//BridgeRelayer" \
BASE_CONTRACT_ADDRESS="<BASE_DEPLOYED_ADDRESS>" \
ETH_CONTRACT_ADDRESS="<ETH_DEPLOYED_ADDRESS>" \
node index.js
```

### Production Deployment (Systemd)

```bash
sudo nano /etc/systemd/system/bridge-relayer.service
```

```ini
[Unit]
Description=Etrid Bridge Relayer
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser/etrid/services/bridge-relayer
Environment="FLARECHAIN_WS=ws://98.71.91.84:9944"
Environment="RELAYER_SEED=//BridgeRelayer"
Environment="BASE_CONTRACT_ADDRESS=0x..."
Environment="ETH_CONTRACT_ADDRESS=0x..."
ExecStart=/usr/bin/node index.js
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable bridge-relayer
sudo systemctl start bridge-relayer
sudo systemctl status bridge-relayer
```

---

## PHASE 5: TEST BRIDGE

### Test 1: FlareChain → Base (Lock and Mint)

```bash
# Via Polkadot.js Apps
# 1. Go to https://polkadot.js.org/apps/
# 2. Connect to ws://98.71.91.84:9944
# 3. Developer → Extrinsics
# 4. Select: baseBridge.bridgeEtrToBase
#    - amount: 100000000000000000000 (100 ETR with 18 decimals)
#    - baseDestination: 0xYourBaseWalletAddress
# 5. Sign and Submit

# Watch relayer logs
sudo journalctl -u bridge-relayer -f

# Check Base network for minted wETR
# Add wETR token to MetaMask on Base network
# Token address: <BASE_CONTRACT_ADDRESS>
```

### Test 2: Base → FlareChain (Burn and Unlock)

```bash
# Via MetaMask on Base network
# 1. Add wETR token: <BASE_CONTRACT_ADDRESS>
# 2. Go to Base block explorer
# 3. Find wETR contract, Write Contract tab
# 4. Call bridgeBurn():
#    - amount: 50000000000000000000 (50 wETR)
#    - flareChainRecipient: "5YourFlareChainAddress..."
# 5. Confirm transaction

# Watch relayer process the burn
sudo journalctl -u bridge-relayer -f

# Check FlareChain balance
# Via Polkadot.js Apps → Accounts
# You should receive 50 ETR back
```

### Verify Lock Status

```bash
cd /Users/macbook/Desktop/etrid/scripts
node check-lock-status.js
```

Should show:
```
💰 Total Locked: 50 ETR

📊 Locked per Chain:
   Base        : 50 ETR
```

---

## PHASE 6: MONITORING AND MAINTENANCE

### Monitor Lock Account

Create monitoring script `/Users/macbook/Desktop/etrid/scripts/monitor-locks.js`:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

const FLARECHAIN_WS = 'ws://98.71.91.84:9944';

async function monitor() {
  const provider = new WsProvider(FLARECHAIN_WS);
  const api = await ApiPromise.create({ provider });

  console.log('📊 ETR Lock Monitoring Dashboard\n');

  const lockAccount = await api.query.etrLock.lockAccount();
  if (!lockAccount.isSome) {
    console.error('❌ Lock account not configured!');
    process.exit(1);
  }

  const address = lockAccount.unwrap().toString();
  const { data: balance } = await api.query.system.account(address);
  const totalLocked = await api.query.etrLock.totalLocked();

  console.log(`Lock Account: ${address}`);
  console.log(`Account Balance: ${formatBalance(balance.free)} ETR`);
  console.log(`Total Locked: ${formatBalance(totalLocked)} ETR`);
  console.log(`Available: ${formatBalance(balance.free - totalLocked)} ETR\n`);

  // Alert if balance is low
  if (balance.free < totalLocked * 1.1) {
    console.warn('⚠️  WARNING: Lock account balance is low!');
    console.warn('   Add more ETR to cover locked amount + fees\n');
  }

  // Per-chain breakdown
  const chains = {
    Base: 0, Arbitrum: 1, Optimism: 2, Polygon: 3,
    Ethereum: 10, BnbChain: 11, Avalanche: 12, Solana: 13,
    Bitcoin: 20, Cardano: 21, Stellar: 22, Ripple: 23,
    Dogecoin: 24, Tron: 25, Chainlink: 26, UsdtBridge: 30
  };

  console.log('Per-Chain Locks:');
  for (const [name, id] of Object.entries(chains)) {
    const locked = await api.query.etrLock.lockedForChain(id);
    if (locked.toString() !== '0') {
      console.log(`  ${name.padEnd(12)}: ${formatBalance(locked)} ETR`);
    }
  }

  process.exit(0);
}

function formatBalance(balance) {
  const balanceStr = balance.toString();
  if (balanceStr === '0') return '0';
  if (balanceStr.length <= 18) {
    return `0.${balanceStr.padStart(18, '0')}`;
  }
  const whole = balanceStr.slice(0, -18);
  const decimal = balanceStr.slice(-18).replace(/0+$/, '');
  return decimal ? `${whole}.${decimal}` : whole;
}

monitor().catch(console.error);
```

Run monitoring:
```bash
cd /Users/macbook/Desktop/etrid/scripts
node monitor-locks.js

# Set up cron job for hourly monitoring
crontab -e
# Add: 0 * * * * cd /Users/macbook/Desktop/etrid/scripts && node monitor-locks.js >> /tmp/lock-monitor.log 2>&1
```

### Monitor Relayer Health

```bash
# Check relayer logs
sudo journalctl -u bridge-relayer -f

# Check for errors
sudo journalctl -u bridge-relayer --since "1 hour ago" | grep -i error

# Restart if needed
sudo systemctl restart bridge-relayer
```

### Alerts Setup

Create alert script `/Users/macbook/Desktop/etrid/scripts/alerts.sh`:

```bash
#!/bin/bash

LOCK_BALANCE=$(node check-lock-status.js | grep "Balance:" | awk '{print $2}')
TOTAL_LOCKED=$(node check-lock-status.js | grep "Total Locked:" | awk '{print $3}')

# Alert if balance < total locked * 1.5
THRESHOLD=$(echo "$TOTAL_LOCKED * 1.5" | bc)

if (( $(echo "$LOCK_BALANCE < $THRESHOLD" | bc -l) )); then
  echo "ALERT: Lock account balance low!"
  echo "Balance: $LOCK_BALANCE ETR"
  echo "Locked: $TOTAL_LOCKED ETR"
  echo "Threshold: $THRESHOLD ETR"

  # Send alert (configure your notification method)
  # curl -X POST https://your-webhook-url -d "Lock account low: $LOCK_BALANCE ETR"
fi
```

---

## DEPLOYMENT CHECKLIST

### Pre-Deployment
- [ ] WebSocket connection working on FlareChain node
- [ ] Foundation sudo account seed phrase secured
- [ ] Deployer wallet funded on all target chains
- [ ] API keys for RPC providers (Infura, Alchemy, etc.)
- [ ] Block explorer API keys for verification

### Lock Account Setup
- [ ] Node.js dependencies installed
- [ ] Lock account created and saved
- [ ] Lock account funded with 1000 ETR
- [ ] Lock account verified via check-lock-status.js

### Smart Contract Deployment
- [ ] WrappedETR deployed to Base
- [ ] WrappedETR deployed to Arbitrum
- [ ] WrappedETR deployed to Optimism
- [ ] WrappedETR deployed to Polygon
- [ ] WrappedETR deployed to Ethereum
- [ ] WrappedETR deployed to BSC
- [ ] WrappedETR deployed to Avalanche
- [ ] All contracts verified on block explorers
- [ ] All contract addresses documented

### Relayer Setup
- [ ] Relayer account created on FlareChain
- [ ] Relayer account funded with 1000 ETR
- [ ] Relayer service deployed
- [ ] Relayer set on all deployed contracts
- [ ] Relayer service running and monitoring events

### Testing
- [ ] Test lock on FlareChain (100 ETR)
- [ ] Test mint on external chain (verify wETR received)
- [ ] Test burn on external chain (100 wETR)
- [ ] Test unlock on FlareChain (verify ETR received)
- [ ] Test on all deployed chains
- [ ] Verify lock status matches bridged amounts

### Monitoring
- [ ] Lock monitoring script running
- [ ] Relayer health checks configured
- [ ] Alerts configured for low balances
- [ ] Dashboard for tracking bridge activity

### Documentation
- [ ] Contract addresses published
- [ ] User guide for bridging ETR
- [ ] Developer integration guide
- [ ] Emergency procedures documented

---

## TROUBLESHOOTING

### Issue: WebSocket Connection Refused

**Symptoms**: `API-WS: disconnected from ws://98.71.91.84:9944: 1002`

**Solutions**:
1. Check if node is running: `sudo systemctl status etrid-validator`
2. Check firewall: `sudo ufw status` - ensure port 9944 is open
3. Try local connection first: `ws://127.0.0.1:9944`
4. Deploy dedicated RPC node (see Phase 1, Solution B)

### Issue: Lock Account Not Set

**Symptoms**: `Error: LockAccountNotSet`

**Solution**: Run `node set-lock-account.js` with foundation sudo account

### Issue: Insufficient Balance for Lock

**Symptoms**: `Error: InsufficientBalance`

**Solutions**:
1. Check user has enough ETR: Polkadot.js Apps → Accounts
2. Ensure lock account has funds for fees
3. Check total supply hasn't been exceeded

### Issue: Burn Already Processed

**Symptoms**: `Error: BurnAlreadyProcessed`

**Explanation**: This burn transaction was already processed. Each external chain burn can only unlock ETR once.

**Solution**: This is expected behavior for duplicate transactions. No action needed.

### Issue: Relayer Not Processing Burns

**Symptoms**: Burned wETR on external chain but no unlock on FlareChain

**Solutions**:
1. Check relayer is running: `sudo systemctl status bridge-relayer`
2. Check relayer logs: `sudo journalctl -u bridge-relayer -f`
3. Verify relayer has ETR for fees
4. Verify contract addresses in relayer config
5. Check relayer was set on contract: `contract.relayer()`

### Issue: Lock Account Balance Low

**Symptoms**: Alert from monitoring script

**Solution**: Transfer more ETR to lock account to cover locked amounts + fees

---

## SECURITY CONSIDERATIONS

### Private Key Management

⚠️ **CRITICAL**: Never commit or expose these private keys:
- Foundation sudo seed phrase
- Lock account seed (`//EtrBridgeLock`)
- Relayer account seed (`//BridgeRelayer`)
- Deployer wallet private key

Store in:
- Hardware wallet (Ledger, Trezor)
- Encrypted keystore
- Secure secret management (HashiCorp Vault, AWS Secrets Manager)

### Lock Account Security

The lock account holds ALL bridged ETR. Security measures:
1. Use dedicated account (not foundation multisig)
2. Regular balance monitoring
3. Alert on unusual activity
4. Emergency procedures documented
5. Backup seed phrase in secure location

### Relayer Security

The relayer can unlock ETR on FlareChain:
1. Validate burn proofs before unlocking
2. Check burn transaction confirmations (12+ blocks)
3. Rate limiting on unlocks
4. Multi-signature for large unlocks (future enhancement)
5. Monitor for abnormal activity

### Smart Contract Security

Before mainnet deployment:
1. Audit WrappedETR contract
2. Test on testnets extensively
3. Use timelocks for admin functions
4. Consider upgradeable proxies
5. Bug bounty program

---

## CONTACT AND SUPPORT

**Developer**: Eoj (Etrid Foundation)

**Resources**:
- Bridge Integration Guide: `/Users/macbook/Desktop/etrid/pallets/pallet-etr-lock/BRIDGE_INTEGRATION_GUIDE.md`
- Implementation Status: `/Users/macbook/Desktop/etrid/pallets/pallet-etr-lock/IMPLEMENTATION_STATUS.md`
- FlareChain Documentation: `/Users/macbook/Desktop/etrid/docs/`

**Emergency Procedures**:
If bridge issue detected:
1. Stop relayer: `sudo systemctl stop bridge-relayer`
2. Assess issue severity
3. If funds at risk, use foundation sudo to pause bridge
4. Communicate with users
5. Deploy fix
6. Resume operations

---

## NEXT SESSION PROMPT

```
I need to complete the FlareChain PBC bridge deployment. Here's the current status:

COMPLETED:
- FlareChain runtime with pallet-etr-lock integrated (all 12 bridges)
- Node.js setup scripts created (set-lock-account.js, check-lock-status.js)
- All 21 validators running on mainnet
- Dependencies installed

CURRENT ISSUE:
- WebSocket connection to FlareChain node (ws://98.71.91.84:9944) is not working
- Error: API-WS: disconnected with 1002 status code
- Need to enable WebSocket on validator node or deploy dedicated RPC node

NEXT STEPS:
1. Fix WebSocket connection (either enable on VM1 or deploy dedicated RPC node)
2. Run set-lock-account.js to configure ETR lock account
3. Deploy WrappedETR contracts to: Base, Arbitrum, Optimism, Polygon, Ethereum, BSC, Avalanche
4. Set up bridge relayer service
5. Test bridge flow with small amounts

CRITICAL INFORMATION:
- FlareChain mainnet: 98.71.91.84:9944
- Foundation sudo account: [secure]
- Lock account seed: //EtrBridgeLock (will be created)
- Total ETR supply: 2.521 billion (must maintain 1:1 backing)

FILES:
- Setup scripts: /Users/macbook/Desktop/etrid/scripts/
- Bridge pallets: /Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/
- Deployment guide: /Users/macbook/Desktop/etrid/scripts/BRIDGE_DEPLOYMENT_HANDOFF.md

Please help me continue from Step 1: Fix WebSocket connection and proceed with the deployment.
```

---

**END OF HANDOFF DOCUMENT**

Generated: 2025-11-01
Version: 1.0
Status: Ready for WebSocket fix and deployment
