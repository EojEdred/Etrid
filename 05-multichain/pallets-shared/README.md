# Shared Pallets for Etrid Multi-Chain

This directory contains **generic, reusable Substrate pallets** that can be used across multiple PBCs (Partition Burst Chains) in the Etrid ecosystem.

## Philosophy

Instead of duplicating bridge logic in each PBC, we create **shared pallets** that provide common functionality for all chains. This approach:

- Reduces code duplication
- Ensures consistent security model
- Simplifies maintenance
- Accelerates new bridge development
- Provides battle-tested components

## Available Pallets

### 1. pallet-bridge-attestation

**Generic M-of-N threshold signature verification for cross-chain bridges**

- **Location**: `./pallet-bridge-attestation/`
- **Purpose**: Verify cross-chain messages have sufficient valid signatures from independent attesters
- **Use Cases**: Any bridge implementation (EDSC, BSC, Polygon, Polkadot, etc.)
- **Features**:
  - M-of-N threshold signatures
  - ECDSA (EVM) and SR25519 (Substrate) support
  - Configurable ChainId per runtime
  - Nonce-based replay protection
  - Byzantine fault tolerant
  - Emergency pause mechanism

**Documentation**:
- [README.md](./pallet-bridge-attestation/README.md) - Full pallet docs
- [INTEGRATION_GUIDE.md](./pallet-bridge-attestation/INTEGRATION_GUIDE.md) - Step-by-step integration
- [QUICK_REFERENCE.md](./pallet-bridge-attestation/QUICK_REFERENCE.md) - Quick command reference

**Quick Integration**:
```rust
impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = ConstU32<100>;
    type MaxAttesters = ConstU32<100>;
    type MaxAttestersPerMessage = ConstU32<20>;
    type MinSignatureThreshold = ConstU32<2>;
    type AttestationMaxAge = ConstU32<1000>;
    type WeightInfo = ();
}
```

### 2. pallet-circuit-breaker

**Emergency pause mechanism for DeFi operations**

- **Location**: `./pallet-circuit-breaker/`
- **Purpose**: Halt operations during emergencies (exploits, bugs, etc.)
- **Use Cases**: DEX pools, lending protocols, bridge operations

### 3. pallet-xcm-bridge

**Cross-chain messaging via XCM protocol**

- **Location**: `./pallet-xcm-bridge/`
- **Purpose**: Enable XCM-based cross-chain communication
- **Use Cases**: Polkadot/Kusama bridges, parachain communication

## Directory Structure

```
pallets-shared/
├── README.md                      # This file
├── pallet-bridge-attestation/     # Generic bridge attestation (NEW)
│   ├── Cargo.toml
│   ├── README.md
│   ├── INTEGRATION_GUIDE.md
│   ├── QUICK_REFERENCE.md
│   ├── ARCHITECTURE.md
│   ├── CHANGES_FROM_EDSC.md
│   └── src/
│       ├── lib.rs                 # Main pallet (1053 lines)
│       ├── weights.rs             # Weight functions
│       ├── mock.rs                # Test runtime
│       └── tests.rs               # Test suite (15 tests)
├── pallet-circuit-breaker/        # Emergency pause
└── pallet-xcm-bridge/             # XCM messaging
```

## Usage Guidelines

### 1. Choose the Right Pallet

| If you need... | Use this pallet |
|----------------|-----------------|
| Cross-chain message attestation | `pallet-bridge-attestation` |
| Emergency pause functionality | `pallet-circuit-breaker` |
| XCM cross-chain messaging | `pallet-xcm-bridge` |

### 2. Add to Your Runtime

In your PBC runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-bridge-attestation = { path = "../../pallets-shared/pallet-bridge-attestation", default-features = false }

[features]
std = [
    "pallet-bridge-attestation/std",
]
```

### 3. Configure

```rust
// Configure the pallet for your specific chain
parameter_types! {
    pub const YourChainId: u32 = 100;
}

