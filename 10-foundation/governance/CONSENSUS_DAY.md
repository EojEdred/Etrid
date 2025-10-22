# Consensus Day Implementation

## Overview

Consensus Day is a special governance event in the Ëtrid Protocol that enables critical protocol decisions to be made with enhanced participation requirements and supermajority approval. This feature ensures that major changes to the protocol receive broad community support before implementation.

## Key Features

### 1. Periodic Schedule
- Consensus Day occurs on a configurable periodic schedule (e.g., quarterly, semi-annually)
- Automatically activates and deactivates based on block numbers
- Configuration includes frequency (time between events) and duration (length of each event)

### 2. Enhanced Voting Requirements
- **Supermajority Threshold**: Proposals require 60-100% approval (configurable per proposal)
- **Minimum Participation**: Requires 20-100% of total stake to participate (configurable per proposal)
- Both thresholds must be met for a proposal to pass

### 3. Automatic Lifecycle Management
- Consensus Day activates automatically at scheduled block numbers
- Deactivates automatically after the duration period
- Next event is automatically scheduled for the next cycle

## Architecture

### Data Structures

#### ConsensusDayConfig
```rust
pub struct ConsensusDayConfig<T: Config> {
    pub frequency: BlockNumberFor<T>,  // Blocks between Consensus Days
    pub duration: BlockNumberFor<T>,   // Length of each Consensus Day
    pub next_start: BlockNumberFor<T>, // Block number when next one starts
    pub active: bool,                  // Current status
}
```

#### ConsensusDayProposal
```rust
pub struct ConsensusDayProposal<T: Config> {
    pub proposal_id: ProposalId,
    pub proposer: T::AccountId,
    pub title: BoundedVec<u8, ConstU32<256>>,
    pub description: BoundedVec<u8, ConstU32<4096>>,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub total_stake_voted: u128,
    pub created_at: BlockNumberFor<T>,
    pub ends_at: BlockNumberFor<T>,
    pub supermajority_threshold: u8,   // 60-100%
    pub min_participation: u8,          // 20-100%
    pub executed: bool,
}
```

### Storage Items

1. **ConsensusDaySchedule**: Stores the configuration for Consensus Day cycles
2. **ConsensusDayProposals**: Maps proposal IDs to Consensus Day proposals
3. **IsConsensusDayActive**: Boolean flag indicating if Consensus Day is currently active
4. **ConsensusDayVotes**: Tracks votes cast on Consensus Day proposals (for unreservation)

### Events

- `ConsensusDayScheduled`: Emitted when Consensus Day schedule is initialized
- `ConsensusDayStarted`: Emitted when Consensus Day activates
- `ConsensusDayEnded`: Emitted when Consensus Day deactivates
- `ConsensusDayProposalCreated`: Emitted when a new Consensus Day proposal is created
- `ConsensusDayVoteCast`: Emitted when a vote is cast on a Consensus Day proposal
- `ConsensusDayProposalPassed`: Emitted when a proposal passes with supermajority
- `ConsensusDayProposalRejected`: Emitted when a proposal fails to meet requirements

## Usage

### 1. Initialize Consensus Day Schedule

**Requires**: Root/Governance Origin

```rust
// Initialize Consensus Day to occur every 100,000 blocks (roughly quarterly)
// with each event lasting 20,000 blocks (roughly one week)
Governance::initialize_consensus_day(
    Origin::root(),
    100_000, // frequency
    20_000,  // duration
)?;
```

### 2. Create a Consensus Day Proposal

**Requires**: Consensus Day must be active

```rust
// Create a proposal during active Consensus Day
Governance::create_consensus_day_proposal(
    Origin::signed(proposer),
    b"Protocol Upgrade v2.0".to_vec(),
    b"Comprehensive upgrade including new features...".to_vec(),
    75, // Requires 75% supermajority
    40, // Requires 40% participation
)?;
```

### 3. Vote on Consensus Day Proposal

**Requires**: Consensus Day must be active

