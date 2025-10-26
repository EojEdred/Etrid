# Ëtrid Ember Testnet - Technical Architecture

**Document ID**: ETRID-ARCH-EMBER-2025
**Status**: SPECIFICATION
**Target Launch**: Q1 2026 (January-March)
**Last Updated**: October 24, 2025

---

## 📋 OVERVIEW

**Ember** is the first public testnet for the Ëtrid Protocol, providing a production-like environment for:
- Testing consensus mechanisms (ASF, GRANDPA, BABE)
- Validating multichain coordination (FlareChain + 13 PBCs)
- Simulating governance (Consensus Day)
- Benchmarking performance (TPS, finality, cross-chain)
- Onboarding community validators

This document describes the **technical architecture** of Ember, complementing the infrastructure deployment plan.

---

## 🏗️ ARCHITECTURE OVERVIEW

### System Layers

```
┌────────────────────────────────────────────────────────────┐
│              Layer 5: Applications & UIs                   │
│  • Block Explorer • Validator Dashboard • Governance UI    │
└────────────────────────────────────────────────────────────┘
                           ↕ RPC/WebSocket
┌────────────────────────────────────────────────────────────┐
│          Layer 4: API & Developer Interface                │
│  • JSON-RPC • WebSocket • Substrate API • Polkadot.js     │
└────────────────────────────────────────────────────────────┘
                           ↕ Runtime API
┌────────────────────────────────────────────────────────────┐
│              Layer 3: Runtime (Pallets)                    │
│  • Consensus • Staking • Governance • Treasury • Oracle    │
│  • AIDID • DID Registry • VMw Metering • EtwasmVM         │
└────────────────────────────────────────────────────────────┘
                           ↕ Host Functions
┌────────────────────────────────────────────────────────────┐
│           Layer 2: Consensus & State Machine               │
│  • GRANDPA (Finality) • BABE (Block Production)           │
│  • ASF (Ascending Scale of Finality)                      │
│  • State Trie (Patricia Merkle) • Block Authoring         │
└────────────────────────────────────────────────────────────┘
                           ↕ p2p Network
┌────────────────────────────────────────────────────────────┐
│              Layer 1: Network & Storage                    │
│  • DETR p2p (libp2p) • RocksDB (Storage)                  │
│  • Transaction Pool • Block Import Queue                   │
└────────────────────────────────────────────────────────────┘
```

---

## 🔗 FLARECHAIN: THE ROOT COORDINATION LAYER

### Purpose

FlareChain is the **root chain** that coordinates all activity across the Ëtrid ecosystem:
- Produces blocks every ~6 seconds
- Finalizes blocks using GRANDPA + ASF
- Maintains global state (accounts, staking, governance)
- Anchors Partition Burst Chain (PBC) state proofs
- Processes cross-chain messages

### Block Production (BABE)

**BABE** (Blind Assignment for Blockchain Extension) is the block production algorithm:

```rust
// Simplified BABE logic
fn produce_block(
    slot: SlotNumber,
    validator: ValidatorId,
    vrf_output: VRFOutput,
) -> Option<Block> {
    // VRF determines if validator wins slot
    if is_slot_leader(validator, slot, vrf_output) {
        let parent_hash = get_best_block_hash();
        let txs = select_transactions_from_pool();
        let inherents = construct_inherents(slot);

        // Build block
        let block = Block {
            header: Header {
                parent_hash,
                number: current_block_number + 1,
                state_root: calculate_state_root(),
                extrinsics_root: calculate_extrinsics_root(&txs),
                digest: construct_digest(slot, vrf_output),
            },
            extrinsics: [inherents, txs].concat(),
        };

        Some(block)
    } else {
        None // Not our slot
    }
}
```

**Block Time**: ~6 seconds (configurable)
**Slot Duration**: 6 seconds
**Epoch Length**: 600 slots (1 hour)

### Finality (GRANDPA + ASF)

**GRANDPA** (GHOST-based Recursive Ancestor Deriving Prefix Agreement):
- Byzantine-fault-tolerant finality gadget
- Finalizes chains, not individual blocks
- Requires 2/3+ validator votes

**ASF** (Ascending Scale of Finality):
- Ëtrid's innovation: finality as a spectrum (0-100%)
- Blocks gain confidence over time
- Allows flexible finality thresholds per application

