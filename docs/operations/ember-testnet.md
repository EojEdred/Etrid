# EDSC Cross-Chain Bridge - Ember Testnet

## Overview

The EDSC (Ëtrid Dollar Stablecoin) cross-chain bridge enables trustless transfer of EDSC tokens between Ethereum (Sepolia testnet) and Ëtrid's Ember testnet using a decentralized attestation model.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Ethereum (Sepolia)                          │
│  ┌──────────────┐  ┌───────────────────┐  ┌──────────────────┐ │
│  │ EDSC Token   │──│ Token Messenger   │──│ Message          │ │
│  │ (ERC-20)     │  │ (Burn & Emit)     │  │ Transmitter      │ │
│  └──────────────┘  └───────────────────┘  │ (Verify & Mint)  │ │
│                                            └──────────────────┘ │
│                    ┌───────────────────┐                        │
│                    │ Attester Registry │                        │
│                    │ (5 Attesters,     │                        │
│                    │  3-of-5 Threshold)│                        │
│                    └───────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                │ Cross-Chain Message
                                │
                    ┌───────────┴───────────┐
                    │ Attestation Services  │
                    │ (3+ instances)        │
                    │ - Monitor events      │
                    │ - Sign messages       │
                    │ - Store signatures    │
                    └───────────┬───────────┘
                                │
                    ┌───────────┴───────────┐
                    │ Relayer Service       │
                    │ - Collect signatures  │
                    │ - Submit to dest      │
                    └───────────┬───────────┘
                                │
┌─────────────────────────────────────────────────────────────────┐
│                       Ëtrid (Ember)                              │
│  ┌──────────────┐  ┌───────────────────┐  ┌──────────────────┐ │
│  │ EDSC Pallet  │──│ Token Messenger   │──│ Message          │ │
│  │ (Native)     │  │ Pallet            │  │ Transmitter      │ │
│  └──────────────┘  └───────────────────┘  │ Pallet           │ │
│                                            └──────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Features

- **Trustless**: Decentralized M-of-N attestation (3-of-5)
- **Secure**: Non-custodial, burn-and-mint model
- **Fast**: ~3-5 minute cross-chain transfers
- **Rate Limited**: Per-transaction and daily limits
- **Pausable**: Emergency pause mechanism
- **Monitored**: Prometheus metrics and health endpoints

## Deployment Status

### Local Testing ✅
- [x] Smart contracts deployed to Hardhat
- [x] End-to-end transfer tested successfully
- [x] Event emission verified
- [x] Balance tracking confirmed

### Ember Testnet 📋
- [ ] Ethereum contracts deployed to Sepolia
- [ ] Ember pallet deployed
- [ ] Attestation services running (3+)
- [ ] Relayer service running
- [ ] End-to-end testing complete

## Quick Start

### For Developers

1. **Review Documentation**
   - [Deployment Plan](./contracts/ethereum/EMBER_DEPLOYMENT_PLAN.md)
   - [Deployment Checklist](./EMBER_DEPLOYMENT_CHECKLIST.md)

2. **Configure Environment**
   ```bash
   cd contracts/ethereum
   cp .env.sepolia.example .env
   # Fill in your values
   ```

3. **Deploy to Sepolia**
   ```bash
   npm install
   npx hardhat compile
   npx hardhat run scripts/deploy.js --network sepolia
   npx hardhat run scripts/register-attesters.js --network sepolia
   npx hardhat run scripts/verify-all.js --network sepolia
   ```

4. **Check Deployment**
   ```bash
   npx hardhat run scripts/check-deployment.js --network sepolia
   ```

### For Users

Once deployed to Ember testnet:

**Transfer Ethereum → Ember:**
1. Get Sepolia EDSC tokens
2. Approve TokenMessenger contract
3. Call `burnAndSendTo(destinationDomain, recipient, amount)`
4. Wait ~3-5 minutes for attestation and relay
5. Check balance on Ember

**Transfer Ember → Ethereum:**
1. Use Polkadot.js Apps
2. Connect to Ember testnet
3. Submit `tokenMessenger.burnAndSend` extrinsic
4. Wait ~3-5 minutes
5. Check balance on Sepolia

