# FlareChain Systemd Service Guide

**Date:** November 3, 2025
**Purpose:** Production-grade service management for FlareChain validators

---

## Overview

This guide shows how to deploy and manage FlareChain validators using systemd services for:
- ✅ Auto-restart on crash
- ✅ Auto-start on VM reboot
- ✅ Clean log management
- ✅ Resource limits and security hardening
- ✅ Standard service management commands

---

## Files Provided

1. **`flarechain-validator.service.template`** - Systemd service template
2. **`deploy-systemd-services.sh`** - Automated deployment script
3. **`SYSTEMD_SERVICE_GUIDE.md`** - This documentation

---

## Quick Start

### 1. Update Configuration

Before deploying, update these values in `deploy-systemd-services.sh`:

```bash
# Update bootnode addresses (after genesis)
BOOTNODES="/ip4/BOOTNODE_IP_1/tcp/30333/p2p/PEER_ID_1,..."

# Update VM IP addresses
VM_IPS=(
    ["oracle_vm1"]="129.xxx.xxx.xxx"  # Replace with actual IPs
    ["azure_vm1"]="20.xxx.xxx.xxx"
    # ... etc
)

# Update validator names (optional)
VALIDATORS=(
    ["oracle_vm1"]="GizziDirector"
    ["azure_vm1"]="AzureDirector1"
    # ... etc
)
```

### 2. Deploy to All VMs

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
chmod +x deploy-systemd-services.sh
./deploy-systemd-services.sh
```

**This will:**
- Create customized service files for each validator
- Deploy to all 21 VMs in parallel
- Install services (but not start them yet)

### 3. Start Validators

After deployment, start services on all VMs:

```bash
# Start all validators in parallel
for vm in oracle_vm{1..6} azure_vm{1..10} aws_vm{1..4} local_vm1; do
    ssh <user>@<vm-ip> "sudo systemctl start flarechain-validator" &
done
wait
```

### 4. Enable Auto-Start

```bash
# Enable auto-start on boot for all VMs
for vm in oracle_vm{1..6} azure_vm{1..10} aws_vm{1..4} local_vm1; do
    ssh <user>@<vm-ip> "sudo systemctl enable flarechain-validator" &
done
wait
```

---

## Service Management Commands

### On Each VM:

```bash
# Start validator
sudo systemctl start flarechain-validator

# Stop validator
sudo systemctl stop flarechain-validator

# Restart validator
sudo systemctl restart flarechain-validator

# Check status
sudo systemctl status flarechain-validator

# Enable auto-start on boot
sudo systemctl enable flarechain-validator

# Disable auto-start
sudo systemctl disable flarechain-validator

# View logs (live)
sudo journalctl -u flarechain-validator -f

# View logs (last 100 lines)
sudo journalctl -u flarechain-validator -n 100

# View logs since boot
sudo journalctl -u flarechain-validator -b
```

---

## Service File Features

### Security Hardening

```ini
NoNewPrivileges=true      # Prevent privilege escalation
PrivateTmp=true           # Isolated /tmp directory
ProtectSystem=strict      # Read-only system directories
ProtectHome=true          # Protect /home directories
ReadWritePaths=/var/lib/flarechain  # Only this path is writable
```

### Resource Limits

```ini
LimitNOFILE=65536        # Max open files
LimitNPROC=4096          # Max processes
```

### Restart Policy

```ini
Restart=always           # Always restart on failure
RestartSec=10           # Wait 10 seconds before restart
StartLimitBurst=5       # Max 5 restarts in 10 minutes
```

### Logging

All output goes to systemd journal:
- Accessible via `journalctl`
- Automatic log rotation
- Query by time, priority, etc.

---

## Customization

### Per-Validator Customization

Edit the service file on a specific VM:

```bash
sudo systemctl edit --full flarechain-validator
```

Then reload:

```bash
sudo systemctl daemon-reload
sudo systemctl restart flarechain-validator
```

### Common Customizations

**Change log level:**
```ini
ExecStart=/usr/local/bin/flarechain-node \
  --chain=/home/validator/chainspec-mainnet-raw-FIXED.json \
  --validator \
  -l runtime=debug  # Add this line
