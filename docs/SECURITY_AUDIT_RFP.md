# Ëtrid Protocol - Security Audit Request for Proposal (RFP)

**Document ID**: ETRID-SEC-RFP-2025
**Status**: DRAFT - Ready to Send
**Issue Date**: October 24, 2025
**Response Deadline**: November 15, 2025
**Project Contact**: Eoj Edred, Founder
**Email**: foundation@etrid.org

---

## 📋 EXECUTIVE SUMMARY

**Ëtrid** is seeking qualified security auditing firms to conduct comprehensive security audits of the Ëtrid Protocol ahead of mainnet launch (Q2 2026). We are soliciting proposals from firms with expertise in:

- Blockchain consensus mechanisms (GRANDPA, BABE)
- Substrate/Rust-based blockchain development
- Smart contract security (WebAssembly)
- Cryptographic protocol analysis
- Multi-chain bridge security

**Audit Timeline**: Q1 2026 (January-March)
**Estimated Budget**: $50,000-100,000 per audit firm
**Target**: 2 independent audit firms

---

## 🎯 PROJECT OVERVIEW

### About Ëtrid

**Ëtrid** is a decentralized multichain blockchain platform designed to achieve true democratic governance at scale through:
- **Consensus Day**: Annual on-chain voting event for protocol decisions
- **FODDoS Protocol**: Free and Open Decentralized Democracy of Stakeholders
- **Ascending Scale of Finality (ASF)**: Dynamic consensus mechanism treating finality as a spectrum
- **Partition Burst Chains (PBCs)**: 13 specialized sovereign runtimes for cross-chain interoperability

### Technical Foundation

**Base Framework**: Substrate (Rust)
**Consensus**: Hybrid BABE (block production) + GRANDPA (finality) + ASF (Ëtrid innovation)
**Smart Contracts**: WebAssembly-based runtime (EtwasmVM)
**Network**: Custom p2p protocol (DETR) based on libp2p
**Cryptography**: Ed25519 (current), post-quantum migration path (CRYSTALS-Dilithium, SPHINCS+)

### Current Status

- **E³20 Components**: 13/13 complete (100%)
- **Test Coverage**: 90%+ (412+ tests passing)
- **Documentation**: 67,000+ lines
- **Ember Testnet**: Launching Q1 2026
- **Mainnet Launch**: Target Q2 2026

### Online Resources

- **GitHub**: https://github.com/EojEdred/Etrid
- **Documentation**: docs.etrid.org (deploying Q4 2025)
- **Whitepaper**: Ivory Papers Volumes I-III (available on request)
- **Architecture**: EMBER_TESTNET_ARCHITECTURE.md (attached)

---

## 🔍 AUDIT SCOPE

### Primary Focus Areas

We request comprehensive security audits covering the following components:

#### 1. Core Consensus Mechanisms ⭐ **CRITICAL**

**Components**:
- BABE block production logic
- GRANDPA finality gadget
- **ASF (Ascending Scale of Finality)** - Ëtrid's innovation
- Validator election and rotation
- Session management

**Files**:
```
/pallets/pallet-babe/
/pallets/pallet-grandpa/
/pallets/pallet-authority-discovery/
/pallets/pallet-session/
/pallets/pallet-im-online/
```

**Specific Concerns**:
- Equivocation detection and slashing
- Byzantine fault tolerance (BFT) assumptions
- ASF calculation correctness (finality formula)
- Liveness vs. safety trade-offs
- Block production timing attacks

#### 2. Staking & Economics ⭐ **CRITICAL**

**Components**:
- Validator staking logic
- Reward distribution
- Slashing conditions
- Treasury management
- Fee burning and minting

**Files**:
```
/pallets/pallet-staking/
/pallets/pallet-treasury/
/pallets/pallet-balances/
/pallets/pallet-transaction-payment/
```

**Specific Concerns**:
- Economic exploits (reward gaming, slashing avoidance)
- Integer overflow/underflow in balance calculations
- Precision loss in reward distribution
- Flash loan attacks (if applicable)
- Inflation/deflation mechanics