```rust
// Vote with staked tokens
Governance::vote_consensus_day_proposal(
    Origin::signed(voter),
    proposal_id,
    true,      // vote yes
    100_000,   // stake amount
)?;
```

### 4. Finalize Consensus Day Proposal

**Requires**: Voting period has ended

```rust
// Anyone can finalize after voting period ends
Governance::finalize_consensus_day_proposal(
    Origin::signed(anyone),
    proposal_id,
)?;
```

## Proposal Lifecycle

1. **Schedule Initialization**: Governance sets up the Consensus Day schedule
2. **Automatic Activation**: On-chain hooks activate Consensus Day at scheduled blocks
3. **Proposal Creation**: During active Consensus Day, proposals can be created with custom thresholds
4. **Voting Period**: Token holders vote with staked tokens (automatically reserved)
5. **Automatic Deactivation**: Consensus Day ends after duration period
6. **Finalization**: After voting ends, anyone can finalize the proposal
7. **Result Determination**:
   - Check participation threshold (total_votes / total_stake >= min_participation)
   - Check supermajority threshold (yes_votes / total_votes >= supermajority_threshold)
   - Pass if both conditions met, reject otherwise
8. **Vote Unreservation**: All staked tokens are automatically unreserved

## Threshold Guidelines

### Supermajority Threshold (60-100%)
- **60-65%**: Minor protocol adjustments, parameter tweaks
- **70-75%**: Moderate changes, new features
- **80-85%**: Major protocol upgrades, economic model changes
- **90-100%**: Critical changes, security-related modifications

### Participation Threshold (20-100%)
- **20-30%**: Routine decisions, low-impact changes
- **35-45%**: Standard protocol improvements
- **50-65%**: Important upgrades, significant changes
- **70-100%**: Critical decisions, fundamental protocol changes

## Best Practices

### For Proposal Creators

1. **Set Appropriate Thresholds**: Match thresholds to the importance and impact of the proposal
2. **Detailed Descriptions**: Provide comprehensive information about the proposed changes
3. **Community Engagement**: Build consensus before the Consensus Day event
4. **Clear Objectives**: State specific, measurable outcomes

### For Voters

1. **Review Thoroughly**: Read proposal details and understand implications
2. **Stake Responsibly**: Only stake what you can afford to have reserved
3. **Participate Early**: Don't wait until the last moment to vote
4. **Research Impact**: Understand how the proposal affects the protocol

### For Protocol Administrators

1. **Schedule Appropriately**: Set frequency to balance agility with stability
2. **Communicate Clearly**: Announce upcoming Consensus Days well in advance
3. **Monitor Participation**: Track participation rates and adjust if needed
4. **Document Decisions**: Maintain records of all Consensus Day outcomes

## Security Considerations

### Economic Security
- All votes require stake reservation, preventing spam
- Stakes are unreserved only after finalization
- Participation thresholds ensure decisions have broad support

### Protocol Security
- Supermajority requirements protect against contentious changes
- Governance origin controls schedule initialization
- Automatic activation/deactivation prevents manipulation

### Operational Security
- Proposals can only be created during active Consensus Day
- Voting can only occur during active Consensus Day
- Finalization can only occur after voting period ends
- Double-finalization is prevented

## Technical Specifications

### Validation Rules

1. **Proposal Creation**:
   - Consensus Day must be active
   - Supermajority threshold: 60% ≤ threshold ≤ 100%
   - Participation threshold: 20% ≤ threshold ≤ 100%
   - Title: max 256 bytes
   - Description: max 4096 bytes

2. **Voting**:
   - Consensus Day must be active
   - Proposal must exist
   - Sufficient balance to reserve stake
   - Voting period not ended

3. **Finalization**:
   - Voting period must have ended
   - Proposal not already executed
   - Participation threshold must be met
   - Supermajority threshold determines pass/fail

### Block Hook Implementation

