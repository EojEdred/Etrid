# DEX Deployment & Liquidity Guide

**Date:** October 25, 2025
**Status:** 🚀 Ready for deployment
**Purpose:** Deploy ÉTR to BSC and Solana, set up liquidity pools

---

## Overview

This guide covers deploying the ÉTR token to multiple chains and setting up decentralized exchange (DEX) liquidity pools for trading.

**From TOKEN_ALLOCATION_FOR_LIQUIDITY.md:**
- **Community LP Pool:** 250,000,000 ÉTR (10% of total supply)
- **Purpose:** Initial liquidity on DEXes for price discovery and trading

---

## Part 1: BSC (Binance Smart Chain) Deployment

### Prerequisites

**Required:**
- BSC wallet with BNB for gas (estimate: 0.05 BNB)
- Deployer private key (secure!)
- BSCScan API key (for verification)
- Foundation multisig address

**Environment variables (.env):**
```bash
# BSC Configuration
BSC_RPC_URL=https://bsc-dataseed.binance.org
DEPLOYER_PRIVATE_KEY=your_private_key_here
BSCSCAN_API_KEY=your_bscscan_api_key_here

# Addresses
FOUNDATION_MULTISIG=your_multisig_address_here
BRIDGE_CONTRACT=your_bridge_address_here
```

### Step 1: Deploy ÉTR Token to BSC

**Contract:** `EtridToken.sol` (already exists at `05-multichain/bridge/adapters/bsc/contracts/EtridToken.sol`)

**Features:**
- ERC-20 standard compliance
- Bridge-controlled minting (only bridge can mint)
- Burnable (for bridging back to FlareChain)
- Pausable (emergency circuit breaker)
- 18 decimals (ERC-20 standard)

**Deploy command:**
```bash
cd /Users/macbook/Desktop/etrid/contracts/ethereum

# Install dependencies (if not already done)
npm install

# Deploy to BSC Testnet (for testing first)
npx hardhat run scripts/deploy-bsc.js --network bscTestnet

# Deploy to BSC Mainnet (production)
npx hardhat run scripts/deploy-bsc.js --network bsc
```

**Expected output:**
```
Deploying ÉTR token to BSC...
Deploying with account: 0x...
Account balance: 1000000000000000000

✅ ÉTR Token deployed to: 0x1234567890abcdef...

Token Details:
  Name: Etrid Coin
  Symbol: ÉTR
  Decimals: 18
```

### Step 2: Verify Contract on BSCScan

```bash
npx hardhat verify --network bsc <TOKEN_ADDRESS> "Etrid Coin" "ÉTR"
```

**Benefits:**
- Users can view contract source code
- Enables BSCScan token tracking
- Required for DEX listings

### Step 3: Configure Roles

**Transfer roles to secure addresses:**

```javascript
// Using ethers.js or Hardhat console
const token = await ethers.getContractAt("EtridToken", TOKEN_ADDRESS);

// Grant BRIDGE_ROLE to bridge contract
await token.grantRole(
  ethers.id("BRIDGE_ROLE"),
  BRIDGE_CONTRACT_ADDRESS
);

// Grant DEFAULT_ADMIN_ROLE to multisig
await token.grantRole(
  ethers.ZeroHash, // DEFAULT_ADMIN_ROLE
  FOUNDATION_MULTISIG_ADDRESS
);

// Revoke deployer's roles (optional, for security)
await token.renounceRole(ethers.id("BRIDGE_ROLE"), deployer.address);
await token.renounceRole(ethers.ZeroHash, deployer.address);
```

### Step 4: Set Up PancakeSwap Liquidity Pool

**PancakeSwap V3 (recommended):**

**Option A: Via PancakeSwap UI**
1. Go to https://pancakeswap.finance/add
2. Select ÉTR token (paste contract address)
3. Select BNB as pair
4. Choose fee tier (0.25% recommended for new tokens)
5. Set price range (full range for initial liquidity)
6. Add liquidity:
   - **ÉTR amount:** 100,000,000 ÉTR (40% of LP pool)
   - **BNB amount:** Calculate based on desired initial price

**Initial price calculation:**
```
Target initial price: $0.10 per ÉTR
BNB price: $300 (example)

100M ÉTR × $0.10 = $10,000,000 liquidity value
$10,000,000 / $300 = 33,333 BNB needed

Alternative (more conservative):
25M ÉTR + 8,333 BNB = $2,500,000 liquidity
```