#### 3. Governance System ⭐ **HIGH PRIORITY**

**Components**:
- Consensus Day voting mechanism
- Proposal submission and execution
- Dual quorum (community + validator)
- Vote weight calculation
- Emergency governance

**Files**:
```
/pallets/pallet-democracy/
/pallets/pallet-collective/
/pallets/pallet-elections-phragmen/
```

**Specific Concerns**:
- Vote manipulation (Sybil, vote buying)
- Proposal spam prevention
- Execution delays and race conditions
- Multi-sig treasury controls
- Emergency override safeguards

#### 4. Cross-Chain Security (PBCs) ⭐ **HIGH PRIORITY**

**Components**:
- Partition Burst Chain (PBC) state proofs
- Cross-chain message passing
- Bridge contracts (Ethereum, Base, BSC)
- Validity node verification
- Collator incentives

**Files**:
```
/pallets/pallet-pbc-registry/
/contracts/ethereum/EtridBridge.sol
/contracts/ethereum/ETR_Ethereum.sol
/contracts/ethereum/EDSC_Ethereum.sol
```

**Specific Concerns**:
- Bridge contract exploits (reentrancy, front-running)
- State proof forgery
- Replay attacks
- Collator misbehavior
- Liquidity pool manipulation

#### 5. Smart Contract Runtime (EtwasmVM) **MEDIUM PRIORITY**

**Components**:
- WebAssembly VM execution
- Gas metering (VMw system)
- Storage access controls
- Contract deployment
- Inter-contract calls

**Files**:
```
/pallets/pallet-etwasm/
/etwasm-runtime/
```

**Specific Concerns**:
- Reentrancy attacks
- Unchecked external calls
- Gas limit exploits
- Storage collision attacks
- Arithmetic overflow/underflow

#### 6. Identity & Access Control **MEDIUM PRIORITY**

**Components**:
- AIDID (AI Decentralized Identity)
- DID Registry
- Multi-signature accounts
- Proxy accounts
- Session key management

**Files**:
```
/pallets/pallet-aidid/
/pallets/pallet-did-registry/
/pallets/pallet-multisig/
/pallets/pallet-proxy/
```

**Specific Concerns**:
- Identity spoofing
- Key management vulnerabilities
- Privilege escalation
- Access control bypasses

#### 7. Network & P2P Layer **LOW PRIORITY**

**Components**:
- DETR p2p protocol
- Peer discovery (Kademlia DHT)
- Gossip protocols
- Transaction pool management

**Files**:
```
/network/src/
/client/network/
```

**Specific Concerns**:
- Eclipse attacks
- Denial of service (DoS)
- Sybil attacks
- Transaction censorship

#### 8. Cryptography **FOUNDATIONAL**

**Components**:
- Signature verification (Ed25519, ECDSA)
- Hash functions (Blake2, SHA-256)
- VRF (Verifiable Random Functions)
- Post-quantum migration path

**Files**:
```
/primitives/core/src/crypto.rs
/primitives/application-crypto/
```

**Specific Concerns**:
- Weak randomness
- Side-channel attacks
- Signature malleability
- Hash collision vulnerabilities

---

## 📦 DELIVERABLES REQUESTED

We expect the following deliverables from the audit:

### 1. Preliminary Assessment Report (Week 1)

**Contents**:
- Initial review of codebase
- High-level architecture assessment
- Identification of critical areas
- Preliminary findings (if any)
- Recommended focus areas

**Format**: PDF, 5-10 pages

### 2. Detailed Audit Report (Final)

**Contents**:
- Executive summary (non-technical stakeholders)
- Methodology description
- Detailed findings per component:
  - **Critical**: Exploitable vulnerabilities requiring immediate fix
  - **High**: Serious issues requiring fix before mainnet
  - **Medium**: Issues to address post-launch
  - **Low**: Recommendations for improvement
  - **Informational**: Best practices, code quality notes
- For each finding:
  - Description
  - Impact assessment
  - Proof of concept (if applicable)
  - Recommended remediation
  - Code references (file, line numbers)
- Summary statistics (findings by severity)
- Recommendations for ongoing security