impl pallet_bridge_attestation::Config for Runtime {
    type ChainId = YourChainId;
    // ... other config
}
```

### 4. Use in Your Bridge

```rust
// In your bridge token messenger
use pallet_bridge_attestation as BridgeAttestation;

pub fn receive_message(...) -> DispatchResult {
    // Verify attestation before processing
    BridgeAttestation::<T>::verify_attestation_for_message(&message, hash)?;
    
    // Process the message...
}
```

## Design Principles

### 1. Generic First
Pallets should be usable by any PBC without modification. Chain-specific logic goes in the PBC runtime, not the shared pallet.

### 2. Configurable
Use runtime constants (`type ChainId: Get<u32>`) to allow each PBC to customize behavior.

### 3. Minimal Dependencies
Keep dependencies minimal to reduce compilation time and potential conflicts.

### 4. Well Documented
Every pallet should have:
- README.md with overview and API docs
- Integration guide with examples
- Inline rustdoc comments
- Test suite

### 5. Security First
Shared pallets are used by multiple chains, so security is paramount:
- Comprehensive tests
- Security audits
- Clear error handling
- Proper weight functions

## PBC Usage Matrix

| PBC | pallet-bridge-attestation | pallet-circuit-breaker | pallet-xcm-bridge |
|-----|---------------------------|------------------------|-------------------|
| EDSC (Ethereum) | ✓ | ✓ | - |
| BSC | ✓ | ✓ | - |
| Polygon | ✓ | ✓ | - |
| Avalanche | ✓ | ✓ | - |
| Polkadot | ✓ | ✓ | ✓ |
| Kusama | ✓ | ✓ | ✓ |

## Contributing

When adding new shared pallets:

1. **Check for Reusability**: Will this be used by 2+ PBCs?
2. **Make it Generic**: Remove chain-specific logic
3. **Document Thoroughly**: README + Integration guide + inline docs
4. **Test Comprehensively**: Unit tests + integration examples
5. **Version Properly**: Follow semantic versioning

## Testing Shared Pallets

```bash
# Test a specific pallet
cargo test -p pallet-bridge-attestation

# Test all shared pallets
cd pallets-shared
cargo test --all

# Check compilation
cargo check --all
```

## Benchmarking

Shared pallets should include WeightInfo traits:

```rust
pub trait WeightInfo {
    fn extrinsic_name() -> Weight;
}

// Use in pallet
#[pallet::weight(T::WeightInfo::extrinsic_name())]
pub fn extrinsic_name(...) -> DispatchResult { ... }
```

Generate weights in each PBC:
```bash
./target/release/your-pbc-node benchmark pallet \
    --pallet=pallet_bridge_attestation \
    --extrinsic='*' \
    --output=./weights/
```

## Versioning

Shared pallets follow semantic versioning:
- **Major** (1.0.0): Breaking changes to API
- **Minor** (0.1.0): New features, backwards compatible
- **Patch** (0.0.1): Bug fixes

## License

All shared pallets: MIT OR Apache-2.0

## Support

- GitHub Issues: Main Etrid repository
- Documentation: Each pallet's README
- Examples: Integration guides in each pallet

## Future Shared Pallets

Planned additions:
- `pallet-token-wrapper` - Generic token wrapping/unwrapping
- `pallet-fee-distributor` - Fee distribution across chains
- `pallet-governance-proxy` - Cross-chain governance
- `pallet-oracle-aggregator` - Price oracle aggregation

## Migration from Chain-Specific Pallets

If you have a chain-specific pallet that could be shared:

1. Copy to `pallets-shared/`
2. Remove chain-specific logic
3. Add configurable types
4. Update documentation
5. Add integration examples
6. Test with multiple PBCs
7. Update all PBCs to use shared version

See `pallet-bridge-attestation/CHANGES_FROM_EDSC.md` for a real example.

---

**Maintained by**: Etrid Team  
**Last Updated**: 2025-12-04
