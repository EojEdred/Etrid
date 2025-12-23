# ËTRID Bridge Attestation - Quick Setup Guide

## Overview

This directory contains everything needed to configure the ËTRID bridge attestation system with M-of-N threshold signatures (default: 3-of-5).

## Files Created

### 1. Configuration Files

- **`attester-genesis.json`** - Genesis configuration with 5 placeholder attester definitions
- **`README.md`** - Comprehensive guide covering all aspects of attester management
- **`runtime-integration-example.rs`** - Example code showing how to integrate into a runtime
- **`generate-attester-keys.sh`** - Automated script to generate real attester keys
- **`SETUP_GUIDE.md`** - This file (quick start guide)

### 2. Pallet Code

- **`pallets-shared/pallet-bridge-attestation/src/genesis.rs`** - Genesis configuration module
- **`pallets-shared/pallet-bridge-attestation/src/lib.rs`** - Updated to export genesis module

## Quick Start

### Step 1: Generate Real Attester Keys

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge-config

# Generate 5 attester key pairs
./generate-attester-keys.sh 5

# Keys will be saved to: ~/.etrid/attester-keys/
# Genesis config will be saved to: ./attester-genesis-generated.json
```

### Step 2: Integrate into Runtime

Add to your `runtime/Cargo.toml`:

```toml
[dependencies]
pallet-bridge-attestation = { path = "../../05-multichain/pallets-shared/pallet-bridge-attestation", default-features = false }

[features]
std = [
    "pallet-bridge-attestation/std",
    # ... other pallets
]
```

Add to your `runtime/src/lib.rs`:

```rust
// Configure the pallet
impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = ConstU32<1001>;  // Your chain ID
    type MaxAttesters = ConstU32<100>;
    type MaxAttestersPerMessage = ConstU32<20>;
    type MinSignatureThreshold = ConstU32<3>;
    type AttestationMaxAge = ConstU32<1000>;
    type WeightInfo = ();
}

// Add to construct_runtime! macro
construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        BridgeAttestation: pallet_bridge_attestation,
    }
);
```

### Step 3: Configure Genesis

In your chain spec file (`chain_spec.rs`):

```rust
use pallet_bridge_attestation::{GenesisAttester, GenesisConfig, GenesisThreshold};

fn testnet_genesis() -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        // ... other pallets
        bridge_attestation: BridgeAttestationConfig {
            attesters: vec![
                GenesisAttester {
                    public_key: hex::decode("02a1b2c3d4e5f6...").unwrap(),
                    name: b"Attester-Alpha".to_vec(),
                    enabled: true,
                },
                // ... add all 5 attesters
            ],
            threshold: GenesisThreshold {
                min_signatures: 3,
                total_attesters: 5,
            },
            domain_thresholds: vec![],
            _phantom: Default::default(),
        },
    }
}
```

### Step 4: Deploy and Verify

```bash
# Build runtime
cargo build --release

# Generate chain spec
./target/release/etrid-node build-spec --chain=testnet > chain-spec.json

# Convert to raw format
./target/release/etrid-node build-spec --chain=chain-spec.json --raw > chain-spec-raw.json

# Start node with new genesis
./target/release/etrid-node --chain=chain-spec-raw.json

# Verify attesters are registered
# Via polkadot.js: Developer -> Chain State -> bridgeAttestation -> attesters
```

## Security Checklist

Before deploying to mainnet:

- [ ] Generated unique keys using secure method (not placeholder keys)
- [ ] Stored private keys in HSM or secure offline storage
- [ ] Verified all attester public keys are correct
- [ ] Tested attestation threshold (3-of-5) works correctly
- [ ] Set up monitoring for attester health
- [ ] Configured alerting for failed attestations
- [ ] Documented incident response procedures
- [ ] Created key rotation schedule (6-12 months)
- [ ] Tested emergency pause functionality
- [ ] Audited attester infrastructure security
- [ ] Backed up keys to multiple secure locations
- [ ] Added `.gitignore` entries for private keys
- [ ] Reviewed all attester endpoints are correct
- [ ] Tested cross-chain message signing flow
- [ ] Verified replay protection (nonce-based) works

## M-of-N Threshold System

### Default: 3-of-5 Configuration

- **N = 5**: Total attesters (Alpha, Beta, Gamma, Delta, Epsilon)
- **M = 3**: Minimum signatures required
- **Byzantine Fault Tolerance**: Can tolerate up to 2 faulty/malicious attesters
- **Liveness**: Only need 3 attesters online (not all 5)

### Adjusting the Threshold

Via governance:

```rust
// Update global threshold
BridgeAttestation::update_threshold(
    origin: Root,
    new_threshold: 4  // Change to 4-of-5
);

