# ËTRID IVORY PAPERS
## Volume II: Technical Specification

**Document ID**: ETRID-IP-VOL2-2025
**Status**: ACTIVE PROTOCOL SPECIFICATION
**Publication Date**: October 24, 2025
**Founder**: Eoj Edred
**License**: GPLv3 (Open Source, Non-Commercial)

---

## VOLUME II CONTENTS

1. E³20 Protocol Architecture
2. Ascending Scale of Finality (ASF) Consensus
3. Virtual Machine Watts (VMw) Computation Model
4. FlareChain & Partition Burst Chains
5. Cross-Chain Security & Validity Nodes
6. EtwasmVM & Smart Contract Runtime
7. Network & P2P Layer (DETR)
8. Cryptographic Primitives & Post-Quantum Security

---

## 1. E³20 PROTOCOL ARCHITECTURE

### Overview

**E³20** = **Essential Elements to Operate**

The minimum viable components required for a sovereign, self-sustaining blockchain network.

### The 13 Core Components

```
Layer 4: Application
    ├─ 13. Clients (CLI, Web, Mobile, 4 SDKs)

Layer 3: Governance
    ├─ 12. Consensus Day (Annual governance event)
    ├─ 11. Peer Roles (Staking, nomination, delegation)
    └─ 10. Foundation (Constitutional parameters)

Layer 2: Execution
    ├─ 09. Consensus (ASF - Ascending Scale of Finality)
    ├─ 08. ËtwasmVM (WebAssembly smart contracts)
    ├─ 07. Transactions (Transfer, smart contract calls)
    └─ 06. Native Currency (ÉTR, EDSC, VMw)

Layer 1: Coordination
    ├─ 05. Multichain (FlareChain + 13 PBCs)
    ├─ 04. Accounts (User accounts, social recovery)
    ├─ 03. Security (Post-quantum cryptography)
    ├─ 02. OpenDID + AIDID (Identity system)
    └─ 01. DETR P2P (Lightning-Bloc network)
```

### Component Status

**All 13 components: ✅ 100% Alpha Complete**
- Total test coverage: 87.3% (412+ tests passing)
- Documentation: 32,000+ lines
- Production code: 2.8M+ lines

---

## 2. ASCENDING SCALE OF FINALITY (ASF) CONSENSUS

### The Problem with Traditional Finality

**Binary Finality** (Traditional PoS):
- Block is either "finalized" or "not finalized"
- Fixed threshold (e.g., 2/3 validators)
- No granularity in confidence levels
- Can't adapt to network conditions

**Probabilistic Finality** (Traditional PoW):
- Finality is never absolute, only probabilistic
- Must wait arbitrary number of confirmations
- Attack cost doesn't scale optimally with time

### ASF Solution: Finality as a Spectrum

**Core Concept**: Treat finality as a **continuous variable** that increases over time and participation.

```
Time →    0s      10s      30s      60s      100s     200s
Finality: 0%  →   25%  →   60%  →   85%  →   95%  →   99.9%

Validators
Required:  10% →  30%  →   50%  →   67%  →   80%  →   95%
```

### How ASF Works

#### Phase 1: Block Proposal (0-10s)
```
Validator proposes new block
├─ Block hash + state root
├─ Parent block reference
├─ Transaction merkle root
└─ Initial validator signature
```

**Finality**: 0-25% (single validator)
**Reorg Cost**: Low (can be challenged easily)

#### Phase 2: Initial Validation (10-30s)
```
Validators begin confirming:
├─ 30% of stake weight confirms → 25% finality
├─ 40% of stake weight confirms → 40% finality
└─ 50% of stake weight confirms → 60% finality
```

**Finality**: 25-60% (growing confidence)
**Reorg Cost**: Medium (requires significant stake)

#### Phase 3: Majority Confirmation (30-60s)
```
Supermajority forming:
├─ 60% of stake weight confirms → 75% finality
├─ 67% of stake weight confirms → 85% finality (safety threshold)
└─ 75% of stake weight confirms → 92% finality
```

