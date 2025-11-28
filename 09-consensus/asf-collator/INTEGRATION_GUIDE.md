# ASF Collator Integration Guide

This guide shows how to integrate ASF consensus into PBC runtimes.

## Step 1: Add Dependencies

Add to your PBC runtime's `Cargo.toml`:

```toml
[dependencies]
# ASF collator consensus
pallet-asf-collator = { path = "../../../../09-consensus/pallet-asf-collator", default-features = false }
asf-collator = { path = "../../../../09-consensus/asf-collator", default-features = false }
asf-algorithm = { path = "../../../../09-consensus/asf-algorithm", default-features = false }

[features]
std = [
    # ... existing features
    "pallet-asf-collator/std",
    "asf-collator/std",
    "asf-algorithm/std",
]
```

## Step 2: Configure Runtime

Add to your runtime's `lib.rs`:

```rust
// ASF Collator Configuration
parameter_types! {
    pub const ParaId: u32 = 2000; // Your parachain ID
    pub const MinCollators: u32 = 7;
    pub const MaxCollators: u32 = 11;
    pub const MinCollatorStake: Balance = 1_000_000 * UNITS; // 1M ETR
    pub const SessionLength: BlockNumber = 600; // ~1 hour
    pub const RotationPeriod: u64 = 6; // Rotate every 6 blocks
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

// Add to construct_runtime! macro
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // ... existing pallets
        AsfCollator: pallet_asf_collator,
    }
);
```

## Step 3: Initialize Genesis

In your chain spec or genesis config:

```rust
use pallet_asf_collator::GenesisConfig as AsfCollatorConfig;

// Genesis configuration
GenesisConfig {
    // ... existing configs
    asf_collator: AsfCollatorConfig {
        initial_collators: vec![
            (account_1, 5_000_000 * UNITS),
            (account_2, 5_000_000 * UNITS),
            (account_3, 5_000_000 * UNITS),
            (account_4, 5_000_000 * UNITS),
            (account_5, 5_000_000 * UNITS),
            (account_6, 5_000_000 * UNITS),
            (account_7, 5_000_000 * UNITS),
        ],
    },
}
```

## Step 4: Collator Node Integration

Create a collator service that integrates with Cumulus:

```rust
// In your collator node service
use asf_collator::{RotationManager, RotationConfig, FinalityTracker};

pub struct AsfCollatorService {
    rotation_manager: RotationManager,
    finality_tracker: FinalityTracker,
}

impl AsfCollatorService {
    pub fn new(para_id: u32) -> Self {
        let config = RotationConfig::default();
        Self {
            rotation_manager: RotationManager::new(config),
            finality_tracker: FinalityTracker::new(para_id),
        }
    }

    pub async fn run(self, client: Arc<Client>, relay_chain_interface: Arc<dyn RelayChainInterface>) {
        // Collator consensus loop
        loop {
            // 1. Check if should propose block (based on rotation)
            let current_block = client.info().best_number;

            if self.rotation_manager.should_rotate(current_block) {
                // 2. Get relay chain parent
                let relay_parent = relay_chain_interface.best_block_hash().await;

                // 3. Propose block using cumulus
                let proposal = self.propose_block(relay_parent).await;

                // 4. Gather votes from other collators
                let votes = self.gather_votes(proposal).await;

                // 5. Create ASF certificate
                let certificate = self.create_certificate(votes).await;

                // 6. Submit to runtime
                self.submit_certificate(certificate).await;
            }

            // 7. Update finality from relay chain
            self.sync_relay_finality(relay_chain_interface.clone()).await;

            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }
}
```

## Step 5: Bridge Integration

For bridges, add ASF security:

```rust
use asf_bridge_security::{BridgeManager, BridgeTransfer, BridgeSecurityProof};

// In your bridge pallet
pub fn transfer_to_chain(
    origin: OriginFor<T>,
    target_para: ParaId,
    asset: AssetId,
    amount: Balance,
    recipient: Vec<u8>,
) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    // 1. Create transfer request
    let transfer = BridgeTransfer {
        transfer_id: Self::generate_transfer_id(),
        source_para: T::ParaId::get(),
        target_para,
        source_block: frame_system::Pallet::<T>::block_hash(frame_system::Pallet::<T>::block_number()),
        source_block_number: frame_system::Pallet::<T>::block_number(),
        asset,
        amount,
        sender: sender.encode(),
        recipient,
        direction: BridgeDirection::Outbound,
    };

    // 2. Lock assets
    T::Assets::lock(asset, &sender, amount)?;

    // 3. Wait for ASF finality
    PendingTransfers::<T>::insert(transfer.transfer_id, transfer);

    // 4. Collators will create attestations once finalized

    Ok(())
}

// Collators create attestations
pub fn create_bridge_attestation(
    origin: OriginFor<T>,
    transfer_id: Hash,
) -> DispatchResult {
    let collator = ensure_signed(origin)?;

    let transfer = PendingTransfers::<T>::get(transfer_id)
        .ok_or(Error::<T>::TransferNotFound)?;

    // Check finality level
    let finality = AsfCollator::finality_level(transfer.source_block);
    ensure!(
        finality >= CollatorFinalityLevel::Strong,
        Error::<T>::InsufficientFinality
    );

    // Create attestation
    let attestation = CrossChainAttestation::new(
        T::ParaId::get(),
        transfer.target_para,
        transfer.source_block,
        transfer.source_block_number,
        finality,
        collator,
        /* relay_block */ 0,
    );

    // Submit to ASF collator pallet
    AsfCollator::submit_cross_chain_attestation(origin, attestation)?;

    Ok(())
}
```

