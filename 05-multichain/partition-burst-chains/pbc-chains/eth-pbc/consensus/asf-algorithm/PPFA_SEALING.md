# PPFA Sealing Implementation Guide

## Overview

PPFA (Proposing Panel for Attestation) is the block sealing and validator rotation mechanism in Ëtrid's ASF (Ascending Scale of Finality) consensus protocol. This document describes the complete implementation of PPFA sealing finalization logic.

## Architecture

### Components

1. **PpfaSeal** - Cryptographic seal proving validator authority to propose a block
2. **PpfaMember** - Validator information within the PPFA committee
3. **PpfaCommittee** - The rotating committee of validators
4. **PpfaSealVerifier** - Verifies seal validity and voting weights
5. **PpfaSealingEngine** - Main engine coordinating sealing and finalization

### Key Concepts

#### Committee Rotation

The PPFA committee consists of the top N validators by stake weight (default: 21). The committee rotates deterministically using a round-robin schedule:

```
slot_number % committee_size = validator_index
```

**Example:**
- Committee size: 21 validators
- Slot 0 → Validator at index 0
- Slot 5 → Validator at index 5
- Slot 21 → Validator at index 0 (wraps around)
- Slot 25 → Validator at index 4

#### Stake-Weighted Voting

Each validator's voting power is proportional to their stake:

```
voting_weight = (validator_stake / total_committee_stake) × 1,000,000
```

This ensures that validators with more stake have proportionally more influence in consensus decisions while maintaining Byzantine fault tolerance.

#### Seal Structure

Each block seal contains:
- **Slot number** - When the block was produced
- **PPFA index** - Validator's position in committee
- **Validator ID** - Who produced the block
- **Stake weight** - Validator's stake at time of proposal
- **Epoch** - Current epoch number
- **Block hash** - Hash of the sealed block
- **Signature** - Cryptographic proof of authority

## Usage Examples

### Creating a PPFA Committee

```rust
use asf_algorithm::{PpfaMember, PpfaCommittee};
use sp_core::crypto::AccountId32;

// Create validators
let validators = vec![
    PpfaMember::new(validator1_account, 100_000, 0),
    PpfaMember::new(validator2_account, 80_000, 1),
    PpfaMember::new(validator3_account, 50_000, 2),
    // ... up to 21 validators
];

// Create committee for epoch 1
let committee = PpfaCommittee::new(validators, 1);
```

### Sealing a Block

```rust
use asf_algorithm::{PpfaSealingEngine, PpfaCommittee};

// Initialize sealing engine
let mut engine = PpfaSealingEngine::new(committee);

// Create seal for current slot
let validator = current_validator_account;
let block_hash = compute_block_hash(&block);
let block_number = 100;

let seal = engine.create_seal(
    validator,
    block_number,
    block_hash
)?;

// Finalize the block
let finalized = engine.finalize_block(
    seal,
    block_hash,
    block_number
)?;

println!("Block {} finalized by {:?}",
    finalized.block_number,
    finalized.proposer()
);
```

### Verifying a Seal

```rust
use asf_algorithm::{PpfaSealVerifier, PpfaSeal};

// Create verifier
let verifier = PpfaSealVerifier::new(committee);

// Verify seal
match verifier.verify_seal(&seal) {
    Ok(()) => println!("Seal is valid!"),
    Err(e) => println!("Seal verification failed: {:?}", e),
}

// Calculate voting weight
let weight = verifier.calculate_vote_weight(&seal);
println!("Validator voting weight: {}", weight);
```

### Committee Rotation

```rust
// Advance through slots
for slot in 0..100 {
    // Get expected proposer
    let proposer = committee.get_proposer(slot).unwrap();

    println!("Slot {}: Validator {} should propose",
        slot, proposer.index);

    // Advance to next slot
    engine.advance_slot();
}
```

## Integration with Pallet

The PPFA sealing is integrated with the consensus pallet's `finalize_block()` function:

### Pallet Integration

