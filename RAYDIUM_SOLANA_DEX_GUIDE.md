# 🌊 Raydium DEX Integration Guide for Ëtrid
## Deploying ETR & EDSC on Solana with Raydium Liquidity Pools

**Created:** 2025-11-30
**Purpose:** Complete guide to listing Ëtrid tokens on Raydium (Solana DEX)
**Status:** Planning Phase

---

## 📋 Overview

**Raydium** is Solana's leading AMM DEX with:
- Lightning-fast transactions (<400ms)
- Ultra-low fees ($0.00025/tx)
- Serum integration for order book liquidity
- Deep liquidity pools

**What You Need to Deploy:**
1. **SPL Tokens** - Solana token standard (like ERC20 on Ethereum)
2. **Token Metadata** - On-chain token info
3. **Raydium Pool** - Liquidity pool creation
4. **Initial Liquidity** - Recommended: $50k-$500k

---

## 🏗️ Architecture Difference: Solana vs EVM

### **Ethereum/EVM (PrimeSwap)**
```
Smart Contracts (Solidity) → Deploy to EVM → Interact via Web3
```

### **Solana (Raydium)**
```
Programs (Rust) → Deploy to Solana → Interact via Web3.js/Anchor
```

**Key Differences:**

| Aspect | EVM (Ethereum/BSC) | Solana |
|--------|-------------------|---------|
| **Language** | Solidity | Rust (Anchor framework) |
| **Token Standard** | ERC20 | SPL Token |
| **Account Model** | Account-based | Account-based (different structure) |
| **Fees** | $2-$50/tx | $0.00025/tx |
| **Speed** | 12-60 seconds | <1 second |
| **Development** | Hardhat/Truffle | Anchor/Solana CLI |

---

## 🛠️ Implementation Path (3 Options)

### **Option 1: Use Raydium's SDK (Recommended - Easiest)**
**Don't write contracts, use Raydium's existing infrastructure**

✅ **Pros:**
- No Rust programming needed
- Just create SPL tokens
- Use Raydium's UI/SDK to create pools
- Fastest to market (1-2 weeks)

❌ **Cons:**
- Less customization
- Dependent on Raydium protocol

**Steps:**
1. Create SPL tokens (ETR.s, EDSC.s)
2. Create token metadata
3. Use Raydium SDK/UI to create pools
4. Add liquidity via their interface

**Cost:** ~$500 (dev) + $50k liquidity

---

### **Option 2: Write Custom Solana Program (Like PrimeSwap)**
**Build your own AMM on Solana**

✅ **Pros:**
- Full control
- Custom features
- Own brand/protocol

❌ **Cons:**
- Complex Rust development
- 3-6 months development
- Requires Solana expertise
- Higher audit costs ($50k-$100k)

**Cost:** ~$50k-$150k (dev) + $100k+ liquidity

---

### **Option 3: Hybrid - Bridge to Raydium (BEST FOR YOU)**
**Use your existing Solana PBC + Raydium's pools**

✅ **Pros:**
- Leverage existing Solana PBC infrastructure
- Use Raydium for DEX (no reinventing wheel)
- Your bridge = your control
- Faster than custom AMM

❌ **Cons:**
- Need to complete Solana bridge
- Two-step process (bridge → trade)

**Cost:** ~$10k-$20k (bridge completion) + $50k liquidity

---

## 🎯 RECOMMENDED: Option 3 (Hybrid Approach)

### **Why This Makes Sense:**

1. **You Already Have:**
   - ✅ Solana PBC (`/05-multichain/partition-burst-chains/pbc-chains/sol-pbc/`)
   - ✅ Solana bridge infrastructure (`/05-multichain/bridges/protocols/solana-bridge/`)
   - ✅ Experience with multichain (13 PBCs)

2. **What You Need to Complete:**
   - 🔧 Finish Solana bridge integration
   - 🔧 Deploy SPL token contracts
   - 🔧 Create Raydium pools
   - 💰 Add $50k-$100k liquidity

3. **Timeline:** 2-4 weeks (vs 3-6 months for custom AMM)

---

## 📝 Step-by-Step Implementation (Hybrid Approach)

### **Phase 1: Deploy SPL Tokens (Week 1)**

