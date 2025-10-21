# EDSC Bridge Local Testing Guide

**Complete guide for testing the EDSC cross-chain bridge on your local machine.**

Created: October 20, 2025
Status: Ready to test

---

## Overview

This guide walks you through testing the full bridge locally:
- ✅ FlareChain (Substrate) node
- ✅ Ethereum local network (Hardhat)
- ✅ Ethereum smart contracts
- ✅ Attestation service
- ✅ Relayer service
- ✅ End-to-end transfer test

**Time Required**: 30-60 minutes for full setup

---

## Prerequisites

### Already Completed ✅
- [x] All dependencies installed
- [x] FlareChain node built (`target/release/flarechain-node`)
- [x] Services built (attestation + relayer)
- [x] Contracts ready to deploy

### What You Need
- **5 terminal windows** (or tmux/screen)
- **Node.js** v18+ (you already have this)
- **Rust** (you already have this)

---

## Testing Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Your Local Machine                         │
│                                                              │
│  Terminal 1              Terminal 2           Terminal 3    │
│  ┌────────────┐          ┌────────────┐      ┌──────────┐  │
│  │ FlareChain │          │  Hardhat   │      │Attester  │  │
│  │    Node    │          │  Network   │      │ Service  │  │
│  └────────────┘          └────────────┘      └──────────┘  │
│  ws://127.0.0.1          http://127.0.0.1    Port 3000     │
│     :9944                   :8545                           │
│                                                              │
│  Terminal 4              Terminal 5                         │
│  ┌────────────┐          ┌────────────┐                    │
│  │  Relayer   │          │   Test     │                    │
│  │  Service   │          │  Scripts   │                    │
│  └────────────┘          └────────────┘                    │
│  Port 3001                                                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Step-by-Step Setup

### Terminal 1: Start FlareChain Node

```bash
# Navigate to project root
cd /Users/macbook/Desktop/etrid

# Start FlareChain in dev mode
./target/release/flarechain-node \
  --dev \
  --tmp \
  --rpc-cors all \
  --rpc-methods unsafe \
  --rpc-port 9944

# ✅ You should see:
# - "Running in --dev mode"
# - "Local node identity: 12D3KooW..."
# - Blocks being produced every 6 seconds
```

**What this does:**
- Starts a local Substrate blockchain
- Alice is the validator (built-in dev account)
- Fresh state each time (`--tmp`)
- RPC available at `ws://127.0.0.1:9944`

**Keep this terminal running!**

---

### Terminal 2: Start Hardhat Local Network

```bash
# Navigate to contracts
cd /Users/macbook/Desktop/etrid/contracts/ethereum

# Start local Ethereum node
npx hardhat node

# ✅ You should see:
# - Started HTTP and WebSocket JSON-RPC server at http://127.0.0.1:8545/
# - List of 20 accounts with private keys
# - Each account has 10000 ETH
```

**Copy the first account private key** - you'll need it for deployment!

Example:
```
Account #0: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (10000 ETH)
Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

**Keep this terminal running!**

---

### Terminal 3: Deploy Ethereum Contracts

```bash
# Still in contracts/ethereum directory
cd /Users/macbook/Desktop/etrid/contracts/ethereum

# Deploy contracts to local Hardhat network
npx hardhat run scripts/deploy.js --network localhost

# ✅ You should see:
# - EDSC deployed to: 0x5FbDB...
# - AttesterRegistry deployed to: 0xe7f1...
# - MessageTransmitter deployed to: 0x9fE4...
# - TokenMessenger deployed to: 0xCf7E...
```

**IMPORTANT: Save these addresses!** They'll be written to a `deployment-localhost-*.json` file.

Example output:
```json
{
  "network": "localhost",
  "contracts": {
    "EDSC": "0x5FbDB2315678afecb367f032d93F642f64180aa3",
    "AttesterRegistry": "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
    "MessageTransmitter": "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
    "TokenMessenger": "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
  }
}
```

---

### Step 4: Register Attester on Ethereum

We need to register at least 3 attesters (for 3-of-5 threshold).

```bash
# Create a script to register attesters
cd /Users/macbook/Desktop/etrid/contracts/ethereum

# Create register-attesters.js
cat > scripts/register-attesters.js << 'EOF'
const hre = require("hardhat");

