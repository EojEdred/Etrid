# ËTRID Bridge Attestation - Quick Reference Card

## Files Overview

| File | Purpose | Key Features |
|------|---------|--------------|
| `attester-genesis.json` | Genesis config with 5 attesters | Placeholder ECDSA keys, 3-of-5 threshold |
| `README.md` | Complete documentation (500+ lines) | Key gen, registration, operations, security |
| `generate-attester-keys.sh` | Automated key generation | Uses subkey, creates JSON, secure permissions |
| `runtime-integration-example.rs` | Integration examples | Runtime config, genesis loading, tests |
| `SETUP_GUIDE.md` | Quick start guide | Step-by-step setup, troubleshooting |
| `genesis.rs` | Pallet genesis module | Validation, build logic, 8 tests |

## Key Commands

### Generate Real Keys
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge-config
./generate-attester-keys.sh 5
```

### Build Runtime
```bash
cd runtime/flare-chain  # or your runtime
cargo build --release --features runtime-benchmarks
```

### Register Attester (Governance)
```rust
// Via sudo/governance
BridgeAttestation::register_attester(
    RuntimeOrigin::root(),
    hex::decode("02a1b2c3d4e5f6...").unwrap()
)
```

### Update Threshold
```rust
BridgeAttestation::update_threshold(
    RuntimeOrigin::root(),
    3  // New threshold
)
```

### Check Status
```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"bridgeAttestation_activeAttesterCount"}' \
  http://localhost:9944
```

## Attester Configuration

### Default Setup (3-of-5)

| ID | Name | Role | Operator |
|----|------|------|----------|
| 1 | Attester-Alpha | Primary | ËTRID Foundation |
| 2 | Attester-Beta | Secondary | ËTRID Foundation |
| 3 | Attester-Gamma | Tertiary | Community Partner A |
| 4 | Attester-Delta | Quaternary | Community Partner B |
| 5 | Attester-Epsilon | Quinary | Community Partner C |

**Threshold**: 3 signatures required out of 5 total

**Byzantine Fault Tolerance**: Can tolerate up to 2 faulty/malicious attesters

## Key Formats

### ECDSA (EVM-Compatible)

```
Compressed:   33 bytes (0x02... or 0x03...)
Uncompressed: 65 bytes (0x04...)
Private Key:  32 bytes
Signature:    65 bytes (r, s, v)
```

### SR25519 (Substrate Native)

```
Public Key:   32 bytes
Private Key:  32 bytes
Signature:    64 bytes
```

## Common Operations Cheat Sheet

### Development

```bash
# Generate test keys
./generate-attester-keys.sh 5

# Build with genesis
cargo build --release

# Start dev node
./target/release/etrid-node --dev

# Check logs
tail -f /tmp/etrid-node.log | grep attestation
```

### Production

```bash
# Generate production keys (HSM)
./generate-attester-keys.sh 5

# Backup keys securely
tar -czf attester-keys-backup.tar.gz ~/.etrid/attester-keys/
gpg --encrypt --recipient security@etrid.network attester-keys-backup.tar.gz

# Deploy to chain
./target/release/etrid-node --chain=mainnet-raw.json \
  --validator \
  --name="Attester-Alpha" \
  --rpc-methods=Safe \
  --prometheus-external
```

### Monitoring

```bash
# Prometheus metrics
curl -s http://localhost:9615/metrics | grep bridge_attestation

# Active attesters
bridge_attestation_active_count 5

# Total attestations
bridge_attestation_total_verified 1234

# Last attestation timestamp
bridge_attestation_last_attestation_timestamp 1701234567
```

## Threshold Configurations

### Standard Security (3-of-5)
- **Use Case**: General cross-chain transfers
- **Configuration**: M=3, N=5
- **Tolerance**: 2 failures

### High Security (4-of-5)
- **Use Case**: High-value transfers, Ethereum mainnet
- **Configuration**: M=4, N=5
- **Tolerance**: 1 failure

### Testnet (2-of-5)
- **Use Case**: Development and testing
- **Configuration**: M=2, N=5
- **Tolerance**: 3 failures

## Security Checklist (Production)

### Pre-Launch
- [ ] Generate unique keys (not placeholders)
- [ ] Store in HSM/secure vault
- [ ] Test threshold signing
- [ ] Set up monitoring
- [ ] Configure alerts
- [ ] Document procedures
- [ ] Audit infrastructure

### Post-Launch
- [ ] Monitor attester health
- [ ] Track attestation latency
- [ ] Review signature counts
- [ ] Check for anomalies
- [ ] Rotate keys (6-12mo)
- [ ] Update documentation
- [ ] Test failover

## Emergency Procedures

### Suspected Compromise
```rust
// 1. Immediately disable
BridgeAttestation::disable_attester(Root, attester_id);

