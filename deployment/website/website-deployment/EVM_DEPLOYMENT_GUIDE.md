# Adding EVM Support to FlareChain - Complete Guide

**Goal:** Add Frontier EVM to FlareChain runtime so we can deploy MasterChef.sol

**Timeline:** 2-3 days total
- Day 1: Add EVM support to runtime
- Day 2: Deploy and test
- Day 3: Update web app

---

## 📋 Step 1: Add EVM Dependencies to Runtime

### Update Cargo.toml

**File:** `/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml`

Add these dependencies after line 100 (after frame-support):

```toml
# Frontier EVM Support
pallet-evm = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-ethereum = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-base-fee = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-dynamic-fee = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-evm-precompile-simple = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-evm-precompile-modexp = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
pallet-evm-precompile-sha3fips = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
fp-evm = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
fp-rpc = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }
fp-self-contained = { git = "https://github.com/polkadot-evm/frontier", branch = "polkadot-stable2509", default-features = false }

# EVM primitives
ethereum = { version = "0.15", default-features = false }
evm = { version = "0.41", default-features = false }
```

Find the `[features]` section and add EVM pallets to `std`:

```toml
[features]
default = ["std"]
std = [
    # ... existing entries ...

    # EVM Support
    "pallet-evm/std",
    "pallet-ethereum/std",
    "pallet-base-fee/std",
    "pallet-dynamic-fee/std",
    "pallet-evm-precompile-simple/std",
    "pallet-evm-precompile-modexp/std",
    "pallet-evm-precompile-sha3fips/std",
    "fp-evm/std",
    "fp-rpc/std",
    "fp-self-contained/std",
    "ethereum/std",
    "evm/std",
]
```

---

## 📋 Step 2: Configure EVM in Runtime

### Update runtime/src/lib.rs

**File:** `/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs`

#### 2.1: Add imports at top of file

```rust
use fp_evm::FeeCalculator;
use pallet_evm::{
    EnsureAddressNever, EnsureAddressRoot, HashedAddressMapping,
};
```

#### 2.2: Add EVM configuration constants

```rust
// EVM Configuration
pub const WEIGHT_PER_GAS: u64 = 20_000;

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
    fn min_gas_price() -> (sp_core::U256, Weight) {
        // 1 gwei (same as BSC)
        (sp_core::U256::from(1_000_000_000u128), Weight::zero())
    }
}

pub struct FindAuthorTruncated<F>(sp_std::marker::PhantomData<F>);
impl<F: frame_support::traits::FindAuthor<u32>> frame_support::traits::FindAuthor<sp_core::H160>
    for FindAuthorTruncated<F>
{
    fn find_author<'a, I>(digests: I) -> Option<sp_core::H160>
    where
        I: 'a + IntoIterator<Item = (sp_runtime::ConsensusEngineId, &'a [u8])>,
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(sp_core::H160::from_slice(&authority_id.to_raw_vec()[4..24]));
        }
        None
    }
}
```

#### 2.3: Configure pallet-evm

```rust
parameter_types! {
    pub BlockGasLimit: sp_core::U256 = sp_core::U256::from(u64::MAX);
    pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
    pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
    pub SuicideQuickClearLimit: u32 = 0;
}

pub struct FrontierPrecompiles<R>(sp_std::marker::PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
    R: pallet_evm::Config,
{
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn used_addresses() -> [sp_core::H160; 8] {
        [
            addr(1), addr(2), addr(3), addr(4),
            addr(5), addr(6), addr(7), addr(8),
        ]
    }
}

fn addr(a: u64) -> sp_core::H160 {
    sp_core::H160::from_low_u64_be(a)
}

impl<R> pallet_evm::PrecompileSet for FrontierPrecompiles<R>
where
    R: pallet_evm::Config,
{
    fn execute(&self, handle: &mut impl pallet_evm::PrecompileHandle) -> Option<pallet_evm::PrecompileResult> {
        match handle.code_address() {
            a if a == addr(1) => Some(pallet_evm_precompile_simple::ECRecover::execute(handle)),
            a if a == addr(2) => Some(pallet_evm_precompile_simple::Sha256::execute(handle)),
            a if a == addr(3) => Some(pallet_evm_precompile_simple::Ripemd160::execute(handle)),
            a if a == addr(4) => Some(pallet_evm_precompile_simple::Identity::execute(handle)),
            a if a == addr(5) => Some(pallet_evm_precompile_modexp::Modexp::execute(handle)),
            a if a == addr(8) => Some(pallet_evm_precompile_sha3fips::Sha3FIPS256::execute(handle)),
            _ => None,
        }
    }

    fn is_precompile(&self, address: sp_core::H160, _gas: u64) -> pallet_evm::IsPrecompileResult {
        pallet_evm::IsPrecompileResult::Answer {
            is_precompile: Self::used_addresses().contains(&address),
            extra_cost: 0,
        }
    }
}

impl pallet_evm::Config for Runtime {
    type FeeCalculator = FixedGasPrice;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type WeightPerGas = WeightPerGas;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type CallOrigin = EnsureAddressRoot<AccountId>;
    type WithdrawOrigin = EnsureAddressNever<AccountId>;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type PrecompilesType = FrontierPrecompiles<Self>;
    type PrecompilesValue = PrecompilesValue;
    type ChainId = ConstU64<33396>; // 0x8274 in hex (FlareChain chain ID)
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type OnChargeTransaction = ();
    type OnCreate = ();
    type FindAuthor = FindAuthorTruncated<Aura>;
    type GasLimitPovSizeRatio = ConstU64<4>;
    type SuicideQuickClearLimit = SuicideQuickClearLimit;
    type Timestamp = Timestamp;
    type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}
```