**Format**: PDF, 30-100 pages

### 3. Re-Audit Report (After Fixes)

**Contents**:
- Verification of fixes for all critical/high findings
- Regression testing results
- Updated severity ratings
- Final approval status

**Format**: PDF, 10-20 pages

### 4. Optional: Public Summary

**Contents**:
- Condensed version for public disclosure
- High-level findings (without exploit details)
- Overall security rating
- Team responsiveness assessment

**Format**: PDF, 2-5 pages

### 5. Communication & Collaboration

**Expected**:
- Weekly progress updates (email or Slack)
- Mid-audit checkpoint meeting (video call)
- Final presentation of findings (video call)
- Availability for follow-up questions (2 weeks post-audit)

---

## 📅 TIMELINE

### Proposed Schedule

```
Phase 1: Proposal Submission
  Oct 24 - Nov 15:  RFP distribution
  Nov 15:           Proposal deadline
  Nov 16-22:        Proposal evaluation
  Nov 25:           Firm selection and contracting

Phase 2: Audit Preparation
  Nov 25 - Dec 5:   NDA signing, repository access
  Dec 5-12:         Preliminary assessment
  Dec 12:           Preliminary report delivery

Phase 3: Detailed Audit
  Dec 12 - Jan 15:  Full audit (5 weeks)
  Jan 15-20:        Internal review by audit firm
  Jan 20:           Detailed audit report delivery

Phase 4: Remediation
  Jan 20 - Feb 15:  Ëtrid team fixes issues (4 weeks)
  Feb 15-20:        Re-audit of fixes
  Feb 20:           Re-audit report delivery

Phase 5: Public Disclosure (optional)
  Feb 20 - Mar 1:   Prepare public summary
  Mar 1:            Public disclosure (with team approval)

Total Duration: 4-5 months (Nov 2025 → Mar 2026)
```

### Flexible Timeline

We understand audit scheduling can be complex. If the proposed timeline doesn't work, please propose an alternative schedule in your proposal.

**Critical Dates** (non-negotiable):
- **Audit completion**: Before Q2 2026 (mainnet launch)
- **Re-audit completion**: At least 2 weeks before mainnet

---

## 💰 BUDGET

### Budget Range

**Per Audit Firm**: $50,000 - $100,000

**Budget Allocation**:
```
Initial audit:       $35,000 - $70,000 (70%)
Re-audit:            $10,000 - $20,000 (20%)
Public summary:      $2,500 - $5,000 (5%)
Ongoing support:     $2,500 - $5,000 (5%)
```

### Payment Terms

**Proposed Schedule**:
- 25% upon contract signing
- 50% upon delivery of detailed audit report
- 25% upon delivery of re-audit report

**Payment Methods**: Wire transfer, USDC, or crypto (ÉTR, ETH, BTC)

**Invoicing**: Net 30 days

### Additional Costs

If your firm requires travel for on-site work, we can discuss reimbursement separately. However, we prefer **remote audits** to minimize costs.

---

## 📊 PROPOSAL REQUIREMENTS

Please structure your proposal as follows:

### 1. Firm Overview (2-3 pages)

- Company name, location, founding date
- Number of employees, security researchers
- Relevant certifications (OSCP, CEH, etc.)
- Notable clients and past projects
- Blockchain-specific experience (Substrate, Rust, Solidity)

### 2. Team Composition (1-2 pages)

- Lead auditor(s) assigned to this project
- Team members (roles, expertise)
- Estimated hours per team member
- Availability and scheduling

### 3. Methodology (2-3 pages)

- Audit approach (manual review, automated tools, fuzzing, etc.)
- Testing environments (local, testnet, mainnet)
- Tools used (Slither, Echidna, cargo-audit, etc.)
- Reporting standards followed (OWASP, etc.)

### 4. Past Work (2-3 pages)

- 3-5 relevant audit examples (blockchain/Rust projects)
- Links to public audit reports (if available)
- Client testimonials or references

### 5. Timeline & Pricing (1 page)

