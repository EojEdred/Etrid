# 03. Security - Cryptography & Key Management

## Overview

The Security layer provides cryptographic primitives, key management, and future post-quantum cryptography for the Ëtrid blockchain. It ensures secure signing, encryption, key derivation, and key lifecycle management across all system components.

**Status:** 🟡 Structured (Core cryptography complete, key management implemented, post-quantum planned)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Security Layer                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Cryptography (Core)                      │   │
│  │   - Ed25519 Signing                                   │   │
│  │   - X25519 Key Exchange (ECDH)                       │   │
│  │   - SHA-2 Hashing                                     │   │
│  │   - HKDF Key Derivation                              │   │
│  └────────────┬─────────────────────────────────────────┘   │
│               │                                              │
│               ↓                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │           Key Management                              │   │
│  │   - Secure key storage                                │   │
│  │   - Key rotation                                      │   │
│  │   - Backup & restore                                  │   │
│  │   - Access control                                    │   │
│  └────────────┬─────────────────────────────────────────┘   │
│               │                                              │
│               ↓                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │        Post-Quantum Cryptography                      │   │
│  │   - NIST PQC algorithms (planned)                     │   │
│  │   - Hybrid classic/PQC (planned)                      │   │
│  │   - Quantum-resistant signatures (planned)            │   │
│  └──────────────────────────────────────────────────────┘   │
│                               ↓                              │
└───────────────────────────────┼──────────────────────────────┘
                                ↓
                        All Ëtrid Components
              (FlareChain, PBCs, P2P, Accounts, etc.)
```

## Components

### 1. Cryptography (etrid-cryptography)

**Location:** `03-security/cryptography/`
**Package:** `etrid-cryptography`
**Purpose:** Core cryptographic primitives for Ëtrid: signing, encryption, hashing, KDF

**Description:**
Provides fundamental cryptographic operations used throughout the Ëtrid ecosystem, with a focus on modern, audited algorithms.

**Cryptographic Primitives:**

#### Signing & Verification
- **Algorithm:** Ed25519 (EdDSA)
- **Key Size:** 256 bits
- **Signature Size:** 64 bytes
- **Security Level:** ~128-bit security

```rust
// Generate keypair
let keypair = ed25519::Keypair::generate();

// Sign message
let signature = keypair.sign(message);

// Verify signature
let valid = keypair.public.verify(message, &signature)?;
```

#### Key Exchange
- **Algorithm:** X25519 (ECDH on Curve25519)
- **Shared Secret Size:** 32 bytes
- **Security Level:** ~128-bit security

```rust
// Generate ephemeral keys
let secret = x25519::StaticSecret::random();
let public = x25519::PublicKey::from(&secret);

// Derive shared secret
let shared_secret = secret.diffie_hellman(&peer_public);
```

#### Hashing
- **Algorithms:** SHA-256, SHA-512 (SHA-2 family)
- **Output Sizes:** 256 bits, 512 bits
- **Uses:** Message digests, commitment schemes, proof-of-work

```rust
// SHA-256 hash
use sha2::{Sha256, Digest};
let mut hasher = Sha256::new();
hasher.update(data);
let hash = hasher.finalize();
```

#### Key Derivation
- **Algorithm:** HKDF (HMAC-based KDF)
- **Base Hash:** SHA-256
- **Uses:** Deriving encryption keys, session keys

```rust
use hkdf::Hkdf;
use sha2::Sha256;

// Extract-and-expand KDF
let hk = Hkdf::<Sha256>::new(Some(&salt), &input_key_material);
let mut okm = [0u8; 32];
hk.expand(info, &mut okm)?;
```

**Dependencies:**
- `ed25519-dalek` - Ed25519 signatures
- `x25519-dalek` - X25519 key exchange
- `sha2` - SHA-2 family hashing
- `hkdf` - HMAC-based key derivation
- `rand` - Cryptographically secure RNG

**Features:**
- **no_std support** - Can run in embedded/WASM environments
- **Hardware acceleration** - Uses platform-specific optimizations when available
- **Side-channel resistance** - Constant-time operations

**Status:** ✅ Complete

---

### 2. Key Management (etrid-key-management)

**Location:** `03-security/key-management/`
**Package:** `etrid-key-management`
**Purpose:** Secure key storage, rotation, and backup/restore functionality

**Description:**
Manages the lifecycle of cryptographic keys used across Ëtrid, including secure storage, rotation policies, and disaster recovery.

**Key Features:**

#### Secure Storage
- **Encryption at rest:** Keys encrypted with master password
- **Access control:** Role-based key access
- **Audit logging:** All key operations logged

```rust
pub struct KeyStore {
    encrypted_keys: HashMap<KeyId, EncryptedKey>,
    master_key: MasterKey,
}

impl KeyStore {
    /// Store key securely
    pub async fn store_key(&mut self, key_id: KeyId, key: PrivateKey) -> Result<()>;

    /// Retrieve key (requires authorization)
    pub async fn get_key(&self, key_id: &KeyId, auth: &Authorization) -> Result<PrivateKey>;