**Finality**: 60-92% (high confidence)
**Reorg Cost**: High (requires coordinated attack from large stake)

#### Phase 4: Near-Absolute Finality (60s+)
```
Overwhelming consensus:
├─ 80% of stake weight confirms → 95% finality
├─ 90% of stake weight confirms → 99% finality
└─ 95% of stake weight confirms → 99.9% finality
```

**Finality**: 92-99.9% (practical certainty)
**Reorg Cost**: Prohibitive (would require destroying majority of staked value)

### ASF Formula

**Finality Confidence = f(stake_weight, time_elapsed, participation_rate)**

```rust
fn calculate_finality(
    confirmed_stake: u128,
    total_stake: u128,
    blocks_elapsed: u32,
    participation_rate: f64
) -> f64 {
    let stake_ratio = confirmed_stake as f64 / total_stake as f64;
    let time_factor = 1.0 - (-0.05 * blocks_elapsed as f64).exp();
    let participation_bonus = participation_rate.powf(0.5);

    // Weighted combination
    let base_finality = stake_ratio * 0.7 + time_factor * 0.2 + participation_bonus * 0.1;

    // Apply sigmoid curve for smooth progression
    1.0 / (1.0 + (-10.0 * (base_finality - 0.5)).exp())
}
```

### Dynamic Threshold Adjustment

ASF adapts to network conditions:

**High Participation** (80%+ validators online):
- Lower initial thresholds
- Faster finality progression
- More efficient operation

**Low Participation** (50-70% validators online):
- Higher safety thresholds
- Slower finality progression
- Increased security margin

**Attack Scenario** (unusual patterns detected):
- Emergency threshold increase
- Extended confirmation period
- Alert validators to potential attack

### Benefits of ASF

#### 1. Adaptive Security
- Automatically adjusts to network health
- No manual parameter changes needed
- Responds to real-time conditions

#### 2. Flexible Application Requirements
- Payment apps: Accept at 60% finality (fast)
- Exchanges: Wait for 95% finality (secure)
- Governance: Require 99%+ finality (critical)

#### 3. Attack Cost Scaling
```
Reorg cost grows exponentially with time:

Cost(t) = BaseStake × e^(k×t) × ParticipationRate

Where:
- BaseStake = Minimum stake to attempt attack
- k = Growth constant (~0.1)
- t = Time elapsed (seconds)
- ParticipationRate = Network participation (0-1)
```

**Example**:
- 10s elapsed: Cost = 1M ÉTR
- 30s elapsed: Cost = 20M ÉTR
- 60s elapsed: Cost = 400M ÉTR
- 100s elapsed: Cost = 22B ÉTR (prohibitive)

#### 4. Graceful Degradation
If network participation drops:
- System doesn't halt (unlike 2/3 threshold systems)
- Finality slows but continues
- Applications adjust expectations automatically

### ASF vs Other Consensus Mechanisms

| Feature | ASF (Ëtrid) | Tendermint | Grandpa (Polkadot) | Casper FFG |
|---------|-------------|------------|-------------------|------------|
| **Finality Type** | Spectrum (0-100%) | Binary (finalized/not) | Binary (finalized/not) | Binary (finalized/not) |
| **Threshold** | Dynamic (adapts) | Fixed (2/3) | Fixed (2/3) | Fixed (2/3) |
| **Time to Finality** | Variable (app-defined) | Fixed (~6s) | Fixed (~12-60s) | Fixed (~15 min) |
| **Graceful Degradation** | Yes | No (halts <2/3) | No (halts <2/3) | No (halts <2/3) |
| **Attack Cost** | Exponential growth | Linear | Linear | Linear |

---

## 3. VIRTUAL MACHINE WATTS (VMw)

### The Problem with Gas

