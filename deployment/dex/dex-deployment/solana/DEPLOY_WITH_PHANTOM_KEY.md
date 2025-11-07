# 🔑 Deploy Solana Token Using Your Phantom Wallet Key

**Status:** Your Phantom private key is ready to use for deployment

---

## Quick Start (2 Options)

### Option 1: Import Key to Solana CLI (Recommended)

**Step 1: Install Solana CLI**

```bash
# Try this command:
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add to PATH:
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation:
solana --version
```

**Step 2: Import Your Phantom Key**

Your Phantom private key is in **base58 format**. Convert it to keypair format:

```bash
# Create the keypair directory
mkdir -p ~/.config/solana

# You have two ways to import:
```

**Method A: Using Phantom's Export Feature (Easiest)**
1. Open Phantom wallet
2. Click Settings → Security & Privacy
3. Export Private Key
4. Copy the **JSON array format** (looks like: [123,45,67,...])
5. Save to `~/.config/solana/id.json`:

```bash
echo '[YOUR_JSON_ARRAY_HERE]' > ~/.config/solana/id.json
chmod 600 ~/.config/solana/id.json
```

**Method B: Convert Base58 Key to JSON**

Your base58 private key from Phantom needs to be converted. Here's a simple Node.js script:

```bash
# Install required package
npm install -g bs58

# Create conversion script
cat > /tmp/convert-key.js << 'EOF'
const bs58 = require('bs58');
const fs = require('fs');

// Your Phantom private key (base58)
const base58Key = 'YOUR_BASE58_KEY_HERE';

// Convert to byte array
const secretKey = bs58.decode(base58Key);
const keyArray = Array.from(secretKey);

// Save as JSON
const keypairJson = JSON.stringify(keyArray);
fs.writeFileSync(process.env.HOME + '/.config/solana/id.json', keypairJson);
console.log('✅ Keypair saved to ~/.config/solana/id.json');
EOF

# Run it
node /tmp/convert-key.js
chmod 600 ~/.config/solana/id.json
```

**Step 3: Verify Your Wallet**

```bash
# Check your wallet address
solana address

# Check your balance
solana balance
```

**Step 4: Get SOL (if needed)**

You need **0.1 SOL** (~$15-20) for deployment:

- Buy SOL on exchange (Coinbase, Binance, etc.)
- Send to your wallet address (from `solana address`)

**Step 5: Deploy Token**

```bash
cd /Users/macbook/Desktop/etrid/dex-deployment/solana
./deploy-solana.sh
```

Choose option **2** for mainnet deployment!

---

## Option 2: Deploy Using Phantom Wallet Directly (Simpler)

Instead of CLI, you can use Phantom wallet to deploy through a web interface:

### Method: Use Solana Token Creator

**1. Visit Token Creator Website:**
- https://www.solaneyes.com/token-creator (popular)
- OR https://tokentool.app/

**2. Connect Phantom Wallet**
- Click "Connect Wallet"
- Select Phantom
- Approve connection

**3. Create Token:**

Fill in details:
```
Name: Etrid Coin
Symbol: ETR
Decimals: 9
Supply: 100,000,000
```

**4. Upload Logo:**
- Upload your ÉTR logo (PNG/JPG)
- They'll host it on IPFS

**5. Pay & Deploy:**
- Cost: ~0.1 SOL (~$15)
- Click "Create Token"
- Approve transaction in Phantom

**6. Save Token Address:**
- Copy the token mint address
- Save it for creating Raydium pool

---

## After Deployment: Create Raydium Pool

Once token is deployed:

**1. Go to Raydium:**
https://raydium.io/liquidity/create/

**2. Connect Phantom Wallet**

**3. Create Pool:**

```
Token A: [Your ÉTR token address]
Token B: SOL (So11111111111111111111111111111111111111112)
Fee Tier: 0.25%
Initial Liquidity:
  - 100,000 ÉTR
  - ~50 SOL (adjust based on desired price)
```

**4. Add Liquidity:**
- Approve ÉTR spending
- Approve SOL spending
- Confirm pool creation
- Pay fee (~0.4 SOL)

**5. Pool is LIVE!** 🎉
- Raydium will auto-detect
- Jupiter will index within 1-2 hours
- BullX will detect within 24 hours
- Phantom wallet will show ÉTR token

---

## Recommended: Option 2 (Phantom + Web Interface)

**Why?**
- ✅ No CLI installation needed
- ✅ Visual interface (easier)
- ✅ Same cost (~0.1 SOL)
- ✅ Automatic IPFS metadata upload
- ✅ Direct Phantom integration

**Steps:**
1. Buy 1 SOL (~$150) - send to Phantom wallet
2. Use https://www.solaneyes.com/token-creator
3. Create token (0.1 SOL)
4. Create Raydium pool (0.4 SOL + liquidity)
5. Done! 🚀

---

## Cost Breakdown (Both Options)

| Item | Cost |
|------|------|
| Token Creation | 0.1 SOL (~$15) |
| Create Token Account | 0.002 SOL (~$0.30) |
| Mint Supply | 0.001 SOL (~$0.15) |
| Create Raydium Pool | 0.4 SOL (~$60) |
| Initial Liquidity | 50-100 SOL (~$7,500-$15,000) |
| **Total (minimum)** | **50.5 SOL (~$7,575)** |

**To deploy on smaller budget ($50):**
- Token creation: 0.1 SOL (~$15) ✅
- Raydium pool: 0.4 SOL (~$60) ⚠️ (exceeds budget)
- Liquidity: 1 SOL minimum (~$150)

**Minimum realistic: ~$225 total**

---

## Your Phantom Wallet Info

**Private Key Location:**
- Stored in your Phantom wallet
- Base58 format for Solana CLI import
- **DO NOT SHARE PUBLICLY!**

**What You Need:**
1. ✅ Private key (you have it)
2. ⏳ SOL for gas + liquidity
3. ⏳ Choose deployment method (CLI or Web)

---

## Troubleshooting

**Q: Solana CLI won't install?**
A: Use Option 2 (Phantom + web interface) instead!

**Q: How much SOL do I really need?**
A:
- Just token: 0.1 SOL (~$15)
- Token + small pool: 1.5 SOL (~$225)
- Full deployment: 50+ SOL (~$7,500+)

**Q: Can I test first?**
A: Yes!
- Get devnet SOL: https://faucet.solana.com
- Use devnet in web creators
- Test for free before mainnet

**Q: Will BullX detect it?**
A: Yes! After you:
1. Deploy token ✅
2. Create Raydium pool ✅
3. Add liquidity ✅
4. Wait 24 hours for BullX indexing

**Q: What about Phantom wallet display?**
A: Phantom automatically detects all SPL tokens in your wallet. After deployment, your ÉTR will show up instantly!

---

## Quick Decision Guide

**Have Solana CLI experience?** → Use Option 1 (CLI)
**Want easiest method?** → Use Option 2 (Web interface)
**On a budget (<$100)?** → Deploy token only, add pool later
**Ready to go live?** → Option 2 + Raydium pool = Full deployment

---

## Next Steps

1. **Choose your method** (CLI or Web)
2. **Get SOL** (at least 0.5 SOL for testing)
3. **Deploy token**
4. **Create Raydium pool** (when you have liquidity)
5. **Lock equivalent on FlareChain** (maintain 1:1 backing)

---

**Need help?** Check the deployment script at: `./deploy-solana.sh`

**Ready to deploy?** You have everything you need! 🚀