#### 2.4: Configure pallet-ethereum

```rust
impl pallet_ethereum::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
    type PostLogContent = pallet_ethereum::PostLogContent::BlockAndTxnHashes;
    type ExtraDataLength = ConstU32<30>;
}
```

#### 2.5: Configure pallet-base-fee

```rust
impl pallet_base_fee::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Threshold = pallet_base_fee::DefaultBaseFeeThreshold;
    type DefaultBaseFeePerGas = ConstU128<1_000_000_000>; // 1 gwei
    type DefaultElasticity = pallet_base_fee::DefaultElasticity;
}
```

#### 2.6: Add pallets to construct_runtime!

Find the `construct_runtime!` macro and add:

```rust
construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...

        // EVM Support
        EVM: pallet_evm,
        Ethereum: pallet_ethereum,
        BaseFee: pallet_base_fee,
        DynamicFee: pallet_dynamic_fee,
    }
);
```

---

## 📋 Step 3: Build Runtime

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node

# Clean previous build
cargo clean

# Build runtime
cargo build --release

# This will take 30-60 minutes
```

**Expected output:**
```
   Compiling flare-chain-runtime v0.1.0
   Compiling flare-chain-node v0.1.0
    Finished release [optimized] target(s) in 45m 23s
```

---

## 📋 Step 4: Deploy MasterChef.sol

### 4.1: Prepare Deployment Environment

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc

# Install dependencies
npm install

# Create .env file
cat > .env << 'EOF'
# FlareChain EVM
FLARECHAIN_RPC_HTTP=http://98.71.91.84:9933
FLARECHAIN_RPC_WS=ws://98.71.91.84:9944
FLARECHAIN_CHAIN_ID=33396

# Deployment account (use validator key or create new)
PRIVATE_KEY=your_private_key_here

# MasterChef parameters
REWARD_PER_BLOCK=2890000000000000000  # 2.89 ÉTR
START_BLOCK=1000  # Start 1000 blocks from now
EOF
```

### 4.2: Create FlareChain Deployment Script

**File:** `scripts/deploy-masterchef-flarechain.ts`

