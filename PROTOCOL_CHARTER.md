# ËTRID PROTOCOL CHARTER

**Effective Date**: January 1, 2026 (Pending Community Ratification)
**Version**: 1.0
**Status**: Draft - Awaiting First Consensus Day Vote
**Governing Law**: Delaware, USA (or Zug, Switzerland)
**License**: GPLv3 for Code, CC BY-SA 4.0 for Documentation

---

## Preamble

We, the founding contributors and community members of the Ëtrid Protocol, hereby establish this Charter to create a decentralized, transparent, and sustainable blockchain ecosystem that serves as a multi-chain infrastructure connecting major blockchains while maintaining the highest standards of security, fairness, and innovation.

This Charter establishes the foundational governance structure, economic principles, and operational procedures for the Ëtrid Protocol and the Ëtrid Foundation.

**Core Principles**:
- **Decentralization**: No single entity shall control the network
- **Transparency**: All governance decisions shall be public and on-chain
- **Meritocracy**: Leadership based on expertise and contribution, not wealth alone
- **Sustainability**: Long-term viability over short-term gains
- **Openness**: Open-source code (GPLv3), open participation, open governance

---

## Article I: Legal Entity & Structure

### Section 1.1: Ëtrid Foundation

**Legal Formation**: The Ëtrid Foundation shall be incorporated as a Delaware Non-Profit Corporation (or Swiss Stiftung equivalent) with the following properties:

- **Name**: ËTRID Foundation
- **Jurisdiction**: Delaware, USA (or Zug, Switzerland as determined by First Consensus Day vote)
- **Tax Status**: 501(c)(3) Non-Profit or equivalent
- **Purpose**: Steward the development, maintenance, and growth of the Ëtrid Protocol

**Mission Statement**:
> "To build a decentralized, multi-chain blockchain infrastructure that empowers developers, users, and AI systems to transact freely, securely, and efficiently without centralized intermediaries."

### Section 1.2: Organizational Structure

```
ËTRID Foundation
├── Board of Decentralized Directors (9 members)
│   ├── Technical Committee (3 Directors)
│   ├── Legal & Compliance Committee (2 Directors)
│   ├── Community Committee (2 Directors)
│   └── Security Committee (2 Directors)
├── Foundation Treasury (Multi-Sig 5-of-9, 7-of-9 for large expenditures)
├── Validator Network (21 Flare Nodes + 104 Collators)
├── Development Teams (Employed or contracted by Foundation)
└── Community (All ÉTR holders, stakers, and participants)
```

### Section 1.3: Intellectual Property

**Open-Source Commitment**: All protocol code shall be licensed under **GNU General Public License v3.0 (GPLv3)** with the following provisions:

1. **Source Code**: Publicly available on GitHub (https://github.com/etrid/etrid)
2. **Copyleft**: Derivative works must also be GPLv3
3. **Patent Grant**: Contributors grant irrevocable patent license; defensive termination clause applies
4. **No Warranty**: Software provided "as is" without warranty

**Trademarks**: The following marks are registered trademarks of the Ëtrid Foundation:
- **ËTRID™** (Protocol name and logo)
- **ËDSC™** (Stablecoin name)
- **FlareChain™** (Relay chain name)
- **Partition Burst Chain™** / **PBC™** (Parachain architecture)

**Trademark Policy**:
- ✅ **Permitted**: Educational use, attribution in derivative works, community projects with proper attribution
- ❌ **Prohibited**: Commercial products claiming to be "official ËTRID", misleading use, domain squatting

---

## Article II: Governance Structure

### Section 2.1: Decentralized Directors

**Board Size**: 9 Directors (odd number to prevent tie votes)

**Election Process**:
1. **Annual Election**: Every Consensus Day (Third Friday of November)
2. **Term Length**: 1 year (renewable up to 3 consecutive terms)
3. **Method**: Ranked-Choice Voting (Instant Runoff)
4. **Eligibility**:
   - Active community member for minimum 6 months
   - Minimum 1,000 ÉTR staked (refundable nomination deposit: 100 ÉTR)
   - No felony convictions related to fraud or financial crimes
   - Background check required (privacy-preserving)
   - Must disclose conflicts of interest

**Responsibilities**:
- Approve budgets and treasury expenditures
- Review and approve protocol upgrades
- Represent Foundation in legal matters
- Emergency decision-making authority
- Steward long-term protocol development

**Compensation**: 608.78 ÉTR/day (222,204 ÉTR/year) per Director
- 50% paid immediately (monthly)
- 25% vested over 6 months
- 25% vested over 12 months

**Performance Requirements**:
- Minimum 90% meeting attendance
- Community rating > 3.0 stars (5-star scale)
- Disclose all conflicts of interest

**Removal**: Directors may be removed by:
- 7-of-9 Director vote + 60% community approval
- Missing > 10% of meetings
- Community rating < 3.0 for 2 consecutive quarters
- Undisclosed conflict of interest or fraud

### Section 2.2: Committee Structure

**Technical Committee** (3 Directors):
- Review technical proposals and protocol upgrades
- Evaluate security audits and bug reports
- Oversee bug bounty program
- Approve micro-grants (< 1,000 ÉTR)

**Legal & Compliance Committee** (2 Directors):
- Monitor regulatory landscape
- Ensure legal compliance (securities, tax, AML/KYC)
- Manage IP and trademarks
- Handle legal disputes

**Community Committee** (2 Directors):
- Organize community events and outreach
- Manage grant program
- Coordinate with ambassadors
- Gather community feedback

**Security Committee** (2 Directors):
- Oversee security practices and incident response
- Manage vulnerability disclosures
- Review circuit breaker triggers
- Coordinate emergency procedures

### Section 2.3: Voting Rights & Weight

**Stake-Weighted Voting Formula**:
```
Voting Power = Staked ÉTR × Coinage Multiplier × Participation Bonus

Where:
- Coinage Multiplier (1.0x - 2.0x based on stake duration):
  * 0-6 months: 1.0x
  * 6-12 months: 1.25x
  * 12-18 months: 1.5x
  * 18-24 months: 1.75x
  * 24+ months: 2.0x (maximum)

- Participation Bonus (1.0x - 1.2x based on voting history):
  * First-time voter: 1.0x
  * Voted in 1 previous Consensus Day: 1.1x
  * Voted in 2+ previous Consensus Days: 1.2x (maximum)
```

**Participant Tiers**:
1. **Decentralized Directors** (9 members): 1 vote each (9 votes total)
2. **Validators** (125 total): 1 vote each (125 votes total)
3. **Active Stakers** (~1,000 estimated): Stake-weighted (majority of voting power)
4. **Voters** (No stake requirement): 1 vote per identity-verified account (signaling only)

### Section 2.4: Consensus Day Annual Voting

**Date**: Third Friday of November (annually)
**Voting Window**: 24 hours (00:00 UTC to 23:59 UTC)

**2025-2030 Schedule**:
- November 21, 2025 (First Consensus Day)
- November 20, 2026
- November 19, 2027
- November 17, 2028
- November 16, 2029
- November 15, 2030

**Mandatory Annual Votes**:
1. **Director Election** (9 Directors via Ranked-Choice Voting)
2. **Annual Mint Rate** (Current: 10M ÉTR/year, proposed: -10% annually to 2% floor)
3. **Distribution Allocation** (Current: 40% Foundation, 30% Validators, 20% Directors, 10% Voters)
4. **Foundation Budget** (Annual operating budget with itemized breakdown)
5. **Protocol Upgrade Proposals** (If any submitted during year)

**Approval Thresholds**:
- **Standard Proposals**: 60% approval, 20% quorum
- **Critical Proposals** (consensus, economics): 70% approval, 40% quorum
- **Constitutional Amendments**: 80% approval, 60% quorum

---

## Article III: Economic System & Distribution

### Section 3.1: Native Token (ÉTR)

**Token Name**: Ëtrid (ÉTR)
**Decimals**: 18
**Total Supply**: Uncapped (inflationary with governance-controlled mint rate)
**Current Inflation**: 10,000,000 ÉTR/year (27,397 ÉTR/day)

**Supply Schedule** (Proposed):
```
Year 1 (2026): 10,000,000 ÉTR (100% of baseline)
Year 2 (2027): 9,000,000 ÉTR (90% reduction)
Year 3 (2028): 8,100,000 ÉTR (10% reduction)
...
Year 10+ : 2,000,000 ÉTR/year (2% minimum floor)
```

### Section 3.2: Daily Distribution Allocation

**Total Daily**: 27,397 ÉTR

**Allocation Breakdown**:

| Category | Percentage | Daily (ÉTR) | Annual (ÉTR) | Recipients | Distribution Time (UTC) |
|----------|-----------|-------------|--------------|------------|------------------------|
| **Foundation Treasury** | 40% | 10,959 | 4,000,000 | Multi-Sig (5-of-9) | Continuous |
| **Validators** | 30% | 8,219 | 3,000,000 | 21 Flare + 104 Collators | Flare: 04:01, Validity: 06:01 |
| **Directors** | 20% | 5,479 | 2,000,000 | 9 Directors | 12:01 PM |
| **Voters** | 10% | 2,740 | 1,000,000 | Consensus Day Participants | 00:01 AM (daily after vote) |

**Validator Distribution**:
- **Flare Nodes** (15%): 4,110 ÉTR/day ÷ 21 = **195.71 ÉTR/day per node**
- **Validity Nodes** (15%): 4,109 ÉTR/day ÷ 104 = **39.51 ÉTR/day per collator**

**Rationale**: Flare Nodes secure entire relay chain (higher responsibility) vs. Validity Nodes securing individual PBCs (lower responsibility). Ratio: ~5:1

### Section 3.3: Foundation Treasury Management

**Annual Budget**: 4,000,000 ÉTR/year (~10,959 ÉTR/day)

**Allocation Breakdown**:
```
Development (50%):     2,000,000 ÉTR/year
Security (20%):        800,000 ÉTR/year
Marketing (15%):       600,000 ÉTR/year
Legal (10%):           400,000 ÉTR/year
Operations (5%):       200,000 ÉTR/year
```

**Expenditure Approval Tiers**:

| Amount | Approver | Timeframe | Community Review |
|--------|----------|-----------|------------------|
| < 1,000 ÉTR | Technical Committee (3 members) | 24 hours | No |
| 1,000 - 10,000 ÉTR | 5-of-9 Directors | 48 hours | No |
| 10,000 - 100,000 ÉTR | 7-of-9 Directors | 7 days | 3-day review period |
| > 100,000 ÉTR | 7-of-9 Directors + 60% Community Approval | 30 days | Public RFC required |

**Reserve Requirements**:
- **Operating Reserve** (20%): 800K ÉTR (3 months operating costs)
- **Security Reserve** (10%): 400K ÉTR (bug bounties, audits)
- **Strategic Reserve** (10%): 400K ÉTR (partnerships, acquisitions)

**Transparency**:
- Real-time treasury dashboard: https://treasury.etrid.network
- Quarterly financial reports (within 15 days of quarter end)
- Annual audit by independent accounting firm

### Section 3.4: Coinage Multiplier System

**Purpose**: Reward long-term staking and reduce circulating supply

**Formula**:
```rust
Reward = Base Reward × Coinage Multiplier

fn calculate_coinage_multiplier(stake_duration_days: u64) -> u32 {
    match stake_duration_days {
        0..=180 => 10000,    // 1.0x (0-6 months)
        181..=365 => 12500,  // 1.25x (6-12 months)
        366..=545 => 15000,  // 1.5x (12-18 months)
        546..=730 => 17500,  // 1.75x (18-24 months)
        _ => 20000,          // 2.0x (24+ months, max)
    }
}
```

**Example**:
```
Alice: 10,000 ÉTR staked for 24+ months
Reward = 100 ÉTR × 2.0 = 200 ÉTR

Bob: 20,000 ÉTR staked for 3 months
Reward = 100 ÉTR × 1.0 = 100 ÉTR

Result: Alice receives 2x rewards despite having half the stake
```

### Section 3.5: Penalty System

**Validator Penalties**:

| Offense | Penalty | Recovery |
|---------|---------|----------|
| **Missed Block** | 0.1% of daily reward per miss | 24 hours no misses |
| **Offline (1 hour)** | 0.01% per hour | Come back online |
| **Double-Sign** | 10% slash of total stake + removal | 90-day ban |

**Director Penalties**:

| Offense | Penalty | Action |
|---------|---------|--------|
| **Missed Meeting** | 2% of daily compensation | Warning after 3 consecutive |
| **Community Rating < 3.0** | -10% compensation (1 quarter) | Removal vote if 2 quarters |
| **Undisclosed Conflict** | -50% compensation + reprimand | Potential removal |
| **Bribery/Fraud** | Immediate removal + legal action | Permanent blacklist |

---

## Article IV: Protocol Governance

### Section 4.1: Upgrade Proposal Process

**Phase 1: RFC (Request for Comments)** - 30 days
- Technical specification
- Implementation plan
- Security analysis
- Community impact assessment
- Backwards compatibility analysis

**Phase 2: Technical Review** - 14 days
- Technical Committee review
- External auditor review (if budget allows)
- Core developer review
- Community technical expert review

**Phase 3: Governance Vote** - 7 days
- On-chain voting
- Standard: 60% approval, 20% quorum
- Critical: 70% approval, 40% quorum
- Constitutional: 80% approval, 60% quorum

**Phase 4: Implementation**
- Code review (2+ core developers)
- Testnet deployment (14 days minimum)
- Community testing period
- Mainnet deployment via Substrate runtime upgrade

### Section 4.2: Emergency Procedures

**Trigger Conditions**:
- Critical security vulnerability
- Network halt or severe performance degradation
- Active exploit of protocol
- Legal/regulatory emergency

**Fast-Track Process**:
```
1. Security Committee identifies issue (< 1 hour)
2. Emergency patch developed (< 24 hours)
3. 3-of-9 Directors approve (< 4 hours)
4. Deploy to testnet (< 2 hours)
5. If stable, deploy to mainnet immediately
```

**Post-Mortem Requirements**:
- Detailed incident report (within 7 days)
- Community AMA
- Retroactive community vote to ratify (60% approval)
- If rejected, revert and find alternative solution

### Section 4.3: Circuit Breaker System

**Automatic Triggers**:
- Daily withdrawal limit exceeded (e.g., > $10M/day for BTC bridge)
- Oracle price deviation > 20%
- Suspicious activity patterns detected

**Manual Triggers**:
- Any 3 Directors can activate
- Must provide on-chain justification
- Community notified immediately

**Resolution Timeline**:
1. Investigation (0-6 hours)
2. Fix development (6-24 hours)
3. Community discussion (24-48 hours)
4. Deploy fix (48-72 hours)

---

## Article V: Security & Legal

### Section 5.1: Bug Bounty Program

**Reward Tiers**:

| Severity | Reward (ÉTR) | Reward (USD) | Examples |
|----------|--------------|--------------|----------|
| **Critical** | 50,000 - 500,000 | Up to $1,000,000 | Consensus failure, bridge fund drain, RCE |
| **High** | 5,000 - 50,000 | - | Significant vulnerability, data leak |
| **Medium** | 1,000 - 5,000 | - | Non-critical security issue |
| **Low** | 100 - 1,000 | - | Documentation errors, typos |

**Bridge-Specific**:
- Critical bridge vulnerability: Up to **$1,000,000 USD** (paid in USDC/USDT)

**Submission Process**:
1. Email: security@etrid.network (PGP encrypted)
2. Include detailed reproduction steps
3. Wait 48 hours for acknowledgment
4. Cooperate on 30-90 day responsible disclosure
5. Payment within 7 days of verification

### Section 5.2: Regulatory Compliance

**AML/KYC**:
- Foundation maintains AML program
- KYC required for:
  - Directors (background check)
  - Custodians (regulatory license)
  - Large grant recipients (> 100,000 ÉTR)
- Not required for general users (permissionless protocol)

**Securities Compliance**:
- ÉTR not marketed as investment
- No promises of profit
- Utility-focused messaging
- Legal opinion obtained before token launch

**Tax Reporting**:
- Foundation files annual 990 (if US 501c3)
- Directors receive 1099 (if US tax residents)
- Users responsible for own tax reporting

### Section 5.3: Dispute Resolution

**Internal Disputes**:
1. Good faith negotiation (14 days)
2. Mediation by neutral Director committee (30 days)
3. Binding arbitration (if mediation fails)

**External Disputes**:
- Jurisdiction: Delaware courts (if Delaware corp) or Swiss courts (if Stiftung)
- Governing law: Delaware law or Swiss law
- Arbitration: JAMS or Swiss Chambers' Arbitration Institution

---

## Article VI: Amendments

### Section 6.1: Amendment Process

This Charter may be amended by:

1. **Standard Amendment** (non-constitutional):
   - Proposal submitted 60 days before Consensus Day
   - 7-of-9 Director approval
   - 60% community approval
   - 20% quorum

2. **Constitutional Amendment** (governance structure, economic model):
   - Proposal submitted 90 days before Consensus Day
   - 9-of-9 Director approval OR 7-of-9 + 80% community approval
   - 60% quorum
   - 30-day public comment period

3. **Emergency Amendment**:
   - 7-of-9 Directors
   - Retroactive community ratification at next Consensus Day
   - If rejected, amendment automatically reverts

### Section 6.2: Ratification

This Charter shall become effective upon:
1. **First Consensus Day** (November 21, 2025)
2. **80% approval** by voting community
3. **60% quorum** of staked ÉTR participating
4. **Legal incorporation** of Ëtrid Foundation

Until ratification, this document serves as a social contract and guiding principles for the Ëtrid community.

---

## Article VII: Dissolution

### Section 7.1: Conditions for Dissolution

The Ëtrid Foundation may be dissolved only upon:

1. **Community Vote**:
   - 90% approval (supermajority)
   - 75% quorum
   - Conducted at Consensus Day
   - 180-day notice period

2. **Force Majeure**:
   - Government prohibition
   - Insurmountable technical failure
   - Requires 7-of-9 Director approval + emergency community vote

### Section 7.2: Asset Distribution

Upon dissolution, remaining treasury assets shall be distributed:

1. **First Priority**: Pay all outstanding liabilities and legal obligations
2. **Second Priority**: Return staked assets to validators and stakers
3. **Third Priority**: Distribute remainder pro-rata to ÉTR holders OR
4. **Alternative**: Donate to non-profit open-source blockchain organization

Method determined by final dissolution vote.

---

## Signatures & Attestation

**Proposed by**: Founding Contributors of the Ëtrid Project

**Date**: November 16, 2025

**Ratification Vote**: To occur at First Consensus Day (November 21, 2025)

**This Charter is a living document.** The latest version shall always be available at:
- **On-Chain**: Deployed as immutable smart contract on FlareChain
- **GitHub**: https://github.com/etrid/etrid/blob/main/PROTOCOL_CHARTER.md
- **Documentation**: https://docs.etrid.network/charter

---

## Appendix A: Definitions

**Terms used in this Charter**:

- **ÉTR**: Native cryptocurrency of the Ëtrid Protocol
- **ËDSC**: Ëtrid stablecoin (not yet launched)
- **FlareChain**: Layer 1 relay chain using GRANDPA finality
- **PBC**: Partition Burst Chain (Layer 2 parachains)
- **Lightning-Bloc**: Layer 3 payment channel network
- **Consensus Day**: Annual governance voting day (Third Friday of November)
- **Director**: Member of the 9-person Decentralized Director board
- **Coinage**: Time-weighted reward multiplier based on stake duration
- **Quorum**: Minimum participation required for valid vote
- **Supermajority**: >66% or >80% depending on context
- **Multi-Sig**: Multi-signature wallet requiring M-of-N approvals
- **Circuit Breaker**: Emergency pause mechanism for security
- **Stake**: Locked ÉTR used for network security and governance
- **Slashing**: Penalty via confiscation of staked tokens

---

## Appendix B: Document History

| Version | Date | Changes | Approved By |
|---------|------|---------|-------------|
| 1.0 | Nov 16, 2025 | Initial draft | Founding contributors |
| TBD | Nov 21, 2025 | Ratification (pending) | First Consensus Day vote |

---

## Appendix C: Related Documents

**Governance Documents**:
- Ivory Paper Volume 3: Governance & Economics (`docs/specifications/ivory-paper-vol3-governance.md`)
- Ivory Paper Volume 2: Technical Specifications (`docs/specifications/ivory-paper-vol2-technical.md`)
- Ivory Paper Volume 1: Overview (`docs/specifications/ivory-paper.md`)

**Technical Documents**:
- API Reference (`docs/API_REFERENCE.md`)
- Architecture Documentation (`docs/architecture.md`)
- Developer Guide (`docs/DEVELOPER_GUIDE.md`)

**Legal Documents**:
- GPLv3 License (`LICENSE`)
- Trademark Policy (TBD)
- Privacy Policy (TBD)
- Terms of Service (TBD)

---

**END OF CHARTER**

*This charter represents the collective will of the Ëtrid community to build a transparent, decentralized, and sustainable multi-chain blockchain infrastructure.*

*For questions or comments, contact: governance@etrid.network*

*Last Updated: November 16, 2025*
*Document Hash (SHA-256): [To be computed after ratification]*