## Configuration Parameters

### Recommended Values by PBC Type

#### High-Value PBCs (BTC, ETH)
```rust
MinCollators: 11
MaxCollators: 21
MinCollatorStake: 5_000_000 * UNITS  // 5M ETR
RotationPeriod: 12  // Slower rotation for stability
```

#### Medium-Value PBCs (BNB, SOL, XRP)
```rust
MinCollators: 9
MaxCollators: 15
MinCollatorStake: 2_000_000 * UNITS  // 2M ETR
RotationPeriod: 6  // Standard rotation
```

#### Lower-Value PBCs (TRX, SC-USDT)
```rust
MinCollators: 7
MaxCollators: 11
MinCollatorStake: 1_000_000 * UNITS  // 1M ETR
RotationPeriod: 6  // Standard rotation
```

## Finality Thresholds

ASF collator finality levels (lower than relay chain):

- **None**: 0-4 certificates
- **Weak**: 5-9 certificates (sufficient for low-value transfers)
- **Moderate**: 10-19 certificates (sufficient for medium-value transfers)
- **Strong**: 20-49 certificates (required for high-value bridges)
- **Irreversible**: 50+ certificates OR relay chain finalized

## Bridge Security Requirements

For cross-chain bridges:

```rust
// Minimum attestation stake: 2x transfer value
min_attestation_stake = transfer_value * 2

// Minimum finality level: Strong (20+ certificates)
min_finality_level = CollatorFinalityLevel::Strong

// Challenge period: 100 relay chain blocks (~10 minutes)
challenge_period = 100

// Slash amount: 10x deposit
slash_amount = deposit * 10
```

## Testing

Test your integration:

```bash
# Build runtime
cargo build --release --features runtime-benchmarks

# Run collator
./target/release/your-pbc-collator \
  --collator \
  --para-id 2000 \
  --chain your-chain-spec.json \
  --relay-chain-rpc-url ws://localhost:9944

# Test ASF certificate creation
cargo test --package pallet-asf-collator

# Benchmark ASF operations
cargo run --release --features runtime-benchmarks -- \
  benchmark pallet \
  --pallet pallet_asf_collator \
  --extrinsic "*" \
  --output ./pallets/asf-collator/src/weights.rs
```

## Monitoring

Monitor ASF consensus health:

```rust
// Check collator committee
let committee = AsfCollator::committee();
log::info!("Active collators: {:?}", committee.collators.len());

// Check finality levels
let finality = AsfCollator::finality_level(block_hash);
log::info!("Block {} finality: {:?}", block_number, finality);

// Check rotation round
let round = AsfCollator::rotation_round();
log::info!("Current rotation round: {}", round);

// Check pending attestations
let attestation = AsfCollator::cross_chain_attestation(target_para, target_block);
if let Some(att) = attestation {
    log::info!("Attestations: {}, Total stake: {}",
        att.attestations.len(),
        att.total_stake
    );
}
```

## Troubleshooting

### Issue: Certificate not reaching threshold

**Solution**: Check committee size and stake distribution
```rust
let committee = AsfCollator::committee();
let threshold = committee.bft_threshold();
log::info!("Need {} votes, have {}", threshold, votes.len());
```

### Issue: Slow finality

**Solution**: Increase rotation frequency or collator count
```rust
// Reduce rotation period
pub const RotationPeriod: u64 = 3; // Faster rotation

// Or increase collators
pub const MaxCollators: u32 = 15;
```

### Issue: Bridge attestations not finalizing

**Solution**: Check finality levels and stake requirements
```rust
let finality = AsfCollator::finality_level(source_block);
ensure!(finality >= CollatorFinalityLevel::Strong, Error);

let multisig = AsfCollator::cross_chain_attestation(para_id, block_hash);
ensure!(multisig.total_stake >= MIN_STAKE, Error);
```
