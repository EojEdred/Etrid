# Security Remediation Guide

**Date**: November 6, 2025
**Status**: IMMEDIATE ACTION REQUIRED
**Related**: [SECURITY_AUDIT_REPORT.md](./SECURITY_AUDIT_REPORT.md)

---

## Quick Summary

The security audit identified that **all deployment .env files are using the same private key**, and **unified-contracts.env is using a well-known Hardhat test key**. This needs to be fixed immediately before any mainnet deployments.

---

## Immediate Actions

### Step 1: Generate New Unique Keys

Run the key generation script:

```bash
cd /Users/macbook/Desktop/etrid
./scripts/generate-deployment-keys.sh
```

This will create a `generated-keys-TIMESTAMP/` directory with:
- Unique key for each blockchain network
- Master key list with all keys
- Instructions for next steps

### Step 2: Update .env Files

For each blockchain network:

```bash
# Ethereum
cd deployment/dex/dex-deployment/ethereum
nano .env
# Replace PRIVATE_KEY value with the key from generated-keys-*/ethereum-key.txt

# BSC
cd ../bsc
nano .env
# Replace PRIVATE_KEY value with the key from generated-keys-*/bsc-key.txt

# Polygon
cd ../polygon
nano .env
# Replace PRIVATE_KEY value with the key from generated-keys-*/polygon-key.txt

# Arbitrum
cd ../arbitrum
nano .env
# Replace PRIVATE_KEY value with the key from generated-keys-*/arbitrum-key.txt

# Base
cd ../base
nano .env
# Replace PRIVATE_KEY value with the key from generated-keys-*/base-key.txt
```

### Step 3: Update Unified Contracts

```bash
cd /Users/macbook/Desktop/etrid/secrets
nano deployment-env/unified-contracts.env
# Replace DEPLOYER_PRIVATE_KEY value with the key from generated-keys-*/unified-contracts-key.txt
```

### Step 4: Secure Generated Keys

1. **Store in password manager**:
   - Open `generated-keys-*/MASTER_KEY_LIST.md`
   - Copy each key to your password manager (1Password, Bitwarden, etc.)
   - Add labels: "Etrid Ethereum Deployment Key", etc.

2. **Create encrypted backup**:
   ```bash
   # Encrypt with GPG
   gpg -c generated-keys-*/MASTER_KEY_LIST.md
   # This creates MASTER_KEY_LIST.md.gpg

   # Store encrypted file in secure location
   # Then delete the plain text version
   ```

3. **Delete generated files**:
   ```bash
   # After storing in password manager
   rm -rf generated-keys-*/
   ```

### Step 5: Verify

```bash
# Verify each .env file has a unique key
cd /Users/macbook/Desktop/etrid
grep -h "PRIVATE_KEY=" deployment/dex/dex-deployment/*/.env | sort -u | wc -l

# Should output: 5
# (One unique key for each of the 5 chains)
```

---

## Current Problematic Keys

### Key Being Reused Everywhere

```
0x1b4734300c70328ac73f7b7bda27fca85c11ec6cebfd56fb77f147cad5d3faed
```

**Address**: `0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed`

**Action Required**:
1. Check if any funds exist at this address on ANY chain
2. If funds exist, transfer them to a secure address
3. Replace this key in ALL .env files
4. Never use this key again

**Check for funds**:
```bash
# Ethereum
cast balance 0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed --rpc-url https://eth.llamarpc.com

# BSC
cast balance 0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed --rpc-url https://bsc-dataseed.bnbchain.org

# Polygon
cast balance 0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed --rpc-url https://polygon-rpc.com

# Arbitrum
cast balance 0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed --rpc-url https://arb1.arbitrum.io/rpc

# Base
cast balance 0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed --rpc-url https://mainnet.base.org
```

### Hardhat Test Key in Unified Contracts

```
ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

**Address**: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`

**This is the first account from Hardhat's default test mnemonic.**

⚠️ **NEVER use this key for ANY deployment other than local testing!**

**Action Required**:
1. Replace immediately in `secrets/deployment-env/unified-contracts.env`
2. Verify no contracts deployed with this key on mainnet
3. If any contracts exist, emergency transfer ownership to multisig

**Check for contracts deployed by this address**:
- Etherscan: https://etherscan.io/address/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
- BSCScan: https://bscscan.com/address/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
- PolygonScan: https://polygonscan.com/address/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

---

## For Local Development

If you need these keys for **LOCAL TESTING ONLY** (e.g., Hardhat, Anvil, Ganache):

1. **Create separate .env.local files**:
   ```bash
   # Example
   cd deployment/dex/dex-deployment/ethereum
   cp .env.example .env.local
   # Use test keys in .env.local for local testing
   ```

2. **Update deployment scripts** to use .env.local when deploying to localhost

3. **NEVER use .env.local keys for testnet or mainnet**

---

## Verification Checklist

After remediation:

- [ ] New unique keys generated (5 chain keys + 1 unified key = 6 total)
- [ ] All .env files updated with new keys
- [ ] Keys stored in password manager
- [ ] Encrypted backup created
- [ ] Generated keys directory deleted
- [ ] Old keys checked for funds (and transferred if needed)
- [ ] Verified unique keys using `grep | sort -u | wc -l` command
- [ ] No Hardhat test key in any .env file
- [ ] Team notified of key rotation

---

## Timeline

**This remediation should be completed within 24 hours.**

1. **Hour 0**: Generate new keys
2. **Hour 1**: Update all .env files
3. **Hour 2**: Store keys securely and verify
4. **Hour 4**: Check old addresses for funds
5. **Hour 8**: Complete verification checklist
6. **Hour 24**: Confirm all old keys retired

---

## Questions?

If you have questions about this remediation:
1. Review [SECURITY_AUDIT_REPORT.md](./SECURITY_AUDIT_REPORT.md)
2. Review [DEPLOYMENT_SECURITY_CHECKLIST.md](./DEPLOYMENT_SECURITY_CHECKLIST.md)
3. Contact the security team

**DO NOT proceed with ANY mainnet deployment until this remediation is complete.**
