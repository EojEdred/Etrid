# Attestation Service Deployment

Guide for deploying 5 attestation services for testnet operation.

## Architecture

Each attester runs their own attestation service:

```
Attester #0 → Service → https://attestation-0.etrid.io
Attester #1 → Service → https://attestation-1.etrid.io
Attester #2 → Service → https://attestation-2.etrid.io
Attester #3 → Service → https://attestation-3.etrid.io
Attester #4 → Service → https://attestation-4.etrid.io
```

## Prerequisites

### Per Attester

- **1 server**: 2 CPU, 4GB RAM, 50GB SSD
- **Ubuntu 22.04 LTS**
- **Node.js 18+**
- **PM2** or systemd for process management
- **Domain name** (e.g., attestation-0.etrid.io)
- **SSL certificate** (Let's Encrypt)

### Attester Keys

From Ethereum deployment, you should have:
- 5 Ethereum private keys
- 5 Ethereum addresses

Generate corresponding Substrate keys:

```bash
# For each Ethereum private key, derive Substrate key
# Or generate fresh SR25519 keys
./target/release/edsc-pbc-node key generate --scheme Sr25519
```

## Server Setup

For each of the 5 servers:

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
sudo useradd -m -s /bin/bash attestation
sudo su - attestation
```

### 3. Clone Repository

```bash
cd ~
git clone https://github.com/etrid/etrid.git
cd etrid/services/attestation-service
```

### 4. Install Dependencies

```bash
npm install
npm run build
```

## Configuration

### For Each Attester (Attester #0 example)

Create `.env`:

```bash
# Chain Connections
SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
ETHEREUM_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR-API-KEY

# Attester Identity
ATTESTER_ID=0
ATTESTER_ADDRESS=0xYourEthereumAddress0
ATTESTER_PRIVATE_KEY=0xYourPrivateKey0

# Signature Thresholds
MIN_SIGNATURES=3
TOTAL_ATTESTERS=5

# Security
CONFIRMATIONS_REQUIRED=2

# Contract Addresses (from Ethereum deployment)
TOKEN_MESSENGER_ADDRESS=0xTokenMessengerAddress

# API
PORT=3000
LOG_LEVEL=info
```

**⚠️ CRITICAL**: Each attester must have:
- Unique `ATTESTER_ID` (0-4)
- Unique `ATTESTER_ADDRESS`
- Unique `ATTESTER_PRIVATE_KEY`
- Same `TOKEN_MESSENGER_ADDRESS`

### Configuration Matrix

| Attester | ID | Address | Private Key | Domain |
|----------|----|---------| ------------|--------|
| #0 | 0 | 0x... (from deployment) | 0x... | attestation-0.etrid.io |
| #1 | 1 | 0x... (from deployment) | 0x... | attestation-1.etrid.io |
| #2 | 2 | 0x... (from deployment) | 0x... | attestation-2.etrid.io |
| #3 | 3 | 0x... (from deployment) | 0x... | attestation-3.etrid.io |
| #4 | 4 | 0x... (from deployment) | 0x... | attestation-4.etrid.io |

## Deployment Methods

### Method 1: PM2 (Recommended)

```bash
# As attestation user
cd ~/etrid/services/attestation-service

# Start with PM2
pm2 start dist/index.js --name attestation-service

# Save PM2 configuration
pm2 save

# Setup PM2 to start on boot
pm2 startup
# Follow the instructions shown

# View logs
pm2 logs attestation-service

# Monitor
pm2 monit
```

### Method 2: Systemd

Create `/etc/systemd/system/attestation-service.service`:

```ini
[Unit]
Description=EDSC Attestation Service
After=network.target

[Service]
Type=simple
User=attestation
WorkingDirectory=/home/attestation/etrid/services/attestation-service
EnvironmentFile=/home/attestation/etrid/services/attestation-service/.env
ExecStart=/usr/bin/node dist/index.js
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Start service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable attestation-service
sudo systemctl start attestation-service

# Check status
sudo systemctl status attestation-service

# View logs
sudo journalctl -u attestation-service -f
```

## Nginx Reverse Proxy

Setup nginx for SSL and public access:

### 1. Install Nginx

```bash
sudo apt install nginx certbot python3-certbot-nginx
```

### 2. Configure Site

Create `/etc/nginx/sites-available/attestation`:

```nginx
server {
    listen 80;
    server_name attestation-0.etrid.io;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 3. Enable Site and SSL

```bash
sudo ln -s /etc/nginx/sites-available/attestation /etc/nginx/sites-enabled/
sudo certbot --nginx -d attestation-0.etrid.io
sudo systemctl restart nginx
```

**Repeat for each attester** with their respective domains.

## Verification

### 1. Check Service Health

```bash
curl https://attestation-0.etrid.io/health | jq
```

Expected response:

```json
{
  "status": "healthy",
  "uptime": 12345,
  "substrate": {
    "isRunning": true,
    "lastBlock": 98765,
    "eventsProcessed": 42,
    "errors": 0
  },
  "ethereum": {
    "isRunning": true,
    "lastBlock": 54321,
    "eventsProcessed": 38,
    "errors": 0
  },
  "pendingAttestations": 2,
  "readyAttestations": 5
}
```

### 2. Check All Attesters

```bash
# Check all 5 attesters
for i in {0..4}; do
  echo "Checking attester $i..."
  curl -s https://attestation-$i.etrid.io/health | jq '.status'
done
```

Should all return `"healthy"`.

### 3. Check Statistics

```bash
curl https://attestation-0.etrid.io/stats | jq
```

### 4. Monitor Events

Watch logs for event processing:

```bash
# PM2
pm2 logs attestation-service --lines 100

# Systemd
sudo journalctl -u attestation-service -f
```

Look for:
- "Connected to Ethereum"
- "Connected to Substrate"
- "Processing burn event"
- "Signed message"

## Monitoring

### Setup Health Checks

Use a monitoring service (UptimeRobot, Pingdom, etc.):

**For each attester**:
- Endpoint: `https://attestation-N.etrid.io/health`
- Interval: 5 minutes
- Alert on: Status != "healthy", HTTP != 200, Timeout

### Setup Alerting

Configure alerts for:
- Service down (health check fails)
- High error rate (`errors` field increasing)
- No events processed (stuck)
- Low attestation count (not signing)

**Example: PagerDuty, Slack, Email**

### Log Aggregation

Forward logs to centralized logging:

```bash
# Install Filebeat (for ELK stack)
# Or CloudWatch agent (for AWS)
# Or Stackdriver agent (for GCP)
```

## Troubleshooting

### Service Won't Start

```bash
# Check logs
pm2 logs attestation-service --err

# Check environment
cat .env

# Check Node.js version
node --version  # Should be 18+

# Check port availability
sudo lsof -i :3000
```

### Not Processing Events

**Check RPC connections**:

```bash
# Test Ethereum RPC
curl -X POST https://eth-sepolia.g.alchemy.com/v2/YOUR-API-KEY \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Test Substrate RPC
wscat -c wss://ember-rpc.etrid.io
> {"id":1,"jsonrpc":"2.0","method":"chain_getHeader"}
```

**Check contract address**:
- Verify `TOKEN_MESSENGER_ADDRESS` is correct
- Check contract exists on Sepolia Etherscan

### Signatures Not Aggregating

**Check attester IDs are unique**:

```bash
# On each server
grep ATTESTER_ID .env
```

Should be 0, 1, 2, 3, 4 (unique per attester)

**Check same message hash**:
- All attesters must process same events
- Check they're connected to same chains
- Verify block confirmations aligned

### High Memory Usage

```bash
# Check memory
pm2 info attestation-service

# Restart if needed
pm2 restart attestation-service

# Set memory limit
pm2 start dist/index.js --name attestation-service --max-memory-restart 500M
```

## Security

### Key Security

- **Never commit** `.env` files
- **Encrypt** private keys at rest
- **Rotate** keys periodically (requires re-registration)
- **Backup** keys securely (encrypted cloud storage)
- **Use HSM** for mainnet (Hardware Security Module)

### Network Security

- **Firewall**: Only ports 22 (SSH), 80 (HTTP), 443 (HTTPS), 3000 (optional)
- **SSH**: Use key authentication only, disable password auth
- **Fail2ban**: Install to prevent brute force
- **VPN**: Consider for server access

### Monitoring

- **Alert** on unauthorized access attempts
- **Log** all API requests
- **Audit** key usage regularly
- **Review** logs for anomalies

## Maintenance

### Updates

```bash
# Pull latest code
cd ~/etrid
git pull

# Rebuild
cd services/attestation-service
npm install
npm run build

# Restart
pm2 restart attestation-service

# Or with systemd
sudo systemctl restart attestation-service
```

### Backups

```bash
# Backup configuration
cp .env .env.backup

# Backup logs (if not using log aggregation)
tar -czf logs-$(date +%Y%m%d).tar.gz logs/
```

### Key Rotation

If keys are compromised:

1. **Generate new keys**
2. **Update** `.env` with new keys
3. **Register** new attester address on-chain
4. **Remove** old attester address
5. **Restart** service
6. **Verify** new keys working

## Cost Estimates

Per attester:

| Item | Provider | Cost/Month |
|------|----------|------------|
| Server (2 CPU, 4GB RAM) | AWS t3.small | $15-20 |
| Bandwidth | AWS | $5-10 |
| SSL Certificate | Let's Encrypt | Free |
| Domain | Route53 | $0.50 |
| Monitoring | CloudWatch | $5-10 |
| **Total per attester** | | **$25-40/month** |

**Total for 5 attesters**: $125-200/month

## Production Considerations

For mainnet deployment:

- [ ] Use HSM for key storage
- [ ] Multi-region redundancy
- [ ] Auto-scaling for high load
- [ ] DDoS protection (Cloudflare)
- [ ] Regular security audits
- [ ] Incident response plan
- [ ] 24/7 on-call rotation
- [ ] Insurance for key compromise

## Next Steps

1. ✅ Attestation services deployed
2. → Deploy relayer services
3. → End-to-end testing

See [`RELAYER_DEPLOYMENT.md`](./RELAYER_DEPLOYMENT.md) for next steps.
