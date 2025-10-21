# Ëtrid Roadmap to Mainnet Deployment

**Status**: Development Phase Complete
**Current Version**: v1.0.0-dev
**Target Mainnet Date**: Q2 2026

---

## 📍 Current Status (Completed)

### ✅ Core Infrastructure
- **FlareChain Relay Chain**: Operational with Ascending Scale of Finality (ASF) consensus
- **13 Partition Burst Chains (PBCs)**: All chains compiled and tested
  - BTC-PBC, ETH-PBC, DOGE-PBC, SOL-PBC, XLM-PBC, XRP-PBC
  - BNB-PBC, TRX-PBC, ADA-PBC, LINK-PBC, MATIC-PBC, SC-USDT-PBC
  - EDSC-PBC (stablecoin chain)

### ✅ Frontend Applications
- **Mobile Wallet** (React Native + Next.js)
  - Full Polkadot.js integration
  - Multi-chain balance tracking (14 chains)
  - Real-time balance subscriptions
  - Transaction signing and sending

- **Website** (Next.js 15)
  - Homepage with feature showcase
  - Governance page with wallet connection
  - Swap page with DEX interface
  - Real blockchain integration

### ✅ Governance System
- **Consensus Day Governance Pallet** (Rust)
  - Location: `/pallets/consensus-day-governance/`
  - Vote extrinsic (Aye/Nay/Abstain)
  - Proposal submission and storage
  - Vote tracking per account
  - Tally system
  - Coinage tracking for ASF voting power
  - Frontend integration ready

---

## 🚀 Phase 1: Runtime Integration (Weeks 1-4)

### 1.1 Pallet Integration
**Timeline**: Week 1-2

**Tasks**:
- [ ] Add `pallet-consensus-day-governance` to FlareChain runtime `Cargo.toml`
- [ ] Configure pallet in `runtime/src/lib.rs`:
  ```rust
  impl pallet_consensus_day_governance::Config for Runtime {
      type RuntimeEvent = RuntimeEvent;
      type Currency = Balances;
      type MaxTitleLength = ConstU32<256>;
      type MaxDescriptionLength = ConstU32<4096>;
      type MinVotingPeriod = ConstU32<100800>; // ~7 days
      type ProposalDeposit = ConstU128<1000000000000>; // 1000 ÉTR
      type MinimumQuorum = ConstU8<33>; // 33% quorum
  }
  ```
- [ ] Add to `construct_runtime!` macro:
  ```rust
  ConsensusDayGovernance: pallet_consensus_day_governance,
  ```
- [ ] Compile runtime: `SKIP_WASM_BUILD=1 cargo build --release -p flarechain-runtime`
- [ ] Build WASM: `cargo build --release -p flarechain-runtime`

**Deliverables**:
- Working FlareChain runtime with governance pallet
- Updated chain spec with governance pallet

### 1.2 Staking Integration
**Timeline**: Week 2-3

**Tasks**:
- [ ] Enhance staking pallet to track coinage
- [ ] Add `StakeInfo` to staking pallet storage
- [ ] Implement automatic stake updates when bonding/unbonding
- [ ] Connect staking pallet to governance pallet for voting power calculation
- [ ] Test ASF formula: √(stake × coinage)

**Deliverables**:
- Integrated staking with coinage tracking
- Automated voting power calculation

### 1.3 Testing & Debugging
**Timeline**: Week 3-4

**Tasks**:
- [ ] Unit tests for all governance extrinsics
- [ ] Integration tests for voting scenarios
- [ ] Test proposal lifecycle (submit → vote → finalize)
- [ ] Benchmark extrinsics for weight calculation
- [ ] Security audit of governance logic

**Deliverables**:
- Complete test suite with >80% coverage
- Performance benchmarks
- Security audit report

---

## 🔧 Phase 2: DevNet Deployment (Weeks 5-8)

### 2.1 DevNet Setup
**Timeline**: Week 5-6

**Tasks**:
- [ ] Deploy 3-validator FlareChain network
- [ ] Configure all 13 PBC collators
- [ ] Set up monitoring infrastructure (Prometheus + Grafana)
- [ ] Deploy block explorers (Subscan/Polkadot.js Apps)
- [ ] Create faucet for test tokens

**Infrastructure**:
- **FlareChain**: 3 validators (AWS/DigitalOcean)
- **PBC Collators**: 13 collators (1 per chain)
- **RPC Endpoints**: Public WebSocket endpoints
- **Monitoring**: Metrics dashboard

### 2.2 Frontend Deployment
**Timeline**: Week 6-7

**Tasks**:
- [ ] Update RPC endpoints to DevNet
- [ ] Deploy mobile wallet to TestFlight (iOS) / Internal Testing (Android)
- [ ] Deploy website to Vercel/Netlify
- [ ] Configure CDN for static assets
- [ ] Set up error tracking (Sentry)

**Deliverables**:
- Live DevNet applications
- Public access for testing

