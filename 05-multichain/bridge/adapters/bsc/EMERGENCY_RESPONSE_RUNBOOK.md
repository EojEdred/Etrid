# Emergency Response Runbook

**Critical incident response guide for MasterChef LP Rewards Program**

🚨 **For Emergency Use Only** 🚨

Last Updated: October 24, 2025

---

## 📋 Emergency Contact Information

**Update these before going live:**

### Primary Contacts
- **Lead Developer**: eoj@etrid.io
- **Multi-Sig Signers**: [Add contact info]
- **Community Manager**: [Add contact info]
- **Security Auditor**: [Add contact info]

### External Resources
- **BscScan Support**: https://bscscan.com/contactus
- **PancakeSwap Discord**: https://discord.gg/pancakeswap
- **Binance Support**: https://www.binance.com/en/support

---

## 🚨 Emergency Classification

### 🔴 CRITICAL (Act immediately - within 15 minutes)
- Smart contract exploit detected
- Unauthorized token minting/draining
- Contract ownership compromised
- Incorrect reward distribution at scale

**Action**: Pause contracts immediately, assemble multi-sig signers

---

### 🟠 HIGH (Act within 1 hour)
- Rewards running out sooner than expected
- LP pool configuration error
- Multi-sig access issues
- Significant APR calculation error

**Action**: Assess impact, prepare fix, communicate to team

---

### 🟡 MEDIUM (Act within 24 hours)
- Website/dashboard down
- Monitoring alerts failing
- Minor APR discrepancies
- User deposit issues (isolated)

**Action**: Investigate, schedule fix, monitor closely

---

### 🟢 LOW (Act within 1 week)
- Documentation outdated
- Non-critical UI issues
- Feature requests
- Performance optimization

**Action**: Log issue, plan fix for next maintenance window

---

## 🔴 CRITICAL EMERGENCIES

### 1. Smart Contract Exploit Detected

**Symptoms**:
- Unusual transactions on BscScan
- Rapid balance changes
- Community reports of stolen funds
- Unauthorized minting/burning

**Immediate Actions** (within 5 minutes):

```bash
# 1. PAUSE ALL CONTRACTS IMMEDIATELY (if you still have access)
# Via BscScan (if you're still owner):
# - Go to MasterChef contract
# - Call pause()

# Via script (if not transferred ownership yet):
npx hardhat console --network bscMainnet
> const masterChef = await ethers.getContractAt("MasterChef", "YOUR_ADDRESS")
> await masterChef.pause()

# 2. Check health immediately
npm run check-pool-health:mainnet
```

**Within 15 minutes**:
1. ✅ Assemble multi-sig signers (need M-of-N for pause if ownership transferred)
2. ✅ Screenshot all evidence (BscScan transactions, balances)
3. ✅ Post public warning on social media / Discord / Telegram
4. ✅ Contact security auditor if available

**Within 1 hour**:
1. Analyze exploit vector
2. Estimate funds at risk / already lost
3. Prepare incident report
4. Contact BscScan to flag contract if malicious
5. Consider emergency withdrawal for remaining users

**Post-Incident**:
1. Full security audit
2. Bug bounty payment if whitehack
3. Post-mortem report
4. Compensation plan for affected users
5. Smart contract upgrade if needed

---

### 2. MasterChef Running Out of ÉTR

**Symptoms**:
- `npm run check-pool-health:mainnet` shows days remaining < 3
- Users reporting failed harvest transactions
- MasterChef balance critically low

**Immediate Actions** (within 30 minutes):

```bash
# 1. Check current balance
npm run check-pool-health:mainnet

# 2. Check your ÉTR holdings
npx hardhat console --network bscMainnet
> const etr = await ethers.getContractAt("EtridToken", "ETR_ADDRESS")
> const balance = await etr.balanceOf("YOUR_WALLET")
> console.log(ethers.formatEther(balance))

# 3. If you have ÉTR, fund immediately
npm run fund-masterchef:mainnet

# 4. If you don't have ÉTR, reduce emission rate temporarily
npm run update-emission:mainnet
# Enter lower rate to extend runway
```

