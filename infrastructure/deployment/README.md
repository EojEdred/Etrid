# EDSC Bridge Testnet Deployment Guide

Complete guide for deploying the Ëtrid Dollar Stablecoin (EDSC) cross-chain bridge to public testnets.

## Overview

This deployment targets:

- **Ethereum**: Sepolia testnet
- **Substrate**: Ëtrid testnet (custom)
- **Services**: Attestation and Relayer services on cloud infrastructure

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Public Testnets                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────────┐        ┌──────────────────┐     │
│  │  Sepolia         │        │  Ember Testnet   │     │
│  │  (Ethereum)      │        │  (Substrate)     │     │
│  │                  │        │                  │     │
│  │  - EDSC Token    │        │  - Token         │     │
│  │  - Attesters     │        │    Messenger     │     │
│  │  - Transmitter   │        │  - Attestation   │     │
│  │  - Messenger     │        │    Pallet        │     │
│  └────────┬─────────┘        └────────┬─────────┘     │
│           │                           │                │
└───────────┼───────────────────────────┼────────────────┘
            │                           │
            │                           │
┌───────────┼───────────────────────────┼────────────────┐
│           │    Off-Chain Services     │                │
├───────────┼───────────────────────────┼────────────────┤
│           │                           │                │
│  ┌────────▼──────────┐       ┌───────▼────────┐       │
│  │  Attestation      │       │  Attestation   │       │
│  │  Service #1       │       │  Service #2    │       │
│  │  (Attester A)     │       │  (Attester B)  │       │
│  └────────┬──────────┘       └───────┬────────┘       │
│           │                          │                 │
│           └──────────┬───────────────┘                 │
│                      │                                 │
│           ┌──────────▼──────────┐                     │
│           │  Relayer Service    │                     │
│           │  (Permissionless)   │                     │
│           └─────────────────────┘                     │
│                                                        │
└────────────────────────────────────────────────────────┘
```

## Prerequisites

### Required Tools

- **Node.js 18+** and npm
- **Rust & Cargo** (latest stable)
- **Docker** and docker-compose
- **Git**
- **Hardhat** (Ethereum development)
- **Polkadot.js** CLI tools

### Required Accounts

1. **Ethereum Sepolia**:
   - Funded account (get testnet ETH from faucet)
   - Private key for deployment
   - Private keys for attesters (5 recommended)

2. **Ember Testnet**:
   - Sudo account (for initial setup)
   - Funded accounts for attesters
   - SR25519 keys for attesters

3. **Cloud Services** (optional but recommended):
   - AWS/GCP/Azure account for hosting services
   - Domain names for services
   - SSL certificates

### Funding Accounts

#### Get Sepolia ETH:
- [Alchemy Sepolia Faucet](https://sepoliafaucet.com/)
- [Infura Sepolia Faucet](https://www.infura.io/faucet/sepolia)
- [Chainlink Sepolia Faucet](https://faucets.chain.link/sepolia)

Minimum: 0.5 ETH per account (deployment + operations)

#### Get Ember Testnet Tokens:
- Join Discord: [discord.gg/etrid](https://discord.gg/etrid)
- Request testnet tokens in #faucet channel
- Minimum: 1000 EDSC per account

## Deployment Steps

### Phase 1: Ethereum Smart Contracts

Deploy to Sepolia testnet.

**Duration**: 30 minutes

See: [`ethereum/DEPLOYMENT.md`](./ethereum/DEPLOYMENT.md)

**Steps**:
1. Configure Hardhat for Sepolia
2. Deploy contracts (EDSC, AttesterRegistry, MessageTransmitter, TokenMessenger)
3. Configure attesters (5 addresses)
4. Set M-of-N threshold (3-of-5)
5. Verify contracts on Etherscan
6. Record contract addresses

**Output**:
```
EDSC: 0x...
AttesterRegistry: 0x...
MessageTransmitter: 0x...
TokenMessenger: 0x...
```

### Phase 2: Substrate Chain

Deploy Ëtrid testnet with pallets.

**Duration**: 1-2 hours

See: [`substrate/DEPLOYMENT.md`](./substrate/DEPLOYMENT.md)

**Steps**:
1. Build runtime with pallets
2. Generate chain specification
3. Start validator nodes (3+ recommended)
4. Configure attesters in attestation pallet
5. Set M-of-N threshold
6. Verify chain is producing blocks

**Output**:
- Chain ID: `etrid-testnet`
- WebSocket endpoint: `wss://ember-rpc.etrid.io`
- Block explorer: `https://ember-explorer.etrid.io`

