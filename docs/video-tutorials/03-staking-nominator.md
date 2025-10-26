# Video Tutorial 03: Staking as a Nominator on Etrid

**Duration:** 7 minutes
**Target Audience:** ETR token holders, passive income seekers, non-technical users
**Prerequisites:** Tutorial 01 (wallet setup), ETR tokens

---

## Script Overview

This tutorial teaches users how to earn passive rewards by nominating validators through staking, covering validator selection, risk management, and reward optimization.

---

## Time Markers & Script

### 00:00 - 00:45 | Introduction & What is Staking?

**NARRATION:**
"Welcome back to the Etrid tutorial series! Today we're talking about staking - one of the easiest ways to earn passive income with your ETR tokens.

Here's how it works: Etrid uses Ascending Scale of Finality, a consensus mechanism that requires validators to lock up ETR tokens. But what if you don't want to run a validator yourself? That's where nomination comes in. You can 'nominate' validators by staking your ETR with them. Your stake increases their voting power, and in return, they share their block rewards with you.

Think of it like earning interest on a savings account, except you're helping secure the blockchain network. Current APY ranges from 8% to 15%, depending on network conditions and which validators you choose.

The best part? It's completely passive once set up. You don't need technical skills or expensive servers. Just ETR tokens and a wallet. Let's get started!"

**VISUAL CUES:**
- Animated diagram: Nominator → Validator → Network Security → Rewards → Nominator
- APY comparison chart:
  ```
  Traditional Savings: 0.5%
  Treasury Bonds:      4%
  Etrid Staking:       8-15%
  ```
- Staking infographic showing:
  - Nominator provides: ETR tokens
  - Validator provides: Infrastructure, expertise
  - Network provides: Block rewards, transaction fees
  - Nominator receives: Percentage of rewards (minus commission)
- Risk/reward spectrum visualization

**KEY POINTS TO EMPHASIZE:**
- Passive income opportunity (8-15% APY)
- No technical knowledge required
- Helps secure the network
- Rewards shared by validators
- Minimal active management needed

**COMMON MISTAKES TO MENTION:**
- "Staking is not risk-free - validators can be slashed for misbehavior"
- "Don't confuse staking (nomination) with liquidity mining or yield farming"

---

### 00:45 - 02:00 | Why Stake and Expected Returns

**NARRATION:**
"Let's talk numbers. Why should you stake your ETR tokens?

First, the returns. Based on current network parameters, nominators earn between 8% and 15% annually. That means if you stake 1,000 ETR, you could earn 80 to 150 ETR per year - that's about 7 to 12 ETR per month. Not bad for doing essentially nothing!

Where do these rewards come from? Two sources: block rewards and transaction fees. Every time your nominated validator produces a block, they earn newly minted ETR plus fees from transactions in that block. They take their commission - typically 5% to 15% - and distribute the rest to nominators proportionally.

But there's more than just money. By staking, you're actively participating in network governance. Your stake gives you voting power for Consensus Day - our annual on-chain governance event where you vote on inflation rates, network upgrades, and treasury spending.

The risks? Two main ones. First, opportunity cost - your tokens are locked for 28 days if you want to unstake. Second, slashing - if your validator misbehaves or has extended downtime, they can lose up to 10% of their stake, and you lose the same percentage. That's why choosing good validators is crucial!"

**VISUAL CUES:**
- Rewards calculator showing:
  ```
  Stake Amount:     1,000 ETR
  Annual APY:       12%
  Monthly Reward:   ~10 ETR
  Annual Reward:    ~120 ETR
  ```
- Pie chart: Reward distribution
  - Validator commission: 10%
  - Nominators: 90%
- Timeline: Reward payment schedule
  - Earned: Every block (5 seconds)
  - Distributed: Every era (24 hours)
  - Claimable: Anytime after distribution
- Risk visualization:
  ```
  Slashing Example:
  Your stake:      1,000 ETR
  Slash penalty:   10%
  Your loss:       -100 ETR
  Remaining:       900 ETR
  ```
- Governance power meter showing stake weight

**DEMO STEPS:**
1. Show APY calculator on wallet dashboard
2. Input various stake amounts
3. Display projected monthly/annual returns
4. Show historical APY chart (last 90 days)
5. Display current validator commission rates
6. Show example slashing event (historical data)

