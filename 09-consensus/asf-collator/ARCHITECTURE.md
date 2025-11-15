# ASF Collator Architecture

## System Overview

```
┌────────────────────────────────────────────────────────────────────────┐
│                         FlareChain (Relay Chain)                        │
│                        21 Validators + ASF Consensus                    │
└────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
        ┌───────────▼──────┐   ┌───▼────────┐  ┌──▼────────────┐
        │   BTC-PBC        │   │  ETH-PBC   │  │   SOL-PBC     │
        │ 11 Collators     │   │11 Collators│  │  9 Collators  │
        │ ASF Consensus    │   │ASF Consensus│  │ ASF Consensus │
        └──────────────────┘   └────────────┘  └───────────────┘
                │                     │               │
                └─────────────────────┼───────────────┘
                                      │
                        ┌─────────────▼────────────┐
                        │  ASF Bridge Security     │
                        │  Cross-Chain Transfers   │
                        └──────────────────────────┘
```

## Module Architecture

### 1. ASF Collator Module (`asf-collator`)

```
asf-collator/
│
├─ Core Types (lib.rs)
│  ├─ CollatorCommittee         → Manages collator set
│  ├─ CollatorVote              → Individual collator votes
│  ├─ CollatorCertificate       → Aggregated validity certificates
│  ├─ CollatorProposal          → Block proposals
│  ├─ CollatorFinalityLevel     → 5-level finality scale
│  └─ RelayChainFinalityProof   → Relay chain finality inheritance
│
├─ Collator Rotation (collator_rotation.rs)
│  ├─ RotationManager           → Deterministic selection
│  ├─ RotationStrategy          → Round-robin/Stake-weighted/Hybrid
│  ├─ RotationConfig            → Period, cooldown settings
│  └─ CollatorSelection         → Selection results
│
├─ Finality Inheritance (finality_inheritance.rs)
│  ├─ FinalityTracker           → Tracks local + relay finality
│  ├─ FinalityStatus            → Combined finality state
│  └─ RelayChainSyncCoordinator → Sync with relay chain
│
├─ Cross-Chain Attestations (cross_chain_attestations.rs)
│  ├─ CrossChainAttestation     → Individual attestations
│  ├─ MultiSigAttestation       → Aggregated attestations
│  └─ BridgeSecurityManager     → Manages bridge attestations
│
└─ Committee Management (committee_management.rs)
   ├─ SessionManager            → Session lifecycle
   ├─ StakeManager              → Stake deposits & slashing
   ├─ CollatorStake             → Stake records
   └─ CommitteeChange           → Committee updates
```

### 2. Pallet ASF Collator (`pallet-asf-collator`)

```
pallet-asf-collator/
│
├─ Storage
│  ├─ Committee                 → Current collator committee
│  ├─ CollatorStakes            → Stake amounts
│  ├─ PendingProposals          → Block proposals
│  ├─ Certificates              → ASF certificates
│  ├─ FinalityLevels            → Finality per block
│  ├─ RelayFinality             → Relay finality proofs
│  ├─ CrossChainAttestations    → Bridge attestations
│  ├─ RotationRound             → Current rotation round
│  └─ SessionIndex              → Current session
│
├─ Extrinsics
│  ├─ register_collator()       → Join committee
│  ├─ deregister_collator()     → Leave committee
│  ├─ submit_proposal()         → Propose block
│  ├─ submit_vote()             → Vote on block
│  ├─ submit_relay_finality()   → Submit relay proof
│  └─ submit_cross_chain_attestation() → Bridge attestation
│
└─ Hooks
   └─ on_initialize()           → Rotation & session management
```

### 3. Bridge Security Module (`asf-bridge-security`)

```
asf-bridge-security/
│
├─ Core (lib.rs)
│  ├─ BridgeTransfer            → Transfer request
│  ├─ BridgeSecurityProof       → Complete security proof
│  ├─ BridgeRelay               → Cross-chain relay
│  └─ BridgeManager             → Manages all bridges
│
├─ Merkle Proofs (merkle_proof.rs)
│  ├─ MerkleProof               → Inclusion proof
│  └─ MerkleTreeBuilder         → Tree construction
│
├─ Slashing (slash_conditions.rs)
│  ├─ SlashReason               → Misbehavior types
│  ├─ SlashRecord               → Slash events
│  └─ SlashChecker              → Condition checker
│
└─ Economic Security (economic_security.rs)
   ├─ EconomicDeposit           → Collator deposits
   └─ SecurityCalculator        → Security requirements
```

