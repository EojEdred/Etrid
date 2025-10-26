# Relayer Service Deployment

Guide for deploying permissionless relayer services for EDSC bridge testnet.

## Overview

Relayers are **permissionless** - anyone can run them! They fetch signed attestations and submit to destination chains, earning transaction fees.

**Recommended**: 2-3 relayers for redundancy

```
Relayer #1 → Polls attesters → Submits to chains
Relayer #2 → Polls attesters → Submits to chains
Relayer #3 → Polls attesters → Submits to chains
```

All relayers compete to relay messages. The first to succeed gets the transaction included.

## Prerequisites

### Infrastructure

- **1 server**: 2 CPU, 4GB RAM, 50GB SSD
- **Ubuntu 22.04 LTS**
- **Node.js 18+**
- **PM2** or systemd

### Funded Accounts

**Ethereum (Sepolia)**:
- Private key with ~0.5 ETH
- For paying gas fees

**Substrate (Ember Testnet)**:
- Private key with ~100 EDSC
- For paying transaction fees

### Attestation Service URLs

List of all 5 attestation services:
```
https://attestation-0.etrid.io
https://attestation-1.etrid.io
https://attestation-2.etrid.io
https://attestation-3.etrid.io
https://attestation-4.etrid.io
```

## Server Setup

### 1. Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Node.js 18
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Install PM2
sudo npm install -g pm2

# Install build essentials
sudo apt install -y build-essential git
```

### 2. Create Service User

```bash
sudo useradd -m -s /bin/bash relayer
sudo su - relayer
```

### 3. Clone Repository

```bash
cd ~
git clone https://github.com/etrid/etrid.git
cd etrid/services/relayer-service
```

### 4. Install Dependencies

```bash
npm install
npm run build
```

## Configuration

Create `.env`:

```bash
# Attestation Services (comma-separated)
ATTESTATION_SERVICE_URLS=https://attestation-0.etrid.io,https://attestation-1.etrid.io,https://attestation-2.etrid.io,https://attestation-3.etrid.io,https://attestation-4.etrid.io

# Chain Connections
SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
ETHEREUM_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR-API-KEY

# Relayer Identity
RELAYER_ADDRESS=0xYourRelayerAddress
RELAYER_PRIVATE_KEY=0xYourRelayerPrivateKey

# Contract Addresses (from Ethereum deployment)
MESSAGE_TRANSMITTER_ADDRESS=0xMessageTransmitterAddress
TOKEN_MESSENGER_ADDRESS=0xTokenMessengerAddress

# Polling Settings
POLL_INTERVAL_MS=30000    # Poll every 30 seconds
MAX_RETRIES=3
RETRY_DELAY_MS=60000      # Wait 1 minute between retries

# Ethereum Gas Settings
GAS_LIMIT=500000
MAX_FEE_PER_GAS=50        # gwei
MAX_PRIORITY_FEE_PER_GAS=2  # gwei

# API (optional)
ENABLE_API=false
API_PORT=3001

# Logging
LOG_LEVEL=info
```

### Key Configuration Notes

**Polling**:
- `POLL_INTERVAL_MS=30000`: Check every 30 seconds
- Lower = faster relays, higher API load
- Recommended: 15-60 seconds

**Gas Prices**:
- `MAX_FEE_PER_GAS`: Your maximum gas price
- `MAX_PRIORITY_FEE_PER_GAS`: Tip to validators
- Adjust based on network conditions

**Retries**:
- Failed relays are retried up to `MAX_RETRIES` times
- Wait `RETRY_DELAY_MS` between attempts

## Deployment

### Method 1: PM2 (Recommended)

```bash
# As relayer user
cd ~/etrid/services/relayer-service

# Start with PM2
pm2 start dist/index.js --name relayer-service

# Save configuration
pm2 save

# Setup auto-start on boot
pm2 startup
# Follow instructions shown

# View logs
pm2 logs relayer-service

# Monitor
pm2 monit
```

### Method 2: Systemd

Create `/etc/systemd/system/relayer-service.service`:

```ini
[Unit]
Description=EDSC Relayer Service
After=network.target

[Service]
Type=simple
User=relayer
WorkingDirectory=/home/relayer/etrid/services/relayer-service
EnvironmentFile=/home/relayer/etrid/services/relayer-service/.env
ExecStart=/usr/bin/node dist/index.js
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Start service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable relayer-service
sudo systemctl start relayer-service

