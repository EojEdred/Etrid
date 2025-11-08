# Linux x86_64 Build & Deployment Guide

**Target**: Linux x86_64 (Ubuntu/Debian) Validator VMs
**Build Tool**: cargo-zigbuild with Zig 0.15.2
**Status**: ✅ Complete - Ready for Deployment

---

## Build Information

### Cross-Compilation Setup

**Host System**: macOS ARM64 (Apple Silicon)
**Target System**: Linux x86_64 (GNU libc)
**Rust Target**: `x86_64-unknown-linux-gnu`
**Cross-Compiler**: Zig 0.15.2 via cargo-zigbuild

### Build Command

```bash
cargo zigbuild --release -p eth-pbc-collator --target x86_64-unknown-linux-gnu
```

### Build Output Location

```
target/x86_64-unknown-linux-gnu/release/eth-pbc-collator
```

---

## Expected Binary Details

**Size**: ~40-50MB (optimized release build)
**Type**: ELF 64-bit LSB executable, x86-64
**Dependencies**: Dynamically linked against glibc (standard Linux)

### Required Libraries on Target System

```bash
# Standard dependencies (usually pre-installed)
- libc.so.6 (glibc 2.27+)
- libdl.so.2
- librt.so.1
- libpthread.so.0
- libgcc_s.so.1
- libm.so.6
```

Most modern Linux distributions (Ubuntu 18.04+, Debian 10+, CentOS 8+, RHEL 8+) have these libraries pre-installed.

---

## Deployment to Linux VMs

### Step 1: Transfer Binary

**Using SCP**:
```bash
scp target/x86_64-unknown-linux-gnu/release/eth-pbc-collator user@vm-ip:/opt/eth-pbc/
```

**Using rsync** (recommended for multiple files):
```bash
rsync -avz --progress \
  target/x86_64-unknown-linux-gnu/release/eth-pbc-collator \
  user@vm-ip:/opt/eth-pbc/
```

### Step 2: Set Permissions

```bash
ssh user@vm-ip
cd /opt/eth-pbc
chmod +x eth-pbc-collator
```

### Step 3: Verify Binary

```bash
# Check file type
file eth-pbc-collator
# Expected: ELF 64-bit LSB executable, x86-64, ... dynamically linked

# Check dependencies
ldd eth-pbc-collator
# Should show all libraries found

# Test execution
./eth-pbc-collator --help
```

---

## VM System Requirements

### Minimum Specifications

- **CPU**: 4 cores (x86_64)
- **RAM**: 8GB (16GB recommended)
- **Storage**: 100GB SSD minimum
- **OS**: Ubuntu 20.04 LTS or newer, Debian 11+, CentOS 8+, RHEL 8+
- **Network**: Public IP with ports 30333 (P2P) and 9944/9933 (RPC) open

### Recommended Specifications (Validator)

- **CPU**: 8+ cores (x86_64)
- **RAM**: 16-32GB
- **Storage**: 200GB+ NVMe SSD
- **Network**: 100Mbps+ bandwidth, low latency

---

## Installation on Linux VM

### Option 1: Manual Setup

```bash
# Create directory structure
sudo mkdir -p /opt/eth-pbc/{bin,data,keys}
sudo chown -R $USER:$USER /opt/eth-pbc

# Move binary
mv eth-pbc-collator /opt/eth-pbc/bin/

# Create systemd service
sudo nano /etc/systemd/system/eth-pbc.service
```

**Systemd Service File** (`/etc/systemd/system/eth-pbc.service`):
```ini
[Unit]
Description=ETH PBC Collator Node
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=eth-pbc
Group=eth-pbc
WorkingDirectory=/opt/eth-pbc
ExecStart=/opt/eth-pbc/bin/eth-pbc-collator \
  --base-path /opt/eth-pbc/data \
  --validator \
  --name "ETH-PBC-Validator-01" \
  --port 30333 \
  --rpc-port 9933 \
  --rpc-external \
  --rpc-cors all \
  --prometheus-external

Restart=always
RestartSec=10s
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
```

**Enable and Start Service**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable eth-pbc
sudo systemctl start eth-pbc
sudo systemctl status eth-pbc
```

### Option 2: Docker Deployment

```dockerfile
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh eth-pbc && \
    mkdir -p /data /keys && \
    chown -R eth-pbc:eth-pbc /data /keys

