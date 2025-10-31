# Ëtrid Validator Quick Start Guide

**Last Updated:** October 30, 2025
**Status:** ✅ Production Ready

This guide shows you how to deploy an Ëtrid FlareChain validator node in under 5 minutes.

---

## Prerequisites

1. **Ubuntu VM** (22.04+ recommended)
   - 2+ CPU cores
   - 4GB+ RAM
   - 50GB+ storage
   - Public IP address

2. **Open Firewall Ports:**
   - Port 22 (SSH) - Your IP only
   - Port 30333 (P2P) - Public (0.0.0.0/0) **CRITICAL**

3. **SSH Access** to your VM

---

## Method 1: Quick Bootstrap (Recommended)

This method automatically reads your validator keys from the JSON file.

### Step 1: Clone Repository

```bash
# On your VM
cd ~
git clone https://github.com/EojEdred/Etrid.git
cd Etrid
```

### Step 2: Build the Binary

```bash
# Run automated build script
sudo ./build-on-vm.sh
```

**Build time:** 15-30 minutes (first time only)

### Step 3: Install Binary

```bash
# Copy binary to system path
sudo cp /root/Etrid/target/release/flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node

# Verify installation
flarechain-node --version
```

### Step 4: Bootstrap Your Validator

```bash
# Run quick bootstrap (replace X with your validator number)
sudo ./quick-bootstrap-validator.sh X

# Examples:
sudo ./quick-bootstrap-validator.sh 1   # Gizzi (Validator #1)
sudo ./quick-bootstrap-validator.sh 2   # EojEdred (Validator #2)
sudo ./quick-bootstrap-validator.sh 3   # Validator #3
```

**Done!** Your validator is now running.

---

## Method 2: Manual Bootstrap

If you prefer manual control or need custom configuration:

### Step 1-3: Same as Method 1

### Step 4: Run Bootstrap Script Manually

```bash
sudo ./bootstrap-validator.sh \
  "Your-Validator-Name" \
  "0x<session-seed>" \
  "0x<aura-key>" \
  "0x<grandpa-key>" \
  "0x<asf-key>" \
  "<node-key-without-0x>"
```

**Where to find keys:**
- See `validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json`
- Extract your validator's keys from the JSON

---

## Verify Validator is Running

### Check Service Status

```bash
sudo systemctl status etrid-validator
```

Expected output:
```
● etrid-validator.service - Ëtrid FlareChain Validator Node
   Active: active (running)
```

### View Live Logs

```bash
sudo journalctl -u etrid-validator -f
```

You should see:
- ✅ `Authored block #X` - Your validator is producing blocks
- ✅ `Imported #X` - Blocks are being imported
- ✅ `best: #X` - Chain is progressing

### Check Validator Keys

```bash
ls -lh /var/lib/etrid/chains/*/keystore/
```

You should see **3 files** (AURA, GRANDPA, ASF keys).

---

## Troubleshooting

### Service Won't Start

```bash
# Check logs for errors
sudo journalctl -u etrid-validator -n 100 --no-pager

# Common issues:
# 1. Keys not inserted correctly
# 2. Permission issues on /var/lib/etrid
# 3. Port 30333 blocked by firewall
```

### Fix Permission Issues

```bash
sudo mkdir -p /var/lib/etrid
sudo chown -R ubuntu:ubuntu /var/lib/etrid
sudo chmod 755 /var/lib/etrid
```

### Re-insert Keys

```bash
# Stop service
sudo systemctl stop etrid-validator

# Remove old keys
sudo rm -rf /var/lib/etrid/chains/*/keystore/*

# Re-run bootstrap
sudo ./quick-bootstrap-validator.sh <validator-index>
```

### Check Firewall

```bash
# Verify UFW rules
sudo ufw status

# If P2P port not open:
sudo ufw allow 30333/tcp comment 'Validator P2P'
```

---

## Service Management

### Common Commands

```bash
# Check status
sudo systemctl status etrid-validator

# Start validator
sudo systemctl start etrid-validator

# Stop validator
sudo systemctl stop etrid-validator

# Restart validator
sudo systemctl restart etrid-validator

# Enable auto-start on boot
sudo systemctl enable etrid-validator

# Disable auto-start
sudo systemctl disable etrid-validator

# View logs
sudo journalctl -u etrid-validator -f

# View last 100 lines
sudo journalctl -u etrid-validator -n 100 --no-pager
```

---

## Configuration Files