async function main() {
  // Read deployment file
  const fs = require("fs");
  const files = fs.readdirSync(".").filter(f => f.startsWith("deployment-localhost"));
  const latestFile = files.sort().reverse()[0];
  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));

  const attesterRegistryAddress = deployment.contracts.AttesterRegistry;
  console.log("AttesterRegistry:", attesterRegistryAddress);

  const AttesterRegistry = await hre.ethers.getContractFactory("AttesterRegistry");
  const registry = AttesterRegistry.attach(attesterRegistryAddress);

  // Test attester addresses (these should match your attestation service config)
  const attesters = [
    "0x70997970C51812dc3A010C7d01b50e0d17dc79C8", // Hardhat account #1
    "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC", // Hardhat account #2
    "0x90F79bf6EB2c4f870365E785982E1f101E93b906", // Hardhat account #3
  ];

  for (let i = 0; i < attesters.length; i++) {
    console.log(\`Registering attester \${i}: \${attesters[i]}\`);
    const tx = await registry.registerAttester(attesters[i]);
    await tx.wait();
    console.log(\`  ✓ Registered\`);
  }

  console.log("\nAttesters registered successfully!");
}

main().catch(console.error);
EOF

# Run the registration script
npx hardhat run scripts/register-attesters.js --network localhost

# ✅ You should see:
# - Registering attester 0: 0x7099...
# - Registering attester 1: 0x3C44...
# - Registering attester 2: 0x90F7...
```

---

### Step 5: Configure Attestation Service

```bash
cd /Users/macbook/Desktop/etrid/services/attestation-service

# Create .env file
cat > .env << 'EOF'
# Chain connections
SUBSTRATE_WS_URL=ws://127.0.0.1:9944
ETHEREUM_RPC_URL=http://127.0.0.1:8545

# Attester identity (using Hardhat account #1)
ATTESTER_PRIVATE_KEY=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
ATTESTER_ID=0
ATTESTER_ADDRESS=0x70997970C51812dc3A010C7d01b50e0d17dc79C8

# Signature thresholds
MIN_SIGNATURES=3
TOTAL_ATTESTERS=5

# Security
CONFIRMATIONS_REQUIRED=1

# Contract addresses (update these from your deployment!)
TOKEN_MESSENGER_ADDRESS=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

# API
PORT=3000
EOF

# Update TOKEN_MESSENGER_ADDRESS with your actual deployed address
echo "⚠️  UPDATE TOKEN_MESSENGER_ADDRESS in .env with your deployed address!"
```

**Start attestation service:**

```bash
npm run build
npm start

# ✅ You should see:
# - Starting EDSC Attestation Service
# - Configuration loaded
# - Message signer initialized
# - Substrate monitor started
# - Ethereum monitor started
# - API server started
```

**Keep this terminal running!**

---

### Terminal 4: Configure Relayer Service

```bash
cd /Users/macbook/Desktop/etrid/services/relayer-service

# Create .env file
cat > .env << 'EOF'
# Attestation services
ATTESTATION_SERVICE_URLS=http://localhost:3000

# Chain connections
SUBSTRATE_WS_URL=ws://127.0.0.1:9944
ETHEREUM_RPC_URL=http://127.0.0.1:8545

# Relayer identity (using Hardhat account #4)
RELAYER_PRIVATE_KEY=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
RELAYER_ADDRESS=0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65

# Contract addresses (update these!)
MESSAGE_TRANSMITTER_ADDRESS=0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
TOKEN_MESSENGER_ADDRESS=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

# Polling settings
POLL_INTERVAL_MS=10000
MAX_RETRIES=3
RETRY_DELAY_MS=30000

# Gas settings (optional for local)
GAS_LIMIT=500000

# API
ENABLE_API=true
API_PORT=3001
EOF

# Update contract addresses with your deployed addresses
echo "⚠️  UPDATE MESSAGE_TRANSMITTER_ADDRESS and TOKEN_MESSENGER_ADDRESS!"
```

**Start relayer service:**

```bash
npm run build
npm start

# ✅ You should see:
# - Starting relayer service
# - Connected to Ethereum
# - Connected to Substrate
# - Attestation fetcher started
# - Relayer balances (ETH + EDSC)
```

**Keep this terminal running!**

---

## Testing the Bridge

Now that everything is running, let's test a cross-chain transfer!

### Terminal 5: Test Script

```bash
cd /Users/macbook/Desktop/etrid/contracts/ethereum

# Create a test transfer script
cat > scripts/test-transfer.js << 'EOF'
const hre = require("hardhat");
const fs = require("fs");

async function main() {
  // Load deployment
  const files = fs.readdirSync(".").filter(f => f.startsWith("deployment-localhost"));
  const deployment = JSON.parse(fs.readFileSync(files[0], "utf8"));

  const edscAddress = deployment.contracts.EDSC;
  const tokenMessengerAddress = deployment.contracts.TokenMessenger;

  // Get contracts
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = EDSC.attach(edscAddress);

  const TokenMessenger = await hre.ethers.getContractFactory("EDSCTokenMessenger");
  const messenger = TokenMessenger.attach(tokenMessengerAddress);

  const [sender] = await hre.ethers.getSigners();
  console.log("Sender:", sender.address);

  // 1. Mint some EDSC to sender (owner can mint)
  console.log("\n1. Minting 1000 EDSC to sender...");
  const mintTx = await edsc.mint(sender.address, hre.ethers.parseUnits("1000", 18));
  await mintTx.wait();
  console.log("   ✓ Minted");

  // Check balance
  const balance = await edsc.balanceOf(sender.address);
  console.log("   Balance:", hre.ethers.formatUnits(balance, 18), "EDSC");

  // 2. Approve TokenMessenger
  console.log("\n2. Approving TokenMessenger...");
  const approveTx = await edsc.approve(
    tokenMessengerAddress,
    hre.ethers.parseUnits("100", 18)
  );
  await approveTx.wait();
  console.log("   ✓ Approved");

  // 3. Transfer to Substrate (domain 2)
  console.log("\n3. Initiating cross-chain transfer...");
  const recipientSubstrate = "0x1234567890123456789012345678901234567890123456789012345678901234"; // 32 bytes
  const amount = hre.ethers.parseUnits("100", 18);
  const destinationDomain = 2; // Ëtrid

  const transferTx = await messenger.depositForBurn(
    amount,
    destinationDomain,
    recipientSubstrate
  );
  const receipt = await transferTx.wait();
  console.log("   ✓ Transfer initiated");
  console.log("   TX Hash:", receipt.hash);

  // Find MessageSent event
  const event = receipt.logs.find(log => {
    try {
      return messenger.interface.parseLog(log).name === "MessageSent";
    } catch {
      return false;
    }
  });

  if (event) {
    const parsed = messenger.interface.parseLog(event);
    console.log("\n   Message Details:");
    console.log("   - Nonce:", parsed.args.nonce.toString());
    console.log("   - Amount:", hre.ethers.formatUnits(parsed.args.amount, 18), "EDSC");
    console.log("   - Destination Domain:", parsed.args.destinationDomain);
  }

  console.log("\n✅ Transfer initiated successfully!");
  console.log("\nNext steps:");
  console.log("1. Attestation service should detect the burn event");
  console.log("2. After 3 signatures, message becomes 'ready'");
  console.log("3. Relayer will automatically relay to Substrate");
  console.log("4. Check attestation service: http://localhost:3000/stats");
  console.log("5. Check relayer service: http://localhost:3001/health");
}

main().catch(console.error);
EOF

# Run the test
npx hardhat run scripts/test-transfer.js --network localhost
```

---

## Monitoring the Transfer

### Check Attestation Service

```bash
# View attestation stats
curl http://localhost:3000/stats | jq

# Expected output:
# {
#   "pending": 1,
#   "ready": 0,
#   "relayed": 0,
#   "total": 1
# }

# After ~30 seconds (3 signatures):
# {
#   "pending": 0,
#   "ready": 1,
#   "relayed": 0,
#   "total": 1
# }
```

### Check Relayer Service

```bash
# View relayer health
curl http://localhost:3001/health | jq

# View Prometheus metrics
curl http://localhost:3001/metrics
```

### Check Substrate Logs

In Terminal 1 (FlareChain), you should see:
```
Imported #123 (0x1234...)
  Successfully minted 100 EDSC to 0x1234...
```

---

## Troubleshooting

### Problem: FlareChain node won't start

```bash
# Make sure port 9944 is free
lsof -i :9944

# Kill any existing process
kill -9 <PID>

# Try starting again
./target/release/flarechain-node --dev --tmp
```

### Problem: Hardhat network connection failed

```bash
# Make sure Hardhat is running
curl http://localhost:8545 \
  -X POST \
  -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Should return: {"jsonrpc":"2.0","id":1,"result":"0x..."}
```

### Problem: Attestation service can't connect to chains

Check `.env` file:
- `SUBSTRATE_WS_URL=ws://127.0.0.1:9944` (note: `ws://` not `wss://`)
- `ETHEREUM_RPC_URL=http://127.0.0.1:8545` (note: `http://` not `https://`)

### Problem: No attestations being created

1. Check that attesters are registered:
```bash
npx hardhat console --network localhost

> const registry = await ethers.getContractAt("AttesterRegistry", "0x...");
> await registry.getAttesterCount();
// Should return: 3n
```

2. Check attestation service logs for errors

3. Verify contract addresses match in `.env`

### Problem: Relayer not picking up messages

1. Check attestation service is returning ready messages:
```bash
curl http://localhost:3000/attestations/ready
```

2. Check relayer logs for errors

3. Verify relayer has funds:
```bash
curl http://localhost:3001/health | jq .relayerAddress
```

---

## Success Criteria

✅ **You've successfully tested the bridge when:**

1. Ethereum contracts deployed
2. Attesters registered (at least 3)
3. Attestation service running and monitoring both chains
4. Relayer service running and polling attestations
5. Test transfer initiated on Ethereum
6. Attestation service creates attestation (visible in `/stats`)
7. After 3 signatures, attestation status changes to "ready"
8. Relayer automatically relays message to Substrate
9. EDSC minted on Substrate chain
10. Transfer marked as "relayed" in attestation service

---

## Next Steps After Local Testing

Once local testing works:

1. **Test on Sepolia testnet:**
   - Deploy contracts to Sepolia
   - Register real attesters
   - Test with real testnet tokens

2. **Deploy to Ember testnet:**
   - Set up 5 attester VMs
   - Deploy 2-3 relayer VMs
   - Configure monitoring (Prometheus + Grafana)

3. **Public beta:**
   - Invite users
   - Collect feedback
   - Fix bugs

4. **Security audit:**
   - Smart contracts
   - Off-chain services
   - Key management

5. **Mainnet preparation:**
   - Final testing
   - Documentation
   - Launch!

---

**Last Updated**: October 20, 2025
**Status**: Ready for local testing
**Estimated Time**: 30-60 minutes for full setup
