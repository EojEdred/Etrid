# ËTRID PBC Bridge - Mainnet Implementation Plan

**Generated:** 2025-12-01
**Status:** Action Required Before Mainnet

---

## Executive Summary

Based on comprehensive analysis, the PBC bridge architecture is **correctly designed** but has **4 critical security gaps** that must be resolved before mainnet:

| Gap | Severity | Location | Est. Work |
|-----|----------|----------|-----------|
| Signature Verification | CRITICAL | pallet-edsc-bridge-attestation:599-601 | 2 days |
| Attestation Integration | CRITICAL | pallet-edsc-bridge-token-messenger:549-557 | 1 day |
| Relayer Authorization | HIGH | 5 bridge pallets (TODO comments) | 2 days |
| Token Burn/Mint | HIGH | token-messenger:394, 449 | 1 day |

**Total Estimated Work:** 6-8 days focused development

---

## Part 1: Signature Verification Implementation

### Current State (BYPASSED)
```rust
// pallet-edsc-bridge-attestation/src/lib.rs:599-601
// In production, verify signature cryptographically:
// Self::verify_signature(&attester.public_key, &message_hash, signature)?;
```

### Implementation

**Step 1: Add verify_signature function**

```rust
/// Verify ECDSA signature (add after line 707)
fn verify_signature(
    public_key: &[u8],
    message_hash: &H256,
    signature: &[u8],
) -> DispatchResult {
    ensure!(signature.len() == 65, Error::<T>::InvalidSignature);
    ensure!(
        public_key.len() == 33 || public_key.len() == 32,
        Error::<T>::InvalidPublicKey
    );

    let sig_array: [u8; 65] = signature
        .try_into()
        .map_err(|_| Error::<T>::InvalidSignature)?;

    let pubkey_array: [u8; 33] = if public_key.len() == 33 {
        public_key.try_into()
            .map_err(|_| Error::<T>::InvalidPublicKey)?
    } else {
        return Err(Error::<T>::InvalidPublicKey.into());
    };

    let is_valid = sp_io::crypto::ecdsa_verify(
        &sig_array,
        &message_hash.0,
        &pubkey_array,
    );

    ensure!(is_valid, Error::<T>::InvalidSignature);
    Ok(())
}
```

**Step 2: Enable verification in verify_attestation (replace lines 589-601)**

```rust
for (attester_id, signature) in attestation.signatures.iter() {
    let attester = Attesters::<T>::get(attester_id)
        .ok_or(Error::<T>::AttesterNotFound)?;

    ensure!(
        attester.status == AttesterStatus::Active,
        Error::<T>::AttesterNotActive
    );

    // ENABLE CRYPTOGRAPHIC VERIFICATION
    Self::verify_signature(
        &attester.public_key,
        &message_hash,
        signature,
    )?;
}
```

---

## Part 2: Token-Messenger Attestation Integration

### Current State (PLACEHOLDER)
```rust
// pallet-edsc-bridge-token-messenger/src/lib.rs:549-557
fn verify_attestation(_message: &[u8], _attestation: &[u8]) -> DispatchResult {
    // For now, always succeed (placeholder)
    Ok(())
}
```

### Implementation: Trait-Based Loose Coupling

**Step 1: Add traits to pallet config**

```rust
// In pallet-edsc-bridge-token-messenger/src/lib.rs

/// Trait for attestation verification
pub trait AttestationVerifier {
    fn verify_message_attestation(
        message: &[u8],
        message_hash: H256,
    ) -> DispatchResult;
}

/// Trait for token operations
pub trait TokenOperations<AccountId> {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult;
    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult;
}

#[pallet::config]
pub trait Config: frame_system::Config {
    // ... existing config ...

    /// Attestation verification provider
    type AttestationVerifier: AttestationVerifier;

    /// Token operations provider
    type TokenOperations: TokenOperations<Self::AccountId>;
}
```

**Step 2: Replace placeholder**

```rust
fn verify_attestation(message: &[u8], _attestation: &[u8]) -> DispatchResult {
    let message_hash = Self::get_message_hash_from_bytes(message)?;
    T::AttestationVerifier::verify_message_attestation(message, message_hash)
}

fn get_message_hash_from_bytes(message_bytes: &[u8]) -> Result<H256, Error<T>> {
    CrossChainMessage::decode(&mut &message_bytes[..])
        .map(|msg| Self::get_message_hash(&msg))
        .map_err(|_| Error::<T>::InvalidMessageFormat)
}
```

