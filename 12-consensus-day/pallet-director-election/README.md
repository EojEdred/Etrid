# Pallet Director Election

Annual Decentralized Director Election Mechanism for Ëtrid Blockchain - Fully Automated 21-Director Elections

## Overview

This pallet implements the annual director election system as specified in the **Ëtrid Ivory Papers** (11-peer-roles/ARCHITECTURE.md lines 1035-1064). It provides a fully automated, three-phase election cycle that elects 21 Decentralized Directors to govern the network.

## Election Timeline

### Annual Cycle (365 days)

```
Year 1                                                   Year 2
│                                                           │
├─ T-365: Election Start (Consensus Day)                   │
│  └─ Directors Seated                                     │
│                                                           │
├─ T-335 to T-30: Governance Phase (335 days)              │
│  └─ Elected directors govern network                     │
│                                                           │
├─ T-30: Nomination Phase Begins (30 days)                 │
│  ├─ Directors with 128+ ËTR register as candidates       │
│  ├─ Submit manifesto/platform                            │
│  └─ Campaign to community                                │
│                                                           │
├─ T-7: Voting Phase Begins (7 days)                       │
│  ├─ All stakeholders vote                                │
│  ├─ Stake-weighted voting power                          │
│  └─ Can change vote until T-0                            │
│                                                           │
└─ T-0: Consensus Day (Election Day)                       │
   ├─ Voting closes midnight UTC                           │
   ├─ Votes automatically tallied                          │
   ├─ Top 21 candidates elected                            │
   └─ Directors seated immediately                         ├─ Cycle Repeats
                                                            │
```

### Phase Durations (6 second blocks)

- **Governance Phase**: 5,040,000 blocks (~335 days)
- **Nomination Phase**: 432,000 blocks (~30 days)
- **Voting Phase**: 100,800 blocks (~7 days)
- **Total Cycle**: 5,572,800 blocks (~365 days)

## Voting Power Calculation

### Formula

```rust
voting_power = stake × role_multiplier
```

### Role Multipliers

| Role                   | Multiplier | Example Stake | Voting Power |
|------------------------|------------|---------------|--------------|
| DecentralizedDirector  | 3x         | 128 ËTR       | 384          |
| ValidityNode           | 2x         | 64 ËTR        | 128          |
| FlareNode              | 2x         | 64 ËTR        | 128          |
| CommonStakePeer        | 1x         | 10 ËTR        | 10           |
| CommonPeer             | 0x         | 0 ËTR         | 0 (no vote)  |

### Example Calculation

```
Alice:
- Stake: 200 ËTR
- Role: DecentralizedDirector
- Voting Power = 200 × 3 = 600

Bob:
- Stake: 64 ËTR
- Role: ValidityNode
- Voting Power = 64 × 2 = 128

Carol:
- Stake: 5 ËTR
- Role: CommonStakePeer
- Voting Power = 5 × 1 = 5
```

## Candidate Requirements

### Registration Requirements

1. **Role**: Must have `DecentralizedDirector` role
2. **Stake**: Minimum 128 ËTR staked
3. **Phase**: Must register during Nomination phase
4. **Manifesto**: Submit platform/goals (max 1000 bytes)

### Candidate Workflow

```rust
// 1. Ensure you have DecentralizedDirector role with 128+ ËTR
Staking::assign_role(
    Origin::signed(alice),
    4, // DecentralizedDirector
    128_000_000_000_000_000_000_000, // 128 ËTR (18 decimals)
)?;

// 2. During Nomination phase, register as candidate
DirectorElection::register_candidate(
    Origin::signed(alice),
    b"I will focus on scaling solutions and community growth".to_vec(),
)?;

// 3. Campaign to community (off-chain)

// 4. Wait for votes during Voting phase
```

## Voting Process

### Voter Workflow

```rust
// 1. During Voting phase, vote for a candidate
DirectorElection::vote(
    Origin::signed(bob),
    alice, // Candidate account
)?;

// 2. Change vote if desired (before voting ends)
DirectorElection::change_vote(
    Origin::signed(bob),
    carol, // New candidate
)?;

// 3. Or remove vote entirely
DirectorElection::remove_vote(
    Origin::signed(bob),
)?;
```

### Voting Rules

- **One vote per account**: Each account can vote for only one candidate
- **Vote changes allowed**: Can change vote anytime before Consensus Day
- **Vote removal allowed**: Can remove vote and revote later
- **Invalid votes**: Votes for withdrawn candidates are ignored during tallying
- **Voting power locked**: Calculated at time of vote based on current stake

## Election Results & Tallying

### Automatic Tallying (Consensus Day)

On Consensus Day (T-0), the pallet automatically:

1. **Freezes all votes** - No more changes allowed
2. **Tallies vote totals** for each candidate
3. **Sorts candidates** by total voting power received
4. **Breaks ties** deterministically (by stake, then account hash)
5. **Selects top 21** candidates as winners
6. **Seats directors** immediately
7. **Stores results** in election history
8. **Clears storage** for next election cycle