**Option B: Via Smart Contract**
```solidity
// PancakeSwap V3 NonfungiblePositionManager
address constant POSITION_MANAGER = 0x46A15B0b27311cedF172AB29E4f4766fbE7F4364;

// Approve tokens
etrToken.approve(POSITION_MANAGER, 100_000_000e18);
wbnb.approve(POSITION_MANAGER, 33_333e18);

// Create pool and add liquidity
INonfungiblePositionManager(POSITION_MANAGER).mint(
  INonfungiblePositionManager.MintParams({
    token0: address(etrToken),
    token1: address(wbnb),
    fee: 2500, // 0.25%
    tickLower: -887272,  // Full range
    tickUpper: 887272,
    amount0Desired: 100_000_000e18,
    amount1Desired: 33_333e18,
    amount0Min: 0,
    amount1Min: 0,
    recipient: msg.sender,
    deadline: block.timestamp + 1 hours
  })
);
```

### Step 5: Monitor & Manage BSC Pool

**Track metrics:**
- Pool TVL (Total Value Locked)
- Trading volume (24h, 7d)
- Price impact
- Impermanent loss

**Tools:**
- PancakeSwap Analytics: https://pancakeswap.finance/info/v3/bsc/pools
- DexTools: https://www.dextools.io/app/en/bnb
- DexScreener: https://dexscreener.com/bsc

---

## Part 2: Solana Deployment

### Prerequisites

