# Treasury Governance Guide

**Date:** October 25, 2025
**Status:** ✅ Pallet-Treasury integrated (Runtime v103)
**File:** `05-multichain/flare-chain/runtime/src/lib.rs:248-277`

---

## Overview

The Treasury is a decentralized fund management system for Ëtrid's DAO Treasury allocation (875M ÉTR = 35% of total supply).

**Treasury Features:**
- Proposal-based spending (community-driven)
- 7-day spend periods for batch processing
- Root/governance approval required
- No automatic burns (all rejected funds stay in treasury)
- Maximum 100 proposals per spend period

---

## Configuration

### Runtime Parameters

```rust
// File: 05-multichain/flare-chain/runtime/src/lib.rs:248-254

parameter_types! {
    pub const SpendPeriod: BlockNumber = 7 * 24 * 600; // 7 days at 6s blocks
    pub const Burn: Permill = Permill::from_percent(0); // No burn
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const MaxApprovals: u32 = 100; // Max proposals per period
    pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}
```

**Key parameters:**
- **Spend Period:** 7 days (100,800 blocks)
- **Burn Rate:** 0% (no burn, funds stay in treasury)
- **Max Approvals:** 100 proposals can be approved per period
- **Pallet ID:** `py/trsry` (treasury account derivation)

### Access Control

```rust
type RejectOrigin = frame_system::EnsureRoot<AccountId>; // Sudo or governance
type SpendOrigin = NeverEnsureOrigin<Balance>; // Direct spend disabled
```

**Security model:**
- Reject proposals: Requires root/governance (sudo initially, multisig later)
- Approve proposals: Requires root/governance
- Direct spending: Disabled (all spending must go through proposals)

---

## Treasury Account Address

The treasury account is a deterministic address derived from the PalletId:

```rust
pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
```

**To calculate the address:**
```bash
# Using polkadot-js/api
const { u8aToHex } = require('@polkadot/util');
const { blake2AsU8a, encodeAddress } = require('@polkadot/util-crypto');

const PALLET_ID = 'py/trsry';
const palletIdU8a = new Uint8Array([...Buffer.from(PALLET_ID)]);
const address = encodeAddress(blake2AsU8a(palletIdU8a, 256).slice(0, 32), 42);
console.log('Treasury Address:', address);
```

---

## Funding the Treasury

### Option 1: Genesis Allocation

In the mainnet genesis, allocate 875M ÉTR to the treasury account:

```json
{
  "balances": {
    "balances": [
      ["<TREASURY_ADDRESS>", 875000000000000000000]
    ]
  }
}
```

### Option 2: Transfer from Foundation Multisig

If genesis allocates to foundation multisig instead:

```bash
# Using polkadot-js apps or extrinsic
balances.transfer(TREASURY_ADDRESS, 875000000000000000000)
```

### Option 3: Runtime Events

Treasury automatically receives:
- Slashed validator funds (via `OnSlash = Treasury`)
- Transaction fees (if configured)
- Bridge penalties

---

## Using the Treasury

### Step 1: Submit a Proposal

**Anyone can submit a proposal** (with a bond):

```rust
// Extrinsic: treasury.proposeSpend
treasury.proposeSpend {
    value: 10_000_000_000_000_000_000, // 10 ETR (with 12 decimals)
    beneficiary: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // recipient
}
```

**Bond calculation** (in newer treasury versions):
- Bond = 5% of proposed spend (adjustable via ProposalBond)
- Minimum bond = 0.1 ETR
- Maximum bond = 1 ETR

**Bond is returned** when proposal is approved or rejected.

### Step 2: Approval Process

**Root/Governance must approve:**

```rust
// Extrinsic: treasury.approveProposal (sudo required)
sudo.sudo {
    call: treasury.approveProposal {
        proposalId: 0
    }
}
```

**Or via multisig:**

```rust
multisig.asMulti {
    threshold: 5,
    otherSignatories: [...],
    call: treasury.approveProposal { proposalId: 0 }
}
```

### Step 3: Automatic Payout

Once approved, funds are automatically paid out at the end of the current spend period (7 days).

**Payout timing:**
- Proposals approved in days 1-7: Paid out on day 7
- Instant payout: Configured via `PayoutPeriod = 0` (immediate)

