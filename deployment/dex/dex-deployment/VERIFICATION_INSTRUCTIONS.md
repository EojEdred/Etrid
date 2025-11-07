# Contract Verification Instructions

**Date**: November 7, 2025
**Status**: Both BSC and Polygon contracts need verification

---

## ⚠️ API Keys Required

To verify contracts, you need **free API keys** from the block explorers:

### 1. Get BSCScan API Key (FREE)

1. Go to: https://bscscan.com/register
2. Create account (email + password)
3. Verify your email
4. Go to: https://bscscan.com/myapikey
5. Click "Add" to create new API key
6. Copy the API key

### 2. Get PolygonScan API Key (FREE)

1. Go to: https://polygonscan.com/register
2. Create account (email + password)
3. Verify your email
4. Go to: https://polygonscan.com/myapikey
5. Click "Add" to create new API key
6. Copy the API key

---

## 📝 Update .env Files

### BSC .env file:
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc
nano .env
```

Replace this line:
```
BSCSCAN_API_KEY=your_bscscan_api_key
```

With:
```
BSCSCAN_API_KEY=YOUR_ACTUAL_KEY_HERE
```

### Polygon .env file:
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon
nano .env
```

Add or update:
```
POLYGONSCAN_API_KEY=YOUR_ACTUAL_KEY_HERE
```

---

## 🚀 Verification Commands

### Verify BSC Contract

```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc

# Verify the deployed contract
npx hardhat verify --network bscMainnet 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3

# Expected output:
# Successfully submitted source code for contract
# contracts/EtridBSC.sol:EtridBSC at 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
# for verification on the block explorer. Waiting for verification result...
#
# Successfully verified contract EtridBSC on BscScan.
# https://bscscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code
```

### Verify Polygon Contract

```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon

# First install dependencies if needed
npm install

# Verify the deployed contract
npx hardhat verify --network polygon 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3

# Expected output:
# Successfully submitted source code for contract
# contracts/EtridPolygon.sol:EtridPolygon at 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
# for verification on the block explorer. Waiting for verification result...
#
# Successfully verified contract EtridPolygon on PolygonScan.
# https://polygonscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code
```

---

## ✅ Verify Success

After running the commands, check the block explorers:

### BSC Contract:
Visit: https://bscscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code

You should see:
- ✅ Green checkmark next to "Contract"
- "Contract Source Code Verified (Exact Match)"
- Full Solidity source code visible
- "Read Contract" and "Write Contract" tabs working

### Polygon Contract:
Visit: https://polygonscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code

You should see:
- ✅ Green checkmark next to "Contract"
- "Contract Source Code Verified (Exact Match)"
- Full Solidity source code visible
- "Read Contract" and "Write Contract" tabs working

---

## 🐛 Troubleshooting

### Error: "Invalid API Key"
**Solution**: Double-check you copied the API key correctly from BSCScan/PolygonScan

### Error: "Already Verified"
**Solution**: Great! Someone already verified it. Check the block explorer.

### Error: "Compilation failed"
**Solution**:
```bash
# Clean and recompile
npx hardhat clean
npx hardhat compile
# Try verification again
```

### Error: "Constructor arguments required"
**Solution**: This contract has no constructor arguments (just mints to msg.sender), so this shouldn't happen. But if it does:
```bash
npx hardhat verify --network bscMainnet 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3 --constructor-args ""
```

### Error: "Network not found"
**Solution**: Make sure you're in the correct directory:
- BSC: `/Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc`
- Polygon: `/Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon`

---

## 📊 Contract Details for Reference

### BSC Deployment
```
Contract Address: 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
Contract Name:    EtridBSC
Compiler:         0.8.20
Optimization:     Yes (200 runs)
Constructor Args: None
Chain ID:         56
Block:            66,930,168
```

### Polygon Deployment
```
Contract Address: 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
Contract Name:    EtridPolygon (or same as BSC)
Compiler:         0.8.20
Optimization:     Yes (200 runs)
Constructor Args: None
Chain ID:         137
```

---

## 🔒 After Verification

Once both contracts are verified:

1. ✅ **Update docs/DEPLOYED_CONTRACTS_STATUS.md**
   - Change status from ❌ NOT VERIFIED to ✅ VERIFIED

2. ✅ **Submit to token tracking sites**:
   - CoinGecko: https://www.coingecko.com/en/coins/new
   - CoinMarketCap: https://support.coinmarketcap.com/hc/en-us/articles/360043659351
   - Both require verified contracts

3. ✅ **Create liquidity pools**:
   - PancakeSwap (BSC) - after getting more BNB
   - QuickSwap (Polygon) - ready now!

4. ✅ **Commit the verification status**:
   ```bash
   git add secrets/DEPLOYED_CONTRACTS_STATUS.md
   git commit -m "docs: Update contract verification status"
   ```

---

## 📞 Support

**BSCScan Support**: https://bscscan.com/contactus
**PolygonScan Support**: https://polygonscan.com/contactus

**Hardhat Verification Docs**: https://hardhat.org/hardhat-runner/plugins/nomicfoundation-hardhat-verify

---

## ⏱️ Estimated Time

- Getting API keys: 5 minutes
- Updating .env files: 2 minutes
- Running verification: 3 minutes per chain
- **Total**: ~15 minutes

---

## Quick Start (Copy-Paste)

```bash
# 1. Get API keys first (see above)

# 2. Update BSC .env
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc
nano .env
# Paste your BSCSCAN_API_KEY

# 3. Verify BSC
npx hardhat verify --network bscMainnet 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3

# 4. Update Polygon .env
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon
npm install
nano .env
# Paste your POLYGONSCAN_API_KEY

# 5. Verify Polygon
npx hardhat verify --network polygon 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3

# 6. Check results
echo "BSC: https://bscscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code"
echo "Polygon: https://polygonscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code"
```

Done! 🎉
