# Governance Proposal Templates

Standard templates for submitting governance proposals on ËTRID.

## Treasury Proposal Template

```markdown
# Treasury Proposal: [Project Name]

## Summary
[One paragraph summary of what you're proposing]

## Proposer
- **Name/Organization:**
- **Wallet Address:**
- **Contact:**
- **Previous Contributions:**

## Requested Funding
- **Amount:** X,XXX ÉTR
- **USD Equivalent:** $X,XXX (at current rate)

## Project Description

### Overview
[Detailed description of what you plan to build/deliver]

### Problem Statement
[What problem does this solve for ËTRID ecosystem?]

### Solution
[How will your project solve this problem?]

### Target Audience
[Who will benefit from this project?]

## Deliverables

1. **[Deliverable 1]**
   - Description
   - Timeline
   - Success Criteria

2. **[Deliverable 2]**
   - Description
   - Timeline
   - Success Criteria

## Timeline

| Milestone | Deliverable | Duration | Funding |
|-----------|-------------|----------|---------|
| Milestone 1 | [Description] | 4 weeks | X ÉTR |
| Milestone 2 | [Description] | 4 weeks | X ÉTR |
| Milestone 3 | [Description] | 4 weeks | X ÉTR |

**Total Duration:** X weeks

## Budget Breakdown

| Item | Description | Cost (ÉTR) |
|------|-------------|------------|
| Development | [Details] | X,XXX |
| Design | [Details] | X,XXX |
| Testing | [Details] | X,XXX |
| Documentation | [Details] | X,XXX |
| Contingency (10%) | Buffer | X,XXX |
| **Total** | | **X,XXX ÉTR** |

## Team

### [Team Member 1]
- **Role:** Lead Developer
- **Background:** [Experience and qualifications]
- **GitHub:** [Profile link]
- **Commitment:** X hours/week

### [Team Member 2]
- **Role:** [Role]
- **Background:** [Experience]
- **Commitment:** X hours/week

## Success Metrics

- [ ] Metric 1: [Specific, measurable goal]
- [ ] Metric 2: [Specific, measurable goal]
- [ ] Metric 3: [Specific, measurable goal]

## Risks and Mitigation

### Risk 1: [Description]
**Probability:** Low/Medium/High
**Impact:** Low/Medium/High
**Mitigation:** [How you'll address this risk]

### Risk 2: [Description]
**Probability:** Low/Medium/High
**Impact:** Low/Medium/High
**Mitigation:** [How you'll address this risk]

## Community Benefit

[Explain how this benefits the ËTRID community]

## Open Source Commitment

- [ ] All code will be open source (MIT/Apache 2.0 license)
- [ ] Documentation will be public
- [ ] Regular progress updates to community

## References

- [Relevant links, research, or prior work]

## Discussion

[Link to forum discussion thread]

---

**Proposal Submitted:** [Date]
**On-Chain Proposal ID:** [Once submitted]
```

---

## Runtime Upgrade Proposal Template

```markdown
# Runtime Upgrade Proposal: [Feature/Version]

## Summary
[Brief description of the upgrade]

## Motivation

### Problem
[What current limitation or issue does this address?]

### Solution
[How does this upgrade solve it?]

## Technical Specification

### Changes

#### Pallet: [Pallet Name]
**Changes:**
- [Specific change 1]
- [Specific change 2]

**Breaking Changes:** Yes/No
**Migration Required:** Yes/No

#### Storage Changes
- [List any storage migrations]

#### Extrinsics Added/Modified
- [List new or changed functions]

### Testing

- [X] Unit tests pass
- [X] Integration tests pass
- [X] Benchmarked (weights updated)
- [X] Tested on testnet for X days
- [ ] Security audit completed

### Audit Report
[Link to security audit or "N/A - minor change"]

## Deployment Plan

### Testnet Deployment
- **Date:** [Date]
- **Block:** [Block number]
- **Testing Period:** X days

### Mainnet Deployment
- **Proposed Date:** [Date]
- **Notice Period:** 7 days minimum
- **Rollback Plan:** [Describe if things go wrong]

## Impact Assessment

### Node Operators
[Do validators need to upgrade? When?]

### DApp Developers
[Will this break existing DApps?]

### End Users
[How will users be affected?]

## Resources

- **Code:** [GitHub PR link]
- **Documentation:** [Docs PR link]
- **Discussion:** [Forum thread]

---

**Proposed by:** [Council/Technical Committee/Address]
**Proposal Hash:** [Hash]
```

---

## Parameter Change Proposal Template