```rust
// ASF finality calculation (from Ivory Paper Vol II)
fn calculate_finality(
    confirmed_stake: u128,
    total_stake: u128,
    blocks_elapsed: u32,
    participation_rate: f64,
) -> f64 {
    let stake_ratio = confirmed_stake as f64 / total_stake as f64;
    let time_factor = 1.0 - (-0.05 * blocks_elapsed as f64).exp();
    let participation_bonus = participation_rate.powf(0.5);

    let base_finality = stake_ratio * 0.7 + time_factor * 0.2 + participation_bonus * 0.1;

    // Sigmoid for smooth 0-100% curve
    1.0 / (1.0 + (-10.0 * (base_finality - 0.5)).exp())
}
```

**Finality Progression** (typical):
```
Time:      0s    30s    60s    120s   200s
Finality:  5% →  50% →  80% →  95% →  99.9%
Status:    Low   Med    High   Safe   Final
```

### State Model

**Account Model** (not UTXO):
```rust
pub struct AccountInfo {
    nonce: Index,
    consumers: RefCount,
    providers: RefCount,
    sufficients: RefCount,
    data: AccountData,
}

pub struct AccountData {
    free: Balance,      // Transferable balance
    reserved: Balance,  // Locked balance (staking, governance)
    misc_frozen: Balance,
    fee_frozen: Balance,
}
```

**Storage**: Patricia Merkle Trie (state root in block header)

---

## 🧩 PARTITION BURST CHAINS (PBCs)

### Architecture

Each PBC is a **sovereign runtime** with its own:
- State (independent from FlareChain)
- Consensus (collator produces blocks)
- Transactions (domain-specific)
- Bridge logic (to FlareChain)

```
┌──────────────────────────────────────────────────────────┐
│                      FlareChain                          │
│  (Validators: 50-150)                                    │
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  PBC State Registry (pallet-pbc-registry)      │    │
│  │  • BTC-PBC: StateRoot(0xabc...)               │    │
│  │  • ETH-PBC: StateRoot(0xdef...)               │    │
│  │  • ... (all 13 PBCs)                          │    │
│  └────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────┘
        ↕              ↕              ↕              ↕
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ...
│  BTC-PBC    │  │  ETH-PBC    │  │  DOGE-PBC   │
│ (Collator)  │  │ (Collator)  │  │ (Collator)  │
│             │  │             │  │             │
│ • Bridge    │  │ • Bridge    │  │ • Bridge    │
│ • State     │  │ • State     │  │ • State     │
└─────────────┘  └─────────────┘  └─────────────┘
```

### PBC Block Production

**Collator Role**:
1. Collect transactions for their PBC
2. Produce block (every ~6s, synced with FlareChain)
3. Submit state proof to FlareChain
4. Wait for FlareChain to include proof
5. Repeat

**State Proof Format**:
```rust
pub struct PBCStateProof {
    pbc_id: PBCId,
    block_number: BlockNumber,
    state_root: Hash,
    merkle_proof: Vec<Vec<u8>>,  // Proof of state transition
    signature: CollatorSignature,
}
```

### Cross-Chain Message Passing

**Message Flow** (BTC-PBC → ETH-PBC):
```
1. User submits tx on BTC-PBC: "Transfer 10 BTC to ETH-PBC address"
2. BTC-PBC collator includes tx in block
3. BTC-PBC submits state proof to FlareChain
4. FlareChain validates proof, stores message in queue
5. ETH-PBC queries FlareChain for pending messages
6. ETH-PBC processes message: "Mint 10 wrapped BTC"
7. ETH-PBC submits proof of execution to FlareChain
8. FlareChain marks message as complete
```

**Message Format**:
```rust
pub struct CrossChainMessage {
    source_pbc: PBCId,
    dest_pbc: PBCId,
    nonce: u64,
    payload: Vec<u8>,  // Encoded call or data
    signature: Signature,
}
```

---

## 🔐 CONSENSUS PARAMETERS (EMBER)

### Validator Set

**Active Validators**: 50-150 (dynamic)
- **Selection**: Top 100 by stake + 50 random (weighted by stake)
- **Minimum Stake**: 100,000 ÉTR (testnet may be lower)
- **Session Length**: 1 hour (600 blocks)
- **Era Length**: 24 hours (24 sessions)

**Validator Rewards** (per era):
```rust
fn calculate_era_rewards(era: EraIndex) -> Balance {
    // 3% annual inflation / 365 eras
    let annual_inflation_rate = 0.03;
    let circulating_supply = get_circulating_supply();
    let era_reward_pool = circulating_supply * annual_inflation_rate / 365.0;

    Balance::from(era_reward_pool as u128)
}
```

### Slashing

**Conditions**:
| Offense | Severity | Slash % | Examples |
|---------|----------|---------|----------|
| Offline | Low | 0.1% per hour | Node crash, network outage |
| Equivocation | High | 10% | Double-signing blocks |
| Invalid Vote | Med | 5% | Voting for bad block |