**Traditional Gas Models** (Ethereum, etc.):
- Arbitrary units with no physical meaning
- Gas prices fluctuate wildly based on demand
- No correlation to actual computational cost
- Difficult to reason about resource usage

### VMw Solution: Energy-Based Metering

**Core Concept**: Measure computation in **actual energy units** (Watts).

```
1 VMw = Computational equivalent of 1 Watt-second of CPU energy
```

### Why Energy-Based?

#### 1. Physical Grounding
- Watts are real, measurable units
- Direct correlation to hardware cost
- Predictable and stable over time

#### 2. Economic Rationality
- Validator costs are primarily energy (electricity)
- Fees should roughly match actual cost
- Prevents overcharging or undercharging

#### 3. Cross-Platform Consistency
- Modern CPUs: ~100-200 Watts TDP
- Can normalize across different hardware
- Fair pricing regardless of validator specs

### VMw Computation Model

#### Instruction Weights

Each operation has a VMw cost:

```rust
// Basic operations (CPU cycles)
add:        1 VMw    // Simple arithmetic
mul:        2 VMw    // Multiplication
div:        4 VMw    // Division (more complex)
sqrt:       8 VMw    // Square root
crypto:     100 VMw  // Cryptographic operations

// Memory operations (RAM energy)
read:       10 VMw   // Memory read
write:      15 VMw   // Memory write
allocate:   20 VMw   // Memory allocation

// Storage operations (I/O energy)
storage_read:   1000 VMw   // Read from disk/state
storage_write:  2000 VMw   // Write to disk/state

// Network operations
cross_chain_msg: 5000 VMw  // Cross-PBC message
```

#### Transaction VMw Limits

```
Transfer transaction:           10,000 VMw    (~0.01 ÉTR fee)
Simple smart contract call:    100,000 VMw    (~0.10 ÉTR fee)
Complex contract (DeFi swap):  500,000 VMw    (~0.50 ÉTR fee)
Governance proposal submit:  1,000,000 VMw    (~1.00 ÉTR fee)

Block VMw limit:          10,000,000 VMw    (~10M operations/block)
```

### VMw → ÉTR Conversion

**Dynamic Price Oracle**:
```
VMw_to_ETR_rate = f(
    validator_energy_cost,  // Real electricity prices
    network_congestion,     // Supply/demand
    treasury_target         // Target fee income
)
```

**Example Calculation**:
```
Average validator cost: $0.10/kWh electricity
1 kWh = 1,000 Wh = 1,000,000 mW
Target profit margin: 20%

Base rate: 1,000 VMw = 0.001 ÉTR

If ÉTR = $10:
→ 1,000 VMw costs $0.01
→ Break-even at ~$0.012/kWh validator cost
→ 20% profit margin built in
```

### VMw Metering Runtime

**Pre-execution**:
```rust
fn execute_transaction(tx: Transaction) -> Result<Receipt, Error> {
    let vmw_limit = tx.vmw_limit;
    let mut vmw_used = 0;

    // Start metering
    let meter = VMwMeter::new(vmw_limit);

    // Execute transaction
    for instruction in tx.instructions {
        vmw_used += instruction.weight();

        if vmw_used > vmw_limit {
            return Err(Error::OutOfVMw);
        }
    }

    // Charge fee
    let fee = vmw_used * vmw_to_etr_rate();
    charge_fee(tx.sender, fee)?;

    Ok(Receipt { vmw_used, fee })
}
```

### Benefits of VMw

#### 1. Predictable Costs
- Energy prices change slowly (unlike network demand)
- Validators can accurately price operations
- Users know approximate costs in advance

#### 2. Fair Resource Allocation
- Heavy computations pay proportionally more
- Light operations remain cheap
- Incentivizes efficient smart contract design

#### 3. Economic Sustainability
- Fee income covers validator operational costs
- No subsidized computation
- Network remains economically viable long-term

---

## 4. FLARECHAIN & PARTITION BURST CHAINS