#### 1.1 Install Solana Tools
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Install Anchor (Solana's smart contract framework)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
anchor --version

# Install SPL Token CLI
cargo install spl-token-cli
```

#### 1.2 Create Wallet
```bash
# Generate new Solana wallet
solana-keygen new --outfile ~/.config/solana/etrid-deployer.json

# Get wallet address
solana-keygen pubkey ~/.config/solana/etrid-deployer.json

# Set as default
solana config set --keypair ~/.config/solana/etrid-deployer.json

# Set network (devnet for testing, mainnet-beta for production)
solana config set --url https://api.devnet.solana.com  # Testing
solana config set --url https://api.mainnet-beta.solana.com  # Production
```

#### 1.3 Fund Wallet
```bash
# Devnet (free)
solana airdrop 2

# Mainnet (buy SOL from exchange)
# Send SOL to your wallet address
```

#### 1.4 Create SPL Tokens
```bash
# Create ETR.s (Etrid Coin on Solana)
spl-token create-token --decimals 9

# Output example:
# Creating token 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
# Signature: 5sWT...

# Save this address as ETR_TOKEN_ADDRESS

# Create EDSC.s (Etrid Dollar Stablecoin on Solana)
spl-token create-token --decimals 9

# Save this address as EDSC_TOKEN_ADDRESS
```

#### 1.5 Create Token Accounts
```bash
# Create account to hold ETR tokens
spl-token create-account <ETR_TOKEN_ADDRESS>

# Create account to hold EDSC tokens
spl-token create-account <EDSC_TOKEN_ADDRESS>
```

#### 1.6 Mint Initial Supply
```bash
# Mint 100M ETR tokens
spl-token mint <ETR_TOKEN_ADDRESS> 100000000

# Mint 100M EDSC tokens
spl-token mint <EDSC_TOKEN_ADDRESS> 100000000
```

#### 1.7 Add Token Metadata (Metaplex)
```bash
# Install Metaplex Sugar CLI
bash <(curl -sSf https://sugar.metaplex.com/install.sh)

# Create metadata config (create metadata.json)
cat > etr-metadata.json << EOF
{
  "name": "Etrid Coin",
  "symbol": "ETR",
  "description": "Etrid native cryptocurrency on Solana",
  "image": "https://etrid.org/assets/etr-logo.png",
  "external_url": "https://etrid.org",
  "attributes": [
    {"trait_type": "Network", "value": "Solana"},
    {"trait_type": "Type", "value": "Utility Token"}
  ]
}
EOF

# Upload and create metadata
sugar upload etr-metadata.json
sugar create-metadata <ETR_TOKEN_ADDRESS>
```

---

### **Phase 2: Create Raydium Pools (Week 2)**

#### 2.1 Install Raydium SDK
```bash
mkdir etrid-raydium && cd etrid-raydium
npm init -y
npm install @raydium-io/raydium-sdk @solana/web3.js @solana/spl-token
```

#### 2.2 Create Pool Script
Create `create-raydium-pool.js`:

```javascript
const { Connection, Keypair, PublicKey } = require('@solana/web3.js');
const { Liquidity, Token, TokenAmount, Percent } = require('@raydium-io/raydium-sdk');
const fs = require('fs');

async function main() {
  // Load wallet
  const secretKey = JSON.parse(
    fs.readFileSync('/Users/macbook/.config/solana/etrid-deployer.json', 'utf-8')
  );
  const wallet = Keypair.fromSecretKey(Uint8Array.from(secretKey));

  // Connect to Solana
  const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

  // Token addresses
  const ETR_TOKEN = new PublicKey('<ETR_TOKEN_ADDRESS>');
  const USDC_TOKEN = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'); // USDC on Solana

  // Create pool configuration
  const poolConfig = {
    baseMint: ETR_TOKEN,
    quoteMint: USDC_TOKEN,
    baseAmount: new TokenAmount(
      new Token(ETR_TOKEN, 9, 'ETR', 'Etrid Coin'),
      10000000 // 10M ETR
    ),
    quoteAmount: new TokenAmount(
      new Token(USDC_TOKEN, 6, 'USDC', 'USD Coin'),
      40000 // $40k USDC
    ),
    startTime: Math.floor(Date.now() / 1000), // Now
  };

  console.log('Creating Raydium pool...');
  console.log('Base:', poolConfig.baseAmount.toFixed());
  console.log('Quote:', poolConfig.quoteAmount.toFixed());

  // Create pool (requires Raydium SDK setup)
  // This is simplified - actual implementation requires more setup
  const { transaction, signers } = await Liquidity.makeCreatePoolTransaction({
    connection,
    wallet: wallet.publicKey,
    marketInfo: {
      baseMint: ETR_TOKEN,
      quoteMint: USDC_TOKEN,
      // ... market configuration
    },
    baseAmount: poolConfig.baseAmount,
    quoteAmount: poolConfig.quoteAmount,
  });

  // Sign and send
  const txid = await connection.sendTransaction(transaction, [wallet, ...signers]);
  console.log('Pool created! Transaction:', txid);
}

main().catch(console.error);
```

#### 2.3 Alternative: Use Raydium UI (Easier)
```
1. Go to https://raydium.io/create-pool/
2. Connect Phantom/Solflare wallet
3. Select ETR token + USDC
4. Set initial price
5. Add liquidity ($50k recommended)
6. Confirm transaction
7. Pool is live!
```

**UI Method is MUCH easier and recommended**

---

### **Phase 3: Bridge Integration (Week 3-4)**

#### 3.1 Complete Solana Bridge
Your existing infrastructure at `/05-multichain/bridges/protocols/solana-bridge/`

**Required Components:**
```rust
// 1. Lock tokens on Etrid FlareChain
pub fn lock_etr_for_solana(
    origin: OriginFor<T>,
    amount: Balance,
    solana_recipient: Vec<u8>,
) -> DispatchResult {
    // Lock ETR on FlareChain
    // Emit event for watchtowers
}

// 2. Watchtower signs attestation
// (Similar to Ethereum bridge)

// 3. Mint SPL tokens on Solana
// Solana program instruction
pub fn mint_bridged_etr(
    amount: u64,
    recipient: Pubkey,
    attestation_signatures: Vec<Signature>,
) -> ProgramResult {
    // Verify signatures
    // Mint SPL tokens
}
```

#### 3.2 Deploy Solana Bridge Program
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/solana-bridge

# Build Solana program
anchor build

# Deploy to devnet (testing)
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet-beta
```

---

### **Phase 4: Add Liquidity & Launch (Week 4)**

#### 4.1 Add Initial Liquidity
```bash
# Via Raydium UI:
# 1. Go to your pool on Raydium
# 2. Click "Add Liquidity"
# 3. Enter amounts:
#    - ETR: 10,000,000 tokens
#    - USDC: $50,000
# 4. Confirm transaction
# 5. Receive LP tokens
```

#### 4.2 Lock Liquidity (Optional but Recommended)
```bash
# Use a time-lock service like:
# - Streamflow
# - Bonfida
# - Manual vesting

# Creates trust with community
# Shows long-term commitment
```

#### 4.3 Submit to Raydium Frontend
```
1. Fill out form: https://raydium.io/pool-request/
2. Provide:
   - Token mint addresses
   - Pool address
   - Liquidity amount
   - Project info
3. Wait for approval (1-7 days)
4. Pool appears on Raydium.io
```

---

## 💰 Cost Breakdown

### **Development Costs:**
| Item | Cost |
|------|------|
| Solana CLI setup | Free |
| SPL token creation | ~$5 (gas fees) |
| Token metadata | ~$10 (storage) |
| Raydium pool creation | ~$50-100 (fees) |
| Bridge completion (dev time) | $10k-$20k |
| **Total Development** | **~$10k-$20k** |

### **Liquidity Requirements:**
| Pool | Recommended | Minimum |
|------|------------|---------|
| ETR/USDC | $100k | $30k |
| EDSC/USDC | $100k | $30k |
| ETR/SOL | $50k | $20k |
| **Total Liquidity** | **$250k** | **$80k** |

### **Ongoing Costs:**
- Solana RPC: $0-$50/month (Alchemy/Quicknode)
- Watchtower operations: Existing infrastructure
- Marketing: Variable

---

## 🔧 Code Templates

### **Create SPL Token (Anchor)**
```rust
// programs/etrid-spl-token/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("EtridTokenProgramXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod etrid_spl_token {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        decimals: u8,
    ) -> Result<()> {
        msg!("Initializing ETR token on Solana");
        Ok(())
    }

    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        token::mint_to(
            ctx.accounts.mint_ctx(),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
```

### **Bridge Lock Function (Substrate)**
```rust
// pallets/solana-bridge/src/lib.rs
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn lock_for_solana(
        origin: OriginFor<T>,
        amount: BalanceOf<T>,
        solana_recipient: Vec<u8>,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        // Lock tokens on FlareChain
        T::Currency::reserve(&sender, amount)?;

        // Emit event for watchtowers
        Self::deposit_event(Event::TokensLockedForSolana {
            sender,
            amount,
            solana_recipient,
            lock_id: Self::next_lock_id(),
        });

        Ok(())
    }
}
```

---

## 📊 Raydium vs Custom AMM Comparison

| Feature | Use Raydium | Build Custom AMM |
|---------|-------------|------------------|
| **Time to Market** | 2-4 weeks | 3-6 months |
| **Development Cost** | $10k-$20k | $100k-$200k |
| **Audit Cost** | $0 (use theirs) | $50k-$100k |
| **Liquidity Depth** | High (Serum integration) | Low (start from zero) |
| **User Trust** | High (established) | Low (new protocol) |
| **Customization** | Limited | Full |
| **Maintenance** | Minimal | Ongoing |

**Verdict:** Use Raydium unless you need custom features

---

## 🚀 Quick Start (Fastest Path to Raydium)

### **3-Day Quickstart:**

**Day 1: Create Tokens**
```bash
solana-keygen new
solana airdrop 2  # Devnet
spl-token create-token --decimals 9  # ETR
spl-token create-token --decimals 9  # EDSC
spl-token mint <TOKEN> 100000000
```

**Day 2: Add Metadata**
```bash
# Upload logo to IPFS/Arweave
# Create metadata with Sugar CLI
# Link to token
```

**Day 3: Create Pool on Raydium**
```
1. Go to raydium.io/create-pool/
2. Connect wallet
3. Add $50k liquidity
4. Submit to Raydium listing
```

**Done!** 🎉

---

## ⚠️ Important Considerations

### **Before Deploying to Solana:**

1. **Liquidity Requirements**
   - Minimum: $80k across pools
   - Recommended: $250k+
   - More liquidity = better trading experience

2. **Bridge Security**
   - Complete Solana bridge audit
   - Test on devnet extensively
   - Use multisig for mint authority

3. **Market Making**
   - Consider hiring market maker
   - Maintain tight spreads
   - Monitor for manipulation

4. **Compliance**
   - Solana has different regulatory considerations
   - Ensure EDSC stablecoin compliance
   - Consider geographic restrictions

---

## 📞 Resources

### **Documentation:**
- Raydium Docs: https://docs.raydium.io/
- Solana Docs: https://docs.solana.com/
- SPL Token: https://spl.solana.com/token
- Anchor: https://www.anchor-lang.com/

### **Tools:**
- Raydium SDK: https://github.com/raydium-io/raydium-sdk
- Solana Web3.js: https://solana-labs.github.io/solana-web3.js/
- Metaplex: https://www.metaplex.com/

### **Testing:**
- Solana Devnet: https://api.devnet.solana.com
- Solana Explorer: https://explorer.solana.com/
- Raydium Devnet: https://raydium.io/?network=devnet

---

## ✅ Final Recommendation

**For Tonight's Launch:**
- ❌ **NOT Raydium** - Requires Solana development (2-4 weeks minimum)

**For Q1 2026 Expansion:**
- ✅ **Deploy to Raydium** using hybrid approach
- ✅ Complete Solana bridge first
- ✅ Create SPL tokens via CLI
- ✅ Use Raydium UI to create pools
- ✅ Add $80k-$250k liquidity
- ✅ Submit for listing

**Priority Order:**
1. **Tonight:** PrimeSwap ($750) ← START HERE
2. **Month 1-2:** Uniswap V3 ($50k+)
3. **Month 3:** PancakeSwap BSC ($50k+)
4. **Month 4:** Raydium Solana ($80k+) ← THIS GUIDE
5. **Month 6+:** CEX listings

---

**Created for Ëtrid Foundation**
**Status:** Planning Document
**Next Steps:** Complete PrimeSwap launch first, then revisit for Solana expansion