### Step 4: Reject Unwanted Proposals

```rust
// Extrinsic: treasury.rejectProposal (sudo required)
sudo.sudo {
    call: treasury.rejectProposal {
        proposalId: 1
    }
}
```

**Effect:**
- Proposal removed from queue
- Bond returned to proposer
- Funds remain in treasury

---

## Proposal Examples

### Example 1: Infrastructure Grant

**Proposal:** Fund a block explorer development

```javascript
api.tx.treasury.proposeSpend(
  50_000_000_000_000_000_000, // 50 ETR
  "5F3sa2TJAWMqDhXG6jhV4N8ko9rKn6xYb8Wo3LzVjj2ZGUYM" // developer address
).signAndSend(proposer);
```

**Justification:** (off-chain, via forum/GitHub)
- Deliverable: Open-source block explorer
- Timeline: 3 months
- Milestones: UI design, backend API, deployment
- Budget: 50 ETR ($5,000 at $100/ETR)

### Example 2: Marketing Campaign

**Proposal:** Fund exchange listing campaign

```javascript
api.tx.treasury.proposeSpend(
  100_000_000_000_000_000_000, // 100 ETR
  "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // marketing team
).signAndSend(proposer);
```

**Justification:**
- Deliverable: Listing on 2 major exchanges
- Timeline: 2 months
- Milestones: Application submission, negotiations, listing
- Budget: 100 ETR ($10,000 at $100/ETR)

### Example 3: Security Audit

**Proposal:** Fund third-party security audit

```javascript
api.tx.treasury.proposeSpend(
  500_000_000_000_000_000_000, // 500 ETR
  "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y" // audit firm
).signAndSend(proposer);
```

**Justification:**
- Deliverable: Comprehensive security audit report
- Timeline: 6 weeks
- Scope: Runtime pallets, bridge contracts, oracle system
- Budget: 500 ETR ($50,000 at $100/ETR)

---

## Treasury Metrics & Monitoring

### Query Treasury Balance

```javascript
const treasuryAccount = api.consts.treasury.palletId;
const balance = await api.query.system.account(treasuryAccount);
console.log('Treasury Balance:', balance.data.free.toString());
```

### Query Pending Proposals

```javascript
const proposalCount = await api.query.treasury.proposalCount();
for (let i = 0; i < proposalCount; i++) {
  const proposal = await api.query.treasury.proposals(i);
  console.log(`Proposal ${i}:`, proposal.toJSON());
}
```

### Query Approvals

```javascript
const approvals = await api.query.treasury.approvals();
console.log('Approved Proposals:', approvals.toJSON());
```

### Spend Period Progress

```javascript
const currentBlock = await api.query.system.number();
const spendPeriod = api.consts.treasury.spendPeriod;
const blocksUntilPayout = spendPeriod - (currentBlock % spendPeriod);
console.log(`Blocks until next payout: ${blocksUntilPayout}`);
```

---

## Governance Transition Plan

### Phase 1: Sudo Control (Launch → 3 months)

**Current state:**
- Sudo key controls approval/rejection
- Foundation multisig holds sudo
- Fast decision-making for critical proposals

**Operations:**
- Foundation reviews all proposals
- Quick approval for urgent needs
- Community feedback via Discord/forum

### Phase 2: Multisig Governance (3-6 months)

**Transition:**
- Transfer sudo to 5-of-7 multisig
- Multisig includes:
  - 3 foundation members
  - 2 community representatives
  - 2 technical advisors

**Operations:**
- Multisig threshold: 5 signatures required
- Proposal review period: 3 days minimum
- Off-chain coordination via governance forum

### Phase 3: On-Chain Governance (6-12 months)

**Full decentralization:**
- Implement pallet-collective (Council)
- Implement pallet-democracy (Referenda)
- Community token-weighted voting

**Operations:**
- Proposals require council motion + public vote
- Voting period: 14 days
- Quorum: 10% of circulating supply
- Approval threshold: 51% majority

---

## Treasury Best Practices

### For Proposers

1. **Off-chain discussion first:**
   - Post proposal on governance forum
   - Gather community feedback
   - Revise based on comments

2. **Clear deliverables:**
   - Specific, measurable outcomes
   - Timeline with milestones
   - Budget breakdown

