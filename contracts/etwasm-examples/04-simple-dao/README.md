# Simple DAO Contract

**Difficulty**: ⭐⭐⭐⭐ Advanced
**Time to Complete**: 2-3 hours

---

## 📖 What You'll Learn

This contract implements a decentralized autonomous organization (DAO) for governance:
- ✅ **Member management** (add/remove members)
- ✅ **Proposal creation** (members create governance proposals)
- ✅ **Voting mechanism** (yes/no votes)
- ✅ **Threshold-based execution** (proposals pass at X% approval)
- ✅ **Time-locked voting** (voting periods with deadlines)
- ✅ **Proposal lifecycle** (Active → Executed/Rejected)
- ✅ **Access control** (owner-only admin functions)
- ✅ **Comprehensive events** (all governance actions logged)
- ✅ **Robust error handling**

---

## 🏗️ Contract Overview

The Simple DAO contract enables decentralized governance where members vote on proposals.

### Storage
```rust
owner: AccountId                                // DAO owner (admin)
members: Mapping<AccountId, bool>               // Member registry
member_count: u32                               // Total members
proposals: Mapping<u32, Proposal>               // All proposals
next_proposal_id: u32                           // Auto-incrementing ID
votes: Mapping<(proposal_id, voter), VoteType> // Vote records
threshold_percentage: u8                        // % needed to pass (e.g., 51)
voting_period: u64                              // Voting duration (ms)
```

### Proposal Structure
```rust
pub struct Proposal {
    id: u32,                    // Unique identifier
    title: String,              // Proposal title
    description: String,        // Full description
    proposer: AccountId,        // Who created it
    yes_votes: u32,            // Yes vote count
    no_votes: u32,             // No vote count
    status: ProposalStatus,    // Current state
    created_at: Timestamp,     // Creation time
    voting_ends_at: Timestamp, // Deadline
}
```

### Proposal Lifecycle
```
Created → Active → Voting Period → Execute/Reject
                                ↓
                         Executed/Rejected
```

### Core Functions

| Function | Type | Access | Description |
|----------|------|--------|-------------|
| `new(threshold, voting_days)` | Constructor | Public | Create DAO |
| `owner()` | Query | Public | Get owner |
| `member_count()` | Query | Public | Get member count |
| `threshold_percentage()` | Query | Public | Get approval threshold |
| `voting_period()` | Query | Public | Get voting period (ms) |
| `is_member(account)` | Query | Public | Check membership |
| `add_member(member)` | Transaction | Owner Only | Add new member |
| `remove_member(member)` | Transaction | Owner Only | Remove member |
| `create_proposal(title, desc)` | Transaction | Members Only | Create proposal |
| `get_proposal(id)` | Query | Public | Get proposal details |
| `proposal_count()` | Query | Public | Get total proposals |
| `vote(proposal_id, vote_type)` | Transaction | Members Only | Cast vote |
| `get_vote(proposal_id, voter)` | Query | Public | Get user's vote |
| `execute_proposal(id)` | Transaction | Public | Execute after voting |
| `set_threshold(percentage)` | Transaction | Owner Only | Change threshold |
| `set_voting_period(days)` | Transaction | Owner Only | Change voting period |
| `transfer_ownership(new_owner)` | Transaction | Owner Only | Change owner |

### Events
- `MemberAdded(member)` - New member joined
- `MemberRemoved(member)` - Member removed
- `ProposalCreated(id, proposer, title)` - New proposal
- `VoteCast(id, voter, vote)` - Vote recorded
- `ProposalExecuted(id, yes, no)` - Proposal passed
- `ProposalRejected(id, yes, no)` - Proposal failed

### Errors
- `NotMember` - Caller is not a member
- `NotOwner` - Caller is not owner
- `AlreadyMember` - Account already a member
- `NotAMember` - Account not a member
- `ProposalNotFound` - Invalid proposal ID
- `ProposalNotActive` - Proposal not in active state
- `AlreadyVoted` - Member already voted
- `VotingEnded` - Voting period expired
- `VotingNotEnded` - Cannot execute before deadline
- `ProposalAlreadyExecuted` - Already executed
- `ThresholdNotReached` - Insufficient approval
- `EmptyTitle` - Title cannot be empty
- `EmptyDescription` - Description cannot be empty
- `InvalidVotingPeriod` - Invalid period value

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 04-simple-dao
cargo contract build --release
```

### 2. Deploy Contract

```bash
# Deploy with 51% threshold, 7-day voting period
cargo contract instantiate \
  --constructor new \
  --args 51 7 \
  --suri //Alice