**KEY POINTS TO EMPHASIZE:**
- 8-15% APY is current rate (subject to change)
- Rewards compound if not withdrawn
- Commission varies by validator (5-20%)
- 28-day unbonding period for liquidity
- Slashing is rare but possible
- Stake = governance voting power

**COMMON MISTAKES TO MENTION:**
- "APY is not guaranteed - it fluctuates with network activity"
- "Don't stake your entire balance - keep some liquid for fees"
- "Lower commission doesn't always mean better - consider validator reliability"

---

### 02:00 - 04:00 | Choosing Validators: Research and Criteria

**NARRATION:**
"This is the most important decision in staking: which validators to nominate. You can nominate up to 16 validators simultaneously, and Etrid automatically distributes your stake to maximize rewards. But you need to choose wisely.

Let's look at the key criteria. First, commission rate. This is what the validator keeps before distributing rewards. Lower is better for you - look for 5% to 10%. Anything above 15% is high unless they offer special value.

Second, total stake and nominator count. A validator with too much stake might already be at capacity - your nomination won't add value. Too little stake and they might not make it into the active set. Sweet spot: middle of the pack.

Third, uptime and performance. Check their history. Are they consistently producing blocks? Have they ever been slashed? The wallet shows performance metrics - look for 99%+ uptime.

Fourth, identity and reputation. Validators can set on-chain identities with website, email, and social links. Check these! Are they active in the community? Do they run other blockchain infrastructure? Experience matters.

Let me show you how to research. Open the Staking tab and click 'Validator Browser'. You'll see all validators with sortable columns. Let's filter by commission under 10%. Now sort by total stake - we want middle-tier validators. Click on one to see their details.

Here's a great example: 'Staking4All Validator'. 8% commission, 500,000 ETR total stake, 99.8% uptime, verified identity with website, no slashing history. This is what you're looking for. Let's add them to our nomination list. I'll select a few more using the same criteria - diversification is key."

**VISUAL CUES:**
- Validator browser interface showing sortable table:
  ```
  Validator Name       | Commission | Total Stake | Nominators | Uptime | Slashes
  ─────────────────────|─────────────────────────────────────────────────────
  Staking4All          | 8%         | 500K ETR    | 42         | 99.8%  | 0
  BlockchainPros       | 10%        | 750K ETR    | 67         | 99.5%  | 0
  ValidatorOne         | 5%         | 2M ETR      | 156        | 99.9%  | 0
  ```
- Individual validator profile showing:
  - Identity (verified checkmark)
  - Website and social links
  - Performance chart (90-day uptime)
  - Slashing history (empty)
  - Commission history
  - Total stake breakdown (own vs. nominated)
- Filter controls demonstration
- Selection checklist overlay:
  ```
  Good Validator Checklist:
  ✅ Commission: 5-10%
  ✅ Stake: 100K-1M ETR
  ✅ Uptime: >99%
  ✅ Verified identity
  ✅ Active community presence
  ✅ No slashing history
  ```
- Multiple validator cards being selected (up to 16)

**DEMO STEPS:**
1. Navigate to Staking → Validator Browser
2. Show full validator list (21 active + candidates)
3. Apply filter: Commission < 10%
4. Sort by Total Stake (ascending)
5. Click first validator card
6. Show detailed profile view:
   - Commission: 8%
   - Identity: Verified (checkmark icon)
   - Website link (click to show)
   - Uptime chart
   - No slashing events
7. Click "Add to Nominations"
8. Repeat for 4-5 more validators
9. Show nomination list with selected validators
10. Demonstrate removing a validator from list

**CODE TO DISPLAY:**
```javascript
// Validator evaluation pseudocode
function evaluateValidator(validator) {
  let score = 100;

  // Commission scoring (lower is better)
  if (validator.commission > 15) score -= 30;
  else if (validator.commission > 10) score -= 15;
  else if (validator.commission < 8) score += 10;

  // Uptime scoring
  if (validator.uptime < 95) score -= 40;
  else if (validator.uptime < 99) score -= 20;
  else if (validator.uptime > 99.5) score += 15;

  // Slashing history (critical)
  if (validator.slashCount > 0) score -= 50;

  // Identity verification
  if (validator.hasVerifiedIdentity) score += 10;

  // Stake sweet spot (not too high, not too low)
  if (validator.totalStake > 100000 && validator.totalStake < 1000000) {
    score += 20;
  }

  return score;
}

// Example results:
// Staking4All:     Score 95 ✅ Excellent
// BlockchainPros:  Score 85 ✅ Good
// NewValidator123: Score 45 ❌ Poor choice
```

