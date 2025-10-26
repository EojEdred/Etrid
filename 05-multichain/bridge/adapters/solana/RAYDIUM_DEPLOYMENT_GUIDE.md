# Raydium (Solana) Deployment Guide

**Version**: 1.0
**Last Updated**: October 24, 2025
**Purpose**: Deploy ÉTR.sol and EDSC.sol SPL tokens on Solana and create liquidity pools on Raydium

---

## Overview

This guide walks through deploying Ëtrid tokens on Solana as SPL (Solana Program Library) tokens and creating liquidity pools on Raydium, Solana's leading AMM DEX.

### Key Differences from EVM Chains

| Aspect | EVM (Ethereum, BSC, Base) | Solana |
|--------|---------------------------|--------|
| **Token Standard** | ERC-20 / BEP-20 | SPL Token |
| **Programming Language** | Solidity | Rust (Anchor framework) |
| **Account Model** | State in contract | Program Derived Addresses (PDAs) |
| **Gas Model** | Gas price × gas used | Fixed per-signature fee (~0.000005 SOL) |
| **Decimals** | Typically 18 | Typically 9 |
| **Bridge Integration** | Lock/Mint pattern | Associated Token Accounts (ATAs) |

---

## Prerequisites

### 1. Install Solana CLI

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Verify installation
solana --version  # Should show v1.14+ or later

# Set cluster (mainnet-beta for production)
solana config set --url https://api.mainnet-beta.solana.com

# Or use devnet for testing
solana config set --url https://api.devnet.solana.com
```

### 2. Install Anchor Framework

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Verify
anchor --version  # Should show v0.29+ or later
```

### 3. Install SPL Token CLI

```bash
cargo install spl-token-cli

# Verify
spl-token --version
```

### 4. Setup Wallet

```bash
# Generate new keypair (or import existing)
solana-keygen new --outfile ~/.config/solana/deployer.json

# Set as default
solana config set --keypair ~/.config/solana/deployer.json

# Check balance
solana balance

# Fund wallet (devnet only)
solana airdrop 2

# For mainnet, transfer SOL from exchange or another wallet
```

---

## Step 1: Deploy SPL Tokens

### Option A: Using SPL Token CLI (Recommended for Simple Tokens)

#### Deploy ÉTR Token

```bash
# Create token mint (9 decimals standard for Solana)
spl-token create-token --decimals 9

# Output: Creating token <MINT_ADDRESS>
# Save this address!

# Example output:
# Creating token 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
export ETR_MINT=7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU

# Create token account for your wallet
spl-token create-account $ETR_MINT

# Optionally, mint initial supply (for testing)
# spl-token mint $ETR_MINT 1000000 --decimals 9

# Set metadata (name, symbol, logo)
# NOTE: This requires Metaplex Token Metadata program
# See: https://docs.metaplex.com/programs/token-metadata/

# For production, use Metaplex CLI or SDK:
npm install -g @metaplex-foundation/js
```

#### Deploy EDSC Token

```bash
# Create EDSC mint
spl-token create-token --decimals 9

# Example:
export EDSC_MINT=8uJXRgsXKHZZXzK

YfU8gVQjxU3g9xQqpKN5EqXsP

# Create token account
spl-token create-account $EDSC_MINT
```

---

### Option B: Using Anchor Program (Advanced, Bridge-Compatible)

For bridge integration, you need custom mint/burn authority. Here's an Anchor program skeleton:

#### File: `programs/etrid-token/src/lib.rs`

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn};

declare_id!("EtRiDTokenProgram11111111111111111111111111111");

#[program]
pub mod etrid_token {
    use super::*;

