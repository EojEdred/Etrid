# FlareSwap DEX - Complete Deployment Guide

## Overview

FlareSwap is a production-ready decentralized exchange (DEX) with integrated yield farming capabilities. This guide covers deployment, testing, and operation.

## System Architecture

### Core Components

1. **FlareSwap DEX (Uniswap V2 Compatible)**
   - `FlareSwapFactory`: Creates and manages trading pairs
   - `FlareSwapPair`: Individual AMM liquidity pools (x * y = k)
   - `FlareSwapRouter`: User-facing swap and liquidity interface
   - `WETH`: Wrapped ETH for ETH/token pairs

2. **MasterChef Yield Farming**
   - Distributes ETR token rewards to LP token stakers
   - Supports multiple pools with configurable allocation points
   - Fair reward distribution based on stake amount and time

## Test Results

### Comprehensive Test Suite: 73 Tests Passing

#### Test Coverage
```
File                    | % Stmts | % Branch | % Funcs | % Lines
------------------------|---------|----------|---------|--------
core/                   |  81.08% |  56.67%  |  90.00% | 86.36%
farming/MasterChef.sol  |  94.92% |  81.58%  | 100.00% | 96.20%
periphery/              |  48.45% |  33.33%  |  48.39% | 50.81%
Overall                 |  72.52% |  50.80%  |  74.16% | 75.72%
```

#### Test Categories

1. **Basic Functionality Tests** (9 tests)
   - Factory pair creation
   - Router swap operations
   - Liquidity add/remove
   - Reserve tracking

2. **MasterChef Tests** (16 tests)
   - Pool management
   - Staking/unstaking
   - Reward distribution
   - Access control
   - Emergency withdrawals

3. **Integration Tests** (14 tests)
   - Complete user journeys (wallet → swap → LP → stake)
   - Multi-user staking competition
   - ETH trading and staking
   - Emergency scenarios

4. **Security Tests** (25 tests)
   - Access control (factory, MasterChef, pairs)
   - Reentrancy protection
   - Input validation
   - Economic attack protection
   - Slippage protection

5. **Stress Tests** (9 tests)
   - 100 consecutive swaps
   - Large pool operations (10M+ tokens)
   - 10 simultaneous users
   - 20+ pools in MasterChef
   - Rapid deposit/withdraw cycles

### Performance Benchmarks

```
Operation              | Gas Used
-----------------------|----------
Token Swap             | ~167,820
Add Liquidity          | ~148,749
Remove Liquidity       | ~150,000 (est)
Stake LP Tokens        | ~120,000 (est)
Claim Rewards          | ~100,000 (est)
```

## Deployment Instructions

### Prerequisites

```bash
# Install dependencies
npm install

# Compile contracts
npx hardhat compile

# Run tests
npx hardhat test

# Run test coverage
npx hardhat coverage
```

### Environment Setup

Create `.env` file:

```env
# Network RPC URLs
GOERLI_RPC_URL=https://goerli.infura.io/v3/YOUR_KEY
SEPOLIA_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
MAINNET_RPC_URL=https://mainnet.infura.io/v3/YOUR_KEY

# Private Keys (NEVER commit these!)
PRIVATE_KEY=your_private_key_here

# API Keys for verification
ETHERSCAN_API_KEY=your_etherscan_key
BSCSCAN_API_KEY=your_bscscan_key

# Optional: Gas reporting
REPORT_GAS=true
COINMARKETCAP_API_KEY=your_cmc_key
```

### Deployment Scripts

#### Option 1: Basic DEX Only

```bash
npx hardhat run scripts/deploy.js --network localhost
```

Deploys:
- WETH
- FlareSwapFactory
- FlareSwapRouter

#### Option 2: Full Stack (DEX + Staking)

```bash
npx hardhat run scripts/deploy-full.js --network localhost
```

Deploys:
- All DEX contracts
- ETR reward token
- MasterChef staking contract
- Test tokens (USDC, DAI)
- Initial trading pairs
- Pre-configured staking pools