**KEY POINTS TO EMPHASIZE:**
- Nominate 8-16 validators for diversification
- Commission is important but not everything
- Verify validator identity off-chain
- Check performance history, not just current status
- Avoid over-subscribed validators (too many nominators)
- Slashing history is a major red flag
- Active community participation indicates commitment

**COMMON MISTAKES TO MENTION:**
- "Don't choose only by lowest commission - reliability matters more"
- "Don't nominate only one validator - diversification reduces risk"
- "Don't ignore uptime - even 95% means 18 days offline per year"
- "Don't skip identity verification - anonymous validators are risky"
- "Be careful with brand new validators - no performance history"

---

### 04:00 - 05:15 | Nominating Validators Through the UI

**NARRATION:**
"You've researched and selected your validators. Now let's nominate them and start earning rewards!

Make sure you're connected to your wallet with ETR tokens ready to stake. Navigate to the Staking section and click 'Nominate'. You'll see the nomination form.

First, enter your stake amount. Minimum is 100 ETR, but I recommend at least 500 ETR to make the rewards meaningful after fees. Let's stake 1,000 ETR.

Now you'll see your pre-selected validators from earlier. You can add or remove validators here. Remember, you can nominate up to 16. I've chosen 8 solid validators with good track records.

Next, the bond duration. There are two options: 'Staked' means your tokens are locked until you manually unbond them. 'Stash' means you can add more stake later without re-nominating. Choose 'Staked' for simplicity.

Review the transaction details. You're bonding 1,000 ETR to 8 validators. Your expected APY is shown here - about 12% based on current network conditions. That's roughly 10 ETR per month.

Important: Once you click 'Bond and Nominate', your tokens will be locked. You can unbond anytime, but there's a 28-day waiting period before you can access them again. Make sure you're comfortable with this lockup period.

Ready? Click 'Bond and Nominate'. Enter your wallet password to sign the transaction. And... submitted! Wait for the transaction to finalize - about 15 seconds. Done! You're now officially a nominator. Your stake is active and you'll start earning rewards in the next era - within 24 hours."

**VISUAL CUES:**
- Nomination form screen recording with labeled sections:
  ```
  ┌─────────────────────────────────────┐
  │ Nominate Validators                 │
  ├─────────────────────────────────────┤
  │ Stake Amount:  [1000] ETR           │
  │ Minimum: 100 ETR                    │
  │                                     │
  │ Selected Validators (8/16):         │
  │ ☑ Staking4All       (8% comm.)     │
  │ ☑ BlockchainPros    (10% comm.)    │
  │ ☑ SecureNode        (7% comm.)     │
  │ ... 5 more ...                      │
  │                                     │
  │ Bond Type:  ◉ Staked  ◯ Stash      │
  │                                     │
  │ Expected APY: 12.3%                 │
  │ Monthly Reward: ~10.25 ETR          │
  │                                     │
  │ Unbonding Period: 28 days           │
  │                                     │
  │ [Bond and Nominate]                 │
  └─────────────────────────────────────┘
  ```
- Transaction signing modal
- Transaction confirmation with hash
- Success notification: "Nomination active!"
- Staking dashboard updating to show:
  - Active stake: 1,000 ETR
  - Nominated validators: 8
  - Status: Waiting for next era
  - Next reward: ~24 hours

**DEMO STEPS:**
1. Navigate to Staking → Nominate
2. Enter stake amount: 1000 ETR
3. Verify minimum met (100 ETR)
4. Review selected validators (8 shown)
5. Select bond type: Staked
6. Review APY estimate: 12.3%
7. Review monthly reward: ~10.25 ETR
8. Note unbonding period warning
9. Click "Bond and Nominate"
10. Enter wallet password
11. Click "Sign and Submit"
12. Wait for transaction finality (show spinner)
13. Show success notification
14. Navigate to Staking Dashboard
15. Show active nomination status
16. Show "Next Era" countdown timer