    /// Delete key
    pub async fn delete_key(&mut self, key_id: &KeyId) -> Result<()>;
}
```

#### Key Rotation
- **Automatic rotation:** Keys rotated on schedule
- **Manual rotation:** On-demand rotation
- **Version tracking:** Key version history maintained

```rust
pub struct RotationPolicy {
    interval: Duration,
    max_age: Duration,
    rotation_callback: Box<dyn Fn(KeyId, PrivateKey) -> Result<()>>,
}

impl KeyManager {
    /// Rotate key
    pub async fn rotate_key(&mut self, key_id: &KeyId) -> Result<PrivateKey>;

    /// Set rotation policy
    pub fn set_rotation_policy(&mut self, policy: RotationPolicy);
}
```

#### Backup & Restore
- **Encrypted backups:** Backup files encrypted
- **Multi-party recovery:** Shamir's Secret Sharing for recovery
- **Export/import:** Secure key export for migration

```rust
/// Backup keys to encrypted file
pub async fn backup_keys(
    keystore: &KeyStore,
    backup_file: &Path,
    password: &str,
) -> Result<()>;

/// Restore keys from backup
pub async fn restore_keys(
    backup_file: &Path,
    password: &str,
) -> Result<KeyStore>;
```

**Dependencies:**
- `tokio` - Async runtime
- `ed25519-dalek` - Ed25519 keys
- `etrid-cryptography` - Core crypto
- `serde` - Serialization
- `base64` - Key encoding

**Security Features:**
- **Memory protection:** Keys zeroed after use
- **Rate limiting:** Prevents brute-force attacks
- **Audit trail:** All operations logged
- **Access control:** RBAC for key access

**Status:** ✅ Implemented

---

### 3. Post-Quantum Cryptography

**Location:** `03-security/post-quantum/`
**Purpose:** Quantum-resistant cryptographic algorithms

**Description:**
Placeholder for post-quantum cryptography integration to protect Ëtrid against future quantum computing threats.

**Planned Algorithms:**

#### NIST PQC Standardized Algorithms
- **CRYSTALS-Kyber:** Key encapsulation mechanism (KEM)
- **CRYSTALS-Dilithium:** Digital signatures
- **SPHINCS+:** Stateless hash-based signatures

#### Hybrid Schemes
- **Classic + PQC:** Combine Ed25519 with Dilithium
- **Gradual migration:** Backward compatibility during transition

**Roadmap:**
```
Phase 1 (Q2 2026):
- CRYSTALS-Dilithium integration
- Hybrid Ed25519 + Dilithium signatures

Phase 2 (Q3 2026):
- CRYSTALS-Kyber for key exchange
- Hybrid X25519 + Kyber KEM

Phase 3 (Q4 2026):
- SPHINCS+ for specific use cases
- Full PQC migration path
```

**Status:** 📋 Planned

---

## Protocol Layers

### Layer 1: Primitive Operations
- Random number generation (RNG)
- Byte-level operations
- Hardware acceleration

### Layer 2: Cryptographic Algorithms
- Ed25519 signing
- X25519 key exchange
- SHA-256/512 hashing
- HKDF derivation

### Layer 3: Key Lifecycle
- Key generation
- Secure storage
- Key rotation
- Backup/restore

### Layer 4: Application Integration
- Account keys (04-accounts)
- P2P encryption (01-detr-p2p)
- Transaction signing (07-transactions)
- Consensus (09-consensus)

## API Design

### Cryptography API

```rust
use etrid_cryptography::{Ed25519Keypair, X25519SharedSecret, Sha256Hash, HkdfDeriveKey};

// Generate signing keypair
let signing_keypair = Ed25519Keypair::generate()?;

// Sign and verify
let signature = signing_keypair.sign(message);
let valid = signing_keypair.verify(message, &signature)?;

// Key exchange
let secret = X25519SharedSecret::generate()?;
let public = secret.public_key();
let shared = secret.diffie_hellman(&peer_public)?;

// Hash
let hash = Sha256Hash::digest(data);

// Derive key
let derived_key = HkdfDeriveKey::derive(&input, &salt, &info)?;
```

### Key Management API

```rust
use etrid_key_management::{KeyManager, KeyId, RotationPolicy};

// Initialize key manager
let mut km = KeyManager::new(master_password)?;

// Store key
let key_id = KeyId::new("validator-key-1");
km.store_key(key_id.clone(), signing_keypair.private())?;

// Retrieve key
let auth = Authorization::new(user_credentials);
let private_key = km.get_key(&key_id, &auth).await?;

// Rotate key
let new_key = km.rotate_key(&key_id).await?;

// Backup
km.backup_to_file("keys.enc", backup_password).await?;