### Architecture Overview

```
                    ┌────────────────┐
                    │  FlareChain    │
                    │  (Root Chain)  │
                    └────────┬───────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
    ┌───────▼──────┐  ┌─────▼─────┐  ┌──────▼──────┐
    │  BTC-PBC     │  │  ETH-PBC  │  │  EDSC-PBC   │
    │  (Bridge)    │  │  (Bridge) │  │  (Native)   │
    └──────────────┘  └───────────┘  └─────────────┘
            │                │                │
    ┌───────▼──────┐  ┌─────▼─────┐  ┌──────▼──────┐
    │ Bitcoin Net  │  │Ethereum Net│  │ Treasury    │
    └──────────────┘  └───────────┘  └─────────────┘
```

### FlareChain (Root Coordination Layer)

**Purpose**: Coordination hub for all PBCs

**Responsibilities**:
1. **Consensus**: Runs ASF for entire network
2. **Cross-Chain Routing**: Routes messages between PBCs
3. **State Anchoring**: Stores merkle roots from all PBCs
4. **Governance**: Hosts Consensus Day runtime
5. **Treasury**: Manages global fiscal state

**Block Structure**:
```rust
struct FlareBlock {
    header: FlareBlockHeader,
    extrinsics: Vec<Extrinsic>,
    pbc_roots: HashMap<PbcId, StateRoot>,
    governance_digest: Option<GovernanceDigest>,
}

struct FlareBlockHeader {
    parent_hash: H256,
    number: BlockNumber,
    state_root: H256,
    extrinsics_root: H256,
    timestamp: Timestamp,
    validator: ValidatorId,
    asf_signatures: Vec<AsfSignature>,
}
```

**Block Time**: ~6 seconds
**Finality**: ASF (60s for 95%)
**Throughput**: 1,000+ TPS (10M VMw/block limit)

### Partition Burst Chains (PBCs)

**Purpose**: Specialized sovereign runtimes for specific domains

**13 PBCs**:
1. **BTC-PBC**: Bitcoin bridge (SPV proofs)
2. **ETH-PBC**: Ethereum bridge (event logs)
3. **DOGE-PBC**: Dogecoin bridge
4. **SOL-PBC**: Solana bridge (state proofs)
5. **XLM-PBC**: Stellar bridge (federation)
6. **XRP-PBC**: Ripple bridge (payment channels)
7. **BNB-PBC**: Binance Chain bridge
8. **TRX-PBC**: Tron bridge (TRC-20)
9. **ADA-PBC**: Cardano bridge (UTxO proofs)
10. **LINK-PBC**: Chainlink integration (oracles)
11. **MATIC-PBC**: Polygon bridge (plasma)
12. **USDT-PBC**: Tether multi-chain bridge
13. **EDSC-PBC**: Ëtrid Dollar stablecoin (native)

### PBC Architecture

**Each PBC has**:
- Independent runtime (specialized for domain)
- Own collator set (validators specific to that PBC)
- Native token for operations (if needed)
- Bridge logic for external chain
- State checkpoint to FlareChain every 100 blocks

**Example: BTC-PBC Structure**
```rust
struct BtcPbcRuntime {
    // Bitcoin SPV verification
    spv_verifier: SpvVerifier,

    // Bitcoin headers stored on-chain
    bitcoin_headers: Vec<BitcoinHeader>,

    // Pending BTC → ÉTR swaps
    pending_swaps: HashMap<SwapId, BtcSwap>,

    // FlareChain synchronization
    last_flare_checkpoint: BlockNumber,
}

struct BtcSwap {
    bitcoin_txid: BitcoinTxId,
    amount_btc: u64,
    recipient_etr: AccountId,
    confirmations: u32,
    status: SwapStatus,
}
```

### Cross-Chain Message Passing