**CODE TO DISPLAY:**
```
Transaction Details:
═══════════════════════════════════════
Action:    Bond and Nominate
Amount:    1,000.0000 ETR
Validators: 8 selected
Fee:       0.0015 ETR

Nominated Validators:
1. Staking4All       (8% commission)
2. BlockchainPros    (10% commission)
3. SecureNode        (7% commission)
4. EtridValidator    (9% commission)
5. StakePool01       (8% commission)
6. BlockProducerXYZ  (10% commission)
7. ValidatorPro      (7% commission)
8. TrustedNode       (9% commission)

Expected Returns:
─────────────────
APY:           12.3%
Annual:        ~123 ETR
Monthly:       ~10.25 ETR
Daily:         ~0.34 ETR

Unbonding Period: 28 days
First Reward:     ~24 hours
Status:           Pending (next era)

Transaction Hash:
0x9f3e8a7b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f
```

**KEY POINTS TO EMPHASIZE:**
- 100 ETR minimum to nominate
- Up to 16 validators can be nominated
- 28-day unbonding period is mandatory
- Rewards start in next era (within 24 hours)
- APY estimate is not guaranteed
- Tokens are locked but still yours (not sent to validator)
- Can change nominations anytime (takes effect next era)

**COMMON MISTAKES TO MENTION:**
- "Don't stake your entire balance - keep ETR for transaction fees"
- "Don't panic if rewards don't appear immediately - first payout in 24h"
- "Don't unbond impulsively - 28-day waiting period applies"
- "Remember to claim rewards - they don't auto-compound"

---

### 05:15 - 06:00 | Understanding Rewards and Claiming

**NARRATION:**
"Your nomination is active. Now let's talk about rewards - how they work and how to claim them.

Rewards are calculated every era - that's every 24 hours on Etrid. At the end of each era, the protocol looks at which validators produced blocks, how many nominators they have, and divides the rewards proportionally.

Let's say your validator earned 100 ETR in block rewards and fees. They take their 10% commission - that's 10 ETR for them. The remaining 90 ETR is split among all nominators based on their stake percentage. If you have 1% of the total nominator stake, you get 0.9 ETR.

You can see pending rewards in your staking dashboard. They accumulate daily. Here's the cool part: you can either claim them or leave them to auto-compound. Some validators offer auto-compounding where unclaimed rewards increase your stake automatically. Check if your validators support this.

To manually claim, click 'Claim Rewards' in the dashboard. You'll see a list of eras with pending rewards. You can claim all at once or individually. Each claim is a transaction, so it costs a tiny fee - about 0.001 ETR.

I recommend claiming weekly or monthly to minimize fees. Unless you need the liquidity, letting rewards accumulate and compound can significantly boost your returns over time. A 12% APY becomes about 12.7% with monthly compounding!"

**VISUAL CUES:**
- Rewards dashboard showing:
  ```
  ┌─────────────────────────────────────┐
  │ Your Staking Rewards                │
  ├─────────────────────────────────────┤
  │ Total Staked:      1,000 ETR        │
  │ Pending Rewards:   25.47 ETR        │
  │ Claimed (30d):     10.25 ETR        │
  │                                     │
  │ Current APY:       12.3%            │
  │ Effective APY:     12.7% (compound) │
  │                                     │
  │ Last Era Reward:   0.34 ETR         │
  │ Next Era:          6h 23m           │
  │                                     │
  │ [Claim All Rewards]  [Auto-Compound]│
  └─────────────────────────────────────┘
  ```
- Reward breakdown chart (pie chart):
  - Block rewards: 70%
  - Transaction fees: 30%
- Era timeline showing reward calculation points
- Claim history table:
  ```
  Era    | Reward  | Validator       | Status
  ─────────────────────────────────────────────
  #1234  | 0.34 ETR| Staking4All     | Claimed
  #1233  | 0.35 ETR| BlockchainPros  | Claimed
  #1232  | 0.33 ETR| SecureNode      | Pending
  ```
- Compounding effect graph showing:
  - No compounding: 12% → 120 ETR/year
  - Monthly compound: 12.7% → 127 ETR/year
  - Difference: 7 ETR extra

**DEMO STEPS:**
1. Navigate to Staking → Rewards
2. Show pending rewards: 25.47 ETR
3. Show last 30-day claimed: 10.25 ETR
4. Click "Reward Details"
5. Show per-era breakdown (list view)
6. Show per-validator breakdown
7. Click "Claim All Rewards"
8. Review transaction (fee: 0.001 ETR)
9. Sign and submit
10. Show success: "25.47 ETR claimed"
11. Show updated balance
12. Show auto-compound option (if available)

