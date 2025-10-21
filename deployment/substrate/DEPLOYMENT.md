# Substrate Testnet Deployment (Ember Testnet)

Complete guide for deploying Ëtrid testnet with EDSC bridge pallets.

## Prerequisites

### 1. Build Runtime

The runtime with token-messenger and attestation pallets must be built:

```bash
cd /path/to/etrid

# Build EDSC-PBC runtime
cargo build --release -p edsc-pbc-runtime

# Build EDSC-PBC node
cargo build --release -p edsc-pbc-node
```

**Build time**: 20-40 minutes (first time)

### 2. Prepare Validator Accounts

Generate keys for 3+ validator nodes:

```bash
# Generate session keys for validator 1
./target/release/edsc-pbc-node key generate \
  --scheme Sr25519 \
  --output-type json

# Output:
{
  "secretPhrase": "your secret phrase here",
  "secretSeed": "0x...",
  "publicKey": "0x...",
  "accountId": "5...",
  "ss58Address": "5..."
}
```

Repeat for each validator. **Save these securely!**

### 3. Prepare Attester Accounts

Generate 5 attester accounts (same as Ethereum attesters ideally):

```bash
# For each attester
./target/release/edsc-pbc-node key generate --scheme Sr25519
```

**Note**: These should correspond to the Ethereum attester addresses for consistency.

### 4. Prepare Infrastructure

Minimum setup:
- **3 servers** for validators (4 CPU, 8GB RAM, 200GB SSD each)
- **1 server** for RPC node (4 CPU, 16GB RAM, 500GB SSD)
- **Ubuntu 22.04 LTS** recommended
- **Public IPs** and open ports (30333, 9944)

## Chain Specification

### 1. Create Chain Spec Template

Create `deployment/substrate/chain-spec-raw.json`:

The chain spec is generated from the runtime. First, generate the plain spec:

```bash
./target/release/edsc-pbc-node build-spec \
  --chain=dev \
  > chain-spec-plain.json
```

### 2. Customize Chain Spec

Edit `chain-spec-plain.json`:

```json
{
  "name": "Ember Testnet",
  "id": "etrid_testnet",
  "chainType": "Live",
  "bootNodes": [
    "/dns/validator-1.ember.etrid.io/tcp/30333/p2p/12D3KooW...",
    "/dns/validator-2.ember.etrid.io/tcp/30333/p2p/12D3KooW...",
    "/dns/validator-3.ember.etrid.io/tcp/30333/p2p/12D3KooW..."
  ],
  "telemetryEndpoints": [
    [
      "wss://telemetry.polkadot.io/submit/",
      0
    ]
  ],
  "protocolId": "etrid",
  "properties": {
    "tokenSymbol": "EDSC",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {
        "code": "0x..."
      },
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000000000],
          ["VALIDATOR_1_ADDRESS", 100000000000000000000],
          ["VALIDATOR_2_ADDRESS", 100000000000000000000],
          ["VALIDATOR_3_ADDRESS", 100000000000000000000],
          ["ATTESTER_1_ADDRESS", 10000000000000000000],
          ["ATTESTER_2_ADDRESS", 10000000000000000000],
          ["ATTESTER_3_ADDRESS", 10000000000000000000],
          ["ATTESTER_4_ADDRESS", 10000000000000000000],
          ["ATTESTER_5_ADDRESS", 10000000000000000000]
        ]
      },
      "sudo": {
        "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      },
      "attestation": {
        "attesters": [
          "ATTESTER_1_ADDRESS",
          "ATTESTER_2_ADDRESS",
          "ATTESTER_3_ADDRESS",
          "ATTESTER_4_ADDRESS",
          "ATTESTER_5_ADDRESS"
        ],
        "thresholds": [
          [0, 3],
          [2, 3]
        ]
      }
    }
  }
}
```

### 3. Generate Raw Chain Spec

Convert to raw format:

```bash
./target/release/edsc-pbc-node build-spec \
  --chain=chain-spec-plain.json \
  --raw \
  > chain-spec-raw.json
```

**This is the final chain spec to use for all nodes!**

## Validator Setup

For each validator node:

### 1. Install Node Binary

```bash
# On each validator server
scp target/release/edsc-pbc-node validator@server:/usr/local/bin/
chmod +x /usr/local/bin/edsc-pbc-node
```

### 2. Copy Chain Spec

```bash
scp chain-spec-raw.json validator@server:/etc/edsc/chain-spec.json
```

### 3. Create Systemd Service

Create `/etc/systemd/system/edsc-validator.service`:

```ini
[Unit]
Description=Ember Testnet Validator
After=network.target

[Service]
Type=simple
User=validator
WorkingDirectory=/home/validator
ExecStart=/usr/local/bin/edsc-pbc-node \
  --base-path /var/lib/edsc \
  --chain /etc/edsc/chain-spec.json \
  --name "Validator 1" \
  --validator \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --rpc-cors all
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### 4. Start Validator

```bash
sudo systemctl daemon-reload
sudo systemctl enable edsc-validator
sudo systemctl start edsc-validator

# Check logs
sudo journalctl -u edsc-validator -f
```

### 5. Insert Session Keys

On each validator:

```bash
# Generate session keys via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"author_rotateKeys"}' \
  http://localhost:9944