**Slash Distribution**:
- 50% burned (deflationary)
- 50% to treasury (public goods)

### Finality Parameters

**GRANDPA**:
- **Threshold**: 2/3+ of validators must vote
- **Round Duration**: ~6 seconds (adjusts to network conditions)
- **Vote Types**: Prevote, Precommit

**ASF Thresholds**:
```
Applications can choose finality level:
  - Instant payments: 50% finality (30s)
  - Standard tx: 80% finality (60s)
  - High-value tx: 95% finality (120s)
  - Irreversible: 99.9% finality (200s)
```

---

## 💾 STORAGE & STATE

### Database

**RocksDB** (default):
- Key-value store
- Optimized for SSDs
- Column families for different data types

**Schema**:
```
Column Families:
  - state:     Patricia Merkle Trie (accounts, storage)
  - header:    Block headers
  - body:      Block bodies (transactions)
  - justification: GRANDPA finality proofs
  - aux:       Auxiliary data (metadata)
```

### State Pruning

**Pruning Modes**:
1. **Archive** (no pruning):
   - Stores all historical state
   - Disk: ~4TB+ over time
   - Use: Block explorers, analytics

2. **Pruned** (default):
   - Keeps last 256 blocks of state
   - Disk: ~500GB after 1 year
   - Use: Validators, RPC nodes

3. **Compact**:
   - Aggressive pruning (last 64 blocks)
   - Disk: ~100GB
   - Use: Light clients (not yet supported)

### State Snapshots

**Purpose**: Fast sync for new nodes

**Format** (WIP):
```rust
pub struct StateSnapshot {
    block_number: BlockNumber,
    block_hash: Hash,
    state_root: Hash,
    accounts: Vec<(AccountId, AccountInfo)>,
    storage: Vec<(StorageKey, StorageValue)>,
}
```

**Snapshot Servers**: snapshots.ember.etrid.org
**Update Frequency**: Daily (midnight UTC)
**Retention**: Last 30 days

---

## 🌐 NETWORK LAYER (DETR p2p)

### Libp2p Stack

**Protocol**: Based on Substrate's networking (libp2p)

**Transport Protocols**:
```
TCP/IP (default):  /ip4/x.x.x.x/tcp/30333
WebSocket:         /ip4/x.x.x.x/tcp/9944/ws
QUIC (future):     /ip4/x.x.x.x/udp/30333/quic
```

**Peer Discovery**:
- mDNS (local network)
- Kademlia DHT (global network)
- Bootstrap nodes (hardcoded)

**Bootstrap Nodes** (Ember):
```
/dns/boot1.ember.etrid.org/tcp/30333/p2p/12D3KooW...
/dns/boot2.ember.etrid.org/tcp/30333/p2p/12D3KooW...
/dns/boot3.ember.etrid.org/tcp/30333/p2p/12D3KooW...
```

### Gossip Protocols

**Block Propagation**:
```rust
// Validator produces block, gossips to peers
fn gossip_block(block: Block) {
    for peer in connected_peers() {
        send_message(peer, GossipMessage::BlockAnnounce(block.header));
    }
}
```

**Transaction Pool Sync**:
```rust
// Transactions gossip before inclusion in blocks
fn gossip_transaction(tx: Extrinsic) {
    for peer in connected_peers() {
        send_message(peer, GossipMessage::Transaction(tx));
    }
}
```

### Network Parameters

**Target Peers**: 50 (25 in, 25 out)
**Max Peers**: 125
**Peer Timeout**: 60 seconds
**Reputation System**: Peers gain/lose reputation based on behavior

---

## 🧮 COMPUTATION MODEL (VMw)

### Virtual Machine Watts (VMw)

**Definition**: 1 VMw = 1 Watt-second of CPU energy

**Instruction Weights** (examples):
```rust
// Arithmetic
add: 1 VMw
sub: 1 VMw
mul: 2 VMw
div: 4 VMw
rem: 4 VMw

// Memory
memory_copy: 1 VMw per byte
storage_read: 1,000 VMw
storage_write: 2,000 VMw

// Crypto
blake2_256: 100 VMw
ed25519_verify: 2,000 VMw
```

### Block VMw Limit

**Ember Configuration**:
```rust
pub const MAXIMUM_BLOCK_VMW: u64 = 10_000_000; // 10 million VMw
pub const TARGET_BLOCK_VMW: u64 = 5_000_000;   // 50% target utilization
```