// Domain-specific threshold
BridgeAttestation::configure_threshold(
    origin: Root,
    domain_id: Some(1),  // Ethereum mainnet
    min_signatures: 4,   // Higher security
    total_attesters: 5
);
```

## Common Operations

### Register New Attester (Governance)

```bash
# Via polkadot.js UI
Developer -> Extrinsics -> sudo -> sudo(call)
  -> bridgeAttestation -> registerAttester(publicKey)
  -> publicKey: 0x02a1b2c3d4e5f6...
  -> Submit Transaction
```

### Disable Compromised Attester

```rust
// Emergency disable
Sudo::sudo(
    RuntimeOrigin::root(),
    Box::new(Call::BridgeAttestation(
        pallet_bridge_attestation::Call::disable_attester {
            attester_id: 3,
        }
    ))
);
```

### Monitor Attestations

```bash
# Check active attesters
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_call", "params": ["BridgeAttestationApi_active_attester_count", "0x"]}' \
  http://localhost:9944

# Monitor via Prometheus
curl -s http://localhost:9615/metrics | grep bridge_attestation
```

## Troubleshooting

### Issue: "Invalid threshold configuration"

**Cause**: min_signatures > total_attesters or min_signatures = 0

**Solution**: Ensure 0 < min_signatures <= total_attesters

### Issue: "Insufficient active attesters"

**Cause**: Not enough active attesters to meet threshold

**Solution**: Enable more attesters or lower threshold via governance

### Issue: "Invalid public key length"

**Cause**: Public key not 32, 33, or 65 bytes

**Solution**:
- ECDSA compressed: 33 bytes (0x02... or 0x03...)
- ECDSA uncompressed: 65 bytes (0x04...)
- SR25519: 32 bytes

### Issue: "Duplicate public key in genesis"

**Cause**: Same public key used for multiple attesters

**Solution**: Generate unique keys for each attester

### Issue: "Nonce already used"

**Cause**: Replay attack prevention triggered

**Solution**: This is expected - use a new nonce for each message

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    ËTRID Bridge System                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Source Chain              Attesters               Dest Chain    │
│  ┌──────────┐           ┌───────────┐           ┌──────────┐  │
│  │  Token   │           │ Attester 1│ ───┐      │  Token   │  │
│  │Messenger │──────────▶│  (Alpha)  │    │      │Messenger │  │
│  └──────────┘           ├───────────┤    │      └──────────┘  │
│      │                  │ Attester 2│    │          │          │
│      │                  │  (Beta)   │    ▼          │          │
│      │                  ├───────────┤  ┌─────┐     │          │
│      │                  │ Attester 3│─▶│3-of-5│────┘          │
│      │                  │ (Gamma)   │  │Check│   Verify       │
│      │                  ├───────────┤  └─────┘                │
│      │                  │ Attester 4│    │                    │
│      │                  │  (Delta)  │ ───┘                    │
│      └─────────────────▶├───────────┤                         │
│        Submit Message   │ Attester 5│                         │
│                         │ (Epsilon) │                         │
│                         └───────────┘                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Key Storage Best Practices

### Development

```bash
# Store in local directory (git-ignored)
~/.etrid/attester-keys/
├── attester-1/
│   ├── secret-phrase.txt (400 permissions)
│   ├── public-key.txt
│   └── full-keypair.json (400 permissions)
└── README.txt
```

### Production

1. **Hardware Security Module (HSM)**
   - AWS CloudHSM
   - YubiHSM
   - Ledger Nano S/X

2. **Key Management Service (KMS)**
   - AWS KMS
   - Google Cloud KMS
   - Azure Key Vault

3. **Multi-Signature Recovery**
   - Use Shamir's Secret Sharing
   - M-of-N recovery scheme
   - Geographic distribution

## Next Steps

1. Review the comprehensive `README.md` for detailed operations
2. Study `runtime-integration-example.rs` for integration patterns
3. Generate production keys using `generate-attester-keys.sh`
4. Test on local testnet before mainnet deployment
5. Set up monitoring and alerting infrastructure
6. Document your incident response procedures
7. Schedule regular key rotation (every 6-12 months)

## Support

- **Documentation**: `/docs/bridge-attestation.md`
- **Source Code**: `/pallets-shared/pallet-bridge-attestation/`
- **Discord**: https://discord.gg/etrid
- **Forum**: https://forum.etrid.network
- **GitHub Issues**: https://github.com/etrid/etrid/issues

## License

Copyright (C) 2024 ËTRID Foundation