# Check status
sudo systemctl status relayer-service

# View logs
sudo journalctl -u relayer-service -f
```

## Verification

### 1. Check Service is Running

```bash
# PM2
pm2 list

# Systemd
sudo systemctl status relayer-service
```

### 2. Check Logs

Look for successful startup:

```bash
pm2 logs relayer-service --lines 50
```

Expected output:
```
Starting EDSC Relayer Service
Configuration loaded
Connected to Ethereum
Connected to Substrate
Attestation fetcher started
Relayer service started successfully
Relayer balances: ethereum=500000000000000000, substrate=100000000000000000000
```

### 3. Check Balances

Ensure relayer has funds:

```bash
# In logs, look for:
Relayer balances: ethereum=..., substrate=...
```

If low, fund accounts:
- Ethereum: Get Sepolia ETH from faucet
- Substrate: Request from faucet channel

### 4. Test Relay

Perform a test transfer and watch logs:

```bash
# Watch for:
pm2 logs relayer-service | grep "Relaying to"

# Should see:
Relaying to Ethereum: messageHash=0x...
Successfully relayed to Ethereum: txHash=0x...
```

## Monitoring

### Service Health

```bash
# Check if running
pm2 status relayer-service

# Check uptime
pm2 info relayer-service

# Check resource usage
pm2 monit
```

### Relay Statistics

Logs show statistics every 5 minutes:

```json
{
  "uptime": 300000,
  "relays": {
    "total": 42,
    "pending": 0,
    "success": 40,
    "failed": 2,
    "byDestination": {
      "0": { "total": 20, "success": 19, "failed": 1 },
      "2": { "total": 22, "success": 21, "failed": 1 }
    }
  },
  "ethereum": {
    "isConnected": true,
    "totalRelays": 22,
    "successfulRelays": 21,
    "failedRelays": 1
  },
  "substrate": {
    "isConnected": true,
    "totalRelays": 20,
    "successfulRelays": 19,
    "failedRelays": 1
  }
}
```

### Balance Monitoring

Set up alerts for low balances:

```bash
# Create monitoring script
cat > ~/monitor-balance.sh << 'EOF'
#!/bin/bash

# Check Ethereum balance
ETH_BALANCE=$(curl -s -X POST $ETHEREUM_RPC_URL \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["'$RELAYER_ADDRESS'","latest"],"id":1}' \
  | jq -r '.result')

# Convert to ETH
ETH_BALANCE_DEC=$(echo "ibase=16; ${ETH_BALANCE:2}" | bc)
ETH_BALANCE_ETH=$(echo "scale=18; $ETH_BALANCE_DEC / 1000000000000000000" | bc)

if (( $(echo "$ETH_BALANCE_ETH < 0.1" | bc -l) )); then
  echo "⚠️  Low ETH balance: $ETH_BALANCE_ETH ETH"
  # Send alert (email, Slack, etc.)
fi
EOF

chmod +x ~/monitor-balance.sh

# Add to cron (every hour)
crontab -e
# Add: 0 * * * * /home/relayer/monitor-balance.sh
```

## Troubleshooting

### Service Won't Start

```bash
# Check logs
pm2 logs relayer-service --err

# Check environment
cat .env

# Check dependencies
npm install

# Check Node.js version
node --version  # Should be 18+
```

### No Relays Happening

**Check attestation services**:

```bash
# Test each attestation service
for i in {0..4}; do
  echo "Checking attester $i..."
  curl -s https://attestation-$i.etrid.io/health | jq '.status'
done
```

**Check ready attestations**:

```bash
curl https://attestation-0.etrid.io/attestations/ready | jq
```

**Check logs for errors**:

```bash
pm2 logs relayer-service | grep -i error
```

### Failed Relays

**Check gas settings**:
- Increase `MAX_FEE_PER_GAS` if gas price too low
- Check Sepolia gas tracker: https://sepolia.etherscan.io/gastracker

**Check account balance**:
- Ensure sufficient ETH for gas
- Ensure sufficient EDSC for Substrate tx fees

**Check contract addresses**:
- Verify `MESSAGE_TRANSMITTER_ADDRESS` is correct
- Verify contracts deployed on Sepolia

### High Failure Rate

```bash
# Check stats
pm2 logs relayer-service | grep "Service statistics"