### Network-Specific Deployment

#### Local Testnet

```bash
# Terminal 1: Start local node
npx hardhat node

# Terminal 2: Deploy
npx hardhat run scripts/deploy-full.js --network localhost
```

#### Goerli Testnet

```bash
npx hardhat run scripts/deploy-full.js --network goerli
```

#### BSC Testnet

```bash
npx hardhat run scripts/deploy-full.js --network bsc_testnet
```

#### Mainnet (Production)

⚠️ **CRITICAL**: Before mainnet deployment:

1. Complete security audit
2. Run all tests on testnet
3. Verify contract addresses
4. Test with small amounts first
5. Set up monitoring and alerts

```bash
npx hardhat run scripts/deploy-full.js --network mainnet
```

## Post-Deployment Configuration

### 1. Verify Contracts on Block Explorer

```bash
npx hardhat run scripts/verify.js --network <network>
```

### 2. Configure Initial Liquidity

Add liquidity to bootstrap trading:

```javascript
// Approve tokens
await etrToken.approve(router.address, amountETR);
await usdc.approve(router.address, amountUSDC);

// Add liquidity
await router.addLiquidity(
  etrToken.address,
  usdc.address,
  amountETR,
  amountUSDC,
  0, // min amounts
  0,
  owner.address,
  deadline
);
```

### 3. Set Up Staking Pools

Configure reward allocation:

```javascript
// Add pool with allocation points
await masterChef.add(
  1000,              // allocation points
  lpTokenAddress,    // LP token to stake
  true               // update all pools
);

// Update reward rate if needed
await masterChef.updateRewardPerBlock(newRate);
```

### 4. Transfer Ownership (Recommended)

For production, transfer ownership to a multi-sig wallet or DAO:

```javascript
// MasterChef
await masterChef.transferOwnership(multiSigAddress);

// Factory (fee setter)
await factory.setFeeToSetter(daoAddress);
```

## Integration Guide

### Frontend Integration

#### Contract ABIs

After deployment, ABIs are available in:
```
artifacts/contracts/[ContractPath]/[ContractName].sol/[ContractName].json
```

#### Example: Web3 Integration

```javascript
import { ethers } from 'ethers';
import RouterABI from './abis/FlareSwapRouter.json';
import FactoryABI from './abis/FlareSwapFactory.json';
import MasterChefABI from './abis/MasterChef.json';

// Initialize contracts
const provider = new ethers.providers.Web3Provider(window.ethereum);
const signer = provider.getSigner();

const router = new ethers.Contract(ROUTER_ADDRESS, RouterABI.abi, signer);
const factory = new ethers.Contract(FACTORY_ADDRESS, FactoryABI.abi, signer);
const masterChef = new ethers.Contract(MASTERCHEF_ADDRESS, MasterChefABI.abi, signer);

// Example: Swap tokens
async function swapTokens(tokenIn, tokenOut, amountIn, slippage = 0.5) {
  const path = [tokenIn, tokenOut];
  const deadline = Math.floor(Date.now() / 1000) + 60 * 20; // 20 minutes

  const amounts = await router.getAmountsOut(amountIn, path);
  const amountOutMin = amounts[1].mul(100 - slippage).div(100);

  const tx = await router.swapExactTokensForTokens(
    amountIn,
    amountOutMin,
    path,
    await signer.getAddress(),
    deadline
  );

  await tx.wait();
}

// Example: Add liquidity
async function addLiquidity(tokenA, tokenB, amountA, amountB) {
  await tokenA.approve(router.address, amountA);
  await tokenB.approve(router.address, amountB);

  const tx = await router.addLiquidity(
    tokenA.address,
    tokenB.address,
    amountA,
    amountB,
    amountA.mul(95).div(100), // 5% slippage
    amountB.mul(95).div(100),
    await signer.getAddress(),
    Math.floor(Date.now() / 1000) + 60 * 20
  );

  await tx.wait();
}

// Example: Stake LP tokens
async function stakeLPTokens(poolId, amount) {
  const pairAddress = await factory.getPair(tokenA, tokenB);
  const pair = new ethers.Contract(pairAddress, PairABI.abi, signer);

  await pair.approve(masterChef.address, amount);

  const tx = await masterChef.deposit(poolId, amount);
  await tx.wait();
}

// Example: Claim rewards
async function claimRewards(poolId) {
  const userInfo = await masterChef.userInfo(poolId, userAddress);
  const tx = await masterChef.withdraw(poolId, 0); // Withdraw 0 to claim rewards
  await tx.wait();
}
```

