# ËTRID ASF Validator Operations Guide

Comprehensive guide for operating an ËTRID ASF validator using the validator tools.

## Table of Contents

1. [Validator Setup](#validator-setup)
2. [Key Management](#key-management)
3. [Staking Operations](#staking-operations)
4. [Monitoring & Maintenance](#monitoring--maintenance)
5. [Health Checks](#health-checks)
6. [Troubleshooting](#troubleshooting)
7. [Best Practices](#best-practices)
8. [Emergency Procedures](#emergency-procedures)

---

## Validator Setup

### Prerequisites

Before setting up a validator, ensure you have:

- ËTRID node binary installed and synced
- Minimum stake requirement: **100,000 ETR**
- Dedicated server with:
  - 4+ CPU cores
  - 16+ GB RAM
  - 500+ GB SSD storage
  - 100+ Mbps network connection
- Validator tools installed (`asf-keygen`, `asf-monitor`, `asf-health`, `asf-stake`)

### Initial Setup Steps

#### 1. Generate Validator Keys

Generate AURA and GRANDPA session keys:

```bash
# Create a secure directory for keys
mkdir -p ~/.etrid/keys
chmod 700 ~/.etrid/keys

# Generate session keys for your validator
asf-keygen generate-session \
  --output-dir ~/.etrid/keys \
  --name my-validator \
  --password "$(cat ~/.etrid/keypass)"

# Backup keys securely (CRITICAL!)
cp -r ~/.etrid/keys ~/.etrid/keys.backup
# Store backup offline or in secure cloud storage
```

**Important:** Save the mnemonic phrases shown during key generation. These are your recovery mechanism.

#### 2. Configure Node

Update your node configuration to use the generated keys:

```bash
# Insert AURA key
etrid-node key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/chainspec.json \
  --keystore-path ~/.etrid/keys \
  --key-type aura \
  --scheme sr25519

# Insert GRANDPA key
etrid-node key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/chainspec.json \
  --keystore-path ~/.etrid/keys \
  --key-type gran \
  --scheme ed25519
```

#### 3. Start Validator Node

```bash
# Start the validator node
systemctl start etrid-validator

# Verify it's running
systemctl status etrid-validator

# Check logs
journalctl -u etrid-validator -f
```

#### 4. Bond Initial Stake

```bash
# Bond your initial stake (e.g., 100,000 ETR)
asf-stake bond \
  --amount 100000 \
  --validator $(cat ~/.etrid/validator-address) \
  --keyfile ~/.etrid/keys/controller.key \
  --rpc http://localhost:9944

# Verify staking status
asf-stake status \
  --validator $(cat ~/.etrid/validator-address) \
  --detailed \
  --rpc http://localhost:9944
```

---

## Key Management

### Key Types

ËTRID ASF validators use multiple key types:

1. **AURA Key (sr25519)** - For block production
2. **GRANDPA Key (ed25519)** - For finality voting
3. **Controller Key (sr25519)** - For staking operations
4. **Stash Key (sr25519)** - For holding staked funds

### Key Generation Best Practices

```bash
# Generate with strong password
asf-keygen generate \
  --scheme sr25519 \
  --output controller.key \
  --key-type controller

# Verify key was created correctly
asf-keygen inspect --keyfile controller.key

# Export public key for sharing
asf-keygen export \
  --keyfile controller.key \
  --format json \
  --output controller-public.json
```

### Key Rotation

Rotate session keys periodically (recommended every 3-6 months):

```bash
# 1. Generate new session keys
asf-keygen generate-session \
  --output-dir ~/.etrid/keys-new \
  --name my-validator-v2

# 2. Verify new keys
asf-keygen inspect --keyfile ~/.etrid/keys-new/my-validator-v2-aura.key
asf-keygen inspect --keyfile ~/.etrid/keys-new/my-validator-v2-grandpa.key

# 3. Submit new session keys on-chain
# (This would use a runtime call - implementation depends on pallet)

# 4. Wait for next session/epoch

# 5. Update node configuration with new keys

# 6. Restart validator
systemctl restart etrid-validator

# 7. Monitor to ensure everything works
asf-monitor --validator $(cat ~/.etrid/validator-address)
```

### Key Backup & Recovery

```bash
# Backup procedure (run regularly)
#!/bin/bash
BACKUP_DIR="/secure/backup/$(date +%Y%m%d)"
mkdir -p "$BACKUP_DIR"

# Backup keys
cp -r ~/.etrid/keys "$BACKUP_DIR/"

# Backup validator address
cp ~/.etrid/validator-address "$BACKUP_DIR/"

# Create encrypted archive
tar -czf - "$BACKUP_DIR" | \
  gpg --symmetric --cipher-algo AES256 > \
  "$BACKUP_DIR.tar.gz.gpg"

# Upload to secure location
# aws s3 cp "$BACKUP_DIR.tar.gz.gpg" s3://my-backup-bucket/

echo "Backup complete: $BACKUP_DIR.tar.gz.gpg"
```

---

## Staking Operations

### Bonding Additional Stake

Increase your stake to improve validator ranking:

```bash
# Bond additional 50,000 ETR
asf-stake bond \
  --amount 50000 \
  --validator $(cat ~/.etrid/validator-address) \
  --keyfile ~/.etrid/keys/controller.key \
  --rpc http://localhost:9944

# Verify new total stake
asf-stake status --validator $(cat ~/.etrid/validator-address) --detailed
```

### Unbonding Stake

Unbond tokens when reducing stake or exiting:

```bash
# Unbond 25,000 ETR
asf-stake unbond \
  --amount 25000 \
  --validator $(cat ~/.etrid/validator-address) \
  --keyfile ~/.etrid/keys/controller.key \
  --rpc http://localhost:9944

# Note: Unbonding period is 7 days (168 hours)
# Tokens will be available after this period
```

**Important:** Maintain minimum stake (100,000 ETR) to remain in validator set.

### Claiming Rewards

Claim accumulated validator rewards:

```bash
# View pending rewards
asf-stake rewards \
  --validator $(cat ~/.etrid/validator-address) \
  --epochs 30

# Claim all pending rewards
asf-stake claim \
  --validator $(cat ~/.etrid/validator-address) \
  --keyfile ~/.etrid/keys/controller.key \
  --rpc http://localhost:9944
```

**Best Practice:** Claim rewards regularly (weekly) and re-stake to compound returns.

### Monitoring Delegations

If your validator accepts delegations:

```bash
# Check total delegated stake
asf-stake status --validator $(cat ~/.etrid/validator-address) --detailed

# This shows:
# - Self stake
# - Delegated stake
# - Number of delegators
# - Commission rate
```

---

## Monitoring & Maintenance

### Real-Time Dashboard

Use `asf-monitor` for continuous monitoring:

```bash
# Start the monitoring dashboard
asf-monitor \
  --rpc http://localhost:9944 \
  --validator $(cat ~/.etrid/validator-address) \
  --interval 2

# The dashboard shows:
# - Network status (chain, block, peers)
# - Validator status (active, committee, votes)
# - Performance (uptime, blocks signed, health)
# - Real-time activity log
```

**Tip:** Run in a `tmux` or `screen` session for persistent monitoring.

### Automated Monitoring Script

Create a monitoring script for alerts:

```bash
#!/bin/bash
# /usr/local/bin/validator-monitor.sh

VALIDATOR=$(cat ~/.etrid/validator-address)
RPC="http://localhost:9944"
ALERT_EMAIL="ops@example.com"
LOG_FILE="/var/log/etrid/validator-health.log"

# Run health check
asf-health \
  --rpc "$RPC" \
  --validator "$VALIDATOR" \
  --format json \
  --output /tmp/health-check.json \
  --exit-on-failure

if [ $? -ne 0 ]; then
  # Health check failed - send alert
  echo "$(date): Health check FAILED" >> "$LOG_FILE"
  cat /tmp/health-check.json | \
    mail -s "ALERT: Validator Health Check Failed" "$ALERT_EMAIL"
else
  echo "$(date): Health check PASSED" >> "$LOG_FILE"
fi

# Check uptime
UPTIME=$(jq -r '.checks[] | select(.name=="Performance") | .details.uptime_percentage' /tmp/health-check.json)

if [ "$UPTIME" -lt 95 ]; then
  echo "$(date): Low uptime detected: $UPTIME%" >> "$LOG_FILE"
  echo "Validator uptime is $UPTIME%" | \
    mail -s "WARNING: Low Validator Uptime" "$ALERT_EMAIL"
fi
```

Add to crontab:

```bash
# Run every 5 minutes
*/5 * * * * /usr/local/bin/validator-monitor.sh
```

### Performance Metrics

Track key performance indicators:

```bash
# Export current metrics
asf-health \
  --rpc http://localhost:9944 \
  --validator $(cat ~/.etrid/validator-address) \
  --format json \
  --output "metrics-$(date +%Y%m%d-%H%M%S).json"

# Parse specific metrics
cat metrics-*.json | jq '.checks[] | select(.name=="Performance") | .details'
```

---

## Health Checks

### Daily Health Check Routine

Run comprehensive health checks daily:

```bash
#!/bin/bash
# daily-health-check.sh

echo "=== ËTRID Validator Health Check ==="
echo "Date: $(date)"
echo ""

# 1. Node connectivity
echo "1. Checking node connectivity..."
asf-health --rpc http://localhost:9944 --validator $(cat ~/.etrid/validator-address) --verbose

# 2. Staking status
echo ""
echo "2. Checking staking status..."
asf-stake status --validator $(cat ~/.etrid/validator-address) --detailed

# 3. System resources
echo ""
echo "3. System resources:"
free -h
df -h
uptime

# 4. Peer count
echo ""
echo "4. Network peers:"
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944 | jq

echo ""
echo "=== Health Check Complete ==="
```

### Critical Health Indicators

Monitor these critical metrics:

| Metric | Healthy | Warning | Critical |
|--------|---------|---------|----------|
| Uptime | ≥ 99% | 95-99% | < 95% |
| Peer Count | ≥ 20 | 10-19 | < 10 |
| Health Score | ≥ 90 | 70-89 | < 70 |
| Blocks Signed | ≥ 95% | 80-94% | < 80% |
| Sync Status | Synced | 1-10 blocks behind | > 10 blocks behind |

---

## Troubleshooting

### Validator Not in Committee

**Symptoms:** Validator is active but not selected for committee.

**Diagnosis:**
```bash
# Check stake ranking
asf-stake list --sort stake --limit 30

# Check reputation
asf-stake status --validator $(cat ~/.etrid/validator-address) --detailed
```

**Solutions:**
1. Increase stake to be in top 21
2. Improve reputation by maintaining high uptime
3. Ensure no recent slashing events

### Low Uptime

**Symptoms:** Health score below 95%.

**Diagnosis:**
```bash
# Check system resources
asf-health --rpc http://localhost:9944 --validator $(cat ~/.etrid/validator-address) --verbose

# Check node logs
journalctl -u etrid-validator -n 1000 | grep -i error
```

**Solutions:**
1. Upgrade server resources if needed
2. Check network connectivity
3. Verify no disk space issues
4. Restart validator if necessary

### Missed Blocks

**Symptoms:** Blocks signed count is low.

**Diagnosis:**
```bash
# Check if in committee
asf-stake status --validator $(cat ~/.etrid/validator-address) --detailed

# Monitor real-time
asf-monitor --validator $(cat ~/.etrid/validator-address)
```

**Solutions:**
1. Verify session keys are correctly inserted
2. Check node is fully synced
3. Ensure sufficient peer connections
4. Verify no clock drift (use NTP)

### Connection Issues

**Symptoms:** Cannot connect to RPC.

**Diagnosis:**
```bash
# Test RPC endpoint
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944

# Check if node is running
systemctl status etrid-validator

# Check firewall
sudo ufw status
```

**Solutions:**
1. Restart node: `systemctl restart etrid-validator`
2. Check RPC is enabled in config
3. Verify firewall allows port 9944
4. Check node logs for errors

---

## Best Practices

### Security

1. **Use dedicated server** - Don't run validator on shared infrastructure
2. **Enable firewall** - Only expose necessary ports
3. **Regular updates** - Keep node software updated
4. **Key security** - Encrypt key files, store backups offline
5. **Monitoring** - Set up alerts for critical events
6. **2FA** - Use 2FA for any remote access

### Operations

1. **Regular maintenance windows** - Schedule weekly maintenance
2. **Backup strategy** - Daily automated backups of keys and config
3. **Documentation** - Document your setup and procedures
4. **Redundancy** - Have backup server ready for failover
5. **Monitoring** - Use multiple monitoring solutions
6. **Alerts** - Configure alerts for critical metrics

### Performance

1. **Resource allocation** - Ensure adequate CPU, RAM, and disk
2. **Network** - Use high-bandwidth, low-latency connection
3. **SSD storage** - Use NVMe SSD for database
4. **Pruning** - Configure appropriate database pruning
5. **Peer connections** - Maintain 20+ quality peers

---

## Emergency Procedures

### Validator Offline

If validator goes offline unexpectedly:

```bash
# 1. Check node status
systemctl status etrid-validator

# 2. Check logs for errors
journalctl -u etrid-validator -n 100

# 3. Try restart
systemctl restart etrid-validator

# 4. If restart fails, check disk space
df -h

# 5. Check system resources
top
free -h

# 6. Restore from backup if necessary
```

### Lost Keys

If validator keys are compromised or lost:

```bash
# 1. Immediately stop validator
systemctl stop etrid-validator

# 2. Restore from backup
tar -xzf keys-backup.tar.gz -C ~/.etrid/

# 3. Verify restored keys
asf-keygen inspect --keyfile ~/.etrid/keys/my-validator-aura.key

# 4. Restart with restored keys
systemctl start etrid-validator

# 5. Monitor recovery
asf-monitor --validator $(cat ~/.etrid/validator-address)
```

### Slashing Event

If validator is slashed:

```bash
# 1. Check slashing status
asf-health --rpc http://localhost:9944 --validator $(cat ~/.etrid/validator-address)

# 2. Review what caused slashing
# (Check node logs, network events)

# 3. Fix the underlying issue

# 4. Wait for unbonding period to clear slashing

# 5. Re-bond stake if needed
asf-stake bond --amount 100000 --validator $(cat ~/.etrid/validator-address) --keyfile ~/.etrid/keys/controller.key
```

### Server Migration

To migrate validator to new server:

```bash
# On old server:
# 1. Backup everything
tar -czf validator-backup.tar.gz ~/.etrid /var/lib/etrid

# 2. Transfer to new server
scp validator-backup.tar.gz new-server:/tmp/

# On new server:
# 3. Install node software
# 4. Restore backup
tar -xzf /tmp/validator-backup.tar.gz -C /

# 5. Update configuration if needed
# 6. Stop old validator
# 7. Start new validator immediately
systemctl start etrid-validator

# 8. Monitor for successful takeover
asf-monitor --validator $(cat ~/.etrid/validator-address)
```

---

## Appendix

### Useful Commands Reference

```bash
# Quick status check
asf-health --rpc http://localhost:9944 --validator <address>

# Live monitoring
asf-monitor --rpc http://localhost:9944 --validator <address>

# Check rewards
asf-stake rewards --validator <address> --epochs 10

# Bond more stake
asf-stake bond --amount 10000 --validator <address> --keyfile controller.key

# Generate new key
asf-keygen generate --scheme sr25519 --output new-key.key

# List top validators
asf-stake list --sort stake --limit 21
```

### Performance Tuning

```bash
# Node configuration optimizations
# Add to node startup flags:
--pruning=archive          # For archive nodes
--pruning=1000             # Keep last 1000 blocks
--wasm-execution=compiled  # Faster execution
--execution=native         # Use native runtime
--db-cache=4096           # Increase DB cache
```

### Contact & Support

- Technical Support: support@etrid.network
- Validator Discord: https://discord.gg/etrid-validators
- Documentation: https://docs.etrid.network/validators

---

**Remember:** Operating a validator is a responsibility. Maintain high uptime, secure your keys, and stay engaged with the community!
