# PrimeArc Mainnet Health Check Guide

## Quick Command Reference

### From Your Phone (Termius App)

1. **Connect to any of these servers:**
   - `auditdev` (129.80.122.34) - Recommended
   - `gizzi-io-validator` (100.96.84.69 via Tailscale)

2. **Run the health check:**
   ```bash
   check-mainnet-health
   ```

   Or with explicit TERM:
   ```bash
   TERM=xterm check-mainnet-health
   ```

### One-Liner from Termius

Just SSH and run:
```bash
ssh auditdev "TERM=xterm check-mainnet-health"
```

### From Your Local Machine

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@100.70.242.106 "TERM=xterm check-mainnet-health"
```

Or using the Tailscale alias:
```bash
ssh ubuntu@100.96.84.69 "TERM=xterm check-mainnet-health"
```

## Installation on Other VMs

To install the health check script on any other VM in your Tailscale network:

```bash
# Copy the script
scp /Users/macbook/Desktop/etrid/scripts/check-mainnet-health.sh YOUR_VM:/tmp/

# SSH into the VM and install
ssh YOUR_VM
sudo mv /tmp/check-mainnet-health.sh /usr/local/bin/check-mainnet-health
sudo chmod +x /usr/local/bin/check-mainnet-health

# Create alias (optional)
echo 'alias check-mainnet="TERM=xterm check-mainnet-health"' | sudo tee -a /etc/profile.d/etrid-aliases.sh
sudo chmod +x /etc/profile.d/etrid-aliases.sh
```

## Script Features

The health check script provides:

- ✅ Real-time block height and finalization status
- ✅ ASF-BFT consensus verification
- ✅ Network peer count and quorum status
- ✅ Sync state and finalization lag
- ✅ System resource usage (when SSH accessible)
- ✅ Color-coded output for easy reading
- ✅ Overall health summary

## Output Sections

1. **Blockchain Status**
   - Current and finalized block heights
   - Finalization lag
   - Sync status
   - Token information

2. **Consensus & Finality**
   - Consensus mode (Pure ASF)
   - Finality type (ASF-BFT)
   - Block production method (PPFA)
   - Transaction queue status
   - Estimated finality time

3. **Network & Peering**
   - Connected peer count
   - Validator peer count
   - Peer block height variance
   - Quorum strength

4. **System Health**
   - Server uptime
   - CPU load average
   - Memory usage
   - Disk space

5. **Health Summary**
   - Overall status: HEALTHY/WARNING/ERROR
   - Individual component status

## Advanced Usage

### Check a Different Node

```bash
check-mainnet-health <node-ip>
```

Example:
```bash
check-mainnet-health 100.70.242.106
```

### Watch Mode (Continuous Monitoring)

```bash
watch -n 10 -c "TERM=xterm check-mainnet-health"
```

This will refresh the health check every 10 seconds.

### Save Output to File

```bash
check-mainnet-health > mainnet-health-$(date +%Y%m%d-%H%M%S).txt
```

### Check from External Network

If you're not on Tailscale, you can check via the public auditdev server:

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@129.80.122.34 "TERM=xterm check-mainnet-health"
```

## Termius App Setup

### Save as SSH Snippet

1. Open Termius on your phone
2. Go to **Snippets**
3. Create new snippet:
   - **Name**: "Check Mainnet Health"
   - **Command**: `TERM=xterm check-mainnet-health`
4. Save

Now you can run it with one tap when connected to auditdev!

### Create a Host with Auto-Run

1. Go to **Hosts**
2. Edit or create `auditdev` host:
   - **Address**: 129.80.122.34
   - **Username**: ubuntu
   - **Key**: gizzi-validator
3. In **Startup snippet**: Select "Check Mainnet Health"

Now every time you connect, it will auto-run the health check!

## Troubleshooting

### "command not found"

The script is installed at `/usr/local/bin/check-mainnet-health`. If not found:

```bash
ls -lh /usr/local/bin/check-mainnet-health
```

Reinstall if needed using the installation instructions above.

### "Connection failed"

Check Tailscale connectivity:

```bash
tailscale status | grep gizzi
```

Ensure the gizzi validator is online:

```bash
ping 100.96.84.69
```

### "jq: command not found"

The script will auto-install `jq` on first run. If it fails:

```bash
sudo apt-get update && sudo apt-get install -y jq curl
```

### No color output

Make sure TERM is set:

```bash
TERM=xterm check-mainnet-health
```

## Installed Locations

- **Gizzi Validator**: `/usr/local/bin/check-mainnet-health`
- **Auditdev**: `/usr/local/bin/check-mainnet-health`
- **Local**: `/Users/macbook/Desktop/etrid/scripts/check-mainnet-health.sh`

## Quick Reference Card

```
┌─────────────────────────────────────────────────┐
│  MAINNET HEALTH CHECK QUICK REFERENCE          │
├─────────────────────────────────────────────────┤
│  From Termius (Phone):                          │
│  ssh auditdev                                   │
│  check-mainnet-health                           │
│                                                 │
│  One-liner:                                     │
│  ssh auditdev "TERM=xterm check-mainnet-health" │
│                                                 │
│  Watch mode:                                    │
│  watch -n 10 -c "TERM=xterm check-mainnet-health"│
│                                                 │
│  Check different node:                          │
│  check-mainnet-health <ip-address>             │
└─────────────────────────────────────────────────┘
```

## Script Source

The script source is available at:
- Local: `/Users/macbook/Desktop/etrid/scripts/check-mainnet-health.sh`
- GitHub: (commit to repo to share)

## Support

For issues or questions:
- Check the script is executable: `ls -lh /usr/local/bin/check-mainnet-health`
- Verify Tailscale connectivity: `tailscale status`
- Test RPC connection: `curl http://100.96.84.69:9944`