### Phase 3: Attestation Services

Deploy 5 attestation services (one per attester).

**Duration**: 1-2 hours

See: [`services/ATTESTATION_DEPLOYMENT.md`](./services/ATTESTATION_DEPLOYMENT.md)

**Steps**:
1. Prepare 5 cloud instances (AWS EC2, GCP Compute, etc.)
2. Install Node.js and dependencies
3. Configure each service with unique attester keys
4. Start services with PM2/systemd
5. Configure monitoring and alerting
6. Verify health endpoints

**Output**:
```
Attester #0: https://attestation-0.etrid.io
Attester #1: https://attestation-1.etrid.io
Attester #2: https://attestation-2.etrid.io
Attester #3: https://attestation-3.etrid.io
Attester #4: https://attestation-4.etrid.io
```

### Phase 4: Relayer Services

Deploy permissionless relayer services.

**Duration**: 30 minutes

See: [`services/RELAYER_DEPLOYMENT.md`](./services/RELAYER_DEPLOYMENT.md)

**Steps**:
1. Prepare cloud instances (can be shared with attestation)
2. Configure relayer with funded accounts
3. Point to attestation service endpoints
4. Start relayer with PM2/systemd
5. Monitor relay activity

**Output**:
- Relayers can be run by anyone
- Recommended: 2-3 relayers for redundancy

### Phase 5: Monitoring & Alerting

Setup comprehensive monitoring.

**Duration**: 1 hour

See: [`monitoring/SETUP.md`](./monitoring/SETUP.md)

**Steps**:
1. Deploy Prometheus + Grafana
2. Configure service metrics
3. Setup alerting (PagerDuty, Slack, etc.)
4. Create dashboards
5. Configure log aggregation

**Output**:
- Grafana: `https://monitoring.etrid.io`
- Alerts for: downtime, failed relays, low balances

### Phase 6: Testing & Validation

Comprehensive testnet validation.

**Duration**: 2-4 hours

See: [`TESTING_GUIDE.md`](./TESTING_GUIDE.md)

**Steps**:
1. Run automated test suite against testnet
2. Manual testing of all flows
3. Stress testing (high volume)
4. Failure scenario testing
5. Performance benchmarking

**Output**:
- Test report
- Performance metrics
- Identified issues (if any)

### Phase 7: Public Announcement

Launch testnet publicly.

**Duration**: Ongoing

**Steps**:
1. Prepare documentation
2. Create user guides
3. Deploy web interface (optional)
4. Announce on social media
5. Invite community testing
6. Collect feedback

## Deployment Checklist

### Pre-Deployment

- [ ] All code reviewed and audited
- [ ] Test suite passing (100% coverage)
- [ ] Local integration tests successful
- [ ] Keys generated and secured
- [ ] Accounts funded
- [ ] Infrastructure provisioned
- [ ] Monitoring configured
- [ ] Documentation complete

### Ethereum Deployment

- [ ] Hardhat configured for Sepolia
- [ ] EDSC token deployed
- [ ] AttesterRegistry deployed
- [ ] MessageTransmitter deployed
- [ ] TokenMessenger deployed
- [ ] Attesters registered (5 addresses)
- [ ] Threshold set (3-of-5)
- [ ] Contracts verified on Etherscan
- [ ] Ownership transferred to multisig (production only)

### Substrate Deployment

- [ ] Runtime built with pallets
- [ ] Chain spec generated
- [ ] Validator nodes started (3+)
- [ ] Chain producing blocks
- [ ] Attesters registered in pallet
- [ ] Threshold set (3-of-5)
- [ ] Block explorer deployed
- [ ] RPC endpoints public

### Service Deployment