**Step 3: Wire in runtime**

```rust
// In runtime lib.rs
impl pallet_edsc_bridge_token_messenger::Config for Runtime {
    type AttestationVerifier =
        pallet_edsc_bridge_attestation::Pallet<Runtime>;
    type TokenOperations = EdscTokenBridgeOps;
}

pub struct EdscTokenBridgeOps;
impl pallet_edsc_bridge_token_messenger::TokenOperations<AccountId> for EdscTokenBridgeOps {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_edsc_token::Pallet::<Runtime>::do_burn(account, amount)
    }
    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_edsc_token::Pallet::<Runtime>::do_mint(account, amount)
    }
}
```

---

## Part 3: Relayer Authorization Pattern

### Bridges with TODO Gaps (5 pallets)
1. BNB Bridge (line 790)
2. Solana Bridge (line 769)
3. Polygon Bridge (line 798)
4. XRP Bridge (line 789)
5. Bitcoin Bridge (line 640)

### Unified Implementation

**Step 1: Add to common crate (/bridges/protocols/common/src/relayer.rs)**

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum RelayerRole {
    Oracle,     // Can process burns (highest privilege)
    RelayNode,  // Can confirm deposits
    Custodian,  // Multi-sig approvals
}

pub trait RelayerAuthorization<AccountId> {
    fn is_authorized_relayer(relayer: &AccountId) -> bool;
    fn can_confirm_deposit(relayer: &AccountId) -> bool;
    fn can_process_burn(relayer: &AccountId) -> bool;
}
```

**Step 2: Add storage to each bridge pallet**

```rust
#[pallet::storage]
pub type AuthorizedRelayers<T: Config> = StorageMap<
    _, Blake2_128Concat, T::AccountId, RelayerRole, OptionQuery,
>;

#[pallet::storage]
pub type RelayerActive<T: Config> = StorageMap<
    _, Blake2_128Concat, T::AccountId, bool, ValueQuery,