### 2.3 Community Testing
**Timeline**: Week 7-8

**Tasks**:
- [ ] Recruit 100 beta testers
- [ ] Create testing documentation
- [ ] Bug bounty program setup
- [ ] Collect user feedback
- [ ] Iterate on UX/UI

**Key Metrics**:
- Test 1000+ transactions
- 50+ governance proposals
- 500+ votes cast

---

## 🧪 Phase 3: TestNet Launch (Weeks 9-16)

### 3.1 TestNet Infrastructure
**Timeline**: Week 9-10

**Tasks**:
- [ ] Deploy 7-validator FlareChain network
- [ ] All 13 PBC collators with redundancy
- [ ] Geographic distribution (NA, EU, APAC)
- [ ] Load balancers for RPC nodes
- [ ] Backup validators (3 additional)

**Network Specs**:
- **Validators**: 7 active + 3 backup
- **Collators**: 26 (2 per PBC)
- **RPC Nodes**: 6 (2 per region)
- **Block Explorers**: 2 instances

### 3.2 Economic Model Testing
**Timeline**: Week 10-12

**Tasks**:
- [ ] Test inflation schedule (2-3% annual)
- [ ] Simulate Consensus Day voting scenarios
- [ ] Test treasury accumulation
- [ ] Validate staking rewards distribution
- [ ] Economic attack simulations

**Scenarios**:
- Majority attack attempts
- Quorum manipulation tests
- Flash stake/unstake patterns
- Long-term coinage accumulation

### 3.3 Security Audits
**Timeline**: Week 12-14

**Tasks**:
- [ ] Runtime security audit (Trail of Bits / SRLabs)
- [ ] Pallet-specific audits
- [ ] Smart contract audits (if any)
- [ ] Infrastructure penetration testing
- [ ] Frontend security review

**Budget**: $50,000 - $100,000

### 3.4 Performance Optimization
**Timeline**: Week 14-16

**Tasks**:
- [ ] Optimize WASM runtime size
- [ ] Database performance tuning
- [ ] Network latency optimization
- [ ] Frontend bundle size reduction
- [ ] RPC response time improvement

**Targets**:
- Block time: 6 seconds ±0.5s
- Finality: 12 seconds average
- TPS: 1000+ on FlareChain
- Frontend load: <2s

---

## 🌐 Phase 4: Public TestNet (Weeks 17-24)

### 4.1 Public Launch
**Timeline**: Week 17-18

**Tasks**:
- [ ] Public announcement
- [ ] Documentation hub launch
- [ ] Developer tutorials
- [ ] Video walkthrough creation
- [ ] Social media campaign

**Content**:
- Technical whitepaper
- Economic model documentation
- Developer documentation
- User guides

### 4.2 Ecosystem Development
**Timeline**: Week 18-22

**Tasks**:
- [ ] Developer grants program
- [ ] Hackathons (3 events)
- [ ] Partner integrations
- [ ] Cross-chain bridge development
- [ ] DeFi protocol integrations

**Grants Budget**: $500,000

### 4.3 Governance Testing
**Timeline**: Week 20-24

**Tasks**:
- [ ] Real Consensus Day simulation
- [ ] 100+ governance proposals
- [ ] Test emergency proposals
- [ ] Referendum testing
- [ ] Upgrade proposals via governance

**Test Cases**:
- Inflation rate changes
- Treasury spending proposals
- Protocol upgrade proposals
- Emergency pause mechanisms

---

## 🎯 Phase 5: Mainnet Preparation (Weeks 25-32)

### 5.1 Mainnet Infrastructure
**Timeline**: Week 25-27

**Tasks**:
- [ ] 21-validator network setup
- [ ] All PBC collators (3 per chain = 39 total)
- [ ] Professional node operators recruitment
- [ ] Validator key ceremony
- [ ] Genesis configuration

**Network Topology**:
- **Validators**: 21 professional operators
- **Collators**: 39 (3 per PBC)
- **RPC Nodes**: 12 (4 per region)
- **Archive Nodes**: 3
- **Boot Nodes**: 6

### 5.2 Token Generation Event (TGE)
**Timeline**: Week 27-28

**Tasks**:
- [ ] Finalize tokenomics
- [ ] Genesis allocation
- [ ] Vesting schedules
- [ ] Airdrop distribution
- [ ] Exchange listings preparation

**Token Distribution**:
- Community: 40%
- Team & Advisors: 20% (4-year vest)
- Ecosystem Fund: 25%
- Public Sale: 10%
- Strategic Partners: 5%

### 5.3 Legal & Compliance
**Timeline**: Week 28-30

**Tasks**:
- [ ] Legal entity formation
- [ ] Token legal opinion
- [ ] Regulatory compliance review
- [ ] Terms of service
- [ ] Privacy policy

**Jurisdictions**: Switzerland/Singapore

### 5.4 Final Security Review
**Timeline**: Week 30-32