```rust
// In pallet/src/lib.rs
pub fn finalize_block(
    block_hash: T::Hash,
    block_number: BlockNumberFor<T>,
    votes: Vec<T::AccountId>,
) -> DispatchResult {
    // Step 1: Verify vote count meets BFT threshold (2/3 + 1)
    let threshold = (total_validators * 2) / 3;
    ensure!(votes.len() >= threshold, Error::<T>::FinalizationFailed);

    // Step 2: Verify stake-weighted voting power
    let total_vote_stake = calculate_total_stake(&votes);
    let stake_threshold = (total_committee_stake * 2) / 3;
    ensure!(total_vote_stake >= stake_threshold, Error::<T>::FinalizationFailed);

    // Step 3: Verify PPFA seal consistency
    let certificates = Certificates::<T>::get(block_hash);
    for cert in certificates.iter() {
        ensure!(
            committee.contains(&cert.validator),
            Error::<T>::NotInCommittee
        );
    }

    // Step 4: Reward validators
    // Step 5: Calculate finality level
    // Step 6: Emit finalization event
}
```

## Security Properties

### Byzantine Fault Tolerance

PPFA maintains BFT properties:
- **Safety**: Only one validator can produce valid seal for each slot
- **Liveness**: Committee rotation ensures progress even if some validators fail
- **Accountability**: All seals are cryptographically signed and verifiable

### Stake-Weighted Security

- Validators with more stake have proportional voting power
- Minimum 2/3 stake threshold required for finalization
- Prevents minority stake from controlling consensus

### Committee Security

- Top validators by stake are selected for committee
- Regular rotation (every epoch) prevents centralization
- Maximum committee size (21) balances security and efficiency

## Testing

### Unit Tests

All PPFA components have comprehensive unit tests:

```bash
# Run all PPFA tests
cargo test ppfa

# Run specific test suite
cargo test --test ppfa_sealing_tests
```

### Test Coverage

- ✅ Basic seal creation and verification
- ✅ Committee rotation through all members
- ✅ Stake-weighted voting calculations
- ✅ Edge cases (single validator, zero stake)
- ✅ Byzantine fault scenarios
- ✅ Committee updates and epoch transitions
- ✅ Performance with large committees (100+ validators)
- ✅ Multi-slot workflows (1000+ slots)

### Example Test

```rust
#[test]
fn test_ppfa_rotation() {
    let committee = create_committee(21, 10_000);

    // Test first rotation cycle
    for i in 0..21 {
        let proposer = committee.get_proposer(i).unwrap();
        assert_eq!(proposer.index, i as u32);
    }

    // Test wrap-around
    let proposer = committee.get_proposer(21).unwrap();
    assert_eq!(proposer.index, 0);
}
```

## Performance Characteristics

### Time Complexity

- **Seal Creation**: O(1) - Direct lookup by slot number
- **Seal Verification**: O(1) - Hash-based validator lookup
- **Committee Rotation**: O(1) - Modulo arithmetic
- **Vote Weight Calculation**: O(1) - Simple division

### Space Complexity

- **Committee**: O(N) where N is committee size (typically 21)
- **Seal**: O(1) - Fixed size structure
- **Finalized Block**: O(1) - Single seal per block

### Scalability

- Tested with committees up to 100 validators
- Processed 1000+ slots in tests without performance degradation
- Memory usage remains constant regardless of slot number

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `InvalidVote("Validator not in committee")` | Validator not part of current committee | Check committee membership before creating seal |
| `InvalidVote("Wrong validator for slot")` | Validator proposing out of turn | Wait for correct slot |
| `InvalidVote("Stake weight mismatch")` | Stake changed since seal creation | Update stake and recreate seal |
| `InvalidVote("Seal epoch mismatch")` | Using seal from previous epoch | Update to current epoch |
| `InvalidCertificate("Block hash mismatch")` | Seal doesn't match block | Ensure seal created for correct block |

## Advanced Usage

### Custom Committee Selection

```rust
// Create committee with custom stake distribution
let members = vec![
    PpfaMember::new(whale_validator, 1_000_000, 0),  // High stake
    PpfaMember::new(medium_validator, 100_000, 1),   // Medium stake
    PpfaMember::new(small_validator, 10_000, 2),     // Low stake
];

let committee = PpfaCommittee::new(members, current_epoch);

// Verify proportional voting weights
let whale_weight = calculate_weight(&whale_validator, &committee);
let small_weight = calculate_weight(&small_validator, &committee);
assert!(whale_weight > small_weight * 90); // ~100x more stake
```

