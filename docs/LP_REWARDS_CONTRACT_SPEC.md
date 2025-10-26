# Community LP Rewards Contract Specification

**Version**: 1.0
**Last Updated**: October 24, 2025
**Chain**: Binance Smart Chain (BSC)
**Contract Type**: MasterChef V2 Fork (LP Staking + Rewards)

---

## 📋 Overview

This document specifies the smart contract for distributing ÉTR rewards to liquidity providers on PancakeSwap (BSC) and Raydium (Solana).

**Purpose**: Incentivize community members to provide liquidity for ÉTR/BNB and ÉTR/SOL pools by offering high APR rewards paid in ÉTR tokens.

**Total Allocation**: 20,000,000 ÉTR over 6 months (3,333,333 ÉTR per month)

**Key Features**:
- Time-decaying emission rate (150% APR → 35% APR)
- Multiple pool support (ÉTR/BNB on BSC, ÉTR/SOL on Solana)
- Auto-compounding option
- Emergency withdraw function
- Timelock for admin functions

---

## 🏗️ Contract Architecture

### Base Template

**Option 1: PancakeSwap MasterChef V2 Fork** (Recommended for BSC)
- **Pros**: Battle-tested, widely used, community trust
- **Cons**: Slightly higher gas costs than custom implementation
- **Source**: https://github.com/pancakeswap/pancake-smart-contracts/blob/master/projects/farms-pools/contracts/MasterChefV2.sol

**Option 2: Sushiswap MiniChef Fork** (Alternative)
- **Pros**: Lower gas costs, simpler codebase
- **Cons**: Less familiar to BSC users
- **Source**: https://github.com/sushiswap/sushiswap/blob/master/protocols/masterchef/contracts/MiniChefV2.sol

**Option 3: Custom Implementation** (Only if needed)
- **Pros**: Fully optimized for our use case
- **Cons**: Requires security audit, more development time
- **Recommendation**: Avoid unless specific requirements not met by forks

**Decision**: Use **PancakeSwap MasterChef V2 Fork** for familiarity and trust.

---

## 📊 Emission Schedule

### Month-by-Month Breakdown

| Month | APR Target | ÉTR/Month | ÉTR/Day | ÉTR/Block* | Target TVL |
|-------|------------|-----------|---------|-----------|------------|
| Month 1 | 150% | 2,500,000 | 83,333 | 2.89 | $50,000 |
| Month 2 | 120% | 3,500,000 | 116,667 | 4.05 | $100,000 |
| Month 3 | 90% | 4,000,000 | 133,333 | 4.63 | $200,000 |
| Month 4 | 70% | 4,000,000 | 133,333 | 4.63 | $350,000 |
| Month 5 | 50% | 3,500,000 | 116,667 | 4.05 | $500,000 |
| Month 6 | 35% | 2,500,000 | 83,333 | 2.89 | $750,000 |
| **Total** | - | **20,000,000** | - | - | **$750k+** |

\* BSC block time: ~3 seconds (28,800 blocks/day)

### Implementation: Phased Emission Reduction

**Phase 1 (Month 1)**: 2.89 ÉTR per block
**Phase 2 (Month 2)**: 4.05 ÉTR per block
**Phase 3 (Month 3-4)**: 4.63 ÉTR per block
**Phase 4 (Month 5)**: 4.05 ÉTR per block
**Phase 5 (Month 6)**: 2.89 ÉTR per block

**Trigger Mechanism**: Manual update via `updateEmissionRate()` function (timelock protected).

---

## 🔧 Contract Interface

### Core Functions

#### 1. User Functions (LP Interactions)

```solidity
/**
 * @notice Deposit LP tokens to earn ÉTR rewards
 * @param _pid Pool ID (0 for ÉTR/BNB)
 * @param _amount Amount of LP tokens to stake
 */
function deposit(uint256 _pid, uint256 _amount) external;

/**
 * @notice Withdraw LP tokens and claim rewards
 * @param _pid Pool ID
 * @param _amount Amount of LP tokens to withdraw
 */
function withdraw(uint256 _pid, uint256 _amount) external;

/**
 * @notice Claim pending ÉTR rewards without withdrawing LP tokens
 * @param _pid Pool ID
 */
function harvest(uint256 _pid) external;

/**
 * @notice Emergency withdraw LP tokens WITHOUT claiming rewards
 * @param _pid Pool ID
 */
function emergencyWithdraw(uint256 _pid) external;

/**
 * @notice View pending ÉTR rewards for a user
 * @param _pid Pool ID
 * @param _user User address
 * @return Pending ÉTR rewards
 */
function pendingReward(uint256 _pid, address _user) external view returns (uint256);
```

