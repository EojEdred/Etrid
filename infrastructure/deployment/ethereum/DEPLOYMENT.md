# Ethereum Testnet Deployment (Sepolia)

Complete guide for deploying EDSC smart contracts to Ethereum Sepolia testnet.

## Prerequisites

### 1. Get Sepolia ETH

You'll need ~1 ETH on Sepolia for deployment and operations.

**Faucets**:
- [Alchemy Sepolia Faucet](https://sepoliafaucet.com/)
- [Infura Sepolia Faucet](https://www.infura.io/faucet/sepolia)
- [Chainlink Sepolia Faucet](https://faucets.chain.link/sepolia)

### 2. Get RPC Provider

Sign up for a free RPC provider:
- **Alchemy**: https://www.alchemy.com/ (Recommended)
- **Infura**: https://www.infura.io/
- **QuickNode**: https://www.quicknode.com/

Get your Sepolia RPC URL:
```
https://eth-sepolia.g.alchemy.com/v2/YOUR-API-KEY
```

### 3. Get Etherscan API Key

For contract verification:
- Sign up: https://etherscan.io/register
- Create API key: https://etherscan.io/myapikey

### 4. Prepare Attester Addresses

Generate 5 Ethereum addresses for attesters:

```bash
# Using ethers (Node.js)
node -e "const ethers = require('ethers'); for(let i=0; i<5; i++) { const wallet = ethers.Wallet.createRandom(); console.log(\`Attester \${i}: \${wallet.address}\`); console.log(\`Private Key: \${wallet.privateKey}\n\`); }"
```

**Save these securely!** You'll need them for attestation services.

## Configuration

### 1. Update Hardhat Config

Edit `contracts/ethereum/hardhat.config.js`:

```javascript
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

module.exports = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    sepolia: {
      url: process.env.SEPOLIA_RPC_URL,
      accounts: [process.env.DEPLOYER_PRIVATE_KEY],
      chainId: 11155111,
    },
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY,
  },
};
```

### 2. Create Environment File

Create `contracts/ethereum/.env`:

```bash
# RPC Provider
SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR-API-KEY

# Deployer Account
DEPLOYER_PRIVATE_KEY=0x...

# Etherscan (for verification)
ETHERSCAN_API_KEY=YOUR-API-KEY

# Attester Addresses (from above)
ATTESTER_0=0x...
ATTESTER_1=0x...
ATTESTER_2=0x...
ATTESTER_3=0x...
ATTESTER_4=0x...
```

**⚠️ Never commit this file!**

## Deployment Script

### 1. Update Deploy Script

Edit `contracts/ethereum/scripts/deploy.js`:

```javascript
const hre = require("hardhat");

async function main() {
  console.log("Deploying EDSC Bridge contracts to Sepolia...\n");

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", (await deployer.getBalance()).toString(), "\n");

  // Get attester addresses from environment
  const attesters = [
    process.env.ATTESTER_0,
    process.env.ATTESTER_1,
    process.env.ATTESTER_2,
    process.env.ATTESTER_3,
    process.env.ATTESTER_4,
  ];

  console.log("Attesters:", attesters, "\n");

  // 1. Deploy EDSC Token
  console.log("1. Deploying EDSC Token...");
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = await EDSC.deploy();
  await edsc.deployed();
  console.log("   EDSC deployed to:", edsc.address);

  // 2. Deploy AttesterRegistry
  console.log("\n2. Deploying AttesterRegistry...");
  const AttesterRegistry = await hre.ethers.getContractFactory("AttesterRegistry");
  const attesterRegistry = await AttesterRegistry.deploy();
  await attesterRegistry.deployed();
  console.log("   AttesterRegistry deployed to:", attesterRegistry.address);

  // 3. Deploy EDSCMessageTransmitter
  console.log("\n3. Deploying EDSCMessageTransmitter...");
  const EDSCMessageTransmitter = await hre.ethers.getContractFactory(
    "EDSCMessageTransmitter"
  );
  const messageTransmitter = await EDSCMessageTransmitter.deploy(
    edsc.address,
    attesterRegistry.address
  );
  await messageTransmitter.deployed();
  console.log("   EDSCMessageTransmitter deployed to:", messageTransmitter.address);

  // 4. Deploy EDSCTokenMessenger
  console.log("\n4. Deploying EDSCTokenMessenger...");
  const EDSCTokenMessenger = await hre.ethers.getContractFactory("EDSCTokenMessenger");
  const tokenMessenger = await EDSCTokenMessenger.deploy(
    edsc.address,
    messageTransmitter.address
  );
  await tokenMessenger.deployed();
  console.log("   EDSCTokenMessenger deployed to:", tokenMessenger.address);

  // 5. Configure EDSC Token
  console.log("\n5. Configuring EDSC Token...");
  await edsc.setMessageTransmitter(messageTransmitter.address);
  console.log("   ✓ MessageTransmitter set");

  // 6. Register Attesters
  console.log("\n6. Registering Attesters...");
  for (let i = 0; i < attesters.length; i++) {
    await attesterRegistry.addAttester(attesters[i]);
    console.log(`   ✓ Attester ${i} registered: ${attesters[i]}`);
  }

  // 7. Set Threshold (3-of-5 for Ëtrid domain)
  console.log("\n7. Setting Signature Threshold...");
  const ETRID_DOMAIN = 2;
  const THRESHOLD = 3;
  await attesterRegistry.setThreshold(ETRID_DOMAIN, THRESHOLD);
  console.log(`   ✓ Threshold set to ${THRESHOLD}-of-${attesters.length} for domain ${ETRID_DOMAIN}`);

  // Summary
  console.log("\n" + "=".repeat(60));
  console.log("DEPLOYMENT COMPLETE!");
  console.log("=".repeat(60));
  console.log("\nContract Addresses:");
  console.log("  EDSC:                    ", edsc.address);
  console.log("  AttesterRegistry:        ", attesterRegistry.address);
  console.log("  EDSCMessageTransmitter:  ", messageTransmitter.address);
  console.log("  EDSCTokenMessenger:      ", tokenMessenger.address);
  console.log("\nConfiguration:");
  console.log("  Attesters:               ", attesters.length);
  console.log("  Threshold:               ", THRESHOLD);
  console.log("\nNext Steps:");
  console.log("  1. Verify contracts on Etherscan");
  console.log("  2. Save addresses to deployment config");
  console.log("  3. Configure attestation services");
  console.log("  4. Test bridge functionality");
  console.log("");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### 2. Deploy

```bash
cd contracts/ethereum

# Install dependencies (if not already)
npm install

# Deploy to Sepolia
npx hardhat run scripts/deploy.js --network sepolia
```

**Expected output**:
```
Deploying EDSC Bridge contracts to Sepolia...

Deploying with account: 0xYourAddress
Account balance: 1000000000000000000

1. Deploying EDSC Token...
   EDSC deployed to: 0x...

2. Deploying AttesterRegistry...
   AttesterRegistry deployed to: 0x...

3. Deploying EDSCMessageTransmitter...
   EDSCMessageTransmitter deployed to: 0x...

4. Deploying EDSCTokenMessenger...
   EDSCTokenMessenger deployed to: 0x...

5. Configuring EDSC Token...
   ✓ MessageTransmitter set

6. Registering Attesters...
   ✓ Attester 0 registered: 0x...
   ✓ Attester 1 registered: 0x...
   ✓ Attester 2 registered: 0x...
   ✓ Attester 3 registered: 0x...
   ✓ Attester 4 registered: 0x...

7. Setting Signature Threshold...
   ✓ Threshold set to 3-of-5 for domain 2

============================================================
DEPLOYMENT COMPLETE!
============================================================

Contract Addresses:
  EDSC:                     0x...
  AttesterRegistry:         0x...
  EDSCMessageTransmitter:   0x...
  EDSCTokenMessenger:       0x...
```

**⚠️ SAVE THESE ADDRESSES!** You'll need them for service configuration.

## Contract Verification

Verify contracts on Etherscan for transparency:

```bash
# Verify EDSC
npx hardhat verify --network sepolia <EDSC_ADDRESS>

# Verify AttesterRegistry
npx hardhat verify --network sepolia <ATTESTER_REGISTRY_ADDRESS>

# Verify MessageTransmitter
npx hardhat verify --network sepolia <MESSAGE_TRANSMITTER_ADDRESS> \
  "<EDSC_ADDRESS>" "<ATTESTER_REGISTRY_ADDRESS>"

# Verify TokenMessenger
npx hardhat verify --network sepolia <TOKEN_MESSENGER_ADDRESS> \
  "<EDSC_ADDRESS>" "<MESSAGE_TRANSMITTER_ADDRESS>"
```

Verified contracts will have:
- ✅ Green checkmark on Etherscan
- Readable source code
- Direct contract interaction UI

## Save Deployment Addresses

Create `deployment/ethereum/addresses.json`:

```json
{
  "network": "sepolia",
  "chainId": 11155111,
  "deployedAt": "2024-01-15T10:30:00Z",
  "deployer": "0xYourDeployerAddress",
  "contracts": {
    "EDSC": "0x...",
    "AttesterRegistry": "0x...",
    "EDSCMessageTransmitter": "0x...",
    "EDSCTokenMessenger": "0x..."
  },
  "attesters": [
    "0x...",
    "0x...",
    "0x...",
    "0x...",
    "0x..."
  ],
  "config": {
    "threshold": 3,
    "totalAttesters": 5,
    "etridDomain": 2
  }
}
```

## Post-Deployment Testing

### 1. Verify Contract State

```bash
# Check attester count
npx hardhat console --network sepolia
> const registry = await ethers.getContractAt("AttesterRegistry", "<ADDRESS>")
> await registry.getAttesterCount()
5n

# Check threshold
> await registry.getThreshold(2) // Ëtrid domain
3n
```

### 2. Test Token Operations

```javascript
// Mint some test EDSC
const edsc = await ethers.getContractAt("EDSC", "<ADDRESS>");
await edsc.mint(deployer.address, ethers.parseEther("1000"));

// Check balance
await edsc.balanceOf(deployer.address);
```

### 3. Test Burn & Send

```javascript
const tokenMessenger = await ethers.getContractAt("EDSCTokenMessenger", "<ADDRESS>");
const edsc = await ethers.getContractAt("EDSC", "<EDSC_ADDRESS>");

// Approve
await edsc.approve(tokenMessenger.address, ethers.parseEther("100"));

// Burn and send to Ëtrid
const recipient = "0x" + Buffer.from("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").toString("hex").padEnd(64, "0");
await tokenMessenger.burnAndSend(recipient, ethers.parseEther("100"));

// Check event was emitted
// (view transaction on Etherscan)
```

## Monitoring

### Setup Monitoring

Monitor contracts on Etherscan:
- Enable email notifications for contract events
- Watch for: MessageSent, MessageReceived events
- Monitor contract balance

### Setup Tenderly (Recommended)

1. Sign up: https://tenderly.co/
2. Add project
3. Import contracts (paste addresses)
4. Set up alerts for:
   - Failed transactions
   - Unexpected events
   - Low balances

## Troubleshooting

### Deployment Fails

**Error: insufficient funds**
- Get more Sepolia ETH from faucets
- Check balance: `npx hardhat console --network sepolia` → `await ethers.provider.getBalance("0xYourAddress")`

**Error: nonce too low**
- Reset nonce: Add `nonce: await ethers.provider.getTransactionCount(deployer.address)` to deploy transactions

**Error: contract already deployed**
- Use different deployment script or clean artifacts: `npx hardhat clean`

### Verification Fails

**Error: already verified**
- Contract is already verified, no action needed

**Error: constructor arguments mismatch**
- Provide correct constructor arguments to verify command
- Check deployment transaction on Etherscan for exact arguments

### Transaction Pending Too Long

- Check gas price is competitive
- View mempool: https://sepolia.etherscan.io/txs
- Increase gas price if needed

## Security Checklist

- [ ] Private keys secured (encrypted, HSM, etc.)
- [ ] Contract addresses saved securely
- [ ] Contracts verified on Etherscan
- [ ] Attesters registered correctly
- [ ] Threshold set correctly (3-of-5)
- [ ] MessageTransmitter set in EDSC token
- [ ] Test transactions successful
- [ ] Monitoring configured
- [ ] Backup deployment info

## Next Steps

1. ✅ Contracts deployed
2. → Deploy Substrate testnet
3. → Configure attestation services
4. → Configure relayer services
5. → End-to-end testing

See [`../substrate/DEPLOYMENT.md`](../substrate/DEPLOYMENT.md) for next steps.