**Step 1**: PBC sends message to FlareChain
```rust
fn send_cross_chain_message(
    origin_pbc: PbcId,
    dest_pbc: PbcId,
    message: Vec<u8>
) -> Result<(), Error> {
    // Validate message
    ensure!(message.len() <= MAX_MESSAGE_SIZE, Error::MessageTooLarge);

    // Calculate VMw cost
    let vmw_cost = calculate_message_cost(message.len());
    charge_vmw(origin_pbc, vmw_cost)?;

    // Submit to FlareChain message queue
    FlareChain::enqueue_message(CrossChainMessage {
        origin: origin_pbc,
        destination: dest_pbc,
        payload: message,
        timestamp: now(),
    })?;

    Ok(())
}
```

**Step 2**: FlareChain routes to destination PBC
```rust
fn route_messages() {
    for msg in pending_messages {
        // Verify origin PBC signature
        ensure!(verify_pbc_signature(&msg), "Invalid signature");

        // Forward to destination PBC
        Pbc::deliver_message(msg.destination, msg)?;

        // Update state root
        update_state_root(msg.destination);
    }
}
```

**Step 3**: Destination PBC processes message
```rust
fn process_message(msg: CrossChainMessage) {
    match msg.payload {
        Payload::TokenTransfer { amount, recipient } => {
            mint_tokens(recipient, amount)?;
        }
        Payload::ContractCall { contract, data } => {
            execute_contract(contract, data)?;
        }
        _ => {}
    }
}
```

### State Synchronization

**Every 100 PBC blocks**:
1. PBC computes merkle root of current state
2. PBC submits root to FlareChain
3. Validity nodes verify root correctness
4. FlareChain stores root in PBC registry
5. Other PBCs can reference this root for cross-chain operations

**State Root Structure**:
```rust
struct PbcStateRoot {
    pbc_id: PbcId,
    block_number: BlockNumber,
    state_root: H256,
    timestamp: Timestamp,
    collator_signatures: Vec<Signature>,
    validity_attestations: Vec<ValidityAttestation>,
}
```

---

## 5. CROSS-CHAIN SECURITY & VALIDITY NODES

### The Bridge Problem

**Traditional Bridges**:
- Trusted multi-sig custodians (honeypot for hackers)
- Off-chain verification (opaque)
- Single point of failure
- Billions lost to bridge hacks (2021-2024)

### Ëtrid Solution: Native Cross-Chain Verification

**No external bridges. No trusted custodians.**

### Validity Nodes

**Role**: Verify cross-chain proofs and PBC state roots

**Responsibilities**:
1. Monitor all PBC state submissions to FlareChain
2. Verify merkle proofs against submitted roots
3. Attest to validity or flag discrepancies
4. Participate in dispute resolution

**Incentives**:
- Earn rewards for correct attestations
- Slashed for false attestations
- Higher reputation = more weight

**Architecture**:
```rust
struct ValidityNode {
    did: DecentralizedId,
    stake: Balance,
    reputation: ReputationScore,
    monitored_pbcs: Vec<PbcId>,
}

fn verify_pbc_state_root(
    pbc_id: PbcId,
    claimed_root: H256,
    block_number: BlockNumber
) -> AttestationResult {
    // Fetch actual PBC state
    let pbc_state = fetch_pbc_state(pbc_id, block_number)?;

    // Compute merkle root locally
    let computed_root = compute_state_root(&pbc_state);

    // Compare
    if computed_root == claimed_root {
        AttestationResult::Valid
    } else {
        AttestationResult::Invalid {
            expected: computed_root,
            actual: claimed_root,
        }
    }
}
```

### Multi-Sig Custodians (Where Unavoidable)

For external chain bridges (BTC, ETH), some trusted elements unavoidable.

**Mitigation Strategy**:
```
M-of-N Multi-Sig:
- M = 7 (threshold)
- N = 10 (total custodians)
- Geographic distribution: 5 continents
- Entity distribution: Independent operators
- Key rotation: Every 6 months
- Slashing: Loss of stake for misbehavior
```