#### 2. Admin Functions (Governance)

```solidity
/**
 * @notice Add a new LP pool for rewards
 * @param _allocPoint Allocation points for this pool (weight vs other pools)
 * @param _lpToken LP token address (PancakeSwap LP)
 * @param _withUpdate Update all pools before adding
 */
function add(uint256 _allocPoint, address _lpToken, bool _withUpdate) external onlyOwner;

/**
 * @notice Update allocation points for existing pool
 * @param _pid Pool ID
 * @param _allocPoint New allocation points
 * @param _withUpdate Update all pools before changing
 */
function set(uint256 _pid, uint256 _allocPoint, bool _withUpdate) external onlyOwner;

/**
 * @notice Update ÉTR emission rate (per block)
 * @param _rewardPerBlock New emission rate
 */
function updateEmissionRate(uint256 _rewardPerBlock) external onlyOwner;

/**
 * @notice Transfer ownership (with 24-hour timelock)
 * @param _newOwner New owner address
 */
function transferOwnership(address _newOwner) external onlyOwner;
```

---

## 🎯 Deployment Configuration

### BSC Mainnet Deployment Parameters

```solidity
// Contract: EtridMasterChef
// Network: BSC Mainnet (Chain ID: 56)

constructor(
    IERC20 _rewardToken,        // ÉTR token address on BSC
    uint256 _rewardPerBlock,    // Initial: 2.89 ÉTR per block (Month 1)
    uint256 _startBlock         // Block number to start rewards
) {
    rewardToken = _rewardToken;
    rewardPerBlock = _rewardPerBlock;
    startBlock = _startBlock;
}

// Initial Pool Setup (ÉTR/BNB)
add(
    1000,                       // allocPoint: 100% of rewards go to this pool
    0x...,                      // LP token address (PancakeSwap ÉTR/BNB LP)
    false                       // withUpdate: false (first pool)
);
```

### Deployment Checklist

**Pre-Deployment**:
- [ ] Deploy ÉTR token on BSC (`0x...`)
- [ ] Create PancakeSwap LP pool (ÉTR/BNB)
- [ ] Get PancakeSwap LP token address
- [ ] Calculate start block (target: Nov 5, 10:00 AM UTC)
- [ ] Transfer 20M ÉTR to MasterChef contract
- [ ] Set up multi-sig wallet for admin functions

**Deployment**:
- [ ] Deploy MasterChef contract with parameters above
- [ ] Verify contract on BscScan
- [ ] Add ÉTR/BNB pool via `add()` function
- [ ] Transfer ownership to multi-sig (24-hour timelock)
- [ ] Test deposit/withdraw on testnet first

**Post-Deployment**:
- [ ] Announce contract address to community
- [ ] Create UI guide (how to stake LP tokens)
- [ ] Monitor for first deposits and rewards distribution
- [ ] Schedule Month 2 emission rate update (Nov 30)

---

## 🔐 Security Considerations

### Critical Security Features

1. **Reentrancy Protection**:
   ```solidity
   using ReentrancyGuard for all state-changing functions;
   ```

2. **Timelock for Admin Functions**:
   ```solidity
   modifier onlyOwner() {
       require(msg.sender == owner, "Not owner");
       require(block.timestamp >= adminActionTimestamp + 24 hours, "Timelock");
       _;
   }
   ```

3. **Emergency Pause**:
   ```solidity
   bool public paused;

   modifier whenNotPaused() {
       require(!paused, "Contract paused");
       _;
   }

   function pause() external onlyOwner {
       paused = true;
   }
   ```

4. **Max Withdrawal Limit** (Anti-Whale):
   ```solidity
   // Optional: Limit withdrawals to prevent single user draining rewards
   uint256 public constant MAX_WITHDRAWAL_PER_TX = 1000000 * 10**18; // 1M ÉTR
   ```