```

### 3. Interact with Contract

```bash
# Check if Alice is a member
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message is_member \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --suri //Alice \
  --dry-run

# Add Bob as a member (owner only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message add_member \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty \
  --suri //Alice

# Create a proposal (members only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message create_proposal \
  --args "Increase Treasury Allocation" "Allocate 10% of funds to development" \
  --suri //Alice

# Get proposal details
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_proposal \
  --args 0 \
  --suri //Alice \
  --dry-run

# Vote YES on proposal 0
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message vote \
  --args 0 Yes \
  --suri //Alice

# Vote NO on proposal 0 (as Bob)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message vote \
  --args 0 No \
  --suri //Bob

# Check Alice's vote
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_vote \
  --args 0 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --suri //Alice \
  --dry-run

# Execute proposal (after voting period ends)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message execute_proposal \
  --args 0 \
  --suri //Alice

# Change threshold to 66% (owner only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message set_threshold \
  --args 66 \
  --suri //Alice
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

**Expected output**: All 20 tests pass

**Tests cover**:
- DAO initialization
- Member management (add/remove)
- Access control (owner-only functions)
- Proposal creation
- Voting mechanics
- Vote counting
- Threshold calculation
- Proposal execution
- Edge cases (already voted, not member, etc.)

### Run Integration Tests (E2E)

```bash
# Start local node
substrate-contracts-node --dev --tmp

# Run E2E tests
cargo test --features e2e-tests
```

---

## 📝 Code Walkthrough

### Creating a DAO

```rust
// 51% approval required, 7-day voting period
let dao = SimpleDao::new(51, 7);

// Owner is automatically added as first member
assert!(dao.is_member(owner));
assert_eq!(dao.member_count(), 1);
```

### Adding Members

```rust
// Only owner can add members
dao.add_member(bob)?;
dao.add_member(charlie)?;

// Now 3 members total
assert_eq!(dao.member_count(), 3);
```

### Creating a Proposal

```rust
// Members create proposals
let proposal_id = dao.create_proposal(
    "Allocate Treasury Funds".into(),
    "Send 1000 ETR to development team".into()
)?;

// Voting starts immediately
let proposal = dao.get_proposal(proposal_id)?;
assert_eq!(proposal.status, ProposalStatus::Active);
```

### Voting on Proposals

```rust
// Alice votes YES
dao.vote(proposal_id, VoteType::Yes)?;

// Bob votes NO
dao.vote(proposal_id, VoteType::No)?;

// Charlie votes YES
dao.vote(proposal_id, VoteType::Yes)?;

// Result: 2 YES, 1 NO = 66% approval
```

### Executing Proposals

```rust
// Wait for voting period to end (7 days)
// Then anyone can execute

dao.execute_proposal(proposal_id)?;

// If threshold reached (51%):
// - Status → Executed
// - ProposalExecuted event emitted

// If threshold NOT reached:
// - Status → Rejected
// - ProposalRejected event emitted
```

### Threshold Calculation

```rust
// Example: 51% threshold
// 3 members vote: 2 YES, 1 NO
// Calculation: (2 / 3) * 100 = 66%
// 66% >= 51% → APPROVED

// Example: 51% threshold
// 3 members vote: 1 YES, 2 NO
// Calculation: (1 / 3) * 100 = 33%
// 33% < 51% → REJECTED
```

**Key insight**: Threshold is calculated from votes cast, not total members. If only 2 out of 10 members vote, threshold is based on those 2 votes.

### Voting Period

```rust
// 7-day voting period = 7 * 24 * 60 * 60 * 1000 milliseconds
let voting_period_ms = 7 * 24 * 60 * 60 * 1000;

// Proposal created at time T
// Voting ends at time T + 7 days
// Cannot execute before voting ends
// Can execute after voting ends
```

---

## 💡 Try It Yourself

### Challenge 1: Add Quorum Requirement
Require minimum participation (e.g., 50% of members must vote).

**Hint**: Add `quorum_percentage: u8` to storage, check in `execute_proposal()`.

### Challenge 2: Add Proposal Categories
Categorize proposals (Treasury, Governance, Technical, etc.).

**Hint**: Add `category: String` to `Proposal` struct.

### Challenge 3: Add Delegation
Allow members to delegate their voting power to others.

**Hint**: Add `delegates: Mapping<AccountId, AccountId>`.

### Challenge 4: Add Multi-Choice Voting
Support more than Yes/No (e.g., Option A, B, C).

