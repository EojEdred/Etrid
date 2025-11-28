# Phase 2: Real Sr25519 Signing Implementation for ASF Finality

## Overview
This document describes the complete implementation of Phase 2: Replacing dummy signatures with real Sr25519 signing in the ASF finality gadget.

## Security Context
**CRITICAL**: The current implementation uses dummy signatures which is a critical security vulnerability. Votes are not actually cryptographically signed, making the system vulnerable to vote spoofing and consensus attacks.

## Changes Required

### 1. Cargo.toml Updates

Add the following dependencies to `/Users/macbook/Desktop/etrid/09-consensus/finality-gadget/Cargo.toml`:

```toml
[dependencies]
sp-keystore = { workspace = true }
sp-application-crypto = { workspace = true }
```

Update the `std` feature list to include:
```toml
std = [
    # ... existing features ...
    "sp-keystore/std",
    "sp-application-crypto/std",
]
```

### 2. Import Additions (lib.rs top of file)

Add these imports after the existing imports:

```rust
use sp_core::{sr25519, Pair};
use sp_keystore::{Keystore, KeystorePtr};
use sp_application-crypto::RuntimeAppPublic;
```

### 3. Signing Module (Add after CORE TYPES section)

Insert this complete signing module:

```rust
// ============================================================================
// ASF VOTE SIGNING MODULE
// ============================================================================

/// ASF Vote Signing using Sr25519
///
/// This module provides cryptographic signing for ASF consensus votes.
/// Each vote must be signed by the validator's Sr25519 key to prove
/// authenticity and prevent spoofing.
pub mod signing {
    use super::*;
    use sp_core::sr25519::{Public, Signature};
    use codec::Encode;

    /// Key type for ASF consensus keys in the keystore
    pub const ASF_KEY_TYPE: sp_application_crypto::KeyTypeId = sp_application_crypto::KeyTypeId(*b"asfv");

    /// Data that gets signed for a vote
    #[derive(Clone, Debug, Encode)]
    pub struct VoteSigningData {
        /// View number
        pub view: u64,
        /// Block hash being voted for
        pub block_hash: [u8; 32],
        /// Block number
        pub block_number: u32,
        /// Validator's public key
        pub validator_id: [u8; 32],
    }

    impl VoteSigningData {
        /// Create signing data from vote components
        pub fn new(view: u64, block_hash: [u8; 32], block_number: u32, validator_id: [u8; 32]) -> Self {
            Self { view, block_hash, block_number, validator_id }
        }

        /// Get the message bytes to sign
        pub fn to_sign_bytes(&self) -> Vec<u8> {
            // Prefix with domain separator to prevent cross-protocol attacks
            let mut bytes = b"ASF-VOTE-V1:".to_vec();
            bytes.extend(self.encode());
            bytes
        }
    }

    /// Sign a vote using the keystore
    pub fn sign_vote(
        keystore: &KeystorePtr,
        validator_public: &[u8; 32],
        signing_data: &VoteSigningData,
    ) -> Result<[u8; 64], SigningError> {
        let public = Public::from_raw(*validator_public);
        let message = signing_data.to_sign_bytes();

        // Sign using the keystore
        let signature = keystore
            .sr25519_sign(ASF_KEY_TYPE, &public, &message)
            .map_err(|e| SigningError::KeystoreError(format!("{:?}", e)))?
            .ok_or(SigningError::KeyNotFound)?;

        Ok(signature.0)
    }

    /// Verify a vote signature
    pub fn verify_vote_signature(
        validator_public: &[u8; 32],
        signing_data: &VoteSigningData,
        signature: &[u8; 64],
    ) -> bool {
        let public = Public::from_raw(*validator_public);
        let sig = Signature::from_raw(*signature);
        let message = signing_data.to_sign_bytes();

        sp_core::sr25519::Pair::verify(&sig, &message, &public)
    }

    #[derive(Debug, Clone)]
    pub enum SigningError {
        KeystoreError(String),
        KeyNotFound,
        InvalidSignature,
    }

    impl std::fmt::Display for SigningError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SigningError::KeystoreError(e) => write!(f, "Keystore error: {}", e),
                SigningError::KeyNotFound => write!(f, "Validator key not found in keystore"),
                SigningError::InvalidSignature => write!(f, "Invalid signature"),
            }
        }
    }

    impl std::error::Error for SigningError {}
}
```

### 4. Update Vote Struct

