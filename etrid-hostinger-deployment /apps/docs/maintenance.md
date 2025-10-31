# Validator Maintenance

Guide for maintaining ËTRID validator nodes.

## Routine Maintenance

### Daily Tasks

✅ **Monitor Node Health**
```bash
# Check if node is running
systemctl status etrid-validator

# Check peer count
etrid-cli system peers

# Check sync status
etrid-cli system syncState
```

✅ **Review Logs**
```bash
# View recent logs
journalctl -u etrid-validator -n 100 --no-pager

# Monitor in real-time
journalctl -u etrid-validator -f
```

✅ **Check Validator Status**
- Visit validator dashboard
- Verify uptime > 99%
- Check era points earned
- Monitor commission payouts

---

### Weekly Tasks

✅ **System Updates**
```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Reboot if kernel updated
sudo reboot
```

✅ **Disk Space Check**
```bash
# Check disk usage
df -h

# Check blockchain data size
du -sh /var/lib/etrid/chains/

# Clean old logs if needed
sudo journalctl --vacuum-time=7d
```

✅ **Backup Verification**
- Verify automated backups ran
- Test restore procedure
- Confirm session keys backed up

---

### Monthly Tasks

✅ **Performance Review**
- Review monthly era points
- Compare with other validators
- Analyze any downtime incidents
- Optimize if needed

✅ **Security Audit**
- Review firewall rules
- Check for unauthorized access attempts
- Update SSH keys if needed
- Review user permissions

✅ **Software Updates**
- Check for ËTRID node updates
- Test updates on staging
- Schedule mainnet upgrade
- Notify nominators of planned downtime

---

## Node Upgrades

### Preparation

1. **Announce Downtime**
   - Notify nominators 48 hours ahead
   - Post in Discord/Telegram
   - Update validator description

2. **Backup Everything**
```bash
# Backup session keys
cp -r /var/lib/etrid/keys /backup/keys-$(date +%Y%m%d)

# Backup chain data (optional)
rsync -av /var/lib/etrid/chains/ /backup/chains/
```

3. **Test on Testnet**
   - Deploy update to testnet validator
   - Verify functionality
   - Monitor for 24 hours

### Upgrade Procedure

```bash
# Stop validator
sudo systemctl stop etrid-validator

# Backup current binary
sudo cp /usr/local/bin/etrid /usr/local/bin/etrid-backup

# Download new version
wget https://github.com/etrid/etrid/releases/download/v1.1.0/etrid-linux-x64

# Verify checksum
sha256sum etrid-linux-x64

# Install new version
sudo mv etrid-linux-x64 /usr/local/bin/etrid
sudo chmod +x /usr/local/bin/etrid

# Start validator
sudo systemctl start etrid-validator

# Verify running
sudo systemctl status etrid-validator
```

### Post-Upgrade

```bash
# Monitor logs
journalctl -u etrid-validator -f

# Check sync status
etrid-cli system syncState

# Verify producing blocks
# (Check validator dashboard)
```

---

## Troubleshooting

### Node Not Syncing

**Symptoms:**
- Sync status shows "not catching up"
- Block height not increasing

**Solutions:**
```bash
# Check peers
etrid-cli system peers

# If no peers, check firewall
sudo ufw status

# Restart networking
sudo systemctl restart etrid-validator

# Last resort: resync from scratch
sudo rm -rf /var/lib/etrid/chains/*/db
sudo systemctl restart etrid-validator
```

### High CPU/Memory Usage

**Symptoms:**
- CPU constantly > 80%
- Memory usage > 90%

**Solutions:**
```bash
# Check resource usage
top
htop

# Increase limits if needed
sudo systemctl edit etrid-validator
# Add:
[Service]
LimitNOFILE=65536

# Restart service
sudo systemctl daemon-reload
sudo systemctl restart etrid-validator
```

### Not Producing Blocks

**Symptoms:**
- Validator online but no blocks produced
- Zero era points

**Solutions:**
1. Check session keys match on-chain
2. Verify sufficient stake
3. Ensure validator in active set
4. Check for slashing events

---

## Monitoring Setup

### Prometheus + Grafana

```bash
# Install Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.40.0/prometheus-2.40.0.linux-amd64.tar.gz
tar xvfz prometheus-*.tar.gz
sudo mv prometheus-2.40.0.linux-amd64 /opt/prometheus

# Configure Prometheus
cat > /opt/prometheus/prometheus.yml <<EOF
scrape_configs:
  - job_name: 'etrid'
    scrape_interval: 5s
    static_configs:
      - targets: ['localhost:9615']
EOF

# Start Prometheus
/opt/prometheus/prometheus --config.file=/opt/prometheus/prometheus.yml
```

**Key Metrics to Monitor:**
- `etrid_block_height` - Current block height
- `etrid_peers` - Number of connected peers
- `etrid_cpu_usage` - CPU utilization
- `etrid_memory_usage` - Memory utilization
- `etrid_disk_usage` - Disk space

---

## Backup Strategy

### Automated Backups

```bash
# Create backup script
cat > /usr/local/bin/backup-etrid.sh <<'EOF'
#!/bin/bash
BACKUP_DIR="/backup/etrid-$(date +%Y%m%d)"
mkdir -p $BACKUP_DIR

# Backup session keys
cp -r /var/lib/etrid/keys $BACKUP_DIR/

# Backup config
cp /etc/systemd/system/etrid-validator.service $BACKUP_DIR/

echo "Backup completed: $BACKUP_DIR"
EOF

chmod +x /usr/local/bin/backup-etrid.sh

# Schedule daily backups
echo "0 2 * * * /usr/local/bin/backup-etrid.sh" | sudo crontab -
```

### Offsite Backups

```bash
# Sync to remote server
rsync -avz /backup/ remote-server:/backups/etrid/

# Or use cloud storage
rclone sync /backup/ s3:my-bucket/etrid-backups/
```

---

## Emergency Procedures

### Node Crashed

1. Check logs for crash reason
2. Restart node
3. If repeated crashes, rollback to previous version
4. Report issue to ËTRID team

### Session Keys Compromised

1. **IMMEDIATELY** rotate session keys
2. Set new keys on-chain
3. Investigate breach
4. Notify ËTRID security team

### Slashing Event

1. Investigate cause (double signing, downtime)
2. Fix underlying issue
3. Document incident
4. Communicate with nominators

---

## Best Practices

✅ **Uptime**
- Target: 99.9% uptime
- Schedule maintenance during low-traffic periods
- Use backup validator for zero-downtime upgrades

✅ **Security**
- Keep system updated
- Use firewall (only necessary ports open)
- Enable SSH key auth only (no passwords)
- Regular security audits

✅ **Communication**
- Notify nominators of planned maintenance
- Update validator description with contact info
- Be responsive to community

✅ **Monitoring**
- Set up alerts for critical issues
- Monitor 24/7 (use pager services)
- Track historical performance

---

## Resources

**Guides:**
- [Operator Guide](OPERATOR_GUIDE.md) - Full validator setup
- [Monitoring Guide](MONITORING_GUIDE.md) - Detailed monitoring setup

**Tools:**
- [Validator Dashboard](https://validators.etrid.org)
- [Network Status](https://status.etrid.org)
- [Block Explorer](https://explorer.etrid.org)

**Support:**
- 💬 [Discord #validators](https://discord.gg/etrid)
- 📧 Email: validator-support@etrid.org
