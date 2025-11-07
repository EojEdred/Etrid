# Quick Action Guide - Contract Verification

**Date**: November 7, 2025
**Time Required**: 15 minutes
**Cost**: FREE

---

## 🎯 What You Need to Do

Your contracts are deployed but **NOT VERIFIED**. This is blocking:
- ❌ CoinGecko listing
- ❌ CoinMarketCap listing
- ❌ User trust (can't see source code)
- ❌ Some DEX listings

---

## 📋 Checklist

### Step 1: Get API Keys (5 minutes) ⏱️

**BSCScan API Key:**
1. Go to: https://bscscan.com/register
2. Create account → verify email
3. Go to: https://bscscan.com/myapikey
4. Click "Add" → copy the key

**PolygonScan API Key:**
1. Go to: https://polygonscan.com/register
2. Create account → verify email
3. Go to: https://polygonscan.com/myapikey
4. Click "Add" → copy the key

### Step 2: Update .env Files (2 minutes) ⏱️

**BSC:**
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc
nano .env
```
Replace: `BSCSCAN_API_KEY=your_bscscan_api_key`
With: `BSCSCAN_API_KEY=YOUR_ACTUAL_KEY`

**Polygon:**
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon
nano .env
```
Add or replace: `POLYGONSCAN_API_KEY=YOUR_ACTUAL_KEY`

### Step 3: Verify Contracts (6 minutes) ⏱️

**Verify BSC:**
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/bsc
npx hardhat verify --network bscMainnet 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
```

**Verify Polygon:**
```bash
cd /Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/polygon
npm install  # if not already done
npx hardhat verify --network polygon 0x1A065196152C2A70e54AC06D3a3433e3D8606eF3
```

### Step 4: Verify Success (2 minutes) ⏱️

Check block explorers for ✅ green checkmark:
- BSC: https://bscscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code
- Polygon: https://polygonscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3#code

---

## ✅ After Verification

Once verified, you can:
1. ✅ Submit to CoinGecko: https://www.coingecko.com/en/coins/new
2. ✅ Submit to CoinMarketCap: https://coinmarketcap.com/request/
3. ✅ Create liquidity pools with confidence
4. ✅ Update `secrets/DEPLOYED_CONTRACTS_STATUS.md` status

---

## 📚 Detailed Guide

For troubleshooting and full instructions, see:
`/Users/macbook/Desktop/etrid/deployment/dex/dex-deployment/VERIFICATION_INSTRUCTIONS.md`

---

## 🔑 Your Deployed Contracts

| Chain | Address | Status | Explorer |
|-------|---------|--------|----------|
| **BSC** | `0x1A065196152C2A70e54AC06D3a3433e3D8606eF3` | ❌ Not Verified | [BSCScan](https://bscscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3) |
| **Polygon** | `0x1A065196152C2A70e54AC06D3a3433e3D8606eF3` | ❌ Not Verified | [PolygonScan](https://polygonscan.com/address/0x1A065196152C2A70e54AC06D3a3433e3D8606eF3) |
| **Solana** | `8XdUXcvWUYnyKg6hR5yEDFHJqhqD2CbizLURVQCXNppq` | ✅ N/A | [Solscan](https://solscan.io/token/8XdUXcvWUYnyKg6hR5yEDFHJqhqD2CbizLURVQCXNppq) |

---

**DO THIS NOW!** ⏰

Verification is quick, free, and required for token listings.