# If many failures:
# 1. Check if another relayer is faster
# 2. Reduce POLL_INTERVAL_MS
# 3. Increase MAX_FEE_PER_GAS
```

## Optimization

### Faster Relaying

```bash
# Reduce poll interval
POLL_INTERVAL_MS=15000  # 15 seconds

# Increase gas price (get included faster)
MAX_FEE_PER_GAS=100  # 100 gwei
MAX_PRIORITY_FEE_PER_GAS=5
```

### Cost Optimization

```bash
# Increase poll interval (fewer API calls)
POLL_INTERVAL_MS=60000  # 1 minute

# Reduce gas price (but slower)
MAX_FEE_PER_GAS=30  # 30 gwei
```

### Multi-Region Deployment

Deploy relayers in different regions for redundancy:

```
Relayer #1: US East (AWS us-east-1)
Relayer #2: EU West (AWS eu-west-1)
Relayer #3: Asia Pacific (AWS ap-southeast-1)
```

Benefits:
- Lower latency to chains
- Geographic redundancy
- Higher reliability

## Economics

### Costs (Per Relayer)

| Item | Cost/Month |
|------|------------|
| Server (t3.small) | $15-20 |
| Bandwidth | $5-10 |
| Ethereum gas (testnet) | Free |
| **Total** | **$20-30/month** |

### Revenue (Testnet)

Testnet has no revenue, but mainnet relayers could:
- Charge fees for priority relaying
- Earn MEV from transaction ordering
- Get tips from users for fast relays

### Profitability Analysis (Mainnet)

Example:
- **Cost per relay**: $20-50 gas (Ethereum)
- **Volume**: 100 relays/day
- **Cost**: $2,000-5,000/day
- **Revenue**: Fee-based or sponsored

**Note**: Testnet relayers are public service (non-profit)

## Security

### Key Security

- **Separate keys** from mainnet
- **Encrypt** private keys at rest
- **Backup** keys securely
- **Monitor** for unauthorized access

### Network Security

- **Firewall**: Only port 22 (SSH)
- **SSH keys only**: Disable password auth
- **VPN**: Consider for access
- **Fail2ban**: Prevent brute force

### Operational Security

- **Monitor** relay activity
- **Alert** on failures
- **Audit** logs regularly
- **Update** software regularly

## Maintenance

### Updates

```bash
# Pull latest code
cd ~/etrid
git pull

# Rebuild
cd services/relayer-service
npm install
npm run build

# Restart
pm2 restart relayer-service
```

### Log Rotation

```bash
# PM2 handles log rotation automatically
pm2 install pm2-logrotate

# Configure
pm2 set pm2-logrotate:max_size 50M
pm2 set pm2-logrotate:retain 7
```

### Backups

```bash
# Backup configuration
cp .env .env.backup

# Backup logs
pm2 save
```

## Advanced Topics

### Load Balancing Attestation Services

If one attestation service is slow, the relayer automatically tries others:

```bash
# Relayer tries services in order:
# 1. attestation-0.etrid.io
# 2. attestation-1.etrid.io
# 3. attestation-2.etrid.io
# etc.
```

### Running Multiple Relayers

Run multiple relayer instances on same server:

```bash
# Start multiple instances
pm2 start dist/index.js --name relayer-1
pm2 start dist/index.js --name relayer-2

# They'll compete for relays (redundancy)
```

### Custom Gas Strategies

Implement dynamic gas pricing:

```javascript
// Monitor gas prices and adjust
const gasPrice = await provider.getFeeData();
const maxFee = gasPrice.maxFeePerGas * 120n / 100n; // +20%
```

## Community Relayers

Encourage community to run relayers:

**Benefits**:
- Decentralization
- Redundancy
- Lower operational cost for protocol
- Community engagement

**Provide**:
- Clear documentation
- Support channel (Discord)
- Relayer dashboard (optional)
- Recognition/rewards (mainnet)

## Next Steps

1. ✅ Relayer services deployed
2. → End-to-end testing
3. → Monitor and optimize

See [`../TESTING_GUIDE.md`](../TESTING_GUIDE.md) for testing procedures.