## Consensus Flow

### Block Production and Finalization

```
┌──────────────────────────────────────────────────────────────────┐
│ Block N                                                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│ 1. ROTATION CHECK                                                │
│    ├─ Current block % rotation_period == 0?                      │
│    └─ Yes → Increment rotation round                             │
│                                                                   │
│ 2. COLLATOR SELECTION                                            │
│    ├─ RotationManager.select_collator()                          │
│    ├─ Strategy: Hybrid (stake-weighted + cooldown)               │
│    ├─ Randomness: hash(para_id || block_num || relay_parent)     │
│    └─ Selected: Collator C                                       │
│                                                                   │
│ 3. BLOCK PROPOSAL (by Collator C)                                │
│    ├─ Build block with Cumulus                                   │
│    ├─ Create CollatorProposal                                    │
│    └─ Broadcast to committee                                     │
│                                                                   │
│ 4. PHASE 1: PREPARE                                              │
│    ├─ All collators validate block                               │
│    ├─ Each creates CollatorVote(Prepare)                         │
│    ├─ Leader collects votes                                      │
│    ├─ Aggregates into Certificate(Prepare)                       │
│    └─ Requires: 2/3+ collators (BFT threshold)                   │
│                                                                   │
│ 5. PHASE 2: PRECOMMIT                                            │
│    ├─ Prepare certificate broadcast                              │
│    ├─ Collators vote CollatorVote(PreCommit)                     │
│    ├─ Aggregated into Certificate(PreCommit)                     │
│    └─ Requires: 2/3+ collators                                   │
│                                                                   │
│ 6. PHASE 3: COMMIT                                               │
│    ├─ PreCommit certificate broadcast                            │
│    ├─ Collators vote CollatorVote(Commit)                        │
│    ├─ Aggregated into Certificate(Commit)                        │
│    ├─ Requires: 2/3+ collators                                   │
│    └─ Block state locked                                         │
│                                                                   │
│ 7. PHASE 4: DECIDE                                               │
│    ├─ Commit certificate broadcast                               │
│    ├─ Collators vote CollatorVote(Decide)                        │
│    ├─ Aggregated into Certificate(Decide)                        │
│    ├─ Requires: 2/3+ collators                                   │
│    └─ Block finalized (local ASF finality)                       │
│                                                                   │
│ 8. FINALITY UPDATE                                               │
│    ├─ Count certificates: 4 phases = 4 certificates              │
│    ├─ Local finality: None (0-4 certs)                           │
│    └─ Store in FinalityLevels                                    │
│                                                                   │
│ 9. RELAY CHAIN INCLUSION                                         │
│    ├─ Cumulus submits to relay chain                             │
│    ├─ Wait for relay chain finality                              │
│    └─ Relay finality → Upgrade to Irreversible                   │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

### Finality Progression

```
Timeline: Block N

T=0s     │ Block N proposed
         │
T=3s     │ Phase 1 complete → Prepare certificate
         │ Local finality: None (1 cert)
         │
T=6s     │ Phase 2 complete → PreCommit certificate
         │ Local finality: None (2 certs)
         │
T=9s     │ Phase 3 complete → Commit certificate
         │ Local finality: None (3 certs)
         │
T=12s    │ Phase 4 complete → Decide certificate
         │ Local finality: None (4 certs)
         │
T=30s    │ More certificates accumulate
         │ Local finality: Weak (7 certs)
         │
T=60s    │ Continues accumulating
         │ Local finality: Moderate (15 certs)
         │
T=120s   │ Many certificates
         │ Local finality: Strong (25 certs)
         │
T=600s   │ Relay chain finalizes
         │ Combined finality: Irreversible (relay finalized)
         │