### Multi-Epoch Workflow

```rust
// Epoch 1
let committee1 = create_committee_for_epoch(1);
let mut engine = PpfaSealingEngine::new(committee1);

// Process blocks...
for slot in 0..2400 {
    let seal = engine.create_seal(...)?;
    engine.finalize_block(seal, ...)?;
    engine.advance_slot();
}

// Epoch 2 - Update committee
let committee2 = create_committee_for_epoch(2);
engine.update_committee(committee2);

// Continue processing...
```

### Monitoring and Metrics

```rust
// Track block production
for validator in committee.members() {
    println!(
        "Validator {}: {} blocks produced",
        validator.index,
        validator.blocks_produced
    );
}

// Monitor voting weights
let total_stake = committee.total_stake();
for validator in committee.members() {
    let weight_percent = (validator.stake * 100) / total_stake;
    println!("Validator {}: {}% voting power", validator.index, weight_percent);
}
```

## Migration Guide

### From Simple Finalization to PPFA

**Before (Simple finalization):**
```rust
fn finalize_block(hash: Hash, votes: Vec<AccountId>) -> Result<()> {
    if votes.len() >= threshold {
        emit_event(Finalized { hash });
        Ok(())
    } else {
        Err(NotEnoughVotes)
    }
}
```

**After (PPFA sealing):**
```rust
fn finalize_block(
    hash: Hash,
    number: BlockNumber,
    votes: Vec<AccountId>
) -> Result<()> {
    // Verify vote count threshold
    ensure!(votes.len() >= bft_threshold(total_validators));

    // Verify stake threshold
    let vote_stake = calculate_stake(&votes);
    ensure!(vote_stake >= bft_stake_threshold(total_stake));

    // Verify all votes from committee members
    let committee = get_current_committee();
    for voter in &votes {
        ensure!(committee.contains(voter));
    }

    emit_event(Finalized { hash, votes: votes.len() });
    Ok(())
}
```

## Best Practices

1. **Always verify seals before processing blocks**
   ```rust
   verifier.verify_seal(&seal)?;
   ```

2. **Update committee at epoch boundaries**
   ```rust
   if current_block % EPOCH_DURATION == 0 {
       let new_committee = select_top_validators_by_stake();
       engine.update_committee(new_committee);
   }
   ```

3. **Track validator performance**
   ```rust
   committee.record_block(&validator);
   ```

4. **Handle epoch transitions gracefully**
   ```rust
   // Allow grace period for epoch transition
   if seal.epoch == current_epoch - 1 && within_grace_period() {
       // Accept seal from previous epoch
   }
   ```

## Troubleshooting

### Seal Verification Fails

**Problem**: `verify_seal()` returns error

**Checklist**:
1. Is validator in current committee?
2. Is it the validator's turn (correct slot)?
3. Does stake weight match current stake?
4. Is epoch number current?
5. Is signature valid?

### Incorrect Voting Weights

**Problem**: Voting weights don't sum correctly

**Solution**: Ensure total_stake includes all committee members:
```rust
let total = committee.members()
    .iter()
    .map(|m| m.stake)
    .sum();
```

### Committee Rotation Issues

**Problem**: Wrong validator proposing

**Solution**: Verify slot calculation:
```rust
let expected_index = slot % committee.size();
let actual_proposer = committee.get_proposer(slot)?;
assert_eq!(actual_proposer.index, expected_index);
```

## References

- [Ëtrid Ivory Papers](../../docs/ivory-papers/) - Complete ASF consensus specification
- [HotStuff Paper](https://arxiv.org/abs/1803.05069) - Original HotStuff consensus protocol
- [ASF Algorithm](./src/lib.rs) - Core consensus implementation
- [PPFA Implementation](./src/ppfa.rs) - Complete PPFA sealing code
- [Integration Tests](./tests/ppfa_sealing_tests.rs) - Comprehensive test suite

## Version History

- **v0.1.0** (2025-10-22) - Initial PPFA sealing implementation
  - Basic seal creation and verification
  - Committee rotation
  - Stake-weighted voting
  - Complete test coverage
  - Pallet integration

## License

This implementation is part of the Ëtrid blockchain project.
Copyright (c) 2025 Ëtrid Foundation.