## Smart Contracts (Sepolia)

| Contract | Purpose | Address |
|----------|---------|---------|
| EDSC | ERC-20 token | TBD after deployment |
| AttesterRegistry | Manages attesters | TBD |
| MessageTransmitter | Receives messages | TBD |
| TokenMessenger | Burns tokens | TBD |

## Services

### Attestation Services
- **Purpose**: Monitor burn events, sign messages
- **Instances**: Minimum 3 (for 3-of-5 threshold)
- **Endpoints**:
  - Health: `http://service:3000/health`
  - Stats: `http://service:3000/stats`
  - Metrics: `http://service:9090/metrics`

### Relayer Service
- **Purpose**: Collect signatures and relay to destination
- **Instances**: 1+ (for redundancy)
- **Endpoints**:
  - Health: `http://service:3001/health`
  - Metrics: `http://service:9091/metrics`

## Configuration

### Rate Limits

| Limit | Value |
|-------|-------|
| Max burn per transaction | 1,000,000 EDSC |
| Daily burn limit | 10,000,000 EDSC |
| Confirmation blocks (Sepolia) | 12 (~3 minutes) |

### Domains

| Chain | Domain ID |
|-------|-----------|
| Ethereum | 0 |
| Solana | 1 |
| Ëtrid | 2 |
| Polygon | 3 |
| BNB Chain | 4 |

## Security

### Attestation Model
- **Threshold**: 3-of-5 signatures required
- **Key Management**: Each attester controls their own private key
- **Decentralization**: Attesters run independently

### Access Control
- **Owner**: Can pause contracts, update limits
- **MessageTransmitter**: Only contract authorized to mint/burn EDSC
- **Pausable**: Emergency pause available

### Audits
- [ ] Internal security review
- [ ] External audit (planned)
- [ ] Bug bounty program (planned)

## Monitoring

### Metrics
- Messages seen per chain
- Attestations created
- Relay success/failure rate
- Block heights
- Service uptime

### Alerts
- Service down
- Block height not advancing
- Relay failure rate > 5%
- Signature threshold not met

## Testing

### Local Testing
```bash
# Start local environment
cd contracts/ethereum
npm install
npx hardhat node  # Terminal 1

cd ../../
./target/release/flarechain-node --dev  # Terminal 2

# Deploy and test
cd contracts/ethereum
npx hardhat run scripts/deploy.js --network localhost
npx hardhat run scripts/register-attesters.js --network localhost
npx hardhat run scripts/authorize-token-messenger.js --network localhost
npx hardhat run scripts/test-transfer.js --network localhost
```

### Testnet Testing
See [EMBER_DEPLOYMENT_CHECKLIST.md](./EMBER_DEPLOYMENT_CHECKLIST.md) for comprehensive testing steps.

## Troubleshooting

### Common Issues

**Q: Transfer not completing?**
- Check attestation service logs
- Verify at least 3 services are running
- Confirm relayer has gas on both chains

**Q: "Insufficient allowance" error?**
- Must approve TokenMessenger before burning
- Approval amount must be >= transfer amount

**Q: "Daily limit exceeded"?**
- Daily limit resets every 7200 blocks (~24h on Sepolia)
- Contact team for limit increase if needed

## Resources

### Documentation
- [Architecture Overview](./ARCHITECTURE.md)
- [API Documentation](./docs/API.md)
- [Security Model](./docs/SECURITY.md)

### Links
- Sepolia Faucet: https://sepoliafaucet.com/
- Sepolia Explorer: https://sepolia.etherscan.io/
- Ember Explorer: TBD
- Discord: https://discord.gg/etrid
- GitHub: https://github.com/etrid/etrid

### Support
- Technical Issues: GitHub Issues
- Questions: Discord #bridge-support
- Security: security@etrid.io

## License

Apache 2.0

## Disclaimer

This is testnet software. Do not use with real funds. While the contracts implement security best practices, they have not been audited. Use at your own risk.

---

**Last Updated**: October 20, 2025
**Version**: 1.0.0-beta
**Network**: Ember Testnet
