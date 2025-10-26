# Ëtrid Protocol - Production Deployment Guide

**Version:** 1.0
**Last Updated:** October 22, 2025
**Status:** Ready for Production Deployment

---

## Table of Contents

1. [Pre-Deployment Checklist](#pre-deployment-checklist)
2. [Hardware Requirements](#hardware-requirements)
3. [Security Hardening](#security-hardening)
4. [Network Configuration](#network-configuration)
5. [Monitoring & Alerting](#monitoring--alerting)
6. [Deployment Procedures](#deployment-procedures)
7. [Post-Deployment Validation](#post-deployment-validation)
8. [Disaster Recovery](#disaster-recovery)
9. [Maintenance & Operations](#maintenance--operations)

---

## Pre-Deployment Checklist

### Critical Requirements (Must Complete)

- [ ] **Runtime Weights Generated**
  ```bash
  ls -lh runtime-weights/*.rs
  # Should have 8+ weight files
  ```

- [ ] **Load Testing Passed**
  ```bash
  grep "Actual rate" stress-test-results/stress-test-*.log
  # Should show 1,000+ TPS
  ```

- [ ] **Database Optimized**
  ```bash
  cat config/production/database.toml
  # Should have cache_size_mb >= 4096
  ```

- [ ] **Monitoring Configured**
  ```bash
  curl http://localhost:9090/-/healthy
  curl http://localhost:3000/api/health
  # Both should respond
  ```

- [ ] **Security Audit Complete**
  - Code review completed
  - Penetration testing done
  - Vulnerabilities remediated

- [ ] **Backup Strategy Defined**
  - Database backup schedule
  - State snapshot procedures
  - Recovery time objectives (RTO)
  - Recovery point objectives (RPO)

### High Priority (Strongly Recommended)

- [ ] **Multi-Node Testing Complete**
  ```bash
  cd data/multi-node-testnet && ./status.sh
  # Should show all nodes in consensus
  ```

- [ ] **72-Hour Stability Test Passed**
  ```bash
  cat stability-test-results/STABILITY_REPORT_*.md
  # Should show <100 MB memory growth
  ```

- [ ] **Profiling Complete**
  ```bash
  ls -lh profiling-results/flamegraph-*.svg
  # Should have CPU flamegraph
  ```

- [ ] **DDoS Protection**
  - Rate limiting configured
  - RPC endpoint protection
  - Bandwidth limits set

- [ ] **SSL/TLS Certificates**
  - Certificates obtained
  - Auto-renewal configured
  - HTTPS enforced

---

## Hardware Requirements

### Validator Node (Minimum)

```yaml
CPU: 8 cores (x86_64)
RAM: 16 GB
Storage: 500 GB NVMe SSD
Network: 100 Mbps symmetric
OS: Ubuntu 22.04 LTS or later
```

### Validator Node (Recommended)

```yaml
CPU: 16 cores (x86_64)
RAM: 32 GB
Storage: 1 TB NVMe SSD
Network: 1 Gbps symmetric
OS: Ubuntu 22.04 LTS
```

### Archive Node

```yaml
CPU: 16+ cores
RAM: 64 GB
Storage: 2 TB NVMe SSD (expandable)
Network: 1 Gbps symmetric
OS: Ubuntu 22.04 LTS
```

### RPC Node

```yaml
CPU: 8-16 cores
RAM: 32 GB
Storage: 1 TB NVMe SSD
Network: 1 Gbps symmetric
OS: Ubuntu 22.04 LTS
Load Balancer: nginx or HAProxy
```

### Cloud Provider Recommendations

**AWS:**
- Validator: `c6i.4xlarge` or `c6i.8xlarge`
- Archive: `r6i.4xlarge` with EBS `io2` storage
- RPC: `c6i.4xlarge` with Application Load Balancer

**Google Cloud:**
- Validator: `c2-standard-16`
- Archive: `n2-highmem-16`
- RPC: `c2-standard-8` with Cloud Load Balancing

**Dedicated Hosting:**
- Hetzner: `AX102` or `RX170`
- OVH: `Advance-2` or `Advance-4`
- Linode: `Dedicated 32GB` or `64GB`

---

## Security Hardening

### 1. Operating System

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install security updates automatically
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure --priority=low unattended-upgrades

# Configure firewall
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow 22/tcp      # SSH (change port if needed)
sudo ufw allow 30333/tcp   # P2P
sudo ufw allow 9944/tcp    # RPC (only if needed publicly)
sudo ufw enable

# Disable root login
sudo sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config
sudo systemctl restart sshd

# Enable fail2ban
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### 2. Node Security

```bash
# Create dedicated user
sudo useradd -m -s /bin/bash etrid
sudo usermod -aG sudo etrid

# Set file permissions
sudo chown -R etrid:etrid /opt/etrid
sudo chmod 700 /opt/etrid/data
sudo chmod 600 /opt/etrid/config/*

# Use systemd for process management (not screen/tmux)
# See systemd service files below
```

### 3. Network Security

```bash
# Install and configure nginx as reverse proxy (for RPC)
sudo apt install -y nginx

# Configure rate limiting
cat > /etc/nginx/conf.d/rate-limit.conf <<EOF
limit_req_zone \$binary_remote_addr zone=rpc_limit:10m rate=10r/s;
limit_conn_zone \$binary_remote_addr zone=conn_limit:10m;
EOF

# RPC proxy configuration
cat > /etc/nginx/sites-available/etrid-rpc <<EOF
server {
    listen 443 ssl http2;
    server_name rpc.etrid.example.com;

    ssl_certificate /etc/letsencrypt/live/etrid.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/etrid.example.com/privkey.pem;

    limit_req zone=rpc_limit burst=20;
    limit_conn conn_limit 10;

    location / {
        proxy_pass http://127.0.0.1:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/etrid-rpc /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

### 4. Key Management

```bash
# Store keys securely
sudo mkdir -p /opt/etrid/keys
sudo chmod 700 /opt/etrid/keys

# Use hardware security modules (HSM) for production
# or encrypted key storage

# Never commit keys to git
echo "/opt/etrid/keys/*" >> /opt/etrid/.gitignore

# Use key rotation policies
# Rotate session keys monthly
# Rotate node keys annually
```

---

## Network Configuration

### Validator Node Configuration

```bash
#!/bin/bash
# /opt/etrid/start-validator-production.sh

/opt/etrid/flarechain-node \
  --chain flare-mainnet \
  --name "validator-001" \
  --base-path /opt/etrid/data \
  --validator \
  \
  # Network
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  --no-mdns \
  \
  # Security (RPC not publicly accessible)
  --rpc-methods Safe \
  --rpc-max-connections 100 \
  \
  # Performance
  --db-cache 4096 \
  --state-cache-size 1073741824 \
  --pruning 256 \
  --wasm-execution compiled \
  \
  # Network optimization
  --max-parallel-downloads 8 \
  --in-peers 25 \
  --out-peers 25 \
  \
  # Monitoring
  --prometheus-external \
  --prometheus-port 9615 \
  \
  # Logging
  --log info,runtime=warn
```

### Archive Node Configuration

```bash
#!/bin/bash
# /opt/etrid/start-archive-production.sh

/opt/etrid/flarechain-node \
  --chain flare-mainnet \
  --name "archive-001" \
  --base-path /opt/etrid/data \
  \
  # Archive settings
  --pruning archive \
  --db-cache 8192 \
  --state-cache-size 4294967296 \
  \
  # Network
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  \
  # Security
  --rpc-methods Safe \
  --rpc-max-connections 1000 \
  --rpc-cors all \
  \
  # Performance
  --max-parallel-downloads 16 \
  --in-peers 50 \
  --out-peers 50 \
  \
  # Monitoring
  --prometheus-external \
  --prometheus-port 9615
```

### Systemd Service Files

**Validator Service:** `/etc/systemd/system/etrid-validator.service`

```ini
[Unit]
Description=Ëtrid Protocol Validator Node
After=network.target

[Service]
Type=simple
User=etrid
Group=etrid
WorkingDirectory=/opt/etrid
ExecStart=/opt/etrid/start-validator-production.sh
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=etrid-validator

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/etrid/data

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

**Enable and start:**

```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator

# View logs
sudo journalctl -u etrid-validator -f
```

---

## Monitoring & Alerting

### 1. Prometheus Configuration

```yaml
# /opt/etrid/monitoring/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

rule_files:
  - "alerts.yml"

scrape_configs:
  - job_name: 'etrid-validator'
    static_configs:
      - targets: ['localhost:9615']
        labels:
          instance: 'validator-001'
          environment: 'production'
```

### 2. Alert Rules

**File:** `/opt/etrid/monitoring/alerts.yml`

```yaml
groups:
  - name: etrid_critical
    interval: 30s
    rules:
      - alert: NodeDown
        expr: up{job="etrid-validator"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Ëtrid validator node is down"

      - alert: HighMemoryUsage
        expr: process_resident_memory_bytes > 30000000000  # 30GB
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage detected"

      - alert: BlockProductionStalled
        expr: increase(substrate_block_height{status="best"}[5m]) == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Block production has stalled"

      - alert: HighFinalityLag
        expr: substrate_block_height{status="best"} - substrate_block_height{status="finalized"} > 100
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Finality lag is high"

      - alert: LowPeerCount
        expr: substrate_sub_libp2p_peers_count < 5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Low peer count detected"
```

### 3. Grafana Dashboards

Import pre-configured dashboard:

```bash
# Already created at:
cp scripts/testnet/grafana-dashboard.json \
   /var/lib/grafana/dashboards/etrid-production.json

# Restart Grafana
sudo systemctl restart grafana-server
```

### 4. Log Management

```bash
# Install Loki for log aggregation
sudo apt install -y loki promtail

# Configure Promtail to collect node logs
cat > /etc/promtail/config.yml <<EOF
server:
  http_listen_port: 9080

positions:
  filename: /tmp/positions.yaml

clients:
  - url: http://localhost:3100/loki/api/v1/push

scrape_configs:
  - job_name: etrid-validator
    static_configs:
      - targets:
          - localhost
        labels:
          job: etrid-validator
          __path__: /var/log/journal/*
    pipeline_stages:
      - match:
          selector: '{job="etrid-validator"}'
          stages:
            - regex:
                expression: '.*(?P<message>ERROR|WARN|CRITICAL).*'
            - labels:
                level:
EOF

sudo systemctl enable promtail
sudo systemctl start promtail
```

---

## Deployment Procedures

### Step-by-Step Deployment

#### 1. Prepare Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y \
  build-essential \
  git \
  curl \
  wget \
  htop \
  iotop \
  jq

# Create deployment directory
sudo mkdir -p /opt/etrid
sudo chown etrid:etrid /opt/etrid
```

#### 2. Deploy Binary

```bash
# Build on build server or download release
# Build:
cargo build --release -p flarechain-node

# Copy to production server
scp target/release/flarechain-node etrid@prod-server:/opt/etrid/

# Or use pre-built release
wget https://github.com/etrid-protocol/etrid/releases/download/v1.0.0/flarechain-node
chmod +x flarechain-node
sudo mv flarechain-node /opt/etrid/
```

#### 3. Deploy Configuration

```bash
# Copy optimized configs
scp -r config/production etrid@prod-server:/opt/etrid/config/
scp scripts/start-validator-optimized.sh etrid@prod-server:/opt/etrid/start-validator-production.sh

# Set permissions
chmod +x /opt/etrid/*.sh
chmod 600 /opt/etrid/config/*
```

#### 4. Deploy Monitoring

```bash
# Run monitoring stack setup
./scripts/setup-monitoring-stack.sh

# Configure alerts
sudo cp monitoring/alerts.yml /etc/prometheus/
sudo systemctl reload prometheus
```

#### 5. Initialize Node

```bash
# Generate keys
/opt/etrid/flarechain-node key generate --scheme Sr25519 --output-type Json > /opt/etrid/keys/session.json

# Insert keys into node
/opt/etrid/flarechain-node key insert \
  --base-path /opt/etrid/data \
  --chain flare-mainnet \
  --scheme Sr25519 \
  --suri "$(cat /opt/etrid/keys/session.json | jq -r '.secretPhrase')" \
  --key-type gran

# Secure keys
chmod 600 /opt/etrid/keys/*
```

#### 6. Start Node

```bash
# Start with systemd
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator

# Check logs
sudo journalctl -u etrid-validator -f
```

#### 7. Verify Deployment

```bash
# Wait 60 seconds for startup
sleep 60

# Check RPC
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://localhost:9944

# Check metrics
curl http://localhost:9615/metrics | grep substrate_block_height

# Check monitoring
curl http://localhost:9090/api/v1/query?query=up
```

---

## Post-Deployment Validation

### Validation Checklist

```bash
# 1. Node is running
systemctl is-active etrid-validator
# Expected: active

# 2. Node is syncing/producing blocks
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
  | jq .result.number

# 3. Peers connected
curl -s http://localhost:9615/metrics | grep "substrate_sub_libp2p_peers_count "
# Expected: > 5 peers

# 4. Memory usage normal
ps aux | grep flarechain-node | awk '{print $6/1024 " MB"}'
# Expected: < 16 GB

# 5. Prometheus scraping
curl http://localhost:9090/api/v1/targets | jq '.data.activeTargets[] | select(.labels.job=="etrid-validator")'
# Expected: up status

# 6. Grafana accessible
curl http://localhost:3000/api/health
# Expected: 200 OK

# 7. Alerts configured
curl http://localhost:9090/api/v1/rules | jq '.data.groups[].rules[].name'
# Expected: List of alert rules
```

### Performance Validation

```bash
# Run automated validation
./scripts/validate-performance.sh

# Expected: 85%+ pass rate
```

---

## Disaster Recovery

### Backup Procedures

#### 1. Database Backup

```bash
#!/bin/bash
# /opt/etrid/backup-database.sh

BACKUP_DIR="/opt/etrid/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Stop node
sudo systemctl stop etrid-validator

# Create backup
tar -czf "$BACKUP_DIR/database-$TIMESTAMP.tar.gz" \
  /opt/etrid/data/chains

# Start node
sudo systemctl start etrid-validator

# Rotate old backups (keep 7 days)
find "$BACKUP_DIR" -name "database-*.tar.gz" -mtime +7 -delete
```

#### 2. State Snapshot

```bash
# Create state snapshot
/opt/etrid/flarechain-node export-state \
  --chain flare-mainnet \
  --base-path /opt/etrid/data \
  > /opt/etrid/backups/state-snapshot-$(date +%Y%m%d).json
```

#### 3. Key Backup

```bash
# Encrypt and backup keys
tar -czf - /opt/etrid/keys | \
  gpg --symmetric --cipher-algo AES256 \
  > /opt/etrid/backups/keys-$(date +%Y%m%d).tar.gz.gpg

# Store encrypted backup off-site
# DO NOT store unencrypted keys off-site
```

### Recovery Procedures

#### Scenario 1: Node Crash

```bash
# Simply restart
sudo systemctl start etrid-validator

# Node will sync automatically
```

#### Scenario 2: Data Corruption

```bash
# Stop node
sudo systemctl stop etrid-validator

# Restore from backup
tar -xzf /opt/etrid/backups/database-YYYYMMDD_HHMMSS.tar.gz -C /

# Start node
sudo systemctl start etrid-validator
```

#### Scenario 3: Complete Server Failure

```bash
# On new server:
# 1. Install OS and dependencies
# 2. Deploy binary and configs
# 3. Restore database backup
# 4. Restore keys
# 5. Start node

# Detailed steps:
scp etrid@backup-server:/opt/etrid/backups/database-latest.tar.gz .
tar -xzf database-latest.tar.gz -C /

gpg --decrypt keys-latest.tar.gz.gpg | tar -xz -C /opt/etrid/

sudo systemctl start etrid-validator
```

---

## Maintenance & Operations

### Daily Tasks

```bash
# Check node status
sudo systemctl status etrid-validator

# Check resource usage
htop
iotop

# Check logs for errors
sudo journalctl -u etrid-validator --since "1 hour ago" | grep -i error

# Check peer count
curl -s http://localhost:9615/metrics | grep peers_count
```

### Weekly Tasks

```bash
# Review Grafana dashboards
# Check for performance degradation

# Review alerts
# Investigate any warnings

# Check disk space
df -h /opt/etrid/data

# Review backup logs
ls -lh /opt/etrid/backups
```

### Monthly Tasks

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Rotate logs
sudo journalctl --vacuum-time=30d

# Review and update firewall rules
sudo ufw status

# Performance benchmark
./scripts/run-profiling-suite.sh

# Review and rotate keys if needed
```

### Upgrade Procedures

```bash
# 1. Test upgrade on testnet first

# 2. Backup everything
./scripts/backup-all.sh

# 3. Download new binary
wget https://github.com/etrid-protocol/etrid/releases/download/v1.1.0/flarechain-node

# 4. Stop node
sudo systemctl stop etrid-validator

# 5. Replace binary
sudo mv flarechain-node /opt/etrid/flarechain-node
sudo chmod +x /opt/etrid/flarechain-node

# 6. Start node
sudo systemctl start etrid-validator

# 7. Monitor closely
sudo journalctl -u etrid-validator -f

# 8. Validate
./scripts/validate-performance.sh
```

---

## Emergency Contacts

```
Team Lead: [email/phone]
DevOps: [email/phone]
Security: [email/phone]
On-Call Rotation: [schedule]

Discord: https://discord.gg/etrid
GitHub Issues: https://github.com/etrid-protocol/etrid/issues
```

---

## Appendix A: Quick Reference

### Common Commands

```bash
# Start node
sudo systemctl start etrid-validator

# Stop node
sudo systemctl stop etrid-validator

# Restart node
sudo systemctl restart etrid-validator

# View logs
sudo journalctl -u etrid-validator -f

# Check status
./scripts/monitoring-status.sh

# Run diagnostics
./scripts/validate-performance.sh

# Backup
./scripts/backup-all.sh
```

### Important URLs

```
Node RPC: http://localhost:9944
WebSocket: ws://localhost:9945
Prometheus: http://localhost:9090
Grafana: http://localhost:3000
Metrics: http://localhost:9615/metrics
```

---

**Document Version:** 1.0
**Last Review:** October 22, 2025
**Next Review:** January 22, 2026