**Tasks**:
- [ ] Third-party security audit
- [ ] Bug bounty program (mainnet-level)
- [ ] Disaster recovery testing
- [ ] Incident response plan
- [ ] Insurance coverage

**Bug Bounty**: $250,000 pool

---

## 🚢 Phase 6: Mainnet Launch (Week 33-36)

### 6.1 Genesis Launch
**Timeline**: Week 33

**Day 1-3**: Genesis Block
- [ ] Genesis validators activate
- [ ] Network stabilization (24h)
- [ ] First blocks produced
- [ ] Validator coordination

**Day 4-7**: Network Activation
- [ ] Enable transfers
- [ ] PBC collators activate
- [ ] Cross-chain messaging enabled
- [ ] First cross-chain transactions

### 6.2 Application Deployment
**Timeline**: Week 34

**Tasks**:
- [ ] Mobile wallet production release
- [ ] App Store submissions
- [ ] Website production deployment
- [ ] Analytics setup
- [ ] Support infrastructure

**Platforms**:
- iOS App Store
- Google Play Store
- Web (wallet-web)

### 6.3 First Consensus Day
**Timeline**: Week 35-36 (June 1, 2026)

**Preparation**:
- [ ] Proposals submitted 2 weeks prior
- [ ] Community discussions
- [ ] Educational content
- [ ] Voting guides

**Consensus Day Activities**:
- Voting period: 7 days
- Live governance dashboard
- Real-time results
- Community celebrations

---

## 📊 Success Metrics

### Technical Metrics
- **Uptime**: >99.9%
- **Block Time**: 6 seconds ±0.5s
- **Finality**: <15 seconds
- **TPS**: >1000 transactions/second
- **Cross-chain Messages**: >100/hour

### Governance Metrics
- **Active Voters**: >10,000
- **Proposals Submitted**: >50 annually
- **Quorum Achievement**: >60%
- **Average Participation**: >40%

### Adoption Metrics
- **Active Addresses**: >100,000
- **Daily Transactions**: >50,000
- **Staking Participation**: >50% of supply
- **Developer Projects**: >100

---

## 💰 Budget Estimate

| Category | Amount | Timeline |
|----------|--------|----------|
| Development Team | $800,000 | 36 weeks |
| Infrastructure | $200,000 | Ongoing |
| Security Audits | $150,000 | Weeks 12-32 |
| Marketing | $300,000 | Weeks 17-36 |
| Legal & Compliance | $100,000 | Weeks 28-32 |
| Ecosystem Grants | $500,000 | Weeks 18-36 |
| Bug Bounties | $250,000 | Ongoing |
| **Total** | **$2,300,000** | **36 weeks** |

---

## 🛡️ Risk Mitigation

### Technical Risks
- **Mitigation**: Extensive testing, audits, gradual rollout
- **Contingency**: Rollback mechanisms, emergency governance

### Economic Risks
- **Mitigation**: Economic modeling, simulations, testnet trials
- **Contingency**: Parameter adjustment via governance

### Regulatory Risks
- **Mitigation**: Legal counsel, compliance framework
- **Contingency**: Jurisdictional flexibility

### Security Risks
- **Mitigation**: Multi-layer security, audits, bounties
- **Contingency**: Insurance, emergency response team

---

## 📞 Team & Resources

### Core Team
- **Blockchain Engineers**: 3-4
- **Frontend Developers**: 2-3
- **DevOps Engineers**: 2
- **Product Manager**: 1
- **Community Manager**: 1
- **Marketing Lead**: 1

### External Partners
- **Security Auditors**: 2-3 firms
- **Legal Counsel**: 1 firm
- **Marketing Agency**: 1
- **Node Operators**: 20+

---

## 🔗 Next Immediate Steps

1. **Week 1**: Integrate governance pallet into FlareChain runtime
2. **Week 2**: Compile and test updated runtime
3. **Week 3**: Deploy to local devnet
4. **Week 4**: Begin staking integration
5. **Week 5**: Start DevNet infrastructure setup

---

**Document Version**: 1.0
**Last Updated**: October 2025
**Next Review**: After Phase 1 completion

---

## Appendix: File Locations

### Governance Pallet
- **Path**: `/pallets/consensus-day-governance/`
- **Files**:
  - `Cargo.toml` - Dependencies and features
  - `src/lib.rs` - Main pallet implementation

### Frontend Integration
- **Website**: `/apps/wallet-web/etrid-crypto-website/`
- **Mobile Wallet**: `/apps/wallet-mobile/etrid-wallet/`
- **Polkadot.js Integration**: `/apps/wallet-web/etrid-crypto-website/lib/polkadot/`

### Blockchain
- **FlareChain Runtime**: TBD (find location)
- **PBC Runtimes**: `/05-multichain/partition-burst-chains/pbc-chains/*/runtime/`

---

*This roadmap is subject to change based on technical discoveries, security findings, and market conditions.*
