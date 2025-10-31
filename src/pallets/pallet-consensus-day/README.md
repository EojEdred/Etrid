# Pallet Consensus Day

Annual governance event implementing Ëtrid's constitutional Consensus Day mechanism as specified in **Ivory Papers Vol III**.

## Overview

Consensus Day is Ëtrid's annual constitutional event where all major governance decisions are made collectively. It occurs every December 1st at 12:00 AM PST and operates through a 4-phase structure spanning 22 hours total.

## Features

### 4-Phase Governance Cycle

1. **Registration Phase (6 hours)**
   - Submit proposals with 10,000 ËTR bond
   - Lock stakes to earn voting power
   - Validators signal participation
   - Nominate directors (requires 128 ËTR)

2. **Voting Phase (12 hours)**
   - Vote Yes/No/Abstain on proposals
   - Time-weighted voting power (Staked ËTR × Coinage)
   - Vote for 9 director positions
   - Dual quorum requirement (33% community + 51% validators)

3. **Minting Phase (3 hours)**
   - Execute approved budgets
   - Mint new ËTR (0-5% inflation cap enforced)
   - Apply voted inflation rate
   - Fund treasury with allocated budgets

4. **Distribution Phase (1 hour)**
   - Distribute participation rewards (1% of minted tokens)
   - Completeness bonus (20% extra for voting on all proposals)
   - Increment participation history for future bonuses
   - Elect top 9 directors by vote count

## Proposal System

### Proposal Categories

- **InflationRate**: Set annual inflation (0-5% hard cap)
- **ParameterChange**: Adjust protocol parameters
- **BudgetAllocation**: Allocate funds for development/grants/operations
- **ProtocolUpgrade**: Runtime upgrades and pallet additions
- **DirectorElection**: Elect 9 Decentralized Directors
- **EmergencyAction**: Emergency protocol changes

### Proposal Requirements

- **Bond**: 10,000 ËTR (locked during voting)
- **Bond Refund**: Full refund if quorum reached
- **Bond Slash**: 50% slashed if quorum not met

### Approval Thresholds

- **Simple Majority** (>50%): Budget allocations, parameter changes, inflation rate
- **Supermajority** (>66%): Protocol upgrades, emergency actions

## Voting Power Calculation

```rust
voting_power = base_stake × duration_multiplier × history_multiplier

// Duration bonus: max +20% for long-term stakes
duration_multiplier = 1.0 + min(stake_duration / blocks_per_year, 0.2)

// History bonus: max +10% for consistent participation
history_multiplier = 1.0 + min(participation_history × 0.02, 0.1)
```

### Example

```
Base stake: 1,000 ËTR
Staked for 6 months: +10% duration bonus
Participated in 3 previous Consensus Days: +6% history bonus

Voting power = 1,000 × 1.10 × 1.06 = 1,166
```

## Quorum Requirements

**Dual Quorum System** ensures both community and validators participate:

- **Community Quorum**: 33% of circulating ËTR must vote
- **Validator Quorum**: 51% of active validators must vote
- **Both required**: Proposals need BOTH quorums to be considered

## Director Elections

### Requirements

- Minimum stake: **128 ËTR**
- Must serve as OD Flare Nodes (Operational Director Flare Nodes)
- Active participation in network operations

### Election Process

1. Candidates nominate during Registration phase
2. Voting occurs during Voting phase
3. Top 9 vote-getters are elected
4. Elected directors serve 1-year terms

### Term Limits

- **One-year terms** (renewable)
- **One-year cooldown** between terms
- **Maximum 3 lifetime terms**

## Participation Rewards

### Distribution Formula

```rust
participation_pool = total_minted × 1% // 1% of minted tokens

voter_share = (voter_voting_power / total_voting_power) × participation_pool

// Completeness bonus
if voted_on_all_proposals {
    final_reward = voter_share × 1.2 // 20% bonus
} else {
    final_reward = voter_share
}
```

### Additional Rewards

- **Validator Bonus**: 0.5% of minted tokens divided among validators
- **Director Stipends**: 0.2% of minted tokens divided among 9 directors
- **Proposer Rewards**: 100 ËTR per approved proposal

## Storage Items

### Core State

- `ConsensusDayState`: Current phase and timing information
- `Proposals`: All submitted proposals indexed by ID
- `NextProposalId`: Counter for proposal IDs
- `TotalMinted`: Total tokens minted during current Consensus Day

### Voting Data

- `Votes`: Vote records (voter × proposal_id → vote)
- `VotingPowerMap`: Calculated voting power per account
- `ProposalsVoted`: Number of proposals voted on (for completeness bonus)
- `Validators`: Registered validators for quorum checking

### Director Elections

- `DirectorCandidates`: Candidates for director positions
- `DirectorVotes`: Director election votes
- `ElectedDirectors`: Currently elected 9 directors

### Rewards

- `ParticipationRewards`: Pending participation rewards to claim
- `InflationRate`: Current inflation rate (basis points)
- `VotedInflationRate`: Newly voted inflation rate (applied next year)

### Governance Parameters

- `CirculatingSupply`: Total circulating ËTR (for quorum calculation)
- `ActiveValidatorCount`: Number of active validators

## Extrinsics

### Governance

- `start_consensus_day()` - Start annual event (root only)
- `advance_phase()` - Move to next phase when duration elapsed (anyone can call)

### Proposals

- `submit_proposal(title, category, budget_request)` - Submit proposal with 10k ËTR bond
- `vote(proposal_id, vote_type)` - Cast Yes/No/Abstain vote

