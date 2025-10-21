# EDSC Bridge User Guide

Complete guide for using the Ëtrid Dollar Stablecoin (EDSC) cross-chain bridge to transfer tokens between Ethereum and Ëtrid.

## What is the EDSC Bridge?

The EDSC Bridge allows you to transfer EDSC tokens between:
- **Ethereum** (Sepolia testnet) ↔ **Ëtrid** (Substrate testnet)

### Key Features

✅ **Secure**: M-of-N attester signatures (3-of-5 threshold)
✅ **Decentralized**: Multiple independent attesters
✅ **Permissionless**: Anyone can relay messages
✅ **Fast**: Transfers complete in 2-5 minutes
✅ **Transparent**: All operations on-chain

---

## Getting Started

### Prerequisites

#### 1. Wallets

**Ethereum Wallet** (Choose one):
- MetaMask (Browser extension)
- Rainbow Wallet
- Coinbase Wallet
- Any Ethereum wallet

**Ëtrid Wallet** (Choose one):
- Polkadot.js Extension
- Talisman
- SubWallet
- Any Substrate wallet

#### 2. Testnet Tokens

**Get Sepolia ETH** (for gas):
- [Alchemy Sepolia Faucet](https://sepoliafaucet.com/)
- [Infura Sepolia Faucet](https://www.infura.io/faucet/sepolia)

**Get EDSC Testnet Tokens**:
- Join Discord: [discord.gg/etrid](https://discord.gg/etrid)
- Request in #faucet channel
- Minimum: 100 EDSC for testing

#### 3. Add Networks

**Add Sepolia to MetaMask**:
```
Network Name: Sepolia
RPC URL: https://rpc.sepolia.org
Chain ID: 11155111
Currency Symbol: ETH
Block Explorer: https://sepolia.etherscan.io
```

**Add Ëtrid to Polkadot.js**:
```
Network: Ember Testnet
Endpoint: wss://ember-rpc.etrid.io
```

---

## Transfer Ethereum → Ëtrid

Send EDSC from Ethereum to Ëtrid.

### Step 1: Connect Wallet

1. Visit bridge interface (ember-bridge.etrid.io)
2. Click "Connect Ethereum Wallet"
3. Approve connection in MetaMask
4. Click "Connect Ëtrid Wallet"
5. Approve connection in Polkadot.js

### Step 2: Enter Transfer Details

1. Select "Ethereum → Ëtrid"
2. Enter amount (e.g., 100 EDSC)
3. Enter your Ëtrid address (or auto-fill from connected wallet)
4. Review details:
   - **Amount**: 100 EDSC
   - **From**: Your Ethereum address
   - **To**: Your Ëtrid address
   - **Fee**: ~$5-20 (Ethereum gas)
   - **Time**: 2-5 minutes

### Step 3: Approve EDSC

1. Click "Approve EDSC"
2. Confirm transaction in MetaMask
3. Wait for confirmation (~15 seconds)

### Step 4: Burn & Send

1. Click "Transfer"
2. Confirm transaction in MetaMask
3. Transaction hash will appear

### Step 5: Wait for Attestation

1. Status changes to "Waiting for signatures..."
2. Attesters detect burn event
3. 3-of-5 attesters sign message (~30-60 seconds)
4. Status changes to "Ready to relay"

### Step 6: Automatic Relay

1. Permissionless relayer picks up signed attestation
2. Submits to Ëtrid chain
3. Status changes to "Relaying..."
4. EDSC arrives in your Ëtrid wallet! (~1-2 minutes)

### Step 7: Verify Receipt

1. Check your Ëtrid wallet balance
2. View transaction on block explorer
3. Done! ✅

---

## Transfer Ëtrid → Ethereum

Send EDSC from Ëtrid to Ethereum.

### Step 1: Connect Wallets

1. Visit bridge interface
2. Connect both Ëtrid and Ethereum wallets

### Step 2: Enter Transfer Details

1. Select "Ëtrid → Ethereum"
2. Enter amount (e.g., 50 EDSC)
3. Enter your Ethereum address
4. Review details:
   - **Amount**: 50 EDSC
   - **From**: Your Ëtrid address
   - **To**: Your Ethereum address
   - **Fee**: ~0.01 EDSC (Ëtrid tx fee)
   - **Time**: 2-5 minutes

### Step 3: Burn & Send

1. Click "Transfer"
2. Confirm transaction in Polkadot.js extension
3. Transaction hash will appear

### Step 4: Wait for Attestation

1. Status changes to "Waiting for signatures..."
2. Attesters detect burn event on Ëtrid
3. 3-of-5 attesters sign message
4. Status changes to "Ready to relay"

### Step 5: Automatic Relay

1. Relayer submits to Ethereum
2. Status changes to "Relaying..."
3. EDSC arrives in your Ethereum wallet!

### Step 6: Verify Receipt

1. Check balance on Etherscan
2. View in MetaMask
3. Done! ✅

---

## Using Without Web Interface

Advanced users can interact with contracts directly.

### Ethereum → Ëtrid (Direct)

Using ethers.js:

```javascript
const { ethers } = require('ethers');

// Setup
const provider = new ethers.JsonRpcProvider('https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY');
const wallet = new ethers.Wallet('YOUR-PRIVATE-KEY', provider);

// Contract addresses
const EDSC_ADDRESS = '0x...';
const TOKEN_MESSENGER_ADDRESS = '0x...';

// ABIs
const edscAbi = ['function approve(address spender, uint256 amount) external returns (bool)'];
const messengerAbi = ['function burnAndSend(bytes calldata recipient, uint256 amount) external returns (uint64)'];

// Contracts
const edsc = new ethers.Contract(EDSC_ADDRESS, edscAbi, wallet);
const messenger = new ethers.Contract(TOKEN_MESSENGER_ADDRESS, messengerAbi, wallet);

// Approve
const approveTx = await edsc.approve(TOKEN_MESSENGER_ADDRESS, ethers.parseEther('100'));
await approveTx.wait();

// Burn and send
const recipientBytes = ethers.toUtf8Bytes('YOUR_ETRID_ADDRESS'.padEnd(32, '\0'));
const burnTx = await messenger.burnAndSend(recipientBytes, ethers.parseEther('100'));
const receipt = await burnTx.wait();

console.log('Burned! TX:', receipt.hash);
console.log('Now wait for attesters to sign and relayer to submit');
```

### Ëtrid → Ethereum (Direct)

Using Polkadot.js:

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

// Setup
const provider = new WsProvider('wss://ember-rpc.etrid.io');
const api = await ApiPromise.create({ provider });

const keyring = new Keyring({ type: 'sr25519' });
const account = keyring.addFromUri('YOUR_SEED_PHRASE');

// Encode Ethereum address as bytes
const ethAddress = '0xYourEthereumAddress';
const addressBytes = ethers.getBytes(ethAddress);

// Burn and send
const tx = api.tx.tokenMessenger.burnAndSend(
  0, // destination domain (Ethereum)
  Array.from(addressBytes),
  ethers.parseEther('50').toString()
);

await tx.signAndSend(account, ({ status }) => {
  if (status.isInBlock) {
    console.log('Burned! Block:', status.asInBlock.toString());
    console.log('Now wait for attesters to sign and relayer to submit');
  }
});
```

---

## Understanding the Process

### What Happens During a Transfer?

#### 1. Burn Phase (5-30 seconds)

**Ethereum → Ëtrid**:
- You call `TokenMessenger.burnAndSend()` on Ethereum
- EDSC is burned (destroyed)
- `MessageSent` event emitted
- Nonce assigned

**Ëtrid → Ethereum**:
- You call `tokenMessenger.burnAndSend()` on Ëtrid
- EDSC is burned
- `BurnMessageSent` event emitted
- Nonce assigned

#### 2. Attestation Phase (30-90 seconds)

- 5 attestation services monitor both chains
- Each detects the burn event
- Each signs the message independently
- Signatures collected in attestation store
- When 3-of-5 signatures collected → Ready!

#### 3. Relay Phase (30-120 seconds)

- Relayer polls attestation services
- Finds ready attestation (3+ signatures)
- Submits to destination chain:
  - **To Ethereum**: Calls `MessageTransmitter.receiveMessage()`
  - **To Ëtrid**: Calls `attestation.receiveMessage()`
- EDSC is minted to recipient
- Transfer complete!

### Why Multiple Attesters?

**Security**: No single point of failure
- Need 3-of-5 attesters to approve
- If 1-2 attesters are compromised, bridge still secure
- If 1-2 attesters go offline, bridge still works

**Decentralization**: Independent operators
- Each attester runs their own infrastructure
- Different geographic locations
- Different parties (foundations, companies, DAOs)

### Why Permissionless Relayers?

**Decentralization**: Anyone can operate
- No gatekeeping
- Competitive relaying
- Censorship resistant

**Redundancy**: Multiple relayers
- If one fails, others continue
- Prevents single point of failure

**Economics**: Market-driven
- Relayers compete on speed
- Fastest relay gets included
- Users benefit from competition

---

## Fees & Limits

### Ethereum → Ëtrid

| Item | Cost |
|------|------|
| Ethereum gas (burn) | ~$5-20 (varies) |
| Ëtrid transaction (relay) | ~$0.01 |
| Bridge fee | $0 (testnet) |
| **Total** | **~$5-20** |

**Limits**:
- Minimum: 1 EDSC
- Maximum: 100,000 EDSC per transaction (testnet)
- Daily limit: 1,000,000 EDSC (testnet)

### Ëtrid → Ethereum

| Item | Cost |
|------|------|
| Ëtrid transaction (burn) | ~$0.01 |
| Ethereum gas (relay) | ~$10-30 (varies) |
| Bridge fee | $0 (testnet) |
| **Total** | **~$10-30** |

**Note**: Relayers pay Ethereum gas, so Ëtrid → Ethereum may be subsidized on testnet.

---

## Monitoring Your Transfer

### Track Transfer Status

1. **Get Transaction Hash**: Save the TX hash from your burn transaction

2. **Check Attestation Status**:
   ```bash
   # By transaction/nonce
   curl https://attestation-0.etrid.io/attestation/DOMAIN/NONCE

   # Example: Ethereum (domain 0), nonce 42
   curl https://attestation-0.etrid.io/attestation/0/42
   ```

3. **View on Block Explorers**:
   - Ethereum: https://sepolia.etherscan.io/tx/YOUR_TX_HASH
   - Ëtrid: https://ember-explorer.etrid.io

### Transfer Status Meanings

- **Burning**: Initial transaction confirming
- **Waiting for signatures**: Attesters are signing (30-90s)
- **Ready to relay**: Threshold met, waiting for relayer
- **Relaying**: Relayer submitting to destination
- **Complete**: EDSC received on destination!
- **Failed**: Error occurred (contact support)

---

## Troubleshooting

### Transfer Stuck on "Burning"

**Cause**: Ethereum transaction not confirmed

**Solutions**:
- Check transaction on Etherscan
- Increase gas price if pending
- Wait for confirmation (up to 5 minutes during high congestion)

### Transfer Stuck on "Waiting for Signatures"

**Cause**: Attesters haven't signed yet

**Solutions**:
- Wait (can take up to 2 minutes)
- Check attestation service status:
  ```bash
  curl https://attestation-0.etrid.io/health
  ```
- If all attesters down, contact team on Discord

### Transfer Stuck on "Ready to Relay"

**Cause**: No relayers active or high gas prices

**Solutions**:
- Wait (relayers poll every 30-60 seconds)
- Check if relayers are active (Discord #bridge-status)
- Manually relay (advanced):
  ```bash
  # Get attestation
  curl https://attestation-0.etrid.io/attestation/0/NONCE

  # Submit to destination chain using contract
  ```

### "Insufficient Balance" Error

**Cause**: Not enough tokens or gas

**Solutions**:
- **EDSC**: Get more from faucet
- **ETH**: Get from Sepolia faucet
- Check balances before transferring

### "Approval Failed" Error

**Cause**: Approval transaction reverted

**Solutions**:
- Check you have EDSC to approve
- Check gas price is set
- Try again with higher gas

---

## Safety Tips

### ✅ Do's

- ✅ Start with small amounts (10-100 EDSC)
- ✅ Double-check recipient address
- ✅ Save transaction hashes
- ✅ Wait for full confirmation
- ✅ Use official bridge interface
- ✅ Verify contract addresses

### ❌ Don'ts

- ❌ Don't send to exchange addresses
- ❌ Don't transfer more than you can afford to lose (testnet!)
- ❌ Don't use untrusted bridge interfaces
- ❌ Don't share your private keys
- ❌ Don't rush during high gas periods

---

## FAQ

### How long does a transfer take?

**Typical**: 2-5 minutes
**Range**: 1-10 minutes depending on:
- Network congestion
- Number of confirmations required
- Relayer activity

### Can I cancel a transfer?

**No**. Once the burn transaction is confirmed, it cannot be reversed. The EDSC will arrive on the destination chain once attesters sign and relayers submit.

### What if I entered the wrong address?

**Ethereum → Ëtrid**: EDSC will be minted to the address you specified. If it's wrong, those tokens may be lost.

**Solution**: Always double-check addresses!

### Is there a minimum/maximum amount?

**Minimum**: 1 EDSC
**Maximum**: 100,000 EDSC per transaction (testnet limit)

Mainnet limits will be higher.

### Do I need both wallets connected?

**No**, but it's convenient. You can:
1. Send from Ethereum without connecting Ëtrid wallet (enter address manually)
2. Send from Ëtrid without connecting Ethereum wallet (enter address manually)

### Can I bridge other tokens?

**No**, only EDSC is supported. In the future, additional tokens may be added.

### What happens if attesters disagree?

Attesters independently verify the burn event. If they disagree:
- Valid burns will get 5/5 signatures
- Invalid attempts will get 0/5 signatures
- Threshold is 3-of-5, so bridge is secure even if 2 attesters malfunction

### Can I run my own relayer?

**Yes!** Relaying is permissionless. See [Relayer Deployment Guide](deployment/services/RELAYER_DEPLOYMENT.md).

**Benefits**:
- Contribute to decentralization
- Potentially earn fees (mainnet)
- Learn about cross-chain infrastructure

---

## Support

### Get Help

- **Discord**: [discord.gg/etrid](https://discord.gg/etrid) (#bridge-support)
- **Telegram**: [t.me/EtridOfficial](https://t.me/EtridOfficial)
- **Email**: support@etrid.io
- **Twitter**: [@EtridMultichain](https://twitter.com/EtridMultichain)

### Report Issues

- **GitHub**: [github.com/etrid/etrid/issues](https://github.com/etrid/etrid/issues)
- Tag with: `bridge`, `testnet`

### Status Page

Check service status:
- Attesters: https://status.etrid.io
- Relayers: https://status.etrid.io
- RPC nodes: https://status.etrid.io

---

## Learn More

- **Technical Documentation**: [deployment/README.md](deployment/README.md)
- **Smart Contracts**: [contracts/ethereum/](contracts/ethereum/)
- **Substrate Pallets**: [05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/pallets/](05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/pallets/)
- **Services**: [services/](services/)

---

## Testnet Disclaimer

⚠️ **This is testnet software**:
- Not for production use
- Tokens have no real value
- May have bugs or downtime
- Used for testing only

Do not use with real funds until mainnet launch!

---

<p align="center">
  <strong>Happy Bridging! 🌉</strong>
</p>

<p align="center">
  <sub>Built by the Ëtrid community</sub>
</p>
