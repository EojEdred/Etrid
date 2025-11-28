# MasterChef LP Rewards - Complete Guide

**Smart Contract for Community LP Rewards Program**

Last Updated: October 24, 2025

---

## 📋 Overview

MasterChef is the smart contract that distributes **20M ÉTR rewards** over 6 months to liquidity providers on PancakeSwap.

**Key Features**:
- ✅ Multi-pool support (ÉTR/BNB, future pools)
- ✅ Time-decaying emission rate (150% APR → 35% APR)
- ✅ Deposit/withdraw/harvest functions
- ✅ Emergency withdraw (no rewards)
- ✅ Pausable for emergencies

**Contract**: `contracts/MasterChef.sol` (394 lines)

---

## 🚀 Deployment

### Testnet Deployment

```bash
# Prerequisites:
# 1. ÉTR token deployed on testnet
# 2. ETR_TOKEN_ADDRESS_TESTNET in .env

# Deploy MasterChef
npm run deploy:masterchef:testnet

# Save the contract address to .env
echo "MASTERCHEF_ADDRESS_TESTNET=0x..." >> .env
```

**Cost**: ~$0 (testnet BNB from faucet)

---

### Mainnet Deployment

```bash
# Prerequisites:
# 1. ÉTR token deployed on mainnet
# 2. ETR_TOKEN_ADDRESS_MAINNET in .env
# 3. Have ~$10-15 BNB for gas
# 4. Have 20M ÉTR ready to transfer

# Deploy MasterChef
npm run deploy:masterchef:mainnet

# Follow prompts carefully!
```

**Cost**: ~$10-15 (real BNB)

---

## 📊 Initial Configuration

### 1. Transfer ÉTR Rewards to MasterChef

**CRITICAL**: You must fund MasterChef with 20M ÉTR!

**Via BscScan**:
1. Go to ÉTR token contract on BscScan
2. Connect wallet (Write Contract tab)
3. Call `transfer()`:
   - `to`: MasterChef address
   - `amount`: `20000000000000000000000000` (20M ÉTR with 18 decimals)
4. Confirm transaction

**Verify transfer**:
```bash
# Check MasterChef ÉTR balance
cast call <MASTERCHEF_ADDRESS> \
  "balanceOf(address)(uint256)" \
  <ETR_TOKEN_ADDRESS> \
  --rpc-url <RPC_URL>
```

---

### 2. Add LP Pool (ÉTR/BNB)

**Prerequisites**:
- ÉTR/BNB liquidity pool exists on PancakeSwap
- You have the LP token address

**Via BscScan**:
1. Go to MasterChef contract (Write Contract)
2. Call `add()`:
   - `_allocPoint`: `1000` (100% of rewards to this pool)
   - `_lpToken`: PancakeSwap ÉTR/BNB LP token address
   - `_withUpdate`: `false` (first pool)
3. Confirm transaction

**Pool ID**: 0 (first pool)

---

### 3. Transfer Ownership to Multi-Sig

**CRITICAL**: Do this immediately after setup!

**Via BscScan**:
1. Go to MasterChef contract (Write Contract)
2. Call `transferOwnership()`:
   - `newOwner`: Multi-sig wallet address
3. Confirm transaction
4. Multi-sig must accept ownership

---

## 👥 User Functions (LP Interactions)

### Deposit LP Tokens

**What**: Stake ÉTR/BNB LP tokens to earn rewards

**Via BscScan**:
1. First, approve MasterChef to spend your LP tokens:
   - Go to LP Token contract
   - Call `approve()`:
     - `spender`: MasterChef address
     - `amount`: `115792089237316195423570985008687907853269984665640564039457584007913129639935` (max uint256)
2. Then, go to MasterChef
3. Call `deposit()`:
   - `_pid`: `0` (ÉTR/BNB pool)
   - `_amount`: Amount of LP tokens (in wei)

**Example**: Deposit 1 LP token
- `_amount`: `1000000000000000000` (1e18)

---

### Harvest Rewards

**What**: Claim accumulated ÉTR rewards without withdrawing LP tokens

**Via BscScan**:
1. Go to MasterChef (Write Contract)
2. Call `harvest()`:
   - `_pid`: `0`

