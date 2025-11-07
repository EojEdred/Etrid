# EDSC Reserve Vault Setup

**CRITICAL:** All EDSC tokens must be minted to the Reserve Vault address, NOT to individual wallets.

---

## What is the Reserve Vault?

**The Reserve Vault is where ALL 1 billion EDSC tokens are stored initially.**

```
When EDSC deploys:
↓
1 billion EDSC minted
↓
ALL sent to Reserve Vault (0x...)
↓
Users buy EDSC from vault with crypto
↓
Vault accumulates crypto (backing)
↓
Vault releases EDSC to buyers
```

---

## Reserve Vault Address Options

### Option 1: Foundation Multisig (RECOMMENDED for Mainnet)

**Use the 6-of-9 multisig from FOUNDATION_CHARTER.md:**

```
Address: [To be created from Foundation Board keys]
Type: 6-of-9 multisig
Signers: Foundation Board of Directors
Security: High (requires 6 of 9 signatures)
```

**How to get Foundation multisig address:**
1. See `/Users/macbook/Desktop/etrid/FOUNDATION_CHARTER.md`
2. Use the 9 board member addresses
3. Create 6-of-9 multisig on Gnosis Safe
4. Use that address as `RESERVE_VAULT`

**Creating Foundation Multisig:**
```bash
# Visit https://safe.global/
# 1. Connect wallet
# 2. Create new Safe
# 3. Add 9 Foundation Board addresses
# 4. Set threshold: 6 of 9
# 5. Deploy multisig
# 6. Copy address → Use as RESERVE_VAULT
```

---

### Option 2: Simple Wallet (For Testnet Only)

**For testing on testnet, can use simple address:**

```
Address: Your test wallet address
Type: EOA (Externally Owned Account)
Security: Low (single key)
Use: Testnet ONLY
```

**⚠️ WARNING:** Do NOT use single wallet for mainnet!

---

### Option 3: Temporary Deployment Wallet

**For initial deployment, then transfer:**

```
1. Deploy with your wallet as vault (temporary)
2. All EDSC mints to your wallet
3. After deployment, transfer ALL EDSC to proper multisig
4. Update vault address in contract
```

**Transfer command:**
```javascript
// After deployment
await edsc.transfer(MULTISIG_ADDRESS, await edsc.balanceOf(deployer.address));
```

---

## Setting Reserve Vault in Deployment

### In .env File

**Add to each chain's .env:**

```bash
# Foundation multisig (owner of contract)
FOUNDATION_MULTISIG=0x... # 6-of-9 multisig

# Reserve vault (holds all EDSC initially)
RESERVE_VAULT=0x...       # Same or different from FOUNDATION_MULTISIG
```

**Can be same or different:**
- **Same address:** Owner also controls reserve (simpler)
- **Different addresses:** Separate governance from treasury (more secure)

---

### Example Configurations

**Configuration A: Same Address (Simpler)**
```bash
# Both governance and treasury use same multisig
FOUNDATION_MULTISIG=0xF0undation123...
RESERVE_VAULT=0xF0undation123...
```

**Configuration B: Separate Addresses (More Secure)**
```bash
# Governance multisig
FOUNDATION_MULTISIG=0xF0undation123...

# Treasury multisig (different signers or threshold)
RESERVE_VAULT=0xTreasury456...
```

**Configuration C: Testnet (Testing Only)**
```bash
# Use your wallet for both (TESTNET ONLY!)
FOUNDATION_MULTISIG=0xYourWallet789...
RESERVE_VAULT=0xYourWallet789...
```

---

## Deployment Commands

### With Reserve Vault Configured

**After setting RESERVE_VAULT in .env:**

```bash
cd edsc-stablecoin/base
npm run deploy:mainnet
```

**What happens:**
```
1. Contract deploys
2. Constructor called with:
   - initialOwner = FOUNDATION_MULTISIG
   - reserveVault = RESERVE_VAULT
3. 1 billion EDSC minted to reserveVault
4. Deployment complete
```

**Check vault balance:**
```bash
# On Etherscan/block explorer
# Navigate to RESERVE_VAULT address
# Check EDSC token balance
# Should show: 1,000,000,000 EDSC
```

---

## After Deployment

### Verify Vault Received EDSC

**Check on block explorer:**
```
1. Go to Etherscan (or chain explorer)
2. Search for RESERVE_VAULT address
3. Click "Token" tab
4. Look for EDSC token
5. Balance should show: 1,000,000,000 EDSC ✅
```

**If balance is 0:**
- ❌ Wrong vault address was used
- ❌ Tokens minted to wrong address
- Need to redeploy or transfer

