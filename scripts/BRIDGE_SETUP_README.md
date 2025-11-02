# FlareChain Bridge Setup Guide

This directory contains Node.js scripts for setting up the ETR lock account and configuring cross-chain bridges.

## Prerequisites

- Node.js v16+ installed
- Access to FlareChain mainnet (ws://98.71.91.84:9944)
- Foundation sudo account seed phrase

## Installation

```bash
cd /Users/macbook/Desktop/etrid/scripts
npm install
```

## Scripts

### 1. Set Lock Account (`set-lock-account.js`)

Sets the ETR lock account for cross-chain bridging.

### 2. Check Lock Status (`check-lock-status.js`)

Displays current lock account configuration and locked balances.

---

## Usage

### Option A: Create Dedicated Bridge Account (Recommended)

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Set with foundation sudo account
SUDO_SEED="your foundation multisig seed phrase" \
LOCK_ACCOUNT_TYPE="dedicated" \
node set-lock-account.js
```

**Output:**
```
🔐 ETR Lock Account Setup
========================

📡 Connecting to FlareChain: ws://98.71.91.84:9944
✅ Connected to FlareChain Etrid FlareChain
   Version: 0.1.0

🔑 Sudo Account: 5GizziFoundation...

📦 Creating new dedicated bridge lock account...
   Address: 5EtrBridgeLock...
   ⚠️  SAVE THIS SEED: //EtrBridgeLock

🔍 Checking current lock account configuration...
   Current: None (not set)

🚀 Setting lock account...
   Transaction status: Ready
   Transaction status: InBlock
   ✅ Included in block: 0x1234...
   ✅ Finalized in block: 0x1234...

🎉 Success! Lock account set successfully

🔍 Verifying lock account...
✅ Lock account verified: 5EtrBridgeLock...

📊 Current Lock Status:
   Total Locked: 0 ETR
   Lock Account Balance: 0 ETR

✅ Setup complete!

📝 Next steps:
   1. Fund the lock account with some ETR for transaction fees
   2. Deploy PBC smart contracts on external chains
   3. Configure and start the relayer service
   4. Test bridge with small amount (100 ETR)
```

---

### Option B: Use Foundation Multisig

```bash
SUDO_SEED="your foundation multisig seed phrase" \
LOCK_ACCOUNT_TYPE="multisig" \
node set-lock-account.js
```

This uses your foundation multisig account as the lock account.

---

### Option C: Use Specific Address

```bash
SUDO_SEED="your foundation multisig seed phrase" \
LOCK_ACCOUNT_TYPE="address" \
LOCK_ACCOUNT_ADDRESS="5YourCustomAddress..." \
node set-lock-account.js
```

---

## Check Lock Status

```bash
node check-lock-status.js
```

**Output:**
```
🔍 Checking ETR Lock Status

📡 Connected to: Etrid FlareChain
   Version: 0.1.0

🔐 Lock Account:
   Address: 5EtrBridgeLock...
   Balance: 10.5 ETR

💰 Total Locked: 0 ETR

📊 Locked per Chain:
   (No locks yet)
```

---

## After Setup

### 1. Fund the Lock Account

```bash
# Via Polkadot.js Apps or command line
# Send 100-1000 ETR to the lock account for transaction fees
```

### 2. Verify Configuration

```bash
node check-lock-status.js
```

### 3. Deploy External Contracts

See: `/Users/macbook/Desktop/etrid/dex-deployment/`

### 4. Test Bridge

```bash
# Via Polkadot.js Apps:
# Developer → Extrinsics
# ethBridge.bridge_etr_to_ethereum(
#   amount: 100000000000000000000,  # 100 ETR
#   destination: "0xYourEthAddress"
# )
```

---

## Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SUDO_SEED` | Foundation sudo account seed phrase | - | Yes |
| `LOCK_ACCOUNT_TYPE` | Type of lock account: `dedicated`, `multisig`, `address` | `dedicated` | No |
| `LOCK_ACCOUNT_ADDRESS` | Address to use (if type=address) | - | Only if type=address |
| `FLARECHAIN_WS` | FlareChain WebSocket URL | `ws://98.71.91.84:9944` | No |

---

## Security Notes

⚠️ **IMPORTANT:**

1. **NEVER share your SUDO_SEED** - This controls the entire chain
2. **SAVE the lock account seed** (`//EtrBridgeLock`) - Needed for emergency recovery
3. **Fund lock account** with minimal ETR (100-1000 for fees)
4. **Test with small amounts** first (100 ETR)
5. **Monitor locked balances** regularly

---

## Troubleshooting

### Error: "SUDO_SEED environment variable not set"
```bash
# Make sure to provide the seed phrase
SUDO_SEED="your seed phrase" node set-lock-account.js
```

### Error: "Lock account is already set"
The script will ask if you want to overwrite. Type `yes` to confirm.

### Error: "Connection refused"
Check that FlareChain node is running:
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://98.71.91.84:9944
```

---

## Files

- `set-lock-account.js` - Set lock account script
- `check-lock-status.js` - Check lock status script
- `package.json` - Node.js dependencies
- `BRIDGE_SETUP_README.md` - This file

---

## Next Steps

After setting the lock account:

1. ✅ Lock account configured
2. [ ] Fund lock account with ETR
3. [ ] Deploy PBC contracts (Base, Arbitrum, Polygon, BSC)
4. [ ] Configure relayer service
5. [ ] Test bridge with 100 ETR
6. [ ] Monitor lock status
7. [ ] Open to public usage

---

For more information, see:
- `/Users/macbook/Desktop/etrid/pallets/pallet-etr-lock/BRIDGE_INTEGRATION_GUIDE.md`
- `/Users/macbook/Desktop/etrid/pallets/pallet-etr-lock/IMPLEMENTATION_STATUS.md`
