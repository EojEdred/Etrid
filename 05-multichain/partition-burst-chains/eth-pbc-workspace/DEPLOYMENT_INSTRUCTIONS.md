# ETH PBC Linux Deployment Instructions

**Generated**: November 4, 2025
**Package**: `eth-pbc-linux-x86_64-20251104.tar.gz`
**Status**: ✅ Ready for Deployment

---

## Build Summary

### Artifacts
- **Binary**: `eth-pbc-collator` (43MB uncompressed)
- **Package**: `eth-pbc-linux-x86_64-20251104.tar.gz` (18MB compressed)
- **Target**: Linux x86_64 (Ubuntu 18.04+, Debian 10+, RHEL 8+)
- **Build Type**: Release (full optimizations)
- **Build Time**: 18 minutes 33 seconds

### Binary Verification
```
Type: ELF 64-bit LSB pie executable, x86-64
Interpreter: /lib64/ld-linux-x86-64.so.2
Linking: Dynamic (glibc)
Size: 43MB
Format: GNU/Linux 2.0.0
```

### Wasm Runtime
- **Compressed**: 686KB (production-ready)
- **Compact**: 2.5MB
- **Full**: 2.6MB (with debug info)

---

## Quick Deployment to Linux VM

### Step 1: Transfer Package

**From Build Machine** (macOS):
```bash
# Transfer to VM
scp eth-pbc-linux-x86_64-20251104.tar.gz user@vm-ip:/tmp/

# Or using rsync
rsync -avz --progress eth-pbc-linux-x86_64-20251104.tar.gz user@vm-ip:/tmp/
```

### Step 2: Extract and Install on VM

**On Linux VM**:
```bash
# Create installation directory
sudo mkdir -p /opt/eth-pbc
cd /opt/eth-pbc

# Extract package
sudo tar -xzf /tmp/eth-pbc-linux-x86_64-20251104.tar.gz

# Verify binary
file eth-pbc-collator
# Expected: ELF 64-bit LSB pie executable, x86-64

# Check dependencies (all should be found)
ldd eth-pbc-collator

# Test execution
./eth-pbc-collator --version
./eth-pbc-collator --help
```

### Step 3: Set Permissions

```bash
sudo chmod +x /opt/eth-pbc/eth-pbc-collator
sudo chown -R root:root /opt/eth-pbc
```

---

## System Requirements

### Minimum for Testing
- **CPU**: 2 cores (x86_64)
- **RAM**: 4GB
- **Storage**: 50GB SSD
- **OS**: Ubuntu 20.04 LTS, Debian 11+, RHEL 8+

### Recommended for Validators
- **CPU**: 8+ cores (x86_64)
- **RAM**: 16-32GB
- **Storage**: 200GB+ NVMe SSD
- **Network**: 100Mbps+ bandwidth, public IP
- **Ports**: 30333 (P2P), 9933/9944 (RPC), 9615 (Metrics)

---

## Production Deployment

### Option 1: Systemd Service (Recommended)

**Create service user**:
```bash
sudo useradd -m -r -s /bin/bash eth-pbc
sudo mkdir -p /opt/eth-pbc/{bin,data,keys}
sudo mv eth-pbc-collator /opt/eth-pbc/bin/
sudo chown -R eth-pbc:eth-pbc /opt/eth-pbc
```

**Create systemd service** (`/etc/systemd/system/eth-pbc.service`):
```ini
[Unit]
Description=ETH PBC Collator Node
After=network-online.target
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

**Enable and start**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable eth-pbc
sudo systemctl start eth-pbc
sudo systemctl status eth-pbc
```

### Option 2: Docker Deployment

**Dockerfile** (included in docs):
```dockerfile
FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -U -s /bin/sh eth-pbc && \
    mkdir -p /data /keys && \
    chown -R eth-pbc:eth-pbc /data /keys

COPY --chown=eth-pbc:eth-pbc eth-pbc-collator /usr/local/bin/

USER eth-pbc
EXPOSE 30333 9933 9944 9615
VOLUME ["/data", "/keys"]

ENTRYPOINT ["/usr/local/bin/eth-pbc-collator"]
CMD ["--help"]
```

**Run container**:
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

## Key Generation