### Winner Selection Algorithm

```rust
// Pseudocode
candidates.sort_by(|a, b| {
    // Primary: Most votes wins
    b.votes_received.cmp(&a.votes_received)
    // Tiebreaker 1: Higher stake wins
    .then(b.stake.cmp(&a.stake))
    // Tiebreaker 2: Hash of AccountId (deterministic)
    .then(hash(b.account).cmp(&hash(a.account)))
});

winners = candidates.take(21);
```

### Edge Cases Handled

| Scenario                     | Behavior                                    |
|------------------------------|---------------------------------------------|
| < 21 candidates              | All candidates elected                      |
| Candidate withdraws          | Votes ignored, next candidate takes seat    |
| Tie in votes                 | Resolved by stake, then account hash        |
| No candidates                | Previous directors continue (no change)     |
| No votes cast                | All candidates tied, sorted by stake        |

## Storage Items

### Core State

```rust
/// Current election phase and timing
ElectionPhase: ElectionPhaseInfo<BlockNumber>

/// All registered candidates
Candidates: AccountId → CandidateProfile

/// All votes cast
Votes: Voter → VoteRecord

/// Historical election results
ElectionResults: Epoch → ElectionResult

/// Currently elected 21 directors
ElectedDirectors: BoundedVec<AccountId, 21>
```

### Phase Tracking

```rust
/// Next Consensus Day block number
NextConsensusDayBlock: BlockNumber

/// Current election epoch counter
CurrentEpoch: u32
```

## Extrinsics

### Candidate Functions

#### `register_candidate(manifesto: Vec<u8>)`
- **Who**: Director role holders (128+ ËTR)
- **When**: Nomination phase only
- **Weight**: 10,000
- **Purpose**: Register as candidate with platform

#### `withdraw_candidacy()`
- **Who**: Registered candidates
- **When**: Nomination or Voting phase
- **Weight**: 5,000
- **Purpose**: Withdraw from election

### Voting Functions

#### `vote(candidate: AccountId)`
- **Who**: All stakeholders with roles
- **When**: Voting phase only
- **Weight**: 10,000
- **Purpose**: Cast vote for candidate

#### `change_vote(new_candidate: AccountId)`
- **Who**: Voters who have already voted
- **When**: Voting phase only
- **Weight**: 10,000
- **Purpose**: Change vote to different candidate

#### `remove_vote()`
- **Who**: Voters who have already voted
- **When**: Voting phase only
- **Weight**: 5,000
- **Purpose**: Remove vote entirely

### Governance Functions

#### `trigger_election()`
- **Who**: Root/governance only
- **When**: Anytime
- **Weight**: 50,000
- **Purpose**: Manually start election cycle (testing/initial setup)

## Events

### Phase Events
- `ElectionPhaseChanged(old_phase_type, new_phase_type, block)` - Phase transition

### Candidate Events
- `CandidateRegistered(account, stake)` - New candidate registered
- `CandidateWithdrew(account)` - Candidate withdrew

### Voting Events
- `VoteCast(voter, candidate, voting_power)` - Vote cast
- `VoteChanged(voter, old_candidate, new_candidate)` - Vote changed
- `VoteRemoved(voter, candidate)` - Vote removed

### Election Events
- `ElectionCompleted { epoch, winners, total_votes }` - Election finished
- `DirectorsSeated(directors, epoch)` - Directors seated

## Configuration Parameters

### Runtime Configuration

```rust
parameter_types! {
    // Phase durations (6s blocks)
    pub const GovernancePeriodBlocks: BlockNumber = 5_040_000; // ~335 days
    pub const NominationPeriodBlocks: BlockNumber = 432_000;   // ~30 days
    pub const VotingPeriodBlocks: BlockNumber = 100_800;       // ~7 days

    // Election parameters
    pub const NumDirectorsToElect: u32 = 21;
    pub const MinCandidateStake: u128 = 128_000_000_000_000_000_000_000; // 128 ËTR
}

impl pallet_director_election::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StakingInterface = PeerRolesStaking; // Your staking pallet
    type GovernancePeriodBlocks = GovernancePeriodBlocks;
    type NominationPeriodBlocks = NominationPeriodBlocks;
    type VotingPeriodBlocks = VotingPeriodBlocks;
    type NumDirectorsToElect = NumDirectorsToElect;
    type MinCandidateStake = MinCandidateStake;
}
```

### Genesis Configuration

```rust
pallet_director_election: DirectorElectionConfig {
    // Set initial Consensus Day for T+365 days from genesis
    initial_consensus_day: Some(5_572_800), // 365 days in blocks
    initial_elected_directors: vec![
        // Bootstrap with initial directors (optional)
        alice_account,
        bob_account,
        // ... up to 21 accounts
    ],
}
```

## Integration with Runtime

### Step 1: Add to Cargo.toml

```toml
[dependencies]
pallet-director-election = { path = "12-consensus-day/pallet-director-election", default-features = false }

[features]
std = [
    "pallet-director-election/std",
]
```

