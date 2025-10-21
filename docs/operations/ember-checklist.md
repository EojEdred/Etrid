# EDSC Bridge - Ember Testnet Deployment Checklist

## Pre-Deployment Checklist

### 1. Wallet Setup
- [ ] Create deployer wallet
  - [ ] Save private key securely
  - [ ] Get Sepolia ETH from faucet (https://sepoliafaucet.com/)
  - [ ] Verify balance: minimum 0.5 ETH for deployment + gas

- [ ] Create 5 attester wallets
  - [ ] Generate 5 keypairs
  - [ ] Save all private keys securely
  - [ ] Get small amount of Sepolia ETH for each (~0.01 ETH)
  - [ ] Record all addresses

- [ ] Create relayer wallet
  - [ ] Generate keypair
  - [ ] Save private key securely
  - [ ] Get Sepolia ETH (~0.2 ETH for relay operations)

### 2. RPC Endpoints
- [ ] Infura account created OR Alchemy account created
- [ ] Sepolia endpoint URL obtained
- [ ] Endpoint tested with curl:
  ```bash
  curl -X POST YOUR_RPC_URL \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
  ```

### 3. Etherscan Setup
- [ ] Etherscan account created
- [ ] API key generated
- [ ] API key tested

### 4. Code Preparation
- [ ] Remove `testMint()` function from EDSC.sol
- [ ] All tests passing: `npm test`
- [ ] No compiler warnings
- [ ] Gas optimization review complete

### 5. Environment Configuration
- [ ] Copy `.env.sepolia.example` to `.env`
- [ ] Fill in all required values:
  - [ ] SEPOLIA_RPC_URL
  - [ ] PRIVATE_KEY (deployer)
  - [ ] ETHERSCAN_API_KEY
  - [ ] ATTESTER addresses (all 5)
  - [ ] MIN_SIGNATURES=3
  - [ ] TOTAL_ATTESTERS=5

## Deployment Phase

### Step 1: Deploy Ethereum Contracts (30 minutes)

```bash
cd /Users/macbook/Desktop/etrid/contracts/ethereum
```

- [ ] Compile contracts
  ```bash
  npx hardhat clean
  npx hardhat compile
  ```
  - [ ] No errors
  - [ ] No warnings

- [ ] Deploy to Sepolia
  ```bash
  npx hardhat run scripts/deploy.js --network sepolia
  ```
  - [ ] Deployment successful
  - [ ] Save deployment JSON file
  - [ ] Record contract addresses:
    - EDSC: _________________
    - AttesterRegistry: _________________
    - MessageTransmitter: _________________
    - TokenMessenger: _________________

- [ ] Verify contracts on Etherscan
  ```bash
  npx hardhat verify --network sepolia <EDSC_ADDRESS> "<DEPLOYER_ADDRESS>"
  npx hardhat verify --network sepolia <ATTESTER_REGISTRY_ADDRESS> <MIN_SIGS> <TOTAL_ATTESTERS>
  npx hardhat verify --network sepolia <MESSAGE_TRANSMITTER_ADDRESS> <ATTESTER_REGISTRY_ADDRESS>
  npx hardhat verify --network sepolia <TOKEN_MESSENGER_ADDRESS> <EDSC_ADDRESS>
  ```
  - [ ] All contracts verified
  - [ ] Verified on Etherscan UI

- [ ] Register attesters
  ```bash
  npx hardhat run scripts/register-attesters.js --network sepolia
  ```
  - [ ] All 5 attesters registered
  - [ ] Verification: 5 RegisterAttester events emitted

### Step 2: Configure Ember Testnet (1-2 hours)

- [ ] Ember testnet node accessible
  ```bash
  wscat -c wss://ember-rpc.etrid.io
  ```

- [ ] MessageTransmitter pallet deployed on Ember
  - [ ] Domain ID = 2
  - [ ] Same 5 attester addresses registered
  - [ ] Threshold = 3-of-5

- [ ] EDSC token configured on Ember
  - [ ] MessageTransmitter authorized to mint
  - [ ] Test minting disabled

- [ ] Test Ember connection
  ```bash
  curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
    https://ember-rpc.etrid.io
  ```

### Step 3: Deploy Attestation Services (2-3 hours)

For each of 3 attestation services:

**Service 1** (Attester ID 0)
- [ ] VPS provisioned
  - Instance type: 2GB RAM minimum
  - [ ] SSH access configured
  - [ ] Firewall: Allow ports 22, 3000, 9090

- [ ] Install dependencies
  ```bash
  curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
  sudo apt-get install -y nodejs
  sudo npm install -g pm2
  ```

- [ ] Clone repository
  ```bash
  git clone https://github.com/etrid/etrid.git
  cd etrid/services/attestation-service
  npm install
  ```

- [ ] Configure environment
  ```bash
  cp .env.ember.example .env
  nano .env  # Fill in all values
  ```
  - [ ] ATTESTER_ID=0
  - [ ] ATTESTER_PRIVATE_KEY set
  - [ ] All contract addresses from Sepolia deployment
  - [ ] Ember WSS URL
  - [ ] Sepolia RPC URL

- [ ] Build and test
  ```bash
  npm run build
  node dist/index.js  # Test run
  ```
  - [ ] Connects to Sepolia
  - [ ] Connects to Ember
  - [ ] No errors in logs

- [ ] Start with PM2
  ```bash
  pm2 start dist/index.js --name attestation-service-0
  pm2 save
  pm2 startup  # Follow instructions
  ```

- [ ] Verify service
  ```bash
  curl http://localhost:3000/health
  curl http://localhost:3000/stats
  ```

**Service 2** (Attester ID 1)
- [ ] Repeat above with ATTESTER_ID=1

**Service 3** (Attester ID 2)
- [ ] Repeat above with ATTESTER_ID=2

### Step 4: Deploy Relayer Service (1 hour)

- [ ] VPS provisioned
  - [ ] 2GB RAM minimum
  - [ ] SSH access configured
  - [ ] Firewall: Allow ports 22, 3001, 9091

- [ ] Install dependencies (same as attestation)

- [ ] Clone and setup
  ```bash
  cd etrid/services/relayer-service
  npm install
  cp .env.ember.example .env
  nano .env
  ```
  - [ ] ATTESTATION_SERVICE_URLS (all 3)
  - [ ] RELAYER_PRIVATE_KEY set
  - [ ] Contract addresses set
  - [ ] Chain URLs configured

- [ ] Build and test
  ```bash
  npm run build
  node dist/index.js
  ```

- [ ] Start with PM2
  ```bash
  pm2 start dist/index.js --name relayer-service
  pm2 save
  ```

- [ ] Verify service
  ```bash
  curl http://localhost:3001/health
  ```

## Testing Phase

### Test 1: Ethereum → Ember Transfer (20 minutes)

- [ ] Prepare test wallet with EDSC
  ```bash
  cd contracts/ethereum
  # Create test script that uses owner to mint initial supply
  ```

- [ ] Execute transfer
  ```bash
  npx hardhat run scripts/test-sepolia-transfer.js --network sepolia
  ```
  - [ ] Transaction confirmed on Sepolia
  - [ ] MessageSent event emitted
  - [ ] Note: TX hash: _________________

- [ ] Monitor attestation (wait ~3 minutes for confirmations)
  ```bash
  # Check each attestation service
  curl http://att1-ip:3000/stats
  curl http://att2-ip:3000/stats
  curl http://att3-ip:3000/stats
  ```
  - [ ] Event detected by all 3 services
  - [ ] 3 signatures collected
  - [ ] Message status = "ready"

- [ ] Monitor relay (automatic within 10 seconds)
  ```bash
  curl http://relayer-ip:3001/health
  # Check relayer logs
  pm2 logs relayer-service
  ```
  - [ ] Relayer detected ready message
  - [ ] Relay transaction submitted to Ember
  - [ ] Relay transaction confirmed

- [ ] Verify on Ember
  - [ ] Check recipient balance on Ember testnet
  - [ ] Balance increased by expected amount
  - [ ] MintMessage event emitted on Ember

### Test 2: Ember → Ethereum Transfer (20 minutes)

- [ ] Use Polkadot.js Apps UI
  - [ ] Connect to wss://ember-rpc.etrid.io
  - [ ] Submit burnAndSend extrinsic
  - [ ] Amount: 50 EDSC
  - [ ] Destination: Ethereum (domain 0)
  - [ ] Recipient: test wallet address

- [ ] Monitor attestation services
  - [ ] BurnMessageSent event detected
  - [ ] 3 signatures collected

- [ ] Monitor relayer
  - [ ] Message relayed to Sepolia
  - [ ] Transaction confirmed

- [ ] Verify on Sepolia
  - [ ] Check recipient EDSC balance on Etherscan
  - [ ] Balance increased correctly

### Test 3: Error Handling (10 minutes)

- [ ] Test insufficient balance
  - [ ] Attempt to burn more than balance
  - [ ] Verify transaction reverts

- [ ] Test invalid recipient
  - [ ] Attempt transfer with invalid address
  - [ ] Verify transaction reverts

- [ ] Test service resilience
  - [ ] Stop 1 attestation service
  - [ ] Verify transfer still completes (need 3/5)
  - [ ] Restart service

## Monitoring Setup

### Prometheus + Grafana (Optional but recommended)

- [ ] Prometheus installed
  ```yaml
  # prometheus.yml
  scrape_configs:
    - job_name: 'attestation-1'
      static_configs:
        - targets: ['att1-ip:9090']
    - job_name: 'attestation-2'
      static_configs:
        - targets: ['att2-ip:9090']
    - job_name: 'attestation-3'
      static_configs:
        - targets: ['att3-ip:9090']
    - job_name: 'relayer'
      static_configs:
        - targets: ['relayer-ip:9091']
  ```

- [ ] Grafana dashboard imported
  - [ ] Messages seen metric
  - [ ] Attestations created metric
  - [ ] Relay success/failure rate
  - [ ] Block heights (both chains)

### Alerts Setup

- [ ] Pagerduty/Opsgenie configured
- [ ] Alert: Service down
- [ ] Alert: Block height not advancing
- [ ] Alert: Relay failure rate > 5%
- [ ] Alert: Signature threshold not met

## Documentation

- [ ] README updated with Ember addresses
- [ ] API documentation published
- [ ] User guide created for transfers
- [ ] Troubleshooting guide created

## Security Review

- [ ] All private keys stored securely (not in git)
- [ ] Owner wallet is multi-sig (for production)
- [ ] Rate limits configured correctly
- [ ] Services running as non-root user
- [ ] UFW/firewall configured
- [ ] SSH key-only access
- [ ] Fail2ban configured

## Post-Deployment

- [ ] Monitor for 24 hours
- [ ] Test 5+ transfers in both directions
- [ ] Review all metrics
- [ ] Team notification sent
- [ ] Documentation published
- [ ] Testnet announced to community

## Rollback Plan

If critical issues found:

1. [ ] Pause contracts immediately
   ```javascript
   await edsc.pause()
   await tokenMessenger.pause()
   ```

2. [ ] Stop all services
   ```bash
   pm2 stop all
   ```

3. [ ] Investigate issue
4. [ ] Fix and redeploy if needed
5. [ ] Resume services

## Success Criteria

- [  ] All contracts deployed and verified
- [ ] 3+ attestation services operational
- [ ] Relayer service operational
- [ ] 5+ successful Ethereum → Ember transfers
- [ ] 5+ successful Ember → Ethereum transfers
- [ ] Monitoring and alerts active
- [ ] Zero loss of funds
- [ ] Average attestation time < 5 minutes
- [ ] Average relay time < 30 seconds
- [ ] 99%+ relay success rate

## Notes

Deployment Date: _________________
Deployed By: _________________
Sepolia Contract Addresses: (see above)
Ember RPC: wss://ember-rpc.etrid.io

Issues Encountered:
_________________________________________
_________________________________________
_________________________________________