# Copy binary
COPY --chown=eth-pbc:eth-pbc eth-pbc-collator /usr/local/bin/

USER eth-pbc
EXPOSE 30333 9933 9944 9615
VOLUME ["/data", "/keys"]

ENTRYPOINT ["/usr/local/bin/eth-pbc-collator"]
CMD ["--help"]
```

**Build and Run**:
```bash
docker build -t eth-pbc-collator .
docker run -d \
  --name eth-pbc \
  -p 30333:30333 \
  -p 9933:9933 \
  -p 9944:9944 \
  -v /opt/eth-pbc/data:/data \
  -v /opt/eth-pbc/keys:/keys \
  eth-pbc-collator:latest \
  --base-path /data \
  --validator \
  --name "ETH-PBC-Validator-Docker"
```

---

## Key Management on Linux VMs

### Generate Keys on VM

```bash
# Generate Aura key (Sr25519)
./eth-pbc-collator key generate --scheme Sr25519

# Generate Grandpa key (Ed25519)
./eth-pbc-collator key generate --scheme Ed25519
```

### Insert Keys into Keystore

```bash
# Insert Aura key
./eth-pbc-collator key insert \
  --base-path /opt/eth-pbc/data \
  --chain local \
  --scheme Sr25519 \
  --suri "your-secret-phrase-here" \
  --key-type aura

# Insert Grandpa key
./eth-pbc-collator key insert \
  --base-path /opt/eth-pbc/data \
  --chain local \
  --scheme Ed25519 \
  --suri "your-grandpa-secret-phrase" \
  --key-type gran
```

---

## Firewall Configuration

### UFW (Ubuntu/Debian)

```bash
# Allow P2P port
sudo ufw allow 30333/tcp

# Allow RPC (if external access needed)
sudo ufw allow 9933/tcp
sudo ufw allow 9944/tcp

# Allow Prometheus metrics (internal only)
sudo ufw allow from 10.0.0.0/8 to any port 9615

sudo ufw enable
```

### firewalld (RHEL/CentOS)

```bash
# P2P port
sudo firewall-cmd --permanent --add-port=30333/tcp

# RPC ports
sudo firewall-cmd --permanent --add-port=9933/tcp
sudo firewall-cmd --permanent --add-port=9944/tcp

# Prometheus (internal only)
sudo firewall-cmd --permanent --add-rich-rule='rule family="ipv4" source address="10.0.0.0/8" port port="9615" protocol="tcp" accept'

sudo firewall-cmd --reload
```

---

## Monitoring & Logs

### View Logs (Systemd)

```bash
# Follow logs
sudo journalctl -u eth-pbc -f

# Last 100 lines
sudo journalctl -u eth-pbc -n 100

# Today's logs
sudo journalctl -u eth-pbc --since today
```

### Prometheus Metrics

Access metrics at: `http://vm-ip:9615/metrics`

**Key Metrics**:
- `substrate_block_height` - Current block number
- `substrate_peers` - Number of connected peers
- `substrate_ready_transactions_number` - Transactions in pool
- `substrate_sync_validation_total` - Validation statistics

### Health Checks

```bash
# Check if node is syncing
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://localhost:9933/

# Check node peers
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://localhost:9933/

# Check sync status
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_syncState"}' \
  http://localhost:9933/
```

---

## Ethereum RPC Compatibility

The ETH PBC supports Ethereum JSON-RPC for wallet/dApp integration:

### Standard Ethereum RPC Endpoints

```bash
# Get chain ID
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"eth_chainId"}' \
  http://localhost:9933/

# Get latest block
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"eth_blockNumber"}' \
  http://localhost:9933/

# Get account balance
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"eth_getBalance", "params":["0xYourAddress", "latest"]}' \
  http://localhost:9933/
```

### MetaMask Configuration

```
Network Name: ETH PBC
RPC URL: http://vm-ip:9933
Chain ID: [from eth_chainId]
Currency Symbol: ETR
Block Explorer: N/A (or custom)
```

---

## Backup & Recovery

### Backup Important Data