**ÉTR will be sent to your wallet**

---

### Withdraw LP Tokens

**What**: Unstake LP tokens (auto-harvests rewards first)

**Via BscScan**:
1. Go to MasterChef (Write Contract)
2. Call `withdraw()`:
   - `_pid`: `0`
   - `_amount`: Amount to withdraw (in wei)

**Example**: Withdraw all (if you deposited 1 LP token)
- `_amount`: `1000000000000000000`

---

### Check Pending Rewards

**What**: View unclaimed ÉTR rewards

**Via BscScan**:
1. Go to MasterChef (Read Contract)
2. Call `pendingReward()`:
   - `_pid`: `0`
   - `_user`: Your wallet address

**Returns**: Pending ÉTR (in wei)

**Convert to ÉTR**: Divide by 1e18

---

### Emergency Withdraw

**What**: Withdraw LP tokens WITHOUT claiming rewards (emergency only)

**Via BscScan**:
1. Go to MasterChef (Write Contract)
2. Call `emergencyWithdraw()`:
   - `_pid`: `0`

**⚠️  You forfeit all unclaimed rewards!**

---

## 🔧 Admin Functions (Governance)

### Update Emission Rate (Monthly)

**When**: End of each month to adjust APR

**Month Schedule**:
- Month 1: 2.89 ÉTR/block (150% APR) ← INITIAL
- Month 2: 4.05 ÉTR/block (120% APR)
- Month 3: 4.63 ÉTR/block (90% APR)
- Month 4: 4.63 ÉTR/block (70% APR)
- Month 5: 4.05 ÉTR/block (50% APR)
- Month 6: 2.89 ÉTR/block (35% APR)

**Via BscScan** (Multi-sig wallet):
1. Go to MasterChef (Write Contract)
2. Call `updateRewardPerBlock()`:
   - `_rewardPerBlock`: New rate in wei

**Example - Month 2**:
- `_rewardPerBlock`: `4050000000000000000` (4.05 ÉTR)

---

### Add New LP Pool

**When**: Adding ÉTR/SOL or other pairs

**Via BscScan** (Multi-sig wallet):
1. Go to MasterChef (Write Contract)
2. Call `add()`:
   - `_allocPoint`: Weight (e.g., 500 = 50% of rewards)
   - `_lpToken`: New LP token address
   - `_withUpdate`: `true` (update existing pools)

**Example**: Add ÉTR/SOL pool with 50% of rewards
- `_allocPoint`: `500`
- `_lpToken`: `0x...` (Raydium ÉTR/SOL LP)
- `_withUpdate`: `true`

**Note**: Total allocPoints across all pools = weight distribution

---

### Update Pool Allocation

**When**: Rebalancing rewards between pools

**Via BscScan** (Multi-sig wallet):
1. Go to MasterChef (Write Contract)
2. Call `set()`:
   - `_pid`: Pool ID to update
   - `_allocPoint`: New allocation points
   - `_withUpdate`: `true`

**Example**: Give ÉTR/BNB pool 70% of rewards
- Assuming 2 pools exist
- ÉTR/BNB: `_allocPoint = 700`
- ÉTR/SOL: `_allocPoint = 300`

---

### Pause/Unpause (Emergency)

**When**: Critical bug discovered

**Pause** (stops deposits, allows withdrawals):
```
Call pause()
```

**Unpause** (resume normal operation):
```
Call unpause()
```

---

## 📊 Monitoring & Analytics

### Key Metrics to Track

**On-Chain** (via BscScan Read Contract):

1. **Total Value Locked (TVL)**:
   ```
   poolInfo(0) → totalStaked
   Convert to ÉTR/BNB LP value
   ```

2. **Reward Rate**:
   ```
   rewardPerBlock()
   Returns: wei per block
   ```

3. **Pool Allocation**:
   ```
   poolInfo(0) → allocPoint
   totalAllocPoint()
   Percentage = allocPoint / totalAllocPoint
   ```

4. **User Balance**:
   ```
   userInfo(0, <USER_ADDRESS>)
   Returns: amount staked, rewardDebt
   ```

---

### Calculate APR