# Returns: "0x..."  <- This is your session key
```

Then, using Polkadot.js Apps or extrinsic:

```javascript
// Using sudo account, call:
session.setKeys(sessionKey, proof)
```

### 6. Start Validating

Using sudo account:

```javascript
// Add validator to active set
staking.validate(commission, blocked)
```

## RPC Node Setup

Deploy a public RPC node:

### 1. Create Service

`/etc/systemd/system/edsc-rpc.service`:

```ini
[Unit]
Description=Ember Testnet RPC Node
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/home/etrid
ExecStart=/usr/local/bin/edsc-pbc-node \
  --base-path /var/lib/edsc \
  --chain /etc/edsc/chain-spec.json \
  --name "Public RPC" \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods Safe \
  --ws-external \
  --ws-max-connections 1000 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### 2. Configure Nginx Reverse Proxy

Install nginx:

```bash
sudo apt install nginx certbot python3-certbot-nginx
```

Create `/etc/nginx/sites-available/etrid-rpc`:

```nginx
server {
    listen 80;
    server_name ember-rpc.etrid.io;

    location / {
        proxy_pass http://localhost:9944;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 86400;
    }
}
```

Enable and get SSL:

```bash
sudo ln -s /etc/nginx/sites-available/etrid-rpc /etc/nginx/sites-enabled/
sudo certbot --nginx -d ember-rpc.etrid.io
sudo systemctl restart nginx
```

**Public endpoint**: `wss://ember-rpc.etrid.io`

## Configure Pallets

### 1. Register Attesters

Using sudo account via Polkadot.js Apps:

```javascript
// attestation.addAttester(attesterAddress)
for (let attester of attesters) {
  await api.tx.sudo
    .sudo(api.tx.attestation.addAttester(attester))
    .signAndSend(sudoAccount);
}
```

### 2. Set Thresholds

```javascript
// For Ethereum (domain 0): 3-of-5
await api.tx.sudo
  .sudo(api.tx.attestation.setThreshold(0, 3))
  .signAndSend(sudoAccount);

// For Ëtrid (domain 2): 3-of-5
await api.tx.sudo
  .sudo(api.tx.attestation.setThreshold(2, 3))
  .signAndSend(sudoAccount);
```

### 3. Initialize Assets

```javascript
// Create EDSC asset (ID: 0)
await api.tx.sudo
  .sudo(api.tx.assets.create(
    0, // Asset ID
    sudoAccount.address, // Admin
    1 // Min balance
  ))
  .signAndSend(sudoAccount);

// Set metadata
await api.tx.sudo
  .sudo(api.tx.assets.setMetadata(
    0, // Asset ID
    "Ëtrid Dollar Stablecoin",
    "EDSC",
    18 // Decimals
  ))
  .signAndSend(sudoAccount);
```

## Block Explorer

Deploy a block explorer for user visibility:

### Option 1: Subscan

Contact Subscan to add your chain:
- Website: https://www.subscan.io/
- Email: hello@subscan.io

### Option 2: Polkadot.js Apps

Host your own Polkadot.js Apps instance:

```bash
git clone https://github.com/polkadot-js/apps.git
cd apps
yarn install
yarn build

# Serve with nginx
sudo cp -r build/* /var/www/html/
```

Configure to connect to your RPC:
- Update `apps-config` to include your chain
- Default endpoint: `wss://ember-rpc.etrid.io`

**Public explorer**: `https://ember-explorer.etrid.io`

## Post-Deployment Verification

### 1. Check Chain is Producing Blocks

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getHeader"}' \
  https://ember-rpc.etrid.io

# Should return increasing block numbers
```

### 2. Verify Attesters Registered

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"state_getStorage", "params":["0x..."]}' \
  https://ember-rpc.etrid.io

# Or use Polkadot.js Apps:
# Developer → Chain State → attestation → attesters()
```

### 3. Test Pallet Functions

Using Polkadot.js Apps:

**Test tokenMessenger.burnAndSend**:
```javascript
api.tx.tokenMessenger.burnAndSend(
  0, // destination domain (Ethereum)
  recipientBytes,
  amount
).signAndSend(account);
```

**Check for BurnMessageSent event**

## Monitoring

### 1. Telemetry

View your nodes on:
- https://telemetry.polkadot.io/
- Search for "Ember Testnet"

### 2. Prometheus + Grafana

Configure Substrate metrics:

```bash
# Start node with prometheus enabled
--prometheus-external --prometheus-port 9615
```

Setup Grafana dashboard:
- Import Substrate dashboard template
- Monitor: block production, finality, peer count

### 3. Logs

```bash
# View validator logs
sudo journalctl -u edsc-validator -f

# View RPC logs
sudo journalctl -u edsc-rpc -f
```

## Troubleshooting

### Chain Not Producing Blocks

- Check validators are online: `systemctl status edsc-validator`
- Check session keys are set correctly
- Check at least 2/3 validators are active
- View logs for errors

### Peers Not Connecting

- Check firewall allows port 30333
- Check boot nodes are reachable
- Add manual peer: `--reserved-nodes /ip4/.../tcp/30333/p2p/...`

### RPC Not Responding

- Check service status: `systemctl status edsc-rpc`
- Check nginx config: `nginx -t`
- Check SSL cert: `certbot certificates`
- Check WebSocket connection: Browser dev tools

## Security Checklist

- [ ] Validator keys secured (encrypted, backed up)
- [ ] Sudo key secured (multisig recommended for mainnet)
- [ ] Firewall configured (only necessary ports)
- [ ] SSL certificates installed
- [ ] Validators behind VPN/private network
- [ ] RPC rate limiting configured
- [ ] Monitoring and alerting active
- [ ] Backup nodes ready for failover

## Next Steps

1. ✅ Substrate chain deployed
2. → Configure attestation services
3. → Configure relayer services
4. → End-to-end testing

See [`../services/ATTESTATION_DEPLOYMENT.md`](../services/ATTESTATION_DEPLOYMENT.md) for next steps.
