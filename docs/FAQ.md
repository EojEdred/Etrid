# Frequently Asked Questions (FAQ)

## General Questions

### What is ËTRID?

ËTRID is a next-generation blockchain platform that connects 13 major blockchains (Bitcoin, Ethereum, Solana, etc.) through specialized Partition Burst Chains (PBCs). It combines the security of a main relay chain (Primearc Core) with the scalability of Layer 2 solutions (Lightning Bloc) to achieve **171,000+ TPS** total network capacity.

---

### How fast are Ëtrid transactions?

It depends on which layer you use:

- **Lightning Bloc (Layer 3)**: ~100ms (instant, off-chain)
- **PBCs (Layer 2)**: ~2 seconds (optimistic finality)
- **Primearc Core (Layer 1)**: ~10 minutes (ASF checkpoint finality)

For everyday payments, Lightning Bloc provides instant confirmation. For final settlement on the main chain, it takes approximately 61 minutes from Lightning → PBC → Primearc Core.

---

### What's the difference between Primearc Core, PBCs, and Lightning Bloc?

**Primearc Core (Layer 1 - Relay Chain)**
- The main chain that coordinates everything
- 1,000 TPS capacity
- ASF checkpoint finality (BFT security with 15+/21 validators)
- Stores only checkpoint data (32 bytes per PBC)
- Run by Directors 1-5

**Partition Burst Chains (Layer 2 - Sidechains)**
- 14 specialized chains for different asset types
- 5,000 TPS per chain = 70,000 TPS total
- Checkpoint to Primearc Core every 100 blocks (~10 minutes)
- Run by Validators 6-21

**Lightning Bloc (Layer 3 - Payment Channels)**
- Off-chain payment network
- 100,000+ TPS capacity
- Zero fees (off-chain)
- Instant finality for payments
- Batch settlements to PBCs every 5 minutes

**Flame Analogy:**
- 🔵 **Primearc Core** = Blue flame core (2,600°F) - Hottest, most secure, maximum finality
- 🔶 **PBCs** = Orange middle flame (1,800-2,100°F) - High energy, burst throughput
- ⚡ **Lightning** = Yellow outer flame (1,200-1,800°F) - Fastest reach, instant transactions

---

### Why is it called the "Flame Architecture"?

Like a natural flame, ËTRID has three distinct layers that get hotter toward the center:

🔵 **Blue Core (Primearc Core):** The hottest part of a flame (2,600°F), representing maximum security and finality. Just as blue flames indicate complete combustion, Primearc Core provides absolute consensus via ASF (Asynchronous Safety Finality). This is where the most critical operations happen - governance, validator coordination, and final checkpoints.

🔶 **Orange Ring (PBCs):** The middle flame (1,800-2,100°F), where most of the throughput happens. PBCs "burst" with activity, processing 70,000 TPS across 13 specialized chains. Like the orange zone of a flame that produces the most visible light and heat, PBCs handle the bulk of network transactions.

⚡ **Yellow Outer (Lightning):** The outer flame (1,200-1,800°F), fastest and most energetic. Lightning Bloc enables instant transactions with zero friction, reaching outward to users like the outermost part of a flame. While technically "cooler," it's the fastest layer with 100,000+ TPS and 100ms latency.

**Physical Accuracy:**
- Real flames ARE hottest at the blue core
- Blue indicates complete combustion (highest temperature)
- Orange/yellow indicates partial combustion (lower temperature)
- Our architecture mirrors this: Highest security (blue) → High throughput (orange) → Fastest speed (yellow)

**Total Heat:** 171,000+ TPS network capacity 🔥

[View Interactive Flame Visualization →](architecture-viz)

---

### How do I know which layer to use?

**Use Lightning Bloc when:**
- Making small, frequent payments
- You want instant confirmation
- You want zero fees
- You trust your counterparty (or watchtowers)

**Use PBCs when:**
- Transferring larger amounts
- You need on-chain proof within minutes
- You're interacting with smart contracts
- You're bridging assets from other chains

**Use Primearc Core when:**
- You need absolute, irreversible finality
- You're participating in governance
- You're staking/unstaking ÉTR
- You're submitting important state checkpoints

---

## Technical Questions

### How does the three-layer architecture work?

Each layer serves a specific purpose:

1. **Lightning Bloc** batches thousands of off-chain transactions
2. **PBCs** receive batch settlements and execute smart contracts
3. **Primearc Core** stores PBC checkpoints for final verification

**Data Flow:**
```
Lightning Transaction (Layer 3)
    ↓ (5 minutes or 1,000 transactions)
Batch Settlement to PBC (Layer 2)
    ↓ (100 blocks = ~10 minutes)
Checkpoint to Primearc Core (Layer 1)
    ↓ (100 blocks = ~10 minutes)
ASF Checkpoint Finality ✅
```