**Hint**: Change `VoteType` to `VoteOption(u8)`, track multiple counts.

### Challenge 5: Add Proposal Cancellation
Allow proposer to cancel their proposal before execution.

**Hint**: Add `cancel_proposal()` function, check `caller == proposal.proposer`.

### Challenge 6: Add Weighted Voting
Give members different voting power based on token holdings.

**Hint**: Replace vote counting with balance-weighted votes.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~150,000 | One-time |
| `is_member()` | ~200 | Read from mapping |
| `add_member()` | ~5,000 | Update mapping + event |
| `create_proposal()` | ~8,000 | Store proposal + event |
| `vote()` | ~6,000 | Update proposal + vote mapping |
| `execute_proposal()` | ~7,000 | Update status + event |
| `get_proposal()` | ~300 | Read proposal data |

**Optimization tips**:
- Batch member additions in deployment
- Keep proposal titles/descriptions concise
- Archive old proposals off-chain
- Use indexed events for efficient querying

---

## 🌐 Real-World Use Cases

### 1. Treasury Management DAO
```rust
// Members vote on fund allocation
"Allocate 5000 ETR to marketing campaign"
"Send 2000 ETR to security audit"
```

### 2. Protocol Governance DAO
```rust
// Members vote on protocol changes
"Upgrade contract to v2.0"
"Change staking reward rate to 8%"
```

### 3. Investment DAO
```rust
// Members vote on investments
"Invest 10 ETH in Project X"
"Exit position in Token Y"
```

### 4. Community DAO
```rust
// Members vote on community decisions
"Accept partnership with Company Z"
"Host conference in Q2 2026"
```

### 5. Developer DAO
```rust
// Members vote on feature prioritization
"Implement feature A in next sprint"
"Hire 2 new developers"
```

---

## 📊 Comparison to Previous Examples

| Feature | ERC20 Token | Simple DAO |
|---------|-------------|------------|
| Storage complexity | High (balances, allowances) | Very high (proposals, votes, members) |
| State management | Token balances | Proposal lifecycle |
| Access control | Owner + allowances | Owner + members |
| Time-based logic | None | Voting periods |
| Multi-user interaction | Transfers | Voting + execution |
| Real-world usage | Token systems | Governance systems |

---

## 🐛 Common Issues

### "NotMember: caller is not a member"
**Cause**: Non-member trying to create proposal or vote
**Solution**: Add member with `add_member()` first

### "AlreadyVoted: member already voted"
**Cause**: Member trying to vote twice on same proposal
**Solution**: Each member can only vote once per proposal

### "VotingEnded: voting period expired"
**Cause**: Trying to vote after deadline
**Solution**: Vote earlier or increase voting period

### "VotingNotEnded: cannot execute before deadline"
**Cause**: Trying to execute proposal before voting period ends
**Solution**: Wait for voting period to end

### "ThresholdNotReached: insufficient approval"
**Cause**: Proposal didn't reach required threshold
**Solution**: Get more YES votes or lower threshold

---

## 🔗 Integrating with Other Contracts

### Example: DAO-Controlled ERC20 Token

```rust
// DAO owns an ERC20 token contract
// Proposals can call token.mint() or token.transfer()

// 1. Deploy ERC20 token
let token = Erc20Token::new(1_000_000, "DAO Token", "DAOT", 18);

// 2. Transfer ownership to DAO
token.transfer_ownership(dao_address);

// 3. Create DAO proposal
dao.create_proposal(
    "Mint 10,000 tokens".into(),
    "Mint tokens for new member rewards".into()
);

// 4. Members vote
// 5. If approved, execute: token.mint(recipient, amount)
```

### Example: DAO-Controlled Treasury

```rust
// DAO controls a treasury contract
// Proposals can withdraw funds

// Proposal: "Send 1000 ETR to developer"
// If approved: treasury.transfer(developer, 1000)
```

---

## 📚 Next Steps

After mastering this contract:
1. **Add token-weighted voting** - Integrate with ERC20 for voting power
2. **Build multi-sig DAO** - Require multiple signatures for execution
3. **Create timelock DAO** - Add delay between approval and execution
4. **Deploy to Ember testnet** - Launch your own DAO!

---

## 📖 Resources

- DAO Concepts: https://ethereum.org/en/dao/
- Compound Governance: https://compound.finance/governance
- Moloch DAO: https://molochdao.com/
- Substrate governance: https://wiki.polkadot.network/docs/learn-governance

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy DAO Building! 🏛️🗳️**