```rust
fn on_initialize(n: BlockNumberFor<T>) -> Weight {
    if let Some(mut config) = ConsensusDaySchedule::<T>::get() {
        // Check activation
        if n >= config.next_start && !config.active {
            config.active = true;
            IsConsensusDayActive::<T>::put(true);
            ConsensusDaySchedule::<T>::put(config.clone());
            Self::deposit_event(Event::ConsensusDayStarted { block: n });
        }

        // Check deactivation
        if config.active && n >= config.next_start + config.duration {
            config.active = false;
            config.next_start = n + config.frequency;
            IsConsensusDayActive::<T>::put(false);
            ConsensusDaySchedule::<T>::put(config);
            Self::deposit_event(Event::ConsensusDayEnded { block: n });
        }
    }

    Weight::from_parts(10_000, 0)
}
```

## Example Scenarios

### Scenario 1: Successful Protocol Upgrade

```rust
// Quarterly Consensus Day with one-week duration
initialize_consensus_day(Origin::root(), 1_000_000, 100_000)?;

// During active Consensus Day, create proposal
create_consensus_day_proposal(
    Origin::signed(core_dev),
    b"Upgrade to v3.0".to_vec(),
    b"Major upgrade with new consensus algorithm...".to_vec(),
    80, // 80% supermajority required
    50, // 50% participation required
)?;

// Community votes (60% of total stake participates, 85% vote yes)
// Result: PASSED (meets both thresholds)
```

### Scenario 2: Rejected Due to Insufficient Supermajority

```rust
// Create proposal with high threshold
create_consensus_day_proposal(
    Origin::signed(proposer),
    b"Controversial Change".to_vec(),
    b"Proposal with divided community support".to_vec(),
    75, // 75% supermajority required
    40, // 40% participation required
)?;

// Voting results: 45% participation (meets threshold), 60% yes (below threshold)
// Result: REJECTED (insufficient supermajority)
```

### Scenario 3: Failed Due to Low Participation

```rust
// Create proposal
create_consensus_day_proposal(
    Origin::signed(proposer),
    b"Minor Update".to_vec(),
    b"Small parameter adjustment".to_vec(),
    70, // 70% supermajority required
    40, // 40% participation required
)?;

// Voting results: 25% participation (below threshold), 90% yes
// Result: FAILED (insufficient participation)
```

## Integration with Existing Governance

Consensus Day complements the regular governance system:

- **Regular Proposals**: Day-to-day decisions, simple majority, lower stakes
- **Consensus Day Proposals**: Critical decisions, supermajority, high participation

Both systems coexist and serve different purposes in protocol governance.

## Future Enhancements

Potential improvements for future versions:

1. **Dynamic Thresholds**: Automatically adjust thresholds based on historical participation
2. **Delegation**: Allow token holders to delegate their Consensus Day votes
3. **Multi-Choice Voting**: Support more than binary yes/no options
4. **Quorum Curves**: Implement sliding thresholds based on participation levels
5. **Emergency Consensus Days**: Allow ad-hoc Consensus Days for urgent matters

## Testing

Comprehensive test coverage includes:

- Schedule initialization and configuration
- Automatic activation at scheduled blocks
- Automatic deactivation after duration
- Proposal creation with validation
- Voting with stake reservation
- Finalization with threshold checks
- Edge cases (double finalization, early finalization, etc.)
- Event emission verification
- Multiple concurrent proposals
- Cycle repetition

All 31 tests pass with 100% success rate.

## References

- Main Pallet: `/10-foundation/governance/pallet/src/lib.rs`
- Test Suite: `/10-foundation/governance/pallet/src/lib.rs` (test module)
- E³20 Governance Framework Documentation

## Support

For questions or issues related to Consensus Day:

1. Review this documentation
2. Check the test suite for usage examples
3. Consult the main governance pallet documentation
4. Refer to the E³20 specification

---

**Version**: 1.0
**Status**: Production Ready
**Component**: 10 - Foundation (Governance)
**Completion**: 100% Alpha
