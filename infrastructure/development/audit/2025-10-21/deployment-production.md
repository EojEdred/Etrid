# Ëtrid Protocol - Production Deployment Guide

**Last Updated:** October 21, 2025
**Version:** 1.0.0 (Pre-Mainnet)
**Status:** Pre-Audit Phase

---

## Table of Contents

1. [Overview](#overview)
2. [System Requirements](#system-requirements)
3. [Pre-Deployment Checklist](#pre-deployment-checklist)
4. [Infrastructure Setup](#infrastructure-setup)
5. [Building from Source](#building-from-source)
6. [Node Configuration](#node-configuration)
7. [Network Deployment](#network-deployment)
8. [Monitoring & Observability](#monitoring--observability)
9. [Security Hardening](#security-hardening)
10. [Backup & Disaster Recovery](#backup--disaster-recovery)
11. [Maintenance & Updates](#maintenance--updates)
12. [Troubleshooting](#troubleshooting)

---

## Overview

This guide covers deploying the Ëtrid Protocol to production, including:

- **FlareChain** (relay chain with ASF consensus)
- **13 Partition Burst Chains (PBCs)**: BTC, ETH, SOL, ADA, XRP, TRX, BNB, DOGE, MATIC, LINK, XLM, SC-USDT, EDSC
- **ËDSC Bridge** infrastructure
- **Validator & Collator** nodes

### Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    FLARECHAIN (Relay)                   │
│         ASF Consensus + 100+ Validator Nodes            │
└─────────────────────────────────────────────────────────┘
                          │
          ┌───────────────┴───────────────┐
          │                               │
┌─────────▼─────────┐         ┌──────────▼──────────┐
│   PBC Collators   │         │   ËDSC Bridge       │
│  (13 chains x 3   │         │  (Oracle + Attesters)│
│   collators each) │         └─────────────────────┘
└───────────────────┘
```

---

## System Requirements

### FlareChain Validator Node

| Component | Minimum | Recommended | Notes |
|-----------|---------|-------------|-------|
| **CPU** | 8 cores | 16+ cores | AMD EPYC or Intel Xeon |
| **RAM** | 32 GB | 64 GB | ECC memory recommended |
| **Storage** | 1 TB NVMe SSD | 2 TB NVMe SSD | High IOPS required |
| **Network** | 100 Mbps | 1 Gbps | Low latency preferred |
| **OS** | Ubuntu 22.04 LTS | Ubuntu 22.04 LTS | Debian-based distros supported |

### PBC Collator Node (per chain)

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 4 cores | 8 cores |
| **RAM** | 16 GB | 32 GB |
| **Storage** | 500 GB NVMe SSD | 1 TB NVMe SSD |
| **Network** | 50 Mbps | 100 Mbps |

### ËDSC Bridge Oracle/Attester

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 4 cores | 8 cores |
| **RAM** | 16 GB | 32 GB |
| **Storage** | 250 GB SSD | 500 GB SSD |
| **Network** | 100 Mbps | 500 Mbps |

### Total Infrastructure (Full Deployment)

- **1 FlareChain relay chain**: 100+ validators (distributed globally)
- **13 PBCs**: 3 collators each = 39 collator nodes
- **ËDSC Bridge**: 5-7 oracle/attester nodes
- **Monitoring**: 3-5 monitoring servers

**Estimated Total Nodes:** 150-200 globally distributed

---

## Pre-Deployment Checklist

### Security Audit

- [ ] External security audit completed
- [ ] All critical/high severity findings resolved
- [ ] Audit report published (if applicable)
- [ ] Bug bounty program launched

### Testing

- [ ] All unit tests passing (coverage > 80%)
- [ ] Integration tests passing
- [ ] Property-based tests passing
- [ ] Stress tests completed successfully
- [ ] 24h+ continuous operation tested on testnet

### Code Quality

- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` formatting applied
- [ ] All TODOs resolved or documented
- [ ] Documentation complete

### Legal & Compliance

- [ ] Legal review completed
- [ ] Terms of service published
- [ ] Privacy policy published
- [ ] Regulatory compliance verified (if applicable)

### Operational Readiness

- [ ] Monitoring dashboards configured
- [ ] Alert rules configured
- [ ] Incident response plan documented
- [ ] On-call rotation established
- [ ] Runbooks prepared

---

## Infrastructure Setup

### 1. Server Provisioning

#### Cloud Providers (Recommended)

- **AWS**: EC2 instances (i3en.4xlarge or similar)
- **Google Cloud**: n2-highmem-16 with local SSD
- **Azure**: Standard_E16s_v5 with premium SSD
- **Bare Metal**: Hetzner, OVH, or similar

#### Geographic Distribution (FlareChain Validators)

Distribute validators across:
- North America (30%)
- Europe (30%)
- Asia-Pacific (25%)
- Other regions (15%)

This ensures network resilience and decentralization.

### 2. Operating System Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install required packages
sudo apt install -y \
    build-essential \
    git \
    curl \
    clang \
    libssl-dev \
    llvm \
    libudev-dev \
    pkg-config \
    protobuf-compiler \
    libclang-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify Rust installation
rustc --version
cargo --version
```

### 3. Network Configuration

```bash
# Configure firewall (UFW)
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH
sudo ufw allow 22/tcp

# FlareChain P2P port
sudo ufw allow 30333/tcp

# FlareChain RPC (only from specific IPs)
# sudo ufw allow from <MONITORING_IP> to any port 9944 proto tcp

# Enable firewall
sudo ufw enable

# Verify status
sudo ufw status verbose
```

### 4. Create Service User

```bash
# Create etrid user
sudo useradd -m -s /bin/bash etrid
sudo usermod -aG sudo etrid

# Switch to etrid user
sudo su - etrid
```

---

## Building from Source

### 1. Clone Repository

```bash
cd /home/etrid
git clone https://github.com/etrid-protocol/etrid.git
cd etrid

# Checkout specific release (replace with actual version)
git checkout v1.0.0

# Verify commit signature (if applicable)
# git verify-commit HEAD
```

### 2. Build FlareChain Node

```bash
cd 05-multichain/flare-chain

# Build in release mode
cargo build --release

# This takes 30-60 minutes depending on hardware
# Binary will be at: target/release/flarechain-node

# Verify build
./target/release/flarechain-node --version
```

### 3. Build PBC Nodes (Example: BTC PBC)

```bash
cd ../partition-burst-chains/pbc-chains/btc-pbc

cargo build --release

# Binary: target/release/btc-pbc-node
./target/release/btc-pbc-node --version
```

### 4. Install Binaries

```bash
# Copy binaries to /usr/local/bin
sudo cp /home/etrid/etrid/05-multichain/flare-chain/target/release/flarechain-node \
    /usr/local/bin/

sudo cp /home/etrid/etrid/05-multichain/partition-burst-chains/pbc-chains/*/target/release/*-pbc-node \
    /usr/local/bin/

# Verify installation
flarechain-node --version
```

---

## Node Configuration

### 1. FlareChain Validator Configuration

Create chain specification:

```bash
# Generate chain spec
flarechain-node build-spec \
    --chain=flarechain-mainnet \
    --disable-default-bootnode \
    > /home/etrid/flarechain-spec.json

# Convert to raw format
flarechain-node build-spec \
    --chain=/home/etrid/flarechain-spec.json \
    --raw \
    --disable-default-bootnode \
    > /home/etrid/flarechain-spec-raw.json
```

### 2. Generate Validator Keys

```bash
# Generate session keys
flarechain-node key generate --scheme Sr25519

# Output:
# Secret phrase: <MNEMONIC_PHRASE>
# Network ID: etrid
# Secret seed: <SECRET_SEED>
# Public key (hex): <PUBLIC_KEY>
# Account ID: <ACCOUNT_ID>
# SS58 Address: <SS58_ADDRESS>

# IMPORTANT: Store the secret phrase securely!
# Use a hardware security module (HSM) or secure key management system
```

### 3. Node Data Directory

```bash
# Create data directory
mkdir -p /home/etrid/.local/share/flarechain

# Set permissions
chmod 700 /home/etrid/.local/share/flarechain
```

### 4. Configuration File

Create `/home/etrid/flarechain-config.toml`:

```toml
# FlareChain Validator Configuration

[network]
listen_address = "/ip4/0.0.0.0/tcp/30333"
public_address = "/ip4/<PUBLIC_IP>/tcp/30333"
boot_nodes = [
    "/ip4/BOOTNODE1_IP/tcp/30333/p2p/BOOTNODE1_PEER_ID",
    "/ip4/BOOTNODE2_IP/tcp/30333/p2p/BOOTNODE2_PEER_ID",
]

[rpc]
port = 9944
cors = ["http://localhost:*", "http://127.0.0.1:*"]
methods = ["Safe"]  # Only safe RPC methods exposed

[telemetry]
url = "wss://telemetry.etrid.io/submit"
verbosity = 0

[database]
path = "/home/etrid/.local/share/flarechain"
cache_size = 1024  # MB
state_pruning = "archive"  # or "constrained" for pruning nodes

[prometheus]
enabled = true
port = 9615
```

---

## Network Deployment

### 1. Systemd Service Configuration

Create `/etc/systemd/system/flarechain.service`:

```ini
[Unit]
Description=FlareChain Validator Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=etrid
Group=etrid
WorkingDirectory=/home/etrid

ExecStart=/usr/local/bin/flarechain-node \
    --validator \
    --name="Validator-Node-1" \
    --chain=/home/etrid/flarechain-spec-raw.json \
    --base-path=/home/etrid/.local/share/flarechain \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors=all \
    --rpc-methods=Safe \
    --prometheus-port 9615 \
    --prometheus-external \
    --telemetry-url 'wss://telemetry.etrid.io/submit 0' \
    --bootnodes /ip4/BOOTNODE_IP/tcp/30333/p2p/PEER_ID

# Restart configuration
Restart=always
RestartSec=10

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=/home/etrid/.local/share/flarechain

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

### 2. Start FlareChain Node

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable flarechain

# Start service
sudo systemctl start flarechain

# Check status
sudo systemctl status flarechain

# View logs
sudo journalctl -u flarechain -f
```

### 3. Verify Node Synchronization

```bash
# Check node sync status
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://localhost:9944

# Expected output:
# {"jsonrpc":"2.0","result":{"isSyncing":false,"peers":50,"shouldHavePeers":true},"id":1}

# Check current block
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
    http://localhost:9944
```

### 4. Insert Session Keys

```bash
# Generate session keys on the node
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
    http://localhost:9944

# Output: "0x<SESSION_KEYS_HEX>"

# Submit session keys on-chain via governance extrinsic
# This requires the validator account to have tokens staked
```

---

## Monitoring & Observability

### 1. Prometheus Setup

Install Prometheus:

```bash
# Download Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.47.0/prometheus-2.47.0.linux-amd64.tar.gz

# Extract
tar xvfz prometheus-*.tar.gz
cd prometheus-*

# Create config
cat > prometheus.yml <<EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'flarechain-validator'
    static_configs:
      - targets: ['localhost:9615']
        labels:
          instance: 'validator-1'
EOF

# Run Prometheus
./prometheus --config.file=prometheus.yml
```

### 2. Grafana Dashboard

Install Grafana:

```bash
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install grafana

sudo systemctl start grafana-server
sudo systemctl enable grafana-server
```

Access Grafana at `http://<SERVER_IP>:3000` (default credentials: admin/admin)

Import Substrate node dashboard:
- Dashboard ID: 13759 (Substrate Node Metrics)

### 3. Key Metrics to Monitor

| Metric | Alert Threshold | Description |
|--------|----------------|-------------|
| `substrate_block_height` | Falling behind chain | Current block height |
| `substrate_finalized_height` | Not increasing | Finalized block height |
| `substrate_peers_count` | < 10 | Number of connected peers |
| `substrate_ready_transactions_number` | > 10000 | Transaction pool size |
| `process_cpu_seconds_total` | > 90% sustained | CPU usage |
| `process_resident_memory_bytes` | > 90% of RAM | Memory usage |
| `substrate_database_cache_bytes` | Near limit | Database cache usage |

### 4. Log Aggregation

Use Loki or ELK stack for centralized logging:

```bash
# Install Promtail (Loki log shipper)
wget https://github.com/grafana/loki/releases/download/v2.9.0/promtail-linux-amd64.zip
unzip promtail-linux-amd64.zip
sudo mv promtail-linux-amd64 /usr/local/bin/promtail

# Configure Promtail to ship journalctl logs to Loki
```

---

## Security Hardening

### 1. SSH Hardening

Edit `/etc/ssh/sshd_config`:

```bash
# Disable root login
PermitRootLogin no

# Disable password authentication (use SSH keys only)
PasswordAuthentication no
PubkeyAuthentication yes

# Limit users
AllowUsers etrid

# Change default port (optional but recommended)
Port 2222

# Restart SSH
sudo systemctl restart sshd
```

### 2. Fail2ban

```bash
# Install fail2ban
sudo apt install fail2ban

# Configure for SSH
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local

# Edit jail.local
sudo nano /etc/fail2ban/jail.local

# Enable SSH protection
[sshd]
enabled = true
port = 2222
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600

# Restart fail2ban
sudo systemctl restart fail2ban
```

### 3. Key Management

**CRITICAL**: Validator keys control staked funds and network security.

Best practices:
- ✅ Use hardware security modules (HSM) for production validators
- ✅ Store mnemonic phrases offline in secure vaults
- ✅ Use multi-signature schemes for governance keys
- ✅ Rotate session keys periodically
- ✅ Implement key backup and recovery procedures
- ❌ Never store keys in plain text
- ❌ Never commit keys to version control

### 4. Network Isolation

```bash
# Use VPN or private network for node communication
# Only expose necessary ports publicly

# Example: Restrict RPC access to monitoring server only
sudo ufw delete allow 9944/tcp
sudo ufw allow from <MONITORING_SERVER_IP> to any port 9944 proto tcp
```

---

## Backup & Disaster Recovery

### 1. Database Backups

```bash
# Stop node
sudo systemctl stop flarechain

# Backup database
tar -czvf flarechain-backup-$(date +%Y%m%d).tar.gz \
    /home/etrid/.local/share/flarechain/chains/flarechain/db

# Restart node
sudo systemctl start flarechain

# Upload backup to cloud storage (S3, GCS, etc.)
aws s3 cp flarechain-backup-*.tar.gz s3://etrid-backups/
```

### 2. Automated Backup Script

Create `/home/etrid/backup.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/home/etrid/backups"
CHAIN_DATA="/home/etrid/.local/share/flarechain"
S3_BUCKET="s3://etrid-backups"

mkdir -p $BACKUP_DIR

# Create backup
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/flarechain-$DATE.tar.gz"

tar -czvf $BACKUP_FILE $CHAIN_DATA

# Upload to S3
aws s3 cp $BACKUP_FILE $S3_BUCKET/

# Delete local backups older than 7 days
find $BACKUP_DIR -type f -mtime +7 -delete

# Delete S3 backups older than 30 days (requires lifecycle policy)
```

### 3. Disaster Recovery Procedure

1. **Node Failure:**
   - Provision new server
   - Restore latest database backup
   - Restore validator keys from secure storage
   - Start node and verify sync

2. **Data Corruption:**
   - Stop node
   - Restore from last known good backup
   - Re-sync from that point

3. **Key Compromise:**
   - Immediately unbond validator
   - Rotate all keys
   - Investigate breach
   - Report to security team

---

## Maintenance & Updates

### 1. Runtime Upgrades

FlareChain supports forkless upgrades via on-chain governance:

```bash
# When new runtime is approved:
# 1. Node operators do not need to manually upgrade
# 2. Runtime is automatically updated via SetCode extrinsic
# 3. Monitor logs for successful runtime upgrade

# View runtime version
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' \
    http://localhost:9944
```

### 2. Node Binary Updates

```bash
# Stop node
sudo systemctl stop flarechain

# Backup current binary
sudo cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup

# Download and install new binary
# (Follow building from source steps)

# Verify new version
flarechain-node --version

# Start node
sudo systemctl start flarechain

# Monitor logs
sudo journalctl -u flarechain -f
```

### 3. Database Pruning (for non-archive nodes)

```bash
# Stop node
sudo systemctl stop flarechain

# Prune database
flarechain-node purge-chain \
    --chain=/home/etrid/flarechain-spec-raw.json \
    --base-path=/home/etrid/.local/share/flarechain

# Restart and re-sync
sudo systemctl start flarechain
```

---

## Troubleshooting

### Node Not Syncing

```bash
# Check peer count
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
    http://localhost:9944

# If peers = 0, check:
# 1. Firewall allows port 30333
# 2. Bootnodes are correct
# 3. Network connectivity

# Add more bootnodes
flarechain-node \
    --bootnodes /ip4/BOOTNODE1/tcp/30333/p2p/PEER_ID \
    --bootnodes /ip4/BOOTNODE2/tcp/30333/p2p/PEER_ID
```

### High Memory Usage

```bash
# Check database cache size
# Reduce in config: database.cache_size = 512

# Enable state pruning (non-archive)
flarechain-node --state-pruning=constrained
```

### Missing Blocks (Validator)

```bash
# Check if session keys are correctly set
# Verify validator is in active set
# Check network latency
# Ensure clock is synchronized (NTP)

# Install NTP
sudo apt install ntp
sudo systemctl start ntp
```

### Database Corruption

```bash
# Stop node
sudo systemctl stop flarechain

# Remove corrupted database
rm -rf /home/etrid/.local/share/flarechain/chains/flarechain/db

# Restore from backup or re-sync from genesis
sudo systemctl start flarechain
```

---

## Additional Resources

- **Official Documentation:** https://docs.etrid.io
- **Network Status:** https://telemetry.etrid.io
- **Block Explorer:** https://explorer.etrid.io
- **Support:** support@etrid.io
- **Emergency Contact:** security@etrid.io (for security incidents)

---

## Appendix: Network Ports Reference

| Service | Port | Protocol | Public | Description |
|---------|------|----------|--------|-------------|
| SSH | 22 (or custom) | TCP | ✅ | Secure shell access |
| P2P (FlareChain) | 30333 | TCP | ✅ | Peer-to-peer networking |
| RPC (HTTP) | 9944 | TCP | ❌ | JSON-RPC (internal only) |
| WebSocket RPC | 9945 | TCP | ❌ | WebSocket RPC (internal only) |
| Prometheus | 9615 | TCP | ❌ | Metrics endpoint |
| Grafana | 3000 | TCP | ❌ | Monitoring dashboard |

---

**IMPORTANT SECURITY NOTICE:**

- Never expose RPC ports (9944, 9945) to the public internet
- Always use TLS/SSL for RPC connections
- Implement IP whitelisting for sensitive endpoints
- Regularly update node software
- Monitor security advisories

---

**Deployment Checklist Summary:**

- [ ] Hardware provisioned per requirements
- [ ] OS updated and hardened
- [ ] Firewall configured
- [ ] Node built and installed
- [ ] Configuration files created
- [ ] Systemd service configured
- [ ] Node syncing successfully
- [ ] Validator keys generated and secured
- [ ] Session keys registered on-chain
- [ ] Monitoring configured (Prometheus + Grafana)
- [ ] Alerting configured
- [ ] Backup procedures tested
- [ ] Disaster recovery plan documented
- [ ] Team trained on operations

---

**End of Production Deployment Guide**

For testnet deployment guide, see: `docs/guides/deployment-testnet.md`