**If funds exhausted before you can top up**:
1. Post immediate announcement explaining situation
2. Pause deposits (let existing users withdraw)
3. Prepare to source additional ÉTR
4. Calculate owed rewards and prepare compensation plan

**Prevention**:
- Set up automated monitoring (see monitoring setup section)
- Alert when < 30 days remaining
- Top up monthly, not last minute

---

### 3. Ownership Compromised

**Symptoms**:
- Ownership transferred without your authorization
- Cannot execute owner functions
- Unexpected contract changes

**Immediate Actions**:

```bash
# 1. Verify current owner
npx hardhat console --network bscMainnet
> const masterChef = await ethers.getContractAt("MasterChef", "ADDRESS")
> const owner = await masterChef.owner()
> console.log("Current owner:", owner)

# 2. If compromised and you still have access to ÉTR token:
# - Pause ÉTR token to prevent further minting
> const etr = await ethers.getContractAt("EtridToken", "ETR_ADDRESS")
> await etr.pause()
```

**Within 15 minutes**:
1. ✅ Post public warning immediately
2. ✅ Contact all multi-sig signers
3. ✅ Warn users to NOT deposit
4. ✅ Screenshot all evidence
5. ✅ File police report if significant funds

**Recovery**:
- If private key stolen: No recovery possible, deploy new contracts
- If multi-sig compromise: Work with remaining signers to regain control
- If contract exploit: May need new deployment

**Prevention**:
- Use hardware wallet for deployer key
- Use multi-sig (Gnosis Safe) for all admin functions
- Never share private keys
- Regularly rotate access

---

## 🟠 HIGH PRIORITY INCIDENTS

### 4. LP Pool Misconfigured

**Symptoms**:
- APR calculation shows unexpected values
- One pool getting 100% of rewards when it shouldn't
- Users reporting no rewards

**Immediate Actions**:

```bash
# 1. Check pool configuration
npm run monitor-tvl:mainnet
npm run calculate-apr:mainnet

# 2. Check total allocation
npx hardhat console --network bscMainnet
> const masterChef = await ethers.getContractAt("MasterChef", "ADDRESS")
> const totalAlloc = await masterChef.totalAllocPoint()
> console.log("Total allocation:", totalAlloc.toString())

# 3. Check each pool's allocation
> for (let i = 0; i < poolCount; i++) {
    const pool = await masterChef.poolInfo(i)
    console.log(`Pool ${i}:`, pool.allocPoint.toString())
  }

# 4. Fix allocation if wrong
npm run add-pool:mainnet  # If missing a pool
# OR use BscScan to call set() function to update allocation
```

**Communication**:
1. Post announcement explaining issue
2. Explain if/how rewards will be adjusted
3. Provide timeline for fix
4. Offer to manually compensate if rewards were missed

---

### 5. Emission Rate Update Failed

**Symptoms**:
- APR not matching announced rate
- Emission rate still showing old value
- Users complaining about APR discrepancy

**Immediate Actions**:

```bash
# 1. Verify current emission rate
npm run calculate-apr:mainnet

# 2. Check what it should be
# Month 1: 2.89 ÉTR/block
# Month 2: 4.05 ÉTR/block
# Month 3: 4.63 ÉTR/block
# (see MASTERCHEF_GUIDE.md for full schedule)

# 3. Update if wrong
npm run update-emission:mainnet

# 4. Verify it updated
npm run check-pool-health:mainnet
```

**Communication**:
- Post correction notice
- Apologize for confusion
- Confirm correct APR now in effect

---

### 6. Multi-Sig Access Issues

**Symptoms**:
- Cannot get enough signers to approve transaction
- Signer lost access to wallet
- Time-sensitive operation blocked

**Immediate Actions**:

1. **Contact all signers immediately**
   - Email, call, Discord, Telegram
   - Explain urgency
   - Coordinate signing time

2. **If cannot reach required signers**:
   - Check if operation can wait
   - Consider if emergency justifies risk
   - Document decision carefully

