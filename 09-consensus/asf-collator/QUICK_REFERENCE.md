# ASF Collator Quick Reference

## Module Locations

```
asf-collator              → /Users/macbook/Desktop/etrid/09-consensus/asf-collator/
pallet-asf-collator       → /Users/macbook/Desktop/etrid/09-consensus/pallet-asf-collator/
asf-bridge-security       → /Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/asf-bridge-security/
```

## Key Types

```rust
// Collator committee
CollatorCommittee {
    para_id: ParaId,
    collators: Vec<CollatorId>,
    min_collators: u32,
    max_collators: u32,
    rotation_round: u64,
    total_stake: Balance,
}

// Collator vote
CollatorVote {
    para_id: ParaId,
    block_hash: Hash,
    block_number: BlockNumber,
    collator: CollatorId,
    phase: ConsensusPhase,
    rotation_round: u64,
    signature: Vec<u8>,
}

// Validity certificate
CollatorCertificate {
    para_id: ParaId,
    block_hash: Hash,
    block_number: BlockNumber,
    phase: ConsensusPhase,
    rotation_round: u64,
    votes: Vec<CollatorVote>,
    stake_weight: Balance,
}

// Finality levels
CollatorFinalityLevel {
    None = 0,        // 0-4 certs
    Weak = 1,        // 5-9 certs
    Moderate = 2,    // 10-19 certs
    Strong = 3,      // 20-49 certs
    Irreversible = 4 // 50+ certs OR relay finalized
}
```

## Runtime Integration

### 1. Add Dependencies

```toml
[dependencies]
pallet-asf-collator = { path = "path/to/pallet-asf-collator", default-features = false }
asf-collator = { path = "path/to/asf-collator", default-features = false }
```

### 2. Configure Pallet

```rust
parameter_types! {
    pub const ParaId: u32 = 2000;
    pub const MinCollators: u32 = 7;
    pub const MaxCollators: u32 = 11;
    pub const MinCollatorStake: Balance = 1_000_000 * UNITS;
    pub const SessionLength: BlockNumber = 600;
    pub const RotationPeriod: u64 = 6;
}

impl pallet_asf_collator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ParaId = ParaId;
    type MinCollators = MinCollators;
    type MaxCollators = MaxCollators;
    type MinCollatorStake = MinCollatorStake;
    type SessionLength = SessionLength;
    type RotationPeriod = RotationPeriod;
}
```

### 3. Add to Runtime

```rust
construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        AsfCollator: pallet_asf_collator,
    }
);
```

## Common Operations

### Register as Collator

```rust
// From user account
let stake = 5_000_000 * UNITS;
AsfCollator::register_collator(origin, stake)?;
```

### Submit Block Vote

```rust
// From collator account
AsfCollator::submit_vote(
    origin,
    block_hash,
    block_number,
    ConsensusPhase::Prepare,
    signature,
)?;
```

### Check Finality

```rust
let finality = AsfCollator::finality_level(block_hash);
match finality {
    CollatorFinalityLevel::None => { /* Not finalized */ }
    CollatorFinalityLevel::Weak => { /* Low-value OK */ }
    CollatorFinalityLevel::Moderate => { /* Medium-value OK */ }
    CollatorFinalityLevel::Strong => { /* High-value OK */ }
    CollatorFinalityLevel::Irreversible => { /* Bridge OK */ }
}
```

### Create Bridge Attestation

```rust
use asf_collator::CrossChainAttestation;

let attestation = CrossChainAttestation::new(
    source_para,     // e.g., 2000 (BTC-PBC)
    target_para,     // e.g., 2001 (ETH-PBC)
    block_hash,
    block_number,
    finality_level,  // Must be Strong or higher
    collator_id,
    relay_block,
);

AsfCollator::submit_cross_chain_attestation(origin, attestation)?;
```

## Configuration Presets

### High Security (BTC, ETH)

```rust
MinCollators: 11
MaxCollators: 21
MinStake: 5_000_000 * UNITS
RotationPeriod: 12
SessionLength: 1200
```

### Medium Security (SOL, BNB, XRP)

```rust
MinCollators: 9
MaxCollators: 15
MinStake: 2_000_000 * UNITS
RotationPeriod: 6
SessionLength: 600
```

### Lower Security (TRX, SC-USDT)

```rust
MinCollators: 7
MaxCollators: 11
MinStake: 1_000_000 * UNITS
RotationPeriod: 6
SessionLength: 600
```

## BFT Thresholds

```
Committee Size | BFT Threshold | Max Byzantine
7              | 5             | 2
9              | 7             | 3
11             | 8             | 3
15             | 11            | 4
21             | 15            | 6
```

Formula: `threshold = (size * 2) / 3 + 1`

## Finality Times