```markdown
# Parameter Change Proposal: [Parameter Name]

## Summary
Change [parameter name] from [current value] to [proposed value]

## Parameter Details

**Parameter:** `[module].[parameter]`
**Current Value:** [value]
**Proposed Value:** [value]
**Change:** [+X% / -X%]

## Rationale

### Why This Change?
[Explain the reasoning]

### Data Supporting Change
[Provide metrics, charts, or analysis]

### Expected Impact

**Positive:**
- [Benefit 1]
- [Benefit 2]

**Potential Risks:**
- [Risk 1 and mitigation]
- [Risk 2 and mitigation]

## Simulation Results

[Results from simulating this change on testnet or staging]

## Comparison with Other Chains

| Chain | Parameter Value | Notes |
|-------|----------------|-------|
| ËTRID (current) | [value] | |
| Polkadot | [value] | |
| Kusama | [value] | |

## Implementation

- **Takes Effect:** Immediately upon enactment
- **Requires:** On-chain governance approval
- **Can Be Reverted:** Yes

## Monitoring Plan

[How will we monitor the effects of this change?]

---

**Proposed by:** [Address]
**Discussion:** [Forum link]
```

---

## Community Initiative Proposal Template

```markdown
# Community Initiative: [Initiative Name]

## Overview
[What is this initiative?]

## Goals

1. [Primary goal]
2. [Secondary goal]
3. [Tertiary goal]

## Target Audience
[Who will participate/benefit?]

## Activities

### [Activity 1]
- **Description:** [Details]
- **Schedule:** [When/frequency]
- **Resources Needed:** [What's required]

### [Activity 2]
- **Description:** [Details]
- **Schedule:** [When/frequency]
- **Resources Needed:** [What's required]

## Budget Request

**Total Requested:** X,XXX ÉTR

| Item | Cost |
|------|------|
| [Item 1] | X ÉTR |
| [Item 2] | X ÉTR |
| **Total** | **X ÉTR** |

## Success Metrics

- [Metric 1: e.g., "50 participants"]
- [Metric 2: e.g., "10 new contributors"]
- [Metric 3: e.g., "5 projects launched"]

## Timeline

- **Month 1:** [Activities]
- **Month 2:** [Activities]
- **Month 3:** [Activities]

## Team

- **Organizer:** [Name]
- **Volunteers:** [Names or "recruiting"]
- **Advisors:** [Names]

---

**Proposal by:** [Name/Address]
**Discussion:** [Forum link]
```

---

## Submitting a Proposal

### Step 1: Draft Proposal
1. Choose appropriate template
2. Fill in all sections
3. Be specific and detailed

### Step 2: Community Discussion
1. Post to [forum.etrid.org](https://forum.etrid.org)
2. Announce in Discord/Telegram
3. Gather feedback for at least 7 days
4. Revise based on input

### Step 3: On-Chain Submission

```bash
# Via CLI
etrid-cli governance propose-treasury \
  --beneficiary <YOUR_ADDRESS> \
  --value <AMOUNT_IN_ETR> \
  --bond <BOND_AMOUNT>

# Or via wallet UI
# Navigate to Governance → Treasury → Submit Proposal
```

### Step 4: Voting Period
- Council reviews proposal
- Community discusses
- Vote YES/NO with conviction

### Step 5: Execution
- If approved: Funds released upon milestone completion
- If rejected: Bond forfeited, proposal archived

---

## Proposal Best Practices

### ✅ DO

- Provide detailed budget breakdown
- Include realistic timeline
- Show previous work/experience
- Define clear success metrics
- Engage with community feedback
- Update proposal based on discussion

### ❌ DON'T

- Request excessive funding
- Provide vague deliverables
- Skip community discussion
- Ignore feedback
- Rush the process
- Make unrealistic promises

---

## Proposal Review Criteria

Proposals are evaluated on:

1. **Community Benefit** (30%)
   - How many people benefit?
   - Does this advance ËTRID ecosystem?

2. **Feasibility** (25%)
   - Is the timeline realistic?
   - Does team have required skills?

3. **Budget** (20%)
   - Is funding reasonable?
   - Clear budget breakdown?

4. **Deliverables** (15%)
   - Clearly defined?
   - Measurable outcomes?

5. **Team** (10%)
   - Proven track record?
   - Sufficient capacity?

---

## Resources

**Governance:**
- [Governance Overview](../specifications/governance-appendix.md)
- [Treasury Guide](../ai-devs/TREASURY_GOVERNANCE_GUIDE.md)
- [Forum](https://forum.etrid.org)

**Support:**
- 💬 [Discord #governance](https://discord.gg/etrid)
- 📧 Email: governance@etrid.org

---

**Ready to submit?** [Start Discussion →](https://forum.etrid.org/c/proposals)