// 2. Investigate
// Check logs, signatures, network activity

// 3. If confirmed, remove
BridgeAttestation::remove_attester(Root, attester_id);
```

### Service Outage
```rust
// Pause all attestations
BridgeAttestation::pause_attestation(Root);

// Fix issues...

// Resume operations
BridgeAttestation::unpause_attestation(Root);
```

### Threshold Adjustment
```rust
// Temporarily lower threshold
BridgeAttestation::update_threshold(Root, 2);

// Restore after recovery
BridgeAttestation::update_threshold(Root, 3);
```

## File Locations

### Development
```
Keys:           ~/.etrid/attester-keys/
Genesis:        ./attester-genesis-generated.json
Config:         ./bridge-config/
Logs:           /tmp/etrid-node.log
```

### Production
```
Keys (HSM):     AWS KMS / YubiHSM / CloudHSM
Config:         /etc/etrid/bridge-config/
Logs:           /var/log/etrid/attestation.log
Backups:        s3://etrid-backups/attester-keys/
```

## Integration Pattern

```rust
// 1. Add to Cargo.toml
pallet-bridge-attestation = { path = "...", default-features = false }

// 2. Configure in runtime
impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = ConstU32<1001>;
    type MaxAttesters = ConstU32<100>;
    type MaxAttestersPerMessage = ConstU32<20>;
    type MinSignatureThreshold = ConstU32<3>;
    type AttestationMaxAge = ConstU32<1000>;
    type WeightInfo = ();
}

// 3. Add to construct_runtime!
BridgeAttestation: pallet_bridge_attestation,

// 4. Configure genesis
bridge_attestation: BridgeAttestationConfig {
    attesters: vec![/* ... */],
    threshold: GenesisThreshold { min_signatures: 3, total_attesters: 5 },
    domain_thresholds: vec![],
    _phantom: Default::default(),
}
```

## Troubleshooting Quick Fixes

| Error | Quick Fix |
|-------|-----------|
| "Invalid threshold" | Ensure 0 < M <= N |
| "Insufficient attesters" | Enable more or lower threshold |
| "Invalid key length" | Use 32, 33, or 65 bytes |
| "Duplicate key" | Generate unique keys |
| "Nonce already used" | Increment nonce for new message |
| "Attestation expired" | Increase AttestationMaxAge |
| "Service paused" | Call unpause_attestation |

## Performance Tuning

### Block Time: 6 seconds
```rust
type AttestationMaxAge = ConstU32<1000>;  // ~2 hours
```

### Block Time: 12 seconds
```rust
type AttestationMaxAge = ConstU32<500>;   // ~2 hours
```

### High Throughput
```rust
type MaxAttestersPerMessage = ConstU32<50>;  // More concurrent signatures
```

## Support Resources

| Resource | URL |
|----------|-----|
| Full Documentation | `README.md` (this directory) |
| Setup Guide | `SETUP_GUIDE.md` |
| Integration Example | `runtime-integration-example.rs` |
| Pallet Source | `/pallets-shared/pallet-bridge-attestation/` |
| Discord | https://discord.gg/etrid |
| Forum | https://forum.etrid.network |
| GitHub | https://github.com/etrid/etrid |

## Key Generation Methods

### Method 1: Subkey (Recommended)
```bash
subkey generate --scheme ecdsa --output-type json
```

### Method 2: OpenSSL
```bash
openssl ecparam -name secp256k1 -genkey -out key.pem
```

### Method 3: Web3.js
```javascript
const account = web3.eth.accounts.create();
```

### Method 4: ethers.js
```javascript
const wallet = ethers.Wallet.createRandom();
```

## Version Information

- **Pallet Version**: 1.0.0
- **Substrate Framework**: Compatible with Polkadot SDK
- **Signature Schemes**: ECDSA (secp256k1), SR25519
- **Hash Algorithm**: Blake2-256
- **Replay Protection**: Nonce-based

---

**Last Updated**: 2024-12-04
**Maintainer**: ËTRID Foundation
**Status**: Production Ready
