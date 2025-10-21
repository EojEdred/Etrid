# EDSC Bridge Operations Guide

Operational runbook for managing EDSC bridge services in production.

## Table of Contents

- [Overview](#overview)
- [Service Architecture](#service-architecture)
- [Daily Operations](#daily-operations)
- [Monitoring & Alerts](#monitoring--alerts)
- [Common Issues & Solutions](#common-issues--solutions)
- [Incident Response](#incident-response)
- [Maintenance Procedures](#maintenance-procedures)
- [Emergency Procedures](#emergency-procedures)
- [On-Call Runbook](#on-call-runbook)

---

## Overview

The EDSC bridge consists of:
- **5 Attestation Services** (M-of-N signature providers)
- **2-3 Relayer Services** (permissionless message relayers)
- **Ethereum Contracts** (on Sepolia testnet)
- **Substrate Chain** (Ember Testnet)

**Operational Goal**: 99.9% uptime with <5 minute message relay times

---

## Service Architecture

### Attestation Service Topology

```
┌─────────────────────────────────────────────────────────────┐
│                    Attestation Services                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │ Attester │  │ Attester │  │ Attester │  │ Attester │... │
│  │    0     │  │    1     │  │    2     │  │    3     │    │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘    │
│       │             │             │             │           │
│       └─────────────┴─────────────┴─────────────┘           │
│                           │                                  │
│                      Attestation                            │
│                        Store                                │
│                    (3-of-5 threshold)                       │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │   Relayer Services     │
              │   (permissionless)     │
              └───────────┬───────────┘
                          │
         ┌────────────────┴────────────────┐
         ▼                                 ▼
    ┌─────────┐                       ┌─────────┐
    │Ethereum │                       │ Ëtrid   │
    │Contracts│                       │ Chain   │
    └─────────┘                       └─────────┘
```

### Service Dependencies

**Attestation Service depends on:**
- Ethereum RPC node (Alchemy/Infura)
- Substrate RPC node (wss://ember-rpc.etrid.io)
- PostgreSQL (optional, for persistent storage)

**Relayer Service depends on:**
- All 5 attestation service APIs
- Ethereum RPC node
- Substrate RPC node
- Funded accounts (ETH + EDSC)

---

## Daily Operations

### Morning Checklist

```bash
# 1. Check service health
curl https://attestation-0.etrid.io/health | jq
curl https://attestation-1.etrid.io/health | jq
curl https://attestation-2.etrid.io/health | jq
curl https://attestation-3.etrid.io/health | jq
curl https://attestation-4.etrid.io/health | jq

# 2. Check attestation status
curl https://attestation-0.etrid.io/stats | jq

# 3. Check relayer processes
pm2 status

# 4. Check relayer balances
# Ethereum (need at least 0.1 ETH)
curl -X POST https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["RELAYER_ADDRESS","latest"],"id":1}' | jq

# Substrate (need at least 10 EDSC)
# Use substrate helper or polkadot.js app

# 5. Check for stuck messages
curl https://attestation-0.etrid.io/attestations/ready | jq '.count'
# If count > 10 and not decreasing, investigate relayers

# 6. Review error logs
pm2 logs --lines 100 --nostream | grep -i error
journalctl -u attestation-service --since "1 hour ago" | grep -i error

# 7. Check Ethereum gas prices
# If >100 gwei, consider pausing relayers temporarily
```

### Key Metrics to Monitor

| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| Attestation service uptime | 99.9% | <99% |
| Relayer service uptime | 99% | <95% |
| Message relay time (avg) | <3 min | >5 min |
| Attestation threshold time | <90 sec | >120 sec |
| Relayer balance (ETH) | >0.5 ETH | <0.1 ETH |
| Relayer balance (EDSC) | >100 EDSC | <10 EDSC |
| Failed relay attempts | <1% | >5% |
| Ethereum gas price | <50 gwei | >100 gwei |

---

## Monitoring & Alerts

### Prometheus Metrics

Attestation services expose metrics at `/metrics`:

```
# Service health
attestation_service_up 1

# Message processing
attestation_messages_seen_total{source_domain="0"} 142
attestation_messages_seen_total{source_domain="2"} 138
attestation_signatures_created_total 280
attestation_threshold_reached_total 140

# API requests
attestation_api_requests_total{endpoint="/health",status="200"} 5234
attestation_api_requests_duration_seconds{endpoint="/attestation"} 0.045

# Chain monitoring
attestation_chain_block_height{chain="ethereum"} 4567890
attestation_chain_block_height{chain="substrate"} 123456
attestation_chain_connection_status{chain="ethereum"} 1
```

Relayer services expose:

```
# Relay operations
relayer_messages_relayed_total{destination="ethereum"} 68
relayer_messages_relayed_total{destination="substrate"} 72
relayer_relay_failures_total{destination="ethereum",reason="gas_too_high"} 3
relayer_relay_duration_seconds{destination="ethereum"} 12.4

# Balances
relayer_balance_eth 0.4523
relayer_balance_edsc 45.23

# Fetcher
relayer_attestations_fetched_total 140
relayer_attestations_ready_current 0
```

### Alert Rules (Prometheus)

Create `alerts.yml`:

```yaml
groups:
  - name: edsc_bridge
    interval: 30s
    rules:
      # Service down
      - alert: AttestationServiceDown
        expr: attestation_service_up == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Attestation service {{ $labels.instance }} is down"

      - alert: RelayerServiceDown
        expr: relayer_service_up == 0
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "Relayer service {{ $labels.instance }} is down"

      # Threshold not met
      - alert: AttestationThresholdNotMet
        expr: attestation_threshold_reached_total < 3
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Attestation threshold not being met (need 3-of-5)"

      # Slow relay
      - alert: MessageRelayingSlow
        expr: relayer_relay_duration_seconds > 300
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Message relaying taking >5 minutes"

      # Low balances
      - alert: RelayerLowEthBalance
        expr: relayer_balance_eth < 0.1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Relayer ETH balance below 0.1 ETH"

      - alert: RelayerLowEdscBalance
        expr: relayer_balance_edsc < 10
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Relayer EDSC balance below 10 EDSC"

      # High failure rate
      - alert: HighRelayFailureRate
        expr: rate(relayer_relay_failures_total[5m]) > 0.05
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Relay failure rate >5%"

      # Chain connection issues
      - alert: ChainConnectionLost
        expr: attestation_chain_connection_status == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "{{ $labels.chain }} chain connection lost"

      # Stuck messages
      - alert: MessagesStuck
        expr: relayer_attestations_ready_current > 10
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "{{ $value }} messages stuck in ready state"
```

### Grafana Dashboards

Import dashboard JSON (see monitoring/grafana-dashboard.json)

**Key Panels:**
1. Service uptime (gauge)
2. Messages relayed per hour (graph)
3. Average relay time (graph)
4. Relayer balances (gauge)
5. Attestation threshold status (stat)
6. Error rate (graph)
7. Chain block heights (stat)

---

## Common Issues & Solutions

### Issue: Attestation Service Not Signing

**Symptoms:**
- `/stats` shows `signatures_created: 0` or very low
- Logs show "Error signing message"

**Diagnosis:**
```bash
# Check service logs
pm2 logs attestation-service-0 --lines 50

# Common errors:
# - "Private key invalid" → Check ATTESTER_PRIVATE_KEY
# - "Cannot connect to Substrate" → Check SUBSTRATE_WS_URL
# - "Transaction failed" → Check gas/fees
```

**Solution:**
```bash
# 1. Verify configuration
cat /etc/attestation-service/.env | grep ATTESTER_PRIVATE_KEY

# 2. Test RPC connections
curl https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY
wscat -c wss://ember-rpc.etrid.io

# 3. Restart service
pm2 restart attestation-service-0

# 4. Monitor logs
pm2 logs attestation-service-0
```

---

### Issue: Relayer Not Relaying

**Symptoms:**
- `relayer_attestations_ready_current` increasing
- Logs show "Skipping relay"

**Diagnosis:**
```bash
# Check relayer logs
pm2 logs relayer-service --lines 50

# Common causes:
# - Low balance → "Insufficient funds"
# - High gas → "Gas price too high"
# - Already relayed → "Message already received"
# - Invalid attestation → "Signature verification failed"
```

**Solution:**
```bash
# 1. Check balances
# If low ETH:
# Send 0.5 ETH to relayer address

# If low EDSC:
# Send 100 EDSC to relayer address

# 2. Check gas settings
cat /etc/relayer-service/.env | grep GAS

# If gas too high, adjust MAX_FEE_PER_GAS:
# Edit .env: MAX_FEE_PER_GAS=100  # Increase limit
pm2 restart relayer-service

# 3. Check attestation validity
curl https://attestation-0.etrid.io/attestation/MESSAGE_HASH | jq

# Should have 3+ signatures with status "ready"
```

---

### Issue: Messages Stuck in "Pending"

**Symptoms:**
- `/attestations/ready` shows count = 0
- Individual attestations have <3 signatures
- Time since burn >5 minutes

**Diagnosis:**
```bash
# Check how many attesters are signing
curl https://attestation-0.etrid.io/attestation/SOURCE_DOMAIN/NONCE | jq '.signatures | length'

# Should be 3-5
# If 0-2, attesters aren't signing
```

**Solution:**
```bash
# 1. Check all attester health
for i in {0..4}; do
  echo "Attester $i:"
  curl -s https://attestation-$i.etrid.io/health | jq '.status'
done

# 2. Check attester logs for errors
ssh attester-0.etrid.io "pm2 logs attestation-service --lines 20"

# 3. Restart unhealthy attesters
ssh attester-0.etrid.io "pm2 restart attestation-service"

# 4. Verify they start signing
# Wait 1 minute, then check signature count again
```

---

### Issue: High Gas Prices

**Symptoms:**
- Relayers paused or failing
- Logs show "Gas price exceeds limit"

**Diagnosis:**
```bash
# Check current gas price
curl https://api.etherscan.io/api?module=gastracker&action=gasoracle | jq '.result.ProposeGasPrice'

# If >100 gwei, consider pausing
```

**Solution:**
```bash
# Option 1: Increase gas limit (temporary)
# Edit relayer .env:
MAX_FEE_PER_GAS=150  # Or higher
pm2 restart relayer-service

# Option 2: Pause relaying until gas drops
pm2 stop relayer-service

# Monitor gas prices
watch -n 60 'curl -s https://api.etherscan.io/api?module=gastracker&action=gasoracle | jq'

# When <50 gwei:
pm2 start relayer-service

# Option 3: Community announcement
# Post on Discord: "Bridge experiencing delays due to high Ethereum gas prices"
```

---

### Issue: Chain RPC Down

**Symptoms:**
- Logs show "WebSocket connection failed"
- `chain_connection_status` = 0

**Diagnosis:**
```bash
# Test Ethereum RPC
curl -X POST https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Test Substrate RPC
wscat -c wss://ember-rpc.etrid.io
> {"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}
```

**Solution:**
```bash
# If Ethereum RPC down:
# 1. Switch to backup RPC in .env
ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/BACKUP-KEY
pm2 restart attestation-service
pm2 restart relayer-service

# If Substrate RPC down:
# 1. Check substrate node status
ssh substrate-rpc.etrid.io "systemctl status edsc-pbc-node"

# 2. Restart if needed
ssh substrate-rpc.etrid.io "systemctl restart edsc-pbc-node"

# 3. If node won't start, check logs
ssh substrate-rpc.etrid.io "journalctl -u edsc-pbc-node -n 100"

# Common issue: Database corruption
# Solution: Restart with --unsafe-pruning (TESTNET ONLY)
```

---

## Incident Response

### Severity Levels

**P0 - Critical** (Response: Immediate)
- Bridge completely down (no messages relaying)
- Security breach or exploit
- Data loss or corruption
- Multiple attesters offline (threshold not met)

**P1 - High** (Response: <30 min)
- Single attester offline
- Relayer balance critically low
- Messages delayed >10 minutes
- Elevated error rate (>10%)

**P2 - Medium** (Response: <2 hours)
- Single service degraded
- Messages delayed 5-10 minutes
- Non-critical errors increasing

**P3 - Low** (Response: <24 hours)
- Minor issues
- Performance degradation
- Documentation updates needed

### Incident Response Process

```
1. DETECT
   ├─ Alert fires → PagerDuty → On-call engineer
   ├─ User report → Discord/Twitter → Team
   └─ Monitoring → Grafana → Dashboard

2. ASSESS
   ├─ Check service health endpoints
   ├─ Review recent logs
   ├─ Identify scope (single service vs. systemic)
   └─ Determine severity (P0-P3)

3. RESPOND
   ├─ P0: Page all hands, start incident channel
   ├─ P1: Page on-call, notify team
   ├─ P2/P3: Create ticket, handle during business hours
   └─ Post status update (Discord/Twitter)

4. MITIGATE
   ├─ Apply immediate fix (restart, config change)
   ├─ Monitor for improvement
   └─ Escalate if not resolved

5. RESOLVE
   ├─ Verify fix working
   ├─ Monitor for 30 minutes
   ├─ Post resolution update
   └─ Schedule postmortem (P0/P1)

6. POSTMORTEM (within 48 hours for P0/P1)
   ├─ What happened?
   ├─ What was the impact?
   ├─ What was the root cause?
   ├─ How was it resolved?
   ├─ What can we do to prevent this?
   └─ Action items with owners
```

---

## Maintenance Procedures

### Updating Attestation Service

```bash
# 1. Test update on one attester first
ssh attestation-0.etrid.io

# 2. Pull latest code
cd /opt/attestation-service
git pull origin main

# 3. Install dependencies
npm install

# 4. Build
npm run build

# 5. Run tests
npm test

# 6. Restart service
pm2 restart attestation-service

# 7. Monitor logs for 5 minutes
pm2 logs attestation-service --lines 50

# 8. Verify health
curl http://localhost:3000/health | jq

# 9. If successful, roll out to other attesters
# Wait 30 minutes between each to ensure stability
for i in {1..4}; do
  ssh attestation-$i.etrid.io "./deploy.sh"
  sleep 1800  # 30 minute soak
done
```

### Rotating Attester Keys

**WARNING:** This requires coordinating contract updates and service restarts.

```bash
# 1. Generate new key pair
node -e "console.log(require('ethers').Wallet.createRandom().privateKey)"
# Save: NEW_PRIVATE_KEY=0x...

# 2. Get address
node -e "console.log(new (require('ethers').Wallet)('NEW_PRIVATE_KEY').address)"
# Save: NEW_ADDRESS=0x...

# 3. Update contract (owner only)
# Call AttesterRegistry.updateAttester(OLD_ADDRESS, NEW_ADDRESS)

# 4. Wait for confirmation

# 5. Update service config
ssh attestation-0.etrid.io
vim /etc/attestation-service/.env
# Update: ATTESTER_PRIVATE_KEY=NEW_PRIVATE_KEY

# 6. Restart service
pm2 restart attestation-service

# 7. Verify signing with new key
curl http://localhost:3000/stats | jq

# 8. Update monitoring (if using key-specific alerts)
```

### Database Backup (if using PostgreSQL)

```bash
# Daily backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/var/backups/attestation-db"

# Create backup
pg_dump -U attestation -d attestation_db | gzip > "$BACKUP_DIR/backup_$DATE.sql.gz"

# Keep only last 30 days
find "$BACKUP_DIR" -name "backup_*.sql.gz" -mtime +30 -delete

# Upload to S3 (optional)
aws s3 cp "$BACKUP_DIR/backup_$DATE.sql.gz" s3://etrid-backups/attestation-db/
```

### Chain Spec Update

When updating substrate chain spec:

```bash
# 1. Build new runtime
cargo build --release -p edsc-pbc-runtime

# 2. Test on local chain first
./target/release/edsc-pbc-node --dev --tmp

# 3. Coordinate validator upgrade
# All validators must upgrade within upgrade window

# 4. Notify services to expect chain downtime
# Post: "Scheduled maintenance: 30 minute window starting HH:MM UTC"

# 5. Perform rolling validator restart
# Start with backup validator, end with primary

# 6. Monitor block production
# Should resume within 1-2 minutes

# 7. Verify services reconnect
curl https://attestation-0.etrid.io/health | jq
```

---

## Emergency Procedures

### Emergency: Bridge Exploit Detected

```bash
# IMMEDIATE ACTIONS (within 5 minutes):

# 1. PAUSE ALL RELAYERS
for host in relayer-{1..3}.etrid.io; do
  ssh $host "pm2 stop relayer-service"
done

# 2. POST PUBLIC NOTICE
# Discord: "@everyone Bridge paused due to security investigation. Funds are safe."
# Twitter: "EDSC Bridge temporarily paused for security review. Updates soon."

# 3. NOTIFY CORE TEAM
# Page: CTO, Security Lead, DevOps Lead

# 4. ASSESS DAMAGE
# - Check for unusual message patterns
# - Review recent contract events
# - Check for balance discrepancies

# 5. SECURE EVIDENCE
# - Snapshot all logs
# - Export relevant transactions
# - Document timeline

# 6. DETERMINE ROOT CAUSE
# - Code review
# - Log analysis
# - External security audit (if needed)

# 7. IMPLEMENT FIX
# - Patch vulnerability
# - Test thoroughly
# - Deploy to testnet

# 8. RESTART BRIDGE (after full verification)
# - Resume relayers one at a time
# - Monitor closely for 24 hours
# - Post incident report within 72 hours
```

### Emergency: Attester Private Key Compromised

```bash
# IMMEDIATE:
# 1. Disable compromised attester in contract
# Call: AttesterRegistry.disableAttester(COMPROMISED_ADDRESS)

# 2. Verify threshold still met (need 3 active)
# If not, emergency deploy new attester

# 3. Generate new key
NEW_KEY=$(node -e "console.log(require('ethers').Wallet.createRandom().privateKey)")

# 4. Add new attester
# Call: AttesterRegistry.addAttester(NEW_ADDRESS)

# 5. Update service config
ssh attestation-X.etrid.io
vim /etc/attestation-service/.env
# Update ATTESTER_PRIVATE_KEY

# 6. Restart service
pm2 restart attestation-service

# 7. Post incident report
# Document: How was key compromised? How to prevent?
```

### Emergency: Chain Halt / No Blocks

```bash
# Substrate chain stopped producing blocks

# 1. Check validator status
for i in {1..3}; do
  ssh validator-$i.etrid.io "systemctl status edsc-pbc-node"
done

# 2. Check logs
ssh validator-1.etrid.io "journalctl -u edsc-pbc-node -n 100"

# 3. Common causes:
# - Consensus failure (validators offline)
# - Database corruption
# - Network partition
# - Runtime panic

# 4. If >50% validators down:
# Contact validators to restart

# 5. If runtime panic:
# May need to revert to previous runtime version
# This requires coordinated effort

# 6. If database corruption:
# Restore from backup or resync from peers

# 7. Post updates every 30 minutes
# "Block production paused. Investigating. ETA: XX:XX UTC"
```

---

## On-Call Runbook

### On-Call Responsibilities

- Monitor alerts (PagerDuty)
- Respond to incidents per SLA
- Escalate if needed
- Document all actions
- Handoff status to next shift

### On-Call Shift Checklist

**Start of Shift:**
```bash
# 1. Test alert routing
# Trigger test alert, verify you receive it

# 2. Review current status
curl https://status.etrid.io/api/status

# 3. Check for ongoing issues
# Review Slack #incidents channel

# 4. Review schedule
# Note any planned maintenance

# 5. Verify access
# SSH to all servers
# AWS/GCP console access
# Grafana access
# PagerDuty access
```

**During Shift:**
```bash
# Every 4 hours: Quick health check
./scripts/health-check.sh | tee -a logs/oncall.log

# If alert fires:
# 1. Acknowledge in PagerDuty
# 2. Assess severity (see Incident Response)
# 3. Follow runbook for specific alert
# 4. Document actions in incident log
# 5. Update status page if user-facing
# 6. Resolve alert when fixed
```

**End of Shift:**
```bash
# 1. Document shift summary
# - Incidents handled
# - Ongoing issues
# - Scheduled maintenance

# 2. Handoff to next on-call
# Post in #oncall Slack channel

# 3. Update incident tickets

# 4. File any needed follow-up tasks
```

### Contact Escalation

```
Level 1: On-call Engineer (you)
  ↓ (if unresolved in 30 min for P1, immediately for P0)
Level 2: Senior DevOps Engineer
  ↓ (if unresolved in 1 hour)
Level 3: Engineering Manager
  ↓ (if critical/security issue)
Level 4: CTO + Security Lead
```

### Quick Reference Commands

```bash
# Service health
alias health='curl -s https://attestation-0.etrid.io/health | jq'

# All services
alias health-all='for i in {0..4}; do echo "Attester $i:"; curl -s https://attestation-$i.etrid.io/health | jq .status; done'

# Restart all
alias restart-all='pm2 restart all'

# Check balances
alias check-balances='./scripts/check-relayer-balances.sh'

# Recent errors
alias errors='pm2 logs --lines 100 --nostream | grep -i error'

# Gas price
alias gas='curl -s "https://api.etherscan.io/api?module=gastracker&action=gasoracle" | jq .result.ProposeGasPrice'
```

---

## Appendix

### Service Configuration Summary

| Service | Port | Path | Logs |
|---------|------|------|------|
| Attestation 0 | 3000 | /opt/attestation-service | pm2 logs attestation-service-0 |
| Attestation 1 | 3000 | /opt/attestation-service | pm2 logs attestation-service-1 |
| ... | ... | ... | ... |
| Relayer 1 | N/A | /opt/relayer-service | pm2 logs relayer-service-1 |
| Substrate RPC | 9944 | /var/lib/edsc | journalctl -u edsc-pbc-node |
| Nginx (SSL) | 443 | /etc/nginx | /var/log/nginx/error.log |

### Useful Scripts

Located in `scripts/operations/`:

- `health-check.sh` - Check all service health
- `check-balances.sh` - Check relayer balances
- `restart-attesters.sh` - Rolling restart of attesters
- `emergency-pause.sh` - Pause all relayers
- `emergency-resume.sh` - Resume all relayers
- `backup-logs.sh` - Archive logs to S3
- `test-e2e.sh` - Run quick E2E test

---

## License

Apache-2.0

---

**For security incidents, contact: security@etrid.io**

**For operational support, contact: ops@etrid.io**
