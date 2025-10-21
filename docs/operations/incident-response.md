# EDSC Bridge Incident Response Guide

Comprehensive incident response procedures for security and operational issues.

## Table of Contents

- [Overview](#overview)
- [Incident Classification](#incident-classification)
- [Response Team](#response-team)
- [General Response Process](#general-response-process)
- [Security Incidents](#security-incidents)
- [Operational Incidents](#operational-incidents)
- [Communication Templates](#communication-templates)
- [Post-Incident](#post-incident)
- [Appendix](#appendix)

---

## Overview

### Purpose

This guide provides step-by-step procedures for responding to incidents affecting the EDSC bridge, from detection through resolution and post-incident review.

### Scope

Covers:
- Security breaches and exploits
- Service outages
- Data integrity issues
- Performance degradation
- Third-party dependencies failure

### Key Principles

1. **Safety First**: Protect user funds and data
2. **Rapid Response**: Act quickly to contain damage
3. **Clear Communication**: Keep stakeholders informed
4. **Root Cause Analysis**: Understand what happened
5. **Continuous Improvement**: Learn and prevent recurrence

---

## Incident Classification

### Severity Levels

#### P0 - Critical (Response: Immediate, 24/7)

**Examples:**
- Bridge exploit or hack
- Funds at risk or stolen
- Complete bridge outage (>1 hour)
- Attester key compromise
- Smart contract vulnerability
- Data breach with PII exposure

**Response Team:** All hands on deck
**Communication:** Public statement within 1 hour
**Escalation:** CEO, CTO, Board (if financial impact >$100K)

#### P1 - High (Response: <30 min, business hours; <2 hours, off-hours)

**Examples:**
- Partial bridge outage
- Multiple services down
- Messages delayed >30 minutes
- Threshold at risk (2/5 attesters down)
- Significant performance degradation
- Suspected security issue

**Response Team:** On-call + senior engineers
**Communication:** Status page update within 2 hours
**Escalation:** Engineering Manager, CTO

#### P2 - Medium (Response: <4 hours)

**Examples:**
- Single service degraded
- Messages delayed 10-30 minutes
- High error rates
- Elevated resource usage
- Non-critical monitoring alerts

**Response Team:** On-call engineer
**Communication:** Internal only, status page if user-facing
**Escalation:** Team lead

#### P3 - Low (Response: <24 hours)

**Examples:**
- Minor bugs
- Documentation issues
- Low-impact performance issues
- Single missed monitoring check

**Response Team:** Assigned engineer
**Communication:** Internal ticket/Slack
**Escalation:** None typically required

---

## Response Team

### Roles and Responsibilities

#### Incident Commander (IC)

**Primary:** On-call DevOps Engineer
**Backup:** Senior DevOps Engineer

**Responsibilities:**
- Declare incident and severity
- Coordinate response efforts
- Make go/no-go decisions
- Manage communications
- Call in additional resources
- Declare incident resolved

#### Technical Lead

**Primary:** Senior Backend Engineer
**Backup:** CTO

**Responsibilities:**
- Technical investigation
- Implement fixes
- Coordinate with developers
- Root cause analysis

#### Communications Lead

**Primary:** Community Manager
**Backup:** Marketing Manager

**Responsibilities:**
- Draft public statements
- Update status page
- Manage Discord/Twitter
- Coordinate with support team
- Prepare FAQs

#### Security Lead

**Primary:** Security Engineer
**Backup:** CTO

**Responsibilities:**
- Security assessment
- Evidence preservation
- Coordinate with auditors
- Legal/compliance liaison
- Post-incident security review

### Contact Information

Keep updated in PagerDuty and team wiki.

**On-Call Rotation:**
- Primary: [PagerDuty schedule]
- Backup: [PagerDuty schedule]

**Emergency Escalation:**
- CTO: [phone], [signal]
- CEO: [phone], [signal]
- Legal: [phone], [email]

---

## General Response Process

### Phase 1: Detection & Assessment (0-5 minutes)

```
1. ALERT RECEIVED
   ├─ Auto: Monitoring system → PagerDuty → On-call
   └─ Manual: User report → Support → On-call

2. ACKNOWLEDGE
   - Acknowledge in PagerDuty (stops escalation)
   - Join #incident-response Slack channel

3. INITIAL ASSESSMENT
   - Review alert details
   - Check service status dashboard
   - Review recent deployments/changes
   - Determine severity (P0-P3)

4. DECLARE INCIDENT
   /incident declare [P0|P1|P2|P3] "[Brief description]"

5. ASSEMBLE TEAM
   - P0: Page all senior engineers + management
   - P1: Page technical lead + communications
   - P2/P3: On-call handles, escalate if needed
```

### Phase 2: Containment (5-30 minutes)

```
1. STABILIZE
   - Stop the bleeding (pause services if needed)
   - Prevent additional damage
   - Preserve evidence (logs, transactions)

2. ASSESS IMPACT
   - How many users affected?
   - Funds at risk? How much?
   - Data compromised?
   - Reputation impact?

3. IMMEDIATE ACTIONS
   Based on incident type (see specific sections below)

4. COMMUNICATION
   - Internal: Update incident channel every 15 min
   - External: Post status update (P0/P1 only)

   Template:
   "We're aware of [issue] affecting [scope].
    Investigating. ETA for update: [time]."
```

### Phase 3: Investigation & Resolution (30 min - hours)

```
1. ROOT CAUSE ANALYSIS
   - Review logs systematically
   - Check recent changes (code, config, infra)
   - Reproduce issue if possible
   - Document findings in incident doc

2. DEVELOP FIX
   - Identify solution
   - Test in staging (if time permits)
   - Get approval from IC
   - Prepare rollback plan

3. DEPLOY FIX
   - Announce deployment window
   - Execute deployment
   - Monitor metrics closely
   - Verify fix working

4. VERIFY RESOLUTION
   - Confirm issue resolved
   - Check for side effects
   - Monitor for 30 min minimum
   - Get IC approval to resolve
```

### Phase 4: Recovery & Monitoring (1-24 hours)

```
1. GRADUAL RECOVERY
   - Resume services incrementally
   - Monitor metrics continuously
   - Be ready to rollback

2. VERIFY NORMAL OPERATIONS
   - All services healthy
   - Metrics return to baseline
   - No new related errors

3. COMMUNICATE RESOLUTION
   - Update status page: "Resolved"
   - Post-mortem preview (24-48h)
   - Thank users for patience

4. SCHEDULE POST-MORTEM
   - Within 72 hours for P0/P1
   - Within 1 week for P2
   - P3: Optional
```

---

## Security Incidents

### SEC-1: Bridge Exploit / Unauthorized Transfer

**Indicators:**
- Unexpected mint/burn events
- Balance discrepancies
- Invalid signature accepted
- Unusual transaction patterns

**IMMEDIATE ACTIONS (within 5 minutes):**

```bash
# 1. PAUSE ALL RELAYERS
for host in relayer-{1..3}.etrid.io; do
  ssh $host "pm2 stop relayer-service"
done

# 2. VERIFY EXTENT OF DAMAGE
# Check recent transactions on both chains
# Ethereum
curl https://api.etherscan.io/api?module=account&action=txlist&address=MESSAGE_TRANSMITTER&startblock=RECENT&apikey=KEY

# Substrate
# Check via block explorer or RPC

# 3. ASSESS FUNDS AT RISK
# Calculate total exposure
# Document all suspicious transactions

# 4. EMERGENCY COMMUNICATION
# Post IMMEDIATELY on all channels:
```

**Public Statement Template:**

```
⚠️ SECURITY ALERT ⚠️

The EDSC Bridge has been temporarily paused while we investigate
a potential security issue.

Your funds are safe. Do NOT attempt to use the bridge.

We will provide an update within 1 hour.

Follow @EtridMultichain for updates.
```

**INVESTIGATION (30 min - 2 hours):**

```bash
# 1. PRESERVE EVIDENCE
# Archive all logs
./scripts/backup-logs.sh
# Take snapshots of current state
# Document timeline of events

# 2. IDENTIFY ATTACK VECTOR
# - Smart contract vulnerability?
# - Attester compromise?
# - Replay attack?
# - Signature forgery?
# - Oracle manipulation?

# 3. CONTACT SECURITY PARTNERS
# - Notify auditors (CertiK, Trail of Bits)
# - Contact blockchain security firms
# - Consider white hat involvement

# 4. LEGAL/COMPLIANCE
# - Notify legal counsel
# - Prepare regulatory disclosures (if required)
# - Document everything
```

**MITIGATION:**

```bash
# Option A: Contract Pause (if function exists)
# Call emergency pause function from owner multisig

# Option B: Attester Coordination
# If attester key compromised:
# 1. Disable compromised attester in registry
# 2. Verify threshold still met with remaining attesters
# 3. Generate new keys for compromised attester

# Option C: Force Upgrade
# If smart contract vulnerability:
# 1. Deploy patched contracts
# 2. Migrate to new contracts via governance
# 3. Deprecate old contracts
```

**RECOVERY:**

```
1. PATCH VULNERABILITY
   - Deploy fix to testnet
   - Security audit patch
   - Deploy to mainnet

2. GRADUAL RESUME
   - Start with small test transfer
   - Monitor closely for 1 hour
   - Gradually increase limits
   - Full resume after 24h of stability

3. POST-INCIDENT
   - Public post-mortem
   - Compensate affected users (if applicable)
   - Security improvements roadmap
```

---

### SEC-2: Attester Private Key Compromise

**Indicators:**
- Unauthorized signatures detected
- Attester reports key theft
- Suspicious login activity

**IMMEDIATE ACTIONS:**

```bash
# 1. DISABLE COMPROMISED ATTESTER (within 1 minute)
# Via Ethereum (owner only)
cast send $ATTESTER_REGISTRY "disableAttester(address)" $COMPROMISED_ADDRESS \
  --private-key $OWNER_KEY

# Via Substrate (sudo only)
# Call: attestation.disableAttester(attester_account)

# 2. VERIFY THRESHOLD STILL MET
# Need 3 active attesters minimum
# If below threshold, emergency add new attester

# 3. AUDIT RECENT SIGNATURES
# Check all signatures from compromised attester in last 24h
# Verify they were legitimate

# 4. GENERATE NEW KEYS
# Secure environment only
node -e "console.log(require('ethers').Wallet.createRandom().privateKey)"
# Store in KMS/HSM

# 5. ADD NEW ATTESTER
# Call: AttesterRegistry.addAttester(new_address)
# Update service config with new key

# 6. RESTART SERVICE
pm2 restart attestation-service
```

**INVESTIGATION:**

```
1. How was key compromised?
   - Server breach?
   - Phishing?
   - Insider threat?
   - Poor key management?

2. What did attacker sign?
   - Review all signatures
   - Identify fraudulent transactions
   - Calculate damage

3. Preserve evidence
   - Server logs
   - Access logs
   - Network traffic
   - File timestamps
```

---

### SEC-3: DDoS Attack

**Indicators:**
- Extremely high request rates
- Service unavailability
- Network saturation
- Cloudflare alerts

**IMMEDIATE ACTIONS:**

```bash
# 1. ENABLE CLOUDFLARE "I'M UNDER ATTACK" MODE
# Or equivalent DDoS protection

# 2. RATE LIMITING
# Nginx config
location / {
  limit_req zone=api burst=20 nodelay;
  limit_conn addr 10;
}

# 3. BLOCK MALICIOUS IPs
# Identify top offenders
tail -f /var/log/nginx/access.log | awk '{print $1}' | sort | uniq -c | sort -rn | head -20

# Add to blocklist
ufw deny from $ATTACKER_IP

# 4. SCALE UP (if cloud)
# Increase instance count
# Enable auto-scaling

# 5. CDN/PROXY
# Route traffic through DDoS protection service
```

---

## Operational Incidents

### OPS-1: All Relayers Down

**Indicators:**
- No messages being relayed
- All relayer processes stopped
- `relayers_active:count` = 0

**ACTIONS:**

```bash
# 1. CHECK RELAYER PROCESSES
for host in relayer-{1..3}.etrid.io; do
  echo "=== $host ==="
  ssh $host "pm2 status"
done

# 2. RESTART RELAYERS
for host in relayer-{1..3}.etrid.io; do
  ssh $host "pm2 restart relayer-service"
done

# 3. CHECK LOGS FOR ERRORS
ssh relayer-1.etrid.io "pm2 logs relayer-service --lines 50"

# Common issues:
# - Out of ETH → Fund relayer
# - RPC connection failed → Check RPC endpoints
# - Code error → Rollback deployment

# 4. VERIFY RELAYING RESUMED
# Should see messages relaying within 2 minutes
curl https://attestation-0.etrid.io/attestations/ready | jq .count
# Should decrease over time
```

---

### OPS-2: Attestation Threshold At Risk

**Indicators:**
- Only 2/5 or 1/5 attesters active
- Alert: "AttestationThresholdAtRisk"

**ACTIONS:**

```bash
# 1. IDENTIFY DOWN ATTESTERS
for i in {0..4}; do
  echo "Attester $i:"
  curl -s https://attestation-$i.etrid.io/health | jq .status
done

# 2. RESTART DOWN ATTESTERS
ssh attestation-X.etrid.io "pm2 restart attestation-service"

# 3. IF CAN'T RESTART QUICKLY
# Emergency add temporary attester
# Deploy new attester instance
# Add to registry
# Configure with private key

# 4. INVESTIGATE ROOT CAUSE
# - Server failures?
# - Network issues?
# - Code bugs?
# - Coordinated attack?

# 5. VERIFY THRESHOLD RESTORED
# Need 3+ active
# Wait 5 minutes, verify signatures being created
```

---

### OPS-3: Chain RPC Failure

**Indicators:**
- Chain connection lost
- Logs: "WebSocket connection failed"
- No new blocks detected

**ACTIONS:**

```bash
# 1. TEST RPC DIRECTLY
# Ethereum
curl -X POST $ETHEREUM_RPC_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Substrate
wscat -c $SUBSTRATE_WS_URL
> {"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}

# 2. IF RPC DOWN
# Switch to backup RPC in .env
ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/BACKUP-KEY

# 3. RESTART SERVICES
pm2 restart attestation-service
pm2 restart relayer-service

# 4. IF SUBSTRATE RPC DOWN
# Check node status
ssh substrate-rpc.etrid.io "systemctl status edsc-pbc-node"

# Restart if needed
systemctl restart edsc-pbc-node

# 5. MONITOR RECOVERY
# Services should reconnect within 1-2 minutes
```

---

### OPS-4: High Gas Prices

**Indicators:**
- Gas price >100 gwei
- Relayers pausing/failing
- Alert: "HighEthereumGasPrice"

**ACTIONS:**

```bash
# 1. CHECK CURRENT GAS
curl -s "https://api.etherscan.io/api?module=gastracker&action=gasoracle" | jq .result.ProposeGasPrice

# 2. DECISION MATRIX
# <50 gwei: Normal operations
# 50-100 gwei: Monitor, may increase relay time
# 100-200 gwei: Increase gas limit or pause
# >200 gwei: Pause relayers, wait for drop

# 3. IF PAUSING
# Stop relayers
for host in relayer-{1..3}.etrid.io; do
  ssh $host "pm2 stop relayer-service"
done

# 4. COMMUNICATE
# Discord/Twitter:
"⚠️ Bridge experiencing delays due to high Ethereum gas prices
(currently XXX gwei). Messages will be relayed when gas normalizes.
Your funds are safe. Updates: https://status.etrid.io"

# 5. MONITOR GAS
# Set up watch
watch -n 60 'curl -s "https://api.etherscan.io/api?module=gastracker&action=gasoracle" | jq'

# 6. RESUME WHEN <50 GWEI
pm2 start relayer-service
```

---

## Communication Templates

### Status Page Update Template

```markdown
**Investigating** - [TIME]
We are investigating reports of [issue description].
Updates will be posted as we learn more.

**Identified** - [TIME]
We have identified the issue as [root cause].
Working on a fix. ETA: [time estimate]

**Monitoring** - [TIME]
A fix has been deployed. Monitoring to ensure stability.

**Resolved** - [TIME]
This incident has been resolved. Service is operating normally.
Post-mortem will be published within 48 hours.
```

### Discord Announcement Template

```
🔧 **BRIDGE STATUS UPDATE** 🔧

Issue: [Brief description]
Impact: [What's affected]
Status: [Investigating / Fixing / Resolved]
ETA: [Time estimate or "Unknown"]

Your funds are safe. [If applicable]

We'll update every 30 minutes.

Last updated: [TIME UTC]
```

### Twitter Template

```
We're aware of [issue] affecting the EDSC Bridge.
Team is investigating. Updates: https://status.etrid.io

Your funds remain secure.
```

### Email Template (for affected users)

```
Subject: EDSC Bridge Service Update - [DATE]

Dear EDSC User,

We experienced [brief issue] on [DATE] between [TIME] and [TIME UTC].

What happened:
[2-3 sentence explanation]

Impact:
[What users experienced]

Resolution:
[What we did to fix it]

Your action required:
[None / or specific action]

We apologize for the inconvenience. We're committed to
preventing this from happening again.

For questions, contact support@etrid.io

Best regards,
The Ëtrid Team
```

---

## Post-Incident

### Post-Mortem Template

Create Google Doc using this template:

```markdown
# Incident Post-Mortem: [TITLE]

**Date:** [YYYY-MM-DD]
**Severity:** [P0/P1/P2/P3]
**Duration:** [X hours Y minutes]
**Impact:** [# users, $ value, etc.]
**Author:** [Name]
**Reviewers:** [Names]

## Summary

One paragraph summary of what happened.

## Timeline (all times UTC)

- **HH:MM** - Event occurred
- **HH:MM** - Alert fired
- **HH:MM** - Incident declared
- **HH:MM** - [Key action taken]
- **HH:MM** - Fix deployed
- **HH:MM** - Incident resolved

## Impact

### Users Affected
- X users experienced [impact]

### Financial Impact
- $X in failed transactions
- $X in operational costs
- $X in potential compensation

### Reputation Impact
- X negative tweets
- X Discord complaints

## Root Cause

[Detailed technical explanation of what caused the incident]

## What Went Well

- [Thing 1]
- [Thing 2]

## What Went Wrong

- [Thing 1]
- [Thing 2]

## Action Items

| Action | Owner | Priority | Deadline | Status |
|--------|-------|----------|----------|--------|
| [Action 1] | [Name] | P0 | [Date] | [ ] |
| [Action 2] | [Name] | P1 | [Date] | [ ] |

## Lessons Learned

- [Lesson 1]
- [Lesson 2]

## Appendix

- [Links to logs, screenshots, etc.]
```

### Post-Incident Review Meeting

**Attendees:** Response team + stakeholders
**Timing:** Within 72 hours of resolution
**Duration:** 1 hour

**Agenda:**
1. Review timeline (10 min)
2. Discuss root cause (15 min)
3. What went well / wrong (15 min)
4. Action items (15 min)
5. Documentation review (5 min)

**Outcome:**
- Published post-mortem
- Tracked action items
- Updated runbooks

---

## Appendix

### Useful Commands Quick Reference

```bash
# Service health
for i in {0..4}; do curl -s https://attestation-$i.etrid.io/health | jq; done

# Restart all attesters
for i in {0..4}; do ssh attestation-$i.etrid.io "pm2 restart attestation-service"; done

# Restart all relayers
for i in {1..3}; do ssh relayer-$i.etrid.io "pm2 restart relayer-service"; done

# Check gas price
curl -s "https://api.etherscan.io/api?module=gastracker&action=gasoracle" | jq .result.ProposeGasPrice

# Check ready attestations
curl https://attestation-0.etrid.io/attestations/ready | jq .count

# Backup logs
./scripts/backup-logs.sh

# Emergency pause (relayers)
./scripts/emergency-pause.sh

# Emergency resume
./scripts/emergency-resume.sh
```

### Escalation Matrix

| Incident Type | Initial | If Not Resolved In | Escalate To |
|---------------|---------|-------------------|-------------|
| Security | On-call + Security Lead | 30 min | CTO + CEO |
| Service Down | On-call | 1 hour | Engineering Manager |
| Performance | On-call | 4 hours | Tech Lead |
| Gas Price | On-call | N/A (wait it out) | Communications |

### Required Training

All on-call engineers must complete:
- [ ] Incident response training (4 hours)
- [ ] Bridge architecture overview (2 hours)
- [ ] Security protocols (2 hours)
- [ ] Practice incident drill (1 hour)
- [ ] Post-mortem review (ongoing)

---

**For immediate assistance during an incident:**
- **Slack:** #incident-response
- **Phone:** On-call hotline (PagerDuty)
- **Email:** oncall@etrid.io

**Security incidents:**
- **Email:** security@etrid.io
- **PGP:** [Key fingerprint]
- **Signal:** [Number]

---

## License

Apache-2.0