```bash
# Backup keystore
sudo tar -czf eth-pbc-keys-$(date +%Y%m%d).tar.gz /opt/eth-pbc/keys/

# Backup chain database (optional, can resync)
sudo tar -czf eth-pbc-data-$(date +%Y%m%d).tar.gz /opt/eth-pbc/data/

# Copy to safe location
scp eth-pbc-keys-*.tar.gz backup-server:/backups/
```

### Recovery

```bash
# Restore keys
sudo tar -xzf eth-pbc-keys-YYYYMMDD.tar.gz -C /

# Restore data (if needed)
sudo tar -xzf eth-pbc-data-YYYYMMDD.tar.gz -C /

# Fix permissions
sudo chown -R eth-pbc:eth-pbc /opt/eth-pbc/
```

---

## Upgrade Procedure

### Zero-Downtime Upgrade (for multiple validators)

1. Build new binary with fixes/updates
2. Transfer to staging VM first
3. Test on staging
4. Rolling upgrade across validators:
   ```bash
   # For each validator
   sudo systemctl stop eth-pbc
   sudo cp /opt/eth-pbc/bin/eth-pbc-collator{,.backup}
   sudo cp new-eth-pbc-collator /opt/eth-pbc/bin/eth-pbc-collator
   sudo systemctl start eth-pbc
   # Verify sync before proceeding to next validator
   ```

---

## Troubleshooting

### Binary Won't Execute

```bash
# Check if it's the right architecture
file eth-pbc-collator
# Should show: x86-64

# Check dependencies
ldd eth-pbc-collator
# All libraries should show "=> /lib/..." (found)

# Check permissions
chmod +x eth-pbc-collator
```

### Missing Library Errors

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libc6

# RHEL/CentOS
sudo yum install -y glibc
```

### Port Already in Use

```bash
# Find process using port
sudo lsof -i :30333
sudo lsof -i :9933

# Kill if needed
sudo kill -9 <PID>
```

---

## Production Checklist

- [ ] Binary transferred to all VMs
- [ ] Systemd service configured
- [ ] Firewall rules applied
- [ ] Keys generated and inserted
- [ ] Monitoring/metrics accessible
- [ ] Logs rotating properly (`logrotate` configured)
- [ ] Backup cron job set up
- [ ] Health checks configured
- [ ] Alert system set up (Prometheus/Grafana/PagerDuty)
- [ ] Documentation updated with VM IPs and ports

---

## Security Best Practices

1. **Never expose RPC ports to public internet** without authentication
2. **Use reverse proxy (nginx/caddy)** with SSL for external RPC access
3. **Restrict SSH access** - key-based auth only, no root login
4. **Keep system updated**: `sudo apt update && sudo apt upgrade -y`
5. **Backup keys** to encrypted, offline storage
6. **Monitor logs** for suspicious activity
7. **Use separate user** for running the collator (not root)
8. **Enable fail2ban** for SSH protection
9. **Rotate keys periodically** (session keys)
10. **Audit permissions** regularly

---

## Support & Resources

- **Build Log**: `/tmp/eth-pbc-linux-build.log` (on build machine)
- **Runtime Docs**: `ETH_PBC_BUILD_FIXES.md`
- **Substrate Docs**: https://docs.substrate.io
- **Polkadot Validator**: https://wiki.polkadot.network/docs/maintain-guides-how-to-validate

---

**Last Updated**: November 4, 2025
**Build Status**: ✅ Complete

---

## Build Results

**Build completed successfully!**

- **Build Time**: 18 minutes 33 seconds
- **Binary Location**: `target/x86_64-unknown-linux-gnu/release/eth-pbc-collator`
- **Binary Size**: 43MB
- **Binary Format**: ELF 64-bit LSB pie executable, x86-64
- **Deployment Package**: `eth-pbc-linux-x86_64-20251104.tar.gz` (18MB compressed)
- **SHA256**: `650c8c1480afba29c09be08102a636ec73b4be59f18f48a90173d80135ac4f71`

**Package Contents**:
- `eth-pbc-collator` - Linux x86_64 binary
- `LINUX_BUILD_DEPLOYMENT.md` - This deployment guide
- `ETH_PBC_BUILD_FIXES.md` - Build documentation

**Ready for deployment** to Linux validator VMs (Ubuntu 18.04+, Debian 10+, RHEL 8+, CentOS 8+).