**Dynamic Fee Adjustment**:
```rust
fn calculate_vmw_price(block_fullness: f64) -> Balance {
    let base_price = 1_000_000; // 1e-6 ÉTR per VMw

    // Surge pricing above 75% utilization
    let multiplier = if block_fullness > 0.75 {
        1.0 + ((block_fullness - 0.75) * 8.0)
    } else {
        1.0
    };

    Balance::from((base_price as f64 * multiplier) as u128)
}
```

### Transaction Fees

**Formula**:
```
Fee = VMw_Cost × VMw_Price + Priority_Tip

Example:
  Simple transfer: 100 VMw × 0.000001 ÉTR = 0.0001 ÉTR ($0.001 at $10/ÉTR)
  Token swap: 5,000 VMw × 0.000001 ÉTR = 0.005 ÉTR ($0.05)
```

---

## 🏛️ GOVERNANCE (EMBER TESTNET)

### Limited Governance

**Ember ≠ Full Governance**:
- No Consensus Day (annual vote) on testnet
- Simplified proposal system for testing
- Foundation-controlled treasury (testnet only)

### Proposal Testing

**Types Enabled**:
1. **Parameter Changes**: Block time, VMw limits, fee structure
2. **Runtime Upgrades**: Test forkless upgrades
3. **Treasury Spend**: Test budget allocations (play money)

**Process**:
```rust
1. Submit proposal (requires 1,000 testnet ÉTR bond)
2. Voting period: 7 days
3. Threshold: Simple majority (>50%)
4. Execution: Automatic if approved
```

### Voting Mechanism

```rust
pub fn vote(
    origin: OriginFor<T>,
    proposal_id: ProposalIndex,
    vote: bool, // true = yes, false = no
) -> DispatchResult {
    let voter = ensure_signed(origin)?;
    let voting_power = get_voting_power(&voter);

    // Record vote
    Votes::<T>::insert((proposal_id, voter), (vote, voting_power));

    // Update tally
    if vote {
        YesVotes::<T>::mutate(proposal_id, |v| *v += voting_power);
    } else {
        NoVotes::<T>::mutate(proposal_id, |v| *v += voting_power);
    }

    Ok(())
}
```

---

## 🔧 RUNTIME CONFIGURATION

### Runtime Version

```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("etrid-ember"),
    impl_name: create_runtime_str!("etrid-node"),
    authoring_version: 1,
    spec_version: 900, // 0.9.0
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
```

### Pallets Enabled (Ember)

**Core** (always enabled):
- `frame_system` - Core system functionality
- `pallet_timestamp` - Block timestamps
- `pallet_sudo` - Superuser (testnet only!)

**Consensus**:
- `pallet_babe` - Block production
- `pallet_grandpa` - Finality
- `pallet_authority_discovery` - Validator discovery
- `pallet_im_online` - Uptime tracking

**Accounts & Assets**:
- `pallet_balances` - Native ÉTR token
- `pallet_transaction_payment` - Fee handling
- `pallet_assets` - Multi-asset support

**Staking & Governance**:
- `pallet_staking` - Validator staking
- `pallet_session` - Session management
- `pallet_democracy` - Proposal voting (simplified)
- `pallet_treasury` - Community treasury
- `pallet_elections_phragmen` - Council elections (optional)

**Ëtrid-Specific**:
- `pallet_reserve_oracle` - Price oracle for EDSC
- `pallet_aidid` - AI decentralized identity
- `pallet_did_registry` - DID management
- `pallet_validator_committee` - Validator coordination
- `pallet_pbc_registry` - PBC state tracking

**Smart Contracts** (Phase 2):
- `pallet_etwasm` - WebAssembly VM (deployed later)

---

## 📊 PERFORMANCE TARGETS (EMBER)

### Target Metrics

| Metric | Ember Target | Production Target |
|--------|--------------|-------------------|
| Block Time | ~6 seconds | ~6 seconds |
| Finality Lag | <100 blocks | <50 blocks |
| TPS (sustained) | 1,000+ | 2,000+ |
| TPS (burst) | 5,000+ | 10,000+ |
| Active Validators | 50-150 | 100-300 |
| RPC Latency (p95) | <200ms | <100ms |
| Network Uptime | 99.9% | 99.95% |

### Benchmarking

**Extrinsic Weights** (re-benchmark for Ember):
```bash
# Benchmark all pallets
cargo run --release --features=runtime-benchmarks \
  -- benchmark pallet \
  --chain=ember \
  --execution=wasm \
  --wasm-execution=compiled \
  --pallet="*" \
  --extrinsic="*" \
  --output=./runtime/src/weights/
```