- Detailed timeline (matching our phases or proposing alternative)
- Itemized pricing breakdown
- Payment terms
- Contingency plans (if delays occur)

### 6. References (1 page)

- 2-3 references from past blockchain audit clients
- Contact information (name, role, email, phone)

### 7. Legal & Compliance (optional)

- Insurance coverage (E&O, cyber liability)
- Conflict of interest disclosure
- NDA/confidentiality agreements

**Total Proposal Length**: 10-15 pages (PDF preferred)

---

## 🎯 EVALUATION CRITERIA

Proposals will be evaluated based on:

| Criterion | Weight | Description |
|-----------|--------|-------------|
| **Relevant Experience** | 30% | Substrate/Rust audits, consensus mechanism expertise |
| **Team Qualifications** | 25% | Credentials, past performance, availability |
| **Methodology** | 20% | Thoroughness, use of tools, testing approach |
| **Pricing** | 15% | Cost competitiveness, value for money |
| **Timeline** | 10% | Ability to meet our schedule, flexibility |

### Preferred Qualifications

**Bonus Points**:
- Experience auditing Substrate-based chains (Polkadot, Kusama, parachains)
- Published research on consensus mechanisms or cryptography
- Open-source contributions to blockchain security tools
- Participation in bug bounty programs (HackerOne, Immunefi)
- Formal verification expertise

---

## 🔒 CONFIDENTIALITY & LEGAL

### Non-Disclosure Agreement (NDA)

Selected firms will be required to sign an NDA before receiving repository access. A standard NDA template will be provided, but we are open to using your firm's template if preferred.

### Intellectual Property

All findings and reports produced during the audit are **jointly owned** by Ëtrid and the audit firm:
- Ëtrid may use reports for internal improvement and public disclosure
- Audit firm may use findings for case studies (with permission)
- Exploit details remain confidential until fixes are deployed

### Public Disclosure

We plan to publicly disclose audit results **after remediation** (estimated March 2026). Audit firms will be credited in the public summary. If you prefer anonymity, please specify in your proposal.

---

## 📞 CONTACT INFORMATION

### Primary Contact

**Name**: Eoj Edred
**Title**: Founder, Ëtrid Protocol
**Email**: foundation@etrid.org
**Telegram**: @EojEdred
**Location**: United States

### Technical Contact

**Name**: Development Team
**Email**: dev@etrid.org
**GitHub**: https://github.com/EojEdred/Etrid

### Proposal Submission

**Email**: foundation@etrid.org
**Subject Line**: "Security Audit Proposal - [Your Firm Name]"
**Deadline**: November 15, 2025, 11:59 PM UTC

### Questions & Clarifications

If you have questions about this RFP, please email foundation@etrid.org with subject line "RFP Question - [Your Firm Name]". We will respond within 2 business days.

---

## 📚 APPENDIX A: TECHNICAL DETAILS

### Codebase Statistics (as of Oct 24, 2025)

```
Total Lines of Code:    ~50,000 (Rust)
Total Lines of Docs:    67,000+ (Markdown)
Number of Pallets:      15 custom + 20 standard Substrate
Smart Contracts:        3 Solidity (bridge contracts)
Test Coverage:          90%+ (412+ tests)
Security Tests:         Included (unit, integration)
```

### Technology Stack

```yaml
Language: Rust (stable)
Framework: Substrate (Polkadot SDK)
Consensus: BABE + GRANDPA + ASF (custom)
Smart Contracts: WebAssembly (wasmi interpreter)
Networking: libp2p (custom DETR protocol)
Storage: RocksDB
Cryptography: Ed25519, ECDSA, SR25519
```

### Repository Access

Upon contract signing, audit firms will receive:
- **GitHub**: Private repository access (read-only)
- **Documentation**: Complete technical specs (Ivory Papers I-III)
- **Testnet Access**: Ember testnet RPC endpoints
- **Discord**: Private audit channel for communication

---

## 📚 APPENDIX B: KNOWN ISSUES & CONCERNS

To ensure transparency, we disclose the following known concerns:

### 1. ASF (Ascending Scale of Finality)

