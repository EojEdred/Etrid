# EDSC Bridge - Ember Testnet Deployment Plan

## Overview
Deploy the EDSC cross-chain bridge to Ember testnet (Ëtrid's testnet) and Sepolia (Ethereum testnet) for end-to-end testing.

## Prerequisites

### Ethereum Side (Sepolia)
- [ ] Sepolia RPC endpoint (Infura/Alchemy)
- [ ] Deployer wallet with Sepolia ETH
- [ ] 5 attester wallets with small Sepolia ETH balances
- [ ] 1 relayer wallet with Sepolia ETH

### Substrate Side (Ember)
- [ ] Ember testnet RPC endpoint
- [ ] Ember testnet WSS endpoint
- [ ] Deployer wallet with EDSC tokens
- [ ] 5 attester wallets (same addresses as Ethereum)
- [ ] 1 relayer wallet

## Deployment Steps

### Phase 1: Ethereum Contract Deployment

1. **Configure Hardhat for Sepolia**
   ```javascript
   // hardhat.config.js
   sepolia: {
     url: process.env.SEPOLIA_RPC_URL,
     accounts: [process.env.DEPLOYER_PRIVATE_KEY],
     chainId: 11155111
   }
   ```

2. **Deploy Contracts**
   ```bash
   cd contracts/ethereum
   npx hardhat run scripts/deploy.js --network sepolia
   ```

3. **Register Attesters**
   ```bash
   npx hardhat run scripts/register-attesters.js --network sepolia
   ```

4. **Verify Contracts on Etherscan**
   ```bash
   npx hardhat verify --network sepolia <CONTRACT_ADDRESS>
   ```

### Phase 2: Ember Testnet Setup

1. **Deploy MessageTransmitter Pallet**
   - Ensure pallet is included in Ember runtime
   - Configure domain ID (2 for Ëtrid)
   - Set attester registry with same 5 addresses

2. **Configure Token Minting**
   - EDSC pallet configured on Ember
   - MessageTransmitter authorized to mint

3. **Verify Ember Node**
   ```bash
   # Connect to Ember testnet
   wscat -c wss://ember-rpc.etrid.io
   ```

### Phase 3: Attestation Service Deployment

1. **Deploy 3+ Attestation Services** (for 3-of-5 threshold)

   Service 1:
   ```env
   SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
   ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
   ATTESTER_PRIVATE_KEY=<attester_1_key>
   ATTESTER_ID=0
   TOKEN_MESSENGER_ADDRESS=<sepolia_address>
   PORT=3000
   ```

   Service 2:
   ```env
   ATTESTER_PRIVATE_KEY=<attester_2_key>
   ATTESTER_ID=1
   PORT=3000
   ```

   Service 3:
   ```env
   ATTESTER_PRIVATE_KEY=<attester_3_key>
   ATTESTER_ID=2
   PORT=3000
   ```

2. **Start Services**
   ```bash
   # On each server
   cd services/attestation-service
   npm run build
   pm2 start dist/index.js --name attestation-service
   ```

### Phase 4: Relayer Service Deployment

1. **Configure Relayer**
   ```env
   ATTESTATION_SERVICE_URLS=https://att1.etrid.io,https://att2.etrid.io,https://att3.etrid.io
   SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
   ETHEREUM_RPC_URL=https://sepolia.infura.io/v3/YOUR_KEY
   RELAYER_PRIVATE_KEY=<relayer_key>
   MESSAGE_TRANSMITTER_ADDRESS=<sepolia_address>
   TOKEN_MESSENGER_ADDRESS=<sepolia_address>
   ```

2. **Start Relayer**
   ```bash
   cd services/relayer-service
   npm run build
   pm2 start dist/index.js --name relayer-service
   ```

### Phase 5: Testing

1. **Test Ethereum → Ember**
   ```bash
   npx hardhat run scripts/test-transfer.js --network sepolia
   ```
   - Monitor attestation service logs
   - Verify signatures collected (3-of-5)
   - Confirm relay to Ember
   - Check EDSC balance on Ember

2. **Test Ember → Ethereum**
   - Use Polkadot.js Apps to send transaction
   - Burn EDSC on Ember targeting Ethereum
   - Monitor attestation/relay
   - Verify EDSC minted on Sepolia

## Monitoring & Observability

### Metrics Endpoints
- Attestation Service: `https://att1.etrid.io/metrics`
- Relayer Service: `https://relayer.etrid.io/metrics`

### Health Checks
- Attestation: `https://att1.etrid.io/health`
- Relayer: `https://relayer.etrid.io/health`

### Logs
```bash
pm2 logs attestation-service
pm2 logs relayer-service
```

## Security Considerations

1. **Private Keys**
   - Use environment variables
   - Never commit to git
   - Use key management service for production

2. **Rate Limiting**
   - Configure burn limits on contracts
   - Monitor for unusual activity

3. **Multi-Signature**
   - Deployer wallet should be multi-sig
   - Owner functions protected

## Rollback Plan

If issues are detected:

1. **Pause Contracts**
   ```javascript
   await edsc.pause()
   await tokenMessenger.pause()
   ```

2. **Stop Services**
   ```bash
   pm2 stop attestation-service
   pm2 stop relayer-service
   ```

3. **Investigate & Fix**
   - Check logs
   - Verify attestation signatures
   - Confirm balance consistency

4. **Resume**
   ```javascript
   await edsc.unpause()
   await tokenMessenger.unpause()
   pm2 restart all
   ```

## Success Criteria

- ✅ Contracts deployed and verified on Sepolia
- ✅ 3+ attestation services running
- ✅ Relayer service operational
- ✅ Successful Ethereum → Ember transfer
- ✅ Successful Ember → Ethereum transfer
- ✅ Metrics and monitoring active
- ✅ Documentation complete

## Timeline

- **Day 1**: Deploy Ethereum contracts, verify
- **Day 2**: Configure Ember testnet, deploy pallets
- **Day 3**: Deploy and configure attestation services
- **Day 4**: Deploy relayer service
- **Day 5**: End-to-end testing
- **Day 6**: Monitoring and optimization
- **Day 7**: Documentation and handoff

## Resources Needed

### Infrastructure
- 3 VPS servers for attestation services (2GB RAM each)
- 1 VPS server for relayer service (2GB RAM)
- Domain names for services
- SSL certificates

### Costs (Estimated Monthly)
- VPS hosting: $20-40
- RPC endpoints: $0-50 (depending on usage)
- Total: ~$70/month for testnet

### Team
- 1 DevOps engineer for deployment
- 1 Backend developer for monitoring
- 1 QA engineer for testing

## Next Steps

1. Set up Sepolia deployer wallet
2. Get Sepolia ETH from faucet
3. Configure Hardhat for Sepolia
4. Prepare attester wallets
5. Begin Phase 1 deployment