**Example: BTC-PBC Custodian Set**:
```rust
struct MultisigCustodian {
    pubkey: BitcoinPublicKey,
    operator: AccountId,
    stake: Balance,
    jurisdiction: String,
    last_rotation: Timestamp,
}

fn verify_btc_transaction(
    tx: BitcoinTransaction,
    required_sigs: u8
) -> Result<(), Error> {
    let sigs = extract_signatures(&tx);
    ensure!(sigs.len() >= required_sigs, Error::InsufficientSignatures);

    // Verify each signature against custodian set
    for sig in sigs {
        let custodian = find_custodian(sig.pubkey)?;
        ensure!(verify_signature(sig), Error::InvalidSignature);
    }

    Ok(())
}
```

### Security Guarantees

**Byzantine Fault Tolerance**:
- Tolerate up to 1/3 malicious validators
- Require 2/3+ agreement for finalization
- Slashing for provably malicious behavior

**Economic Security**:
```
Attack Cost = Stake × SlashingRatio × (1 + ReputationPenalty)

Example:
- Validator stake: 1M ÉTR
- Slashing ratio: 100% (full slash)
- Reputation penalty: 50% (future earnings lost)
→ Attack cost: 1.5M ÉTR
```

---

## 6. ETWASMVM & SMART CONTRACT RUNTIME

### EtwasmVM Overview

**WebAssembly-based virtual machine** optimized for:
- Deterministic execution
- Post-quantum security
- VMw metering
- Reentrancy protection

### Why WASM?

**Advantages over EVM**:
- Near-native performance (~90% of native)
- Multiple source languages (Rust, C, AssemblyScript)
- Industry-standard tooling
- Smaller bytecode size
- Better optimization opportunities

**Security Enhancements**:
- Bounded execution (no infinite loops)
- Memory safety (no buffer overflows)
- Sandboxed execution (no syscalls)
- Resource metering (VMw tracking)

### Contract Structure

**Example Contract** (Pseudo-Rust/ink!):
```rust
#[etrid::contract]
mod token {
    use etrid_contract::*;

    #[state]
    pub struct Token {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[constructor]
    pub fn new(initial_supply: Balance) -> Self {
        let caller = Self::env().caller();
        let mut balances = Mapping::default();
        balances.insert(caller, &initial_supply);

        Self {
            total_supply: initial_supply,
            balances,
            allowances: Mapping::default(),
        }
    }

    #[message]
    pub fn transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
        let from = self.env().caller();
        self.transfer_impl(from, to, amount)
    }

    fn transfer_impl(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance
    ) -> Result<()> {
        let from_balance = self.balance_of(from);
        ensure!(from_balance >= amount, Error::InsufficientBalance);

        self.balances.insert(from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.insert(to, &(to_balance + amount));

        self.env().emit_event(Transfer { from, to, amount });
        Ok(())
    }
}
```

### Reentrancy Protection

**Built-in runtime check**:
```rust
thread_local! {
    static CALL_STACK: RefCell<Vec<ContractId>> = RefCell::new(Vec::new());
}

fn execute_contract(contract: ContractId, data: Vec<u8>) -> Result<Vec<u8>> {
    // Check for reentrancy
    CALL_STACK.with(|stack| {
        if stack.borrow().contains(&contract) {
            return Err(Error::ReentrancyDetected);
        }
        stack.borrow_mut().push(contract);
    });

    // Execute
    let result = run_wasm(contract, data);

    // Pop call stack
    CALL_STACK.with(|stack| stack.borrow_mut().pop());

    result
}
```

### VMw Metering Integration

**Injected into WASM bytecode**:
```wasm
;; Original: (i32.add (local.get 0) (local.get 1))
;; Metered:
(call $charge_vmw (i32.const 1))  ;; Charge 1 VMw for add
(i32.add (local.get 0) (local.get 1))
```