```

## Bridge Transfer Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ Cross-Chain Transfer: BTC-PBC → ETH-PBC                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ SOURCE CHAIN (BTC-PBC)                                          │
│ ──────────────────────                                          │
│                                                                  │
│ 1. User initiates transfer                                      │
│    ├─ Amount: 1 BTC                                             │
│    ├─ Recipient: 0x123... (ETH address)                         │
│    └─ Assets locked on BTC-PBC                                  │
│                                                                  │
│ 2. Transfer included in Block N                                 │
│    ├─ Block hash: H_N                                           │
│    ├─ Merkle tree of transfers built                            │
│    └─ Merkle root in block header                               │
│                                                                  │
│ 3. Wait for Strong finality (20+ certs)                         │
│    └─ ~2-3 minutes                                              │
│                                                                  │
│ 4. Collators create attestations                                │
│    ├─ Each collator observes finality                           │
│    ├─ Creates CrossChainAttestation                             │
│    │  ├─ Source: BTC-PBC                                        │
│    │  ├─ Target: ETH-PBC                                        │
│    │  ├─ Block: H_N                                             │
│    │  ├─ Finality: Strong                                       │
│    │  └─ Signature                                              │
│    └─ Submits to BTC-PBC runtime                                │
│                                                                  │
│ 5. Attestations aggregated                                      │
│    ├─ Collect 2/3+ collator attestations                        │
│    ├─ Verify minimum finality (Strong)                          │
│    ├─ Check total stake >= 2x transfer value                    │
│    └─ Create MultiSigAttestation                                │
│                                                                  │
│ 6. Build security proof                                         │
│    ├─ BridgeSecurityProof                                       │
│    │  ├─ Transfer details                                       │
│    │  ├─ MultiSigAttestation (2/3+ collators)                   │
│    │  ├─ Merkle proof (transfer in block)                       │
│    │  └─ Economic deposits (2x value)                           │
│    └─ Submit to bridge relay                                    │
│                                                                  │
│ TARGET CHAIN (ETH-PBC)                                          │
│ ──────────────────────                                          │
│                                                                  │
│ 7. Receive security proof                                       │
│    ├─ Verify MultiSigAttestation                                │
│    │  ├─ Check signatures                                       │
│    │  ├─ Verify 2/3+ threshold                                  │
│    │  └─ Check stake >= 2x value                                │
│    ├─ Verify Merkle proof                                       │
│    │  ├─ Recompute path                                         │
│    │  └─ Compare with block header                              │
│    └─ Verify economic deposits                                  │
│                                                                  │
│ 8. Challenge period (100 relay blocks)                          │
│    ├─ Anyone can challenge with counter-proof                   │
│    ├─ If challenged: Dispute resolution                         │
│    └─ If unchallenged: Continue                                 │
│                                                                  │
│ 9. Mint wrapped asset                                           │
│    ├─ Challenge period passed                                   │
│    ├─ Proof verified                                            │
│    ├─ Mint 1 wBTC to recipient                                  │
│    └─ Transfer complete                                         │
│                                                                  │
│ Total Time: ~12-15 minutes                                      │
│ ├─ Strong finality: 2-3 min                                     │
│ ├─ Attestation aggregation: 1-2 min                             │
│ └─ Challenge period: 10 min                                     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Security Model

### Byzantine Fault Tolerance

```
Committee: 11 collators

Honest: H = 11 - f
Byzantine: f

Safety requires: H > 2n/3
                 11 - f > 22/3
                 11 - f > 7.33
                 f < 3.67
                 f ≤ 3

Maximum Byzantine: 3 collators
Minimum Honest: 8 collators

Attack success requires: f ≥ 4 collators (36% of committee)
```

### Economic Security

```
Transfer Value: V
Required Security: S = V × 2 (minimum)

Attack Cost:
├─ Collude 2/3+ collators
├─ Each deposits 2x transfer value
├─ If caught: Slashed 10x deposit
└─ Total risk: 2/3 × 11 × 2V × 10 = 146.67V

Attack Gain: V (steal transfer)
Attack Risk: 146.67V
Risk/Reward: 146.67:1

Conclusion: Economically irrational to attack
```

### Cryptographic Security

```
Hash Function: BLAKE2b-256
├─ Pre-image resistance: 2^256
├─ Collision resistance: 2^128
└─ Second pre-image: 2^256

