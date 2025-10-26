# Etrid Network Operator Guide

**Version**: 1.0.0
**Last Updated**: October 22, 2025
**Status**: Production Ready
**Target Audience**: Validator operators, watchtower operators, node administrators

---

## Table of Contents

1. [Introduction](#introduction)
2. [Validator Setup Guide](#validator-setup-guide)
3. [Watchtower Operator Setup](#watchtower-operator-setup)
4. [Monitoring and Alerting](#monitoring-and-alerting)
5. [Maintenance and Upgrades](#maintenance-and-upgrades)
6. [Troubleshooting](#troubleshooting)
7. [Security Best Practices](#security-best-practices)

---

## Introduction

### About Etrid Network Operations

Etrid is a multichain blockchain implementing the E320 (Essential Elements to Operate) protocol with:

- **FlareChain Relay Chain** with Adaptive Stake Finality (ASF) consensus
- **13 Partition Burst Chains (PBCs)** for cross-chain interoperability
- **Lightning-Bloc Layer 2** for payment channels and instant transactions
- **Watchtower Network** for security monitoring and fraud detection

### Network Roles

```
┌─────────────────────────────────────────────────────────────────┐
│                    Etrid Network Topology                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │            FlareChain Validators (21 nodes)                │  │
│  │  - Run ASF consensus (PPFA committee)                      │  │
│  │  - Produce blocks every 5 seconds                          │  │
│  │  - Achieve finality in ~15 seconds                         │  │
│  │  - Minimum stake: 64 ETR (Validity Node)                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │         PBC Collators (13 chains x N collators)            │  │
│  │  - Validate partition burst chain blocks                   │  │
│  │  - Submit state checkpoints to FlareChain                  │  │
│  │  - Operate cross-chain bridges                             │  │
│  │  - Minimum stake: 64 ETR per chain                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │         Watchtower Operators (Lightning-Bloc)              │  │
│  │  - Monitor payment channel states                          │  │
│  │  - Detect fraud attempts                                   │  │
│  │  - Earn rewards for interventions                          │  │
│  │  - No minimum stake required                               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Peer Types and Stake Requirements

| Peer Type | Minimum Stake | Role | Privileges |
|-----------|---------------|------|------------|
| **Common Peer** | 0 ETR | Network participant | Read access, transactions |
| **Staking Common** | 1 ETR | Staker | Vote on governance proposals |
| **Validity Node** | 64 ETR | PBC Validator | Validate PBC blocks, bridge operations |
| **Flare Node** | 64 ETR | FlareChain Validator | PPFA committee, block production |
| **Decentralized Director** | 128 ETR | Governance leader | Propose changes, vote weights |

---

## Validator Setup Guide

### Hardware Requirements

#### Minimum Requirements (FlareChain Validator)

```
CPU:        4 cores (8 threads) @ 2.5 GHz
RAM:        16 GB DDR4
Storage:    500 GB NVMe SSD (high IOPS)
Network:    100 Mbps symmetric, <50ms latency
OS:         Ubuntu 22.04 LTS or later
```

#### Recommended Requirements (Production)

```
CPU:        8 cores (16 threads) @ 3.0 GHz (AMD EPYC/Intel Xeon)
RAM:        32 GB DDR4 ECC
Storage:    1 TB NVMe SSD (Samsung 980 PRO or better)
            - 10,000+ IOPS read/write
            - Hardware RAID 1 recommended
Network:    1 Gbps symmetric fiber, <20ms latency
            - Dual WAN links for redundancy
            - DDoS protection service
OS:         Ubuntu 22.04 LTS Server
Backup:     Separate backup server with 2 TB storage
```

#### Enterprise Requirements (High-Availability)

```
Primary Node:
  CPU:        16 cores (32 threads) @ 3.5 GHz
  RAM:        64 GB DDR4 ECC
  Storage:    2 TB NVMe SSD RAID 1
  Network:    10 Gbps fiber, <10ms latency

Hot Standby:
  Identical specs to primary
  Geographic redundancy (different datacenter)

Monitoring:
  Dedicated Prometheus + Grafana server
  Alertmanager with PagerDuty integration
```

#### PBC Collator Requirements

Each PBC collator has similar requirements to FlareChain validators, but can share infrastructure:

```
CPU:        2 cores per PBC
RAM:        4 GB per PBC
Storage:    100 GB per PBC
Network:    50 Mbps per PBC

Example: Running 3 PBCs requires:
  CPU:      6 cores total
  RAM:      12 GB total
  Storage:  300 GB total
  Network:  150 Mbps total
```

### Software Requirements

#### Operating System

**Supported OS:**
- Ubuntu 22.04 LTS (recommended)
- Ubuntu 20.04 LTS
- Debian 11+
- CentOS 8+ / Rocky Linux 8+
- Arch Linux (advanced users)

**NOT supported:**
- Windows (use WSL2 for development only)
- macOS (development only)

#### Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install core dependencies
sudo apt install -y \
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    pkg-config \
    protobuf-compiler

# Install Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version  # Should be 1.70.0 or higher
cargo --version
```

### Installation Steps

#### Step 1: Clone Repository

```bash
# Create installation directory
mkdir -p ~/etrid
cd ~/etrid

# Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Checkout stable release
git checkout v1.0.0  # Replace with latest stable tag
```

#### Step 2: Build Validator Node

```bash
# Build FlareChain validator (release mode)
cargo build --release --package etrid-flare-node

# Verify build
./target/release/etrid-flare-node --version

# Optional: Build specific components
cargo build --release --package etrid-btc-pbc-collator  # Bitcoin PBC
cargo build --release --package etrid-eth-pbc-collator  # Ethereum PBC
```

**Build Time Estimates:**
- First build: 30-60 minutes (downloads dependencies)
- Incremental builds: 5-15 minutes
- Clean build on enterprise hardware: 10-20 minutes

#### Step 3: Create System User

```bash
# Create dedicated user for validator
sudo useradd -m -s /bin/bash etrid-validator
sudo usermod -aG sudo etrid-validator  # Optional: for troubleshooting

# Create directories
sudo mkdir -p /var/lib/etrid-validator/{chains,keystore,config}
sudo chown -R etrid-validator:etrid-validator /var/lib/etrid-validator

# Copy binary
sudo cp target/release/etrid-flare-node /usr/local/bin/
sudo chown etrid-validator:etrid-validator /usr/local/bin/etrid-flare-node
sudo chmod 755 /usr/local/bin/etrid-flare-node
```

### Configuration

#### Generate Validator Keys

```bash
# Switch to validator user
sudo -u etrid-validator -i

# Generate session keys
etrid-flare-node key generate --scheme Sr25519 --output-type json > ~/stash-key.json
etrid-flare-node key generate --scheme Sr25519 --output-type json > ~/controller-key.json
etrid-flare-node key generate --scheme Ed25519 --output-type json > ~/session-key.json

# IMPORTANT: Backup these files to secure offline storage!
# Store passwords in password manager (1Password, Bitwarden, etc.)

# View stash address (your validator identity)
cat ~/stash-key.json | grep -o '"ss58Address":"[^"]*"'

# Import keys to keystore
etrid-flare-node key insert \
  --base-path /var/lib/etrid-validator \
  --chain flarechain \
  --scheme Sr25519 \
  --suri "$(cat ~/stash-key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)" \
  --key-type stash

etrid-flare-node key insert \
  --base-path /var/lib/etrid-validator \
  --chain flarechain \
  --scheme Sr25519 \
  --suri "$(cat ~/controller-key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)" \
  --key-type controller
```

**Key Security Best Practices:**
- Generate keys on offline machine if possible
- Use hardware security module (HSM) for production
- Never share private keys or seed phrases
- Use separate stash (cold) and controller (hot) keys
- Rotate session keys every 6-12 months

#### Node Configuration File

Create `/var/lib/etrid-validator/config/config.toml`:

```toml
# Etrid FlareChain Validator Configuration

[chain]
name = "flarechain"
base_path = "/var/lib/etrid-validator"

[network]
# Node name (visible in telemetry)
name = "MyValidator-01"

# Listen addresses
listen_addr = [
    "/ip4/0.0.0.0/tcp/30333",      # P2P
    "/ip4/0.0.0.0/tcp/30334/ws"    # WebSocket
]

# Bootnodes (connect to network)
bootnodes = [
    "/dns4/bootnode-01.etrid.network/tcp/30333/p2p/12D3KooWBootnode1...",
    "/dns4/bootnode-02.etrid.network/tcp/30333/p2p/12D3KooWBootnode2...",
]

# External address (your public IP/domain)
public_addr = [
    "/dns4/validator.example.com/tcp/30333"
]

# Reserved peers (other validators)
reserved_peers = []

# Maximum peer connections
max_peers_in = 50
max_peers_out = 50

[rpc]
# RPC endpoints (localhost only for security)
rpc_addr = "127.0.0.1:9944"
rpc_max_connections = 100
rpc_cors = ["http://localhost:*"]

# WebSocket RPC
ws_addr = "127.0.0.1:9945"
ws_max_connections = 100

[consensus]
# Validator mode
validator = true

# PPFA committee participation
ppfa_enabled = true

# Block production
enable_authoring = true

[database]
# Database backend
backend = "rocksdb"

# Database cache (in MB)
db_cache = 2048

# State pruning (keep last N blocks)
state_pruning = 256

# Block pruning (archive mode = keep all)
blocks_pruning = "archive"

[telemetry]
# Enable telemetry
enabled = true

# Telemetry endpoints
endpoints = [
    "wss://telemetry.etrid.network/submit 0"
]

[prometheus]
# Prometheus metrics endpoint
enabled = true
addr = "127.0.0.1:9615"

[log]
# Logging level (trace, debug, info, warn, error)
level = "info"

# Log targets
targets = [
    "sync=debug",
    "consensus=debug",
    "network=info"
]
```

#### Systemd Service

Create `/etc/systemd/system/etrid-validator.service`:

```ini
[Unit]
Description=Etrid FlareChain Validator
After=network.target
Documentation=https://docs.etrid.network

[Service]
Type=simple
User=etrid-validator
Group=etrid-validator

# Working directory
WorkingDirectory=/var/lib/etrid-validator

# Command to start validator
ExecStart=/usr/local/bin/etrid-flare-node \
  --config /var/lib/etrid-validator/config/config.toml \
  --validator \
  --name "MyValidator-01" \
  --chain flarechain \
  --base-path /var/lib/etrid-validator \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  --prometheus-port 9615 \
  --prometheus-external

# Restart policy
Restart=always
RestartSec=10

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/etrid-validator

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=etrid-validator

[Install]
WantedBy=multi-user.target
```

### Starting and Stopping the Validator

```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable etrid-validator

# Start validator
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator

# View logs
sudo journalctl -u etrid-validator -f

# Stop validator
sudo systemctl stop etrid-validator

# Restart validator
sudo systemctl restart etrid-validator
```

### Registering as a Validator

#### Step 1: Acquire Minimum Stake

```bash
# You need at least 64 ETR for Validity Node / Flare Node
# Transfer ETR to your stash account address
# Verify balance using web wallet or CLI:

etrid-cli account balance <STASH_ADDRESS>
# Expected output: Balance: 64.000000 ETR
```

#### Step 2: Bond Stake

```bash
# Using web wallet (https://wallet.etrid.network):
# 1. Navigate to "Staking" tab
# 2. Click "Bond Funds"
# 3. Enter stash account
# 4. Enter controller account
# 5. Amount: 64 ETR (or more)
# 6. Payment destination: "Stash account (increase stake)"
# 7. Sign and submit transaction

# Using CLI:
etrid-cli staking bond \
  --stash <STASH_ADDRESS> \
  --controller <CONTROLLER_ADDRESS> \
  --value 64000000000000000000 \  # 64 ETR in wei (18 decimals)
  --payment-destination stash
```

#### Step 3: Generate Session Keys

```bash
# On your validator node, generate session keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944

# Output will be a hex string like:
# 0x1234567890abcdef...

# Copy this hex string for next step
```

#### Step 4: Set Session Keys

```bash
# Using web wallet:
# 1. Navigate to "Staking" > "Account Actions"
# 2. Click "Set Session Key"
# 3. Paste hex string from previous step
# 4. Sign and submit transaction

# Using CLI:
etrid-cli staking set-session-keys \
  --controller <CONTROLLER_ADDRESS> \
  --keys <SESSION_KEYS_HEX>
```

#### Step 5: Validate

```bash
# Using web wallet:
# 1. Navigate to "Staking" > "Account Actions"
# 2. Click "Validate"
# 3. Set commission: 0-100% (recommended: 5-10%)
# 4. Sign and submit transaction

# Using CLI:
etrid-cli staking validate \
  --controller <CONTROLLER_ADDRESS> \
  --commission 10  # 10% commission
```

#### Step 6: Wait for Next Era

```bash
# Check validator status
etrid-cli query validators

# Check if you're in the next validator set
etrid-cli query validators --era next

# Monitor session progress
etrid-cli query session info
```

**Expected Timeline:**
- Transaction confirmation: ~15 seconds (finality)
- Validator set update: Next era (~4 hours)
- First block production: Within 5 seconds of era start

---

## Watchtower Operator Setup

### Purpose and Requirements

Watchtower operators monitor Lightning-Bloc payment channels for fraudulent activity and earn rewards for successful interventions.

**Purpose:**
- Monitor channel states for old state broadcasts
- Detect double-spend attempts
- Identify invalid signatures
- Prevent unauthorized channel closures

**Economic Model:**
- Earn rewards for fraud detection
- Earn subscription fees from channel participants
- Earn uptime bonuses for 99.9%+ availability
- Reputation-based ranking system

### Hardware Requirements

#### Minimum Requirements

```
CPU:        2 cores @ 2.0 GHz
RAM:        4 GB
Storage:    100 GB SSD
Network:    25 Mbps symmetric, <100ms latency
OS:         Ubuntu 22.04 LTS
```

#### Recommended Requirements

```
CPU:        4 cores @ 2.5 GHz
RAM:        8 GB
Storage:    200 GB SSD
Network:    100 Mbps symmetric, <50ms latency
OS:         Ubuntu 22.04 LTS
Uptime:     99.9% (hosted on reliable VPS)
```

### Installation and Configuration

#### Step 1: Install Watchtower Service

```bash
# Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid/07-transactions/lightning-bloc/watchtower

# Build watchtower
cargo build --release

# Copy binary
sudo cp target/release/lightning-watchtower /usr/local/bin/
sudo chmod 755 /usr/local/bin/lightning-watchtower
```

#### Step 2: Configuration File

Create `/etc/etrid/watchtower-config.toml`:

```toml
# Lightning-Bloc Watchtower Configuration

[service]
# Service name
name = "MyWatchtower-01"

# Check interval (seconds)
check_interval = 30

# Maximum concurrent channels to monitor
max_channels = 1000

[network]
# FlareChain node endpoint
chain_endpoint = "ws://127.0.0.1:9944"

# Lightning network topology file
topology_path = "/var/lib/etrid-watchtower/network-topology.json"

[alerts]
# Alert webhook URL (Slack, Discord, etc.)
webhook_url = "https://hooks.slack.com/services/YOUR/WEBHOOK/URL"

# Alert thresholds
balance_deviation_threshold = 0.1  # 10%
time_until_expiry_critical = 3600  # 1 hour
time_until_expiry_warning = 86400  # 24 hours

[subscriptions]
# Subscription pricing (per channel, per month)
basic_monthly = 50      # $50/month
premium_monthly = 125   # $125/month
enterprise_monthly = 250 # $250/month

# Default tier for new subscriptions
default_tier = "basic"

[monitoring]
# Prometheus metrics
prometheus_enabled = true
prometheus_port = 9616

# Health check endpoint
health_check_port = 8080

[storage]
# Database for channel states
database_path = "/var/lib/etrid-watchtower/watchtower.db"

# Evidence storage (IPFS, local, etc.)
evidence_storage = "local"
evidence_path = "/var/lib/etrid-watchtower/evidence"

[reputation]
# Enable reputation tracking
enabled = true

# Reputation scoring weights
detection_accuracy_weight = 0.4
uptime_weight = 0.4
intervention_success_weight = 0.2
```

#### Step 3: Systemd Service

Create `/etc/systemd/system/etrid-watchtower.service`:

```ini
[Unit]
Description=Etrid Lightning-Bloc Watchtower
After=network.target etrid-validator.service
Documentation=https://docs.etrid.network

[Service]
Type=simple
User=etrid-watchtower
Group=etrid-watchtower

WorkingDirectory=/var/lib/etrid-watchtower

ExecStart=/usr/local/bin/lightning-watchtower \
  --config /etc/etrid/watchtower-config.toml

Restart=always
RestartSec=10

LimitNOFILE=65536

StandardOutput=journal
StandardError=journal
SyslogIdentifier=etrid-watchtower

[Install]
WantedBy=multi-user.target
```

#### Step 4: Create User and Directories

```bash
# Create user
sudo useradd -m -s /bin/bash etrid-watchtower

# Create directories
sudo mkdir -p /var/lib/etrid-watchtower/{evidence,channels}
sudo chown -R etrid-watchtower:etrid-watchtower /var/lib/etrid-watchtower

# Create config directory
sudo mkdir -p /etc/etrid
sudo chown root:root /etc/etrid
sudo chmod 755 /etc/etrid
```

### Monitoring Oracle Feeds

Watchtowers monitor multiple data sources:

```bash
# 1. On-chain channel state
# Query channel status from FlareChain
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "lightning_getChannelState",
    "params": ["<CHANNEL_ID>"],
    "id": 1
  }'

# 2. Lightning network gossip
# Subscribe to network topology updates
# (handled automatically by watchtower service)

# 3. Payment channel events
# Monitor for:
# - Channel opening
# - State updates
# - Channel closing
# - Dispute initiation
```

### Alert Configuration

#### Webhook Integration

**Slack:**
```bash
# Create Slack webhook:
# 1. Go to https://api.slack.com/apps
# 2. Create new app
# 3. Add "Incoming Webhooks" feature
# 4. Create webhook for channel (e.g., #watchtower-alerts)
# 5. Copy webhook URL to config

# Test webhook:
curl -X POST https://hooks.slack.com/services/YOUR/WEBHOOK/URL \
  -H "Content-Type: application/json" \
  -d '{
    "text": "🚨 Test alert from Etrid Watchtower",
    "blocks": [{
      "type": "section",
      "text": {
        "type": "mrkdwn",
        "text": "*Channel*: 0xabc123\n*Type*: Old State Broadcast\n*Severity*: Critical"
      }
    }]
  }'
```

**Discord:**
```bash
# Create Discord webhook:
# 1. Go to Server Settings > Integrations > Webhooks
# 2. Create webhook
# 3. Copy webhook URL

# Test webhook:
curl -X POST https://discord.com/api/webhooks/YOUR/WEBHOOK \
  -H "Content-Type: application/json" \
  -d '{
    "content": "🚨 **Watchtower Alert**",
    "embeds": [{
      "title": "Old State Broadcast Detected",
      "description": "Channel: 0xabc123",
      "color": 15158332,
      "fields": [
        {"name": "Severity", "value": "Critical", "inline": true},
        {"name": "Penalty", "value": "5000 ETR", "inline": true}
      ]
    }]
  }'
```

#### Email Alerts

```bash
# Install mail utilities
sudo apt install -y mailutils postfix

# Configure SMTP in /etc/postfix/main.cf
relayhost = [smtp.gmail.com]:587
smtp_sasl_auth_enable = yes
smtp_sasl_password_maps = hash:/etc/postfix/sasl_passwd
smtp_sasl_security_options = noanonymous
smtp_tls_security_level = encrypt

# Add credentials to /etc/postfix/sasl_passwd
[smtp.gmail.com]:587 your-email@gmail.com:your-app-password

# Reload postfix
sudo postmap /etc/postfix/sasl_passwd
sudo systemctl reload postfix

# Test email
echo "Test alert" | mail -s "Watchtower Test" your-email@example.com
```

---

## Monitoring and Alerting

### Key Metrics to Monitor

#### Validator Metrics

```
┌─────────────────────────────────────────────────────────────┐
│                  Validator Health Dashboard                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Consensus Metrics:                                          │
│  - Blocks produced (per era)                                 │
│  - Blocks missed (should be 0)                               │
│  - Era points earned                                         │
│  - PPFA committee participation rate (%)                     │
│  - Finality vote participation (%)                           │
│                                                               │
│  Network Metrics:                                            │
│  - Peer count (should be 30-50)                              │
│  - Sync status (should be "synced")                          │
│  - Best block number                                         │
│  - Finalized block number                                    │
│  - Block import time (ms)                                    │
│                                                               │
│  System Metrics:                                             │
│  - CPU usage (%) - alert if >80%                             │
│  - RAM usage (%) - alert if >85%                             │
│  - Disk usage (%) - alert if >80%                            │
│  - Disk I/O (IOPS) - alert if degraded                       │
│  - Network bandwidth (Mbps)                                  │
│  - System uptime                                             │
│                                                               │
│  Economic Metrics:                                           │
│  - Stake amount (ETR)                                        │
│  - Nomination count                                          │
│  - Commission rate (%)                                       │
│  - Rewards earned (per era)                                  │
│  - Slash events (should be 0)                                │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

#### Watchtower Metrics

```
┌─────────────────────────────────────────────────────────────┐
│                  Watchtower Health Dashboard                 │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Monitoring Metrics:                                         │
│  - Channels monitored (count)                                │
│  - Channel health checks (per minute)                        │
│  - Average check duration (ms)                               │
│  - Failed checks (should be <1%)                             │
│                                                               │
│  Detection Metrics:                                          │
│  - Fraud alerts generated (count)                            │
│  - Fraud types detected (breakdown)                          │
│  - False positive rate (%) - target <5%                      │
│  - Detection accuracy (%) - target >95%                      │
│  - Average response time (ms)                                │
│                                                               │
│  Economic Metrics:                                           │
│  - Subscription revenue (per month)                          │
│  - Fraud detection rewards (total)                           │
│  - Uptime bonuses earned                                     │
│  - Reputation score (0-100)                                  │
│  - Network ranking                                           │
│                                                               │
│  System Metrics:                                             │
│  - Service uptime (%) - target >99.9%                        │
│  - CPU usage (%)                                             │
│  - Memory usage (%)                                          │
│  - Database size (GB)                                        │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Setting up Prometheus/Grafana

#### Step 1: Install Prometheus

```bash
# Download Prometheus
cd /tmp
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xvfz prometheus-*.tar.gz
sudo mv prometheus-2.45.0.linux-amd64 /opt/prometheus

# Create user
sudo useradd -M -r -s /bin/false prometheus

# Create directories
sudo mkdir -p /etc/prometheus /var/lib/prometheus
sudo chown prometheus:prometheus /var/lib/prometheus

# Create configuration
sudo tee /etc/prometheus/prometheus.yml > /dev/null <<EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'etrid-validator'
    static_configs:
      - targets: ['localhost:9615']

  - job_name: 'etrid-watchtower'
    static_configs:
      - targets: ['localhost:9616']

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['localhost:9100']
EOF

# Create systemd service
sudo tee /etc/systemd/system/prometheus.service > /dev/null <<EOF
[Unit]
Description=Prometheus
After=network.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/opt/prometheus/prometheus \\
  --config.file=/etc/prometheus/prometheus.yml \\
  --storage.tsdb.path=/var/lib/prometheus \\
  --web.console.templates=/opt/prometheus/consoles \\
  --web.console.libraries=/opt/prometheus/console_libraries

[Install]
WantedBy=multi-user.target
EOF

# Start Prometheus
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus

# Verify
curl http://localhost:9090/-/healthy
```

#### Step 2: Install Node Exporter

```bash
# Download Node Exporter
cd /tmp
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.1/node_exporter-1.6.1.linux-amd64.tar.gz
tar xvfz node_exporter-*.tar.gz
sudo mv node_exporter-1.6.1.linux-amd64/node_exporter /usr/local/bin/

# Create systemd service
sudo tee /etc/systemd/system/node-exporter.service > /dev/null <<EOF
[Unit]
Description=Node Exporter
After=network.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/usr/local/bin/node_exporter

[Install]
WantedBy=multi-user.target
EOF

# Start Node Exporter
sudo systemctl daemon-reload
sudo systemctl enable node-exporter
sudo systemctl start node-exporter
```

#### Step 3: Install Grafana

```bash
# Add Grafana repository
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -

# Install Grafana
sudo apt-get update
sudo apt-get install -y grafana

# Start Grafana
sudo systemctl enable grafana-server
sudo systemctl start grafana-server

# Access Grafana at http://localhost:3000
# Default credentials: admin / admin
```

#### Step 4: Configure Grafana Dashboards

```bash
# 1. Login to Grafana (http://localhost:3000)
# 2. Add Prometheus data source:
#    - Configuration > Data Sources > Add data source
#    - Select Prometheus
#    - URL: http://localhost:9090
#    - Click "Save & Test"

# 3. Import Etrid dashboards:
#    - Create > Import
#    - Upload dashboard JSON from:
#      /docs/monitoring/grafana-validator-dashboard.json
#      /docs/monitoring/grafana-watchtower-dashboard.json

# 4. Create custom alerts in Grafana
```

### Alert Rules and Notifications

Create `/etc/prometheus/alert-rules.yml`:

```yaml
groups:
  - name: validator_alerts
    interval: 30s
    rules:
      # Validator is not producing blocks
      - alert: ValidatorNotProducing
        expr: increase(substrate_proposer_block_constructed_total[5m]) == 0
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "Validator stopped producing blocks"
          description: "No blocks produced in last 10 minutes"

      # High CPU usage
      - alert: HighCPUUsage
        expr: 100 - (avg by (instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 80
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage detected"
          description: "CPU usage above 80% for 5 minutes"

      # Low disk space
      - alert: LowDiskSpace
        expr: (node_filesystem_avail_bytes / node_filesystem_size_bytes) * 100 < 20
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Low disk space"
          description: "Less than 20% disk space remaining"

      # Validator lost peers
      - alert: ValidatorLostPeers
        expr: substrate_sub_libp2p_peers_count < 10
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Validator has few peers"
          description: "Peer count below 10 for 5 minutes"

  - name: watchtower_alerts
    interval: 30s
    rules:
      # Watchtower service down
      - alert: WatchtowerDown
        expr: up{job="etrid-watchtower"} == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Watchtower service is down"
          description: "Watchtower has been down for 2 minutes"

      # High false positive rate
      - alert: HighFalsePositiveRate
        expr: watchtower_false_positive_rate > 0.1
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High false positive rate"
          description: "False positive rate above 10%"

      # Low reputation score
      - alert: LowReputationScore
        expr: watchtower_reputation_score < 50
        for: 30m
        labels:
          severity: warning
        annotations:
          summary: "Low reputation score"
          description: "Reputation score below 50"
```

Update Prometheus config to include alert rules:

```yaml
# /etc/prometheus/prometheus.yml
rule_files:
  - "/etc/prometheus/alert-rules.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']
```

#### Install and Configure Alertmanager

```bash
# Download Alertmanager
cd /tmp
wget https://github.com/prometheus/alertmanager/releases/download/v0.26.0/alertmanager-0.26.0.linux-amd64.tar.gz
tar xvfz alertmanager-*.tar.gz
sudo mv alertmanager-0.26.0.linux-amd64 /opt/alertmanager

# Create configuration
sudo tee /opt/alertmanager/alertmanager.yml > /dev/null <<EOF
global:
  resolve_timeout: 5m
  smtp_smarthost: 'smtp.gmail.com:587'
  smtp_from: 'alerts@example.com'
  smtp_auth_username: 'alerts@example.com'
  smtp_auth_password: 'your-app-password'

route:
  group_by: ['alertname', 'severity']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'email-notifications'
  routes:
    - match:
        severity: critical
      receiver: 'pagerduty'
      continue: true

receivers:
  - name: 'email-notifications'
    email_configs:
      - to: 'operator@example.com'
        headers:
          Subject: '{{ .GroupLabels.alertname }}'

  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_KEY'
        description: '{{ .GroupLabels.alertname }}'

inhibit_rules:
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'instance']
EOF

# Create systemd service
sudo tee /etc/systemd/system/alertmanager.service > /dev/null <<EOF
[Unit]
Description=Alertmanager
After=network.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/opt/alertmanager/alertmanager \\
  --config.file=/opt/alertmanager/alertmanager.yml \\
  --storage.path=/var/lib/alertmanager

[Install]
WantedBy=multi-user.target
EOF

# Start Alertmanager
sudo mkdir -p /var/lib/alertmanager
sudo chown prometheus:prometheus /var/lib/alertmanager
sudo systemctl daemon-reload
sudo systemctl enable alertmanager
sudo systemctl start alertmanager

# Restart Prometheus to load alert rules
sudo systemctl restart prometheus
```

---

## Maintenance and Upgrades

### Runtime Upgrades

Etrid uses forkless runtime upgrades via governance:

```bash
# 1. Monitor for upgrade proposals
etrid-cli query governance proposals --status active

# 2. Review upgrade proposal
etrid-cli query governance proposal <PROPOSAL_ID>

# 3. Prepare for upgrade:
#    - Backup database
#    - Ensure sufficient disk space
#    - Update monitoring dashboards

# 4. Upgrade happens automatically when proposal passes
#    - No node restart required
#    - Runtime upgrades in-place
#    - Monitor logs for upgrade event

# 5. Verify upgrade success
etrid-cli query runtime version
sudo journalctl -u etrid-validator -n 100 | grep -i upgrade
```

**Node Software Upgrades:**

When a new node binary is released:

```bash
# 1. Backup current binary
sudo cp /usr/local/bin/etrid-flare-node /usr/local/bin/etrid-flare-node.backup

# 2. Build or download new version
cd ~/etrid/Etrid
git fetch --tags
git checkout v1.1.0  # New version
cargo build --release --package etrid-flare-node

# 3. Stop validator gracefully
sudo systemctl stop etrid-validator

# 4. Replace binary
sudo cp target/release/etrid-flare-node /usr/local/bin/
sudo chown etrid-validator:etrid-validator /usr/local/bin/etrid-flare-node
sudo chmod 755 /usr/local/bin/etrid-flare-node

# 5. Restart validator
sudo systemctl start etrid-validator

# 6. Monitor logs for successful start
sudo journalctl -u etrid-validator -f

# 7. Verify version
etrid-flare-node --version

# 8. Rollback if needed
# sudo cp /usr/local/bin/etrid-flare-node.backup /usr/local/bin/etrid-flare-node
# sudo systemctl restart etrid-validator
```

### Database Maintenance

#### Database Pruning

```bash
# Check database size
du -sh /var/lib/etrid-validator/chains/flarechain/db

# Archive mode (keeps all blocks):
# - Size: ~50 GB after 1 year
# - Required for: Archive nodes, block explorers

# Pruned mode (keeps last 256 blocks):
# - Size: ~5-10 GB
# - Suitable for: Most validators

# Switch to pruned mode:
sudo systemctl stop etrid-validator

# Edit config or command line args
# Add: --state-pruning 256 --blocks-pruning 256

sudo systemctl start etrid-validator
```

#### Database Compaction

```bash
# RocksDB auto-compacts, but manual compaction can help

# Stop node
sudo systemctl stop etrid-validator

# Run compaction (this can take hours)
etrid-flare-node purge-chain \
  --base-path /var/lib/etrid-validator \
  --chain flarechain \
  --pruning 256 \
  --blocks-pruning 256

# Restart node (will re-sync)
sudo systemctl start etrid-validator

# Alternative: Use ParityDB (faster, smaller)
# Add to config: --database paritydb
```

### Backup Procedures

#### Critical Data to Backup

```
1. Validator keys:
   /var/lib/etrid-validator/keystore/*
   ~/stash-key.json
   ~/controller-key.json
   ~/session-key.json

2. Configuration:
   /var/lib/etrid-validator/config/config.toml
   /etc/systemd/system/etrid-validator.service

3. Database (optional, for quick recovery):
   /var/lib/etrid-validator/chains/flarechain/db/
```

#### Automated Backup Script

Create `/usr/local/bin/backup-validator.sh`:

```bash
#!/bin/bash

# Etrid Validator Backup Script
# Run daily via cron

BACKUP_DIR="/backup/etrid-validator"
DATE=$(date +%Y%m%d-%H%M%S)
BACKUP_PATH="$BACKUP_DIR/backup-$DATE"

# Create backup directory
mkdir -p "$BACKUP_PATH"

# Backup keystore (CRITICAL)
echo "Backing up keystore..."
cp -r /var/lib/etrid-validator/keystore "$BACKUP_PATH/"

# Backup config
echo "Backing up configuration..."
cp /var/lib/etrid-validator/config/config.toml "$BACKUP_PATH/"
cp /etc/systemd/system/etrid-validator.service "$BACKUP_PATH/"

# Backup session keys (if stored locally)
if [ -f ~/stash-key.json ]; then
    cp ~/stash-key.json "$BACKUP_PATH/"
fi
if [ -f ~/controller-key.json ]; then
    cp ~/controller-key.json "$BACKUP_PATH/"
fi
if [ -f ~/session-key.json ]; then
    cp ~/session-key.json "$BACKUP_PATH/"
fi

# Create tarball
echo "Creating archive..."
cd "$BACKUP_DIR"
tar -czf "backup-$DATE.tar.gz" "backup-$DATE"
rm -rf "backup-$DATE"

# Encrypt backup (using GPG)
echo "Encrypting backup..."
gpg --encrypt --recipient your-email@example.com "backup-$DATE.tar.gz"
rm "backup-$DATE.tar.gz"

# Upload to remote storage (S3, Backblaze, etc.)
echo "Uploading to remote storage..."
aws s3 cp "backup-$DATE.tar.gz.gpg" s3://your-backup-bucket/etrid-validator/

# Keep local backups for 7 days
find "$BACKUP_DIR" -name "backup-*.tar.gz.gpg" -mtime +7 -delete

echo "Backup completed: backup-$DATE.tar.gz.gpg"
```

Make executable and add to cron:

```bash
sudo chmod +x /usr/local/bin/backup-validator.sh

# Add to crontab (run daily at 2 AM)
sudo crontab -e
# Add line:
0 2 * * * /usr/local/bin/backup-validator.sh >> /var/log/validator-backup.log 2>&1
```

#### Restore from Backup

```bash
# 1. Download backup from remote storage
aws s3 cp s3://your-backup-bucket/etrid-validator/backup-20251022-020000.tar.gz.gpg .

# 2. Decrypt backup
gpg --decrypt backup-20251022-020000.tar.gz.gpg > backup-20251022-020000.tar.gz

# 3. Extract backup
tar -xzf backup-20251022-020000.tar.gz

# 4. Stop validator
sudo systemctl stop etrid-validator

# 5. Restore keystore
sudo rm -rf /var/lib/etrid-validator/keystore
sudo cp -r backup-20251022-020000/keystore /var/lib/etrid-validator/
sudo chown -R etrid-validator:etrid-validator /var/lib/etrid-validator/keystore

# 6. Restore config
sudo cp backup-20251022-020000/config.toml /var/lib/etrid-validator/config/

# 7. Start validator
sudo systemctl start etrid-validator

# 8. Verify
sudo journalctl -u etrid-validator -f
```

---

## Troubleshooting

### Common Issues and Solutions

#### Issue: Validator Not Producing Blocks

**Symptoms:**
- No blocks produced for multiple eras
- Era points not increasing
- "Not in active validator set" message

**Diagnosis:**
```bash
# Check if validator is active
etrid-cli query validators --active | grep <YOUR_ADDRESS>

# Check stake amount
etrid-cli query staking validators <YOUR_ADDRESS>

# Check session keys
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_hasSessionKeys","params":["<SESSION_KEYS_HEX>"],"id":1}'
```

**Solutions:**
1. **Insufficient stake:** Add more ETR (minimum 64)
2. **Session keys not set:** Run `author_rotateKeys` and set keys
3. **Not in active set:** Wait for next era (validator rotation)
4. **Node offline:** Check systemd status and restart if needed

#### Issue: Sync Stalled

**Symptoms:**
- Best block not increasing
- "Syncing" status for extended period
- Peer count dropping

**Diagnosis:**
```bash
# Check sync status
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","id":1}'

# Check peer count
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_peers","id":1}'

# Check logs for errors
sudo journalctl -u etrid-validator -n 100 --no-pager
```

**Solutions:**
```bash
# 1. Add bootnodes
etrid-flare-node \
  --bootnodes /dns4/bootnode-01.etrid.network/tcp/30333/p2p/12D3KooW... \
  --bootnodes /dns4/bootnode-02.etrid.network/tcp/30333/p2p/12D3KooW...

# 2. Clear peer database
sudo systemctl stop etrid-validator
rm -rf /var/lib/etrid-validator/chains/flarechain/network/*
sudo systemctl start etrid-validator

# 3. Purge and re-sync from snapshot
sudo systemctl stop etrid-validator
etrid-flare-node purge-chain --base-path /var/lib/etrid-validator --chain flarechain
# Download snapshot from https://snapshots.etrid.network
wget https://snapshots.etrid.network/latest.tar.gz
tar -xzf latest.tar.gz -C /var/lib/etrid-validator/chains/flarechain/
sudo chown -R etrid-validator:etrid-validator /var/lib/etrid-validator
sudo systemctl start etrid-validator
```

#### Issue: High Resource Usage

**Symptoms:**
- CPU usage consistently >90%
- RAM usage >90%, OOM kills
- Disk I/O bottleneck

**Diagnosis:**
```bash
# Check resource usage
htop  # or top

# Check disk I/O
iostat -x 1

# Check database size
du -sh /var/lib/etrid-validator/chains/flarechain/db

# Check for memory leaks
sudo journalctl -u etrid-validator | grep -i "memory\|oom"
```

**Solutions:**
```bash
# 1. Upgrade hardware (if at minimum specs)
# 2. Enable pruning
# Add to config: --state-pruning 256 --blocks-pruning 256

# 3. Reduce peer connections
# Add to config: --max-peers-in 25 --max-peers-out 25

# 4. Adjust database cache
# Add to config: --db-cache 2048  # MB

# 5. Use ParityDB instead of RocksDB
# Add to config: --database paritydb

# 6. Restart node regularly (via systemd timer)
# Create /etc/systemd/system/etrid-validator-restart.timer
```

#### Issue: Watchtower Missing Alerts

**Symptoms:**
- Fraud not detected
- False negative rate high
- Reputation score dropping

**Diagnosis:**
```bash
# Check watchtower service
sudo systemctl status etrid-watchtower

# Check logs
sudo journalctl -u etrid-watchtower -n 100

# Check channel subscription status
curl http://localhost:8080/api/v1/channels

# Check chain connection
curl http://localhost:8080/health
```

**Solutions:**
```bash
# 1. Restart watchtower service
sudo systemctl restart etrid-watchtower

# 2. Verify chain endpoint is accessible
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","id":1}'

# 3. Update network topology
# Download latest topology:
wget https://topology.etrid.network/lightning-network.json \
  -O /var/lib/etrid-watchtower/network-topology.json

# 4. Reduce check interval (if missing events)
# Edit /etc/etrid/watchtower-config.toml
# check_interval = 10  # seconds

# 5. Increase concurrent channel limit
# max_channels = 2000

sudo systemctl restart etrid-watchtower
```

### Log Analysis

#### Validator Logs

```bash
# View real-time logs
sudo journalctl -u etrid-validator -f

# Search for errors
sudo journalctl -u etrid-validator | grep -i error

# Search for specific events
sudo journalctl -u etrid-validator | grep "Block produced"
sudo journalctl -u etrid-validator | grep "PPFA"
sudo journalctl -u etrid-validator | grep "Finality"

# Export logs for analysis
sudo journalctl -u etrid-validator --since "2025-10-22 00:00:00" \
  --until "2025-10-22 23:59:59" > validator-logs-20251022.txt

# Common log patterns:

# Good signs:
✅ "Imported #12345 (0xabc...)"        # Block imported
✅ "Block produced: #12346"            # Produced block
✅ "Finalized #12340"                  # Block finalized
✅ "PPFA vote submitted"               # Participated in consensus

# Warning signs:
⚠️  "Peer count: 5"                    # Low peer count
⚠️  "Sync lag detected"                # Sync issues
⚠️  "Database cache exhausted"        # Need more RAM

# Critical issues:
❌ "Session keys not set"              # Need to set keys
❌ "Insufficient stake"                # Need to bond more
❌ "Cannot connect to bootnode"       # Network issues
```

#### Watchtower Logs

```bash
# View real-time logs
sudo journalctl -u etrid-watchtower -f

# Search for fraud detections
sudo journalctl -u etrid-watchtower | grep "Fraud detected"

# Search for interventions
sudo journalctl -u etrid-watchtower | grep "Intervention"

# Common log patterns:

# Good signs:
✅ "Loaded 523 channels from topology"
✅ "Channel health check: Healthy"
✅ "Network health: 520/523 healthy"

# Warning signs:
⚠️  "Channel expires in 2h"
⚠️  "Channel imbalanced: 85%/15%"
⚠️  "Low capacity utilization"

# Critical issues:
❌ "Fraud detected: Old state broadcast"
❌ "Critical: Channel expired"
❌ "Failed to submit penalty transaction"
```

### Recovery Procedures

#### Validator Slashed

If your validator is slashed (due to double-signing or extended downtime):

```bash
# 1. Identify slash event
etrid-cli query staking slashes <ERA_INDEX>

# 2. Stop validator immediately
sudo systemctl stop etrid-validator

# 3. Investigate root cause
#    - Check logs for double-signing
#    - Check for clock drift (ntpdate)
#    - Check for duplicate node instances

# 4. Fix underlying issue
#    - Sync system clock: sudo ntpdate -s time.nist.gov
#    - Ensure only one node instance running
#    - Restore from backup if database corrupted

# 5. Wait for unbonding period (28 days)
etrid-cli query staking unbonding <STASH_ADDRESS>

# 6. Withdraw unbonded funds
etrid-cli staking withdraw-unbonded --controller <CONTROLLER_ADDRESS>

# 7. Re-bond and restart (if desired)
etrid-cli staking bond --stash <STASH> --controller <CONTROLLER> --value 64000000000000000000
etrid-cli staking validate --controller <CONTROLLER> --commission 10
```

#### Database Corruption

```bash
# Symptoms:
# - "Database error" in logs
# - "Cannot open database"
# - Segmentation faults

# Recovery:
# 1. Stop node
sudo systemctl stop etrid-validator

# 2. Backup corrupted database (for analysis)
sudo cp -r /var/lib/etrid-validator/chains /backup/corrupted-db-$(date +%Y%m%d)

# 3. Purge database
etrid-flare-node purge-chain \
  --base-path /var/lib/etrid-validator \
  --chain flarechain

# 4. Restore from snapshot (faster than full sync)
cd /var/lib/etrid-validator/chains/flarechain
wget https://snapshots.etrid.network/flarechain-latest.tar.gz
tar -xzf flarechain-latest.tar.gz
sudo chown -R etrid-validator:etrid-validator /var/lib/etrid-validator

# 5. Start node
sudo systemctl start etrid-validator

# 6. Monitor sync progress
sudo journalctl -u etrid-validator -f
```

---

## Security Best Practices

### Key Management

#### Hot vs. Cold Wallets

```
┌─────────────────────────────────────────────────────────────┐
│                   Key Separation Strategy                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  COLD WALLET (Offline, Hardware Security Module):           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Stash Account (holds funds)                          │   │
│  │  - Private key NEVER on server                       │   │
│  │  - Stored on hardware wallet (Ledger, Trezor)       │   │
│  │  - Used only for:                                    │   │
│  │    * Initial bonding                                 │   │
│  │    * Unbonding                                       │   │
│  │    * Changing controller                             │   │
│  │  - Accessed via air-gapped machine                   │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                   │
│                           │ (sets)                            │
│                           ↓                                   │
│  HOT WALLET (Online, On Server):                            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Controller Account (manages operations)              │   │
│  │  - Private key on server (encrypted)                 │   │
│  │  - Used for:                                         │   │
│  │    * Setting session keys                            │   │
│  │    * Starting/stopping validation                    │   │
│  │    * Setting commission                              │   │
│  │  - Limited funds (~1 ETR for fees)                   │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                   │
│                           │ (sets)                            │
│                           ↓                                   │
│  SESSION KEYS (Rotated Regularly):                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ Session Keys (block signing)                         │   │
│  │  - Private keys on server (keystore)                 │   │
│  │  - Used for:                                         │   │
│  │    * Signing blocks                                  │   │
│  │    * PPFA votes                                      │   │
│  │    * Finality votes                                  │   │
│  │  - Rotated every 6-12 months                         │   │
│  │  - Isolated from account funds                       │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                               │
│  BENEFITS:                                                   │
│  ✅ Stash keys never exposed to network                     │
│  ✅ Controller compromise = limited damage                  │
│  ✅ Session key compromise = rotate without unbonding       │
│  ✅ Layered security model                                  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

#### Key Rotation Procedure

```bash
# Rotate session keys every 6-12 months

# 1. Generate new session keys
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_rotateKeys","id":1}'

# Output: "0x1234567890abcdef..." (save this)

# 2. Set new session keys (using controller account)
etrid-cli staking set-session-keys \
  --controller <CONTROLLER_ADDRESS> \
  --keys <NEW_SESSION_KEYS_HEX>

# 3. Wait for next era
# Old keys remain valid until next era
# New keys activate automatically

# 4. Verify new keys are active
etrid-cli query session keys <STASH_ADDRESS>

# 5. Backup new keystore
cp -r /var/lib/etrid-validator/keystore ~/backup/keystore-$(date +%Y%m%d)
```

### Firewall Configuration

```bash
# Install ufw (Uncomplicated Firewall)
sudo apt install -y ufw

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH (change port if using non-standard)
sudo ufw allow 22/tcp comment 'SSH'

# Allow P2P (FlareChain)
sudo ufw allow 30333/tcp comment 'FlareChain P2P'

# Allow WebSocket (if exposing publicly - NOT recommended)
# sudo ufw allow 30334/tcp comment 'FlareChain WebSocket'

# Allow Prometheus (only from monitoring server)
sudo ufw allow from <MONITORING_IP> to any port 9615 proto tcp comment 'Prometheus'

# Allow Node Exporter (only from monitoring server)
sudo ufw allow from <MONITORING_IP> to any port 9100 proto tcp comment 'Node Exporter'

# Enable firewall
sudo ufw enable

# Verify rules
sudo ufw status numbered

# IMPORTANT: DO NOT expose RPC/WebSocket ports publicly
# Use SSH tunneling or VPN for remote access
```

#### Advanced Firewall Rules

```bash
# Rate limiting for SSH (prevent brute force)
sudo ufw limit 22/tcp

# Allow P2P only from known validator IPs
sudo ufw delete allow 30333/tcp
sudo ufw allow from <VALIDATOR_1_IP> to any port 30333 proto tcp
sudo ufw allow from <VALIDATOR_2_IP> to any port 30333 proto tcp
# ... (repeat for all validators)

# Log dropped packets
sudo ufw logging on

# View logs
sudo tail -f /var/log/ufw.log
```

### DDoS Protection

#### Network-Level Protection

```bash
# Use a DDoS protection service:
# - Cloudflare Spectrum (Layer 4 protection)
# - AWS Shield (if on AWS)
# - OVH DDoS protection
# - Hetzner DDoS protection

# Example: Cloudflare Spectrum setup
# 1. Add your server IP to Cloudflare
# 2. Create Spectrum application:
#    - Origin: your-server-ip:30333
#    - Edge: validator.example.com:30333
# 3. Update public_addr in node config:
#    public_addr = ["/dns4/validator.example.com/tcp/30333"]
```

#### System-Level Protection

```bash
# Install fail2ban
sudo apt install -y fail2ban

# Create jail for SSH
sudo tee /etc/fail2ban/jail.local > /dev/null <<EOF
[sshd]
enabled = true
port = 22
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
findtime = 600
EOF

# Enable and start fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

# Check banned IPs
sudo fail2ban-client status sshd
```

#### Kernel Hardening

Add to `/etc/sysctl.conf`:

```bash
# IP Spoofing protection
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1

# Ignore ICMP redirects
net.ipv4.conf.all.accept_redirects = 0
net.ipv6.conf.all.accept_redirects = 0

# Ignore send redirects
net.ipv4.conf.all.send_redirects = 0

# Disable source packet routing
net.ipv4.conf.all.accept_source_route = 0
net.ipv6.conf.all.accept_source_route = 0

# Log Martians
net.ipv4.conf.all.log_martians = 1

# Ignore ICMP ping requests
net.ipv4.icmp_echo_ignore_all = 1

# Enable TCP SYN cookies (DDoS protection)
net.ipv4.tcp_syncookies = 1
net.ipv4.tcp_max_syn_backlog = 2048
net.ipv4.tcp_synack_retries = 2
net.ipv4.tcp_syn_retries = 5

# Apply settings
sudo sysctl -p
```

### Server Hardening

```bash
# 1. Disable root login
sudo sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config

# 2. Use SSH keys only (disable password auth)
sudo sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config

# 3. Change SSH port (optional, security through obscurity)
sudo sed -i 's/#Port 22/Port 2222/' /etc/ssh/sshd_config
# Update firewall: sudo ufw allow 2222/tcp

# 4. Restart SSH
sudo systemctl restart sshd

# 5. Install and configure automatic security updates
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades

# 6. Enable AppArmor
sudo systemctl enable apparmor
sudo systemctl start apparmor

# 7. Install and configure auditd (security auditing)
sudo apt install -y auditd
sudo systemctl enable auditd
sudo systemctl start auditd

# 8. Set up log monitoring
sudo apt install -y logwatch
sudo logwatch --output mail --mailto your-email@example.com --detail high
```

### Monitoring for Security Events

```bash
# Monitor failed login attempts
sudo journalctl -u sshd | grep -i failed

# Monitor sudo usage
sudo journalctl | grep sudo

# Monitor file changes in critical directories
sudo apt install -y aide
sudo aideinit
# Creates /var/lib/aide/aide.db.new
sudo mv /var/lib/aide/aide.db.new /var/lib/aide/aide.db

# Run integrity check
sudo aide --check

# Add to daily cron
echo "0 3 * * * root /usr/bin/aide --check | mail -s 'AIDE Report' your-email@example.com" \
  | sudo tee -a /etc/crontab
```

---

## Appendix

### Useful Commands Reference

```bash
# Validator Management
etrid-cli query validators                              # List all validators
etrid-cli query validators --active                     # List active validators
etrid-cli staking bond <params>                         # Bond stake
etrid-cli staking validate <params>                     # Start validating
etrid-cli staking chill                                 # Stop validating

# Network Status
curl -X POST http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","id":1}'
curl -X POST http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_syncState","id":1}'

# Session Management
curl -X POST http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_rotateKeys","id":1}'

# Service Control
sudo systemctl start etrid-validator
sudo systemctl stop etrid-validator
sudo systemctl restart etrid-validator
sudo systemctl status etrid-validator
sudo journalctl -u etrid-validator -f
```

### Resource Links

- **Official Documentation**: https://docs.etrid.network
- **Block Explorer**: https://explorer.etrid.network
- **Telemetry**: https://telemetry.etrid.network
- **Validator Dashboard**: https://validator-dashboard.etrid.network
- **Community Discord**: https://discord.gg/etrid
- **GitHub Repository**: https://github.com/EojEdred/Etrid
- **Security Contact**: security@etrid.network

### Support Channels

- **Email**: support@etrid.network
- **Discord**: #validator-support channel
- **GitHub Issues**: Technical problems
- **Emergency Contact**: For critical security issues, contact security@etrid.network

---

**Document Version**: 1.0.0
**Last Updated**: October 22, 2025
**Maintainer**: Etrid Development Team
**License**: MIT