```typescript
import { ethers } from "hardhat";

async function main() {
  console.log("\n🚀 MASTERCHEF DEPLOYMENT TO FLARECHAIN\n");
  console.log("=" .repeat(60));

  // Get deployer
  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", await deployer.getAddress());

  // Check balance
  const balance = await ethers.provider.getBalance(deployer.getAddress());
  console.log("Balance:", ethers.formatEther(balance), "ÉTR");

  // MasterChef parameters
  const ETR_TOKEN_ADDRESS = "0x0000000000000000000000000000000000000000"; // Native ÉTR (check if exists)
  const REWARD_PER_BLOCK = ethers.parseEther("2.89");
  const currentBlock = await ethers.provider.getBlockNumber();
  const START_BLOCK = currentBlock + 1000;

  console.log("\nMasterChef Config:");
  console.log("  Reward Token:", ETR_TOKEN_ADDRESS);
  console.log("  Reward/Block:", ethers.formatEther(REWARD_PER_BLOCK), "ÉTR");
  console.log("  Start Block:", START_BLOCK);
  console.log("  Current Block:", currentBlock);

  // Deploy
  console.log("\n⏳ Deploying MasterChef...");
  const MasterChef = await ethers.getContractFactory("MasterChef");
  const masterChef = await MasterChef.deploy(
    ETR_TOKEN_ADDRESS,
    REWARD_PER_BLOCK,
    START_BLOCK
  );

  await masterChef.waitForDeployment();
  const address = await masterChef.getAddress();

  console.log("\n✅ DEPLOYED!");
  console.log("=" .repeat(60));
  console.log("MasterChef Address:", address);
  console.log("=" .repeat(60));

  // Save deployment
  const fs = require("fs");
  fs.writeFileSync(
    "masterchef-flarechain-deployment.json",
    JSON.stringify({
      network: "flarechain",
      chainId: 33396,
      address: address,
      rewardPerBlock: REWARD_PER_BLOCK.toString(),
      startBlock: START_BLOCK,
      deployer: await deployer.getAddress(),
      timestamp: new Date().toISOString(),
    }, null, 2)
  );

  console.log("\n💾 Deployment info saved to masterchef-flarechain-deployment.json");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### 4.3: Update hardhat.config.ts

Add FlareChain network:

```typescript
const config: HardhatUserConfig = {
  networks: {
    // ... existing networks ...

    flarechain: {
      url: process.env.FLARECHAIN_RPC_HTTP || "http://98.71.91.84:9933",
      chainId: 33396,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      gas: 8000000,
      gasPrice: 1000000000, // 1 gwei
    },
  },
};
```

### 4.4: Deploy

```bash
# Deploy to FlareChain
npx hardhat run scripts/deploy-masterchef-flarechain.ts --network flarechain
```

---

## 📋 Step 5: Update Web App

### 5.1: Add EVM Contract Integration

**File:** `/Desktop/etrid/etrid-hostinger-deployment/apps/masterchef/index.html`

Add after existing blockchain connection code:

```javascript
// EVM Contract ABI (simplified)
const MASTERCHEF_ABI = [
  "function poolLength() view returns (uint256)",
  "function poolInfo(uint256) view returns (address lpToken, uint256 allocPoint, uint256 lastRewardBlock, uint256 accRewardPerShare, uint256 totalStaked)",
  "function userInfo(uint256, address) view returns (uint256 amount, uint256 rewardDebt, uint256 pendingRewards)",
  "function pendingReward(uint256, address) view returns (uint256)",
  "function deposit(uint256, uint256)",
  "function withdraw(uint256, uint256)",
  "function harvest(uint256)",
  "function rewardPerBlock() view returns (uint256)",
  "function totalAllocPoint() view returns (uint256)",
];

const MASTERCHEF_ADDRESS = "YOUR_DEPLOYED_ADDRESS_HERE"; // From deployment

let evmProvider = null;
let masterChefContract = null;

// Connect to EVM
async function connectToEVM() {
  try {
    // Use EVM RPC
    evmProvider = new ethers.JsonRpcProvider("http://98.71.91.84:9933");

    // Create contract instance
    masterChefContract = new ethers.Contract(
      MASTERCHEF_ADDRESS,
      MASTERCHEF_ABI,
      evmProvider
    );

    console.log("✅ Connected to FlareChain EVM");
    return true;
  } catch (error) {
    console.error("Failed to connect to EVM:", error);
    return false;
  }
}

// Fetch REAL pool data from contract
async function fetchRealPoolData() {
  if (!masterChefContract) return;

  try {
    const poolCount = await masterChefContract.poolLength();

    for (let pid = 0; pid < Math.min(poolCount, 3); pid++) {
      const poolInfo = await masterChefContract.poolInfo(pid);
      const totalAllocPoint = await masterChefContract.totalAllocPoint();
      const rewardPerBlock = await masterChefContract.rewardPerBlock();

      // Calculate real APY
      const poolShare = Number(poolInfo.allocPoint) / Number(totalAllocPoint);
      const poolRewardPerBlock = Number(ethers.formatEther(rewardPerBlock)) * poolShare;
      const dailyRewards = poolRewardPerBlock * 14400; // ~6s blocks
      const yearlyRewards = dailyRewards * 365;
      const poolTVL = Number(ethers.formatEther(poolInfo.totalStaked));
      const apy = poolTVL > 0 ? (yearlyRewards / poolTVL) * 100 : 0;

      // Update UI with REAL data
      console.log(`Pool ${pid}: APY=${apy}%, TVL=${poolTVL} LP`);

      // Update DOM elements (you'll need to add IDs to HTML)
      // document.getElementById(`pool${pid}Apy`).textContent = apy.toFixed(0) + '%';
      // document.getElementById(`pool${pid}Tvl`).textContent = '$' + (poolTVL * 8).toFixed(1) + 'M';
    }
  } catch (error) {
    console.error("Failed to fetch pool data:", error);
  }
}