3. **For critical operations (pause, etc.)**:
   - Use any remaining admin access if available
   - Consider if can announce workaround to users
   - Prepare post-mortem on multi-sig setup

**Prevention**:
- Require M-of-N where M is achievable (e.g., 3-of-5, not 5-of-5)
- Have backup signers in different timezones
- Test multi-sig operations regularly
- Document recovery procedures
- Keep signer contact info updated

---

## 🟡 MEDIUM PRIORITY INCIDENTS

### 7. User Cannot Deposit/Withdraw

**Symptoms**:
- Individual user reports transaction failing
- Error message on BscScan
- Harvest/withdraw reverting

**Investigation**:

```bash
# 1. Check if contract is paused
npm run check-pool-health:mainnet

# 2. Check if user has approved LP tokens
# Ask user to check on BscScan:
# LP Token contract > Read Contract > allowance(user, masterChef)

# 3. Check if pool exists and is configured
npm run monitor-tvl:mainnet

# 4. Check if MasterChef has enough ÉTR for rewards
npm run check-pool-health:mainnet
```

**Common Fixes**:
- User needs to approve LP tokens first
- User trying to withdraw more than deposited
- Contract paused (check why, unpause if safe)
- Pool not added yet (add it)
- MasterChef out of ÉTR (fund it)

---

### 8. TVL Discrepancy

**Symptoms**:
- TVL on website doesn't match BscScan
- Analytics showing incorrect values
- Community questioning numbers

**Investigation**:

```bash
# 1. Get accurate on-chain data
npm run monitor-tvl:mainnet

# 2. Check if price feeds are working
# (if you've integrated price oracles)

# 3. Verify LP token decimals
npx hardhat console --network bscMainnet
> const lpToken = await ethers.getContractAt("IERC20", "LP_ADDRESS")
> const decimals = await lpToken.decimals()
> console.log("Decimals:", decimals)
```

**Common Causes**:
- Price feed down/outdated
- Decimal conversion error
- Cached data on website
- Using wrong LP token address

**Fix**:
- Update website with correct on-chain data
- Fix price feed integration
- Post correction notice

---

## 🛠️ Emergency Tools & Commands

### Quick Health Check

```bash
npm run check-pool-health:mainnet
```

### Check MasterChef Balance

```bash
npx hardhat console --network bscMainnet
> const etr = await ethers.getContractAt("EtridToken", "ETR_ADDRESS")
> const masterChef = "MASTERCHEF_ADDRESS"
> const balance = await etr.balanceOf(masterChef)
> console.log("Balance:", ethers.formatEther(balance), "ÉTR")
```

### Check Pause Status

```bash
npx hardhat console --network bscMainnet
> const masterChef = await ethers.getContractAt("MasterChef", "ADDRESS")
> const paused = await masterChef.paused()
> console.log("Paused:", paused)
```

### Check Owner

```bash
npx hardhat console --network bscMainnet
> const masterChef = await ethers.getContractAt("MasterChef", "ADDRESS")
> const owner = await masterChef.owner()
> console.log("Owner:", owner)
```

### Get Recent Transactions

```
Visit BscScan:
https://bscscan.com/address/YOUR_MASTERCHEF_ADDRESS

Filter by:
- Method: deposit, withdraw, harvest
- Age: Last 24 hours
Look for unusual patterns
```

---

## 📢 Communication Templates

### Template: Contract Paused (Critical)

```
🚨 URGENT ANNOUNCEMENT 🚨

We have temporarily PAUSED the MasterChef contract as a precautionary measure.

What this means:
- No new deposits allowed
- Withdrawals ARE still allowed
- Your funds are safe

We are investigating [brief description of issue] and will provide an update within [timeframe].

You can still withdraw your LP tokens at any time.

Thank you for your patience.

Update to follow: [time]
```

---

### Template: Rewards Running Low

```
📢 IMPORTANT NOTICE

Our MasterChef reward balance is running lower than expected.

Current status:
- Rewards remaining: X days
- We are topping up: Y ÉTR
- Normal operations: Continue as normal

There is NO risk to your deposited LP tokens. Rewards distribution will continue uninterrupted.

We're implementing additional monitoring to prevent this in the future.

Questions? Ask in #support
```

