# CONSËNSUS - ËTRID Governance Portal

**Custom governance UI built specifically for ËTRID Protocol**

## Overview

This is a standalone governance application that implements ËTRID's unique **Consensus Day** governance model. Unlike generic governance frameworks (like Snapshot), this UI is custom-built to support ËTRID's specific governance mechanics:

- **Consensus Day**: Annual 22-hour governance event (December 1st)
- **Four Phases**: Registration → Voting → Minting → Distribution
- **Proposal Types**: Inflation rate, parameter changes, budget, upgrades, director elections
- **Stake-Weighted Voting**: Based on staked amount, duration, and participation history

## Features

✅ **Dual Bootstrap Node Support**
- Connects to VM #1 (Alice): `20.186.91.207:9944`
- Automatic failover to VM #2 (Bob): `172.177.44.73:9944`

✅ **Consensus Day Countdown**
- Real-time countdown to next Consensus Day
- Phase indicator (Registration/Voting/Minting/Distribution)

✅ **Proposal Management**
- View active proposals
- Submit new proposals (10,000 ÉTR bond required)
- Vote on proposals with stake-weighted voting power

✅ **Wallet Integration**
- Polkadot.js extension support
- View your voting power
- Track participation history

✅ **Governance History**
- View past Consensus Day results
- Track historical votes and outcomes

## Files

- `index.html` - Main governance portal UI
- `app.js` - JavaScript logic & blockchain connection
- `.htaccess` - Hostinger server configuration
- `README.md` - This file

## Deployment to Hostinger

### Option 1: Manual Upload

1. **Upload via File Manager**:
   - Go to Hostinger File Manager
   - Navigate to `/public_html/governance/`
   - Upload all files from this directory
   - Extract if uploaded as ZIP

2. **Set Permissions**:
   - Ensure `.htaccess` has 644 permissions
   - Ensure `index.html` and `app.js` have 644 permissions

3. **Test**:
   - Visit https://governance.etrid.org
   - Should see the Consensus Day countdown

### Option 2: Create ZIP and Upload

```bash
# Create ZIP
cd /Users/macbook/Desktop/etrid/hostinger-upload
zip -r governance-dual-node.zip governance-standalone/

# Upload governance-dual-node.zip to Hostinger
# Extract in /public_html/governance/
```

## Azure Firewall Requirements

⚠️ **CRITICAL**: Both Azure VMs must have port 9944 open to `0.0.0.0/0`:

1. **VM #1 (Alice - 20.186.91.207)**:
   - Azure Portal → VM → Networking
   - Allow inbound port 9944 from 0.0.0.0/0

2. **VM #2 (Bob - 172.177.44.73)**:
   - Azure Portal → VM → Networking
   - Allow inbound port 9944 from 0.0.0.0/0

Without open firewalls, the governance portal cannot connect to the blockchain!

## How It Works

1. **Page Load**:
   - Connects to Azure bootstrap nodes (tries Alice first, then Bob)
   - Loads governance data from blockchain
   - Starts Consensus Day countdown timer

2. **Connect Wallet**:
   - User clicks "Connect Wallet"
   - Polkadot.js extension prompts for account
   - Loads user's voting power and stake info

3. **Voting**:
   - During Consensus Day Voting Phase (06:00-18:00 UTC)
   - Users can vote on active proposals
   - Votes weighted by stake amount, duration, and history

4. **Submit Proposal**:
   - During Registration Phase (00:00-06:00 UTC)
   - Requires 10,000 ÉTR bond
   - Requires 3 validator supporters
   - Bond refunded if proposal reaches 5% quorum

## Governance Specs

Based on **ËTRID Ivory Paper Volume III: Governance & Fiscal Mechanics**

### Consensus Day Timeline

| Phase | Time (UTC) | Duration | Actions |
|-------|-----------|----------|---------|
| **Registration** | 00:00-06:00 | 6 hours | Submit proposals, lock stakes |
| **Voting** | 06:00-18:00 | 12 hours | Cast votes on proposals |
| **Minting** | 18:00-21:00 | 3 hours | Calculate results, mint tokens |
| **Distribution** | 21:00-22:00 | 1 hour | Distribute rewards |

### Proposal Requirements

- **Minimum Bond**: 10,000 ÉTR
- **Validator Support**: 3 validators minimum
- **Quorum for Refund**: 5% of total voting power
- **Character Limits**:
  - Title: 100 characters max
  - Description: 2000 characters max

### Voting Power Formula

```
VotingPower = StakedAmount × (1 + DurationBonus + HistoryBonus)

DurationBonus: Up to +20% based on how long stake has been locked
HistoryBonus: Up to +10% based on past participation (2% per past Consensus Day)
```

## Development Notes

### Mock Data

Currently uses **mock data** until the on-chain governance pallet is fully deployed. Once the pallet is live, update these sections in `app.js`:

```javascript
// Replace mock data with real queries:
const proposals = await api.query.governance.activeProposals();
const voters = await api.query.governance.registeredVoters();
const inflation = await api.query.governance.currentInflation();
const votingPower = await api.query.governance.votingPower(account.address);
```

### Future Enhancements

- [ ] Connect to real governance pallet queries
- [ ] Implement actual voting transactions
- [ ] Add proposal submission transactions
- [ ] Real-time vote counting during Voting Phase
- [ ] Notification system for phase changes
- [ ] Mobile app version
- [ ] Multi-language support

## Technical Stack

- **HTML/CSS**: TailwindCSS for styling
- **JavaScript**: Vanilla JS (no framework overhead)
- **Blockchain**: Polkadot.js API for Substrate connection
- **Wallet**: Polkadot.js extension integration
- **Deployment**: Static hosting on Hostinger

## Support

- **Documentation**: https://docs.etrid.org
- **Ivory Papers**: https://etrid.org/whitepaper/
- **GitHub**: https://github.com/EojEdred/Etrid

## License

GPLv3 Open Source License

---

**Built specifically for ËTRID Protocol**
*The Free and Open Decentralized Democracy of Stakeholders*