```
Level         | Certificates | Time (typical)
None          | 0-4          | 0-24s
Weak          | 5-9          | 30s-54s
Moderate      | 10-19        | 60s-114s
Strong        | 20-49        | 2-3 min
Irreversible  | 50+ or relay | ~10 min
```

## Bridge Security

### Requirements

```
Minimum Attestation Stake: transfer_value × 2
Minimum Finality Level: Strong (20+ certs)
Challenge Period: 100 relay blocks (~10 min)
Slash Multiplier: 10x deposit
```

### Security Proof Components

```rust
BridgeSecurityProof {
    transfer: BridgeTransfer,           // Transfer details
    source_attestation: MultiSig,       // 2/3+ collators
    merkle_proof: MerkleProof,          // Inclusion proof
    economic_deposits: Vec<Deposit>,    // 2x value
    total_security: Balance,            // Total deposits
}
```

### Verification

```rust
proof.verify(config, min_security)?;
// Checks:
// ✓ Attestation meets stake threshold
// ✓ Finality level >= Strong
// ✓ Merkle proof valid
// ✓ Economic security >= 2x value
```

## CLI Commands

### Build Collator Node

```bash
cargo build --release -p your-pbc-collator
```

### Run Collator

```bash
./target/release/your-pbc-collator \
  --collator \
  --para-id 2000 \
  --chain spec.json \
  --relay-chain-rpc-url ws://localhost:9944
```

### Generate Keys

```bash
# Session keys
./target/release/your-pbc-collator key generate-node-key

# Collator keys
./target/release/your-pbc-collator key generate \
  --scheme sr25519 \
  --output-type json
```

## Monitoring

### Check Committee

```rust
let committee = AsfCollator::committee();
log::info!("Collators: {:?}", committee.collators.len());
log::info!("Total stake: {}", committee.total_stake);
```

### Check Rotation

```rust
let round = AsfCollator::rotation_round();
let session = AsfCollator::session_index();
log::info!("Round: {}, Session: {}", round, session);
```

### Check Certificates

```rust
let count = AsfCollator::certificate_count(block_hash);
let finality = AsfCollator::finality_level(block_hash);
log::info!("Block {}: {} certs, {:?} finality", block_hash, count, finality);
```

## Troubleshooting

### Not Producing Blocks

1. Check collator registration:
   ```rust
   let stake = AsfCollator::collator_stake(&account);
   ensure!(stake > 0, "Not registered");
   ```

2. Check rotation round:
   ```rust
   let round = AsfCollator::rotation_round();
   // Should increment every rotation_period blocks
   ```

3. Check committee:
   ```rust
   let committee = AsfCollator::committee();
   ensure!(committee.is_valid(), "Invalid committee");
   ```

### Slow Finality

1. Increase collators:
   ```rust
   pub const MaxCollators: u32 = 15; // Increase
   ```

2. Reduce rotation period:
   ```rust
   pub const RotationPeriod: u64 = 3; // Faster
   ```

3. Check network connectivity:
   ```bash
   # Verify peers
   curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
     http://localhost:9933
   ```

### Bridge Transfers Stuck

1. Check source finality:
   ```rust
   let finality = AsfCollator::finality_level(source_block);
   ensure!(finality >= CollatorFinalityLevel::Strong, "Wait for finality");
   ```

2. Check attestations:
   ```rust
   let attestation = AsfCollator::cross_chain_attestation(para_id, block_hash);
   match attestation {
       Some(att) => log::info!("Attestations: {}", att.attestations.len()),
       None => log::warn!("No attestations yet"),
   }
   ```

3. Check economic security:
   ```rust
   if let Some(att) = attestation {
       let required = transfer_value * 2;
       ensure!(att.total_stake >= required, "Insufficient security");
   }
   ```

## Testing

### Unit Tests

```bash
# Test asf-collator module
cargo test -p asf-collator

# Test pallet
cargo test -p pallet-asf-collator

# Test bridge security
cargo test -p asf-bridge-security
```

### Integration Tests

```bash
# Run full collator test
cargo test --release --features runtime-benchmarks
```

### Benchmarks

```bash
# Benchmark pallet extrinsics
cargo run --release --features runtime-benchmarks -- \
  benchmark pallet \
  --pallet pallet_asf_collator \
  --extrinsic "*" \
  --output ./weights.rs
```

## Resources

- **Implementation Report**: `ASF_COLLATOR_IMPLEMENTATION_REPORT.md`
- **Integration Guide**: `INTEGRATION_GUIDE.md`
- **Architecture**: `ARCHITECTURE.md`
- **Source Code**: `src/lib.rs`, `src/*.rs`

---

**Quick Reference Version**: 1.0
**Last Updated**: 2025-11-15