Signature Scheme: SR25519
├─ Schnorr signatures
├─ EdDSA variant
└─ Security: 128-bit

Merkle Trees: Binary tree with BLAKE2b
├─ Proof size: O(log n)
├─ Verification: O(log n)
└─ Forgery: Computationally infeasible
```

## Performance Characteristics

### Latency Analysis

```
Operation                    | Time
─────────────────────────────┼──────────────
Block proposal               | ~100ms
Vote verification            | ~100μs
Vote collection (11 votes)   | ~1.1ms
Certificate aggregation      | ~500μs
Certificate broadcast        | ~200ms
Single phase completion      | ~3s
Full 4-phase consensus       | ~12s
Strong finality (20 certs)   | ~2-3 min
Relay chain finality         | ~10 min
```

### Throughput

```
Metric                       | Value
─────────────────────────────┼───────────────
Blocks per minute            | 10 (6s period)
Votes per block              | 11 (committee)
Certificates per block       | 4 (phases)
Network messages per block   | ~44 (11×4)
Data per block               | ~5KB
Data per hour                | ~3MB
```

### Storage Growth

```
Component                    | Size/Block | Size/Month
─────────────────────────────┼────────────┼────────────
Certificates (4)             | 2KB        | 86GB
Finality records             | 200B       | 8.6GB
Attestations (avg 1/block)   | 1KB        | 43GB
Total                        | 3.2KB      | 138GB

With pruning (keep 10,000)   | 3.2KB      | 32MB
```

## Deployment Configuration

### BTC-PBC (High Security)

```toml
[asf_collator]
para_id = 2000
min_collators = 11
max_collators = 21
min_stake = "5000000000000000000000000"  # 5M ETR
rotation_period = 12  # Slower for stability
session_length = 1200  # Longer sessions
```

### ETH-PBC (High Security)

```toml
[asf_collator]
para_id = 2001
min_collators = 11
max_collators = 21
min_stake = "5000000000000000000000000"  # 5M ETR
rotation_period = 12
session_length = 1200
```

### SOL-PBC (Medium Security)

```toml
[asf_collator]
para_id = 2002
min_collators = 9
max_collators = 15
min_stake = "2000000000000000000000000"  # 2M ETR
rotation_period = 6  # Standard
session_length = 600
```

### TRX-PBC (Lower Security)

```toml
[asf_collator]
para_id = 2005
min_collators = 7
max_collators = 11
min_stake = "1000000000000000000000000"  # 1M ETR
rotation_period = 6
session_length = 600
```

## Integration Points

### With Cumulus

```rust
// Collator runs both cumulus and ASF
async fn start_collator(
    parachain_config: Configuration,
    relay_chain_config: Configuration,
) -> Result<(), Error> {
    // 1. Start cumulus services
    let cumulus_task = cumulus_client_service::start_collator(
        parachain_config,
        relay_chain_config,
    ).await?;

    // 2. Start ASF consensus service
    let asf_task = asf_collator_service::start(
        client.clone(),
        relay_chain_interface.clone(),
    ).await?;

    // 3. Run both in parallel
    futures::future::join(cumulus_task, asf_task).await;

    Ok(())
}
```

### With Runtime

```rust
// Runtime calls pallet
impl pallet_bridge::Config for Runtime {
    // ...

    // Check finality before releasing bridge transfers
    fn check_finality(block_hash: Hash) -> bool {
        let finality = pallet_asf_collator::Pallet::<Runtime>::finality_level(block_hash);
        finality >= CollatorFinalityLevel::Strong
    }
}
```

### With Bridge

```rust
// Bridge validates with ASF proof
impl BridgeValidator {
    fn validate_transfer(proof: BridgeSecurityProof) -> Result<(), Error> {
        // 1. Verify ASF attestation
        proof.source_attestation.verify()?;

        // 2. Verify merkle proof
        proof.merkle_proof.verify(
            proof.transfer.transfer_hash(),
            proof.transfer.source_block,
        )?;

        // 3. Verify economic security
        ensure!(
            proof.total_security >= proof.transfer.amount * 2,
            Error::InsufficientSecurity
        );

        Ok(())
    }
}
```

---

**Architecture Version**: 1.0
**Last Updated**: 2025-11-15
**Status**: Production Ready