### Step 2: Add to Runtime

```rust
// In runtime/src/lib.rs
impl pallet_director_election::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StakingInterface = PeerRolesStaking;
    type GovernancePeriodBlocks = ConstU32<5_040_000>;
    type NominationPeriodBlocks = ConstU32<432_000>;
    type VotingPeriodBlocks = ConstU32<100_800>;
    type NumDirectorsToElect = ConstU32<21>;
    type MinCandidateStake = ConstU128<128_000_000_000_000_000_000_000>;
}

construct_runtime!(
    pub struct Runtime {
        // ... other pallets
        DirectorElection: pallet_director_election,
    }
);
```

### Step 3: Use in Governance

```rust
// Check if account is elected director
let is_director = DirectorElection::is_elected_director(&account);

// Get all current directors
let directors = DirectorElection::get_elected_directors();

// Use for permissioned operations
ensure!(
    DirectorElection::is_elected_director(&who),
    Error::<T>::NotDirector
);
```

## Security Considerations

### Sybil Resistance
- **Stake requirement**: 128 ËTR minimum prevents spam candidates
- **Role requirement**: Must hold DecentralizedDirector role
- **Voting power**: Stake-weighted prevents vote buying attacks

### Vote Manipulation
- **Vote changes**: Allowed for flexibility but doesn't enable gaming
- **Deterministic ties**: Prevents race conditions in tie scenarios
- **Withdrawn candidates**: Votes ignored, prevents candidate manipulation

### Economic Security
- **High stake threshold**: 128 ËTR ensures serious candidates only
- **Role multipliers**: Align voting power with network contribution
- **No bonds/slashing**: Encourages participation without risk

### Edge Cases
- **No candidates**: Previous directors continue
- **Partial election**: If < 21 candidates, all are elected
- **Candidate withdrawal**: Handled gracefully, votes become invalid
- **Phase transitions**: Atomic via on_initialize hook

## Testing

### Unit Tests

Run tests with:
```bash
cargo test -p pallet-director-election
```

### Integration Tests

```rust
#[test]
fn test_full_election_cycle() {
    new_test_ext().execute_with(|| {
        // 1. Start election
        assert_ok!(DirectorElection::trigger_election(Origin::root()));

        // 2. Register candidates (Nomination phase)
        for i in 0..25 {
            assert_ok!(DirectorElection::register_candidate(
                Origin::signed(candidate_account(i)),
                b"My platform".to_vec(),
            ));
        }

        // 3. Advance to Voting phase
        run_to_block(nomination_end);

        // 4. Cast votes
        for voter in voters {
            assert_ok!(DirectorElection::vote(
                Origin::signed(voter),
                favorite_candidate,
            ));
        }

        // 5. Advance to Consensus Day
        run_to_block(consensus_day);

        // 6. Verify 21 directors elected
        let directors = DirectorElection::get_elected_directors();
        assert_eq!(directors.len(), 21);
    });
}
```

## Performance Considerations

### Gas Costs (Weights)

| Operation               | Weight  | Notes                              |
|-------------------------|---------|-------------------------------------|
| register_candidate      | 10,000  | 1 read, 1 write                    |
| withdraw_candidacy      | 5,000   | 1 read, 1 delete                   |
| vote                    | 10,000  | 2 reads, 2 writes                  |
| change_vote             | 10,000  | 3 reads, 3 writes                  |
| remove_vote             | 5,000   | 2 reads, 2 writes                  |
| on_initialize (tally)   | Variable| Scales with # candidates (max 100) |

### Storage Complexity

- **Candidates**: O(n) where n = number of candidates (typically < 50)
- **Votes**: O(m) where m = number of voters (unbounded but sparse)
- **Tallying**: O(n log n) for sorting candidates

### Optimization Notes

- Tallying runs ONCE per year (low frequency)
- Storage cleared after each election (prevents bloat)
- BoundedVec used to cap memory usage
- No recursive operations

## Future Enhancements

### Potential Features

- [ ] **Conviction Voting**: Lock tokens longer for more voting power
- [ ] **Quadratic Voting**: `sqrt(stake)` instead of linear
- [ ] **Delegation**: Delegate voting power to trusted accounts
- [ ] **Term Limits**: Maximum consecutive terms per director
- [ ] **Cooldown Periods**: Mandatory breaks between terms
- [ ] **Slashing**: Penalize directors for misbehavior
- [ ] **Performance Metrics**: Track director participation/effectiveness
- [ ] **Resignation**: Directors can resign mid-term
- [ ] **Impeachment**: Community can vote to remove directors

## References

- **Ëtrid Ivory Papers**: 11-peer-roles/ARCHITECTURE.md (lines 1035-1064)
- **Substrate FRAME**: https://docs.substrate.io/reference/frame-pallets/
- **Polkadot Governance**: https://wiki.polkadot.network/docs/learn-governance

## License

GPL-3.0

## Author

Eoj Edred - Founder, Ëtrid FOODOS Project