Replace the existing `Vote` struct with:

```rust
/// A signed vote from a validator
#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Vote {
    pub validator_id: ValidatorId,
    pub view: View,
    pub block_hash: BlockHash,
    /// Sr25519 signature (64 bytes)
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

impl Vote {
    /// Create and sign a new vote
    pub fn new(
        keystore: &KeystorePtr,
        validator_id: ValidatorId,
        view: View,
        block_hash: BlockHash,
        block_number: u32,
    ) -> Result<Self, signing::SigningError> {
        let validator_bytes: &[u8; 32] = validator_id.0.as_ref();
        let signing_data = signing::VoteSigningData::new(
            view.0,
            *block_hash.as_bytes(),
            block_number,
            *validator_bytes,
        );
        let signature = signing::sign_vote(keystore, validator_bytes, &signing_data)?;

        Ok(Self {
            validator_id,
            view,
            block_hash,
            signature: signature.to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    /// Verify this vote's signature
    pub fn verify(&self, block_number: u32) -> bool {
        if self.signature.len() != 64 {
            return false;
        }

        let validator_bytes: &[u8; 32] = self.validator_id.0.as_ref();
        let signing_data = signing::VoteSigningData::new(
            self.view.0,
            *self.block_hash.as_bytes(),
            block_number,
            *validator_bytes,
        );

        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&self.signature[..64]);

        signing::verify_vote_signature(validator_bytes, &signing_data, &sig_array)
    }
}
```

### 5. Update Certificate Struct

Replace the existing `Certificate` struct with:

```rust
/// A certificate proving block finality at a specific view
#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Certificate {
    pub view: View,
    pub block_hash: BlockHash,
    pub block_number: u32,
    pub signatures: Vec<(ValidatorId, Vec<u8>)>,
    pub timestamp: u64,
}

impl Certificate {
    /// Verify all signatures in this certificate
    pub fn verify_all_signatures(&self) -> bool {
        for (validator_id, signature) in &self.signatures {
            if signature.len() != 64 {
                tracing::warn!(
                    "Certificate has invalid signature length from validator {:?}",
                    hex::encode(validator_id.0.as_ref())
                );
                return false;
            }

            let validator_bytes: &[u8; 32] = validator_id.0.as_ref();
            let signing_data = signing::VoteSigningData::new(
                self.view.0,
                *self.block_hash.as_bytes(),
                self.block_number,
                *validator_bytes,
            );

            let mut sig_array = [0u8; 64];
            sig_array.copy_from_slice(&signature[..64]);

            if !signing::verify_vote_signature(validator_bytes, &signing_data, &sig_array) {
                tracing::warn!(
                    "Certificate has invalid signature from validator {:?}",
                    hex::encode(validator_bytes)
                );
                return false;
            }
        }
        true
    }

    /// Check if certificate has enough signatures (2f+1)
    pub fn has_quorum(&self, committee_size: usize) -> bool {
        let threshold = (2 * committee_size / 3) + 1;
        self.signatures.len() >= threshold
    }
}
```

### 6. Update VoteCollector Struct

Add block_numbers field:

```rust
pub struct VoteCollector {
    votes: HashMap<View, HashMap<BlockHash, Vec<(ValidatorId, Vec<u8>)>>>,
    block_numbers: HashMap<BlockHash, u32>,  // ADD THIS LINE
    quorum_threshold: u32,
    max_validators: u32,
}
```

Update constructor:

```rust
impl VoteCollector {
    pub fn new(max_validators: u32) -> Self {
        let quorum_threshold = (2 * max_validators / 3) + 1;
        Self {
            votes: HashMap::new(),
            block_numbers: HashMap::new(),  // ADD THIS LINE
            quorum_threshold,
            max_validators,
        }
    }
```

Update add_vote method signature and add verification:

```rust
pub fn add_vote(&mut self, vote: Vote, block_number: u32) -> Result<bool, String> {
    if vote.signature.is_empty() {
        return Err("Empty signature".to_string());
    }

    // VERIFY SIGNATURE
    if !vote.verify(block_number) {
        return Err("Invalid vote signature".to_string());
    }

    // Track block number
    self.block_numbers.insert(vote.block_hash, block_number);

    // ... rest of existing code ...
}
```

Add new method to VoteCollector:

```rust
pub fn get_block_number(&self, block_hash: &BlockHash) -> Option<u32> {
    self.block_numbers.get(block_hash).copied()
}
```