---

### Vault Management

**Who controls the vault?**
- Depends on vault type (EOA vs multisig)
- Multisig: Requires 6 of 9 signatures
- EOA: Single private key (risky!)

**What can vault do?**
- Transfer EDSC to users (purchases)
- Receive crypto from users (backing)
- Bridge EDSC between chains
- Emergency pause/unpause

**What vault CANNOT do:**
- Mint new EDSC (supply is fixed)
- Change contract ownership
- Modify smart contract code

---

## Security Best Practices

### DO:
- ✅ Use multisig for mainnet vault
- ✅ Require 6+ signatures for transactions
- ✅ Test on testnet first with EOA
- ✅ Audit multisig configuration
- ✅ Backup multisig keys securely
- ✅ Document all signers

### DON'T:
- ❌ Use EOA for mainnet (single point of failure)
- ❌ Use low signature threshold (e.g., 2-of-9)
- ❌ Share vault private keys
- ❌ Deploy without testing vault address
- ❌ Lose access to multisig keys

---

## Multisig Creation Guide

### Using Gnosis Safe

**Step 1: Visit Safe.global**
```
1. Go to https://safe.global/
2. Connect your wallet
3. Click "Create new Safe"
```

**Step 2: Add Signers**
```
1. Add 9 Foundation Board addresses
2. Each address gets 1 vote
3. Double-check addresses (typos = lost funds!)
```

**Step 3: Set Threshold**
```
1. Set threshold: 6 of 9
2. Requires 6 signers to approve transactions
3. Prevents single signer from draining vault
```

**Step 4: Deploy Safe**
```
1. Review configuration
2. Pay gas fee (~$50-100 on Ethereum, $1-5 on L2s)
3. Wait for deployment
4. Copy Safe address → This is your RESERVE_VAULT
```

**Step 5: Test Safe**
```
1. Send small test amount (0.01 ETH)
2. Create test transaction
3. Get 6 signers to approve
4. Execute transaction
5. Verify it worked ✅
```

---

## For Each Chain

**You need Reserve Vault on EACH chain:**

| Chain | Vault Options |
|-------|--------------|
| Base | Same multisig address (works across chains) |
| Arbitrum | Same multisig address |
| Polygon | Same multisig address |
| BSC | Same multisig address |
| Ethereum | Same multisig address |
| Solana | DIFFERENT (Solana uses different address format) |

**EVM Chains (Base, Arb, Polygon, BSC, ETH):**
- Same multisig address works on all
- Deploy multisig once, use everywhere
- Same signers control all chains

**Solana:**
- Different address format
- Need Solana multisig (Squads Protocol)
- Different process (see Solana docs)

---

## Quick Start: Testnet Deployment

**For testnet (Sepolia, Mumbai, etc.):**

```bash
# Use your own wallet as vault (testing only)
cd edsc-stablecoin/base

# Set in .env
echo "RESERVE_VAULT=0xYourWalletAddress" >> .env

# Deploy
npm run deploy:testnet

# Verify
# Check your wallet has 1B EDSC
```

**For mainnet:**

```bash
# Create multisig first!
# https://safe.global/

# Set in .env
echo "RESERVE_VAULT=0xMultisigAddress" >> .env

# Deploy
npm run deploy:mainnet

# Verify
# Check multisig has 1B EDSC
```

---

## Troubleshooting

**"ReserveVault is not set" error:**
```bash
# Add to .env:
RESERVE_VAULT=0x...
```

**Tokens minted to wrong address:**
```bash
# Check deployment script used correct variable
# Redeploy if needed, or transfer tokens
```

**Can't access multisig:**
```bash
# Make sure you're one of the signers
# Need 6 of 9 signatures to transact
# Contact other signers for approval
```

**Lost multisig access:**
```bash
# If < 6 signers available, vault is locked!
# CRITICAL: Keep signer keys secure
# This is why we use 6-of-9 (can lose 3 keys)
```

---

## Summary

**Reserve Vault = Where ALL EDSC lives initially**

**Setup:**
1. Create Foundation multisig (6-of-9)
2. Add RESERVE_VAULT to .env
3. Deploy EDSC contracts
4. Verify vault has 1 billion EDSC
5. Vault releases EDSC as users purchase

**Critical:**
- ✅ ALL tokens go to vault (not owner/team)
- ✅ Vault must be multisig for mainnet
- ✅ Test on testnet first
- ✅ Backup multisig keys securely

---

**Next:** After deployment, implement purchase/redemption functions so users can buy EDSC from the vault!