**KEY POINTS TO EMPHASIZE:**
- Rewards calculated every era (24 hours)
- Proportional to your stake percentage
- Commission deducted before distribution
- Can claim anytime but costs transaction fee
- Auto-compounding increases effective APY
- Unclaimed rewards are safe (not lost)
- Claim frequency affects returns (fees vs. compound)

**COMMON MISTAKES TO MENTION:**
- "Don't claim too frequently - transaction fees eat into rewards"
- "Don't forget to claim before unbonding - unclaimed rewards stay locked"
- "Check if auto-compound is enabled - manual claiming disables it"
- "Rewards shown are estimates until era finalizes"

---

### 06:00 - 06:45 | Unstaking and Withdrawal Period

**NARRATION:**
"What if you need your ETR back? Let's talk about unstaking.

You can unbond your stake anytime, but there's a mandatory 28-day waiting period before you can withdraw. This is called the unbonding period, and it's a security feature to prevent sudden mass unstaking that could destabilize the network.

To unbond, go to your staking dashboard and click 'Unbond'. Enter the amount you want to unbond - you can unbond partially or fully. Let's unbond 200 ETR. Click 'Unbond Funds'.

The transaction will finalize, and you'll see your unbonding schedule. It shows: 200 ETR unlocking in 28 days. During this period, your tokens earn NO rewards - they're frozen. After 28 days, you can click 'Withdraw Unbonded' to get your tokens back into your transferable balance.

Important notes: You can have multiple unbonding chunks active at once. Each unbonding action starts its own 28-day timer. Also, if you change your mind, you can't cancel unbonding - once initiated, you have to wait it out.

Pro tip: If you need liquidity but don't want to unbond, consider liquid staking derivatives. These are tokens representing your staked position that you can trade while still earning rewards. Check out LETR - Liquid ETR - on Etrid's DeFi platforms."

**VISUAL CUES:**
- Unbonding interface:
  ```
  ┌─────────────────────────────────────┐
  │ Unbond Stake                        │
  ├─────────────────────────────────────┤
  │ Current Stake:     1,000 ETR        │
  │ Unbond Amount:     [200] ETR        │
  │ Remaining Stake:   800 ETR          │
  │                                     │
  │ ⚠️ Unbonding Period: 28 days       │
  │ ⚠️ No rewards during unbonding     │
  │                                     │
  │ [Unbond Funds]                      │
  └─────────────────────────────────────┘
  ```
- Unbonding schedule showing:
  ```
  Active Unbonding:

  Chunk 1:  200 ETR
  Started:  Oct 22, 2025
  Unlocks:  Nov 19, 2025 (23 days remaining)

  Chunk 2:  500 ETR
  Started:  Oct 15, 2025
  Unlocks:  Nov 12, 2025 (16 days remaining)
  ```
- Timeline visualization:
  ```
  Day 0          Day 28         Day 56
  │──────────────│──────────────│
  Unbond       Withdraw     Fully Liquid
  Initiated    Available

  Rewards:  ✅ Earning  ❌ No Rewards  ✅ Can Stake Again
  ```
- Liquid staking explanation graphic showing LETR token

**DEMO STEPS:**
1. Navigate to Staking → Manage
2. Show current stake: 1,000 ETR
3. Click "Unbond"
4. Enter amount: 200 ETR
5. Show remaining stake: 800 ETR
6. Review warning: 28-day period, no rewards
7. Click "Unbond Funds"
8. Sign transaction
9. Show unbonding schedule
10. Fast-forward demo: Show "Withdraw Unbonded" button (day 28)
11. Click "Withdraw Unbonded"
12. Show funds returned to transferable balance

**CODE TO DISPLAY:**
```
Unbonding Details:
═══════════════════════════════════════
Action:         Unbond Stake
Amount:         200.0000 ETR
Remaining:      800.0000 ETR

Timeline:
─────────────────────────────────────
Today (Day 0):     Unbonding starts
                   Rewards STOP for unbonded amount

Day 28:            Funds unlocked
                   Can withdraw to transferable balance

After Withdrawal:  Funds fully liquid
                   Can transfer, trade, or re-stake

Active Unbonding Chunks:
1. 200 ETR - Unlocks Nov 19, 2025 (23d)
2. 500 ETR - Unlocks Nov 12, 2025 (16d)

Total Unbonding: 700 ETR
Currently Staked: 300 ETR
```