### 7. Update FinalityGadget Struct

Add keystore field:

```rust
pub struct FinalityGadget {
    validator_id: ValidatorId,
    max_validators: u32,
    keystore: KeystorePtr,  // ADD THIS LINE
    vote_collector: VoteCollector,
    // ... rest of fields ...
}
```

Update constructor:

```rust
impl FinalityGadget {
    pub fn new(
        validator_id: ValidatorId,
        max_validators: u32,
        keystore: KeystorePtr,  // ADD THIS PARAMETER
        network_bridge: Arc<dyn NetworkBridge>,
    ) -> Self {
        Self {
            validator_id,
            max_validators,
            keystore,  // ADD THIS LINE
            // ... rest of initialization ...
        }
    }
```

### 8. Update handle_vote Method

Change signature and add verification:

```rust
pub async fn handle_vote(&mut self, vote: Vote, block_number: u32) -> Result<(), String> {
    // ... existing view validation ...

    // Add to collector (this verifies the signature)
    let reached_quorum = self.vote_collector.add_vote(vote.clone(), block_number)?;

    // ... existing reputation update ...

    // If quorum reached, create certificate
    if reached_quorum {
        if let Some(signatures) = self.vote_collector.get_quorum_for_block(vote.view, vote.block_hash) {
            let cert = Certificate {
                view: vote.view,
                block_hash: vote.block_hash,
                block_number,  // ADD THIS LINE
                signatures: signatures.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            // Verify certificate before broadcasting
            if !cert.verify_all_signatures() {
                tracing::error!("Certificate verification failed! This should not happen.");
                return Err("Certificate verification failed".to_string());
            }

            // ... rest of logging and broadcasting ...
        }
    }

    Ok(())
}
```

### 9. Update handle_certificate Method

Add signature verification:

```rust
pub async fn handle_certificate(&mut self, cert: Certificate) -> Result<(), String> {
    // Verify all signatures in the certificate
    if !cert.verify_all_signatures() {
        return Err("Certificate has invalid signatures".to_string());
    }

    // Check quorum
    if !cert.has_quorum(self.max_validators as usize) {
        return Err("Certificate does not have quorum".to_string());
    }

    // ... rest of existing code ...
}
```

### 10. REPLACE propose_block Method (CRITICAL)

**LOCATION**: Around line 621-648 in lib.rs

**REMOVE THIS ENTIRE DUMMY SIGNATURE BLOCK**:

```rust
pub async fn propose_block(&mut self, block_hash: BlockHash) -> Result<Vote, String> {
    // V9 TEMPORARY FIX: Use dummy signature to unblock finality
    // TODO: Implement proper Sr25519 signing with validator keystore
    let dummy_signature = {
        let mut sig = Vec::with_capacity(64);
        // V9: Create deterministic signature from full AccountId32 + block_hash for uniqueness
        sig.extend_from_slice(self.validator_id.0.as_ref()); // 32 bytes from AccountId32
        sig.extend_from_slice(&block_hash.0[0..32]); // + 32 bytes from block_hash = 64 bytes total
        sig
    };

    let vote = Vote {
        validator_id: self.validator_id.clone(),
        view: self.view_timer.get_current_view(),
        block_hash,
        signature: dummy_signature,  // V7: Dummy signature for testing BFT consensus
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    // Reset view timer when proposing - blocks are progressing, don't change view
    self.view_timer.reset();

    self.broadcast_vote(vote.clone()).await?;
    Ok(vote)
}
```

**REPLACE WITH THIS PRODUCTION-READY CODE**:

```rust
pub async fn propose_block(&mut self, block_hash: BlockHash, block_number: u32) -> Result<Vote, String> {
    // Create and sign the vote using validator's keystore key
    let vote = Vote::new(
        &self.keystore,
        self.validator_id.clone(),
        self.view_timer.get_current_view(),
        block_hash,
        block_number,
    ).map_err(|e| format!("Failed to sign vote: {}", e))?;

    // Verify our own signature before broadcasting (sanity check)
    if !vote.verify(block_number) {
        tracing::error!("Self-verification of vote signature failed!");
        return Err("Self-verification failed".to_string());
    }

    tracing::info!(
        "🗳️ Created signed vote for block #{} ({:02x}{:02x}..{:02x}{:02x}) in view {}",
        block_number,
        block_hash.0[0], block_hash.0[1],
        block_hash.0[30], block_hash.0[31],
        self.view_timer.get_current_view().0
    );

    // Reset view timer when proposing - blocks are progressing, don't change view
    self.view_timer.reset();

    self.broadcast_vote(vote.clone()).await?;
    Ok(vote)
}
```