Total time: **~61 minutes** from Lightning to final Primearc Core settlement.

---

### What happens if my Lightning channel counterparty goes offline?

You can always exit:

1. **Submit emergency withdrawal request** to the PBC
2. **24-hour timeout period** begins
3. If counterparty is still unresponsive, **force execute withdrawal**
4. You get your funds back using the last known checkpoint

**Security:** Watchtowers monitor channels 24/7 and will automatically challenge fraudulent closure attempts.

---

### How long until my transaction is final?

It depends on what "final" means:

| Finality Type | Time | Security Level |
|---------------|------|----------------|
| **Instant (Lightning)** | ~100ms | Cryptographic (off-chain) |
| **Optimistic (PBC)** | ~2 seconds | Economic (fraud proofs) |
| **Economic (Checkpoint)** | ~10 minutes | Multi-validator consensus |
| **Absolute (ASF)** | ~61 minutes | Byzantine fault tolerance (15+/21) |

For everyday transactions, **optimistic finality (~2 seconds)** is sufficient. For large settlements, wait for **absolute finality (~61 minutes)**.

---

### What are PBC checkpoints?

Every 100 blocks (~10 minutes), each PBC submits a **checkpoint** to Primearc Core:

**Checkpoint contains:**
- Merkle root of entire PBC state (32 bytes)
- Block number
- Total supply snapshot
- Timestamp

**Why this matters:**
- Primearc Core doesn't store full PBC data (too large)
- Directors can verify any PBC state using Merkle proofs
- Fraud proofs protect against invalid checkpoints
- 7-day challenge period for disputes

**Storage efficiency:** Instead of storing terabytes of PBC data, Primearc Core stores only 32 bytes per PBC per checkpoint.

---

### How do validators participate in the network?

There are two types of validators:

**Primearc Nodes (Directors 1-5)**
- Run Primearc Core validators
- Verify PBC checkpoints
- Coordinate governance
- Minimum stake: 128 ÉTR
- Hardware: 4-core CPU, 16 GB RAM, 500 GB SSD

**Validity Nodes (Validators 6-21)**
- Run PBC validators
- Calculate state Merkle roots
- Submit checkpoints to Primearc Core
- Process Lightning settlements
- Minimum stake: 64 ÉTR
- Hardware: 2-core CPU, 8 GB RAM, 100 GB SSD

**Key principle:** Only Decentralized Directors can be Primearc Nodes. This prevents centralization and ensures governance accountability.

---

### What is a Merkle proof?

A Merkle proof lets you verify data without downloading the entire dataset.

**Example:**
```
User asks: "What's Alice's balance on PBC-EDSC?"

[1] Get Primearc Core checkpoint
    State root: 0xabc123...

[2] Request proof from PBC node
    Merkle path: [hash1, hash2, ..., hash32]

[3] Verify locally
    hash(Alice's balance + merkle_path) == 0xabc123 ✅

[4] Result
    Alice has 9,900 ÉTR (verified!)
```

**Proof size:** ~1 KB (32 hashes × 32 bytes)

**Benefit:** You don't need to download gigabytes of PBC data to verify a single balance.

---

## Governance & Staking

### How do I stake ÉTR?

1. **Get ÉTR tokens** from an exchange or bridge
2. **Choose a validator** from the leaderboard
3. **Stake your tokens** (minimum 32 ÉTR recommended)
4. **Earn rewards** (8-15% APY)

[Full staking guide →](staking.md)

---

### What is Consensus Day?

An annual democratic event (December 1st each year) where ÉTR holders vote on:

- **Inflation rate** for the next year
- **Network upgrades** (protocol changes)
- **Treasury allocation** (funding proposals)
- **Community proposals** (ecosystem initiatives)

**Voting power** = Your staked ÉTR balance

[Learn about governance →](specifications/governance-appendix.md)

---

## Security & Safety

### Is ËTRID secure?

Yes, with multiple layers of security:

**Layer 1 (Primearc Core):**
- ASF checkpoint finality (Byzantine fault tolerant)
- Requires 15+/21 honest validators (>2/3 + 1)
- Audited codebase

**Layer 2 (PBCs):**
- Fraud proofs with 7-day challenge period
- Economic incentives (slashing for fraud)
- Requires 5/8 honest validators per PBC

**Layer 3 (Lightning Bloc):**
- Cryptographic signatures
- Watchtower network
- 24-hour emergency exit timeout
- 7-day dispute window

**Economic security:** To attack ËTRID, an adversary would need to:
1. Control 15+/21 Primearc Core validators (very expensive)
2. Control 5/8 of PBC validators (slashing risk)
3. Evade watchtower network (cryptographically infeasible)

---

### What if a PBC stops submitting checkpoints?

Primearc Core monitors checkpoint liveness:

```
No checkpoint for 512 blocks (102 minutes)
    ↓
Alert: PBC-EDSC unresponsive ⚠️
    ↓
Action: Freeze PBC (stop new transactions)
    ↓
Users: Emergency withdrawals using last checkpoint
```

Your funds are always recoverable using the last known checkpoint stored on Primearc Core.

---

### What are fraud proofs?

Fraud proofs protect against invalid checkpoints:

```
[1] Malicious collator submits fake checkpoint
    ↓
[2] Honest validator detects invalid state root
    ↓
[3] Submit fraud proof with Merkle evidence
    ↓
[4] Primearc Core verifies fraud proof
    ↓
[5] Slash malicious collator (10,000 ÉTR penalty)
    ↓
[6] Reward challenger (1,000 ÉTR bounty) ✅
```

**Challenge period:** 7 days

**Incentive:** Anyone can earn bounties by catching fraud.

---

## Performance & Scalability

### How does ËTRID achieve 171,000 TPS?

By distributing work across three optimized layers:

| Layer | Capacity | Method |
|-------|----------|--------|
| **Layer 1** | 1,000 TPS | Primearc Core main chain |
| **Layer 2** | 70,000 TPS | 14 PBCs × 5,000 TPS each |
| **Layer 3** | 100,000+ TPS | Off-chain Lightning channels |
| **Total** | **171,000+ TPS** | Combined capacity |

**Comparison:**
- Bitcoin: ~7 TPS
- Ethereum L1: ~15 TPS
- Ethereum L2: ~4,000 TPS (optimistic rollups)
- **ËTRID: 171,000+ TPS**

---

### Why not just use one layer?

Different use cases need different trade-offs:

**High Security (Layer 1):**
- Slow but absolutely final
- Expensive (higher fees)
- Best for: Governance, large settlements

**Medium Speed (Layer 2):**
- Fast but needs checkpoints
- Moderate fees
- Best for: Smart contracts, bridging

**Instant (Layer 3):**
- Instant but off-chain
- Zero fees
- Best for: Payments, micropayments

**Multi-layer = Best of all worlds**

---

### Can ËTRID scale beyond 171,000 TPS?

Yes, by adding more PBCs:

- **Current:** 14 PBCs × 5,000 TPS = 70,000 TPS
- **Future:** 50 PBCs × 5,000 TPS = 250,000 TPS

Each new PBC adds ~5,000 TPS to the network. Primearc Core can support hundreds of PBCs since it only stores 32-byte checkpoints.

---

## Cross-Chain & Bridges

### How do I bridge assets from Ethereum?

1. **Visit the bridge UI** at https://bridge.etrid.org
2. **Connect your MetaMask wallet**
3. **Select ETH → PBC-ETH** bridge
4. **Approve token spending** (one-time setup)
5. **Submit bridge transaction**
6. **Wait 12 confirmations** (~3 minutes)
7. **Receive wrapped ETH on PBC-ETH** ✅

[Full bridge guide →](bridges.md)

---

### Are bridges secure?

ËTRID bridges use multi-signature custodians:

- **5-of-8 multisig** required to release funds
- **Validators 6-13** control keys
- **Threshold signatures** prevent single points of failure
- **Fraud proofs** challenge invalid unlocks

**Bridge reserves are always monitored** and automatically trigger emergency checkpoints if balance mismatches are detected.

---

## Getting Started

### How do I get started with ËTRID?

1. **[Get a wallet](wallets.md)** - Web, mobile, or desktop
2. **[Acquire ÉTR tokens](getting-etr.md)** - Exchange or bridge
3. **[Explore the ecosystem](/)** - DApps, staking, bridges
4. **[Join the community](COMMUNITY_GUIDE.md)** - Discord, forums, governance

---

### Where can I ask more questions?

- **Documentation:** https://docs.etrid.org
- **Discord:** https://discord.gg/etrid
- **Forum:** https://forum.etrid.org
- **GitHub:** https://github.com/etrid/etrid
- **Twitter:** @etrid_protocol

---

## Developer Questions

### How do I build on ËTRID?

Start with our developer guide:

[→ Developer Guide](DEVELOPER_GUIDE.md)

Key resources:
- **API Reference:** [API_REFERENCE.md](API_REFERENCE.md)
- **SDKs:** Rust, JavaScript, Python, Swift
- **Smart Contracts:** Solidity-compatible (EVM) on PBC-ETH
- **Testing:** [testing.md](testing.md)

---

### Which PBC should I deploy my DApp on?

Choose based on your target audience:

- **PBC-ETH**: Ethereum DApp ports (Solidity)
- **PBC-BTC**: Bitcoin-related apps
- **PBC-EDSC**: Native ÉTR ecosystem apps
- **PBC-SOL**: Solana-like performance needs

All PBCs support Lightning Bloc Layer 2 for instant payments.

---

## Still have questions?

Visit our [Community Guide](COMMUNITY_GUIDE.md) to connect with the ËTRID team and community!