**KEY POINTS TO EMPHASIZE:**
- 28-day unbonding period is mandatory
- No rewards earned during unbonding
- Can unbond partially or fully
- Multiple unbonding chunks allowed
- Cannot cancel unbonding once initiated
- Must manually withdraw after 28 days (not automatic)
- Liquid staking derivatives offer instant liquidity alternative

**COMMON MISTAKES TO MENTION:**
- "Don't unbond unless necessary - 28 days is a long time"
- "Don't forget to withdraw after 28 days - funds don't auto-return"
- "Remember: unbonding = no rewards for that amount"
- "Plan liquidity needs in advance - don't wait until last minute"

---

### 06:45 - 07:00 | Tips for Successful Staking + Outro

**NARRATION:**
"Let's wrap up with pro tips for maximizing your staking rewards.

Tip one: Diversify across 8-16 validators. Don't put all your eggs in one basket. If one validator gets slashed, you lose only a fraction.

Tip two: Monitor validator performance monthly. Validators can change commission or experience downtime. Re-evaluate your nominations every few months.

Tip three: Compound your rewards! Claim and re-stake monthly to maximize returns. A 12% APY becomes 12.7% with compounding.

Tip four: Stay informed. Join the Etrid Discord staking channel. Validators announce commission changes, and the community warns about problematic validators.

Tip five: Don't chase the highest APY blindly. A 15% APY with a risky validator is worse than 11% with a reliable one if you get slashed.

And remember: staking is a long-term game. Don't panic during short-term volatility. Keep your stake active, monitor occasionally, and let the rewards compound.

Congratulations - you're now earning passive income on Etrid! Check out tutorial four to learn about deploying smart contracts, or tutorial five for building DApps. Welcome to the staking community, and happy earning!"

**VISUAL CUES:**
- Pro tips checklist overlay:
  ```
  Staking Pro Tips:
  ✅ Diversify 8-16 validators
  ✅ Monitor performance monthly
  ✅ Compound rewards regularly
  ✅ Stay informed (Discord)
  ✅ Reliability > High APY
  ✅ Think long-term
  ```
- Final stats summary:
  ```
  Your Staking Summary:
  Stake:     1,000 ETR
  Validators: 8 nominated
  APY:       12.3%
  Monthly:   ~10 ETR
  Annual:    ~123 ETR (if compounded)

  Status: Active ✅
  ```
- Resource links:
  - 📚 docs.etrid.io/staking
  - 💬 discord.gg/etrid #staking
  - 📊 Staking calculator
  - 🔍 Validator explorer
- Next tutorial thumbnails
- Subscribe/community CTA

**KEY POINTS TO EMPHASIZE:**
- Long-term strategy beats short-term optimization
- Community support valuable for staying informed
- Regular monitoring maintains optimal returns
- Compounding significantly boosts returns

---

## Production Notes

### Visual Assets Needed

**Static Graphics:**
1. Staking flow diagram (nominator → validator → network → rewards)
2. APY comparison chart
3. Validator evaluation criteria checklist
4. Rewards calculation breakdown
5. Unbonding timeline visualization
6. Compounding effect graph
7. Pro tips checklist
8. Resource links end card

**Screen Recordings:**
1. Validator browser navigation
2. Validator profile detailed view
3. Nomination form completion
4. Rewards dashboard
5. Claim rewards flow
6. Unbonding process
7. Withdrawal demo

**Animations:**
1. Stake distribution among validators
2. Reward flow from block to nominator
3. Era timeline with reward points
4. Compounding snowball effect

### Demo Requirements

**Environment:**
- Etrid wallet with 1,000+ ETR
- Testnet with active validators
- Multiple validator profiles set up
- Pre-configured nomination for demonstration
- Rewards accumulated for claiming demo

**Preparation:**
- Research 8-16 actual validators to recommend
- Calculate current APY from network
- Prepare validator comparison spreadsheet
- Test all UI flows in advance

### Editing Notes

**Pacing:**
- Slow down during validator selection (critical decision)
- Add pauses for viewers to read validator details
- Speed up transaction waiting periods (2x)

**Graphics:**
- Highlight sortable columns in validator browser
- Add tooltips for terms (APY, commission, slashing)
- Use consistent color coding: green (good), yellow (caution), red (warning)

---

**Tutorial Complete**
Next: 04-deploying-smart-contracts.md