**Required:**
- Solana wallet with SOL for rent + gas (estimate: 5 SOL)
- Solana CLI installed (`sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`)
- Anchor framework (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`)

**Environment setup:**
```bash
# Set cluster (mainnet-beta for production)
solana config set --url https://api.mainnet-beta.solana.com

# Or devnet for testing
solana config set --url https://api.devnet.solana.com

# Check balance
solana balance
```

### Step 1: Create SPL Token

**Using Solana Token Program:**

```bash
# Create token mint (generates new mint address)
spl-token create-token --decimals 9

# Output: Creating token <MINT_ADDRESS>
# Save this MINT_ADDRESS for later use

# Create token account for yourself
spl-token create-account <MINT_ADDRESS>

# Mint initial supply (for liquidity pool)
# Mint 250M ÉTR (with 9 decimals = 250,000,000,000,000,000)
spl-token mint <MINT_ADDRESS> 250000000000000000

# Set token metadata (name, symbol, etc)
# Requires Metaplex Token Metadata program
```

**Token configuration:**
```
Name: Etrid Coin
Symbol: ÉTR
Decimals: 9 (Solana standard)
Supply: 250,000,000 ÉTR (for liquidity)
Mint Authority: Foundation multisig (eventually)
Freeze Authority: Foundation multisig (for security)
```

### Step 2: Configure Token Metadata (Metaplex)

**Install Metaplex CLI:**
```bash
npm install -g @metaplex-foundation/js
```

**Create metadata:**
```javascript
// Using Metaplex JS SDK
import { Metaplex } from '@metaplex-foundation/js';
import { Connection, Keypair } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const wallet = Keypair.fromSecretKey(/* your secret key */);
const metaplex = Metaplex.make(connection).use(keypairIdentity(wallet));

// Create metadata
await metaplex.nfts().create({
  uri: "https://etrid.network/token-metadata.json",  // Host metadata JSON
  name: "Etrid Coin",
  symbol: "ÉTR",
  sellerFeeBasisPoints: 0,
  mintAddress: new PublicKey(MINT_ADDRESS),
});
```

**Metadata JSON (host at https://etrid.network/token-metadata.json):**
```json
{
  "name": "Etrid Coin",
  "symbol": "ÉTR",
  "description": "Native token of the Ëtrid blockchain, bridged to Solana",
  "image": "https://etrid.network/logo-512.png",
  "external_url": "https://etrid.network",
  "properties": {
    "category": "fungible-token",
    "files": [
      {
        "uri": "https://etrid.network/logo-512.png",
        "type": "image/png"
      }
    ]
  }
}
```

### Step 3: Set Up Raydium Liquidity Pool

**Raydium CLMM (Concentrated Liquidity Market Maker) - Recommended:**

**Option A: Via Raydium UI**
1. Go to https://raydium.io/liquidity/create
2. Select CLMM pool type
3. Choose ÉTR token (paste mint address)
4. Choose SOL as pair
5. Set fee tier (0.25% for new tokens)
6. Set initial price and range
7. Add liquidity:
   - **ÉTR amount:** 100,000,000 ÉTR (40% of LP pool)
   - **SOL amount:** Calculate based on desired price

**Initial price calculation:**
```
Target initial price: $0.10 per ÉTR
SOL price: $150 (example)

100M ÉTR × $0.10 = $10,000,000 liquidity value
$10,000,000 / $150 = 66,667 SOL needed

Alternative (more conservative):
25M ÉTR + 16,667 SOL = $2,500,000 liquidity
```

**Option B: Via Raydium SDK**
```typescript
import { Connection, Keypair } from '@solana/web3.js';
import { Raydium } from '@raydium-io/raydium-sdk';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const raydium = await Raydium.load({ connection });

// Create CLMM pool
const { poolAddress } = await raydium.clmm.createPool({
  mint1: ETR_MINT_ADDRESS,
  mint2: WSOL_MINT_ADDRESS,  // Wrapped SOL
  ammConfig: RAYDIUM_AMM_CONFIG,  // 0.25% fee tier
  initialPrice: 0.00067,  // $0.10 ÉTR / $150 SOL
  startTime: Math.floor(Date.now() / 1000),
});

// Add liquidity
await raydium.clmm.addLiquidity({
  poolAddress,
  amountA: 100_000_000_000000000,  // 100M ÉTR (9 decimals)
  amountB: 66_667_000000000,        // 66,667 SOL (9 decimals)
  tickLower: -443636,  // Full range
  tickUpper: 443636,
});
```

### Step 4: Monitor & Manage Solana Pool

**Track metrics:**
- Jupiter aggregator integration (automatic)
- Raydium Analytics: https://raydium.io/clmm/pools
- Solscan: https://solscan.io/token/<MINT_ADDRESS>
- Birdeye: https://birdeye.so/token/<MINT_ADDRESS>

---

## Part 3: Bridge Integration

### FlareChain ↔ BSC Bridge

**Requirements:**
1. Bridge relayer infrastructure (already in pallets)
2. Oracle for price feeds
3. Validator committee signing

**Configuration on FlareChain:**
```rust
// In runtime configuration
impl bnb_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BridgeTokenAddress = ConstAddress<BSC_ETR_TOKEN_ADDRESS>;
    type MinimumTransfer = ConstU128<1_000_000_000_000>; // 0.001 ÉTR min
    type MaximumTransfer = ConstU128<100_000_000_000_000_000_000>; // 100k ÉTR max
}
```

**BSC token contract integration:**
```javascript
// Grant BRIDGE_ROLE to relayer
await etrToken.grantRole(
  ethers.id("BRIDGE_ROLE"),
  BSC_BRIDGE_RELAYER_ADDRESS
);
```

### FlareChain ↔ Solana Bridge

**Using Solana bridge pallet:**
```rust
impl sol_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type SolanaTokenMint = ConstPubkey<SOLANA_ETR_MINT>;
    type MinimumTransfer = ConstU128<1_000_000_000_000>; // 0.001 ÉTR min
    type MaximumTransfer = ConstU128<100_000_000_000_000_000_000>; // 100k ÉTR max
}
```

---

## Part 4: LP Rewards & Incentives

### Implementing LP Reward Distribution

**From Community LP Pool (250M ÉTR total):**
- 100M ÉTR: Initial liquidity (BSC + Solana)
- 150M ÉTR: LP rewards over 3 years

**Reward distribution schedule:**
```
Year 1: 75M ÉTR (50% of rewards)
Year 2: 45M ÉTR (30% of rewards)
Year 3: 30M ÉTR (20% of rewards)