5. **Audit Requirements**:
   - **Option 1**: Use unmodified PancakeSwap MasterChef V2 (no audit needed, already audited)
   - **Option 2**: If custom modifications → get CertiK or PeckShield audit ($5k-15k)
   - **Recommendation**: Use unmodified fork to avoid audit costs

---

## 🧪 Testing Strategy

### Unit Tests (Foundry or Hardhat)

**Test Coverage Requirements**: >95%

```solidity
// Test Cases

1. Deposit Tests:
   - [x] User can deposit LP tokens
   - [x] User balance updates correctly
   - [x] Pool total staked updates correctly
   - [x] Cannot deposit 0 amount
   - [x] Cannot deposit to non-existent pool

2. Withdrawal Tests:
   - [x] User can withdraw LP tokens
   - [x] User receives correct amount
   - [x] Cannot withdraw more than deposited
   - [x] Cannot withdraw from empty balance

3. Reward Calculation Tests:
   - [x] Rewards accrue per block correctly
   - [x] Multiple users share rewards proportionally
   - [x] Rewards stop when pool ends
   - [x] PendingReward view function matches actual rewards

4. Harvest Tests:
   - [x] User can claim rewards without withdrawing LP
   - [x] Rewards transfer to user wallet
   - [x] Pending rewards reset after harvest

5. Admin Tests:
   - [x] Only owner can add pools
   - [x] Only owner can update emission rate
   - [x] Timelock prevents immediate admin actions
   - [x] Emergency pause works correctly

6. Edge Cases:
   - [x] First depositor receives correct rewards
   - [x] Deposit/withdraw in same block
   - [x] Large number of users (gas optimization)
   - [x] Rounding errors < 1 wei
```

### Integration Tests (BSC Testnet)

**Testnet Deployment**: https://testnet.bscscan.com/

```bash
# Deploy to BSC Testnet
npx hardhat run scripts/deploy-masterchef.ts --network bscTestnet

# Verify contract
npx hardhat verify --network bscTestnet <CONTRACT_ADDRESS> <CONSTRUCTOR_ARGS>

# Integration test scenarios:
1. Deploy ÉTR token on testnet
2. Create PancakeSwap LP pool (ÉTR/BNB)
3. Deploy MasterChef with test parameters
4. Add LP pool via add()
5. User deposits testnet LP tokens
6. Wait 100 blocks (~5 minutes)
7. Harvest rewards
8. Verify ÉTR balance increased
9. Withdraw LP tokens
10. Emergency withdraw test
```

---

## 📐 Gas Cost Estimates (BSC)

| Function | Estimated Gas | Cost @ 3 gwei |
|----------|---------------|---------------|
| `deposit()` | ~120,000 | $0.04 |
| `withdraw()` | ~100,000 | $0.03 |
| `harvest()` | ~80,000 | $0.02 |
| `emergencyWithdraw()` | ~60,000 | $0.02 |
| `add()` (admin) | ~150,000 | $0.05 |
| `updateEmissionRate()` (admin) | ~50,000 | $0.02 |

**Total User Cost per Cycle** (deposit → wait → harvest → withdraw): ~$0.11

**Comparison**: Ethereum mainnet would be ~$50-100 per cycle (450x more expensive)

---

## 🔄 Maintenance Schedule

### Monthly Admin Actions

**End of Month 1 (Nov 30, 2025)**:
- [ ] Call `updateEmissionRate(4.05e18)` to increase to 4.05 ÉTR/block
- [ ] Announce Month 2 APR (120%) to community
- [ ] Review TVL and adjust if needed (e.g., increase APR if TVL < target)

**End of Month 2 (Dec 31, 2025)**:
- [ ] Call `updateEmissionRate(4.63e18)` to increase to 4.63 ÉTR/block
- [ ] Announce Month 3-4 APR (90-70%)

**End of Month 4 (Feb 28, 2026)**:
- [ ] Call `updateEmissionRate(4.05e18)` to reduce to 4.05 ÉTR/block
- [ ] Announce Month 5 APR (50%)

**End of Month 5 (Mar 31, 2026)**:
- [ ] Call `updateEmissionRate(2.89e18)` to reduce to 2.89 ÉTR/block
- [ ] Announce Month 6 APR (35%)
- [ ] Begin planning for Month 7+ sustainability (lower APR, fee-based)