---

### Template: Configuration Error

```
⚠️ ANNOUNCEMENT

We discovered a configuration error in Pool [X]:
- Issue: [description]
- Impact: [who was affected]
- Status: FIXED

If you were affected:
[Explanation of compensation if applicable]

We apologize for the inconvenience and have added additional checks to prevent this.

Full post-mortem: [link]
```

---

## 📊 Monitoring & Alerts

### Daily Health Check (Automated)

Add to cron:
```bash
0 9,15,21 * * * cd /path/to/bsc && npm run check-pool-health:mainnet && npm run monitor-tvl:mainnet
```

Runs at 9 AM, 3 PM, 9 PM daily.

---

### Alert Thresholds

**Set up alerts for:**
- ✅ Days of ÉTR remaining < 30 (warning) / < 7 (critical)
- ✅ TVL drops > 50% in 24 hours
- ✅ Pool health check fails
- ✅ Unusual transaction patterns (>10% of TVL in single tx)
- ✅ Contract pause status changes

**Alert Destinations:**
- Email to all admins
- Discord webhook
- Telegram bot
- SMS for critical alerts

See `AUTOMATED_MONITORING_SETUP.md` for configuration.

---

## 📝 Post-Incident Checklist

After resolving any emergency:

- [ ] Document what happened (timeline, root cause)
- [ ] Document what actions were taken
- [ ] Document outcome (funds recovered, issue fixed, etc.)
- [ ] Identify what could have prevented it
- [ ] Implement preventive measures
- [ ] Update this runbook if needed
- [ ] Post public post-mortem (transparency)
- [ ] Thank community for patience
- [ ] Conduct team retrospective
- [ ] Update monitoring/alerts to catch it earlier next time

---

## 🔐 Security Best Practices

### Before Going Live
- [ ] Full security audit completed
- [ ] Bug bounty program announced
- [ ] Multi-sig configured and tested
- [ ] All private keys secured (hardware wallets)
- [ ] Emergency contacts list updated
- [ ] Monitoring and alerts configured
- [ ] Team trained on this runbook
- [ ] Testnet fully tested
- [ ] Insurance explored (if available)

### Ongoing
- [ ] Weekly health checks
- [ ] Monthly security reviews
- [ ] Quarterly access audits
- [ ] Keep this runbook updated
- [ ] Practice emergency drills
- [ ] Monitor security news
- [ ] Update dependencies regularly

---

## 📞 Escalation Path

### Level 1: Developer On-Call
- Check logs and monitoring
- Run diagnostic scripts
- Fix if within authority
- Escalate if needed

### Level 2: Lead Developer
- Assess severity
- Coordinate response
- Communicate to team
- Decide if need Level 3

### Level 3: Multi-Sig + Leadership
- Critical decisions
- Contract pauses
- Public communications
- Compensation decisions

### Level 4: External (if needed)
- Security audit firm
- Legal counsel
- Law enforcement (if theft)
- Insurance (if applicable)

---

## 📚 Related Documentation

- [Scripts README](SCRIPTS_README.md) - All script commands
- [MasterChef Guide](MASTERCHEF_GUIDE.md) - Contract interactions
- [Deployment Guide](README_DEPLOYMENT.md) - Deployment procedures
- [Automated Monitoring Setup](AUTOMATED_MONITORING_SETUP.md) - Monitoring config

---

## ⚠️ IMPORTANT REMINDER

**This runbook is only as good as your preparation.**

Before going live:
1. ✅ Read this entire document
2. ✅ Update emergency contacts
3. ✅ Test multi-sig access
4. ✅ Configure monitoring
5. ✅ Practice incident response
6. ✅ Keep this updated

**Hope for the best. Prepare for the worst.**

---

**Last Updated**: October 24, 2025
**Version**: 1.0
**Status**: Production Ready

🚨 **Keep this document accessible 24/7** 🚨