Daily rewards:
Year 1: ~205,479 ÉTR/day
Year 2: ~123,288 ÉTR/day
Year 3: ~82,192 ÉTR/day
```

### BSC LP Rewards (PancakeSwap)

**Option A: MasterChef V3 (recommended)**
```solidity
// PancakeSwap MasterChef V3 integration
contract EtridLPRewards {
    IPancakeMasterChefV3 public masterChef;
    address public etrToken;
    uint256 public rewardPerBlock;

    constructor(address _masterChef, address _etr) {
        masterChef = IPancakeMasterChefV3(_masterChef);
        etrToken = _etr;
        rewardPerBlock = 7_130e18;  // ~205,479 ÉTR/day / 28,800 blocks
    }

    // Distribute rewards to LP stakers
    function distributeRewards() external {
        // Implementation
    }
}
```

**Option B: Custom Staking Contract**
```solidity
contract EtridLPStaking {
    IERC20 public lpToken;  // PancakeSwap ÉTR-BNB LP token
    IERC20 public rewardToken;  // ÉTR

    mapping(address => uint256) public stakedBalance;
    mapping(address => uint256) public rewardDebt;

    uint256 public rewardPerSecond = 2_379e18;  // 205,479 ÉTR/day

    function stake(uint256 amount) external {
        // Stake LP tokens, earn ÉTR rewards
    }

    function withdraw(uint256 amount) external {
        // Unstake and claim rewards
    }

    function claimRewards() external {
        // Claim accumulated ÉTR rewards
    }
}
```

### Solana LP Rewards (Raydium)

**Raydium Farms integration:**
```typescript
import { Raydium } from '@raydium-io/raydium-sdk';

// Create farm for ÉTR-SOL pool
const farm = await raydium.farm.create({
  poolId: ETR_SOL_POOL_ID,
  rewardMints: [ETR_MINT_ADDRESS],
  rewardPerSecond: [2_379_000000000],  // 205,479 ÉTR/day (9 decimals)
  startTime: Math.floor(Date.now() / 1000),
  endTime: Math.floor(Date.now() / 1000) + (365 * 24 * 60 * 60),  // 1 year
});
```

---

## Part 5: Token Listings & Visibility

### Price Aggregators

**CoinGecko:**
1. Submit token listing: https://www.coingecko.com/en/coins/new
2. Required info:
   - Contract addresses (BSC + Solana)
   - Project website
   - Logo (200x200px PNG)
   - Social media links
   - Whitepaper/documentation

**CoinMarketCap:**
1. Submit request: https://support.coinmarketcap.com/hc/en-us/articles/360043659351
2. Required info:
   - Same as CoinGecko
   - Proof of liquidity (DEX pool links)
   - Trading volume history

### DEX Aggregators

**Auto-listed on:**
- Jupiter (Solana) - automatic via Raydium
- 1inch (BSC) - automatic via PancakeSwap
- ParaSwap (BSC)
- Kyber Network (BSC)

**Manual listing:**
- Uniswap token lists (if bridging to Ethereum later)
- Trust Wallet token list

---

## Summary & Checklist

### BSC Deployment
- [ ] Deploy EtridToken.sol to BSC
- [ ] Verify contract on BSCScan
- [ ] Configure roles (multisig, bridge)
- [ ] Create PancakeSwap ÉTR-BNB pool
- [ ] Add initial liquidity (100M ÉTR)
- [ ] Set up LP rewards (MasterChef or custom)
- [ ] Test bridge BSC ↔ FlareChain

### Solana Deployment
- [ ] Create SPL token mint
- [ ] Configure token metadata (Metaplex)
- [ ] Create Raydium ÉTR-SOL pool
- [ ] Add initial liquidity (100M ÉTR)
- [ ] Set up LP rewards (Raydium Farms)
- [ ] Test bridge Solana ↔ FlareChain

### Post-Deployment
- [ ] Submit to CoinGecko
- [ ] Submit to CoinMarketCap
- [ ] Monitor liquidity & volume
- [ ] Adjust LP rewards based on usage
- [ ] Community announcement

---

**Estimated Timeline:**
- BSC deployment: 1-2 days
- Solana deployment: 2-3 days
- Bridge testing: 1-2 days
- Listings & marketing: 1 week

**Estimated Costs:**
- BSC gas: ~0.05 BNB (~$15)
- Solana rent: ~5 SOL (~$750)
- Initial liquidity: Allocated from Community LP Pool (no additional cost)
- Audit (recommended): $10k-$50k (optional but recommended before mainnet)

---

**Status:** 📋 Guide complete, ready for execution
**Next:** Deploy to testnets first, then production