**Status**: Novel mechanism, no prior implementations
**Concern**: Mathematical correctness of finality formula
**Mitigation**: Extensive simulation testing, formal verification planned

### 2. Cross-Chain Bridges

**Status**: Solidity contracts written, tests passing
**Concern**: Bridge contract security (reentrancy, front-running)
**Mitigation**: OpenZeppelin contracts used, SafeMath, ReentrancyGuard

### 3. Governance Complexity

**Status**: Consensus Day tested on private testnet
**Concern**: Edge cases in dual quorum, vote weight calculation
**Mitigation**: Simulation testing, fuzzing planned

### 4. VMw Gas Metering

**Status**: Weights calculated via benchmarking
**Concern**: Accuracy of VMw→ÉTR pricing, DoS via gas exhaustion
**Mitigation**: Dynamic pricing, surge fees above 75% block fullness

### 5. Post-Quantum Migration

**Status**: Migration path designed, not yet implemented
**Concern**: Compatibility with existing accounts, key management
**Mitigation**: Hybrid scheme (Ed25519 + PQ), gradual migration

---

## 📚 APPENDIX C: TARGET AUDIT FIRMS

We plan to distribute this RFP to the following firms (non-exclusive):

**Tier 1** (preferred):
- Trail of Bits (https://www.trailofbits.com/)
- Quantstamp (https://quantstamp.com/)
- OpenZeppelin (https://www.openzeppelin.com/security-audits)
- CertiK (https://www.certik.com/)

**Tier 2** (excellent alternatives):
- Halborn (https://halborn.com/)
- Consensys Diligence (https://consensys.net/diligence/)
- SRLabs (https://www.srlabs.de/)
- NCC Group (https://www.nccgroup.com/)

If your firm is not listed above but meets the qualifications, **you are encouraged to submit a proposal**.

---

## 📚 APPENDIX D: QUESTIONS TO ADDRESS IN PROPOSAL

To help us evaluate your proposal, please address the following:

1. **Experience**: How many Substrate/Rust blockchain audits have you completed?
2. **Consensus Expertise**: Have you audited consensus mechanisms before? (PBFT, Tendermint, GRANDPA, etc.)
3. **Tools**: What automated tools will you use? (cargo-audit, Slither, Echidna, custom?)
4. **Testing**: Will you deploy to testnet for live testing? What attack scenarios will you test?
5. **Scope**: Are there any components you recommend adding or removing from scope?
6. **Timeline**: Can you meet our proposed timeline? If not, what alternative do you propose?
7. **Pricing**: Is your pricing fixed or time-and-materials? What if the audit takes longer than expected?
8. **Remediation**: Do you offer support during the fix phase? (e.g., reviewing our patches)
9. **References**: Can you provide 2-3 references from Substrate or Rust projects?
10. **Availability**: When is the earliest you can start? Are there any scheduling conflicts?

---

## ✅ SUBMISSION CHECKLIST

Before submitting your proposal, ensure you've included:

- [ ] Firm overview and team composition
- [ ] Methodology and tools description
- [ ] Past work examples (3-5 relevant audits)
- [ ] Timeline and pricing breakdown
- [ ] References (2-3 contacts)
- [ ] Answers to the 10 questions in Appendix D
- [ ] Legal/compliance information (insurance, conflicts of interest)
- [ ] Contact information (primary contact, phone, email)

**Submit to**: foundation@etrid.org
**Subject**: "Security Audit Proposal - [Your Firm Name]"
**Deadline**: November 15, 2025, 11:59 PM UTC

---

## 🙏 THANK YOU

Thank you for your interest in auditing the Ëtrid Protocol. We look forward to receiving your proposal and potentially partnering with your firm to ensure the security and reliability of our network before mainnet launch.

If you have any questions or need clarification, please don't hesitate to reach out.

**Eoj Edred**
Founder, Ëtrid Protocol
foundation@etrid.org

---

**Document Version**: 1.0 (Draft)
**Last Updated**: October 24, 2025
**Next Review**: Upon first proposal received

---

*This RFP is subject to modification. We will notify all recipients if significant changes are made.*