3. **Reputation matters:**
   - Link to previous work
   - Provide references
   - Show team credentials

4. **Milestone-based funding:**
   - Request partial payments
   - Tie payments to deliverables
   - Build trust over time

### For Approvers

1. **Due diligence:**
   - Verify proposer identity
   - Check feasibility
   - Assess value-for-money

2. **Community sentiment:**
   - Check forum discussion
   - Consider feedback
   - Balance popular support with strategic value

3. **Risk assessment:**
   - Technical risks
   - Reputation risks
   - Financial risks

4. **Precedent consideration:**
   - Consistent with previous approvals
   - Fair to other applicants
   - Sustainable long-term

---

## Treasury Analytics

### Expected Inflows

**From genesis:**
- Initial balance: 875,000,000 ÉTR ($87.5M at $0.10/ETR)

**Ongoing sources:**
- Transaction fees: ~0.001 ETR per tx
- Slashed validator funds: Variable
- Bridge penalties: Variable

**Annual projection (conservative):**
- Tx volume: 1M tx/year × 0.001 ETR = 1,000 ETR/year
- Total inflows: ~1,000 ETR/year ($100,000/year at $100/ETR)

### Expected Outflows

**Categories:**
- Infrastructure & tooling: 30% (~$300k/year)
- Marketing & adoption: 25% (~$250k/year)
- Security & audits: 20% (~$200k/year)
- Community grants: 15% (~$150k/year)
- Operations: 10% (~$100k/year)

**Annual budget (conservative):**
- Total spending: ~$1M/year
- Treasury depletion rate: ~875 years at current prices

---

## Emergency Procedures

### Freeze Treasury (Emergency Only)

If treasury is compromised:

```rust
// Via sudo
sudo.sudo {
    call: balances.forceTransfer {
        source: TREASURY_ADDRESS,
        dest: EMERGENCY_MULTISIG,
        value: <FULL_BALANCE>
    }
}
```

### Recover from Malicious Proposal

If bad proposal approved:

```rust
// Reject before payout
sudo.sudo {
    call: treasury.rejectProposal { proposalId: X }
}
```

### Multi-Sig Recovery

If sudo key lost, use multisig:

```rust
multisig.asMulti {
    threshold: 5,
    otherSignatories: [...],
    call: system.setCode { code: <NEW_RUNTIME> }
}
```

---

## Integration with Existing Governance

The Treasury pallet integrates with:

1. **pallet-governance** (Ëtrid custom)
   - Existing governance proposals
   - Voting mechanisms
   - Execution framework

2. **pallet-multisig**
   - Foundation multisig controls
   - Threshold signatures
   - Batched approvals

3. **pallet-sudo**
   - Initial bootstrap phase
   - Emergency interventions
   - Gradual removal

**Recommendation:** Use pallet-governance for proposal discussions, pallet-treasury for actual fund management.

---

## Treasury Address Calculation

**Derivation path:**
```
PalletId("py/trsry")
→ blake2_256(b"py/trsry")
→ SS58 encode (prefix 42)
→ Treasury Address
```

**Example (pseudo-code):**
```python
import hashlib
from substrateinterface import Keypair

pallet_id = b"modlpy/trsry"  # "modl" prefix + pallet_id
hash_result = hashlib.blake2b(pallet_id, digest_size=32).digest()
treasury_address = Keypair(public_key=hash_result, ss58_format=42).ss58_address
print(f"Treasury Address: {treasury_address}")
```

---

## Summary

**Treasury pallet provides:**
- ✅ Decentralized fund management
- ✅ Proposal-based spending
- ✅ Community governance
- ✅ Transparent operations
- ✅ Automatic payouts

**Configuration:**
- Spend period: 7 days
- No burns: 100% retained
- Max approvals: 100 per period
- Access control: Root/governance

**Treasury balance:**
- Genesis allocation: 875,000,000 ÉTR
- Current runtime version: 103
- Status: ✅ Production ready

---

**Next steps:**
1. Fund treasury account from genesis
2. Set up governance forum for proposals
3. Document proposal submission process
4. Train community on treasury usage
5. Establish proposal review committee

**Status:** ✅ Treasury pallet integrated and ready for mainnet
