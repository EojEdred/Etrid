# BSC Scripts Quick Reference

**Quick reference guide for all deployment, admin, and monitoring scripts**

Last Updated: October 24, 2025

---

## 📚 Table of Contents

- [Setup Scripts](#setup-scripts)
- [Deployment Scripts](#deployment-scripts)
- [Validation Scripts](#validation-scripts)
- [Admin Helper Scripts](#admin-helper-scripts)
- [Monitoring Scripts](#monitoring-scripts)
- [Common Workflows](#common-workflows)
- [Troubleshooting](#troubleshooting)

---

## 🔧 Setup Scripts

### Generate Wallet

**Purpose**: Generate new Ethereum wallet for deployment

```bash
npm run generate-wallet
```

**Output**:
- Address
- Private key (add to .env as DEPLOYER_PRIVATE_KEY)
- Mnemonic phrase (backup safely!)

**When to use**: Before deploying anything

---

### Check Balance

**Purpose**: Check your BNB balance and estimate gas costs

```bash
npm run check-balance
```

**Shows**:
- Current BNB balance
- Estimated deployment costs
- Whether you have enough gas

**When to use**: Before deploying to verify sufficient funds

---

## 🚀 Deployment Scripts

### Deploy ÉTR Token - Testnet

```bash
npm run deploy:testnet
```

**Cost**: ~$0 (testnet BNB from faucet)
**Time**: ~30 seconds
**Saves**: `deployment-testnet.json`

---

### Deploy ÉTR Token - Mainnet

```bash
npm run deploy:mainnet
```

**Cost**: $5-10
**Time**: ~1 minute
**Requires**: Multiple safety confirmations
**Saves**: `deployment-mainnet.json`

⚠️ **CRITICAL**: Run `npm run pre-launch-check:mainnet` first!

---

### Deploy MasterChef - Testnet

```bash
npm run deploy:masterchef:testnet
```

**Cost**: ~$0 (testnet BNB)
**Time**: ~30 seconds
**Requires**: ÉTR token deployed first
**Saves**: `masterchef-deployment-testnet.json`

---

### Deploy MasterChef - Mainnet

```bash
npm run deploy:masterchef:mainnet
```

**Cost**: $5-10
**Time**: ~1 minute
**Requires**:
- ÉTR token deployed
- Multiple safety confirmations
- 20M ÉTR ready to transfer

**Saves**: `masterchef-deployment-mainnet.json`

⚠️ **CRITICAL**: Run `npm run pre-launch-check:mainnet` first!

---

## ✅ Validation Scripts

### Pre-Launch Check

**Purpose**: Comprehensive validation before mainnet deployment

```bash
# Testnet
npm run pre-launch-check:testnet

# Mainnet (CRITICAL - always run before mainnet!)
npm run pre-launch-check:mainnet
```

**Checks**:
- ✅ Environment configuration
- ✅ Wallet balance
- ✅ Contract deployment status
- ✅ MasterChef configuration
- ✅ Testnet testing completion
- ✅ Security settings
- ✅ Documentation

**Output**:
- ✅/⚠️/❌ for each check
- Detailed report
- JSON file for records
- **Exits with error if critical issues found**

**When to use**:
- Always before mainnet deployment
- After configuration changes
- Monthly for health verification

---

### Verify Contract

```bash
npm run verify:testnet
```

**Purpose**: Verify contract source code on BscScan
**When to use**: After deploying (makes contract readable on BscScan)

---

## 🔐 Admin Helper Scripts

### Fund MasterChef

**Purpose**: Transfer 20M ÉTR to MasterChef for rewards

```bash
# Testnet
npm run fund-masterchef:testnet

# Mainnet
npm run fund-masterchef:mainnet
```

**Prompts for**:
- Amount to transfer (default: 20M ÉTR)

**Validates**:
- Sufficient balance
- Correct recipient

**When to use**:
- After deploying MasterChef
- To top up rewards if running low

---

### Add LP Pool

**Purpose**: Add new liquidity pool to MasterChef

```bash
# Testnet
npm run add-pool:testnet

# Mainnet
npm run add-pool:mainnet
```

**Prompts for**:
- LP token address
- Allocation points (reward weight)
- Whether to update existing pools

**Validates**:
- LP token exists
- Not already added
- Allocation is reasonable

**When to use**:
- Adding ÉTR/BNB pool (first time)
- Adding new pairs (ÉTR/SOL, ÉTR/USDT, etc.)

---

### Update Emission Rate

**Purpose**: Update monthly emission rate (for APR changes)

```bash
# Testnet
npm run update-emission:testnet

# Mainnet
npm run update-emission:mainnet
```

**Prompts for**:
- New emission rate (ÉTR per block)
- Reason for update (optional)

**Shows**:
- Current rate
- Emission schedule reference
- Projected daily/monthly emissions
- Change percentage

**When to use**: End of each month to adjust APR

**Schedule**:
- Nov 30: 2.89 → 4.05 ÉTR/block
- Dec 31: 4.05 → 4.63 ÉTR/block
- Jan 31: stays at 4.63 ÉTR/block
- Feb 28: 4.63 → 4.05 ÉTR/block
- Mar 31: 4.05 → 2.89 ÉTR/block
- Apr 30: 2.89 → 1.16 ÉTR/block

---

### Transfer Ownership

**Purpose**: Transfer contract ownership to multi-sig wallet

```bash
# Testnet
npm run transfer-ownership:testnet

# Mainnet
npm run transfer-ownership:mainnet
```

**Prompts for**:
- Which contract (ÉTR, MasterChef, or both)
- Multi-sig wallet address

**Validates**:
- You are current owner
- Multi-sig address is valid
- (Mainnet) Multi-sig is a contract, not EOA

**Requires**: Multiple confirmations (irreversible!)

**When to use**:
- After testing on testnet
- Before announcing to community
- **ONLY ONCE** - this cannot be undone!

⚠️ **CRITICAL**: Test on testnet first! This is irreversible!

---

## 📊 Monitoring Scripts

### Monitor TVL

**Purpose**: Track Total Value Locked across all pools

```bash
# Testnet
npm run monitor-tvl:testnet

# Mainnet
npm run monitor-tvl:mainnet
```

**Shows**:
- LP staked per pool
- Allocation points
- Reward share percentage
- Last update block

**Outputs**: JSON report file

**When to use**:
- Daily monitoring
- Before/after pool additions
- To track growth

---

### Calculate APR

**Purpose**: Calculate real-time APR for each pool

```bash
# Testnet
npm run calculate-apr:testnet

# Mainnet
npm run calculate-apr:mainnet
```

**Shows**:
- Emission schedule
- Daily/monthly/yearly rewards per pool
- (Interactive) APR based on TVL and price

**Prompts for**:
- ÉTR price (USD)
- Pool TVL (USD)

**Outputs**: JSON report file

**When to use**:
- Before announcing APR
- Monthly APR updates
- To verify expected returns

---

### Check Pool Health

**Purpose**: Comprehensive health check for all pools

```bash
# Testnet
npm run check-pool-health:testnet

# Mainnet
npm run check-pool-health:mainnet
```

**Checks**:
- ✅ Basic configuration (reward token, pause status)
- ✅ Reward balance sufficiency
- ✅ Pool health (LP tokens, allocations)
- ✅ Ownership & security
- ✅ Emission schedule validity

**Outputs**:
- Health report with ✅/⚠️/❌
- JSON report file
- **Exits with error if critical issues**

**When to use**:
- Daily (automated)
- After configuration changes
- Before major announcements
- Monthly maintenance

---

### Export Metrics

**Purpose**: Export comprehensive metrics for dashboards

```bash
# Testnet
npm run export-metrics:testnet

# Mainnet
npm run export-metrics:mainnet
```

**Exports**:
- **JSON** (for APIs, databases, dashboards)
- **CSV** (for Excel, Sheets, analysis)
- **Prometheus** (for Grafana, monitoring)

**Includes**:
- Pool data (staked, rewards, allocations)
- Emissions (rate, daily, monthly, yearly)
- Balances (MasterChef ÉTR, days remaining)
- Configuration (pause status, owner)

**When to use**:
- Hourly (automated)
- For website displays
- For Grafana dashboards
- For historical tracking

---

## 🔄 Common Workflows

### First-Time Deployment (Testnet)

```bash
# 1. Setup
npm install
npm run generate-wallet
# Get testnet BNB from faucet

# 2. Deploy contracts
npm run deploy:testnet
npm run deploy:masterchef:testnet

# 3. Validate
npm run pre-launch-check:testnet

# 4. Configure
npm run fund-masterchef:testnet
npm run add-pool:testnet

# 5. Test monitoring
npm run check-pool-health:testnet
npm run monitor-tvl:testnet
npm run calculate-apr:testnet
```

---

### Mainnet Launch

```bash
# 1. Pre-launch validation (CRITICAL!)
npm run pre-launch-check:mainnet

# 2. Deploy (if checks passed)
npm run deploy:mainnet
npm run deploy:masterchef:mainnet

# 3. Configure
npm run fund-masterchef:mainnet
npm run add-pool:mainnet

# 4. Verify health
npm run check-pool-health:mainnet

# 5. Transfer to multi-sig (LAST STEP!)
npm run transfer-ownership:mainnet
```

---

### Monthly Maintenance

```bash
# 1. Health check
npm run check-pool-health:mainnet

# 2. Monitor metrics
npm run monitor-tvl:mainnet
npm run calculate-apr:mainnet

# 3. Update emission rate (end of month)
npm run update-emission:mainnet

# 4. Export metrics
npm run export-metrics:mainnet
```

---

### Daily Operations

```bash
# Morning: Health check
npm run check-pool-health:mainnet

# Afternoon: Monitor TVL
npm run monitor-tvl:mainnet

# Evening: Export metrics for dashboard
npm run export-metrics:mainnet
```

---

### Adding New LP Pool

```bash
# 1. Verify pool health first
npm run check-pool-health:mainnet

# 2. Add new pool
npm run add-pool:mainnet
# Follow prompts to enter LP token address and allocation

# 3. Verify it was added correctly
npm run monitor-tvl:mainnet
npm run check-pool-health:mainnet

# 4. Calculate new APRs
npm run calculate-apr:mainnet

# 5. Announce to community with new APR
```

---

## 🆘 Troubleshooting

### "Insufficient funds for gas"

**Solution**:
```bash
npm run check-balance
# Get more BNB if needed
```

---

### "Contract not deployed"

**Solution**:
```bash
# Check .env has correct addresses
cat .env | grep ADDRESS

# Deploy if missing
npm run deploy:testnet
```

---

### "Not owner of contract"

**Solution**:
- Check you're using the correct wallet (deployer)
- If ownership was transferred, use multi-sig instead
- For multi-sig operations, use BscScan directly

---

### "MasterChef: LP token already added"

**Solution**: This LP token is already in MasterChef. Use a different LP token or ignore if intentional.

---

### "Days remaining < 7"

**Solution**:
```bash
# Top up MasterChef with more ÉTR
npm run fund-masterchef:mainnet
```

---

### Health check fails with critical issues

**Solution**:
1. Read the error messages carefully
2. Fix critical issues before proceeding
3. Re-run health check
4. Do NOT deploy to mainnet until all critical checks pass

---

### Pre-launch check fails

**Solution**:
1. Read which checks failed
2. Fix the issues (insufficient balance, missing contracts, etc.)
3. Re-run: `npm run pre-launch-check:mainnet`
4. Only proceed when all critical checks pass

---

## 📝 Script Output Files

All scripts save reports to the `bsc/` directory:

- `deployment-{testnet|mainnet}.json` - Contract deployment info
- `masterchef-deployment-{testnet|mainnet}.json` - MasterChef deployment info
- `pre-launch-report-{timestamp}.json` - Pre-launch validation results
- `tvl-report-{timestamp}.json` - TVL monitoring data
- `apr-report-{timestamp}.json` - APR calculation data
- `health-report-{timestamp}.json` - Pool health check results
- `metrics-{timestamp}.json` - Comprehensive metrics (JSON)
- `pool-metrics-{timestamp}.csv` - Pool metrics (CSV)
- `overview-metrics-{timestamp}.csv` - Overview metrics (CSV)
- `metrics-{timestamp}.prom` - Metrics (Prometheus format)

**Backup these files!** They contain deployment history and operational data.

---

## 🔒 Security Best Practices

### For All Scripts

✅ **DO**:
- Run on testnet first
- Verify addresses carefully
- Keep private keys secure
- Backup all .json output files
- Review gas estimates before confirming

❌ **DON'T**:
- Share private keys
- Commit .env to git
- Skip pre-launch checks
- Deploy to mainnet without testing
- Transfer ownership without multi-sig ready

---

### For Mainnet Operations

✅ **CRITICAL**:
- Always run `npm run pre-launch-check:mainnet` first
- Double-check all addresses
- Understand that transactions are irreversible
- Have multi-sig configured before transferring ownership
- Announce changes to community before executing

---

## 📞 Getting Help

**Script errors**: Check error message, see Troubleshooting section above

**Configuration issues**: Review `.env.example` and `README_DEPLOYMENT.md`

**Contract issues**: See `MASTERCHEF_GUIDE.md` for contract interactions

**Emergency**: See `EMERGENCY_RESPONSE_RUNBOOK.md`

---

**Quick Links**:
- [Main Deployment Guide](README_DEPLOYMENT.md)
- [MasterChef Guide](MASTERCHEF_GUIDE.md)
- [Emergency Runbook](EMERGENCY_RESPONSE_RUNBOOK.md)
- [Final Checklist](../../../../../../FINAL_DEPLOYMENT_CHECKLIST.md)

---

**Last Updated**: October 24, 2025
**Version**: 1.0
**Status**: Production Ready
