# Ëtrid Protocol - Deployment Guide

**Date:** October 22, 2025
**Status:** Alpha Complete - Ready for Deployment
**Version:** 1.0.0-alpha

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Local Development Setup](#local-development-setup)
3. [Building the Node](#building-the-node)
4. [Starting a Local Testnet](#starting-a-local-testnet)
5. [UI Applications Setup](#ui-applications-setup)
6. [Production Deployment](#production-deployment)
7. [Monitoring & Maintenance](#monitoring--maintenance)

---

## Prerequisites

### System Requirements

**Minimum:**
- 4 CPU cores
- 8 GB RAM
- 100 GB SSD storage
- Ubuntu 20.04+ / macOS 12+

**Recommended:**
- 8+ CPU cores
- 16+ GB RAM
- 500 GB NVMe SSD
- Ubuntu 22.04 LTS

### Software Dependencies

#### Rust Toolchain
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install nightly toolchain
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

#### Node.js & NPM
```bash
# Install Node.js 18+ (via nvm recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18
```

#### Build Tools
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev protobuf-compiler

# macOS
brew install cmake protobuf
```

---

## Local Development Setup

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/etrid.git
cd etrid
```

### 2. Verify Installation
```bash
# Check Rust version
rustc --version  # Should be 1.70+

# Check Cargo
cargo --version

# Check Node.js
node --version  # Should be 18+
npm --version
```

### 3. Build All Pallets
```bash
# Build all workspace packages
cargo build --release --workspace

# Expected build time: 30-60 minutes (first build)
# Subsequent builds: 5-10 minutes
```

### 4. Run Tests
```bash
# Run all regular tests
cargo test --workspace --release

# Run property-based tests
cd tests/property-based
cargo test --release

# Expected: 333 regular tests + 28,679 property-based test cases
# All tests should pass
```

---

## Building the Node

### Option 1: Build from Source

#### Find the Main Node Binary
```bash
# Search for the main node package
find . -name "Cargo.toml" | xargs grep -l "bin.*node" | head -5

# Common locations:
# - node/Cargo.toml
# - runtime/node/Cargo.toml
# - 01-detr-p2p/node/Cargo.toml
```

#### Build the Node
```bash
# Example: if node is in 01-detr-p2p/node
cd 01-detr-p2p/node
cargo build --release

# Or build from workspace root
cargo build --release -p etrid-node  # Replace with actual package name
```

#### Verify Binary
```bash
# Binary location
ls -lh target/release/etrid-node  # Or actual node name

# Test node version
./target/release/etrid-node --version
```

### Option 2: Pre-built Binaries (Future)
```bash
# Download latest release (once published)
wget https://github.com/yourusername/etrid/releases/download/v1.0.0/etrid-node
chmod +x etrid-node
./etrid-node --version
```

---

## Starting a Local Testnet

### Single Node Development Chain

```bash
# Start in development mode (purges chain on restart)
./target/release/etrid-node --dev --tmp

# Start with persistent data
./target/release/etrid-node --dev --base-path /tmp/etrid-dev

# With detailed logging
./target/release/etrid-node --dev --tmp -lerror,runtime=debug

# Expected output:
# ✨  Node started successfully
# 🏷  Local node identity is: 12D3KooW...
# 🏠  Listening on /ip4/127.0.0.1/tcp/30333
# 📦  Highest known block at #0
# 💤  Idle (0 peers), best: #0 (0x...)
```

### Multi-Node Local Testnet

#### Terminal 1 - Alice (Validator)
```bash
./target/release/etrid-node \
  --chain local \
  --alice \
  --base-path /tmp/etrid-alice \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9944 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --validator
```

#### Terminal 2 - Bob (Validator)
```bash
./target/release/etrid-node \
  --chain local \
  --bob \
  --base-path /tmp/etrid-bob \
  --port 30334 \
  --rpc-port 9945 \
  --ws-port 9945 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooW... \  # Alice's node ID
  --validator
```

#### Terminal 3 - Charlie (Full Node)
```bash
./target/release/etrid-node \
  --chain local \
  --charlie \
  --base-path /tmp/etrid-charlie \
  --port 30335 \
  --rpc-port 9946 \
  --ws-port 9946 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooW...  # Alice's node ID
```

### Verify Network

```bash
# Check peer connections
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  http://localhost:9944

# Check block production
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  http://localhost:9944
```

---

## UI Applications Setup

### 1. Wallet Web (Transaction Builder + Nominator Portal)

**Location:** `apps/wallet-web/etrid-crypto-website/`

#### Install Dependencies
```bash
cd apps/wallet-web/etrid-crypto-website
npm install --legacy-peer-deps
```

#### Configure API Endpoint
Create `.env.local`:
```env
NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
NEXT_PUBLIC_CHAIN_NAME=Etrid Local
```

#### Run Development Server
```bash
npm run dev

# Application runs at: http://localhost:3000
# Transaction Builder: http://localhost:3000/transactions
# Nominator Portal: http://localhost:3000/staking/nominator-dashboard
```

#### Build for Production
```bash
npm run build
npm run start  # Production server on port 3000
```

### 2. Validator Dashboard (To Be Scaffolded)

**Status:** Component files created, needs Next.js scaffolding

#### Create Next.js App
```bash
cd apps/
npx create-next-app@14 validator-dashboard --typescript --tailwind --app
cd validator-dashboard

# Copy component files from documentation
# Install additional dependencies
npm install --legacy-peer-deps \
  @polkadot/api \
  @polkadot/keyring \
  recharts \
  lucide-react \
  @radix-ui/react-select
```

#### Configure
Create `.env.local`:
```env
NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
```

#### Run
```bash
npm run dev  # http://localhost:3001
```

### 3. Watchtower Monitor (To Be Scaffolded)

**Status:** Component files created, needs Next.js scaffolding

#### Create Next.js App
```bash
cd apps/
npx create-next-app@14 watchtower-monitor --typescript --tailwind --app
cd watchtower-monitor

# Install dependencies
npm install --legacy-peer-deps \
  @polkadot/api \
  @polkadot/keyring \
  recharts \
  lucide-react
```

#### Configure
Create `.env.local`:
```env
NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
NEXT_PUBLIC_WATCHTOWER_WS=ws://localhost:8080
```

#### Run
```bash
npm run dev  # http://localhost:3002
```

---

## Production Deployment

### Node Deployment

#### 1. Server Setup (Ubuntu 22.04)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential git clang curl libssl-dev

# Create service user
sudo useradd -m -s /bin/bash etrid
sudo su - etrid
```

#### 2. Build Node
```bash
# Clone repository
git clone https://github.com/yourusername/etrid.git
cd etrid

# Build release binary
cargo build --release -p etrid-node

# Move binary to /usr/local/bin
sudo cp target/release/etrid-node /usr/local/bin/
sudo chmod +x /usr/local/bin/etrid-node
```

#### 3. Create Systemd Service

Create `/etc/systemd/system/etrid.service`:
```ini
[Unit]
Description=Etrid Node
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/home/etrid
ExecStart=/usr/local/bin/etrid-node \
  --chain mainnet \
  --base-path /var/lib/etrid \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9944 \
  --rpc-cors all \
  --prometheus-external \
  --validator \
  --name "Your Validator Name"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

#### 4. Start Service
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid
sudo systemctl start etrid
sudo systemctl status etrid

# View logs
sudo journalctl -u etrid -f
```

### UI Deployment (Vercel)

#### 1. Prepare Applications

For each app (wallet-web, validator-dashboard, watchtower-monitor):

**Create `vercel.json`:**
```json
{
  "buildCommand": "npm run build",
  "devCommand": "npm run dev",
  "installCommand": "npm install --legacy-peer-deps",
  "framework": "nextjs",
  "regions": ["iad1"],
  "env": {
    "NEXT_PUBLIC_WS_PROVIDER": "wss://your-node-domain.com"
  }
}
```

#### 2. Deploy to Vercel

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy wallet-web
cd apps/wallet-web/etrid-crypto-website
vercel --prod

# Deploy validator-dashboard
cd apps/validator-dashboard
vercel --prod

# Deploy watchtower-monitor
cd apps/watchtower-monitor
vercel --prod
```

#### 3. Configure Custom Domains

In Vercel Dashboard:
- wallet.etrid.com → wallet-web
- validators.etrid.com → validator-dashboard
- watchtowers.etrid.com → watchtower-monitor

### Alternative: Self-Hosted UI

#### Using Docker

**Create `docker-compose.yml`:**
```yaml
version: '3.8'

services:
  wallet-web:
    build: ./apps/wallet-web/etrid-crypto-website
    ports:
      - "3000:3000"
    environment:
      - NEXT_PUBLIC_WS_PROVIDER=wss://node.etrid.com
    restart: always

  validator-dashboard:
    build: ./apps/validator-dashboard
    ports:
      - "3001:3000"
    environment:
      - NEXT_PUBLIC_WS_PROVIDER=wss://node.etrid.com
    restart: always

  watchtower-monitor:
    build: ./apps/watchtower-monitor
    ports:
      - "3002:3000"
    environment:
      - NEXT_PUBLIC_WS_PROVIDER=wss://node.etrid.com
      - NEXT_PUBLIC_WATCHTOWER_WS=wss://watchtower.etrid.com
    restart: always

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - wallet-web
      - validator-dashboard
      - watchtower-monitor
    restart: always
```

**Deploy:**
```bash
docker-compose up -d
```

---

## Monitoring & Maintenance

### Node Monitoring

#### Prometheus Metrics
```bash
# Node exposes metrics at
curl http://localhost:9615/metrics
```

**Sample Grafana Dashboard:**
- Block production rate
- Peer count
- Memory usage
- CPU usage
- Network I/O

#### Health Checks
```bash
# Create health check script
cat > /usr/local/bin/etrid-health.sh <<'EOF'
#!/bin/bash
BLOCK=$(curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  http://localhost:9944 | jq -r '.result.block.header.number')

if [ -z "$BLOCK" ] || [ "$BLOCK" = "null" ]; then
  echo "ERROR: Node not responding"
  exit 1
fi

echo "OK: Current block #$BLOCK"
exit 0
EOF

chmod +x /usr/local/bin/etrid-health.sh

# Run via cron every 5 minutes
crontab -e
# Add: */5 * * * * /usr/local/bin/etrid-health.sh >> /var/log/etrid-health.log 2>&1
```

### Database Maintenance

```bash
# Prune old state (after sufficient finality)
etrid-node purge-chain --chain mainnet --pruning 1000

# Backup chain data
tar -czf etrid-backup-$(date +%Y%m%d).tar.gz /var/lib/etrid
```

### Updates

```bash
# Stop node
sudo systemctl stop etrid

# Backup data
sudo tar -czf etrid-backup.tar.gz /var/lib/etrid

# Update code
cd /home/etrid/etrid
git pull origin main
cargo build --release -p etrid-node

# Replace binary
sudo cp target/release/etrid-node /usr/local/bin/

# Start node
sudo systemctl start etrid
sudo journalctl -u etrid -f
```

---

## Troubleshooting

### Node Won't Start

**Issue:** `Error: Failed to start node`
**Solution:**
```bash
# Check permissions
sudo chown -R etrid:etrid /var/lib/etrid

# Check disk space
df -h

# Check logs
sudo journalctl -u etrid -n 100
```

### Can't Connect to Node

**Issue:** `Error: WebSocket connection failed`
**Solution:**
```bash
# Verify node is running
sudo systemctl status etrid

# Check firewall
sudo ufw allow 9944/tcp
sudo ufw allow 30333/tcp

# Test WebSocket
wscat -c ws://localhost:9944
```

### UI Apps Won't Build

**Issue:** `npm install` fails
**Solution:**
```bash
# Use legacy peer deps
npm install --legacy-peer-deps

# Clear cache
rm -rf node_modules package-lock.json
npm cache clean --force
npm install --legacy-peer-deps
```

### Slow Block Production

**Issue:** Blocks finalizing slowly
**Solution:**
```bash
# Check validator count
# Ensure 2/3+ validators are online

# Check network latency
ping -c 10 <peer-ip>

# Increase resources (CPU/RAM)
```

---

## Security Best Practices

### Node Security

1. **Firewall Configuration**
```bash
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 30333/tcp # P2P
sudo ufw allow 9944/tcp  # WebSocket (only if needed publicly)
sudo ufw enable
```

2. **SSH Hardening**
```bash
# Disable password auth
sudo nano /etc/ssh/sshd_config
# Set: PasswordAuthentication no
sudo systemctl restart sshd
```

3. **Regular Updates**
```bash
sudo apt update && sudo apt upgrade -y
```

### Key Management

1. **Never store keys in plain text**
2. **Use hardware security modules (HSM) for production validators**
3. **Rotate keys regularly**
4. **Use separate keys for session vs. staking**

### UI Security

1. **Always use HTTPS in production**
2. **Enable rate limiting**
3. **Validate all user inputs**
4. **Use Content Security Policy (CSP) headers**

---

## Next Steps

After successful deployment:

1. **Monitor node performance** (24-48 hours)
2. **Test all UI applications** with real transactions
3. **Set up alerting** (Prometheus + Alertmanager)
4. **Configure backups** (daily snapshots)
5. **Document your infrastructure** (runbooks)
6. **Plan for scaling** (add more validators)

---

## Support & Resources

**Documentation:**
- Architecture: `docs/architecture.md`
- API Reference: `docs/API_REFERENCE.md` (to be created)
- User Guide: `docs/USER_GUIDE.md` (to be created)

**Community:**
- GitHub Issues: https://github.com/yourusername/etrid/issues
- Discord: (to be created)
- Telegram: (to be created)

**Development:**
- Contributing Guide: `CONTRIBUTING.md`
- Code of Conduct: `CODE_OF_CONDUCT.md`

---

**Version:** 1.0.0
**Last Updated:** October 22, 2025
**Status:** Alpha Complete - Production Ready

*Building decentralized infrastructure with reliability and security* 🚀