## Monitoring and Maintenance

### Key Metrics to Monitor

1. **Trading Volume**
   ```javascript
   const reserves = await pair.getReserves();
   const volume = calculateVolume(reserves, previousReserves);
   ```

2. **Total Value Locked (TVL)**
   ```javascript
   const totalSupply = await pair.totalSupply();
   const reserves = await pair.getReserves();
   const tvl = calculateTVL(reserves);
   ```

3. **Staking Participation**
   ```javascript
   const totalStaked = await masterChef.userInfo(poolId, address);
   const pendingRewards = await masterChef.pendingReward(poolId, address);
   ```

4. **Gas Costs**
   - Monitor transaction costs
   - Optimize operations during low-fee periods

### Emergency Procedures

#### Pause Trading (if needed)

FlareSwap has no built-in pause functionality by design (trustless). However, you can:

1. Stop adding new pools to MasterChef
2. Set fee receiver to zero address temporarily
3. Communicate issues to community via official channels

#### Emergency Withdrawal

Users can always withdraw their LP tokens using:

```javascript
await masterChef.emergencyWithdraw(poolId);
```

This withdraws LP tokens without claiming rewards (use only in emergencies).

## Security Best Practices

### Before Mainnet Launch

1. ✅ **Complete Security Audit**
   - Hire professional auditors (Certik, OpenZeppelin, etc.)
   - Review all unsafe code blocks
   - Test reentrancy protection
   - Verify access controls

2. ✅ **Bug Bounty Program**
   - Set up ImmuneFi or similar program
   - Offer meaningful rewards
   - Clear disclosure process

3. ✅ **Testnet Validation**
   - Deploy to multiple testnets
   - Run all test suites
   - Perform manual testing
   - Simulate attack scenarios

4. ✅ **Gradual Rollout**
   - Start with limited liquidity
   - Monitor closely for 48+ hours
   - Gradually increase caps
   - Community feedback loop

### Ongoing Security

1. **Monitor Transactions**
   - Set up alerts for large swaps
   - Watch for unusual patterns
   - Track failed transactions

2. **Regular Audits**
   - Annual security reviews
   - Update dependencies
   - Review any modifications

3. **Community Engagement**
   - Active Discord/Telegram
   - Rapid response to issues
   - Transparent communication

## Troubleshooting

### Common Issues

#### "Insufficient Output Amount"
- **Cause**: Slippage too tight or price moved
- **Solution**: Increase slippage tolerance or check pool liquidity

#### "Pair Does Not Exist"
- **Cause**: Trading pair not created yet
- **Solution**: Create pair via factory.createPair()

#### "EXPIRED"
- **Cause**: Transaction deadline passed
- **Solution**: Increase deadline or submit faster

#### "LOCKED"
- **Cause**: Reentrancy attempt
- **Solution**: This is working as intended - indicates reentrancy protection

### Getting Help

- **Documentation**: https://docs.etrid.io
- **Discord**: https://discord.gg/etrid
- **GitHub Issues**: https://github.com/etrid/flareswap/issues
- **Security**: security@etrid.io (for vulnerabilities)

## License

GPL-3.0 License - See LICENSE file for details.

## Acknowledgments

- Uniswap V2 (protocol design)
- SushiSwap (MasterChef design)
- OpenZeppelin (security patterns)
- Etrid Foundation (funding and support)

---

**Last Updated**: October 30, 2025
**Version**: 1.0.0
**Status**: Production Ready (pending security audit)