**Formula**:
```
Daily ÉTR = rewardPerBlock × 28,800 (blocks/day)
Annual ÉTR = Daily ÉTR × 365
APR = (Annual ÉTR × ÉTR Price) / TVL
```

**Example** (Month 1):
- rewardPerBlock = 2.89 ÉTR
- Daily = 2.89 × 28,800 = 83,232 ÉTR
- Annual = 83,232 × 365 = 30,379,680 ÉTR
- ÉTR Price = $0.01
- TVL = $50,000
- APR = ($303,796 / $50,000) = 607% (actual will be closer to 150% with price adjustments)

---

## 🐛 Troubleshooting

### Error: "ERC20: insufficient allowance"

**Solution**: Approve MasterChef to spend LP tokens first
```
Call approve() on LP token contract
spender: MasterChef address
amount: max uint256
```

---

### Error: "MasterChef: withdraw amount exceeds balance"

**Solution**: You're trying to withdraw more than you deposited
- Check your balance: `userInfo(0, <YOUR_ADDRESS>)`
- Withdraw exact amount or less

---

### Rewards seem lower than expected

**Check**:
1. `rewardPerBlock()` - Is emission rate correct?
2. `poolInfo(0).allocPoint` - Is allocation correct?
3. `totalAllocPoint()` - Are there other pools diluting rewards?
4. Time elapsed - Rewards accrue per block

---

### Can't harvest (transaction reverts)

**Possible causes**:
1. No pending rewards
2. MasterChef contract paused
3. Insufficient ÉTR balance in MasterChef (should not happen if funded properly)

**Check**:
```
pendingReward(0, <YOUR_ADDRESS>)
```

---

## 🔐 Security Best Practices

### For Users

✅ **DO**:
- Verify contract address before interacting
- Start with small test deposit
- Understand impermanent loss
- Monitor your position regularly

❌ **DON'T**:
- Approve unlimited tokens to unverified contracts
- Deposit more than you can afford to lose
- Panic sell during temporary price drops
- Use `emergencyWithdraw()` unless absolutely necessary

---

### For Admins (Multi-Sig Owners)

✅ **DO**:
- Test all operations on testnet first
- Update emission rates on schedule
- Monitor ÉTR balance in contract
- Keep emergency pause ready
- Document all parameter changes

❌ **DON'T**:
- Change parameters mid-month (breaks trust)
- Update emission rate without announcing
- Add untested LP token addresses
- Transfer ownership to non-multi-sig
- Drain reward tokens (locked for program duration)

---

## 📅 Monthly Maintenance Schedule

### End of Each Month

**Tasks**:
1. [ ] Announce next month's APR change
2. [ ] Call `updateRewardPerBlock()` with new rate
3. [ ] Update website/docs with new APR
4. [ ] Monitor for any issues post-update

**Timeline**:
- **Nov 30**: Month 1 → Month 2 (2.89 → 4.05 ÉTR/block)
- **Dec 31**: Month 2 → Month 3 (4.05 → 4.63 ÉTR/block)
- **Jan 31**: Month 3 → Month 4 (stays at 4.63)
- **Feb 28**: Month 4 → Month 5 (4.63 → 4.05 ÉTR/block)
- **Mar 31**: Month 5 → Month 6 (4.05 → 2.89 ÉTR/block)
- **Apr 30**: Month 6 → Maintenance (2.89 → 1.16 ÉTR/block)

---

## 📚 Additional Resources

- **Smart Contract**: `contracts/MasterChef.sol`
- **Deployment Script**: `scripts/deploy-masterchef-mainnet.ts`
- **Spec Document**: `/docs/LP_REWARDS_CONTRACT_SPEC.md`
- **BscScan**: Interact with contract directly
- **PancakeSwap**: https://pancakeswap.finance/farms

---

## 🆘 Emergency Contacts

**Critical Issues Only**:
- Lead Developer: gizzi_io@proton.me
- Multi-sig Signers: (governance members)
- Community Manager: Discord admin

**Community Support**:
- Discord: #lp-support channel
- Telegram: @EtridSupport

---

**Last Updated**: October 24, 2025
**Contract Version**: 1.0
**Status**: Ready for Deployment