**Block Construction Benchmark**:
```bash
# Measure block import time
cargo run --release -- benchmark block \
  --chain=ember \
  --from=1 \
  --to=1000
```

---

## 🧪 TESTING STRATEGY

### Unit Tests

```bash
# Run all pallet tests
cargo test --workspace

# Expected: 412+ tests passing
```

### Integration Tests

```bash
# Run end-to-end tests
cargo test --features=runtime-benchmarks integration_tests

# Tests:
#   - Block production and finality
#   - Cross-chain message passing
#   - Staking and rewards
#   - Governance proposals
```

### Stress Tests

```bash
# Spam transactions to test throughput
node scripts/stress-test.js \
  --endpoint=wss://ember.etrid.org \
  --tps=1000 \
  --duration=600s
```

### Security Tests

- [ ] Fuzzing (honggfuzz, libfuzzer)
- [ ] Static analysis (cargo-audit, clippy)
- [ ] Formal verification (for critical pallets)
- [ ] External audit (post-launch)

---

## 🚀 DEPLOYMENT PHASES

### Phase 1: Private Alpha (Week 1-2)
- 3 foundation validators only
- No public access
- Internal testing

### Phase 2: Closed Beta (Week 3-4)
- Invite 10-20 trusted community validators
- Open RPC endpoints (rate-limited)
- Faucet available to whitelisted addresses

### Phase 3: Public Launch (Week 5+)
- Open to all validators
- Public faucet
- Block explorer live
- Announce via social media

---

## 📝 APPENDIX A: Chain Specification

### Ember Chain Spec (genesis config)

```json
{
  "name": "Ëtrid Ember Testnet",
  "id": "etrid-ember",
  "chainType": "Live",
  "bootNodes": [
    "/dns/boot1.ember.etrid.org/tcp/30333/p2p/12D3KooW...",
    "/dns/boot2.ember.etrid.org/tcp/30333/p2p/12D3KooW...",
    "/dns/boot3.ember.etrid.org/tcp/30333/p2p/12D3KooW..."
  ],
  "telemetryEndpoints": [
    ["wss://telemetry.etrid.org/submit/", 0]
  ],
  "protocolId": "etrd",
  "properties": {
    "tokenSymbol": "tÉTR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "babe": {
        "authorities": [],
        "epochConfig": {
          "c": [1, 4],
          "allowed_slots": "PrimaryAndSecondaryPlainSlots"
        }
      },
      "grandpa": {
        "authorities": []
      },
      "balances": {
        "balances": [
          ["5GrwvaEF...", "1000000000000000000"],  // Foundation 1
          ["5FHneW46...", "1000000000000000000"],  // Foundation 2
          ["5FLSigC9...", "1000000000000000000"]   // Foundation 3
        ]
      },
      "sudo": {
        "key": "5GrwvaEF..."  // Foundation multisig
      }
    }
  }
}
```

---

## 📝 APPENDIX B: RPC Methods

### Standard Methods (Substrate)

```javascript
// Block info
chain.getBlock(hash?)
chain.getBlockHash(number?)
chain.getHeader(hash?)
chain.getFinalizedHead()

// State queries
state.getStorage(key, hash?)
state.getKeys(prefix, hash?)
state.queryStorage(keys, fromBlock, toBlock?)

// Transaction submission
author.submitExtrinsic(extrinsic)
author.pendingExtrinsics()
```

### Custom Methods (Ëtrid)

```javascript
// ASF finality
etrid.getFinality(blockHash)
// Returns: { finality: 0.95, confirmedStake: 150000000, totalStake: 200000000 }

// PBC state
etrid.getPBCState(pbcId)
// Returns: { blockNumber: 12345, stateRoot: "0xabc...", lastUpdate: 67890 }

// VMw pricing
etrid.getCurrentVMwPrice()
// Returns: { pricePerVMw: 1000000, blockFullness: 0.45 }
```

---

## 📝 APPENDIX C: Known Limitations (Ember)

1. **Sudo Access**: Foundation has sudo key (removed at mainnet)
2. **Limited Governance**: No Consensus Day, simplified voting
3. **Testnet Tokens**: No economic value, can be reset
4. **Performance**: Not fully optimized (benchmarks ongoing)
5. **PBC Coverage**: Not all 13 PBCs may be active at launch

---

**End of Ember Architecture Document**

**See Also**:
- Infrastructure Deployment Plan (EMBER_TESTNET_INFRASTRUCTURE_PLAN.md)
- Ivory Papers Volume II (Technical Specification)
- Validator Guide (coming soon)

---

*This document evolves as Ember develops. Last updated: Oct 24, 2025*