### Voting Power

- `lock_stake_for_voting(amount)` - Lock ËTR to gain voting power

### Directors

- `nominate_director()` - Nominate self for director election (128 ËTR required)
- `vote_director(candidate)` - Vote for director candidate

### Rewards

- `claim_participation_reward()` - Claim voter rewards

### System (Root Only)

- `register_validator(validator)` - Register validator for quorum tracking
- `update_circulating_supply(supply)` - Update circulating supply for quorum

## Events

- `ConsensusDayStarted(year, start_block)`
- `PhaseAdvanced(old_phase, new_phase, block)`
- `ProposalSubmitted(id, proposer, category, bond)`
- `VoteCast(voter, proposal_id, vote_type, voting_power)`
- `ProposalApproved(id, yes_votes, no_votes)`
- `ProposalRejected(id, yes_votes, no_votes)`
- `BudgetMinted(proposal_id, amount)`
- `InflationRateSet(old_rate, new_rate)`
- `ParticipationRewardCalculated(account, amount)`
- `ParticipationRewardClaimed(account, amount)`
- `DirectorNominated(candidate, stake)`
- `DirectorVoteCast(voter, candidate, voting_power)`
- `DirectorsElected(directors)`
- `ConsensusDayCompleted(year, total_minted)`

## Configuration Parameters

```rust
// Phase durations (in blocks, assuming 1 second/block)
RegistrationDuration: 21,600    // 6 hours
VotingDuration: 43,200          // 12 hours
MintingDuration: 10,800         // 3 hours
DistributionDuration: 3,600     // 1 hour

// Economic parameters
ProposalBond: 10,000 ËTR        // Proposal submission bond
DirectorMinStake: 128 ËTR       // Director nomination stake
MaxInflationBps: 500            // 5% maximum inflation

// Limits
MaxProposals: 100               // Maximum proposals per Consensus Day
MaxTitleLength: 100             // Maximum proposal title length
```

## Usage Example

### Complete Consensus Day Flow

```rust
// 1. Start Consensus Day (governance)
start_consensus_day()

// 2. Registration Phase (6 hours)
lock_stake_for_voting(1000 * UNITS) // Lock 1000 ËTR
submit_proposal(b"Increase validator rewards".to_vec(), ProposalCategory::ParameterChange, 0)
nominate_director() // If you have 128+ ËTR

// 3. Voting Phase (12 hours)
vote(proposal_id: 0, VoteType::Yes)
vote(proposal_id: 1, VoteType::No)
vote(proposal_id: 2, VoteType::Abstain)
vote_director(director_candidate_account)

// Advance to Minting phase
advance_phase()

// 4. Minting Phase (3 hours)
// Automatic: Approved proposals executed, tokens minted

// Advance to Distribution phase
advance_phase()

// 5. Distribution Phase (1 hour)
// Automatic: Rewards calculated

// Claim your rewards
claim_participation_reward()

// Advance to Inactive (Consensus Day complete)
advance_phase()
```

## Integration with Other Pallets

### Required Integrations

- **pallet-validator-rewards**: Validator registration and tracking
- **pallet-treasury**: Budget disbursement for approved proposals
- **pallet-balances**: Currency operations (reserve, unreserve, transfer)

### Optional Integrations

- **pallet-democracy**: Alternative proposal mechanisms
- **pallet-collective**: Director multisig operations

## Security Considerations

### Bond Mechanism

- 10,000 ËTR bond prevents spam proposals
- 50% slash if quorum not met incentivizes quality proposals
- Full refund if quorum reached encourages participation

### Quorum Requirements

- Dual quorum (33% community + 51% validators) ensures broad participation
- Prevents minority control of governance
- Validators must actively participate

### Inflation Cap

- Hard-coded 5% maximum inflation enforced by protocol
- Cannot be bypassed even by governance
- Protects token holders from hyperinflation

### Time-Weighted Voting

- Coinage bonus rewards long-term holders
- History bonus rewards consistent participants
- Prevents last-minute stake manipulation

## Technical Notes

### Phase Transitions

Phases advance automatically when duration elapses:
- Anyone can call `advance_phase()` to trigger transition
- Phase-specific logic executes during transition
- State is updated atomically

### Voting Power Persistence

- Voting power calculated once during Registration
- Power remains constant throughout Voting phase
- Participation history increments after Distribution

### Director Elections

- Top 9 candidates by vote count elected
- Ties resolved by stake amount
- Elected directors serve 1-year terms

### Reward Distribution

- Participation pool = 1% of total minted
- Proportional to voting power used
- 20% bonus for voting on all proposals
- Rewards claimable during Distribution phase and after

## Future Enhancements

- [ ] Conviction voting for long-term lock-ups
- [ ] Quadratic voting for contentious proposals
- [ ] Delegation mechanisms for voting power
- [ ] Proposal categorization and filtering
- [ ] Off-chain proposal discussion integration
- [ ] Multi-option voting (beyond Yes/No/Abstain)
- [ ] Ranked-choice voting for director elections

## References

- **Ëtrid Ivory Papers Vol III**: Governance & Fiscal Mechanics
- **Consensus Day Specification**: Section 1 (Lines 25-238)
- **Proposal System**: Section 5 (Lines 706-793)
- **Voting Power**: Lines 92-111
- **Reward Distribution**: Lines 196-225

## License

GPL-3.0

## Author

Eoj Edred - Founder, Ëtrid FOODOS Project