**End of Month 6 (Apr 30, 2026)**:
- [ ] Call `updateEmissionRate(1.16e18)` for maintenance phase (10-20% APR)
- [ ] Review transition to fee-based sustainability
- [ ] Consider migrating to native Ëtrid chain staking if available

---

## 📊 Monitoring & Analytics

### Key Metrics to Track

**On-Chain Metrics** (via BscScan API):
- Total Value Locked (TVL) in USD
- Number of unique stakers
- Total LP tokens staked
- Total ÉTR rewards distributed
- Average stake duration
- Daily deposit/withdrawal volume

**Off-Chain Metrics** (via Dune Analytics or The Graph):
- APR calculation (real-time)
- Reward distribution by user (whale analysis)
- Gas costs for users
- Harvest frequency
- Impermanent loss vs. rewards earned

**Dashboard** (Build with Next.js or React):
- Live TVL and APR
- User's staked balance and pending rewards
- Countdown to next emission rate update
- Leaderboard (top LPs)

---

## 🚨 Emergency Response Plan

### Incident Response Procedures

**Scenario 1: Smart Contract Bug Discovered**
1. Immediately call `pause()` to stop deposits
2. Announce incident to community (Discord, Twitter)
3. Allow existing users to `emergencyWithdraw()` LP tokens
4. Engage security firm for fix
5. Deploy fixed contract
6. Migrate users to new contract

**Scenario 2: ÉTR Price Flash Crash**
1. Monitor APR calculation (may spike if TVL drops)
2. Consider emergency emission rate reduction
3. Announce stability measures to community
4. Do NOT panic-pause (causes more fear)

**Scenario 3: BSC Network Congestion**
1. Announce high gas fees to community
2. Recommend waiting for lower gas periods
3. Do NOT adjust emission rate (would be unfair to active users)

**Contact List** (Emergency Only):
- Lead Developer: eoj@etrid.io
- Security Auditor: (if hired)
- Community Manager: (Discord admin)
- Multi-sig Signers: (governance members)

---

## 📋 Solana Implementation (Separate Spec)

**Note**: Solana uses Anchor framework instead of Solidity. Separate specification needed for Raydium integration.

**Key Differences**:
- Use Anchor program instead of Solidity contract
- Rewards distributed via SPL Token instructions
- Lower gas costs (~$0.0001 per transaction)
- Different pool architecture (Raydium vs PancakeSwap)

**See**: `SOLANA_LP_REWARDS_SPEC.md` (to be created)

---

## ✅ Implementation Checklist

### Week 1: Development & Testing

- [ ] Fork PancakeSwap MasterChef V2 repository
- [ ] Customize for ÉTR rewards (update token addresses, emission rates)
- [ ] Write unit tests (Foundry or Hardhat)
- [ ] Run test coverage (target: >95%)
- [ ] Deploy to BSC testnet
- [ ] Run integration tests (manual deposit/withdraw/harvest)

### Week 2: Mainnet Deployment

- [ ] Deploy ÉTR token on BSC mainnet
- [ ] Create PancakeSwap LP pool (ÉTR/BNB)
- [ ] Deploy MasterChef on BSC mainnet
- [ ] Transfer 20M ÉTR to MasterChef
- [ ] Add ÉTR/BNB pool via `add()`
- [ ] Verify contract on BscScan
- [ ] Transfer ownership to multi-sig
- [ ] Announce to community

### Post-Launch: Monitoring

- [ ] Monitor first deposits and rewards distribution
- [ ] Track TVL growth vs. targets
- [ ] Schedule Month 2 emission update (Nov 30)
- [ ] Build analytics dashboard
- [ ] Gather user feedback and iterate

---

## 📚 References

- **PancakeSwap MasterChef V2**: https://github.com/pancakeswap/pancake-smart-contracts/blob/master/projects/farms-pools/contracts/MasterChefV2.sol
- **OpenZeppelin Contracts**: https://docs.openzeppelin.com/contracts/4.x/
- **BSC Network**: https://docs.bnbchain.org/docs/overview
- **Hardhat Documentation**: https://hardhat.org/docs
- **Foundry Documentation**: https://book.getfoundry.sh/

---

**Last Updated**: October 24, 2025
**Next Review**: November 1, 2025 (before Week 2 deployment)
**Maintainer**: Ëtrid Protocol Team
