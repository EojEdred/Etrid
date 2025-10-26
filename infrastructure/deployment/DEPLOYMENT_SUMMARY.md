# EDSC Bridge Deployment Summary

Quick reference for deploying the complete EDSC cross-chain bridge to testnet.

## TL;DR

1. **Deploy Ethereum contracts** to Sepolia (~30 min)
2. **Deploy Substrate chain** with pallets (~2 hours)
3. **Deploy 5 attestation services** (~2 hours)
4. **Deploy 2-3 relayer services** (~30 min)
5. **Test end-to-end** (~2 hours)

**Total Time**: ~7-9 hours
**Total Cost**: ~$300-500/month (testnet operation)

## Component Checklist

### Ethereum (Sepolia)

- [ ] Get 1 ETH from faucets
- [ ] Get Alchemy/Infura RPC API key
- [ ] Get Etherscan API key
- [ ] Generate 5 attester addresses
- [ ] Deploy EDSC token
- [ ] Deploy AttesterRegistry
- [ ] Deploy MessageTransmitter
- [ ] Deploy TokenMessenger
- [ ] Register 5 attesters
- [ ] Set threshold (3-of-5)
- [ ] Verify contracts on Etherscan

**Output**: 4 contract addresses

### Substrate (Ember Testnet)

- [ ] Build runtime with pallets
- [ ] Generate chain spec
- [ ] Start 3 validator nodes
- [ ] Start RPC node with nginx
- [ ] Register 5 attesters in pallet
- [ ] Set threshold (3-of-5)
- [ ] Deploy block explorer
- [ ] Verify blocks producing

**Output**: `wss://ember-rpc.etrid.io`

### Attestation Services

- [ ] Provision 5 servers (2 CPU, 4GB RAM each)
- [ ] Install Node.js 18+ on each
- [ ] Configure unique attester ID per service
- [ ] Setup nginx + SSL for each
- [ ] Start services with PM2
- [ ] Verify all 5 healthy
- [ ] Setup monitoring

**Output**: 5 public APIs

### Relayer Services

- [ ] Provision 2-3 servers
- [ ] Fund Ethereum accounts (0.5 ETH each)
- [ ] Fund Substrate accounts (100 EDSC each)
- [ ] Configure all attestation service URLs
- [ ] Start services with PM2
- [ ] Verify relaying works
- [ ] Setup balance monitoring

**Output**: Active relayers

### Testing

- [ ] Ethereum → Ëtrid transfer (100 EDSC)
- [ ] Ëtrid → Ethereum transfer (100 EDSC)
- [ ] Round-trip transfer
- [ ] Concurrent transfers (3x)
- [ ] High-value transfer (10,000 EDSC)
- [ ] Verify statistics
- [ ] Load testing

**Output**: Test report

## Quick Commands Reference

### Ethereum Deployment

```bash
cd contracts/ethereum
npx hardhat run scripts/deploy.js --network sepolia
npx hardhat verify --network sepolia <ADDRESS> [CONSTRUCTOR_ARGS]
```

### Substrate Deployment

```bash
# Build
cargo build --release -p edsc-pbc-runtime
cargo build --release -p edsc-pbc-node

# Generate chain spec
./target/release/edsc-pbc-node build-spec --chain=dev > chain-spec-plain.json
# Edit chain-spec-plain.json
./target/release/edsc-pbc-node build-spec --chain=chain-spec-plain.json --raw > chain-spec-raw.json

# Start validator
./target/release/edsc-pbc-node \
  --base-path /var/lib/edsc \
  --chain chain-spec-raw.json \
  --validator \
  --name "Validator 1"
```

### Attestation Service

```bash
cd services/attestation-service
npm install && npm run build
pm2 start dist/index.js --name attestation-service
curl https://attestation-0.etrid.io/health
```

### Relayer Service

```bash
cd services/relayer-service
npm install && npm run build
pm2 start dist/index.js --name relayer-service
pm2 logs relayer-service
```

## Environment Variables Summary

### Attestation Service `.env`

```bash
SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
ETHEREUM_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/API-KEY
ATTESTER_ID=0                    # Unique: 0-4
ATTESTER_ADDRESS=0x...           # From deployment
ATTESTER_PRIVATE_KEY=0x...       # Unique per attester
MIN_SIGNATURES=3
TOTAL_ATTESTERS=5
CONFIRMATIONS_REQUIRED=2
TOKEN_MESSENGER_ADDRESS=0x...    # From deployment
PORT=3000
LOG_LEVEL=info
```

### Relayer Service `.env`

```bash
ATTESTATION_SERVICE_URLS=https://attestation-0.etrid.io,https://attestation-1.etrid.io,https://attestation-2.etrid.io,https://attestation-3.etrid.io,https://attestation-4.etrid.io
SUBSTRATE_WS_URL=wss://ember-rpc.etrid.io
ETHEREUM_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/API-KEY
RELAYER_ADDRESS=0x...
RELAYER_PRIVATE_KEY=0x...
MESSAGE_TRANSMITTER_ADDRESS=0x...  # From deployment
TOKEN_MESSENGER_ADDRESS=0x...      # From deployment
POLL_INTERVAL_MS=30000
MAX_RETRIES=3
GAS_LIMIT=500000
MAX_FEE_PER_GAS=50
MAX_PRIORITY_FEE_PER_GAS=2
LOG_LEVEL=info
```