>;
```

**Step 3: Add authorization function**

```rust
fn ensure_authorized_relayer(
    relayer: &T::AccountId,
    operation: RelayerOperation,
) -> DispatchResult {
    let role = AuthorizedRelayers::<T>::get(relayer)
        .ok_or(Error::<T>::NotAuthorizedRelayer)?;

    ensure!(RelayerActive::<T>::get(relayer), Error::<T>::RelayerInactive);

    match operation {
        RelayerOperation::ConfirmDeposit => {
            ensure!(
                matches!(role, RelayerRole::Oracle | RelayerRole::RelayNode),
                Error::<T>::InsufficientRelayerPermissions
            );
        }
        RelayerOperation::ProcessBurn => {
            ensure!(
                matches!(role, RelayerRole::Oracle),
                Error::<T>::InsufficientRelayerPermissions
            );
        }
    }
    Ok(())
}
```

**Step 4: Replace TODO in each bridge**

```rust
// Before (current):
pub fn process_etr_burn_from_bnb(...) -> DispatchResult {
    let _relayer = ensure_signed(origin)?;
    // TODO: Add relayer authorization check

// After:
pub fn process_etr_burn_from_bnb(...) -> DispatchResult {
    let relayer = ensure_signed(origin)?;
    Self::ensure_authorized_relayer(&relayer, RelayerOperation::ProcessBurn)?;
```

---

## Part 4: Token Burn/Mint Implementation

### Current State (COMMENTED OUT)
```rust
// Line 394: In production, this would call pallet_edsc_token::burn()
// Line 449: In production, this would call pallet_edsc_token::mint()
```

### Implementation

**In burn_edsc_for_external_chain() - after line 380:**
```rust
// ACTUAL BURN - replace placeholder
T::TokenOperations::burn_tokens(&sender, amount)?;
```

**In receive_and_mint() - after line 434:**
```rust
// ACTUAL MINT - replace placeholder
let recipient_account = Self::bytes_to_account(&burn_msg.mint_recipient)?;
T::TokenOperations::mint_tokens(&recipient_account, burn_msg.amount)?;

// Helper function
fn bytes_to_account(bytes: &[u8]) -> Result<T::AccountId, Error<T>> {
    T::AccountId::decode(&mut &bytes[..])
        .map_err(|_| Error::<T>::InvalidRecipient)
}
```

---

## Part 5: Multi-Chain Deployment Strategy

### Deployment Order (Based on PBC Architecture)

| Chain | PBC | Bridge Pallet | Priority | Est. Cost |
|-------|-----|---------------|----------|-----------|
| Solana | sol-pbc | solana-bridge | 1 (Active) | $350 |
| BSC | bnb-pbc | bnb-bridge | 2 | $160 |
| Ethereum | eth-pbc | ethereum-bridge | 3 | $450 |
| Polygon | matic-pbc | polygon-bridge | 4 | $150 |
| Tron | trx-pbc | tron-bridge | 5 | $50 |
| Arbitrum | (new) | (create) | 6 | $150 |

### Flow: External Chain → Native ËTR

```
User buys SPL ËTR on Solana (Raydium)
        ↓
Deposits to Solana bridge contract
        ↓
Solana PBC detects deposit (light client)
        ↓
Relayer submits attestation
        ↓
M-of-N custodians sign
        ↓
Token-messenger verifies attestation
        ↓
Burns wrapped ËTR on Sol-PBC
        ↓
Mints native ËTR on Primearc Core Chain
        ↓
User receives native ËTR!
```

---

## Implementation Timeline

### Week 1: Core Security (CRITICAL)
- [ ] Day 1-2: Implement signature verification in attestation pallet
- [ ] Day 3: Wire attestation to token-messenger
- [ ] Day 4: Implement token burn/mint calls
- [ ] Day 5: Unit tests for all security functions

### Week 2: Relayer System
- [ ] Day 1: Create relayer.rs in common crate
- [ ] Day 2-3: Add relayer storage/functions to all 12 bridge pallets
- [ ] Day 4: Replace TODO comments with authorization checks
- [ ] Day 5: Integration tests

### Week 3: Multi-Chain Deployment
- [ ] Day 1-2: Deploy BEP-20 ËTR on BSC + PancakeSwap
- [ ] Day 3-4: Configure BNB PBC bridge
- [ ] Day 5: Deploy on Polygon + QuickSwap

### Week 4: Testing & Launch
- [ ] Day 1-2: Testnet deployment
- [ ] Day 3-4: Security audit review
- [ ] Day 5: Mainnet activation

---

## File Locations Reference

| Component | Path |
|-----------|------|
| Attestation Pallet | `/05-multichain/bridges/protocols/edsc-bridge/substrate-pallets/pallet-edsc-bridge-attestation/src/lib.rs` |
| Token Messenger | `/05-multichain/bridges/protocols/edsc-bridge/substrate-pallets/pallet-edsc-bridge-token-messenger/src/lib.rs` |
| EDSC Token | `/05-multichain/bridges/protocols/edsc-bridge/substrate-pallets/pallet-edsc-token/src/lib.rs` |
| ETR Lock | `/06-native-currency/pallets/pallet-etr-lock/src/lib.rs` |
| Common Bridge | `/05-multichain/bridges/protocols/common/src/` |
| Solana Bridge | `/05-multichain/bridges/protocols/solana-bridge/src/lib.rs` |
| BNB Bridge | `/05-multichain/bridges/protocols/bnb-bridge/src/lib.rs` |

---

## Risk Assessment

| Risk | Before Fix | After Fix |
|------|------------|-----------|
| Signature spoofing | CRITICAL | RESOLVED |
| Unauthorized minting | CRITICAL | RESOLVED |
| Replay attacks | HIGH | RESOLVED |
| Unauthorized relayers | HIGH | RESOLVED |
| Double-spending | HIGH | RESOLVED |

---

## Conclusion

The PBC bridge architecture is **sound and correctly designed**. The gaps are implementation details, not architectural flaws. With 6-8 days of focused work, the system will be production-ready for mainnet.

**Recommended approach:**
1. Fix Week 1 critical items first
2. Deploy to testnet for integration testing
3. Get security audit on signature verification
4. Proceed with multi-chain deployment

The multi-chain strategy (Solana → BSC → Ethereum → Polygon) leverages your existing PBC infrastructure without needing external bridges like Wormhole or LayerZero.