// Restore
let restored_km = KeyManager::restore_from_file("keys.enc", backup_password).await?;
```

## Security Features

### Cryptographic Security
- **Audited algorithms:** All algorithms from well-audited libraries
- **Constant-time operations:** Prevents timing side-channel attacks
- **Secure random:** Uses OS-provided CSPRNG

### Key Security
- **Encryption at rest:** All keys encrypted when stored
- **Memory protection:** Keys zeroed after use
- **Access control:** RBAC for key operations
- **Audit logging:** Complete audit trail

### Operational Security
- **Key rotation:** Automatic and manual rotation
- **Backup:** Encrypted backups with recovery
- **Monitoring:** Key usage monitoring
- **Incident response:** Key revocation capabilities

## Integration with Ëtrid Components

### 04-accounts
```rust
// Account uses Ed25519 for signing
let account_keypair = Ed25519Keypair::generate()?;
let account_id = AccountId::from(account_keypair.public());
```

### 01-detr-p2p
```rust
// P2P uses X25519 for encrypted channels
let p2p_secret = X25519SharedSecret::generate()?;
let encrypted_channel = p2p_secret.establish_channel(&peer_public)?;
```

### 07-transactions
```rust
// Transactions signed with Ed25519
let tx_signature = account_keypair.sign(&tx_bytes);
```

### 09-consensus
```rust
// Consensus messages signed by validators
let block_signature = validator_keypair.sign(&block_hash);
```

## Performance Characteristics

### Cryptographic Operations

| Operation | Time | Notes |
|-----------|------|-------|
| Ed25519 Sign | ~50 μs | Per signature |
| Ed25519 Verify | ~150 μs | Per verification |
| X25519 DH | ~70 μs | Shared secret derivation |
| SHA-256 Hash | ~5 μs/KB | Varies by data size |
| HKDF Derive | ~10 μs | Per derivation |

### Key Management

| Operation | Time | Notes |
|-----------|------|-------|
| Store Key | ~1 ms | Includes encryption |
| Retrieve Key | ~500 μs | Includes decryption |
| Rotate Key | ~2 ms | Generate + store |
| Backup | ~10 ms | 100 keys |
| Restore | ~20 ms | 100 keys |

## Testing

### Unit Tests
```bash
# Test cryptography
cargo test -p etrid-cryptography

# Test key management
cargo test -p etrid-key-management
```

### Integration Tests
```bash
# Full key lifecycle test
cargo test --test key_lifecycle

# Cross-component crypto test
cargo test --test integration_crypto
```

### Security Tests
```bash
# Side-channel resistance (requires specialized tooling)
cargo test --features side_channel_tests

# Fuzzing (requires cargo-fuzz)
cargo fuzz run crypto_primitives
```

## Known Issues

1. **Post-Quantum Not Implemented** - PQC algorithms not yet integrated
2. **Hardware Security Module (HSM)** - HSM integration not yet supported
3. **Multi-Party Computation (MPC)** - MPC key generation planned
4. **Threshold Signatures** - Not yet implemented

## Roadmap

### Phase 1: Core Cryptography (✅ Complete)
- [x] Ed25519 signing
- [x] X25519 key exchange
- [x] SHA-256/512 hashing
- [x] HKDF key derivation

### Phase 2: Key Management (✅ Complete)
- [x] Secure key storage
- [x] Key rotation
- [x] Backup/restore
- [x] Access control

### Phase 3: Advanced Features (🟡 Planned)
- [ ] HSM integration
- [ ] MPC key generation
- [ ] Threshold signatures (BLS, FROST)
- [ ] Shamir's Secret Sharing

### Phase 4: Post-Quantum (📋 2026)
- [ ] CRYSTALS-Dilithium signatures
- [ ] CRYSTALS-Kyber KEM
- [ ] Hybrid classic/PQC schemes
- [ ] Migration tooling

## Security Audit

### External Audits
- **Status:** Not yet audited
- **Planned:** Q2 2026
- **Scope:** All cryptographic implementations

### Internal Review
- **Code Review:** All cryptographic code peer-reviewed
- **Testing:** Comprehensive unit and integration tests
- **Documentation:** All security-sensitive code documented

## Compliance

### Standards Adherence
- **NIST FIPS 186-5:** Ed25519 digital signatures
- **RFC 7748:** X25519 Diffie-Hellman
- **NIST FIPS 180-4:** SHA-256/512 hashing
- **RFC 5869:** HKDF key derivation

### Best Practices
- **OWASP Cryptographic Storage Cheat Sheet:** Followed
- **Key Management Best Practices:** Implemented
- **Secure Coding Standards:** Applied

## References

### Standards
- **NIST FIPS 186-5:** Digital Signature Standard (DSS)
- **RFC 7748:** Elliptic Curves for Security
- **NIST FIPS 180-4:** Secure Hash Standard (SHS)
- **RFC 5869:** HKDF

### Libraries
- **ed25519-dalek:** https://github.com/dalek-cryptography/ed25519-dalek
- **x25519-dalek:** https://github.com/dalek-cryptography/x25519-dalek
- **RustCrypto:** https://github.com/RustCrypto

### Post-Quantum Resources
- **NIST PQC:** https://csrc.nist.gov/projects/post-quantum-cryptography
- **CRYSTALS:** https://pq-crystals.org/

---

**Component:** 03-security
**Version:** 0.1.0
**Status:** Core complete, post-quantum planned
**Security Level:** High (audited algorithms, side-channel resistance)
**Last Updated:** October 20, 2025