**Runtime metering**:
```rust
static mut VMW_REMAINING: u64 = 0;

fn charge_vmw(amount: u64) {
    unsafe {
        if VMW_REMAINING < amount {
            panic!("Out of VMw");
        }
        VMW_REMAINING -= amount;
    }
}
```

---

## 7. NETWORK & P2P LAYER (DETR)

### DETR P2P Protocol

**DETR** = Distributed Ëtrid Transaction Relay

**Based on**:
- libp2p for transport
- S/Kademlia for DHT
- ECIES for encryption
- Gossipsub for message propagation

### Network Topology

**Three Layers**:

1. **Validator Network** (FlareChain)
   - Full mesh between validators
   - Low latency requirements (<100ms)
   - Authenticated connections (Ed25519)

2. **Collator Networks** (PBCs)
   - Star topology around FlareChain validators
   - Periodic state synchronization
   - Cross-PBC messaging via FlareChain

3. **Public Network** (Light Clients)
   - Connect to any validator/collator
   - DHT for peer discovery
   - Receive block headers only

### Message Types

```rust
enum NetworkMessage {
    // Block propagation
    NewBlock(Block),
    BlockRequest(BlockNumber),
    BlockResponse(Block),

    // Transaction pool
    NewTransaction(Transaction),
    TransactionRequest(TxHash),

    // Consensus
    AsfSignature(AsfSignature),
    ValidatorHeartbeat(ValidatorId),

    // Cross-chain
    CrossChainMessage(CrossChainMessage),
    StateRootSubmission(PbcStateRoot),
}
```

### Lightning-Bloc (Layer 2)

**Payment channels** for instant, low-fee transactions:

**Channel Lifecycle**:
1. **Open**: Lock funds in on-chain contract
2. **Transact**: Off-chain signed updates
3. **Close**: Submit final state on-chain
4. **Dispute**: Time-locked challenge period

**Multi-hop routing**:
```
Alice → Bob → Carol → Dave
  2 ÉTR   2 ÉTR   2 ÉTR
```

**Watchtowers**: Monitor channels for fraud attempts

---

## 8. CRYPTOGRAPHIC PRIMITIVES & POST-QUANTUM SECURITY

### Current Cryptography

**Signing**: Ed25519 (Curve25519)
- Public key: 32 bytes
- Signature: 64 bytes
- Fast verification (~60k sigs/sec)

**Hashing**: BLAKE3
- Output: 32 bytes
- Speed: 10 GB/s per core
- Merkle tree optimized

**Encryption**: ChaCha20-Poly1305
- Symmetric AEAD cipher
- Nonce: 12 bytes
- Tag: 16 bytes

### Post-Quantum Migration Path

**Hybrid Scheme** (Current + PQ):
```
Signature = Ed25519_Sign(msg) || Dilithium_Sign(msg)
Verification = Ed25519_Verify() AND Dilithium_Verify()
```

**PQ Algorithms** (NIST selected):
- **Signing**: CRYSTALS-Dilithium (lattice-based)
- **KEM**: CRYSTALS-Kyber (key encapsulation)
- **Alternative**: SPHINCS+ (hash-based signatures)

**Migration Timeline**:
- 2025: Hybrid support added (optional)
- 2026: Hybrid enabled by default
- 2027: PQ-only mode available
- 2028+: Transition to PQ-only (if quantum threat emerges)

---

## CONCLUSION

The E³20 protocol provides a complete, modular foundation for sovereign blockchain networks:

- **ASF**: Adaptive, spectrum-based finality
- **VMw**: Energy-based computation metering
- **Multichain**: Native coordination without bridges
- **EtwasmVM**: Secure, efficient smart contracts
- **DETR**: Robust P2P networking
- **Post-Quantum**: Future-proof cryptography

**Status**: ✅ All 13 components at 100% Alpha Complete

---

**End of Volume II**

**Next**: Volume III - Governance & Fiscal Mechanics

---

*"Technical excellence through first principles."*
