# How to Run an Ëtrid FlareChain Validator

**Last Updated:** November 2, 2025
**Difficulty:** Intermediate
**Time Required:** 30-60 minutes

---

## Overview

This guide shows you how to set up and run your own Ëtrid FlareChain validator node. Whether you want to join the mainnet or run your own testnet, this guide provides the complete configuration.

**What you'll need:**
- A Linux server (Ubuntu 22.04+ recommended)
- Basic command line knowledge
- Your own session keys (we'll generate these)
- A public IP address

---

## Prerequisites

### Hardware Requirements

**Minimum:**
- 2 CPU cores
- 4GB RAM
- 50GB storage
- Stable internet connection
- Public IP address

**Recommended:**
- 4+ CPU cores
- 8GB+ RAM
- 100GB+ SSD storage
- 100 Mbps connection
- Static public IP

### Software Requirements

- Ubuntu 22.04 LTS or newer
- SSH access to your server
- sudo privileges

---

## Part 1: Build the Validator Binary

### Step 1: Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install build dependencies
sudo apt install -y build-essential git curl pkg-config libssl-dev \
  protobuf-compiler clang libclang-dev cmake
```

### Step 2: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source $HOME/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

### Step 3: Clone and Build Ëtrid

```bash
# Clone repository
cd ~
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Build the validator binary (this takes 15-45 minutes)
cargo build --release

# Install binary to system path
sudo cp target/release/etrid-validator /usr/local/bin/
sudo chmod +x /usr/local/bin/etrid-validator

# Verify installation
etrid-validator --version
```

---

## Part 2: Generate Your Session Keys

Session keys are cryptographic keys your validator uses to participate in consensus.

### Step 1: Generate Keys Automatically

```bash
# Generate a new mnemonic (seed phrase)
# SAVE THIS SECURELY - you'll need it to recover your validator
etrid-validator key generate --scheme sr25519

# Output will look like:
# Secret phrase:       word1 word2 word3 ... word12
# Network ID:          substrate
# Secret seed:         0xabcd...
# Public key (hex):    0x1234...
# Account ID:          0x5678...
# Public key (SS58):   5ABC...
# SS58 Address:        5DEF...
```

**⚠️ CRITICAL: Write down your "Secret phrase" and store it securely offline.**

### Step 2: Generate Network Identity (Node Key)

```bash
# Generate a unique node key for network identity
openssl rand -hex 32

# Output will be a 64-character hex string like:
# a1b2c3d4e5f6...
```

**Save this as your NODE_KEY**

---

## Part 3: Get the Chainspec

### Option A: Join Ëtrid Mainnet

```bash
# Download the mainnet chainspec
cd ~
wget https://raw.githubusercontent.com/EojEdred/Etrid/main/docs/mainnet/chainspec-mainnet-raw.json

# Move to a permanent location
sudo mkdir -p /etc/etrid
sudo mv chainspec-mainnet-raw.json /etc/etrid/chainspec.json
```

### Option B: Create Your Own Testnet

```bash
# Generate a custom chainspec for your own network
cd ~/Etrid
etrid-validator build-spec --disable-default-bootnode --chain local > chainspec.json

# Convert to raw format
etrid-validator build-spec --chain chainspec.json --raw > chainspec-raw.json

# Move to permanent location
sudo mkdir -p /etc/etrid
sudo mv chainspec-raw.json /etc/etrid/chainspec.json
```

---

## Part 4: Insert Your Session Keys

Replace `YOUR_MNEMONIC_PHRASE_HERE` with the 12-word phrase you saved in Part 2.

```bash
# Set your mnemonic as a variable for convenience
MNEMONIC="your twelve word mnemonic phrase goes here in quotes"

# Insert AURA key (block production)
etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/etc/etrid/chainspec.json \
  --key-type aura \
  --scheme sr25519 \
  --suri "$MNEMONIC"

# Insert GRANDPA key (finality)
etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/etc/etrid/chainspec.json \
  --key-type gran \
  --scheme ed25519 \
  --suri "$MNEMONIC"

# Insert ASF key (attestation)
etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/etc/etrid/chainspec.json \
  --key-type asfk \
  --scheme sr25519 \
  --suri "$MNEMONIC"
```

### Verify Keys Were Inserted

```bash
ls -la ~/.local/share/etrid-validator/chains/*/keystore/

# You should see 3 files
```

---

## Part 5: Configure Firewall

**CRITICAL:** Port 30333 must be open for P2P networking.

```bash
# Install UFW if not already installed
sudo apt install -y ufw

# Configure firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH (replace YOUR_IP with your actual IP)
sudo ufw allow from YOUR_IP/32 to any port 22 comment 'SSH'

# Allow P2P networking (required for all validators)
sudo ufw allow 30333/tcp comment 'Validator P2P'

# Optional: Allow RPC access (only if you need it)
# sudo ufw allow from YOUR_IP/32 to any port 9944 comment 'RPC'

# Enable firewall
sudo ufw enable

# Verify rules
sudo ufw status
```

---

## Part 6: Create Systemd Service

### Configuration Variables

Replace these with your actual values:

```bash
YOUR_VALIDATOR_NAME="MyValidator"           # Choose a name
YOUR_NODE_KEY="a1b2c3d4..."                 # From Part 2
YOUR_PUBLIC_IP="123.456.789.012"            # Your server's public IP
BOOTNODE_ADDR="/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"  # Mainnet bootnode
```

### Create Service File

**For joining Ëtrid Mainnet:**

```bash
sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Ëtrid FlareChain Validator - YOUR_VALIDATOR_NAME
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME
ExecStart=/usr/local/bin/etrid-validator \\
  --validator \\
  --name "YOUR_VALIDATOR_NAME" \\
  --chain=/etc/etrid/chainspec.json \\
  --base-path $HOME/.local/share/etrid-validator \\
  --node-key YOUR_NODE_KEY \\
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \\
  --bootnodes $BOOTNODE_ADDR \\
  --rpc-cors all \\
  --rpc-port 9944 \\
  --port 30333
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
```

**For your own testnet (no bootnode needed):**

```bash
sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Ëtrid FlareChain Validator - YOUR_VALIDATOR_NAME
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME
ExecStart=/usr/local/bin/etrid-validator \\
  --validator \\
  --name "YOUR_VALIDATOR_NAME" \\
  --chain=/etc/etrid/chainspec.json \\
  --base-path $HOME/.local/share/etrid-validator \\
  --node-key YOUR_NODE_KEY \\
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \\
  --rpc-cors all \\
  --rpc-port 9944 \\
  --port 30333
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
```

---

## Part 7: Start Your Validator

```bash
# Reload systemd to recognize new service
sudo systemctl daemon-reload

# Enable service to start on boot
sudo systemctl enable etrid-validator

# Start the validator
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator
```

---

## Part 8: Verify It's Working

### Check Service Status

```bash
sudo systemctl status etrid-validator

# Should show: Active: active (running)
```

### View Live Logs

```bash
# Watch logs in real-time
sudo journalctl -u etrid-validator -f

# Or view last 50 lines
sudo journalctl -u etrid-validator -n 50 --no-pager
```

### What to Look For

**✅ Successful startup:**
```
Chain specification: Ëtrid FlareChain Mainnet
🏷  Node name: YOUR_VALIDATOR_NAME
💤 Idle (3+ peers), best: #1234
🏆 Imported #1235
```

**✅ If joining mainnet:**
- Should connect to 3+ peers within 60 seconds
- Should start importing blocks immediately
- Genesis hash should be: `0x2fb6...84c4`

**❌ Common issues:**
```
Error: Service exited with code 1
Error: Could not open database
Error: No peers connected
```
See troubleshooting section below.

---

## Troubleshooting

### Service Won't Start

```bash
# Check detailed logs
sudo journalctl -u etrid-validator -n 100 --no-pager

# Common causes:
# 1. Binary not found → verify: which etrid-validator
# 2. Chainspec not found → verify: ls -l /etc/etrid/chainspec.json
# 3. Keys not inserted → verify: ls ~/.local/share/etrid-validator/chains/*/keystore/
# 4. Permission issues → verify: check user in service file
```

### Only Connected to 0-1 Peers

**Most common cause:** Missing `--public-addr` flag

```bash
# Verify the flag is set
sudo systemctl cat etrid-validator | grep public-addr

# Should show: --public-addr /ip4/YOUR_IP/tcp/30333
```

If missing, add it to the service file and restart:
```bash
sudo systemctl daemon-reload
sudo systemctl restart etrid-validator
```

### Port Already in Use

```bash
# Check what's using port 30333
sudo lsof -i :30333

# If another process is using it, either:
# 1. Stop that process
# 2. Change --port in service file to different port
```

### Database Corruption

```bash
# Stop validator
sudo systemctl stop etrid-validator

# Remove database
rm -rf ~/.local/share/etrid-validator/chains/*/db

# Restart (will re-sync from network)
sudo systemctl start etrid-validator
```

### Reset and Start Fresh

```bash
# Stop service
sudo systemctl stop etrid-validator

# Remove all data
rm -rf ~/.local/share/etrid-validator

# Re-insert keys (see Part 4)
# Restart service
sudo systemctl start etrid-validator
```

---

## Important Flag Reference

### Required Flags

| Flag | Purpose | Example |
|------|---------|---------|
| `--validator` | Enable validator mode | - |
| `--name` | Your validator name | `"MyValidator"` |
| `--chain` | Path to chainspec file | `/etc/etrid/chainspec.json` |
| `--base-path` | Data directory | `~/.local/share/etrid-validator` |
| `--node-key` | Network identity | `a1b2c3d4...` (64 hex chars) |
| `--public-addr` | **CRITICAL** - Your public IP | `/ip4/1.2.3.4/tcp/30333` |
| `--port` | P2P port | `30333` |

### Optional Flags

| Flag | Purpose | When to Use |
|------|---------|-------------|
| `--bootnodes` | Connect to bootnode | When joining existing network |
| `--rpc-cors all` | Allow RPC connections | For local RPC access |
| `--unsafe-rpc-external` | Allow external RPC | For remote RPC (⚠️ use with caution) |
| `--rpc-port` | RPC port | Default: `9944` |
| `--prometheus-port` | Metrics port | For monitoring |

---

## Useful Commands

### Service Management

```bash
# Start validator
sudo systemctl start etrid-validator

# Stop validator
sudo systemctl stop etrid-validator

# Restart validator
sudo systemctl restart etrid-validator

# Check status
sudo systemctl status etrid-validator

# View logs (live)
sudo journalctl -u etrid-validator -f

# View logs (last 100 lines)
sudo journalctl -u etrid-validator -n 100 --no-pager

# Enable auto-start on boot
sudo systemctl enable etrid-validator

# Disable auto-start
sudo systemctl disable etrid-validator
```

### Monitoring

```bash
# Check peer count
sudo journalctl -u etrid-validator -n 5 | grep "Idle"

# Check block height
sudo journalctl -u etrid-validator -n 5 | grep "best:"

# Check if authoring blocks
sudo journalctl -u etrid-validator -f | grep "Authored"

# Monitor resource usage
htop
# Press F4 to filter for "etrid"
```

---

## Security Best Practices

### 1. Secure Your Keys

- **Never share your mnemonic phrase**
- Store it offline in multiple secure locations
- Consider using a hardware security module (HSM) for production

### 2. Firewall Configuration

- Only open required ports (30333 for P2P)
- Restrict SSH access to your IP only
- Never expose RPC ports publicly without authentication

### 3. Regular Backups

```bash
# Backup your keystore
tar -czf etrid-keys-backup.tar.gz ~/.local/share/etrid-validator/chains/*/keystore/

# Store securely offline
```

### 4. System Updates

```bash
# Keep system updated
sudo apt update && sudo apt upgrade -y

# But avoid updating during critical validator operations
```

### 5. Monitoring

Set up monitoring to alert you if:
- Validator goes offline
- Peer count drops to 0
- Disk space runs low
- CPU/RAM usage spikes

---

## Mainnet Information

### Ëtrid FlareChain Mainnet

**Network Details:**
- Chain: Ëtrid FlareChain Mainnet
- Genesis Hash: `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`
- Consensus: ASF PPFA + GRANDPA
- Block Time: ~6 seconds

**Bootnode:**
```
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

**Chainspec:**
- Available at: `docs/mainnet/chainspec-mainnet-raw.json`
- Download: https://raw.githubusercontent.com/EojEdred/Etrid/main/docs/mainnet/chainspec-mainnet-raw.json

---

## Next Steps

After your validator is running:

1. **Monitor Performance**
   - Watch logs regularly
   - Set up alerting for downtime
   - Monitor disk usage

2. **Join the Community**
   - GitHub: https://github.com/EojEdred/Etrid
   - Report issues or ask questions

3. **Keep Software Updated**
   - Watch for validator updates
   - Test updates on testnet first
   - Plan maintenance windows

4. **Consider High Availability**
   - Set up backup nodes
   - Use session key rotation
   - Plan disaster recovery

---

## FAQ

**Q: How much does it cost to run a validator?**
A: Server costs vary, but expect $20-100/month for a basic VPS.

**Q: Can I run multiple validators on one server?**
A: No, each validator needs its own server with a unique IP.

**Q: What happens if my validator goes offline?**
A: The network continues with other validators. Restart your service ASAP to minimize downtime.

**Q: Can I change my validator name later?**
A: Yes, edit the `--name` flag in your service file and restart.

**Q: How do I update the validator binary?**
A:
```bash
cd ~/Etrid
git pull
cargo build --release
sudo systemctl stop etrid-validator
sudo cp target/release/etrid-validator /usr/local/bin/
sudo systemctl start etrid-validator
```

**Q: Where can I find more documentation?**
A: Check the `docs/` directory in the repository for detailed guides.

---

## Success Checklist

Your validator is working correctly when you see:

- ✅ Service status is "active (running)"
- ✅ Connected to 3+ peers (mainnet) or 1+ peers (testnet)
- ✅ Importing blocks: `best: #XXXX` is increasing
- ✅ Correct chain in logs (Mainnet or your testnet name)
- ✅ No errors in recent logs
- ✅ CPU/RAM usage is stable

**Congratulations! You're now running an Ëtrid FlareChain validator! 🎉**

---

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review logs: `sudo journalctl -u etrid-validator -n 100`
3. Verify configuration matches this guide
4. Check GitHub issues: https://github.com/EojEdred/Etrid/issues

---

**Document Version:** 1.0
**Last Updated:** November 2, 2025
**Tested On:** Ubuntu 22.04 LTS
**Status:** ✅ Production Ready