```

**Add telemetry:**
```ini
ExecStart=/usr/local/bin/flarechain-node \
  --chain=/home/validator/chainspec-mainnet-raw-FIXED.json \
  --validator \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"  # Add this
```

**Change ports:**
```ini
  --port=30444 \
  --rpc-port=9944 \
  --prometheus-port=9616
```

---

## Troubleshooting

### Service Won't Start

```bash
# Check detailed status
sudo systemctl status flarechain-validator

# Check logs for errors
sudo journalctl -u flarechain-validator -n 50

# Test command manually
sudo -u validator /usr/local/bin/flarechain-node --version
```

### Service Keeps Restarting

```bash
# Check if hitting restart limits
sudo systemctl status flarechain-validator

# View crash logs
sudo journalctl -u flarechain-validator --since "10 minutes ago"

# Check resource usage
top -u validator
df -h /var/lib/flarechain
```

### Permission Issues

```bash
# Ensure correct ownership
sudo chown -R validator:validator /var/lib/flarechain

# Check binary permissions
ls -l /usr/local/bin/flarechain-node

# Should be: -rwxr-xr-x validator validator
```

---

## Migration from Manual Process

If validators are already running manually, migrate to systemd:

```bash
# 1. Stop manual process
pkill flarechain-node

# 2. Install service (if not done already)
sudo cp flarechain-validator.service /etc/systemd/system/
sudo systemctl daemon-reload

# 3. Start via systemd
sudo systemctl start flarechain-validator

# 4. Verify it's running
sudo systemctl status flarechain-validator

# 5. Enable auto-start
sudo systemctl enable flarechain-validator
```

---

## Monitoring

### Check All Validators Status

```bash
# Create a monitoring script
for vm in oracle_vm{1..6} azure_vm{1..10} aws_vm{1..4} local_vm1; do
    echo "=== $vm ==="
    ssh <user>@<vm-ip> "sudo systemctl is-active flarechain-validator"
done
```

### Prometheus Metrics

Services expose Prometheus metrics on port 9615:
```bash
curl http://localhost:9615/metrics
```

Integrate with Grafana for dashboards.

---

## Advantages Over Manual Start

| Feature | Manual Process | Systemd Service |
|---------|---------------|-----------------|
| Auto-restart on crash | ❌ | ✅ |
| Auto-start on reboot | ❌ | ✅ |
| Log management | Manual | Automatic (journalctl) |
| Resource limits | Manual | Built-in |
| Security hardening | Manual | Built-in |
| Service dependencies | ❌ | ✅ (After=network-online.target) |
| Status monitoring | Manual | `systemctl status` |
| Centralized control | ❌ | ✅ |

---

## Production Checklist

Before mainnet launch, verify on each VM:

- [ ] Service file installed: `/etc/systemd/system/flarechain-validator.service`
- [ ] Binary present: `/usr/local/bin/flarechain-node`
- [ ] Chainspec present: `/home/validator/chainspec-mainnet-raw-FIXED.json`
- [ ] Data directory exists: `/var/lib/flarechain`
- [ ] Data directory owned by validator user
- [ ] Service starts successfully
- [ ] Logs show "Idle" or "Importing blocks"
- [ ] Prometheus metrics accessible
- [ ] Auto-start enabled

---

## Summary

**Systemd services provide:**
- ✅ Production-grade reliability (auto-restart, auto-start)
- ✅ Professional management (standard commands)
- ✅ Built-in logging (journalctl integration)
- ✅ Security hardening (privilege restrictions)
- ✅ Easy deployment (one script for all 21 VMs)

**No rebuilding required** - just deploy the service files and start!

---

**Next Steps:**
1. Update `deploy-systemd-services.sh` with your VM IPs and bootnode addresses
2. Run the deployment script
3. Start and enable services on all VMs
4. Monitor with `systemctl status` and `journalctl`

🚀 **Ready for production mainnet deployment!**