**On the Linux VM**:
```bash
# Generate Aura session key (Sr25519)
./eth-pbc-collator key generate --scheme Sr25519

# Generate Grandpa finality key (Ed25519)
./eth-pbc-collator key generate --scheme Ed25519

# Insert keys into keystore
./eth-pbc-collator key insert \
  --base-path /opt/eth-pbc/data \
  --chain local \
  --scheme Sr25519 \
  --suri "your-secret-phrase-here" \
  --key-type aura

./eth-pbc-collator key insert \
  --base-path /opt/eth-pbc/data \
  --chain local \
  --scheme Ed25519 \
  --suri "your-grandpa-secret-phrase" \
  --key-type gran
```

**⚠️ Security**: Never commit or share secret phrases. Store them in encrypted, offline backups.

---

## Firewall Configuration

### UFW (Ubuntu/Debian)
```bash
sudo ufw allow 30333/tcp comment 'ETH PBC P2P'
sudo ufw allow 9933/tcp comment 'ETH PBC RPC HTTP'
sudo ufw allow 9944/tcp comment 'ETH PBC RPC WS'
sudo ufw allow from 10.0.0.0/8 to any port 9615 comment 'Metrics internal'
sudo ufw enable
```

### firewalld (RHEL/CentOS)
```bash
sudo firewall-cmd --permanent --add-port=30333/tcp
sudo firewall-cmd --permanent --add-port=9933/tcp
sudo firewall-cmd --permanent --add-port=9944/tcp
sudo firewall-cmd --reload
```

---

## Monitoring & Health Checks

### View Logs
```bash
# Follow logs
sudo journalctl -u eth-pbc -f

# Last 100 lines
sudo journalctl -u eth-pbc -n 100

# Today's logs
sudo journalctl -u eth-pbc --since today
```

### Health Check RPC
```bash
# Node health
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://localhost:9933/

# Node peers
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://localhost:9933/

# Sync status
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_syncState"}' \
  http://localhost:9933/
```

### Prometheus Metrics
Access at: `http://vm-ip:9615/metrics`

**Key metrics**:
- `substrate_block_height` - Current block
- `substrate_peers` - Connected peers
- `substrate_ready_transactions_number` - Transaction pool size

---

## Package Contents

The deployment package includes:

1. **eth-pbc-collator** - The Linux x86_64 binary (43MB)
2. **LINUX_BUILD_DEPLOYMENT.md** - Full deployment guide
3. **ETH_PBC_BUILD_FIXES.md** - Build documentation and architecture

---

## Verification Checklist

Before deploying to production:

- [ ] Binary is executable on Linux x86_64
- [ ] All dependencies resolved (`ldd eth-pbc-collator`)
- [ ] CLI commands work (`--help`, `--version`)
- [ ] Keys generated and inserted
- [ ] Systemd service configured
- [ ] Firewall rules applied
- [ ] Monitoring/metrics accessible
- [ ] Backup strategy in place

---

## Troubleshooting

### Binary won't execute
```bash
# Check architecture
file eth-pbc-collator
# Should show: x86-64

# Check dependencies
ldd eth-pbc-collator
# All libraries should be found

# Add execute permission
chmod +x eth-pbc-collator
```

### Missing library errors
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libc6

# RHEL/CentOS
sudo yum install -y glibc
```

### Port already in use
```bash
# Find process
sudo lsof -i :30333
sudo lsof -i :9933

# Kill if needed
sudo kill -9 <PID>
```

---

## Support Resources

- **Full Deployment Guide**: `LINUX_BUILD_DEPLOYMENT.md`
- **Build Documentation**: `ETH_PBC_BUILD_FIXES.md`
- **Substrate Docs**: https://docs.substrate.io
- **Polkadot Validator Guide**: https://wiki.polkadot.network/docs/maintain-guides-how-to-validate

---

## Security Notes

1. **Never expose RPC ports** (9933/9944) to public internet without authentication
2. **Use reverse proxy** (nginx/caddy) with SSL for external RPC access
3. **Restrict SSH access** - key-based auth only, no root login
4. **Keep system updated**: `sudo apt update && sudo apt upgrade -y`
5. **Backup keys** to encrypted, offline storage
6. **Monitor logs** for suspicious activity
7. **Use dedicated user** (not root) to run the collator
8. **Enable fail2ban** for SSH protection

---

**Deployment Package Created**: November 4, 2025
**Cross-compiled from**: macOS ARM64 → Linux x86_64
**Build Tool**: cargo-zigbuild with Zig 0.15.2
**Ready for**: Ubuntu 18.04+, Debian 10+, RHEL 8+, CentOS 8+
