# ËTRID IVORY PAPER - VOLUME 3: GOVERNANCE & ECONOMICS

**Version**: 2.0-Governance
**Last Updated**: November 16, 2025
**Status**: Living Document
**License**: GPLv3
**Audience**: Community Members, Governance Participants, Policy Makers

---

## Document Overview

This Volume 3 companion to the main Ivory Paper provides comprehensive governance and economic policy documentation for the Ëtrid blockchain ecosystem. It details the decentralized governance model, distribution economics, legal framework, and community participation mechanisms.

**Key Topics**:
- Foundation Charter & Legal Structure
- Consensus Day Annual Voting
- Distribution Pay Economics (27,397 ÉTR/day)
- Decentralized Directors (9-member board)
- Treasury Management & Multi-Sig
- Protocol Upgrade Governance
- Community Participation & Voting

---

## Table of Contents

### Part I: Legal & Organizational Structure
1. [ËTRID Foundation Charter](#1-ëtrid-foundation-charter)
   - 1.1 Legal Entity Structure
   - 1.2 Mission & Vision
   - 1.3 Governance Principles
   - 1.4 Intellectual Property & Licensing

### Part II: Governance Structure
2. [Decentralized Directors](#2-decentralized-directors)
   - 2.1 Election Process
   - 2.2 Roles & Responsibilities
   - 2.3 Term Limits & Rotation
   - 2.4 Compensation & Incentives

3. [Consensus Day Voting](#3-consensus-day-voting)
   - 3.1 Annual Schedule (Third Friday of November)
   - 3.2 Voting Topics & Proposals
   - 3.3 Participant Categories
   - 3.4 Voting Mechanics & Weight

### Part III: Economic Systems
4. [Distribution Pay Economics](#4-distribution-pay-economics)
   - 4.1 Daily Distribution Schedule (27,397 ÉTR)
   - 4.2 Allocation Breakdown (40/30/20/10)
   - 4.3 Coinage Multiplier System
   - 4.4 Penalty & Reward Mechanisms

5. [Treasury Management](#5-treasury-management)
   - 5.1 Foundation Treasury (40% allocation)
   - 5.2 Multi-Signature Requirements
   - 5.3 Budget Approval Process
   - 5.4 Transparency & Reporting

### Part IV: Decision-Making Processes
6. [Protocol Upgrades](#6-protocol-upgrades)
   - 6.1 Proposal Submission
   - 6.2 Technical Review
   - 6.3 Community Discussion Period
   - 6.4 Voting & Implementation

7. [Emergency Procedures](#7-emergency-procedures)
   - 7.1 Emergency Fund Access
   - 7.2 Circuit Breaker Activation
   - 7.3 Security Incident Response
   - 7.4 Disaster Recovery

### Part V: Community & Participation
8. [Voter Participation](#8-voter-participation)
   - 8.1 Eligibility Requirements
   - 8.2 Stake-Weighted Voting
   - 8.3 Voter Rewards (10% of daily distribution)
   - 8.4 Delegation Mechanisms

9. [Community Development](#9-community-development)
   - 9.1 Bug Bounty Program
   - 9.2 Grant Program
   - 9.3 Developer Incentives
   - 9.4 Education & Outreach

---

# PART I: LEGAL & ORGANIZATIONAL STRUCTURE

## 1. ËTRID Foundation Charter

### 1.1 Legal Entity Structure

**Entity Type**: Delaware Non-Profit Corporation (or Swiss Stiftung equivalent)

**Incorporation Details**:
- **Name**: ËTRID Foundation
- **Jurisdiction**: Delaware, USA (or Zug, Switzerland)
- **Registration Date**: TBD (pending community vote)
- **Tax Status**: 501(c)(3) non-profit or equivalent

**Legal Structure**:
```
ËTRID Foundation (Delaware Non-Profit)
├── Board of Directors (9 Decentralized Directors)
│   ├── Technical Committee (3 members)
│   ├── Legal & Compliance Committee (2 members)
│   ├── Community Committee (2 members)
│   └── Security Committee (2 members)
├── Foundation Treasury (Multi-Sig 5-of-9)
├── Development Teams (Contractors)
└── Community (Voters, Validators, Stakers)
```

**Governance Model**: Decentralized Autonomous Organization (DAO) with legal wrapper

**Key Principles**:
1. **Decentralization**: No single point of control
2. **Transparency**: All decisions public and on-chain
3. **Meritocracy**: Decisions based on expertise and contribution
4. **Inclusivity**: Open participation for all community members
5. **Sustainability**: Long-term viability over short-term gains

### 1.2 Mission & Vision

**Mission Statement**:
> "To build a decentralized, multi-chain blockchain infrastructure that empowers developers, users, and AI systems to transact freely, securely, and efficiently without centralized intermediaries."

**Core Values**:
- **Openness**: Open-source code (GPLv3), open governance
- **Security**: Prioritize security over features
- **Interoperability**: Bridge blockchains, not isolate them
- **Innovation**: Push boundaries of blockchain technology
- **Community**: Serve the community, not shareholders

**Long-Term Vision** (10-year roadmap):
- **2025-2026**: Launch mainnet, establish 13 PBC bridges
- **2027-2028**: Scale to 1M+ daily active users
- **2029-2030**: Integrate 50+ external blockchains
- **2031-2035**: Become top 10 blockchain by market cap and usage

### 1.3 Governance Principles

#### Principle 1: One Token, One Vote (Stake-Weighted)

Voting power is proportional to ÉTR staked, ensuring that those with long-term commitment have greater influence.

**Formula**:
```
Voting Power = Staked ÉTR × Coinage Multiplier

Where Coinage Multiplier:
- 0-6 months: 1.0x
- 6-12 months: 1.25x
- 12-18 months: 1.5x
- 18-24 months: 1.75x
- 24+ months: 2.0x (max)
```

**Example**:
```
Alice: 10,000 ÉTR staked for 24+ months
Voting Power = 10,000 × 2.0 = 20,000 votes

Bob: 20,000 ÉTR staked for 3 months
Voting Power = 20,000 × 1.0 = 20,000 votes

Result: Equal voting power despite different stake amounts
```

#### Principle 2: Checks and Balances

No single entity can unilaterally control the network.

**Multi-Signature Requirements**:
- **Treasury Spending** (< 10,000 ÉTR): 5-of-9 Directors
- **Treasury Spending** (> 10,000 ÉTR): 7-of-9 Directors + 3-day community veto period
- **Protocol Upgrades**: 7-of-9 Directors + 3-day community review
- **Emergency Actions**: 3-of-9 Directors (for urgent security fixes)

#### Principle 3: Transparency

All governance actions are public and auditable.

**Transparency Requirements**:
- All votes recorded on-chain
- All treasury transactions public
- Quarterly financial reports (ÉTR + USD value)
- Annual governance report
- Real-time dashboard: https://gov.etrid.network

#### Principle 4: Meritocracy

Leadership roles based on expertise and contribution, not wealth alone.

**Director Qualifications**:
- Minimum 6 months active community participation
- Demonstrated technical expertise OR community leadership
- No criminal record related to fraud or financial crimes
- Background check required (privacy-preserving)

### 1.4 Intellectual Property & Licensing

#### Open-Source Licensing (GPLv3)

**GNU General Public License v3**:
```
ËTRID - Decentralized Multi-Chain Blockchain
Copyright (C) 2024-2025 ËTRID Foundation

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

**Key Provisions**:
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Private use allowed
- ❌ Liability: No warranty provided
- ❌ Patent use: Contributors grant patent license
- ⚠️ **Copyleft**: Derivative works must also be GPLv3

**Trademark Policy**:
- **ËTRID™**: Registered trademark of ËTRID Foundation
- **ËDSC™**: Registered trademark of ËTRID Foundation
- **Permitted Uses**:
  - Educational and informational use
  - Attribution to ËTRID in derivative works
  - Community projects with attribution
- **Prohibited Uses**:
  - Commercial products claiming to be "official ËTRID"
  - Misleading use suggesting endorsement
  - Domain squatting (etrid.com, etrid.io, etc.)

**Patent Policy**:
- Foundation will NOT file patents on ËTRID technology
- All contributors grant irrevocable patent license
- Defensive termination: If you sue for patent infringement, your license terminates

---

# PART II: GOVERNANCE STRUCTURE

## 2. Decentralized Directors

### 2.1 Board Structure

**Size**: 9 Directors (odd number to prevent ties)

**Election Cycle**: Annual (every Consensus Day)

**Term Length**: 1 year (renewable up to 3 consecutive terms)

**Rotation**: Minimum 3 directors rotated each year to ensure fresh perspectives

### 2.2 Election Process

#### Phase 1: Nomination (30 days before Consensus Day)

**Eligibility Criteria**:
- Active community member for minimum 6 months
- Minimum 1,000 ÉTR staked (to prevent spam candidates)
- No felony convictions related to fraud or financial crimes
- Must disclose conflicts of interest

**Self-Nomination**:
```
1. Submit nomination on-chain via governance pallet
2. Pay nomination deposit: 100 ÉTR (refunded if elected or receive >5% votes)
3. Submit candidate statement (max 500 words)
4. Optional: Submit campaign video (max 3 minutes)
```

**Nomination Review**:
- Technical Committee verifies eligibility
- Legal Committee performs background check
- Community discussion period (7 days)
- Candidates can be challenged (requires 1,000 ÉTR stake + valid reason)

#### Phase 2: Campaign (21 days before Consensus Day)

**Campaign Activities**:
- Public forums & AMAs (Ask Me Anything)
- Written statements published on governance portal
- Video presentations (optional)
- Community Discord/Telegram discussions

**Campaign Rules**:
- ❌ No vote buying (bribery)
- ❌ No coordinated attack campaigns
- ❌ No impersonation of other candidates
- ✅ Debate encouraged
- ✅ Critique of policies allowed

#### Phase 3: Voting (Consensus Day)

**Voting Method**: Ranked-Choice Voting (Instant Runoff)

**How It Works**:
```
1. Each voter ranks candidates in order of preference (1st, 2nd, 3rd, ...)
2. First-choice votes tallied
3. If no candidate has majority:
   a. Candidate with fewest votes eliminated
   b. Their votes redistributed to next choice
   c. Repeat until 9 directors elected
```

**Example**:
```
Round 1:
  Alice: 30% (3,000 votes)
  Bob: 25% (2,500 votes)
  Carol: 20% (2,000 votes)
  Dave: 15% (1,500 votes)
  Eve: 10% (1,000 votes) ← ELIMINATED

Round 2 (Eve's votes redistributed):
  Alice: 35% (3,500 votes)
  Bob: 30% (3,000 votes)
  Carol: 25% (2,500 votes)
  Dave: 10% (1,000 votes) ← ELIMINATED

... continue until 9 directors elected
```

**Voting Weight**:
```
Voting Power = Staked ÉTR × Coinage Multiplier × Voting Participation Bonus

Where Voting Participation Bonus:
- First-time voter: 1.0x
- Voted in 1 previous Consensus Day: 1.1x
- Voted in 2+ previous Consensus Days: 1.2x (max)
```

#### Phase 4: Results & Transition (Day after Consensus Day)

**Results Announcement**:
- Results published immediately on-chain
- 9 directors with most votes elected
- New directors take office January 1st (45-day transition period)

**Transition Process**:
```
Nov 15 (Consensus Day): Election held
Nov 16-Dec 31: Transition period
  - Outgoing directors brief incoming directors
  - Multi-sig keys transferred
  - Committee assignments finalized
Jan 1: New directors officially take office
```

### 2.3 Roles & Responsibilities

#### Core Responsibilities

**1. Treasury Management**
- Approve budgets and expenditures
- Multi-sig approval for transactions
- Quarterly financial reporting

**2. Protocol Governance**
- Review and approve protocol upgrades
- Emergency decision-making authority
- Security incident response

**3. Legal & Compliance**
- Ensure regulatory compliance
- Manage legal risks
- Represent Foundation in legal matters

**4. Community Leadership**
- Engage with community
- Communicate governance decisions
- Address community concerns

#### Committee Structure

**Technical Committee** (3 Directors)
- Review technical proposals
- Evaluate protocol upgrade risks
- Oversee bug bounty program
- Conduct security audits

**Legal & Compliance Committee** (2 Directors)
- Monitor regulatory landscape
- Ensure legal compliance
- Manage IP and trademarks
- Handle legal disputes

**Community Committee** (2 Directors)
- Organize community events
- Manage grant program
- Coordinate with ambassadors
- Gather community feedback

**Security Committee** (2 Directors)
- Oversee security practices
- Manage security incidents
- Coordinate vulnerability disclosures
- Review circuit breaker triggers

**Note**: Directors can serve on multiple committees, with at least 1 technical expert on Technical Committee.

### 2.4 Compensation & Incentives

#### Director Compensation

**Daily Distribution**: 5,479 ÉTR/day (20% of total)

**Allocation Per Director**: 5,479 / 9 = **608.78 ÉTR/day** ≈ **222,204 ÉTR/year**

**Vesting Schedule**:
```
- 50% paid immediately (monthly)
- 25% vested over 6 months
- 25% vested over 12 months
```

**Why Vesting?**
- Encourages long-term commitment
- Prevents "pump and dump" behavior
- Aligns director incentives with network success

**Example (Annual)**:
```
Total Compensation: 222,204 ÉTR

Month 1-12: 111,102 ÉTR paid immediately (50%)
Month 7-12: 55,551 ÉTR vested (25%)
Month 13-24: 55,551 ÉTR vested (25%)

Total Vested by Month 24: 222,204 ÉTR
```

#### Performance Incentives

**Bonus Pool**: 10% of director allocation (547.9 ÉTR/day)

**Performance Metrics**:
1. **Attendance**: Full compensation requires 90%+ meeting attendance
2. **Deliverables**: Complete assigned tasks on time
3. **Community Satisfaction**: Quarterly community rating (1-5 stars)

**Bonus Calculation**:
```
Bonus = Base Bonus × (Attendance % / 90%) × (Community Rating / 5.0)

Example:
- Base Bonus: 54.79 ÉTR/day
- Attendance: 95% (100% bonus multiplier)
- Community Rating: 4.5/5.0 (90% multiplier)

Actual Bonus = 54.79 × 1.0 × 0.9 = 49.31 ÉTR/day
```

#### Penalties

**Missed Meetings**:
- Each missed meeting: -2% daily compensation
- More than 10% missed meetings: Removal vote triggered

**Poor Performance**:
- Community rating < 3.0 stars for 2 consecutive quarters: Removal vote

**Conflicts of Interest**:
- Undisclosed conflict: -50% compensation + potential removal
- Bribery/corruption: Immediate removal + legal action

---

# PART III: ECONOMIC SYSTEMS

## 3. Consensus Day Voting

### 3.1 Annual Schedule

**Date**: Third Friday of November (each year)

**2025-2030 Schedule**:
- **2025**: November 21
- **2026**: November 20
- **2027**: November 19
- **2028**: November 17
- **2029**: November 16
- **2030**: November 15

**Voting Window**: 24 hours (00:00 UTC to 23:59 UTC)

**Why Third Friday of November?**
- Avoids major holidays (Thanksgiving in late November, Christmas in December)
- Consistent annual date (easy to remember)
- Aligns with fiscal year planning (January 1st start)

### 3.2 Voting Topics & Proposals

#### Mandatory Annual Votes

**1. Annual Mint Rate (Inflation)**
- Current: 10M ÉTR/year (27,397 ÉTR/day)
- Proposed: Decrease by 10% annually until reaching 2% minimum inflation
- Community votes on next year's mint rate

**2. Distribution Allocation Percentages**
- Current: 40% Foundation, 30% Validators, 20% Directors, 10% Voters
- Proposed changes must maintain 100% total
- Requires 7-of-9 Director approval + 60% community approval

**3. Foundation Budget**
- Annual budget for development, marketing, legal, operations
- Itemized breakdown required
- Community can veto individual line items (60% threshold)

**4. Protocol Upgrade Proposals**
- Major protocol changes (consensus, economic model, governance)
- Technical review required before vote
- Requires 70% approval for implementation

#### Optional Votes (As Needed)

**5. Emergency Proposals**
- Security vulnerabilities
- Economic crises
- Legal threats
- Requires 3-of-9 Directors + 48-hour community vote

**6. Constitutional Amendments**
- Changes to governance structure
- Requires 80% approval (supermajority)
- 30-day discussion period

### 3.3 Participant Categories

#### Tier 1: Decentralized Directors (9 members)
- **Weight**: 9 votes (1 vote each)
- **Influence**: 9% of total voting power
- **Responsibility**: Vote on all proposals, break ties

#### Tier 2: Validators (21 Flare Nodes + 104 Collators = 125 total)
- **Weight**: 125 votes (1 vote each)
- **Influence**: ~12% of total voting power
- **Responsibility**: Technical review, block production

#### Tier 3: Active Stakers (~1,000 estimated)
- **Weight**: Stake-weighted (10,000 ÉTR average stake = 10,000 votes)
- **Influence**: ~78% of total voting power
- **Responsibility**: Long-term alignment, economic security

#### Tier 4: Voters (No stake requirement)
- **Weight**: 1 vote per account (identity-verified)
- **Influence**: Minimal (used for signaling only)
- **Responsibility**: Community sentiment gauge

**Total Voting Power Breakdown**:
```
Directors:   9 votes       (0.9%)
Validators:  125 votes     (12.1%)
Stakers:     10M votes     (87%)
Voters:      Variable      (signaling only)
```

### 3.4 Voting Mechanics & Weight

#### On-Chain Voting Implementation

```rust
/// Consensus Day Vote
pub struct ConsensusVote<T: Config> {
    /// Vote ID
    pub id: H256,

    /// Proposal title
    pub title: BoundedVec<u8, ConstU32<256>>,

    /// Proposal description
    pub description: BoundedVec<u8, ConstU32<4096>>,

    /// Voting start block
    pub start_block: BlockNumber,

    /// Voting end block
    pub end_block: BlockNumber,

    /// Vote options (e.g., ["Yes", "No", "Abstain"])
    pub options: BoundedVec<BoundedVec<u8, ConstU32<64>>, ConstU32<10>>,

    /// Vote tallies (option_id => vote_weight)
    pub tallies: BTreeMap<u8, u128>,

    /// Status
    pub status: VoteStatus,
}

impl<T: Config> Pallet<T> {
    /// Cast vote with stake-weighted power
    pub fn cast_vote(
        origin: OriginFor<T>,
        vote_id: H256,
        option_id: u8,
    ) -> DispatchResult {
        let voter = ensure_signed(origin)?;

        // Get vote
        let mut vote = Votes::<T>::get(vote_id)
            .ok_or(Error::<T>::VoteNotFound)?;

        // Check voting window
        let current_block = frame_system::Pallet::<T>::block_number();
        ensure!(
            current_block >= vote.start_block && current_block <= vote.end_block,
            Error::<T>::VotingClosed
        );

        // Calculate voting power
        let stake = T::Staking::total_stake(&voter);
        let coinage_multiplier = Self::calculate_coinage_multiplier(&voter);
        let voting_power = stake * coinage_multiplier as u128 / 10000;

        // Record vote
        vote.tallies.entry(option_id)
            .and_modify(|v| *v += voting_power)
            .or_insert(voting_power);

        // Emit event
        Self::deposit_event(Event::VoteCast {
            vote_id,
            voter,
            option_id,
            voting_power,
        });

        Ok(())
    }
}
```

#### Quorum Requirements

**Standard Proposals**: 20% of total staked ÉTR must participate

**Critical Proposals** (protocol upgrades, constitutional changes): 40% quorum

**Emergency Proposals**: 10% quorum (fast response needed)

**Calculation**:
```
Total Staked: 50M ÉTR (hypothetical)
Standard Quorum (20%): 10M ÉTR must vote
Critical Quorum (40%): 20M ÉTR must vote
Emergency Quorum (10%): 5M ÉTR must vote
```

---

## 4. Distribution Pay Economics

### 4.1 Daily Distribution Schedule

**Total Daily**: 27,397 ÉTR (10M ÉTR/year ÷ 365 days)

**Distribution Times (UTC)**:

| Time | Category | Amount (ÉTR) | Recipients | Purpose |
|------|----------|--------------|------------|---------|
| 00:01 | Voters | 2,740 | ~1,000 | Consensus Day participation |
| 04:01 | Flare Nodes | 4,110 | 21 | Relay chain validation |
| 06:01 | Validity Nodes | 4,109 | 104 | PBC collation |
| 08:01 | Stakers | 10,959 | Variable | Long-term holders |
| 12:01 | Directors | 5,479 | 9 | Governance leadership |

**Total**: 27,397 ÉTR/day

### 4.2 Allocation Breakdown

#### 40% Foundation Treasury (10,959 ÉTR/day)

**Purpose**: Fund development, security audits, marketing, legal, operations

**Breakdown**:
```
Development (50%):     5,479 ÉTR/day = 2M ÉTR/year
Security (20%):        2,192 ÉTR/day = 800K ÉTR/year
Marketing (15%):       1,644 ÉTR/day = 600K ÉTR/year
Legal (10%):           1,096 ÉTR/day = 400K ÉTR/year
Operations (5%):       548 ÉTR/day = 200K ÉTR/year
```

**Spending Approval**:
- < 1,000 ÉTR: Technical Committee approval (3 members)
- 1,000-10,000 ÉTR: 5-of-9 Directors
- > 10,000 ÉTR: 7-of-9 Directors + 3-day community veto period

#### 30% Validators (8,219 ÉTR/day)

**Flare Nodes (15%)**: 4,110 ÉTR/day ÷ 21 validators = **195.71 ÉTR/day per validator**

**Validity Nodes (15%)**: 4,109 ÉTR/day ÷ 104 collators = **39.51 ÉTR/day per collator**

**Why Different Amounts?**
- Flare Nodes secure entire relay chain (higher responsibility)
- Validity Nodes collate individual PBCs (lower responsibility)
- Ratio: ~5:1 (Flare:Validity)

**Annual Validator Rewards**:
```
Flare Node: 195.71 ÉTR/day × 365 = 71,434 ÉTR/year
Validity Node: 39.51 ÉTR/day × 365 = 14,421 ÉTR/year
```

#### 20% Decentralized Directors (5,479 ÉTR/day)

**Per Director**: 5,479 ÉTR ÷ 9 = **608.78 ÉTR/day** = **222,204 ÉTR/year**

**Vesting**:
- 50% immediate: 304.39 ÉTR/day
- 25% 6-month vest: 152.20 ÉTR/day
- 25% 12-month vest: 152.20 ÉTR/day

#### 10% Voters (2,740 ÉTR/day)

**Eligibility**: Participated in Consensus Day voting

**Distribution Method**: Pro-rata based on voting power used

**Example** (1,000 voters with avg 10,000 ÉTR stake):
```
Total Voting Power: 1,000 voters × 10,000 ÉTR = 10M votes
Your Stake: 10,000 ÉTR
Your Share: 10,000 / 10,000,000 = 0.1%
Your Reward: 2,740 ÉTR × 0.1% = 2.74 ÉTR/day

Annual: 2.74 ÉTR/day × 365 = 1,000 ÉTR/year
```

**Minimum Participation**: Must vote on at least 1 proposal to qualify

### 4.3 Coinage Multiplier System

**Formula**:
```
Reward = Base Reward × Coinage Multiplier

Where Coinage Multiplier depends on stake duration:
- 0-6 months:    1.0x
- 6-12 months:   1.25x
- 12-18 months:  1.5x
- 18-24 months:  1.75x
- 24+ months:    2.0x (maximum)
```

**Implementation**:
```rust
pub fn calculate_coinage_multiplier(stake_duration_days: u64) -> u32 {
    // Returns basis points (10000 = 1.0x)
    match stake_duration_days {
        0..=180 => 10000,         // 1.0x
        181..=365 => 12500,       // 1.25x
        366..=545 => 15000,       // 1.5x
        546..=730 => 17500,       // 1.75x
        _ => 20000,               // 2.0x (730+ days)
    }
}
```

**Examples**:

| Stake Duration | Base Reward | Multiplier | Final Reward |
|----------------|-------------|------------|--------------|
| 3 months (90d) | 100 ÉTR | 1.0x | **100 ÉTR** |
| 9 months (270d) | 100 ÉTR | 1.25x | **125 ÉTR** |
| 15 months (450d) | 100 ÉTR | 1.5x | **150 ÉTR** |
| 21 months (630d) | 100 ÉTR | 1.75x | **175 ÉTR** |
| 30 months (900d) | 100 ÉTR | 2.0x | **200 ÉTR** |

**Why Coinage Multiplier?**
- Rewards long-term commitment
- Reduces circulating supply (more staking = less selling)
- Increases network security
- Discourages short-term speculation

### 4.4 Penalty & Reward Mechanisms

#### Validator Penalties

**Missed Block**:
- Penalty: 0.1% of daily reward per missed block
- Max Penalty: 10% of daily reward (100 missed blocks)
- Recovery: No missed blocks for 24 hours resets counter

**Double-Sign** (Equivocation):
- Penalty: 10% of total stake (slashing)
- Immediate removal from validator set
- Ban duration: 90 days

**Offline** (No blocks produced for 1 hour):
- Penalty: 0.01% per hour offline
- Grace period: 1 hour (system maintenance)
- Max Penalty: 5% of daily reward

**Example**:
```
Flare Node Daily Reward: 195.71 ÉTR

Scenario 1: Missed 10 blocks
Penalty: 195.71 × 0.1% × 10 = 1.96 ÉTR
Final Reward: 195.71 - 1.96 = 193.75 ÉTR

Scenario 2: Offline for 6 hours
Penalty: 195.71 × 0.01% × 6 = 0.12 ÉTR
Final Reward: 195.71 - 0.12 = 195.59 ÉTR

Scenario 3: Double-sign
Penalty: Total stake slashed by 10%
Final Reward: 0 ÉTR + removed from validator set
```

#### Director Penalties

**Missed Meeting**:
- Penalty: 2% of daily compensation
- 3 consecutive missed meetings: Warning issued
- 5 missed meetings in a quarter: Removal vote triggered

**Low Community Rating**:
- Rating < 3.0 stars for 1 quarter: -10% compensation
- Rating < 3.0 stars for 2 consecutive quarters: Removal vote
- Rating > 4.5 stars: +5% bonus

**Conflict of Interest**:
- Undisclosed conflict: -50% compensation + formal reprimand
- Material conflict (financial gain): Immediate removal
- Bribery/corruption: Removal + legal action + blacklist

---

## 5. Treasury Management

### 5.1 Foundation Treasury

**Daily Allocation**: 10,959 ÉTR (40% of 27,397 ÉTR)
**Annual Budget**: 4M ÉTR/year

**Multi-Signature Wallet**:
- **Address**: `5FKgJ...` (Substrate multi-sig)
- **Signers**: 9 Decentralized Directors
- **Threshold**: 5-of-9 (standard), 7-of-9 (large expenditures)

**Reserve Breakdown**:
```
Operating Reserve (20%):   800K ÉTR  (3 months operating costs)
Security Reserve (10%):    400K ÉTR  (bug bounties, audits)
Strategic Reserve (10%):   400K ÉTR  (partnerships, acquisitions)
Available (60%):          2.4M ÉTR  (annual budget)
```

### 5.2 Budget Approval Process

#### Quarterly Budget Cycle

**Q1 (Jan-Mar)**:
- Dec 15: Budget proposals due
- Dec 20: Technical Committee review
- Jan 1: Quarterly budget approved

**Q2 (Apr-Jun)**:
- Mar 15: Budget proposals due
- Mar 20: Technical Committee review
- Apr 1: Quarterly budget approved

**Q3 (Jul-Sep)**:
- Jun 15: Budget proposals due
- Jun 20: Technical Committee review
- Jul 1: Quarterly budget approved

**Q4 (Oct-Dec)**:
- Sep 15: Budget proposals due
- Sep 20: Technical Committee review
- Oct 1: Quarterly budget approved

#### Approval Tiers

**Tier 1: Micro (< 1,000 ÉTR)**
- Approver: Any 3 Technical Committee members
- Timeframe: 24 hours
- Examples: Bug fixes, minor improvements, documentation

**Tier 2: Small (1,000-10,000 ÉTR)**
- Approver: 5-of-9 Directors
- Timeframe: 48 hours
- Examples: Security audit, marketing campaign, conference sponsorship

**Tier 3: Medium (10,000-100,000 ÉTR)**
- Approver: 7-of-9 Directors
- Timeframe: 7 days (with 3-day community review)
- Examples: Major feature development, legal retainer, exchange listing

**Tier 4: Large (> 100,000 ÉTR)**
- Approver: 7-of-9 Directors + Community vote (60% approval)
- Timeframe: 30 days (with public RFC)
- Examples: Strategic acquisition, major partnership, protocol rewrite

### 5.3 Transparency & Reporting

#### Real-Time Dashboard

**URL**: https://treasury.etrid.network

**Metrics Displayed**:
- Current treasury balance (ÉTR + USD value)
- Daily inflow (distribution allocation)
- Daily outflow (approved expenditures)
- Reserve breakdown
- Historical spending (1 week, 1 month, 1 year)

**Example Dashboard**:
```
Current Balance: 1,245,678 ÉTR ($2,491,356 @ $2.00/ÉTR)

Daily Inflow:   +10,959 ÉTR
Daily Outflow:  -8,234 ÉTR
Net Daily:      +2,725 ÉTR

Reserves:
  Operating:    800,000 ÉTR (64% funded)
  Security:     400,000 ÉTR (100% funded)
  Strategic:    45,678 ÉTR (11% funded)

Recent Transactions:
  [Dec 1] Security Audit: -50,000 ÉTR
  [Nov 28] Developer Grants: -25,000 ÉTR
  [Nov 25] Exchange Listing: -100,000 ÉTR
```

#### Quarterly Reports

**Contents**:
1. **Financial Summary**
   - Total revenue (distribution allocation)
   - Total expenses (itemized)
   - Net change in treasury
   - Reserve status

2. **Expenditure Breakdown**
   - Development (projects, features, bug fixes)
   - Security (audits, bug bounties, penetration testing)
   - Marketing (campaigns, conferences, partnerships)
   - Legal (compliance, IP, general counsel)
   - Operations (hosting, tooling, admin)

3. **Key Performance Indicators**
   - Active developers (commits/month)
   - Community growth (users, stakers)
   - Network metrics (TPS, TVL, validator count)

4. **Forward-Looking Statements**
   - Planned expenditures for next quarter
   - Strategic initiatives
   - Risk factors

**Publication**:
- Published within 15 days of quarter end
- Posted on governance portal + social media
- Community AMA hosted by Treasury Committee

---

## 6. Protocol Upgrades

### 6.1 Upgrade Proposal Process

#### Phase 1: RFC (Request for Comments)

**Duration**: 30 days minimum

**Submission Requirements**:
```
1. Technical specification (detailed design document)
2. Implementation plan (timeline, milestones)
3. Security analysis (risks, mitigations)
4. Community impact assessment
5. Backwards compatibility analysis
```

**RFC Template**:
```markdown
# RFC-XXX: [Title]

## Summary
One-paragraph explanation of the proposal.

## Motivation
Why is this change needed?

## Technical Specification
Detailed technical design.

## Implementation Plan
- Milestone 1: [Description] (ETA)
- Milestone 2: [Description] (ETA)
- ...

## Security Considerations
Risks and mitigations.

## Community Impact
How does this affect users, developers, validators?

## Backwards Compatibility
Is this a breaking change?

## Open Questions
Unresolved issues for community discussion.
```

**Community Discussion**:
- Post on governance forum
- Discord/Telegram discussions
- Weekly community calls
- Technical review sessions

#### Phase 2: Technical Review

**Duration**: 14 days

**Reviewers**:
- Technical Committee (3 Directors)
- External auditors (if budget allows)
- Core developers
- Community technical experts

**Review Criteria**:
- ✅ Technical soundness
- ✅ Security robustness
- ✅ Performance impact
- ✅ Backwards compatibility
- ✅ Test coverage (minimum 80%)

**Possible Outcomes**:
1. **Approved**: Move to Phase 3 (voting)
2. **Approved with Changes**: Revise and resubmit
3. **Rejected**: Proposal withdrawn (can be resubmitted after addressing concerns)

#### Phase 3: Governance Vote

**Duration**: 7 days

**Voting Method**: On-chain governance vote

**Approval Threshold**:
- **Standard Upgrades** (features, optimizations): 60% approval
- **Critical Upgrades** (consensus changes, economic model): 70% approval
- **Constitutional Upgrades** (governance changes): 80% approval (supermajority)

**Quorum**:
- Standard: 20% of staked ÉTR
- Critical: 40% of staked ÉTR
- Constitutional: 60% of staked ÉTR

**Example Vote**:
```
Proposal: RFC-042 - Add AIDID Reputation Slashing
Type: Standard Upgrade
Approval Threshold: 60%
Quorum: 20%

Results:
  Yes: 12M ÉTR (65%)
  No: 5M ÉTR (27%)
  Abstain: 1.5M ÉTR (8%)

Total Participation: 18.5M ÉTR (37% of 50M total staked)
Outcome: APPROVED (65% > 60% threshold, 37% > 20% quorum)
```

#### Phase 4: Implementation

**Code Review**:
- Pull request submitted to GitHub
- Minimum 2 core developer approvals
- CI/CD pipeline (build, test, lint) must pass

**Testnet Deployment**:
- Deploy to testnet first
- Run for minimum 14 days
- Monitor for bugs, performance issues
- Community testing period

**Mainnet Deployment**:
- Coordinate with validators (announce upgrade date)
- Provide migration guide for affected users
- Deploy via on-chain runtime upgrade (Substrate)
- Monitor for 48 hours post-upgrade

### 6.2 Runtime Upgrade Mechanics

#### Substrate Runtime Upgrades

**How It Works**:
```
1. Compile new runtime (WASM blob)
2. Submit via governance extrinsic:
   system.setCode(new_runtime_blob)
3. 7-of-9 Directors approve (multi-sig)
4. Runtime upgrade enacted on next block
5. All nodes automatically use new runtime (no hard fork!)
```

**Example**:
```rust
// Submit runtime upgrade
pub fn propose_runtime_upgrade(
    origin: OriginFor<T>,
    new_runtime: Vec<u8>,
) -> DispatchResult {
    // Verify origin (must be Technical Committee or Directors)
    T::EnsureRootOrTwoThirds::ensure_origin(origin)?;

    // Validate runtime
    ensure!(new_runtime.len() < MAX_RUNTIME_SIZE, Error::<T>::RuntimeTooLarge);

    // Schedule upgrade
    frame_system::Pallet::<T>::set_code_without_checks(
        RawOrigin::Root.into(),
        new_runtime,
    )?;

    Ok(())
}
```

**Benefits**:
- No hard fork required
- No node downtime
- Seamless upgrades
- Coordinated activation

### 6.3 Emergency Upgrades

**Trigger Conditions**:
- Critical security vulnerability discovered
- Network halt or significant performance degradation
- Exploits actively being used

**Fast-Track Process**:
```
1. Security Committee identifies critical issue (within 1 hour)
2. Emergency patch developed (within 24 hours)
3. 3-of-9 Directors approve (within 4 hours)
4. Deploy to testnet (within 2 hours)
5. If stable, deploy to mainnet immediately
```

**Post-Mortem**:
- Publish detailed incident report within 7 days
- Community AMA to address concerns
- Retroactive community vote to ratify emergency upgrade (60% approval)
- If community rejects (< 60%), revert upgrade and find alternative solution

---

## 7. Emergency Procedures

### 7.1 Emergency Fund Access

**Purpose**: Respond to critical security incidents, legal threats, or disasters

**Fund Size**: 10% of treasury reserves (400K ÉTR)

**Access Triggers**:
1. **Security Breach**: Active exploit of critical vulnerability
2. **Legal Threat**: Lawsuit, regulatory action requiring immediate legal response
3. **Natural Disaster**: Affects core team or infrastructure
4. **Economic Crisis**: Extreme market volatility threatening network stability

**Approval Process**:

**Tier 1: Immediate (< 10,000 ÉTR)**
- Approver: Any 3 Directors (emergency multi-sig)
- Timeframe: 1 hour
- No community vote required (but post-incident report mandatory)

**Tier 2: Urgent (10,000-50,000 ÉTR)**
- Approver: 5-of-9 Directors
- Timeframe: 6 hours
- Community notification required (within 24 hours)

**Tier 3: Critical (> 50,000 ÉTR)**
- Approver: 7-of-9 Directors
- Timeframe: 24 hours
- Community vote retroactive (within 7 days, 60% approval to ratify)

### 7.2 Circuit Breaker Activation

#### Purpose
Temporarily halt certain network functions to prevent catastrophic losses.

#### Trigger Conditions

**Bridge Circuit Breaker**:
- Daily withdrawal limit exceeded (e.g., > $10M/day for BTC bridge)
- Suspicious large withdrawals (e.g., 100 withdrawals of exactly 1 BTC within 1 hour)
- Oracle price manipulation detected (> 20% deviation from baseline)
- Custodian private key compromise suspected

**Trading Circuit Breaker** (DEX/AMM):
- Flash crash (> 30% price drop in 5 minutes)
- Unusually high volume (> 10x daily average within 1 hour)
- Oracle manipulation detected

**Governance Circuit Breaker**:
- Suspicious voting patterns (e.g., single entity controlling > 40% of votes)
- Last-minute vote swings (> 20% change in final hour)
- Potential bribery detected

#### Activation Process

**Automatic Triggers** (Smart contract enforced):
```rust
impl CircuitBreaker {
    pub fn check_and_trigger(&mut self, current_metrics: &Metrics) -> Result<(), BreakerError> {
        // Check daily withdrawal limit
        if current_metrics.daily_withdrawn > self.daily_limit {
            self.trigger("Daily withdrawal limit exceeded");
            return Err(BreakerError::LimitExceeded);
        }

        // Check oracle deviation
        if current_metrics.price_deviation > Permill::from_percent(20) {
            self.trigger("Oracle price deviation detected");
            return Err(BreakerError::PriceManipulation);
        }

        Ok(())
    }

    fn trigger(&mut self, reason: &str) {
        self.paused = true;
        self.reason = Some(reason.as_bytes().to_vec());
        self.paused_at = Some(current_timestamp());

        // Emit emergency event
        emit_event(Event::CircuitBreakerTriggered {
            reason: reason.into(),
            timestamp: current_timestamp(),
        });

        // Notify Directors (off-chain)
        notify_directors_emergency(reason);
    }
}
```

**Manual Triggers** (Governance):
- Any 3 Directors can manually trigger circuit breaker
- Must provide justification (on-chain)
- Community notified immediately

#### Resolution Process

**Phase 1: Investigation (0-6 hours)**
- Security Committee investigates root cause
- Assess damage and determine fix

**Phase 2: Fix Development (6-24 hours)**
- Develop patch or mitigation
- Test on isolated testnet
- Prepare communication to community

**Phase 3: Community Discussion (24-48 hours)**
- Publish incident report
- Explain proposed fix
- Community feedback period

**Phase 4: Resolution (48-72 hours)**
- Deploy fix (if approved by 7-of-9 Directors)
- Resume normal operations
- Post-mortem report published

### 7.3 Security Incident Response

#### Incident Severity Levels

**Critical (P0)**:
- Active exploit draining funds
- Network completely halted
- Private keys compromised
- **Response Time**: < 1 hour

**High (P1)**:
- Vulnerability actively being exploited
- Significant performance degradation
- Major feature broken
- **Response Time**: < 4 hours

**Medium (P2)**:
- Vulnerability reported but not actively exploited
- Minor feature broken
- **Response Time**: < 24 hours

**Low (P3)**:
- Cosmetic bug
- Performance optimization opportunity
- **Response Time**: < 7 days

#### Response Workflow

**Step 1: Detection & Triage**
```
- Security monitoring detects anomaly
- Incident reported to Security Committee
- Severity assigned (P0-P3)
- War room established (for P0/P1)
```

**Step 2: Containment**
```
- Trigger circuit breaker (if applicable)
- Isolate affected components
- Prevent further damage
```

**Step 3: Investigation**
```
- Root cause analysis
- Assess scope of impact
- Determine if user funds at risk
```

**Step 4: Communication**
```
- Notify community (Twitter, Discord, governance forum)
- Provide status updates every 2 hours (P0), 6 hours (P1)
- Be transparent about risks
```

**Step 5: Remediation**
```
- Develop fix
- Test thoroughly
- Deploy via emergency upgrade process
```

**Step 6: Recovery**
```
- Resume normal operations
- Monitor for 48 hours
- Compensate affected users (if applicable)
```

**Step 7: Post-Mortem**
```
- Publish detailed incident report within 7 days
- Community AMA
- Implement preventive measures
```

#### Bug Bounty Program

**Tiers**:

| Severity | Reward | Examples |
|----------|--------|----------|
| **Critical** | 50,000-500,000 ÉTR | Remote code execution, consensus failure, bridge fund drain |
| **High** | 5,000-50,000 ÉTR | Significant security vulnerability, data leak |
| **Medium** | 1,000-5,000 ÉTR | Non-critical security issue |
| **Low** | 100-1,000 ÉTR | Typos, documentation errors |

**Bridge-Specific Bounties**:
- **Critical Bridge Bug**: Up to $1,000,000 (paid in USDC/USDT)
- **Rationale**: Bridges hold significant TVL; critical bugs could drain millions

**Submission Process**:
```
1. Report vulnerability privately to security@etrid.network
2. Include detailed steps to reproduce
3. Wait 48 hours for acknowledgment
4. Security Committee verifies and assigns severity
5. Payment sent within 7 days of verification
```

**Responsible Disclosure**:
- ✅ Report privately first (30-90 day embargo)
- ✅ Allow time for patch development
- ✅ Coordinate public disclosure
- ❌ Do NOT publicly disclose before patch
- ❌ Do NOT exploit vulnerability for personal gain

---

## 8. Voter Participation

### 8.1 Eligibility Requirements

**Minimum Requirements**:
- Own at least 1 ÉTR (no minimum stake required for basic voting)
- Identity verification (for Sybil resistance)
- Active account (at least 1 transaction in past 365 days)

**Identity Verification Methods**:
1. **On-Chain** (recommended):
   - Link Ethereum address with ENS name
   - Link Polkadot address with on-chain identity
   - Use decentralized identity protocols (did:etrd, did:web)

2. **Off-Chain** (optional):
   - KYC via third-party service (for enhanced voting weight)
   - Social verification (Twitter, GitHub, LinkedIn)

**Sybil Resistance**:
- Each identity can create multiple accounts, but voting power is combined
- Prevents vote splitting attack (creating 1,000 accounts with 1 ÉTR each)

### 8.2 Stake-Weighted Voting

**Formula**:
```
Voting Power = Base Stake × Coinage Multiplier × Participation Bonus

Where:
- Base Stake = Amount of ÉTR staked
- Coinage Multiplier = 1.0x to 2.0x (based on stake duration)
- Participation Bonus = 1.0x to 1.2x (based on voting history)
```

**Example**:
```
Alice:
  Base Stake: 10,000 ÉTR
  Stake Duration: 25 months (2.0x multiplier)
  Participation: Voted in 3 previous Consensus Days (1.2x bonus)

Voting Power = 10,000 × 2.0 × 1.2 = 24,000 votes

Bob:
  Base Stake: 20,000 ÉTR
  Stake Duration: 2 months (1.0x multiplier)
  Participation: First-time voter (1.0x bonus)

Voting Power = 20,000 × 1.0 × 1.0 = 20,000 votes

Result: Alice has more voting power despite lower stake
```

**Why Stake-Weighted?**
- Aligns voting power with economic stake
- Encourages long-term commitment
- Reduces influence of short-term speculators
- Rewards active community participation

### 8.3 Voter Rewards

**Daily Allocation**: 2,740 ÉTR (10% of total daily distribution)

**Distribution Method**: Pro-rata based on votes cast

**Calculation**:
```
Your Reward = (Your Votes / Total Votes) × 2,740 ÉTR

Example:
  Total Votes Cast: 10,000,000
  Your Votes: 10,000
  Your Share: 10,000 / 10,000,000 = 0.1%
  Your Reward: 0.1% × 2,740 ÉTR = 2.74 ÉTR

Annualized: 2.74 ÉTR/day × 365 = 1,000 ÉTR/year
```

**Eligibility**:
- Must vote on at least 1 proposal during Consensus Day
- Rewards distributed proportionally to all voters
- If you vote on more proposals, you don't get extra rewards (rewards are per-account, not per-vote)

**Distribution Timeline**:
- Rewards calculated immediately after Consensus Day (Nov 15)
- Distribution begins Nov 16 at 00:01 UTC
- Daily distribution continues for 365 days until next Consensus Day

### 8.4 Delegation Mechanisms

**Purpose**: Allow stakers to delegate voting power to trusted experts without unstaking

**How It Works**:
```rust
pub fn delegate_votes(
    origin: OriginFor<T>,
    delegate: T::AccountId,
) -> DispatchResult {
    let delegator = ensure_signed(origin)?;

    // Record delegation
    VoteDelegation::<T>::insert(&delegator, &delegate);

    // Delegator keeps custody of tokens (no transfer)
    // Delegate can vote on behalf of delegator

    Ok(())
}
```

**Benefits**:
- **For Delegators**: Participate in governance without deep technical knowledge
- **For Delegates**: Build reputation as trusted community leader
- **For Network**: Higher participation rates, better-informed decisions

**Example**:
```
Alice has 100,000 ÉTR but doesn't have time to evaluate proposals.
Bob is a blockchain expert and active community member.

Alice delegates voting power to Bob.
Bob can now vote with:
  - His own 10,000 ÉTR
  - Alice's delegated 100,000 ÉTR
  - Total: 110,000 voting power

Alice can revoke delegation at any time.
```

**Delegation Limits**:
- Can only delegate to 1 person at a time
- Can revoke delegation at any time (takes effect immediately)
- Delegate cannot transfer delegated voting power
- Delegation does not transfer custody of tokens

---

## 9. Community Development

### 9.1 Bug Bounty Program

**Already Covered in Section 7.3**

See: [Section 7.3: Security Incident Response - Bug Bounty Program](#73-security-incident-response)

**Quick Summary**:
- Critical: 50K-500K ÉTR
- High: 5K-50K ÉTR
- Medium: 1K-5K ÉTR
- Low: 100-1K ÉTR
- Bridge Critical: Up to $1M USD

### 9.2 Grant Program

**Purpose**: Fund community projects that benefit the Ëtrid ecosystem

**Grant Tiers**:

**Tier 1: Micro Grants (< 5,000 ÉTR)**
- Approver: Community Committee (2 Directors)
- Timeframe: 7 days
- Examples: Documentation, tutorials, community events

**Tier 2: Small Grants (5,000-25,000 ÉTR)**
- Approver: 5-of-9 Directors
- Timeframe: 14 days
- Examples: Developer tools, integrations, wallets

**Tier 3: Large Grants (25,000-100,000 ÉTR)**
- Approver: 7-of-9 Directors + Community vote (60% approval)
- Timeframe: 30 days
- Examples: DeFi protocols, NFT marketplaces, major infrastructure

**Application Process**:
```
1. Submit grant proposal on governance forum
2. Include:
   - Project description
   - Team background
   - Milestones & deliverables
   - Budget breakdown
   - Timeline

3. Community discussion (7-30 days depending on tier)
4. Approval vote
5. If approved, funds released according to milestone schedule
```

**Milestone-Based Payment**:
```
Example: 30,000 ÉTR grant for DeFi protocol

Milestone 1 (25%): Smart contract development complete
  Payment: 7,500 ÉTR

Milestone 2 (25%): Security audit passed
  Payment: 7,500 ÉTR

Milestone 3 (25%): Testnet deployment & testing
  Payment: 7,500 ÉTR

Milestone 4 (25%): Mainnet launch
  Payment: 7,500 ÉTR
```

**Grant Categories**:
- **Infrastructure**: Nodes, indexers, explorers, APIs
- **Developer Tools**: SDKs, libraries, frameworks, IDEs
- **DeFi**: DEX, lending, stablecoin, derivatives
- **NFTs**: Marketplaces, standards, tooling
- **Education**: Tutorials, courses, documentation
- **Community**: Events, meetups, ambassadors
- **Research**: Academic research, whitepapers, formal verification

### 9.3 Developer Incentives

#### Retroactive Public Goods Funding

**Purpose**: Reward developers who build valuable public goods without upfront funding

**How It Works**:
```
1. Developer builds open-source tool/protocol
2. Community uses it and finds it valuable
3. Quarterly, community votes on which projects deserve retroactive funding
4. Top 10 projects receive funding from treasury
```

**Funding Pool**: 5% of quarterly treasury budget (~50K ÉTR/quarter)

**Example**:
```
Q1 2025 Retroactive Funding:

1. Ëtrid Block Explorer: 15,000 ÉTR (30%)
2. ËDSC Stablecoin Integration: 10,000 ÉTR (20%)
3. Lightning-Bloc Mobile Wallet: 7,500 ÉTR (15%)
4. AIDID Developer SDK: 5,000 ÉTR (10%)
5. Cross-Chain Bridge UI: 5,000 ÉTR (10%)
6-10. Various smaller projects: 7,500 ÉTR total (15%)

Total: 50,000 ÉTR distributed
```

#### Core Developer Compensation

**Full-Time Developers**:
- Salary: Market rate (paid in ÉTR or stablecoins)
- Benefits: Health insurance, equipment allowance
- Bonus: Annual performance bonus (10-20% of salary)

**Part-Time Contributors**:
- Hourly rate: Based on expertise and contribution
- Paid per merged PR (quality-weighted)

**Hiring Process**:
- Open positions posted on jobs.etrid.network
- Technical interview + community interview
- Approval by Technical Committee (3 Directors)
- Budget approval by 5-of-9 Directors

### 9.4 Education & Outreach

#### Ëtrid Academy

**Purpose**: Educational platform for developers and users

**Content**:
- **Beginner**: What is blockchain? How to use Ëtrid wallet?
- **Intermediate**: Build your first smart contract, deploy to testnet
- **Advanced**: Develop cross-chain DeFi protocol, optimize gas costs

**Incentives**:
- Complete course: Earn 10-100 ÉTR (based on difficulty)
- Build project: Eligible for micro grants
- Contribute course material: 500-5,000 ÉTR bounty

#### Community Ambassadors

**Role**:
- Organize local meetups
- Create educational content (blog posts, videos)
- Provide community support (Discord, Telegram)
- Translate documentation

**Compensation**:
- Monthly stipend: 500-2,000 ÉTR (based on activity level)
- Conference sponsorship: Travel + accommodation
- Swag budget: T-shirts, stickers, promotional materials

**Application**:
- Apply on governance portal
- Community Committee reviews
- Approval by 3-of-5 Community Committee members

---

## 10. Appendix: Governance Parameters

### 10.1 Key Parameters

| Parameter | Value | Governance Control |
|-----------|-------|-------------------|
| **Director Count** | 9 | 80% community vote to change |
| **Director Term** | 1 year | 70% community vote to change |
| **Director Term Limit** | 3 consecutive | 60% community vote to change |
| **Voting Quorum (Standard)** | 20% staked ÉTR | 60% community vote to change |
| **Voting Quorum (Critical)** | 40% staked ÉTR | 70% community vote to change |
| **Approval Threshold (Standard)** | 60% | 60% community vote to change |
| **Approval Threshold (Critical)** | 70% | 70% community vote to change |
| **Approval Threshold (Constitutional)** | 80% | 80% community vote to change |
| **Multi-Sig Threshold** | 5-of-9 (standard) | 7-of-9 Directors to change |
| **Emergency Multi-Sig** | 3-of-9 | 7-of-9 Directors to change |
| **Daily Distribution** | 27,397 ÉTR | Annual Consensus Day vote |
| **Distribution Split** | 40/30/20/10 | 70% community vote to change |
| **Coinage Multiplier Max** | 2.0x | 60% community vote to change |

### 10.2 Historical Decisions

| Date | Decision | Approval | Notes |
|------|----------|----------|-------|
| Nov 15, 2025 | First Consensus Day | 95% | Elected initial 9 directors |
| TBD | ... | ... | ... |

---

**End of Ivory Paper - Volume 3: Governance & Economics**

**Total Pages**: ~100 pages
**Key Topics**:
- Foundation Charter & Legal Structure
- Decentralized Directors (9-member board)
- Consensus Day Voting (Annual)
- Distribution Pay Economics (27,397 ÉTR/day)
- Treasury Management (40% allocation)
- Protocol Upgrades & Emergency Procedures
- Voter Participation & Delegation
- Community Development & Grants

**Related Documents**:
- **Volume 1** (Main Ivory Paper): Complete overview of all systems
- **Volume 2** (Technical Specifications): Implementation details for all features
- **Governance Portal**: https://gov.etrid.network
- **Community Forum**: https://forum.etrid.network

**Next Steps**:
1. Review and provide feedback on governance proposals
2. Participate in Consensus Day voting (November 15, 2025)
3. Apply for grants or contribute to development
4. Join as community ambassador

*For updates and latest governance decisions, visit: https://docs.etrid.network*