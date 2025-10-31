# Staking Guide

Earn passive income by staking your ÉTR tokens to secure the ËTRID network.

## What is Staking?

**Staking** means locking your ÉTR tokens to support network validators and earn rewards.

### Benefits

- 💰 **Passive Income:** 8-15% annual rewards (APY varies)
- 🔒 **Network Security:** Help validate transactions
- 🗳️ **Voting Power:** Staked tokens count for governance
- 🌐 **Decentralization:** Support validators you trust

### How Staking Works

```
1. Nominate Validators
   ↓ (choose up to 16 validators)

2. Stake Allocated
   ↓ (algorithm distributes to elected validators)

3. Validators Produce Blocks
   ↓ (earn block rewards + transaction fees)

4. Rewards Distributed
   ↓ (you receive portion based on stake)

5. Auto-Compound
   (rewards added to staked balance)
```

---

## Staking Requirements

| Requirement | Details |
|-------------|---------|
| **Minimum Stake** | 1 ÉTR (for nominators) |
| **Maximum Nominations** | 16 validators |
| **Unbonding Period** | 28 days |
| **Reward Frequency** | Every era (24 hours) |
| **Current APY** | 8-15% (varies) |

---

## How to Stake (Nominating)

### Step 1: Navigate to Staking

1. Open [wallet.etrid.org](https://wallet.etrid.org)
2. Click **"Staking"** in navigation menu
3. Review staking dashboard

```
┌─────────────────────────────────────────────┐
│ Staking Dashboard                            │
├─────────────────────────────────────────────┤
│ Available Balance:    90.0000 ÉTR          │
│ Currently Staked:      0.0000 ÉTR          │
│ Pending Rewards:       0.0000 ÉTR          │
│ Estimated APY:        12.5%                 │
│                                              │
│ [ Start Nominating ]                        │
└─────────────────────────────────────────────┘
```

### Step 2: Choose Validators

1. Click **"Start Nominating"**
2. Browse validator list
3. Select up to 16 validators

**Validator Information:**
```
┌────────────────────────────────────────────┐
│ Validator: ËTRID Foundation               │
├────────────────────────────────────────────┤
│ Commission:     0%                         │
│ Total Stake:    2,000,000 ÉTR             │
│ Nominators:     892                        │
│ Uptime:         100%                       │
│ Identity:       ✓ Verified                │
│ Location:       Switzerland                │
└────────────────────────────────────────────┘
```

**Choosing Validators - Look For:**

✅ **Low Commission** (0-5%)
- You earn more rewards
- Foundation nodes often 0%

✅ **High Uptime** (99%+)
- Offline validators = no rewards
- Check historical performance

✅ **Verified Identity**
- Known operators
- Accountable and transparent

✅ **Reasonable Stake**
- Not too centralized
- Spread across operators

✅ **Active Community**
- Regular updates
- Responsive to issues

**Avoid:**

❌ 100% commission (you earn nothing)
❌ Low uptime (<95%)
❌ Unknown/unverified validators
❌ Excessive stake concentration

### Step 3: Set Stake Amount

1. Click **"Next"** after selecting validators
2. Enter amount to stake
3. Leave some ÉTR for transaction fees

```
┌─────────────────────────────────────────────┐
│ Set Stake Amount                             │
├─────────────────────────────────────────────┤
│ Available Balance: 90.0000 ÉTR              │
│                                              │
│ Amount to Stake: [50.0000] ÉTR             │
│ Keep Available:   40.0000 ÉTR              │
│                                              │
│ Estimated Annual Rewards:                    │
│ 6.25 ÉTR (~12.5% APY)                       │
│                                              │
│ ⓘ Unbonding period: 28 days                │
│                                              │
│ [ Back ]                [ Review ]          │
└─────────────────────────────────────────────┘
```

**Recommended Allocation:**
- Stake: 50-80% of holdings
- Keep Available: 20-50% (fees, flexibility)
- Example: 90 ÉTR → Stake 50, Keep 40

### Step 4: Confirm Nomination

1. Click **"Review"**
2. Verify all details:
   - Stake amount
   - Selected validators
   - Estimated rewards
3. Click **"Sign and Confirm"**
4. Enter wallet password
5. Wait for confirmation

**Success!** You're now staking and earning rewards.

---

## Managing Your Stake

### View Staking Status

```
┌─────────────────────────────────────────────┐
│ Your Active Staking                          │
├─────────────────────────────────────────────┤
│ Total Staked: 50.0000 ÉTR                   │
│ Active Since: October 29, 2025               │
│                                              │
│ Nominated Validators: 3                      │
│ ✓ ËTRID Foundation (active)                │
│ ✓ Validator Alpha (active)                  │
│ ✓ Staking Rewards Inc. (active)            │
│                                              │
│ Rewards Earned Today: 0.0143 ÉTR           │
│ Total Rewards: 1.2456 ÉTR                   │
│ Current APY: 12.5%                           │
│                                              │
│ [Stake More] [Change Validators] [Unstake]  │
└─────────────────────────────────────────────┘
```

### Common Actions

#### Claim Rewards
- Rewards **auto-compound** by default
- Or click "Claim Rewards" to move to available balance
- No penalty for claiming

#### Stake More
1. Click "Stake More"
2. Add additional ÉTR
3. Takes effect next era (24 hours)

#### Change Validators
1. Click "Change Validators"
2. Select new validators
3. Keeps same stake amount
4. Useful if validator performance degrades

#### Unstake (Unbond)
1. Click "Unstake"
2. Enter amount to unbond
3. Wait **28 days** for unbonding period
4. After 28 days, click "Withdraw Unbonded"
5. Funds return to available balance

---

## Understanding Rewards

### Reward Calculation

```
Your Rewards = (Your Stake / Total Stake)
               × (Block Rewards + Fees)
               × (1 - Validator Commission)
```

### Factors Affecting Rewards

1. **Your Stake Amount**
   - More stake = more rewards
   - Proportional to total network stake

2. **Validator Performance**
   - Uptime matters (offline = no rewards)
   - Era points earned by validator

3. **Validator Commission**
   - Lower commission = more for you
   - Typical: 0-5%

4. **Network Participation**
   - Total staked affects reward rate
   - Higher participation = lower individual APY

5. **Era Points**
   - Validators with more era points earn more
   - You benefit proportionally

### Reward Distribution

- **Frequency:** Every era (24 hours)
- **Method:** Auto-compounding (added to staked balance)
- **Visibility:** Check "Pending Rewards" before distribution

### Example Calculation

```
Your stake: 50 ÉTR
Total network stake: 10,000,000 ÉTR
Your share: 0.0005%

Era rewards pool: 1,000 ÉTR
Your validator's share: 50 ÉTR (5%)
Validator commission: 5%

Your rewards = 50 × 0.0005% × 95%
             = 0.02375 ÉTR per era

Annual rewards ≈ 0.02375 × 365 = 8.66875 ÉTR
APY = 8.66875 / 50 = 17.3%
```

---

## Staking Strategies

### Conservative (Low Risk)
**Profile:** Risk-averse, long-term holders

**Strategy:**
- Stake with ËTRID Foundation validators (0% commission)
- Select 10-16 validators for diversification
- Stake 50% of holdings, keep 50% liquid
- Compound rewards automatically

**Expected APY:** 8-12%

---

### Balanced (Medium Risk)
**Profile:** Active community member

**Strategy:**
- Mix of foundation and community validators
- Select 8-12 validators with good track records
- Stake 70% of holdings, keep 30% liquid
- Claim rewards periodically for other uses

**Expected APY:** 10-14%

---

### Aggressive (Higher Risk)
**Profile:** Experienced user, high conviction

**Strategy:**
- Support emerging validators (potentially higher rewards)
- Select 16 validators for maximum diversification
- Stake 80-90% of holdings
- Actively monitor and rebalance

**Expected APY:** 12-15%+

---

## Advanced: Running a Validator

Want to run your own validator and earn more rewards?

### Requirements

| Type | Stake Required | Hardware |
|------|----------------|----------|
| **Validity Node** | 64 ÉTR minimum | Moderate server |
| **Full Validator** | 20,000 ÉTR minimum | High-end server |

### Benefits
- Earn **commission** from nominators (typically 5-10%)
- Full control over operations
- Contribute directly to network security
- Governance influence

### Setup
See [Operator Guide](OPERATOR_GUIDE.md) for detailed instructions.

---

## Staking Risks

### Slashing Risk
**What:** Validators penalized for misbehavior
**Your Risk:** Lose small portion of stake if validator is slashed
**Mitigation:** Choose reputable validators, diversify nominations

### Opportunity Cost
**What:** Staked ÉTR is locked (28-day unbonding)
**Your Risk:** Can't sell quickly if price drops
**Mitigation:** Don't stake more than you can afford to lock

### Validator Downtime
**What:** Offline validators don't earn rewards
**Your Risk:** Lower actual APY than expected
**Mitigation:** Monitor validator performance, change if needed

---

## Tax Considerations

**Important:** Staking rewards may be taxable in your jurisdiction.

- Keep records of rewards earned
- Export transaction history for tax software
- Consult tax professional for guidance
- Some jurisdictions tax on receipt, others on sale

---

## FAQ

**Q: How much should I stake?**
A: Leave at least 1-5 ÉTR for transaction fees. Stake the rest if you don't need liquidity.

**Q: Can I unstake anytime?**
A: You can initiate unstaking anytime, but funds are locked for 28 days.

**Q: Do rewards compound automatically?**
A: Yes! Rewards are added to your staked balance by default.

**Q: What if my validator goes offline?**
A: You won't earn rewards while offline. If prolonged, consider changing validators.

**Q: How do I know if my validator is good?**
A: Check uptime (99%+), commission (0-5%), verified identity, and community reputation.

**Q: Can I stake from a hardware wallet?**
A: Yes, when Ledger/Trezor support is available. Check [wallet.etrid.org](https://wallet.etrid.org) for updates.

---

## Need Help?

**Resources:**
- 📖 [Full User Guide](USER_GUIDE.md)
- 💬 [Discord Community](https://discord.gg/etrid)
- 🎥 [Staking Tutorial Video](https://youtube.com/etrid/staking)

**Support:**
- Email: staking@etrid.org
- Include: wallet address, validator list

---

**Ready to stake?** [Open Wallet →](https://wallet.etrid.org)