## Deployment Addresses Template

Save these after deployment:

```json
{
  "network": "testnet",
  "deployedAt": "2024-01-15T10:00:00Z",
  "ethereum": {
    "network": "sepolia",
    "chainId": 11155111,
    "contracts": {
      "EDSC": "0x...",
      "AttesterRegistry": "0x...",
      "MessageTransmitter": "0x...",
      "TokenMessenger": "0x..."
    },
    "attesters": [
      "0x...", "0x...", "0x...", "0x...", "0x..."
    ],
    "threshold": 3
  },
  "substrate": {
    "network": "etrid-testnet",
    "rpcUrl": "wss://ember-rpc.etrid.io",
    "explorer": "https://ember-explorer.etrid.io",
    "attesters": [
      "5...", "5...", "5...", "5...", "5..."
    ],
    "threshold": 3
  },
  "services": {
    "attestation": [
      "https://attestation-0.etrid.io",
      "https://attestation-1.etrid.io",
      "https://attestation-2.etrid.io",
      "https://attestation-3.etrid.io",
      "https://attestation-4.etrid.io"
    ],
    "relayers": [
      "relayer-1 (active)",
      "relayer-2 (active)"
    ]
  }
}
```

## Monitoring URLs

- **Ethereum Sepolia**: https://sepolia.etherscan.io/
- **Substrate Explorer**: https://ember-explorer.etrid.io
- **Attestation #0**: https://attestation-0.etrid.io/health
- **Attestation #1**: https://attestation-1.etrid.io/health
- **Attestation #2**: https://attestation-2.etrid.io/health
- **Attestation #3**: https://attestation-3.etrid.io/health
- **Attestation #4**: https://attestation-4.etrid.io/health
- **Grafana** (if deployed): https://monitoring.etrid.io

## Health Check Commands

```bash
# Check Ethereum contracts
curl -X POST https://eth-sepolia.g.alchemy.com/v2/API-KEY \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Check Substrate chain
curl -X POST wss://ember-rpc.etrid.io \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}'

# Check all attestation services
for i in {0..4}; do
  curl -s https://attestation-$i.etrid.io/health | jq '.status'
done

# Check ready attestations
curl https://attestation-0.etrid.io/attestations/ready | jq '.count'

# Check relayer logs
pm2 logs relayer-service --lines 20
```

## Common Issues

### Issue: Contract deployment fails

**Solution**: Check account has sufficient ETH, verify RPC URL, check Hardhat config

### Issue: Substrate chain won't start

**Solution**: Check binary exists, verify chain spec format, check ports available

### Issue: Attestation service not processing events

**Solution**: Verify RPC connections, check contract address, check confirmations setting

### Issue: Relayer not relaying

**Solution**: Check attestation service URLs, verify account balances, check gas settings

### Issue: Messages not being signed

**Solution**: Verify attester IDs unique, check private keys correct, restart services

## Rollback Procedure

If critical issues found:

1. **Pause**: Stop relayer services
2. **Assess**: Check logs, identify issue
3. **Fix**: Deploy patch or new contracts
4. **Test**: Validate fixes on separate testnet
5. **Resume**: Restart services
6. **Monitor**: Watch for recurring issues

## Success Criteria

Deployment is successful when:

- [ ] All contracts deployed and verified
- [ ] Substrate chain producing blocks
- [ ] All 5 attestation services healthy
- [ ] 2+ relayers active
- [ ] Ethereum → Ëtrid transfer works
- [ ] Ëtrid → Ethereum transfer works
- [ ] Round-trip transfer works
- [ ] Monitoring shows no errors
- [ ] Block explorers functional
- [ ] Documentation published

## Next Steps After Deployment

1. **Announce**: Post on Discord, Twitter
2. **Invite**: Encourage community testing
3. **Monitor**: Watch for issues
4. **Document**: Record any problems
5. **Iterate**: Fix issues, deploy updates
6. **Prepare**: Plan for mainnet

## Support Contacts

- **Technical Issues**: GitHub Issues
- **General Questions**: Discord #support
- **Security**: security@etrid.io
- **Partnership**: partnerships@etrid.io

## Mainnet Considerations

For mainnet deployment, additionally require:

- [ ] Security audit (smart contracts + pallets)
- [ ] Penetration testing
- [ ] Economic modeling
- [ ] Insurance coverage
- [ ] Legal review
- [ ] 24/7 on-call team
- [ ] Incident response plan
- [ ] Multi-sig for admin functions
- [ ] HSM for key management
- [ ] Professional monitoring
- [ ] DDoS protection
- [ ] High-availability setup

**Mainnet launch should be gradual**:
1. Limited beta (whitelisted users)
2. Soft launch (transfer limits)
3. Full launch (all features)

## License

Apache-2.0