// Fetch user data
async function fetchUserPoolData() {
  if (!masterChefContract || !currentAccount) return;

  try {
    // Get user's Ethereum-style address from Substrate account
    // (This requires address mapping)
    const ethAddress = "0x..."; // TODO: Map Substrate → Ethereum address

    for (let pid = 0; pid < 3; pid++) {
      const userInfo = await masterChefContract.userInfo(pid, ethAddress);
      const pending = await masterChefContract.pendingReward(pid, ethAddress);

      console.log(`Pool ${pid}: Staked=${ethers.formatEther(userInfo.amount)}, Pending=${ethers.formatEther(pending)}`);
    }
  } catch (error) {
    console.error("Failed to fetch user data:", error);
  }
}
```

### 5.2: Update Initialization

```javascript
// On page load
async function initMasterChef() {
  await connectToBlockchain(); // Substrate connection
  await connectToEVM(); // EVM connection
  await fetchMasterChefData(); // Old TVL data
  await fetchRealPoolData(); // NEW: Real pool data from contract
}
```

---

## 📋 Step 6: Test Everything

### Test 1: Verify EVM is Running

```bash
# Test EVM RPC
curl -X POST http://98.71.91.84:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_chainId",
    "params": [],
    "id": 1
  }'

# Expected: {"jsonrpc":"2.0","result":"0x8274","id":1}
```

### Test 2: Check MasterChef Deployment

```bash
# Check contract code exists
curl -X POST http://98.71.91.84:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_getCode",
    "params": ["MASTERCHEF_ADDRESS", "latest"],
    "id": 1
  }'

# Expected: Long hex string (contract bytecode)
```

### Test 3: Call Contract

```bash
# Get pool length
cast call MASTERCHEF_ADDRESS \
  "poolLength()(uint256)" \
  --rpc-url http://98.71.91.84:9933
```

---

## 📋 Step 7: Add LP Pools

Once deployed, add pools via Polkadot.js Apps:

```
1. Go to: https://polkadot.js.org/apps/?rpc=ws://98.71.91.84:9944#/extrinsics

2. Select: evm > call

3. Parameters:
   - source: Your EVM address
   - target: MASTERCHEF_ADDRESS
   - input: Encoded "add(uint256,address,bool)" call
   - value: 0
   - gas_limit: 500000
   - max_fee_per_gas: 1000000000

4. Submit transaction
```

Or use ethers.js:

```javascript
const masterChef = new ethers.Contract(address, abi, signer);
await masterChef.add(
  1000,  // allocation points
  LP_TOKEN_ADDRESS,
  false  // withUpdate
);
```

---

## 🎯 Timeline

**Day 1: Add EVM to Runtime**
- Morning: Update Cargo.toml
- Afternoon: Configure pallets in lib.rs
- Evening: Build runtime (45 min)
- Deploy new runtime to validators

**Day 2: Deploy MasterChef**
- Morning: Prepare deployment scripts
- Afternoon: Deploy MasterChef.sol
- Evening: Add initial LP pools
- Test contract calls

**Day 3: Update Web App**
- Morning: Add ethers.js integration
- Afternoon: Update UI with real data
- Evening: Test and deploy
- Remove placeholder notice

---

## ✅ Checklist

- [ ] Add Frontier dependencies to Cargo.toml
- [ ] Configure pallet-evm in runtime
- [ ] Configure pallet-ethereum in runtime
- [ ] Configure pallet-base-fee in runtime
- [ ] Add pallets to construct_runtime!
- [ ] Build runtime (`cargo build --release`)
- [ ] Deploy new runtime to all validators
- [ ] Test EVM RPC endpoint
- [ ] Deploy MasterChef.sol
- [ ] Verify contract deployment
- [ ] Add LP pools to contract
- [ ] Update web app with ethers.js
- [ ] Fetch real pool data from contract
- [ ] Test staking functionality
- [ ] Remove placeholder notice from UI
- [ ] Deploy updated web app

---

## 🆘 Troubleshooting

**Build Errors:**
- Check Frontier branch matches Polkadot SDK version
- May need to use specific commit hash
- Check for dependency conflicts

**Deployment Fails:**
- Ensure EVM is actually running
- Check gas limit is sufficient
- Verify private key has ÉTR balance

**Contract Not Callable:**
- Verify contract address is correct
- Check ABI matches deployed contract
- Ensure RPC endpoint is accessible

---

**This is the fastest path to get MasterChef working! The contract already exists, we just need EVM support in the runtime.**