- [ ] Attestation service #0 deployed and healthy
- [ ] Attestation service #1 deployed and healthy
- [ ] Attestation service #2 deployed and healthy
- [ ] Attestation service #3 deployed and healthy
- [ ] Attestation service #4 deployed and healthy
- [ ] Relayer service deployed
- [ ] All services monitored
- [ ] Load balancers configured
- [ ] SSL certificates installed
- [ ] Backups configured

### Testing & Validation

- [ ] Ethereum → Ëtrid transfer successful
- [ ] Ëtrid → Ethereum transfer successful
- [ ] Round-trip transfer successful
- [ ] Multiple concurrent transfers successful
- [ ] High-value transfer successful (10,000+ EDSC)
- [ ] Failure scenarios tested
- [ ] Performance benchmarks met
- [ ] No critical issues found

### Public Launch

- [ ] Documentation published
- [ ] User guide available
- [ ] Web interface deployed (optional)
- [ ] Faucet available for testnet tokens
- [ ] Community channels active
- [ ] Announcement posted
- [ ] Feedback mechanism in place

## Cost Estimates

### Infrastructure (Monthly)

| Component | Provider | Instance Type | Cost/Month |
|-----------|----------|---------------|------------|
| Ethereum RPC | Alchemy/Infura | Growth plan | $50-100 |
| Substrate Validators (3x) | AWS EC2 | t3.medium | $100-150 |
| Attestation Services (5x) | AWS EC2 | t3.small | $75-100 |
| Relayer Service | AWS EC2 | t3.small | $15-25 |
| Monitoring | AWS CloudWatch | Standard | $20-30 |
| Load Balancer | AWS ALB | Standard | $20-30 |
| **Total** | | | **$280-435/month** |

### Gas Costs (Testnet)

Testnet ETH is free, but for reference:

| Operation | Gas | Cost @ 50 gwei |
|-----------|-----|----------------|
| Deploy EDSC | ~1,500,000 | ~$150 |
| Deploy Registry | ~2,000,000 | ~$200 |
| Deploy Transmitter | ~3,000,000 | ~$300 |
| Deploy Messenger | ~2,500,000 | ~$250 |
| Burn & Send | ~200,000 | ~$20 |
| Receive Message | ~300,000 | ~$30 |

**Total Deployment**: ~$900 (mainnet equivalent)

## Security Considerations

### Key Management

⚠️ **CRITICAL**: Secure all private keys!

**For Testnet**:
- Use separate keys from mainnet
- Store encrypted
- Use environment variables (never commit)
- Rotate regularly

**For Mainnet**:
- Use Hardware Security Modules (HSM)
- Use Multi-sig wallets for admin functions
- Use MPC (Multi-Party Computation) for attesters
- Implement key rotation

### Network Security

- [ ] Services behind firewalls
- [ ] Only necessary ports open (RPC, API)
- [ ] SSL/TLS for all external endpoints
- [ ] Rate limiting on public APIs
- [ ] DDoS protection (Cloudflare, AWS Shield)
- [ ] VPC networking for internal communication

### Smart Contract Security

- [ ] Ownership transferred to multisig
- [ ] Emergency pause mechanism tested
- [ ] Access controls verified
- [ ] Event emission validated
- [ ] Reentrancy guards in place
- [ ] Integer overflow protections

### Operational Security

- [ ] Monitoring and alerting active
- [ ] Log aggregation configured
- [ ] Incident response plan documented
- [ ] On-call rotation established
- [ ] Backup and recovery tested
- [ ] Regular security audits

## Rollback Plan

If critical issues are discovered:

1. **Immediate**: Pause contracts and services
2. **Assess**: Determine severity and impact
3. **Communicate**: Notify users and community
4. **Fix**: Deploy patches or new contracts
5. **Test**: Validate fixes thoroughly
6. **Resume**: Restart services
7. **Post-Mortem**: Document and learn

## Support & Contact

- **Documentation**: https://docs.etrid.io
- **Discord**: https://discord.gg/etrid
- **GitHub**: https://github.com/etrid/etrid
- **Email**: support@etrid.io
- **Status Page**: https://status.etrid.io

## License

Apache-2.0
