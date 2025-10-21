# Operational Utility Scripts

Command-line utilities for managing and monitoring the EDSC bridge in production.

## Scripts

### health-check.sh

Comprehensive health check of all bridge services.

**Usage:**
```bash
./scripts/operations/health-check.sh
```

**What it checks:**
- All 5 attestation services
- All relayer services
- Ethereum RPC connectivity
- Substrate RPC connectivity
- Ready attestation queue size

**Exit codes:**
- 0: All systems operational
- 1: Issues detected

**Environment variables:**
```bash
export ATTESTATION_SERVICES="attestation-0.etrid.io attestation-1.etrid.io ..."
export RELAYER_SERVICES="relayer-1.etrid.io relayer-2.etrid.io"
export ETHEREUM_RPC="https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY"
export SUBSTRATE_RPC="wss://ember-rpc.etrid.io"
```

---

### check-balances.sh

Check ETH and EDSC balances of relayer accounts.

**Usage:**
```bash
export RELAYER_ADDRESSES="0x123...,0x456..."
./scripts/operations/check-balances.sh
```

**Alerts if:**
- ETH < 0.1 (warning)
- ETH < 0.05 (critical)

**Required:**
- `RELAYER_ADDRESSES`: Comma-separated Ethereum addresses
- `ETHEREUM_RPC`: Ethereum RPC endpoint

---

### emergency-pause.sh

**⚠️ EMERGENCY USE ONLY**

Immediately stops all relayer services. Use in case of:
- Security incident
- Bridge exploit detected
- Critical bug discovered

**Usage:**
```bash
./scripts/operations/emergency-pause.sh
```

**What it does:**
- Stops all relayer services via PM2
- Logs action to `/tmp/bridge-emergency.log`
- Requires confirmation

**Environment variables:**
```bash
export RELAYER_HOSTS="relayer-1.etrid.io relayer-2.etrid.io relayer-3.etrid.io"
export SSH_USER="ubuntu"
```

**After pausing:**
1. Investigate issue
2. Post status update
3. Resume with `emergency-resume.sh` when safe

---

### emergency-resume.sh

Resume all relayer services after emergency pause.

**Usage:**
```bash
./scripts/operations/emergency-resume.sh
```

**What it does:**
- Restarts all relayer services
- Gradual rollout (5s between each)
- Requires confirmation that issue is resolved

**After resuming:**
1. Monitor logs for 30 minutes
2. Verify messages being relayed
3. Post resolution update

---

### restart-attesters.sh

Rolling restart of all attestation services.

**Usage:**
```bash
./scripts/operations/restart-attesters.sh
```

**What it does:**
- Restarts attesters one at a time
- 30 second soak time between restarts
- Maintains 3-of-5 threshold throughout

**Use cases:**
- Apply configuration changes
- Update service version
- Clear memory leaks

**Safe:** Threshold maintained at all times

---

### backup-logs.sh

Backup logs from all services.

**Usage:**
```bash
./scripts/operations/backup-logs.sh
```

**What it does:**
- Fetches last 1000 lines from each service
- Creates compressed archive
- Optional S3 upload
- Cleans up backups >7 days old

**Environment variables:**
```bash
export BACKUP_DIR="/var/backups/bridge-logs"
export S3_BUCKET="my-bridge-backups"  # Optional
export ATTESTER_HOSTS="..."
export RELAYER_HOSTS="..."
```

**Output:**
- Local: `$BACKUP_DIR/bridge-logs-TIMESTAMP.tar.gz`
- S3: `s3://$S3_BUCKET/bridge-logs/bridge-logs-TIMESTAMP.tar.gz`

---

## Setup

### 1. Make scripts executable

```bash
chmod +x scripts/operations/*.sh
```

### 2. Configure SSH access

All scripts use SSH to access remote hosts. Set up SSH keys:

```bash
# Generate key if needed
ssh-keygen -t ed25519

# Copy to all hosts
for host in attestation-{0..4}.etrid.io relayer-{1..3}.etrid.io; do
  ssh-copy-id ubuntu@$host
done
```

### 3. Set environment variables

Create `~/.bridge-env`:

```bash
# Attestation services
export ATTESTATION_SERVICES="attestation-0.etrid.io attestation-1.etrid.io attestation-2.etrid.io attestation-3.etrid.io attestation-4.etrid.io"

# Relayer services
export RELAYER_SERVICES="relayer-1.etrid.io relayer-2.etrid.io relayer-3.etrid.io"

# Hosts for SSH
export ATTESTER_HOSTS="$ATTESTATION_SERVICES"
export RELAYER_HOSTS="$RELAYER_SERVICES"

# SSH user
export SSH_USER="ubuntu"

# RPC endpoints
export ETHEREUM_RPC="https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY"
export SUBSTRATE_RPC="wss://ember-rpc.etrid.io"

# Relayer addresses (for balance checking)
export RELAYER_ADDRESSES="0x123...,0x456...,0x789..."

# Backup
export BACKUP_DIR="/var/backups/bridge-logs"
export S3_BUCKET="etrid-bridge-logs"
```

Source before running scripts:
```bash
source ~/.bridge-env
./scripts/operations/health-check.sh
```

---

## Cron Jobs

### Daily health check

```cron
# Run health check every 6 hours
0 */6 * * * /path/to/etrid/scripts/operations/health-check.sh >> /var/log/bridge-health.log 2>&1
```

### Daily backup

```cron
# Backup logs daily at 2am UTC
0 2 * * * /path/to/etrid/scripts/operations/backup-logs.sh >> /var/log/bridge-backup.log 2>&1
```

### Balance alerts

```cron
# Check balances every hour
0 * * * * /path/to/etrid/scripts/operations/check-balances.sh >> /var/log/bridge-balances.log 2>&1
```

---

## Troubleshooting

### Script can't connect to hosts

**Problem:** `ssh: connect to host ... port 22: Connection refused`

**Solutions:**
- Verify SSH key is installed: `ssh ubuntu@host`
- Check firewall allows SSH from your IP
- Verify hostname resolution: `ping host`

### Permission denied

**Problem:** `pm2: command not found`

**Solutions:**
- PM2 not installed on remote host
- PM2 not in PATH for SSH sessions
- Try: `ssh ubuntu@host "source ~/.bashrc && pm2 status"`

### AWS CLI not found (backup-logs.sh)

**Problem:** `aws: command not found`

**Solutions:**
- Install AWS CLI: `pip install awscli`
- Or skip S3 upload by not setting `S3_BUCKET`

---

## Best Practices

1. **Test in staging first** - Always test scripts on staging/testnet before mainnet

2. **Use version control** - Commit script changes with clear messages

3. **Document customizations** - If you modify scripts, document what and why

4. **Monitor after changes** - Watch logs for 30 min after any operation

5. **Have rollback plan** - Know how to undo before running destructive operations

6. **Coordinate with team** - Announce before running emergency scripts

---

## Support

For issues with these scripts:
- **GitHub**: https://github.com/etrid/etrid/issues
- **Discord**: #devops channel
- **Email**: ops@etrid.io

---

## License

Apache-2.0
