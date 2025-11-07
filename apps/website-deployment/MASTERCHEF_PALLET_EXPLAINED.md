# MasterChef Pallet - What It Is and How to Deploy

**Date:** November 1, 2025
**Status:** ⚠️ NOT YET DEPLOYED TO FLARECHAIN

---

## 📋 What is MasterChef?

**MasterChef** is a yield farming / liquidity mining contract that:

1. **Rewards users for providing liquidity** to decentralized exchanges (DEXs)
2. **Distributes ÉTR tokens** to liquidity providers (LPs) over time
3. **Incentivizes liquidity** for key trading pairs on the ËTRID ecosystem

**Origin:** Based on SushiSwap/PancakeSwap MasterChef V2 design

---

## 🏗️ Current Status

### ✅ What EXISTS:

**Solidity Smart Contract (for BSC/EVM chains):**
- **Location:** `/Desktop/etrid/05-multichain/bridge/adapters/bsc/contracts/MasterChef.sol`
- **Type:** Solidity contract for Ethereum Virtual Machine (EVM)
- **Purpose:** Deploy to Binance Smart Chain (BSC), Ethereum, Polygon, etc.
- **Status:** Ready to deploy to EVM chains

### ❌ What DOESN'T EXIST:

**Substrate Pallet (for FlareChain):**
- **Type:** Rust-based pallet for Substrate runtime
- **Purpose:** Native integration with FlareChain (Substrate-based chain)
- **Status:** NOT CREATED YET
- **Why needed:** FlareChain is a Substrate chain, not an EVM chain

---

## 🔍 How MasterChef Works

### Core Functionality:

**1. Pool Creation**
- Owner adds LP token pools (e.g., ÉTR/BNB, ÉTR/EDSC)
- Each pool has allocation points (weight for rewards)

**2. Staking**
- Users deposit LP tokens into pools
- LP tokens represent liquidity provided to DEX

**3. Reward Distribution**
- ÉTR tokens emitted per block
- Rewards distributed based on:
  - Pool allocation points
  - User's share of pool
  - Time staked

**4. Harvesting**
- Users claim accumulated ÉTR rewards
- Can compound rewards or withdraw

---

## 📊 Technical Details (From MasterChef.sol)

### Key Parameters:

```solidity
// Constructor parameters
IERC20 _rewardToken      // ÉTR token address
uint256 _rewardPerBlock  // ÉTR distributed per block
uint256 _startBlock      // When rewards begin
```

### Emission Schedule (from deployment script):

**Month 1:**
- 2.89 ÉTR per block
- ~83,333 ÉTR per day
- ~2,500,000 ÉTR per month

**Total Budget:** 20M ÉTR for rewards program

### Pool Structure:

```solidity
struct PoolInfo {
    IERC20 lpToken;           // LP token contract
    uint256 allocPoint;       // Weight for rewards
    uint256 lastRewardBlock;  // Last reward calculation
    uint256 accRewardPerShare; // Accumulated rewards per share
    uint256 totalStaked;      // Total LP tokens staked
}
```

### User Info:

```solidity
struct UserInfo {
    uint256 amount;         // LP tokens user deposited
    uint256 rewardDebt;     // Internal accounting
    uint256 pendingRewards; // Unharvested rewards
}
```

---

## 🎯 What The Web App Needs