### 11. Update Tests

Update test setup to use proper keystore:

```rust
#[test]
fn test_vote_accumulation() {
    use sp_keystore::testing::MemoryKeystore;

    let mut collector = VoteCollector::new(3);
    let keystore: KeystorePtr = Arc::new(MemoryKeystore::new());

    // Generate test keys
    let public1 = keystore
        .sr25519_generate_new(signing::ASF_KEY_TYPE, None)
        .unwrap();
    let public2 = keystore
        .sr25519_generate_new(signing::ASF_KEY_TYPE, None)
        .unwrap();

    let validator_id1 = ValidatorId(AccountId32::new(public1.0));
    let validator_id2 = ValidatorId(AccountId32::new(public2.0));

    let vote1 = Vote::new(
        &keystore,
        validator_id1,
        View(0),
        BlockHash([0u8; 32]),
        1,
    ).unwrap();

    let vote2 = Vote::new(
        &keystore,
        validator_id2,
        View(0),
        BlockHash([0u8; 32]),
        1,
    ).unwrap();

    assert!(!collector.add_vote(vote1, 1).unwrap());
    assert!(collector.add_vote(vote2, 1).unwrap());
}
```

Update the finality gadget test:

```rust
#[tokio::test]
async fn test_finality_gadget_vote_flow() {
    use sp_keystore::testing::MemoryKeystore;

    let keystore: KeystorePtr = Arc::new(MemoryKeystore::new());
    let public = keystore
        .sr25519_generate_new(signing::ASF_KEY_TYPE, None)
        .unwrap();
    let validator_id = ValidatorId(AccountId32::new(public.0));

    let bridge = Arc::new(MockNetworkBridge);
    let mut gadget = FinalityGadget::new(
        validator_id.clone(),
        3,
        keystore.clone(),
        bridge
    );

    // Create a properly signed vote
    let vote = Vote::new(
        &keystore,
        validator_id,
        View(0),
        BlockHash([0u8; 32]),
        1,
    ).unwrap();

    gadget.handle_vote(vote, 1).await.unwrap();
    assert_eq!(gadget.get_current_view(), View(0));
}
```

## Implementation Checklist

- [ ] Add `sp-keystore` and `sp-application-crypto` to Cargo.toml
- [ ] Add Sr25519 imports to lib.rs
- [ ] Add signing module with VoteSigningData, sign_vote, verify_vote_signature
- [ ] Update Vote struct with new() constructor and verify() method
- [ ] Update Certificate struct with verify_all_signatures() and has_quorum() methods
- [ ] Add block_numbers field to VoteCollector
- [ ] Update VoteCollector::add_vote to verify signatures
- [ ] Add keystore field to FinalityGadget struct
- [ ] Update FinalityGadget constructor to accept keystore
- [ ] Update handle_vote to pass block_number and verify signatures
- [ ] Update handle_certificate to verify all signatures
- [ ] **REPLACE propose_block dummy signature code with real signing**
- [ ] Update all tests to use MemoryKeystore
- [ ] Test compilation: `cargo check -p etrid-finality-gadget`
- [ ] Test build: `cargo build -p etrid-finality-gadget`
- [ ] Run tests: `cargo test -p etrid-finality-gadget`

## Security Notes

1. **Domain Separation**: The signing uses "ASF-VOTE-V1:" prefix to prevent cross-protocol attacks
2. **Key Type**: Uses custom key type "asfv" for ASF consensus keys
3. **Signature Verification**: All votes are verified before being added to the collector
4. **Certificate Validation**: Certificates verify all signatures and check quorum before acceptance
5. **Self-Verification**: Votes created by the gadget are self-verified before broadcasting

## Testing

After implementation, verify with:

```bash
# Check compilation
cargo check -p etrid-finality-gadget

# Build
cargo build -p etrid-finality-gadget

# Run tests
cargo test -p etrid-finality-gadget

# Run with logging
RUST_LOG=debug cargo test -p etrid-finality-gadget -- --nocapture
```

## Next Steps

After Phase 2 is complete:
- Phase 3: Checkpoint BFT integration
- Phase 4: Implicit finality tracking
- Phase 5: Fork pruning
- Phase 6: Full ASF finality integration