| File | Location | Purpose |
|------|----------|---------|
| Binary | `/usr/local/bin/flarechain-node` | Validator executable |
| Service | `/etc/systemd/system/etrid-validator.service` | Systemd service |
| Data | `/var/lib/etrid` | Blockchain data |
| Keystore | `/var/lib/etrid/chains/*/keystore/` | Session keys |

---

## Network Configuration

### Required Ports

| Port | Protocol | Access | Purpose |
|------|----------|--------|---------|
| 22 | TCP | Your IP | SSH access |
| 30333 | TCP | Public | P2P networking |
| 9944 | TCP | Internal | RPC (optional) |
| 9615 | TCP | Internal | Prometheus (optional) |

### UFW Firewall Example

```bash
# Basic setup
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow from <your-ip>/32 to any port 22 comment 'SSH'
sudo ufw allow 30333/tcp comment 'Validator P2P'
sudo ufw enable

# Verify
sudo ufw status
```

---

## What the Bootstrap Script Does

The automated bootstrap script performs these steps:

1. ✅ **Creates directory structure** (`/var/lib/etrid`)
2. ✅ **Sets proper permissions** (ubuntu:ubuntu, 755/700)
3. ✅ **Inserts session keys** (AURA, GRANDPA, ASF with correct key types)
4. ✅ **Creates systemd service** (auto-restart, proper user)
5. ✅ **Enables and starts service** (validator runs automatically)
6. ✅ **Verifies service is running** (checks logs for errors)

**All the issues we encountered are now automated away!**

---

## Key Types Reference

The bootstrap script correctly handles these key types:

| Key Type | Scheme | Purpose | CLI flag |
|----------|--------|---------|----------|
| AURA | Sr25519 | Block authoring | `--key-type aura` |
| GRANDPA | Ed25519 | Finality voting | `--key-type gran` |
| ASF | Sr25519 | Attestation signatures | `--key-type asfk` |

⚠️ **Important:** The ASF key type is `asfk` (not `asf_` or `asf`)

---

## Next Steps After Deployment

1. **Monitor your validator**
   ```bash
   sudo journalctl -u etrid-validator -f
   ```

2. **Check block production**
   - Look for "Authored block #X" messages every 6 seconds

3. **Set up monitoring** (optional)
   - Prometheus: http://your-vm-ip:9615/metrics
   - Telemetry: https://telemetry.polkadot.io/

4. **Backup your keys**
   - Keys are in: `/var/lib/etrid/chains/*/keystore/`
   - **DO NOT SHARE THESE FILES**

5. **Deploy additional validators**
   - Repeat this process for validators #2, #3, etc.
   - Each validator needs its own VM and unique keys

---

## Production Checklist

Before going live:

- [ ] Firewall configured (SSH + P2P only)
- [ ] Validator service running and producing blocks
- [ ] Keys backed up securely
- [ ] Monitoring set up (logs, Prometheus)
- [ ] SSH secured (key-based auth, no password)
- [ ] Automatic updates disabled (to prevent unexpected restarts)
- [ ] VM has adequate resources (CPU, RAM, storage)
- [ ] Public IP assigned and DNS configured (optional)

---

## FAQ

### Q: How do I know if my validator is working?

A: Check the logs for "Authored block" messages:
```bash
sudo journalctl -u etrid-validator -f | grep "Authored"
```

### Q: Can I run multiple validators on one VM?

A: No, each validator needs its own VM with a unique IP.

### Q: What happens if my validator goes offline?

A: The chain continues with other validators. Restart your service ASAP.

### Q: How do I update the validator binary?

A:
```bash
# Stop service
sudo systemctl stop etrid-validator

# Replace binary
sudo cp /path/to/new/flarechain-node /usr/local/bin/

# Start service
sudo systemctl start etrid-validator
```

### Q: Where can I find my validator keys?

A: See `validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json`

---

## Support

If you encounter issues:

1. Check logs: `sudo journalctl -u etrid-validator -n 200`
2. Review this guide's troubleshooting section
3. Verify firewall rules: `sudo ufw status`
4. Check GitHub issues: https://github.com/EojEdred/Etrid/issues

---

## File Reference

| Script | Purpose |
|--------|---------|
| `bootstrap-validator.sh` | Main bootstrap script (manual keys) |
| `quick-bootstrap-validator.sh` | Quick bootstrap (auto-reads JSON) |
| `build-on-vm.sh` | Automated binary build |
| `VALIDATOR_QUICKSTART.md` | This guide |
| `VALIDATOR_FIREWALL_RULES.md` | Detailed firewall setup |
| `BUILD_FIXES_SUMMARY.md` | Build history and fixes |

---

**Status:** ✅ All deployment issues resolved. Anyone can now bootstrap a validator without manual intervention.