    /// Initialize token mint with bridge authority
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.decimals = decimals;
        mint.mint_authority = COption::Some(ctx.accounts.bridge_authority.key());
        mint.freeze_authority = COption::None;
        Ok(())
    }

    /// Bridge mint (called by bridge adapter when tokens locked on Ëtrid)
    pub fn bridge_mint(
        ctx: Context<BridgeMint>,
        amount: u64,
        etrid_tx_hash: [u8; 32],
    ) -> Result<()> {
        // Check bridge authority
        require!(
            ctx.accounts.bridge_authority.key() == ctx.program_id,
            ErrorCode::Unauthorized
        );

        // Mint tokens to destination
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.bridge_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, amount)?;

        // Emit event
        emit!(BridgeMintEvent {
            destination: ctx.accounts.destination.key(),
            amount,
            etrid_tx_hash,
        });

        Ok(())
    }

    /// Bridge burn (user initiates transfer back to Ëtrid)
    pub fn bridge_burn(
        ctx: Context<BridgeBurn>,
        amount: u64,
        etrid_address: String,
    ) -> Result<()> {
        // Burn tokens from user's account
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.source.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        // Emit event
        emit!(BridgeBurnEvent {
            source: ctx.accounts.source.key(),
            amount,
            etrid_address,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = bridge_authority,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Bridge authority PDA
    pub bridge_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BridgeMint<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    /// CHECK: Bridge authority (validated in instruction)
    pub bridge_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BridgeBurn<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub source: Account<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct BridgeMintEvent {
    pub destination: Pubkey,
    pub amount: u64,
    pub etrid_tx_hash: [u8; 32],
}

#[event]
pub struct BridgeBurnEvent {
    pub source: Pubkey,
    pub amount: u64,
    pub etrid_address: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only bridge can mint")]
    Unauthorized,
}
```

#### Build and Deploy

```bash
# Build Anchor program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Output: Program Id: <PROGRAM_ID>
# Save this!
```

---

## Step 2: Add Token Metadata (Name, Symbol, Logo)

Solana uses Metaplex Token Metadata for token information.

### Using Metaplex JS SDK

Create `scripts/set-metadata.ts`:

```typescript
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import * as fs from "fs";

async function setTokenMetadata() {
  // Connect to Solana
  const connection = new Connection("https://api.mainnet-beta.solana.com");

  // Load deployer keypair
  const keypairData = JSON.parse(
    fs.readFileSync(process.env.HOME + "/.config/solana/deployer.json", "utf-8")
  );
  const deployer = Keypair.fromSecretKey(new Uint8Array(keypairData));

  // Initialize Metaplex
  const metaplex = Metaplex.make(connection).use(keypairIdentity(deployer));

  // ÉTR Token metadata
  const etrMint = new PublicKey(process.env.ETR_MINT!);

  await metaplex.nfts().create({
    uri: "https://etrid.com/metadata/etr-solana.json", // Upload JSON first
    name: "Etrid Coin",
    symbol: "ÉTR",
    sellerFeeBasisPoints: 0,
    tokenStandard: 0, // Fungible
  });

  console.log("✅ ÉTR metadata set");

  // EDSC Token metadata
  const edscMint = new PublicKey(process.env.EDSC_MINT!);

  await metaplex.nfts().create({
    uri: "https://etrid.com/metadata/edsc-solana.json",
    name: "Etrid Dollar Stablecoin",
    symbol: "EDSC",
    sellerFeeBasisPoints: 0,
    tokenStandard: 0,
  });

  console.log("✅ EDSC metadata set");
}

setTokenMetadata().catch(console.error);
```

### Metadata JSON Files

Upload these to your website or IPFS:

**`etr-solana.json`**:
```json
{
  "name": "Etrid Coin (Solana)",
  "symbol": "ÉTR",
  "description": "Etrid's native token on Solana, bridged from FlareChain via E³20 protocol",
  "image": "https://etrid.com/assets/etr-logo.png",
  "external_url": "https://etrid.com",
  "properties": {
    "category": "currency",
    "creators": [
      {
        "address": "<YOUR_AUTHORITY_ADDRESS>",
        "share": 100
      }
    ]
  }
}
```

---

## Step 3: Create Raydium Liquidity Pools

### Prerequisites

1. **Token mints created** (ÉTR, EDSC)
2. **Metadata set** (name, symbol, logo)
3. **Initial liquidity ready** (tokens + SOL/USDC)

### Option A: Using Raydium UI (Easiest)

1. Go to https://raydium.io/liquidity/create
2. Connect wallet (Phantom, Solflare, etc.)
3. Select tokens:
   - Token A: ÉTR (paste mint address)
   - Token B: SOL or USDC
4. Set initial price and amounts
5. Confirm transaction

**Estimated Cost**: ~0.02 SOL for pool creation

### Option B: Using Raydium SDK (Programmatic)

Create `scripts/create-raydium-pool.ts`:

```typescript
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { AmmV4, LiquidityPoolKeys, Token, TokenAmount } from "@raydium-io/raydium-sdk";
import * as fs from "fs";

async function createRaydiumPool() {
  const connection = new Connection("https://api.mainnet-beta.solana.com");

  // Load wallet
  const keypairData = JSON.parse(
    fs.readFileSync(process.env.HOME + "/.config/solana/deployer.json", "utf-8")
  );
  const payer = Keypair.fromSecretKey(new Uint8Array(keypairData));

  // Define tokens
  const ETR = new Token(
    new PublicKey(process.env.ETR_MINT!),
    9, // decimals
    "ÉTR",
    "Etrid Coin"
  );

  const SOL = Token.WSOL; // Wrapped SOL

  // Pool parameters
  const baseAmount = new TokenAmount(ETR, "1000000000000000"); // 1M ÉTR
  const quoteAmount = new TokenAmount(SOL, "500000000000"); // 500 SOL

  // Create pool (simplified - requires Raydium SDK full setup)
  // See: https://docs.raydium.io/raydium/developers/sdk

  console.log("💧 Creating Raydium pool...");

  // This requires:
  // 1. Create AMM keys
  // 2. Initialize market ID
  // 3. Create liquidity pool
  // 4. Add initial liquidity

  // Full implementation is complex - recommend using UI for first pool
  console.log("⚠️  Use Raydium UI for initial pool creation");
  console.log("   https://raydium.io/liquidity/create");
}

createRaydium Pool().catch(console.error);
```

### Recommended Pools

1. **ÉTR / SOL** - Main trading pair
   - Initial: 1M ÉTR + 500 SOL
   - Fee tier: 0.25%

2. **ÉTR / USDC** - Stablecoin pair
   - Initial: 1M ÉTR + $100k USDC
   - Fee tier: 0.25%

3. **EDSC / USDC** - Stablecoin pair (tight spread)
   - Initial: 500k EDSC + $500k USDC
   - Fee tier: 0.05%

---

## Step 4: Bridge Integration

### Connect to Ëtrid SOL-PBC

The SOL-PBC (Solana Partition Burst Chain) on Ëtrid monitors Solana for bridge events.

#### Architecture

```
Ëtrid FlareChain
    ↓
SOL-PBC (Collator)
    ↓
Solana Mainnet
    ↓
ÉTR.sol / EDSC.sol SPL Tokens
    ↓
Raydium Pools
```

#### Bridge Adapter Configuration

Create `/Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/solana/bridge-adapter.ts`:

```typescript
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { ApiPromise, WsProvider } from "@polkadot/api";

interface SolanaBridgeConfig {
  etridWsUrl: string;
  solanaRpcUrl: string;
  etrMint: string;
  edscMint: string;
  bridgeAuthority: string;
}

class SolanaBridgeAdapter {
  private connection: Connection;
  private etridApi: ApiPromise | null = null;

  constructor(private config: SolanaBridgeConfig) {
    this.connection = new Connection(config.solanaRpcUrl);
  }

  async start() {
    console.log("🌉 Starting Solana Bridge Adapter...");

    // Connect to Ëtrid
    const provider = new WsProvider(this.config.etridWsUrl);
    this.etridApi = await ApiPromise.create({ provider });
    console.log("✅ Connected to Ëtrid SOL-PBC");

    // Monitor Solana for burn events
    await this.monitorBurnEvents();
  }

  private async monitorBurnEvents() {
    console.log("👀 Monitoring Solana for burn events...");

    // Subscribe to program logs
    const etrMint = new PublicKey(this.config.etrMint);

    this.connection.onLogs(
      etrMint,
      async (logs, context) => {
        // Parse BridgeBurn events from logs
        // Trigger release on Ëtrid FlareChain
        console.log("🔥 Burn event detected:", logs);
      },
      "confirmed"
    );
  }
}

export default SolanaBridgeAdapter;
```

---

## Step 5: Testing

### Test on Devnet First

```bash
# Switch to devnet
solana config set --url https://api.devnet.solana.com

# Get devnet SOL
solana airdrop 2

# Deploy test tokens
spl-token create-token --decimals 9

# Create test pool on Raydium devnet
# (Use Raydium staging UI)
```

### Mainnet Deployment Checklist

- [ ] Tokens deployed with correct decimals (9)
- [ ] Metadata set (name, symbol, logo via Metaplex)
- [ ] Bridge authority configured
- [ ] Initial liquidity ready (tokens + SOL/USDC)
- [ ] Raydium pools created
- [ ] Bridge adapter running
- [ ] Verified on Solscan: https://solscan.io/token/<MINT_ADDRESS>

---

## Step 6: Post-Deployment

### 1. Submit to Jupiter Aggregator

Jupiter aggregates all Solana DEXs for best price routing.

- Visit: https://jup.ag/
- Submit token info: https://station.jup.ag/guides/general/get-your-token-on-jupiter

### 2. Submit to Phantom Wallet

Phantom is the most popular Solana wallet.

- Form: https://phantom.app/token-verification

### 3. Monitor Pool Performance

```bash
# Check pool stats
solana account <POOL_ADDRESS> --output json

# Check token supply
spl-token supply <MINT_ADDRESS>

# Check your LP tokens
spl-token accounts
```

---

## Costs Summary

| Item | Cost (SOL) | Cost (USD @ $20/SOL) |
|------|------------|----------------------|
| Token mint creation | 0.002 | $0.04 |
| Metadata account | 0.01 | $0.20 |
| Raydium pool creation | 0.02 | $0.40 |
| Initial liquidity | ~$100k | $100k |
| **Total** | ~0.032 + liquidity | ~$100k |

**Much cheaper than EVM chains!** (Ethereum would cost $500+ in gas)

---

## Troubleshooting

### "Insufficient funds"
```bash
# Check balance
solana balance

# Top up wallet
# (Transfer from exchange or airdrop on devnet)
```

### "Token mint already exists"
- Use the existing mint address
- Check `spl-token token-list` to see your mints

### "Failed to create pool on Raydium"
- Ensure both tokens have metadata set
- Check you have sufficient token balances
- Try using Raydium UI instead of SDK

---

## Next Steps

1. ✅ Deploy ÉTR and EDSC on Solana
2. ✅ Set metadata (Metaplex)
3. ✅ Create Raydium pools
4. Configure bridge adapter (SOL-PBC)
5. Test bridge flow (Ëtrid ↔ Solana)
6. Submit to Jupiter aggregator
7. Submit to Phantom wallet verification
8. Launch LP farming rewards (optional)

---

## Resources

- **Solana Docs**: https://docs.solana.com/
- **SPL Token Program**: https://spl.solana.com/token
- **Anchor Framework**: https://www.anchor-lang.com/
- **Raydium Docs**: https://docs.raydium.io/
- **Metaplex Docs**: https://docs.metaplex.com/
- **Jupiter Aggregator**: https://jup.ag/
- **Phantom Wallet**: https://phantom.app/

---

**Document Version**: 1.0
**Last Updated**: October 24, 2025
**Next Review**: After successful mainnet deployment