The MasterChef dashboard (https://etrid.org/masterchef/) needs these functions:

### Read Functions (Query Chain):

1. `poolInfo(uint256 pid)` - Get pool details
2. `userInfo(uint256 pid, address user)` - Get user stake/rewards
3. `pendingReward(uint256 pid, address user)` - Calculate pending rewards
4. `poolLength()` - Get number of pools
5. `totalAllocPoint()` - Get total allocation across all pools
6. `rewardPerBlock()` - Get current emission rate

### Write Functions (Send Transactions):

1. `deposit(uint256 pid, uint256 amount)` - Stake LP tokens
2. `withdraw(uint256 pid, uint256 amount)` - Unstake LP tokens
3. `harvest(uint256 pid)` - Claim rewards
4. `emergencyWithdraw(uint256 pid)` - Emergency withdrawal (forfeit rewards)

---

## 🚀 Deployment Options

### Option 1: EVM Compatibility Layer (Faster)

**If FlareChain has EVM compatibility:**
- Deploy MasterChef.sol directly
- Use existing Solidity contract
- No need to create Substrate pallet

**Pros:**
- ✅ Contract already exists
- ✅ Fast deployment (hours)
- ✅ Battle-tested code

**Cons:**
- ❌ Higher gas costs than native pallet
- ❌ Less Substrate integration

### Option 2: Native Substrate Pallet (Better Long-term)

**Create pallet-masterchef in Rust:**
- Implement same logic as MasterChef.sol
- Use Substrate FRAME framework
- Integrate directly with runtime

**Pros:**
- ✅ Lower transaction costs
- ✅ Better Substrate integration
- ✅ More efficient

**Cons:**
- ❌ Requires development time (1-2 weeks)
- ❌ Needs testing and auditing
- ❌ Runtime upgrade required

---

## 📝 Deployment Process

### For Solidity Contract (BSC):

1. **Preparation:**
   ```bash
   cd /Desktop/etrid/05-multichain/bridge/adapters/bsc
   npm install
   ```

2. **Configure:**
   - Set .env variables
   - Set ÉTR token address
   - Set reward parameters

3. **Deploy:**
   ```bash
   npm run deploy:masterchef:mainnet
   ```

4. **Post-Deployment:**
   - Transfer 20M ÉTR to contract
   - Add LP pools
   - Verify on BscScan
   - Transfer ownership to multisig

### For Substrate Pallet (FlareChain):

**Step 1: Create Pallet**
```rust
// /Desktop/etrid/src/pallets/pallet-masterchef/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type RewardToken: fungible::Inspect<Self::AccountId>;
        // ... more config
    }

    #[pallet::storage]
    pub type Pools<T> = StorageMap<_, Blake2_128Concat, PoolId, PoolInfo<T>>;

    #[pallet::storage]
    pub type UserStakes<T> = StorageDoubleMap<
        _,
        Blake2_128Concat, PoolId,
        Blake2_128Concat, T::AccountId,
        UserInfo<T>
    >;

    // ... palletimplementation
}
```

**Step 2: Add to Runtime**
```toml
# /Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml
[dependencies]
pallet-masterchef = { path = "../../../src/pallets/pallet-masterchef", default-features = false }
```

```rust
// /Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs
impl pallet_masterchef::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type RewardToken = Balances;
    // ... config
}

construct_runtime!(
    pub struct Runtime {
        // ... existing pallets
        MasterChef: pallet_masterchef,
    }
);
```

**Step 3: Build Runtime**
```bash
cd /Desktop/etrid/05-multichain/flare-chain/node
cargo build --release
```

**Step 4: Runtime Upgrade**
```bash
# Create proposal via governance
# Upload new runtime WASM
# Vote during Consensus Day
# Apply upgrade after approval
```

---

## 🎯 What Web App Currently Shows

**REAL DATA:**
- ✅ Total Value Locked (TVL) - from FlareChain total issuance
- ✅ Daily Rewards - calculated from issuance
- ✅ User ÉTR Balance - from FlareChain when wallet connected

**PLACEHOLDER DATA:**
- ⚠️ Pool-specific APYs (245%, 128%, 85%)
- ⚠️ Individual pool TVLs ($8.5M, $6.2M, $4.8M)
- ⚠️ Pool daily rewards (45K, 28K, 18K ÉTR)
- ⚠️ Staking/harvesting functionality (buttons disabled)

**Why Placeholder?**
These require MasterChef pallet to fetch from blockchain:
- `poolInfo(0)` - Pool 1 stats
- `poolInfo(1)` - Pool 2 stats
- `poolInfo(2)` - Pool 3 stats
- `userInfo(poolId, userAddress)` - User stakes
- `pendingReward(poolId, userAddress)` - User rewards

---

## 📦 Files Reference

### Existing Files:

1. **Solidity Contract:**
   - `/Desktop/etrid/05-multichain/bridge/adapters/bsc/contracts/MasterChef.sol`

2. **Deployment Scripts:**
   - `/Desktop/etrid/05-multichain/bridge/adapters/bsc/scripts/deploy-masterchef-mainnet.ts`
   - `/Desktop/etrid/05-multichain/bridge/adapters/bsc/scripts/deploy-masterchef-testnet.ts`
   - `/Desktop/etrid/05-multichain/bridge/adapters/bsc/scripts/fund-masterchef.ts`

3. **Tests:**
   - `/Desktop/etrid/05-multichain/bridge/adapters/bsc/test/MasterChef.test.ts`

### Files That Need Creation:

1. **Substrate Pallet:**
   - `/Desktop/etrid/src/pallets/pallet-masterchef/src/lib.rs` (DOESN'T EXIST)
   - `/Desktop/etrid/src/pallets/pallet-masterchef/Cargo.toml` (DOESN'T EXIST)

2. **Runtime Integration:**
   - Update `/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml`
   - Update `/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs`

---

## ⏱️ Time Estimates

### Quick Solution (EVM):
- **If FlareChain supports EVM:** Deploy Solidity contract → 1-2 days
- Modify web app to use EVM calls → 1 day
- **Total:** 2-3 days

### Proper Solution (Substrate Pallet):
- Develop pallet-masterchef → 5-7 days
- Testing and debugging → 3-5 days
- Runtime integration → 2-3 days
- Governance proposal + voting → 7 days (Consensus Day cycle)
- **Total:** 3-4 weeks

---

## 🔑 Key Decision Point

**Question:** Does FlareChain runtime include EVM compatibility layer?

**Check:**
```bash
cd /Desktop/etrid/05-multichain/flare-chain/runtime
grep -r "pallet-evm\|pallet-ethereum\|frontier" Cargo.toml
```

**If YES:**
- ✅ Deploy MasterChef.sol via EVM
- ✅ Fast deployment
- ✅ Works immediately

**If NO:**
- ❌ Must create Substrate pallet
- ❌ Longer development time
- ✅ Better long-term solution

---

## 📚 Resources

**MasterChef Contract Design:**
- SushiSwap MasterChef V2: https://github.com/sushiswap/sushiswap/blob/master/contracts/MasterChefV2.sol
- PancakeSwap MasterChef: https://github.com/pancakeswap/pancake-smart-contracts/blob/master/projects/farms-pools/contracts/MasterChef.sol

**Substrate Pallet Development:**
- FRAME Pallet Tutorial: https://docs.substrate.io/tutorials/build-application-logic/
- Pallet Examples: https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame

**ËTRID Existing Pallets:**
- `/Desktop/etrid/src/pallets/` - Reference implementations

---

## ✅ Next Steps

1. **Check EVM Compatibility:**
   ```bash
   grep "pallet-evm" /Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml
   ```

2. **If EVM exists:**
   - Deploy MasterChef.sol
   - Update web app
   - Launch in days

3. **If NO EVM:**
   - Create pallet-masterchef
   - Runtime upgrade via governance
   - Launch in weeks

4. **Update Web App:**
   - Replace placeholder values
   - Connect to smart contract/pallet
   - Enable staking/harvesting buttons
   - Remove warning banner

---

## 🎯 Summary

**What is it?**
- Yield farming contract that rewards liquidity providers with ÉTR tokens

**Where is it?**
- ✅ Solidity version exists for BSC/EVM chains
- ❌ Substrate pallet DOESN'T EXIST yet for FlareChain

**Why not deployed?**
- Need to decide: EVM deployment OR create native pallet
- If EVM: Deploy existing contract (fast)
- If native: Build new pallet (better, slower)

**When will it work?**
- With EVM: 2-3 days
- With pallet: 3-4 weeks

The web app is ready and waiting for the backend (smart contract or pallet) to be deployed!
